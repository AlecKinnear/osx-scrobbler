# Regression Analysis: Broken (6d1704e) vs Last Working (853795b)

## Quick Stats
- **Broken:** `6d1704e` - "WIP: Idagio metadata enrichment with Last.fm album art - BROKEN Yandex scrobbling"
- **Working:** `853795b` - "Refine ListenBrainz integration and UI polish"
- **Change delta:** 1264 insertions(+), 118 deletions(-)
- **Files modified:** 16

## Key Changes (Root Cause Candidates)

### 1. New File: `src/metadata_enricher.rs` (+755 lines)
Completely new module with:
- **Bing search API integration** - Searches for album/artist metadata
- **Idagio API calls** - Fetches metadata with bearer token
- **Background threading** - Uses `tokio::spawn` for enrichment
- **MusicBrainz barcode lookup** - Attempts to match UPC to releases

**Risk Level:** 🔴 HIGH
- Executes on every track that matches UPC pattern
- Spawns background tasks that may interfere with event loop
- API error handling unclear (may silently fail)
- Threading overhead during peak scrobbling

### 2. Modified: `src/main.rs` (+135 lines)
Changes to main event loop:
```
Lines 269, 358: Added `mut track` destructuring patterns
New enrichment calls integrated into scrobble flow
Possible blocking or interference with event timing
```

**Risk Level:** 🔴 HIGH
- Direct integration into hot path (event loop)
- May block next event processing
- Could cause track eligibility recalculation issues

### 3. Modified: `src/ui/tray.rs` (+86 lines changed)
- Added album art URL tracking
- Updated menu item rendering for album art
- New `update_album_art` method (currently unused)

**Risk Level:** 🟡 MEDIUM
- UI-layer only, shouldn't break detection
- But unused method suggests incomplete integration

### 4. Modified: `src/media_monitor.rs` (+38 lines)
- Changes to polling/detection logic
- May affect track eligibility checks

**Risk Level:** 🟡 MEDIUM
- Unknown modifications to detection thresholds
- Could explain why Yandex tracks disappear

### 5. Modified: `src/config.rs` (+13 lines)
- Added `idagio_bearer_token` field
- Configuration loading changes

**Risk Level:** 🟢 LOW
- Config shouldn't break detection
- Unless token validation is blocking startup

### 6. Modified: `src/text_cleanup.rs` (+129 lines)
- New `parse_classical_metadata` function
- UPC extraction logic

**Risk Level:** 🟡 MEDIUM
- Only affects classical track parsing
- Shouldn't break non-classical sources

### 7. New File: `src/ui/album_art_window.rs` (+39 lines)
- Placeholder for album art window
- Not integrated

**Risk Level:** 🟢 LOW
- Just a stub, shouldn't cause issues

## Hypothesis for Regression

### Most Likely Causes (in order):
1. **Enrichment blocking event loop** - Background threads or sync calls blocking in `main.rs`
2. **Track eligibility recalculation** - Changes in media_monitor eligibility logic preventing detection
3. **Silent error in non-Idagio paths** - Enricher crashing/returning error for Yandex tracks
4. **Config loading failure** - Idagio token validation blocking startup

## How to Diagnose

### Step 1: Check Media Monitor
```bash
RUST_LOG=debug cargo run
# Look for: "poll_track: detected" messages for Yandex
```

### Step 2: Isolated Test
```bash
# Revert enricher.rs and main.rs changes only
# See if Yandex detection returns
```

### Step 3: Compare Critical Paths
```bash
git diff 853795b 6d1704e -- src/main.rs src/media_monitor.rs
# Find what changed in event handling
```

## Files to Review First

1. **`src/main.rs`** (lines 269, 358) - Most likely culprit
2. **`src/media_monitor.rs`** - Track detection changes
3. **`src/metadata_enricher.rs`** - Error handling in enrichment

## Testing Plan
1. Run with Yandex Music playing → See if "Now Playing" appears
2. Check debug logs for track detection events
3. Isolate enricher by commenting out calls in main.rs
4. Verify Idagio tracks work (if any available)

---

**Current Status:** Documented broken state pushed to GitHub (commit `adde871`)  
**Next Action:** Deep dive into diffs for 3 files above
