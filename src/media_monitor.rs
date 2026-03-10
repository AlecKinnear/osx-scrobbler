// Media monitoring module
// Snapshot-driven state machine for tracking media playback and scrobbling

use crate::config::AppFilteringConfig;
use crate::scrobbler::Track;
use crate::text_cleanup::TextCleaner;
use anyhow::Result;
use chrono::{DateTime, Utc};
use media_remote::NowPlayingInfo;
use std::time::{Duration, Instant};

const MIN_TRACK_DURATION: u64 = 30; // Minimum track duration in seconds to scrobble
const SCROBBLE_TIME_THRESHOLD: u64 = 240; // 4 minutes in seconds

/// Action to take based on app filtering
#[derive(Debug, PartialEq)]
enum AppFilterAction {
    Allow,
    Ignore,
    PromptUser,
}

/// Represents the current play session state
#[derive(Debug, Clone)]
struct PlaySession {
    track: Track,
    bundle_id: Option<String>,
    started_at_utc: DateTime<Utc>,
    duration: u64,
    scrobbled: bool,
    now_playing_sent: bool,
    /// Last known elapsed time from the media player (seconds)
    elapsed_secs: u64,
    is_playing: bool,
    /// When we last updated elapsed_secs from a snapshot
    last_snapshot_at: Instant,
    scrobble_allowed: bool,
}

impl PlaySession {
    fn new(
        track: Track,
        bundle_id: Option<String>,
        duration: u64,
        elapsed: f64,
        is_playing: bool,
        scrobble_allowed: bool,
    ) -> Self {
        Self {
            track,
            bundle_id,
            started_at_utc: Utc::now(),
            duration,
            scrobbled: false,
            now_playing_sent: false,
            elapsed_secs: elapsed as u64,
            is_playing,
            last_snapshot_at: Instant::now(),
            scrobble_allowed,
        }
    }

    /// Update play state from a new snapshot
    fn update_from_snapshot(&mut self, elapsed: f64, is_playing: bool) {
        self.elapsed_secs = elapsed as u64;
        self.is_playing = is_playing;
        self.last_snapshot_at = Instant::now();
    }

    /// Current played time, extrapolating from last snapshot if still playing
    fn played_seconds(&self) -> u64 {
        if self.is_playing {
            let since_snapshot = self.last_snapshot_at.elapsed().as_secs();
            self.elapsed_secs + since_snapshot
        } else {
            self.elapsed_secs
        }
    }

    /// Time needed to reach scrobble threshold
    fn scrobble_at(&self, threshold_percent: u8) -> Option<u64> {
        if self.scrobbled || self.duration < MIN_TRACK_DURATION {
            return None;
        }
        let threshold_time = (self.duration * threshold_percent as u64) / 100;
        Some(threshold_time.min(SCROBBLE_TIME_THRESHOLD))
    }

    fn should_scrobble(&self, threshold_percent: u8) -> bool {
        if !self.scrobble_allowed {
            return false;
        }
        match self.scrobble_at(threshold_percent) {
            Some(target) => self.played_seconds() >= target,
            None => false,
        }
    }

    fn should_send_now_playing(&self) -> bool {
        !self.now_playing_sent
    }
}

/// Media monitor that processes snapshots of now-playing state
pub struct MediaMonitor {
    scrobble_threshold: u8,
    current_session: Option<PlaySession>,
    text_cleaner: TextCleaner,
}

impl MediaMonitor {
    pub fn new(scrobble_threshold: u8, text_cleaner: TextCleaner) -> Self {
        Self {
            scrobble_threshold,
            current_session: None,
            text_cleaner,
        }
    }

