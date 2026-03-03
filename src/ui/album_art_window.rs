// Album art display window for Idagio tracks
//
// Infrastructure for displaying album artwork in a native macOS window.
// Currently logs the request; full implementation is a TODO.
//
// TODO: Implement native NSWindow with NSImageView to display album artwork:
// Implementation notes:
// - Use MainThreadMarker with unsafe { MainThreadMarker::new_unchecked() }
// - Create NSWindow with NSWindowStyleMask::Titled | Closable | Resizable
// - Add NSImageView to window's contentView
// - Fetch image via attohttpc (non-blocking, run in spawned thread)
// - Set image scaling to NSImageScaleProportionallyUpOrDown for crisp retina display
// - Window size: 600x600 points (~1200x1200 pixels on 2x retina for album art)
// - Make window key and order front: msg_send![&window, makeKeyAndOrderFront: ...]
//
// Future enhancements:
// - Image caching (optional, would reduce API calls)
// - Window pooling (reuse same window instead of creating new ones)
// - Add metadata text (track name, artist) below image
// - Double-click to open in default image viewer

use anyhow::Result;

/// Opens a window displaying the album art from the given URL
/// 
/// Future implementation will:
/// 1. Fetch image data from the URL (async, non-blocking)
/// 2. Create a native NSWindow with NSImageView
/// 3. Scale image for retina display (2x)
/// 4. Keep window on top, with close/resize controls
pub fn show_album_art(url: &str) -> Result<()> {
    log::info!("Album art requested: {}", url);
    
    // TODO: Implement full native macOS window
    // For now, just log that album art was requested
    // The URL is captured and available for display
    
    Ok(())
}
