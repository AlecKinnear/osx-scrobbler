// System tray implementation

use anyhow::{Context, Result};
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

/// Load the menu bar icon (32x32 for retina 2x display)
/// Icon is embedded at compile time from the universal scrobbler iconset
fn create_icon() -> Result<Icon> {
    // Icon is embedded at compile time
    let icon_data = include_bytes!("../../../universalescrobbler.iconset/icon_32.png");

    log::info!("Loading embedded menu bar icon (32x32)");

    // Parse the PNG data and create an Icon
    let reader = image::ImageReader::new(std::io::Cursor::new(icon_data))
        .with_guessed_format()
        .context("Failed to detect icon format")?;

    let image = reader.decode().context("Failed to decode icon image")?;
    let rgba_image = image.to_rgba8();
    let (width, height) = rgba_image.dimensions();

    Icon::from_rgba(rgba_image.to_vec(), width, height)
        .context("Failed to create icon from image data")
}

/// Shared state for the tray icon
#[derive(Debug, Clone, Default)]
pub struct TrayState {
    pub now_playing: Option<String>,
    pub last_scrobbled: Option<String>,
    pub album_art_url: Option<String>,
}

/// System tray manager
pub struct TrayManager {
    _tray_icon: TrayIcon,
    state: TrayState,
    #[allow(dead_code)]
    menu: Menu,
    now_playing_item: MenuItem,
    love_item: MenuItem,
    last_scrobble_item: MenuItem,
    pub quit_item: MenuItem,
}

impl TrayManager {
    /// Create a new tray manager
    pub fn new() -> Result<Self> {
        let state = TrayState::default();

        // Create menu items
        let now_playing_item = MenuItem::new("Now Playing: None", false, None);
        let love_item = MenuItem::new("🤍 Love", false, None);
        let last_scrobble_item = MenuItem::new("Last Scrobbled: None", false, None);
        let separator = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new("Quit", true, None);

        // Build menu
        let menu = Menu::new();
        menu.append(&now_playing_item)
            .context("Failed to add now playing item")?;
        menu.append(&love_item).context("Failed to add love item")?;
        menu.append(&last_scrobble_item)
            .context("Failed to add last scrobble item")?;
        menu.append(&separator).context("Failed to add separator")?;
        menu.append(&quit_item).context("Failed to add quit item")?;

        // Create tray icon
        let icon = create_icon()?;
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip("OSX Scrobbler")
            .with_icon(icon)
            .build()
            .context("Failed to create tray icon")?;

        log::info!("Tray icon created successfully");

        Ok(Self {
            _tray_icon: tray_icon,
            state,
            menu,
            now_playing_item,
            love_item,
            last_scrobble_item,
            quit_item,
        })
    }

    /// Get the love item ID for event handling
    pub fn love_item_id(&self) -> tray_icon::menu::MenuId {
        self.love_item.id().clone()
    }

    /// Get the now playing item ID for event handling
    pub fn now_playing_item_id(&self) -> tray_icon::menu::MenuId {
        self.now_playing_item.id().clone()
    }

    /// Get the currently playing track
    pub fn current_track(&self) -> Option<String> {
        self.state.now_playing.clone()
    }

    /// Update love button status
    pub fn update_love_status(&mut self, is_loved: bool) -> Result<()> {
        let text = if is_loved {
            "♥️ Loved"
        } else {
            "🤍 Love"
        };
        self.love_item.set_text(text);
        Ok(())
    }

    /// Update the now playing display
    pub fn update_now_playing(&mut self, track: Option<String>) -> Result<()> {
        let text = if let Some(ref t) = track {
            format!("Now Playing: {}", t)
        } else {
            "Now Playing: None".to_string()
        };

        self.now_playing_item.set_text(text);
        self.state.now_playing = track.clone();

        // Enable/disable love button based on whether there's a track
        let is_track_playing = track.is_some();
        self.love_item.set_enabled(is_track_playing);

        Ok(())
    }

    /// Update the last scrobbled display
    pub fn update_last_scrobbled(&mut self, track: Option<String>) -> Result<()> {
        let text = if let Some(ref t) = track {
            format!("Last Scrobbled: {}", t)
        } else {
            "Last Scrobbled: None".to_string()
        };

        self.last_scrobble_item.set_text(text);
        self.state.last_scrobbled = track;

        Ok(())
    }

    /// Update the album art URL for the currently playing track
    #[allow(dead_code)]
    pub fn update_album_art(&mut self, url: Option<String>) {
        self.state.album_art_url = url;
    }

    /// Get the album art URL for the currently playing track
    pub fn album_art_url(&self) -> Option<String> {
        self.state.album_art_url.clone()
    }
}
