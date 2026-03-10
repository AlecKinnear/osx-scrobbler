#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

mod config;
mod media_monitor;
mod scrobbler;
mod text_cleanup;
mod ui;

use anyhow::Result;
use backoff::{retry, ExponentialBackoff};
use clap::Parser;
use media_monitor::{MediaEvents, MediaMonitor};
use scrobbler::{lastfm_is_loved, Service, Track};
use std::{
    sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex},
    time::Duration,
};
use ui::tray::TrayManager;
use winit::event_loop::{ControlFlow, EventLoop};

/// OSX Scrobbler - Music scrobbling for macOS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Authenticate with Last.fm and obtain session key
    #[arg(long)]
    auth_lastfm: bool,

    /// Install OSX Scrobbler as a macOS app bundle in /Applications/
    #[arg(long)]
    install_app: bool,

    /// Uninstall the app bundle from /Applications/
    #[arg(long)]
    uninstall_app: bool,

    /// Force console output (show logs in terminal)
    #[arg(long)]
    console: bool,
}

static IS_HANDLING_MEDIA_CHANGE: AtomicBool = AtomicBool::new(false);

/// Build a NowPlayingInfo snapshot from the low-level MediaRemote APIs.
/// Uses catch_unwind to prevent crashes from rapid notifications during
/// fast-forwarding or track changes.
fn fetch_media_snapshot() -> Option<media_remote::NowPlayingInfo> {
    std::panic::catch_unwind(|| fetch_media_snapshot_inner())
        .unwrap_or_else(|e| {
            log::warn!("Media snapshot failed (likely rapid track change): {:?}", e);
            None
        })
}

