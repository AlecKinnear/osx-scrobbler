// System tray implementation

use anyhow::{Context, Result};
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

/// Load the menu bar icon (e.g. a quarter-note) from an embedded PNG.
///
/// The PNG file is stored in the repository at `resources/iconset/icon_32.png`
/// and embedded at compile time.
fn create_icon() -> Result<Icon> {
    // Icon is embedded at compile time
    let icon_data = include_bytes!("../../resources/iconset/icon_32.png");

    log::info!("Loading embedded menu bar icon from resources/iconset/icon_32.png");

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
    album_art_item: MenuItem,
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
        // This item will eventually hold the album art image.
        // For now, it's a non-interactive placeholder.
        let album_art_item = MenuItem::new("", false, None);

        let now_playing_item = MenuItem::new("♩ Now Playing: None", false, None);
        let love_item = MenuItem::new("🤍 Love", false, None);
        let last_scrobble_item = MenuItem::new("♩ Last Scrobbled: None", false, None);
        let separator = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new("Quit", true, None);

        // Build menu
        let menu = Menu::new();
        menu.append(&album_art_item)
            .context("Failed to add album art item")?;
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
            .with_tooltip("♩ OSX Scrobbler")
            .with_icon(icon)
            .build()
            .context("Failed to create tray icon")?;

        // On macOS, treat the icon as a template so the system
        // automatically renders it appropriately in light/dark mode.
        tray_icon.set_icon_as_template(true);

        log::info!("Tray icon created successfully");

        Ok(Self {
            _tray_icon: tray_icon,
            state,
            menu,
            album_art_item,
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

    /// Set custom text and enabled state for the love button
    pub fn set_love_button_state(&mut self, text: &str, enabled: bool) -> Result<()> {
        self.love_item.set_text(text);
        self.love_item.set_enabled(enabled);
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
    pub fn update_album_art(&mut self, url: Option<String>) -> Result<()> {
        self.state.album_art_url = url.clone();
        if let Some(ref art_url) = url {
            log::debug!("Updated tray album art URL: {}", art_url);
            // TODO: In a future step, we will fetch the image from the URL,
            // resize it, and set it as the icon for the `album_art_item`.
            // For now, we just show a placeholder text.
            self.album_art_item.set_text("🖼️ Album Art Available");
            self.album_art_item.set_enabled(true);
        } else {
            log::debug!("Cleared tray album art URL");
            self.album_art_item.set_text("");
            self.album_art_item.set_enabled(false);
        }

        Ok(())
    }

    /// Get the album art URL for the currently playing track
    pub fn album_art_url(&self) -> Option<String> {
        self.state.album_art_url.clone()
    }
}
