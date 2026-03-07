// Metadata enricher module
// Enriches track metadata using MusicBrainz API for better scrobbling accuracy

use crate::scrobbler::Track;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

const MUSICBRAINZ_API: &str = "https://musicbrainz.org/ws/2";
const LASTFM_API: &str = "https://ws.audioscrobbler.com/2.0/";
const DURATION_TOLERANCE_MS: u64 = 3000; // Allow 3 seconds tolerance for album track matching

// Thread-safe cache for Last.fm album art URLs
// Key: (artist, album) tuple, Value: Option<String> (None means no art found)
lazy_static::lazy_static! {
    static ref LASTFM_ART_CACHE: Mutex<HashMap<(String, String), Option<String>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize)]
struct LastFmAlbumResponse {
    album: Option<LastFmAlbum>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LastFmAlbum {
    image: Option<Vec<LastFmImage>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LastFmImage {
    size: Option<String>,
    #[serde(rename = "#text")]
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MBReleaseSearchResult {
    releases: Option<Vec<MBRelease>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MBRelease {
    title: Option<String>,
    id: Option<String>,
    #[serde(rename = "media")]
    media: Option<Vec<MBMedia>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MBMedia {
    #[serde(rename = "track-count")]
    track_count: Option<u32>,
    #[serde(rename = "tracks")]
    tracks: Option<Vec<MBTrack>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MBTrack {
    title: Option<String>,
    length: Option<u64>, // Duration in milliseconds
}

/// Try to fetch album art from Last.fm (with caching)
/// Returns the large image URL if available
/// Results are cached by (artist, album) to avoid repeated API calls
pub fn fetch_lastfm_album_art(artist: &str, album: &str, api_key: &str) -> Result<Option<String>> {
    if artist.is_empty() || album.is_empty() || api_key.is_empty() {
        return Ok(None);
    }

    let cache_key = (artist.to_string(), album.to_string());

    // Check cache first
    {
        let cache = LASTFM_ART_CACHE.lock().expect("Cache lock poisoned");
        if let Some(cached_result) = cache.get(&cache_key) {
            log::debug!("Last.fm album art cache hit for '{}' - '{}'", artist, album);
            return Ok(cached_result.clone());
        }
    }

    // Not in cache, fetch from API
    let url = format!(
        "{}?method=album.getinfo&artist={}&album={}&api_key={}&format=json",
        LASTFM_API,
        urlencoding::encode(artist),
        urlencoding::encode(album),
        api_key
    );

    let response = attohttpc::get(&url)
        .header("User-Agent", "OSX-Scrobbler/0.3.4 ( https://github.com/aleckinnear/osx-scrobbler )")
        .send()
        .context("Failed to query Last.fm album API")?;

    if !response.is_success() {
        log::debug!("Last.fm album.getinfo returned status: {}", response.status());
        // Cache the miss
        {
            let mut cache = LASTFM_ART_CACHE.lock().expect("Cache lock poisoned");
            cache.insert(cache_key, None);
        }
        return Ok(None);
    }

    let result: LastFmAlbumResponse = response
        .json()
        .context("Failed to parse Last.fm response")?;

    // Extract the large image URL
    let image_url = if let Some(album_data) = result.album {
        if let Some(images) = album_data.image {
            // Look for large image first, fall back to extralarge or medium
            for image in &images {
                if let (Some(size), Some(url)) = (&image.size, &image.url) {
                    if size == "large" && !url.is_empty() {
                        return_and_cache(cache_key, Some(url.clone()))?;
                        return Ok(Some(url.clone()));
                    }
                }
            }
            // If no large image, try extralarge
            for image in &images {
                if let (Some(size), Some(url)) = (&image.size, &image.url) {
                    if size == "extralarge" && !url.is_empty() {
                        return_and_cache(cache_key, Some(url.clone()))?;
                        return Ok(Some(url.clone()));
                    }
                }
            }
            // Fall back to any non-empty image
            for image in &images {
                if let Some(url) = &image.url {
                    if !url.is_empty() {
                        return_and_cache(cache_key, Some(url.clone()))?;
                        return Ok(Some(url.clone()));
                    }
                }
            }
        }
        None
    } else {
        None
    };

    // Cache the result (which is None)
    {
        let mut cache = LASTFM_ART_CACHE.lock().expect("Cache lock poisoned");
        cache.insert(cache_key, image_url.clone());
    }

    Ok(image_url)
}

/// Helper to cache and return a value
fn return_and_cache(
    key: (String, String),
    value: Option<String>,
) -> Result<()> {
    let mut cache = LASTFM_ART_CACHE.lock().expect("Cache lock poisoned");
    cache.insert(key, value);
    Ok(())
}

/// Try to enrich track with album and track name from MusicBrainz
/// Also fetches Last.fm album art if available and Last.fm is configured
pub fn enrich_from_musicbrainz(track: &mut Track, config: Option<&crate::config::Config>) -> Result<()> {
    let duration = match track.duration {
        Some(d) => d,
        None => {
            log::warn!("  MusicBrainz enrichment: FAILED - no duration available");
            return Ok(());
        }
    };

    // Try barcode lookup first if UPC is available and looks like a real UPC (12-14 digits)
    if let Some(upc) = &track.upc {
        // Only process 12 or 14 digit UPCs which are likely real barcodes
        if (upc.len() == 12 || upc.len() == 14) && upc.chars().all(|c| c.is_numeric()) {

            log::info!("  Attempting MusicBrainz barcode lookup: UPC=\"{}\"", upc);
            match search_by_barcode_and_match_track(upc, duration) {
                Ok(Some((album, track_title))) => {
                    log::info!(
                        "  Identified album: \"{}\"\n  Found matching length track: \"{}\"",
                        album,
                        track_title
                    );
                    track.album = Some(album.clone());
                    track.title = track_title;
                    let output = format!("{} - {}", track.artist, track.title);
                    log::info!("  Final output: [menu bar] \"{}\"", output);

                    // Try to fetch Last.fm album art
                    if let Some(cfg) = config {
                        if let Some(ref lastfm_config) = cfg.lastfm {
                            if lastfm_config.enabled {
                                match fetch_lastfm_album_art(&track.artist, &album, &lastfm_config.api_key) {
                                    Ok(Some(art_url)) => {
                                        track.lastfm_album_art_url = Some(art_url.clone());
                                        log::info!("  Album art (Last.fm): {}", art_url);
                                    }
                                    Ok(None) => {
                                        log::debug!("  No Last.fm album art found");
                                    }
                                    Err(e) => {
                                        log::debug!("  Last.fm album art fetch failed: {}", e);
                                    }
                                }
                            }
                        }
                    }
                    return Ok(());
                }
                Ok(None) => {
                    log::debug!("  MusicBrainz barcode lookup: no match for UPC=\"{}\" at duration={}s", upc, duration);
                }
                Err(e) => {
                    log::debug!("  MusicBrainz barcode lookup: FAILED - {}", e);
                }
            }
        }
    }

    // Fall back to album name search if no UPC or barcode lookup failed
    let album_name = match &track.album {
        Some(a) if !a.is_empty() => a.clone(),
        _ => {
            log::warn!("  MusicBrainz enrichment: FAILED - no album and no UPC available");
            return Ok(());
        }
    };

    log::info!("  Attempting MusicBrainz album name lookup: album=\"{}\"", album_name);
    match search_album_and_match_track(&album_name, duration) {
        Ok(Some((album, track_title))) => {
            log::info!(
                "  Identified album: \"{}\"\n  Found matching length track: \"{}\"",
                album,
                track_title
            );
            track.album = Some(album.clone());
            track.title = track_title;
            let output = format!("{} - {}", track.artist, track.title);
            log::info!("  Final output: [menu bar] \"{}\"", output);
            if let Some(art_url) = track.idagio_album_art_url() {
                log::info!("  Album art: {}", art_url);
            } else if let Some(cfg) = config {
                // Try to fetch Last.fm album art if Idagio art not available
                if let Some(ref lastfm_config) = cfg.lastfm {
                    if lastfm_config.enabled {
                        match fetch_lastfm_album_art(&track.artist, &album, &lastfm_config.api_key) {
                            Ok(Some(art_url)) => {
                                track.lastfm_album_art_url = Some(art_url.clone());
                                log::info!("  Album art (Last.fm): {}", art_url);
                            }
                            Ok(None) => {
                                log::debug!("  No Last.fm album art found");
                            }
                            Err(e) => {
                                log::debug!("  Last.fm album art fetch failed: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Ok(None) => {
            log::warn!("  MusicBrainz album lookup: no match for album=\"{}\" at duration={}s", album_name, duration);
        }
        Err(e) => {
            log::warn!("  MusicBrainz enrichment: FAILED - {}", e);
        }
    }

    Ok(())
}

/// Search MusicBrainz by barcode and match a track by duration
/// Returns (album_title, track_title) if found
fn search_by_barcode_and_match_track(
    barcode: &str,
    duration_secs: u64,
) -> Result<Option<(String, String)>> {
    let duration_ms = duration_secs * 1000;

    let query = format!("barcode:{}", barcode);

    let url = format!(
        "{}/release?query={}&fmt=json&limit=5&inc=recordings",
        MUSICBRAINZ_API,
        urlencoding::encode(&query)
    );

    let response = attohttpc::get(&url)
        .header("User-Agent", "OSX-Scrobbler/0.3.4 ( https://github.com/aleckinnear/osx-scrobbler )")
        .send()
        .context("Failed to query MusicBrainz API")?;

    if !response.is_success() {
        return Err(anyhow::anyhow!("HTTP {}", response.status()));
    }

    let result: MBReleaseSearchResult = response
        .json()
        .context("Failed to parse MusicBrainz response")?;

    // Try to find a release with a matching track
    if let Some(releases) = result.releases {
        for release in releases {
            let album_title = match release.title {
                Some(ref t) => t.clone(),
                None => continue,
            };

            log::debug!("Checking release: {}", album_title);

            // Look through all media in the release
            if let Some(media) = release.media {
                for medium in media {
                    if let Some(tracks) = medium.tracks {
                        // Try to find a track that matches our duration
                        for track in tracks.iter() {
                            if let Some(mb_duration_ms) = track.length {
                                let duration_diff =
                                    (mb_duration_ms as i64 - duration_ms as i64).abs() as u64;

                                if duration_diff <= DURATION_TOLERANCE_MS {
                                    if let Some(track_title) = &track.title {
                                        return Ok(Some((album_title, track_title.clone())));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

/// Search MusicBrainz for an album and match a track by duration
/// Returns (album_title, track_title) if found
fn search_album_and_match_track(
    album_name: &str,
    duration_secs: u64,
) -> Result<Option<(String, String)>> {
    let duration_ms = duration_secs * 1000;

    // Search for releases with this album name
    let query = format!("release:\"{}\"", album_name.replace('"', "\\\""));

    let url = format!(
        "{}/release?query={}&fmt=json&limit=5&inc=recordings",
        MUSICBRAINZ_API,
        urlencoding::encode(&query)
    );

    let response = attohttpc::get(&url)
        .header("User-Agent", "OSX-Scrobbler/0.3.4 ( https://github.com/aleckinnear/osx-scrobbler )")
        .send()
        .context("Failed to query MusicBrainz API")?;

    if !response.is_success() {
        return Err(anyhow::anyhow!("HTTP {}", response.status()));
    }

    let result: MBReleaseSearchResult = response
        .json()
        .context("Failed to parse MusicBrainz response")?;

    // Try to find a release with a matching track
    if let Some(releases) = result.releases {
        for release in releases {
            let album_title = match release.title {
                Some(ref t) => t.clone(),
                None => continue,
            };

            log::debug!("Checking release: {}", album_title);

            // Look through all media in the release
            if let Some(media) = release.media {
                for medium in media {
                    if let Some(tracks) = medium.tracks {
                        // Try to find a track that matches our duration
                        for track in tracks.iter() {
                            if let Some(mb_duration_ms) = track.length {
                                let duration_diff =
                                    (mb_duration_ms as i64 - duration_ms as i64).abs() as u64;

                                if duration_diff <= DURATION_TOLERANCE_MS {
                                    if let Some(track_title) = &track.title {
                                        return Ok(Some((album_title, track_title.clone())));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enrich_track_with_no_album() {
        let mut track = Track {
            title: "Canon and Gigue in D major".to_string(),
            artist: "Johann Pachelbel".to_string(),
            album: None,
            duration: Some(300),
            upc: None,
            lastfm_album_art_url: None,
        };

        // Should not error, but won't enrich without album name or UPC
        let _ = enrich_from_musicbrainz(&mut track, None);
        assert_eq!(track.album, None);
    }

    #[test]
    fn test_enrich_requires_album_for_matching() {
        let mut track = Track {
            title: "Perpetual Night: 17th Century Airs and Songs".to_string(),
            artist: "".to_string(),
            album: Some("Some Album".to_string()),
            duration: Some(180),
            upc: None,
            lastfm_album_art_url: None,
        };

        // Will try to enrich, result depends on MusicBrainz data
        let _ = enrich_from_musicbrainz(&mut track, None);
        // Track will have album set (either original or enriched)
        assert!(track.album.is_some());
    }
}