fn fetch_media_snapshot_inner() -> Option<media_remote::NowPlayingInfo> {
    use media_remote::{
        get_now_playing_application_is_playing,
        get_now_playing_client_bundle_identifier,
        get_now_playing_client_parent_app_bundle_identifier, get_now_playing_info, InfoTypes,
        NowPlayingInfo, Number,
    };
    use std::time::SystemTime;

    let is_playing = get_now_playing_application_is_playing();
    let raw_info = get_now_playing_info()?;

    let bundle_id = get_now_playing_client_parent_app_bundle_identifier()
        .or_else(get_now_playing_client_bundle_identifier);

    let get_string = |key: &str| -> Option<String> {
        match raw_info.get(key)? {
            InfoTypes::String(s) => Some(s.clone()),
            _ => None,
        }
    };

    let get_float = |key: &str| -> Option<f64> {
        match raw_info.get(key)? {
            InfoTypes::Number(Number::Floating(f)) => Some(*f),
            InfoTypes::Number(Number::Unsigned(u)) => Some(*u as f64),
            InfoTypes::Number(Number::Signed(i)) => Some(*i as f64),
            _ => None,
        }
    };

    Some(NowPlayingInfo {
        is_playing,
        title: get_string("kMRMediaRemoteNowPlayingInfoTitle"),
        artist: get_string("kMRMediaRemoteNowPlayingInfoArtist"),
        album: get_string("kMRMediaRemoteNowPlayingInfoAlbum"),
        album_cover: None,
        elapsed_time: get_float("kMRMediaRemoteNowPlayingInfoElapsedTime"),
        duration: get_float("kMRMediaRemoteNowPlayingInfoDuration"),
        info_update_time: raw_info
            .get("kMRMediaRemoteNowPlayingInfoTimestamp")
            .and_then(|f| match f {
                InfoTypes::SystemTime(t) => Some(*t),
                _ => None,
            })
            .or(Some(SystemTime::now())),
        bundle_id,
        bundle_name: None,
        bundle_icon: None,
    })
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle Last.fm authentication if requested
    if args.auth_lastfm {
        return handle_lastfm_auth();
    }

    // Handle app installation if requested
    if args.install_app {
        return handle_install_app();
    }

    // Handle app uninstallation if requested
    if args.uninstall_app {
        return handle_uninstall_app();
    }

    // Set up logging based on environment
    setup_logging(args.console)?;

    // Load configuration (mutable for app filtering updates)
    let mut config = config::Config::load()?;
    log::info!("Configuration loaded successfully");
    log::info!("Scrobble threshold: {}%", config.scrobble_threshold);

    // Initialize scrobblers
    let scrobblers: Arc<Mutex<Vec<Service>>> = Arc::new(Mutex::new(Vec::new()));

    // Initialize Last.fm if enabled
    if let Some(ref lastfm_config) = config.lastfm {
        if lastfm_config.enabled {
            if !lastfm_config.session_key.is_empty() {
                log::info!("Last.fm scrobbler enabled");
                let service = Service::lastfm(
                    lastfm_config.api_key.clone(),
                    lastfm_config.api_secret.clone(),
                    lastfm_config.session_key.clone(),
                );
                scrobblers.lock().unwrap().push(service);
            } else {
                log::warn!("Last.fm is enabled but session_key is not set. Skipping Last.fm.");
            }
        }
    }

    // Initialize ListenBrainz instances if enabled
    for lb_config in &config.listenbrainz {
        if lb_config.enabled {
            log::info!("ListenBrainz scrobbler enabled: {}", lb_config.name);
            let name = lb_config.name.clone();
            let token = lb_config.token.clone();
            let api_url = lb_config.api_url.clone();

            log::debug!(
                "ListenBrainz ({}): Attempting authentication with token: {}...",
                name,
                &token[..token.len().min(10)]
            );
            match Service::listenbrainz(name.clone(), token.clone(), api_url.clone()) {
                Ok(service) => {
                    log::info!("ListenBrainz ({}): Authentication successful", name);
                    scrobblers.lock().unwrap().push(service);
                }
                Err(e) => log::warn!("Failed to initialize ListenBrainz ({}): {}. Check your token at https://listenbrainz.org/settings/", name, e),
            }
        }
    }

    if scrobblers.lock().unwrap().is_empty() {
        log::warn!(
            "No scrobblers enabled! The app will monitor media but won't scrobble anywhere."
        );
    }

    // Initialize text cleaner
    let text_cleaner = text_cleanup::TextCleaner::new(&config.cleanup);
    if config.cleanup.enabled {
        log::info!(
            "Text cleanup enabled with {} patterns",
            config.cleanup.patterns.len()
        );
    }

    // Initialize media monitor
    let mut media_monitor = MediaMonitor::new(config.scrobble_threshold, text_cleaner);

    // Create album art communication channel
    let (_album_art_tx, album_art_rx) = ui::album_art::create_album_art_channel();

    log::info!("Starting OSX Scrobbler...");

    // Define user events for tray menu actions and media notifications
    #[derive(Debug, Clone)]
    enum UserEvent {
        TrayQuit,
        TrayLove,
        TrayShowAlbumArt(()),
        MediaStateChanged,
        LoveStatusUpdated(bool),
    }

    // Run event loop on main thread for tray icon
    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .build()
        .expect("Failed to create event loop");

    // Initialize system tray
    // Must be initialized after EventLoop to ensure NSApplication is set up correctly by winit
    let mut tray = TrayManager::new()?;
    log::info!("System tray initialized");

    // Register for macOS media notifications (event-driven, no polling)
    use media_remote::{add_observer, register_for_now_playing_notifications, Notification};
    register_for_now_playing_notifications();
    log::info!("Media notifications registered (event-driven)");

    // Set up observers that wake the event loop on media changes
    let media_proxy = event_loop.create_proxy();
    let media_proxy2 = event_loop.create_proxy();
    let media_proxy3 = event_loop.create_proxy();
    let _observer_info = add_observer(Notification::NowPlayingInfoDidChange, move || {
        let _ = media_proxy.send_event(UserEvent::MediaStateChanged);
    });
    let _observer_app = add_observer(Notification::NowPlayingApplicationDidChange, move || {
        let _ = media_proxy2.send_event(UserEvent::MediaStateChanged);
    });
    let _observer_state = add_observer(
        Notification::NowPlayingApplicationIsPlayingDidChange,
        move || {
            let _ = media_proxy3.send_event(UserEvent::MediaStateChanged);
        },
    );

    // Spawn minimal thread to forward tray menu events to main event loop
    let quit_item_id = tray.quit_item.id().clone();
    let love_item_id = tray.love_item_id();
    let now_playing_item_id = tray.now_playing_item_id();
    let event_proxy_for_menu = event_loop.create_proxy();
    std::thread::spawn(move || {
        use tray_icon::menu::MenuEvent;
        loop {
            if let Ok(event) = MenuEvent::receiver().recv() {
                if event.id == quit_item_id {
                    log::info!("Quit menu item clicked");
                    let _ = event_proxy_for_menu.send_event(UserEvent::TrayQuit);
                } else if event.id == love_item_id {
                    log::info!("Love menu item clicked");
                    let _ = event_proxy_for_menu.send_event(UserEvent::TrayLove);
                } else if event.id == now_playing_item_id {
                    log::info!("Now Playing menu item clicked");
                    // We don't have access to tray here, send event to get URL from main thread
                    let _ = event_proxy_for_menu.send_event(UserEvent::TrayShowAlbumArt(()));
                }
            }
        }
    });

    // Configure app to be menu bar only (no dock icon)
    // MUST be set AFTER EventLoop creation as winit creates NSApplication
    use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy};
    use objc2_foundation::MainThreadMarker;
    unsafe {
        let mtm = MainThreadMarker::new_unchecked();
        let app = NSApplication::sharedApplication(mtm);
        app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);
    }
    log::info!("Set activation policy to Accessory (no dock icon)");

    // Send an initial media check event so we detect what's already playing.
    // We can't call fetch_media_snapshot() before the event loop starts because
    // the low-level MediaRemote APIs need the run loop to be active.
    let startup_proxy = event_loop.create_proxy();
    let _ = startup_proxy.send_event(UserEvent::MediaStateChanged);

    #[allow(deprecated)]
    event_loop.run(move |event, elwt| {
        // Handle user events (tray menu actions)
        match event {
            winit::event::Event::UserEvent(UserEvent::TrayQuit) => {
                log::info!("OSX Scrobbler shutting down");
                elwt.exit();
                return;
            }
            winit::event::Event::UserEvent(UserEvent::LoveStatusUpdated(is_loved)) => {
                let text = if is_loved { "♥️ Loved" } else { "🤍 Love" };
                if let Err(e) = tray.set_love_button_state(text, true) {
                    log::error!("Failed to update love button text: {}", e);
                }
                return;
            }
            winit::event::Event::UserEvent(UserEvent::TrayLove) => {
                // Love the currently playing track
                if let Some(track_str) = tray.current_track() {
                    // Parse "Artist - Title" format to extract track info
                    if let Some(pos) = track_str.find(" - ") {
                        let artist = track_str[..pos].to_string();
                        let title = track_str[pos + 3..].to_string();
                        let track = Track {
                            title,
                            artist,
                            album: None,
                            duration: None,
                            upc: None,
                            lastfm_album_art_url: None,
                        };

                        // Get Last.fm credentials for loving
                        if let Some(ref lastfm_config) = config.lastfm {
                            let scrobblers = Arc::clone(&scrobblers);
                            let config = config.clone();
                            let event_proxy = elwt.create_proxy();

                            std::thread::spawn(move || {
                                let mut success = false;
                                if let Some(ref lastfm_config) = config.lastfm {
                                    for scrobbler in scrobblers.lock().unwrap().iter() {
                                        if let Err(e) = scrobbler.love(
                                            &track,
                                            &lastfm_config.api_key,
                                            &lastfm_config.api_secret,
                                            &lastfm_config.session_key,
                                        ) {
                                            log::error!("Failed to love track: {}", e);
                                        } else {
                                            log::info!("Track loved successfully");
                                            success = true;
                                        }
                                    }
                                }
                                if success {
                                    let _ = event_proxy.send_event(UserEvent::LoveStatusUpdated(true));
                                }
                            });
                        } else {
                            log::warn!("Last.fm not configured, cannot love track");
                        }
                    }
                } else {
                    log::warn!("No track currently playing to love");
                }
                return;
            }
            winit::event::Event::UserEvent(UserEvent::TrayShowAlbumArt(_)) => {
                // Show album art for currently playing track
                if let Some(url) = tray.album_art_url() {
                    log::info!("User clicked: Opening album art window for: {}", url);
                    // Show full-size album art window
                    if let Err(e) = ui::album_art::fetch_and_display_album_art(&url) {
                        log::error!("Failed to show album art: {}", e);
                    }
                } else {
                    log::warn!("No album art available for current track - not yet enriched?");
                }
                return;
            }
            _ => {}
        }

        // Check for album art updates from enrichment thread (non-blocking)
        while let Ok(album_art_update) = album_art_rx.try_recv() {
            log::info!("Received album art URL from enrichment: {}", album_art_update.url);
            if let Err(e) = tray.update_album_art(Some(album_art_update.url.clone())) {
                log::error!("Failed to update tray album art: {}", e);
            }
            if let Err(e) = ui::album_art::fetch_and_display_album_art(&album_art_update.url) {
                log::error!("Failed to fetch and display album art: {}", e);
            }
        }

        // Determine what triggered this wakeup and process accordingly
        let media_events = match event {
            winit::event::Event::UserEvent(UserEvent::MediaStateChanged) => {
                // Use an atomic bool to prevent re-entrant calls to the media snapshot logic,
                // which can cause crashes with the underlying MediaRemote framework on rapid
                // track changes. This effectively debounces the storm of notifications.
                if IS_HANDLING_MEDIA_CHANGE
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    let snapshot = fetch_media_snapshot();
                    let result = media_monitor.handle_media_change(snapshot, &config.app_filtering);
                    IS_HANDLING_MEDIA_CHANGE.store(false, Ordering::Release);
                    result
                } else {
                    log::debug!("Ignoring concurrent MediaStateChanged event.");
                    Ok(Default::default())
                }
            }
            winit::event::Event::NewEvents(
                winit::event::StartCause::ResumeTimeReached { .. },
            ) => {
                // Scrobble deadline reached
                media_monitor.check_scrobble()
            }
            _ => Ok(Default::default()),
        };

        // Process media events
        match media_events {
            Ok(events) => {
                // Handle now_playing event
                if let Some((mut track, ref bundle_id, should_scrobble)) = events.now_playing {
                    log::info!(
                        "Now playing: {} - {} (album: {}) from {:?}",
                        track.artist,
                        track.title,
                        track.album.as_deref().unwrap_or("Unknown"),
                        bundle_id
                    );

                    let track_str = format!("{} - {}", track.artist, track.title);
                    let track_str = if track.artist.is_empty() {
                        track.title.clone()
                    } else {
                        format!("{} - {}", track.artist, track.title)
                    };

                    if let Err(e) = tray.update_now_playing(Some(track_str)) {
                        log::error!("Failed to update tray now playing: {}", e);
                    }

                    // Display IDAGIO album art immediately
                    if let Some(idagio_art_url) = track.idagio_album_art_url() {
                        log::info!("IDAGIO album art available: {}", idagio_art_url);
                        if let Err(e) = tray.update_album_art(Some(idagio_art_url.clone())) {
                            log::error!("Failed to update tray album art: {}", e);
                        }
                        if let Err(e) = ui::album_art::fetch_and_display_album_art(&idagio_art_url) {
                            log::debug!("Failed to fetch IDAGIO album art: {}", e);
                        }
                    }

                    // Send now playing to scrobblers
                    if should_scrobble {
                        let scrobblers = Arc::clone(&scrobblers);
                        let track = track.clone();
                        std::thread::spawn(move || {
                            for scrobbler in scrobblers.lock().unwrap().iter() {
                                let backoff = ExponentialBackoff {
                                    max_elapsed_time: Some(Duration::from_secs(3)),
                                    ..Default::default()
                                };
                                let result = retry(backoff, || {
                                    scrobbler
                                        .now_playing(&track)
                                        .map_err(backoff::Error::transient)
                                });
                                if let Err(e) = result {
                                    log::error!("Failed to send now playing after retries: {}", e);
                                }
                            }
                        });
                    }

                    // Update the "Love" button state.
                    if !should_scrobble {
                        // This is an Idagio track, which is not scrobbled.
                        if let Err(e) = tray.set_love_button_state("Idagio - cannot be scrobbled", false) {
                            log::error!("Failed to update love button state: {}", e);
                        }
                    } else {
                        // For scrobble-able tracks, reset the button to its default state
                        // while we check the loved status asynchronously.
                        let _ = tray.set_love_button_state("🤍 Love", true);

                        // Asynchronously check if the track is loved on Last.fm
                        if let Some(ref lastfm_config) = config.lastfm {
                            if !lastfm_config.session_key.is_empty() {
                                let track_clone = track.clone();
                                let api_key = lastfm_config.api_key.clone();
                                let api_secret = lastfm_config.api_secret.clone();
                                let session_key = lastfm_config.session_key.clone();
                                let event_proxy = elwt.create_proxy();

                                std::thread::spawn(move || {
                                    if let Ok(is_loved) = lastfm_is_loved(&track_clone, &api_key, &api_secret, &session_key) {
                                        let _ = event_proxy.send_event(UserEvent::LoveStatusUpdated(is_loved));
                                    }
                                });
                            }
                        }
                    }
                }

                // Handle scrobble event
                if let Some((track, timestamp, ref bundle_id)) = events.scrobble {
                    log::info!(
                        "Scrobble: {} - {} at {} from {:?}",
                        track.artist,
                        track.title,
                        timestamp.format("%Y-%m-%d %H:%M:%S"),
                        bundle_id
                    );

                    let scrobblers = Arc::clone(&scrobblers);
                    let track_clone = track.clone();
                    std::thread::spawn(move || {
                        for scrobbler in scrobblers.lock().unwrap().iter() {
                            if let Service::ListenBrainz { name, .. } = scrobbler {
                                if track_clone.artist.is_empty() || track_clone.artist.len() < 3 {
                                    log::debug!(
                                        "Skipping ListenBrainz ({}): artist '{}' too short",
                                        name,
                                        track_clone.artist
                                    );
                                    continue;
                                }
                            }

                            let backoff = ExponentialBackoff {
                                max_elapsed_time: Some(Duration::from_secs(10)),
                                ..Default::default()
                            };
                            let result = retry(backoff, || {
                                scrobbler
                                    .scrobble(&track_clone, timestamp)
                                    .map_err(backoff::Error::transient)
                            });
                            if let Err(e) = result {
                                log::error!("Failed to scrobble after retries: {}", e);
                            }
                        }
                    });

                    let track_str = format!("{} - {}", track.artist, track.title);
                    if let Err(e) = tray.update_last_scrobbled(Some(track_str)) {
                        log::error!("Failed to update tray last scrobbled: {}", e);
                    }
                }

                // Handle unknown app event
                if let Some(ref bundle_id) = events.unknown_app {
                    use ui::app_dialog::{show_app_prompt, AppChoice};

                    log::info!("Prompting user for app: {}", bundle_id);
                    let choice = show_app_prompt(bundle_id);

                    match choice {
                        AppChoice::Allow => {
                            log::info!("User allowed app: {}", bundle_id);
                            if !config.app_filtering.allowed_apps.contains(bundle_id) {
                                config.app_filtering.allowed_apps.push(bundle_id.clone());
                                if let Err(e) = config.save() {
                                    log::error!("Failed to save config: {}", e);
                                } else {
                                    log::info!("Added {} to allowed apps", bundle_id);
                                }
                            }
                        }
                        AppChoice::Ignore => {
                            log::info!("User ignored app: {}", bundle_id);
                            if !config.app_filtering.ignored_apps.contains(bundle_id) {
                                config.app_filtering.ignored_apps.push(bundle_id.clone());
                                if let Err(e) = config.save() {
                                    log::error!("Failed to save config: {}", e);
                                } else {
                                    log::info!("Added {} to ignored apps", bundle_id);
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                log::error!("Error processing media state: {}", e);
            }
        }

        // Set control flow: sleep until scrobble deadline, or wait indefinitely
        if let Some(deadline) = media_monitor.next_scrobble_deadline() {
            elwt.set_control_flow(ControlFlow::WaitUntil(deadline));
        } else {
            elwt.set_control_flow(ControlFlow::Wait);
        }
    })?;

    log::info!("Application exited cleanly");
    Ok(())
}

/// Set up logging based on whether we're running from a terminal
fn setup_logging(force_console: bool) -> Result<()> {
    use std::io::Write;

    // Check if stdout is a TTY (terminal)
    let is_terminal = atty::is(atty::Stream::Stdout);
    let use_console = force_console || is_terminal;

    if use_console {
        // Running from terminal - log to console
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    } else {
        // Not running from terminal (e.g., launched via Spotlight)
        // Log to file instead
        let log_dir = dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join("Library")
            .join("Logs");

        std::fs::create_dir_all(&log_dir)?;
        let log_file = log_dir.join("osx-scrobbler.log");

        let target = Box::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file)?,
        );

        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .target(env_logger::Target::Pipe(target))
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] {} - {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.args()
                )
            })
            .init();

        // Log where we're logging to (this will go to the file)
        log::info!("OSX Scrobbler started (logging to {})", log_file.display());
    }

    Ok(())
}

