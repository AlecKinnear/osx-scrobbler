// Album art management and display
// Handles receiving album art URLs from enrichment thread and displaying them

use std::sync::mpsc::{channel, Receiver, Sender};

/// Album art update message from enrichment thread
#[derive(Debug, Clone)]
pub struct AlbumArtUpdate {
    pub url: String,
}

/// Create a channel for album art updates
pub fn create_album_art_channel() -> (Sender<AlbumArtUpdate>, Receiver<AlbumArtUpdate>) {
    channel()
}

/// Fetch and display album art from URL
pub fn fetch_and_display_album_art(url: &str) -> anyhow::Result<()> {
    log::info!("Fetching album art: {}", url);
    
    // Fetch image data
    let response = attohttpc::get(url)
        .header("User-Agent", "OSX-Scrobbler/0.3.4")
        .send()
        .map_err(|e| anyhow::anyhow!("Failed to fetch album art: {}", e))?;
    
    if !response.is_success() {
        anyhow::bail!("Album art fetch failed with status: {}", response.status());
    }
    
    let image_data = response.bytes()
        .map_err(|e| anyhow::anyhow!("Failed to read album art data: {}", e))?;
    
    // Display in native window
    super::album_art_window::show_album_art_image(&image_data)?;
    
    Ok(())
}
