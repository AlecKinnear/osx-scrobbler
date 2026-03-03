// System tray implementation

use anyhow::{Context, Result};
use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    Icon, TrayIcon, TrayIconBuilder,
};

/// Create a simple icon for the tray
fn create_icon() -> Result<Icon> {
    // Create a simple 22x22 template icon (macOS standard size)
    // Template icons are monochrome and automatically adapt to the menu bar theme
    let width = 22;
    let height = 22;
    let mut rgba = vec![0u8; width * height * 4];

    // Draw an elegant, refined musical note (eighth note)
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            let fx = x as f32;
            let fy = y as f32;

            // Elegant note head (filled circle) - lower left
            let head_x = 5.0;
            let head_y = 16.0;
            let head_radius = 3.0;
            let head_dx = fx - head_x;
            let head_dy = fy - head_y;
            let is_note_head = (head_dx * head_dx + head_dy * head_dy) <= (head_radius * head_radius);

            // Thin, elegant stem
            let is_stem = (7..=8).contains(&x) && (4..=16).contains(&y);

            // Refined curved flag - elegant arc shape
            let is_flag = (y >= 3 && y <= 8) && (
                (x == 9 && y >= 4 && y <= 6) ||
                (x == 10 && y == 3) ||
                (x == 10 && y == 4) ||
                (x == 11 && y == 3) ||
                (x == 11 && y >= 4 && y <= 7) ||
                (x == 12 && y >= 5 && y <= 8)
            );

            if is_note_head || is_stem || is_flag {
                rgba[idx] = 0; // R - black for template icons
                rgba[idx + 1] = 0; // G
                rgba[idx + 2] = 0; // B
                rgba[idx + 3] = 255; // A - fully opaque
            } else {
                rgba[idx + 3] = 0; // Transparent background
            }
        }
    }

    log::info!(
        "Creating tray icon with {}x{} pixels (elegant musical note)",
        width,
        height
    );
    Icon::from_rgba(rgba, width as u32, height as u32)
        .context("Failed to create icon from RGBA data")
}

/// Shared state for the tray icon
#[derive(Debug, Clone, Default)]
pub struct TrayState {
    pub now_playing: Option<String>,
    pub last_scrobbled: Option<String>,
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
            .with_icon_as_template(true)
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
}