    /// Check if an app should be scrobbled based on filtering config
    fn should_scrobble_app(
        &self,
        bundle_id: &Option<String>,
        app_filtering: &AppFilteringConfig,
    ) -> AppFilterAction {
        match bundle_id {
            None => {
                if app_filtering.scrobble_unknown {
                    AppFilterAction::Allow
                } else {
                    AppFilterAction::Ignore
                }
            }
            Some(id) if id.is_empty() => {
                if app_filtering.scrobble_unknown {
                    AppFilterAction::Allow
                } else {
                    AppFilterAction::Ignore
                }
            }
            Some(id) => {
                if app_filtering.allowed_apps.contains(id) {
                    return AppFilterAction::Allow;
                }
                if app_filtering.ignored_apps.contains(id) {
                    return AppFilterAction::Ignore;
                }
                if app_filtering.prompt_for_new_apps {
                    AppFilterAction::PromptUser
                } else {
                    AppFilterAction::Allow
                }
            }
        }
    }

    /// Convert media_remote NowPlayingInfo to our Track structure
    /// Only applies IDAGIO-specific metadata parsing if IDAGIO markers are found
    fn media_info_to_track(&self, info: &NowPlayingInfo) -> Option<(Track, bool)> {
        let title = info.title.clone()?;
        let artist = info.artist.clone()?;
        let album = info.album.clone();

        log::debug!(
            "Media info: artist=\"{}\" title=\"{}\" album={:?}",
            artist,
            title,
            album
        );

        // Detect IDAGIO by presence of "IDAGIO" in metadata (brand-specific marker)
        // IDAGIO is not an English word, so its presence indicates IDAGIO source
        let is_idagio = title.contains("IDAGIO")
            || artist.contains("IDAGIO")
            || album.as_ref().is_some_and(|a| a.contains("IDAGIO"));

        if is_idagio {
            // IDAGIO path: Clean up for display, but DISABLE scrobbling
            let (parsed_artist, parsed_title, upc) = if artist.trim().is_empty() {
                crate::text_cleanup::parse_classical_metadata(&artist, &title)
            } else {
                (artist.clone(), title.clone(), None)
            };

            let clean_artist = self.text_cleaner.clean(&parsed_artist);
            let clean_title = self.text_cleaner.clean(&parsed_title);
            let mut clean_album = self.text_cleaner.clean_option(album);

            if clean_album.is_none() && !clean_title.is_empty() {
                clean_album = Some(clean_title.clone());
            }

            let track = Track {
                title: clean_title,
                artist: clean_artist,
                album: clean_album,
                duration: info.duration.map(|d| d as u64),
                upc,
                lastfm_album_art_url: None,
            };

            Some((track, false))
        } else {
            // Generic path: Standard processing, scrobbling enabled
            let mut final_artist = artist;
            let mut final_title = title;

            // Handle cases like Yandex where artist is empty and title is "Artist - Title"
            // This is explicitly NOT done for Idagio tracks.
            if final_artist.trim().is_empty() && !final_title.is_empty() {
                if let Some(pos) = final_title.find(" - ") {
                    let (possible_artist, possible_title) = final_title.split_at(pos);
                    let possible_title = &possible_title[3..]; // Skip " - "
                    if !possible_artist.is_empty() && !possible_title.is_empty() {
                        final_artist = possible_artist.to_string();
                        final_title = possible_title.to_string();
                    }
                }
            }

            let clean_artist = self.text_cleaner.clean(&final_artist);
            let clean_title = self.text_cleaner.clean(&final_title);
            let clean_album = self.text_cleaner.clean_option(album);

            let track = Track {
                title: clean_title,
                artist: clean_artist,
                album: clean_album,
                duration: info.duration.map(|d| d as u64),
                upc: None,
                lastfm_album_art_url: None,
            };

            Some((track, true))
        }
    }