/// Handle Last.fm authentication flow
fn handle_lastfm_auth() -> Result<()> {
    // Load current config
    let mut config = config::Config::load()?;

    // Check if Last.fm is configured
    let lastfm_config = config
        .lastfm
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Last.fm is not configured in config file"))?;

    if lastfm_config.api_key.is_empty() || lastfm_config.api_secret.is_empty() {
        anyhow::bail!(
            "Last.fm API key and secret must be set in config file before authenticating"
        );
    }

    println!("Last.fm Authentication");
    println!("======================\n");
    println!("API Key: {}", lastfm_config.api_key);
    println!("API Secret: {}\n", lastfm_config.api_secret);

    // Run authentication flow
    let session_key =
        scrobbler::lastfm_auth::authenticate(&lastfm_config.api_key, &lastfm_config.api_secret)?;

    println!("Session Key: {}\n", session_key);

    // Update config with session key
    if let Some(ref mut lastfm) = config.lastfm {
        lastfm.session_key = session_key;
        lastfm.enabled = true;
    }

    // Save config
    config.save()?;

    println!("Configuration updated successfully!");
    println!("Last.fm is now enabled and ready to use.");
    println!("\nYou can now run the scrobbler normally.");

    Ok(())
}

/// Info.plist template for macOS app bundle
const INFO_PLIST_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>OSX Scrobbler</string>
    <key>CFBundleDisplayName</key>
    <string>OSX Scrobbler</string>
    <key>CFBundleIdentifier</key>
    <string>com.osxscrobbler</string>
    <key>CFBundleVersion</key>
    <string>{VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>{VERSION}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>osx-scrobbler</string>
    <key>CFBundleIconFile</key>
    <string>UniversalScrobbler</string>
    <key>LSUIElement</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>"#;

/// Icon filename for the app bundle
const ICON_FILENAME: &str = "UniversalScrobbler.icns";

/// Copy the .icns icon file to the app bundle resources, if available.
fn create_app_icon(resources_dir: &std::path::Path) -> Result<()> {
    use anyhow::Context;
    use std::fs;
    use std::path::Path;

    // Path to the source icon in the project resources
    let source_icon = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/UniversalScrobbler.icns"
    );

    if !Path::new(source_icon).exists() {
        log::warn!(
            "App icon {} not found; using default macOS app icon instead",
            source_icon
        );
        return Ok(());
    }

    // Destination in the app bundle
    let dest_icon = resources_dir.join(ICON_FILENAME);

    // Copy the .icns file for the app icon
    fs::copy(source_icon, &dest_icon).context("Failed to copy icon file to app bundle")?;

    log::info!(
        "Copied app icon from {} to {}",
        source_icon,
        dest_icon.display()
    );

    Ok(())
}

