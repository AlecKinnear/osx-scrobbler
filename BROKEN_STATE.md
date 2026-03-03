# Broken State Documentation - Yandex Regression

**Commit:** 6d1704ecfb8fc230461868e58abce9d8df7dff25  
**Status:** ⚠️ DO NOT USE - YANDEX MUSIC SCROBBLING BROKEN

## Problem Summary
Idagio metadata enrichment feature caused regression affecting Yandex Music and potentially other sources.

### Symptoms
- ❌ Yandex Music: No tracks detected (no now_playing events)
- ❌ Status bar: Shows nothing (no "Now Playing" updates)
- ❌ Scrobbling: Halted for all services
- ✅ Build: Compiles cleanly
- ✅ Tests: All 15 unit tests pass

## Affected Code (Relative to 853795b)
- `src/metadata_enricher.rs` - New Bing search + Idagio API logic
- `src/main.rs` - Enrichment pipeline integration (lines 269, 358)
- `src/ui/tray.rs` - Album art URL tracking
- `src/config.rs` - Added `idagio_bearer_token` field
- `src/ui/album_art_window.rs` - New placeholder file

## Failure Analysis
The enrichment pipeline may be:
1. Blocking event processing in main loop
2. Silently crashing on non-Idagio tracks
3. Breaking track eligibility checks
4. Causing unhandled errors in background threads

## Recovery
**Last Working:** Commit 853795b "Refine ListenBrainz integration and UI polish"
- All services working (Yandex ✓, Last.fm ✓, ListenBrainz ✓)
- No enrichment overhead
- Clean state for comparison

## Next Step
Run: `git diff 853795b 6d1704e --stat` to see scope of changes
Then compare implementation diffs to find regression point.