    /// Process a snapshot of now-playing info and return events
    pub fn handle_media_change(
        &mut self,
        info: Option<NowPlayingInfo>,
        app_filtering: &AppFilteringConfig,
    ) -> Result<MediaEvents> {
        let mut events = MediaEvents::default();

        if let Some(info) = info {
            let is_playing = info.is_playing.unwrap_or(false);
            let elapsed = info.elapsed_time.unwrap_or(0.0);

            log::debug!(
                "now playing info: title={:?}, artist={:?}, album={:?}, \
                 duration={:?}, elapsed={:.0}s, is_playing={}, bundle={:?}",
                info.title,
                info.artist,
                info.album,
                info.duration,
                elapsed,
                is_playing,
                info.bundle_id
            );

            if let Some((track, scrobble_allowed)) = self.media_info_to_track(&info) {
                let duration = track.duration.unwrap_or(0);
                let bundle_id = info.bundle_id.clone();

                // Check if we should scrobble from this app
                match self.should_scrobble_app(&bundle_id, app_filtering) {
                    AppFilterAction::Ignore => {
                        log::debug!("Ignoring playback from {:?}", bundle_id);
                        return Ok(events);
                    }
                    AppFilterAction::PromptUser => {
                        if let Some(ref id) = bundle_id {
                            events.unknown_app = Some(id.clone());
                        }
                        return Ok(events);
                    }
                    AppFilterAction::Allow => {}
                }

                // Check if this is a new track or continuation
                let is_new_track = match &self.current_session {
                    None => true,
                    Some(session) => session.track != track,
                };

                if is_new_track {
                    log::info!(
                        "New track: {} - {} ({}s, {:.0}s in) from {:?}",
                        track.artist,
                        track.title,
                        duration,
                        elapsed,
                        bundle_id
                    );

                    let mut session = PlaySession::new(
                        track.clone(),
                        bundle_id.clone(),
                        duration,
                        elapsed,
                        is_playing,
                        scrobble_allowed,
                    );
                    session.now_playing_sent = true;
                    self.current_session = Some(session);
                    events.now_playing = Some((track, bundle_id, scrobble_allowed));
                } else if let Some(session) = self.current_session.as_mut() {
                    // Same track — update elapsed time from snapshot
                    session.update_from_snapshot(elapsed, is_playing);

                    // Check scrobble eligibility
                    if session.should_scrobble(self.scrobble_threshold) {
                        log::info!(
                            "Scrobbling: {} - {} (played {}s / {}s)",
                            session.track.artist,
                            session.track.title,
                            session.played_seconds(),
                            session.duration
                        );

                        events.scrobble = Some((
                            session.track.clone(),
                            session.started_at_utc,
                            session.bundle_id.clone(),
                        ));
                        session.scrobbled = true;
                    } else if session.should_send_now_playing() {
                        events.now_playing =
                            Some((session.track.clone(), session.bundle_id.clone(), session.scrobble_allowed));
                        session.now_playing_sent = true;
                    }
                }
            }
        } else {
            // No media playing, clear session
            if self.current_session.is_some() {
                log::info!("Media stopped, clearing session");
                self.current_session = None;
            }
        }

        Ok(events)
    }

    /// Check if the current session should scrobble (timer-driven)
    pub fn check_scrobble(&mut self) -> Result<MediaEvents> {
        let mut events = MediaEvents::default();

        if let Some(session) = self.current_session.as_mut() {
            if session.should_scrobble(self.scrobble_threshold) {
                log::info!(
                    "Scrobbling (deadline): {} - {} (played {}s / {}s)",
                    session.track.artist,
                    session.track.title,
                    session.played_seconds(),
                    session.duration
                );

                events.scrobble = Some((
                    session.track.clone(),
                    session.started_at_utc,
                    session.bundle_id.clone(),
                ));
                session.scrobbled = true;
            }
        }

        Ok(events)
    }

    /// Returns when the next scrobble check should happen
    pub fn next_scrobble_deadline(&self) -> Option<Instant> {
        let session = self.current_session.as_ref()?;
        if !session.is_playing {
            return None;
        }

        let scrobble_at = session.scrobble_at(self.scrobble_threshold)?;
        let played = session.played_seconds();

        if played >= scrobble_at {
            return Some(Instant::now());
        }

        let remaining = scrobble_at - played;
        Some(Instant::now() + Duration::from_secs(remaining))
    }
}

/// Events generated by media monitoring
#[derive(Debug, Default)]
pub struct MediaEvents {
    pub now_playing: Option<(Track, Option<String>, bool)>,
    pub scrobble: Option<(Track, DateTime<Utc>, Option<String>)>,
    pub unknown_app: Option<String>,
}