/// Install OSX Scrobbler as a macOS app bundle in /Applications/
fn handle_install_app() -> Result<()> {
    use std::fs;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    println!("OSX Scrobbler App Bundle Installer");
    println!("===================================\n");

    let app_name = "OSX Scrobbler.app";
    let app_path = std::path::Path::new("/Applications").join(app_name);
    let contents_dir = app_path.join("Contents");
    let macos_dir = contents_dir.join("MacOS");

    // Check if app already exists
    if app_path.exists() {
        print!(
            "App bundle already exists at {}. Overwrite? [y/N] ",
            app_path.display()
        );
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Installation cancelled.");
            return Ok(());
        }

        println!("Removing existing app bundle...");
        fs::remove_dir_all(&app_path)?;
    }

    // Create directory structure
    println!("Creating app bundle structure...");
    match fs::create_dir_all(&macos_dir) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("\n❌ Permission denied creating app bundle.");
            eprintln!("\nTry running with sudo:");
            eprintln!("  sudo osx-scrobbler --install-app\n");
            return Err(e.into());
        }
        Err(e) => return Err(e.into()),
    }

    // Create Resources directory for app icon
    let resources_dir = contents_dir.join("Resources");
    fs::create_dir_all(&resources_dir)?;

    // Get current binary path
    let current_exe = std::env::current_exe()?;
    let target_binary = macos_dir.join("osx-scrobbler");

    // Copy binary
    println!("Copying binary to app bundle...");
    fs::copy(&current_exe, &target_binary)?;

    // Set executable permissions
    println!("Setting executable permissions...");
    let mut perms = fs::metadata(&target_binary)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&target_binary, perms)?;

    // Create app icon
    println!("Creating app icon...");
    create_app_icon(&resources_dir)?;

    // Create Info.plist
    println!("Creating Info.plist...");
    let version = env!("CARGO_PKG_VERSION");
    let info_plist = INFO_PLIST_TEMPLATE.replace("{VERSION}", version);
    let plist_path = contents_dir.join("Info.plist");
    fs::write(&plist_path, info_plist)?;

    println!("\n✅ Successfully installed OSX Scrobbler!");
    println!("\nApp bundle location:");
    println!("  {}", app_path.display());
    println!("\nTo launch the app:");
    println!("  open \"{}\"\n", app_path.display());
    println!("Or simply open it from Finder.\n");
    println!("💡 To start at login:");
    println!("  System Settings → General → Login Items → Add \"OSX Scrobbler\"\n");

    Ok(())
}

