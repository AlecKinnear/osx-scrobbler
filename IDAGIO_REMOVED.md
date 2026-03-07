# IDAGIO Support Removal

## Summary

All IDAGIO-specific code has been removed from the project for simplicity. IDAGIO classical music requires complex metadata enrichment (API calls, search engine queries, duration matching) that adds significant complexity to the codebase.

## What Was Removed

### 1. Configuration (`src/config.rs`)
- Removed `IdagioConfig` struct
- Removed `idagio` field from main `Config` struct

### 2. Main Application (`src/main.rs`)
- Removed IDAGIO album art display logic
- Removed synchronous IDAGIO enrichment calls
- Simplified now_playing event handling

### 3. Media Monitor (`src/media_monitor.rs`)
- IDAGIO detection now returns `None` (skips scrobbling entirely)
- Removed complex classical metadata parsing
- Removed UPC/catalog ID handling
- Simplified `media_info_to_track()` function

### 4. Metadata Enricher (`src/metadata_enricher.rs`)
- Removed `IdagioAlbumResponse`, `IdagioAlbum`, `IdagioTrack`, etc. structs
- Removed `enrich_idagio_track()` function
- Removed `search_idagio_album_page()` function
- Removed `fetch_and_parse_idagio_album()` function
- Removed SearXNG search constant
- Removed all IDAGIO API integration
- Simplified `enrich_from_musicbrainz()` to only handle standard UPC barcodes

### 5. Track Model (`src/scrobbler.rs`)
- Removed `idagio_album_art_url()` method from `Track`
- Kept `upc` field (still useful for standard music UPC barcodes)

### 6. Text Cleanup (`src/text_cleanup.rs`)
- Renamed `parse_classical_metadata()` to `_unused_parse_classical_metadata()`
- Marked as `#[allow(dead_code)]` for future reference

## Current Behavior

When IDAGIO is detected in media metadata:
- Log message: "IDAGIO detected in metadata - skipping (IDAGIO not supported)"
- Track is **not scrobbled** to Last.fm or ListenBrainz
- User sees nothing in the menu bar
- This prevents incorrect/incomplete scrobbles

## Lines of Code Removed

- **~450 lines** of IDAGIO-specific code deleted
- Simplified architecture
- Easier to maintain
- Easier to port to Swift

## Future Consideration

If IDAGIO support is needed in the future:
1. Create a separate project/binary specifically for IDAGIO
2. Or implement as an optional plugin with dedicated configuration
3. Swift version could handle this better with async/await

## Testing

The codebase should now:
- ✅ Scrobble tracks from Spotify, Apple Music, Yandex Music, etc.
- ✅ Apply text cleanup (remove [Explicit], etc.)
- ✅ Handle app filtering
- ✅ Support Last.fm and ListenBrainz
- ✅ Fetch Last.fm album art for standard tracks
- ✅ Skip IDAGIO tracks entirely (no partial/broken scrobbles)

## Note for Swift Rewrite

The Swift version should:
- Not include IDAGIO support initially
- Focus on standard music services first
- Keep architecture simple and maintainable
- Consider IDAGIO as a future enhancement if really needed
