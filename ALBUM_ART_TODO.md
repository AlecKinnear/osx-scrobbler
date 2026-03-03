# Album Art Display - Implementation TODO

## Current State
Album art URLs are **collected** but **not displayed**:

1. ✅ IDAGIO tracks: Catalog ID → direct image URL construction
2. ✅ Last.fm lookup: Artist/album → Last.fm API → image URL
3. ❌ **Not displayed in UI**: URLs are set on track in background thread but tray never updated

## What's Needed

### 1. Tray Manager Integration
**File:** `src/ui/tray.rs`

Currently has infrastructure:
- `album_art_url: Option<String>` field in state
- `update_album_art(url)` method (unused)

But it's never called. Need to:
- Export a channel/sender for UI updates from enrichment thread
- Call `tray.update_album_art(url)` after enrichment completes

### 2. Menu Item with Album Art
**File:** `src/ui/tray.rs:build_menu()`

Currently shows:
```
▶ Now Playing: Artist - Title
◀ Last Scrobbled: Artist - Title
```

Need to:
- Add album art thumbnail (64x64?) above the "Now Playing" item
- Display when album_art_url is available
- Hide when not available

### 3. Full-Size Album Art Window
**File:** `src/ui/album_art_window.rs`

Currently just logs. Need to:
- Create native NSWindow (macOS native window)
- Display album image fetched from URL
- Show on click of artist/track name
- Window: 600x600 points (fits most displays, retina 1200x1200)
- Keep window on top, allow close/resize

### 4. Image Fetching
Need to:
- Fetch image from URL (attohttpc)
- Convert to NSImage for display
- Run in background (don't block UI thread)

## Architecture

```
Enrichment Thread
    ↓
Track with album_art_url
    ↓
Send update to TrayManager
    ↓
TrayManager.update_album_art(url)
    ↓
Rebuild menu with album art thumbnail
    ↓
User clicks → show_album_art(url)
    ↓
Fetch image → Create NSWindow → Display
```

## Scope Estimation

- Tray integration: ~30 lines (channel setup)
- Menu item rendering: ~50 lines (tray-icon API)
- Album art window: ~100 lines (objc2 or native-windows-gui)
- Image handling: ~30 lines (fetch + convert to NSImage)

**Total:** ~200 lines of new code

## Dependencies Already Available

- `attohttpc` - for image fetching
- `image` crate - for image processing
- `objc2` - for native macOS integration (already in Cargo.toml)
- `tray-icon` - for menu rendering

## Known Issues to Solve

1. **Thread communication:** Enrichment runs in spawned thread, needs to notify tray manager
   - Solution: Use `std::sync::mpsc` channel or `tokio` channel

2. **Image format:** NSImage requires macOS-native format
   - Solution: Use `objc2` to create NSImage from bytes

3. **Menu refresh:** tray-icon doesn't automatically refresh
   - Solution: May need to rebuild entire menu on album art update

## Testing Plan

1. Play IDAGIO track with UPC
2. Wait for enrichment to complete (~5s)
3. Verify album art appears in menu
4. Click track name
5. Verify full-size window appears with image