/// Uninstall the app bundle from /Applications/
fn handle_uninstall_app() -> Result<()> {
    use std::fs;
    use std::io::Write;

    println!("OSX Scrobbler App Bundle Uninstaller");
    println!("====================================\n");

    let app_name = "OSX Scrobbler.app";
    let app_path = std::path::Path::new("/Applications").join(app_name);

    // Check if app exists
    if !app_path.exists() {
        println!("❌ App bundle not found at {}", app_path.display());
        println!("\nNothing to uninstall.");
        return Ok(());
    }

    // Confirm with user
    print!("Remove app bundle at {}? [y/N] ", app_path.display());
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Uninstallation cancelled.");
        return Ok(());
    }

    // Remove app bundle
    println!("\nRemoving app bundle...");
    match fs::remove_dir_all(&app_path) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            eprintln!("\n❌ Permission denied removing app bundle.");
            eprintln!("\nTry running with sudo:");
            eprintln!("  sudo osx-scrobbler --uninstall-app\n");
            return Err(e.into());
        }
        Err(e) => return Err(e.into()),
    }

    println!("\n✅ Successfully uninstalled OSX Scrobbler!");
    println!("\nThe app bundle has been removed from /Applications/");
    println!("The binary at ~/.cargo/bin/osx-scrobbler is still available.\n");

    Ok(())
}
