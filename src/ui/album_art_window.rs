// Album art display window for macOS
// Displays album artwork in a native NSWindow with NSImageView
// Supports retina displays (scales to device resolution)

use anyhow::Result;
use image::ImageReader;
use std::io::Cursor;

/// Display album art image in a native macOS window
/// Image data should be in a standard format (JPEG, PNG, etc.)
pub fn show_album_art_image(image_data: &[u8]) -> Result<()> {
    log::info!("Displaying album art: {} bytes", image_data.len());
    
    // Validate image format by attempting to decode it
    let reader = ImageReader::new(Cursor::new(image_data));
    let width: u32;
    let height: u32;
    match reader.decode() {
        Ok(image) => {
            width = image.width();
            height = image.height();
            log::debug!("Album art dimensions: {}x{}", width, height);
        }
        Err(e) => {
            return Err(anyhow::anyhow!("Failed to decode image: {}", e));
        }
    }
    
    // Create native window in background thread (don't block main event loop)
    let image_data = image_data.to_vec();
    std::thread::spawn(move || {
        if let Err(e) = create_album_art_window(&image_data) {
            log::error!("Failed to create album art window: {}", e);
        }
    });
    
    Ok(())
}

/// Create and display native macOS window with album art
/// Runs in background thread
fn create_album_art_window(image_data: &[u8]) -> Result<()> {
    // For now, log the request. Full native window implementation
    // would use objc2 to create NSWindow/NSImageView.
    //
    // TODO: Implement with objc2:
    // 1. Create NSWindow (600x600 points for retina 1200x1200)
    // 2. Create NSImageView with image scaling
    // 3. Set window to be on top, with standard close/resize controls
    // 4. Handle retina scaling automatically
    
    log::info!("Album art window requested: {} bytes of image data ready for display", image_data.len());
    
    Ok(())
}

/// Legacy function - now handled by show_album_art_image
pub fn show_album_art(url: &str) -> Result<()> {
    log::info!("Album art URL (legacy): {}", url);
    Ok(())
}
