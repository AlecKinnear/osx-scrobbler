# Testing Checklist for macOS Scrobbler

This document provides a comprehensive testing checklist for validating the application before distribution.

## Prerequisites

- macOS 13.0 (Ventura) or later
- Xcode 14.0 or later installed
- Last.fm API credentials (from https://www.last.fm/api/account/create)
- ListenBrainz token (optional, from https://listenbrainz.org/profile/)
- A music player (Music.app, Spotify, or other supported app)

## Build Testing

### 1. Project Build

- [ ] Open `UniversalScrobbler/UniversalScrobbler.xcodeproj` in Xcode
- [ ] Select "UniversalScrobbler" scheme
- [ ] Product > Clean Build Folder (⇧⌘K)
- [ ] Product > Build (⌘B)
- [ ] Verify no compilation errors
- [ ] Check for and review any warnings
- [ ] Verify build succeeds for Debug configuration
- [ ] Verify build succeeds for Release configuration

### 2. Code Signing

- [ ] Check code signing settings in project
- [ ] Verify bundle identifier is correct
- [ ] Ensure "Sign to Run Locally" is selected (for testing)

## Functional Testing

### 3. Application Launch

- [ ] Product > Run (⌘R)
- [ ] Verify app appears in menu bar (top right)
- [ ] Verify no dock icon appears (LSUIElement works)
- [ ] Check app icon displays correctly in menu bar
- [ ] Click menu bar icon to open menu
- [ ] Verify menu items appear correctly

### 4. First Launch Experience

- [ ] On first launch, verify settings window opens automatically
- [ ] Check that Last.fm section shows "Not authenticated"
- [ ] Verify UI is clean and all elements render properly
- [ ] Test both light and dark mode appearance

### 5. Last.fm Authentication

- [ ] Open Settings > Services tab
- [ ] Enter your Last.fm API Key
- [ ] Enter your Last.fm API Secret
- [ ] Click "Authorize with Last.fm"
- [ ] Verify browser opens to Last.fm authorization page
- [ ] Grant permission on Last.fm website
- [ ] Return to app and click "Complete Authorization"
- [ ] Verify authentication succeeds
- [ ] Check that username displays correctly
- [ ] Verify "Disconnect" button appears
- [ ] Close and reopen app to verify session persists

**Test Error Cases:**
- [ ] Try invalid API credentials
- [ ] Try completing authorization without granting permission
- [ ] Test network error handling (disconnect network)

### 6. ListenBrainz Configuration

- [ ] Open Settings > Services tab
- [ ] Click "Add ListenBrainz Instance"
- [ ] Enter ListenBrainz token
- [ ] Enter custom name (or use default)
- [ ] Verify instance appears in list
- [ ] Test adding multiple instances
- [ ] Test removing an instance
- [ ] Verify configurations persist after app restart

### 7. Media Detection

**With Music.app:**
- [ ] Open Music.app
- [ ] Play a song
- [ ] Verify track info appears in menu bar
- [ ] Check artist name displays correctly
- [ ] Check track title displays correctly
- [ ] Check album name displays correctly
- [ ] Pause playback and verify status updates
- [ ] Resume playback and verify status updates
- [ ] Skip to next track and verify update

**With Spotify:**
- [ ] Open Spotify
- [ ] Play a song
- [ ] Verify track detection works
- [ ] Test pause/resume
- [ ] Test track changes

**With Other Players:**
- [ ] Test with any other media players you use
- [ ] Verify detection or confirm expected behavior

### 8. Scrobbling Functionality

**Now Playing Updates:**
- [ ] Start playing a track
- [ ] Verify "Now Playing" sent within a few seconds
- [ ] Check Last.fm profile shows "Now Playing" status
- [ ] Check ListenBrainz if configured

**Scrobble Submission:**
- [ ] Let a track play past 50% (or 4 minutes, whichever is first)
- [ ] Verify scrobble submits
- [ ] Check Last.fm profile for new scrobble
- [ ] Check ListenBrainz profile if configured
- [ ] Verify scrobble timestamp is correct

**Edge Cases:**
- [ ] Play a track for less than 30 seconds (should not scrobble)
- [ ] Skip tracks quickly (should not scrobble)
- [ ] Play a track exactly 30 seconds (minimum for scrobbling)
- [ ] Test with very long tracks (>10 minutes)
- [ ] Pause mid-track and verify behavior

### 9. Album Art Display

- [ ] Play a track with album art
- [ ] Verify thumbnail appears in menu
- [ ] Click on album art thumbnail
- [ ] Verify full-size album art window opens
- [ ] Test resizing the window
- [ ] Test closing and reopening
- [ ] Test with tracks that have no album art
- [ ] Verify placeholder image displays for missing art

### 10. Settings - General Tab

**Refresh Interval:**
- [ ] Test changing from default 5 seconds
- [ ] Try minimum value (1 second)
- [ ] Try maximum value (60 seconds)
- [ ] Verify changes take effect immediately

**Scrobble Threshold:**
- [ ] Test changing from default 50%
- [ ] Try different values (25%, 75%, 100%)
- [ ] Verify scrobbles submit at correct time
- [ ] Test with 4-minute maximum rule

**Album Art Thumbnails:**
- [ ] Toggle thumbnails on/off
- [ ] Verify thumbnails appear/disappear in menu
- [ ] Check performance with toggle

**Menu Bar Text:**
- [ ] Toggle between "Artist - Title" and "Title - Artist"
- [ ] Verify menu bar text updates correctly

### 11. Settings - App Filtering

- [ ] Open Settings > App Filter tab
- [ ] Add Music.app to allowed list
- [ ] Play music in Music.app (should work)
- [ ] Play music in Spotify (should be blocked)
- [ ] Remove Music.app from filter
- [ ] Test with "Allow all apps" mode
- [ ] Test with multiple apps in filter
- [ ] Verify bundle IDs display correctly

### 12. Settings - Text Cleanup

- [ ] Open Settings > Text Cleanup tab
- [ ] Add a cleanup pattern (e.g., " - Remastered" removal)
- [ ] Play a track with matching text
- [ ] Verify text is cleaned before scrobbling
- [ ] Check scrobbled data on Last.fm/ListenBrainz
- [ ] Test with multiple patterns
- [ ] Test with regex patterns
- [ ] Test adding/removing patterns

### 13. Love Track Feature

- [ ] In Last.fm settings, verify "Love" button appears
- [ ] Play a track
- [ ] Click "Love" in menu
- [ ] Verify track is loved on Last.fm
- [ ] Check Last.fm profile for loved track
- [ ] Test loving multiple tracks
- [ ] Test error handling (network issues)

### 14. Metadata Enrichment

- [ ] Play a track with incomplete metadata
- [ ] Verify app attempts MusicBrainz lookup
- [ ] Check if missing data is filled in
- [ ] Monitor performance with enrichment
- [ ] Test with tracks that have no MusicBrainz match

### 15. Settings Persistence

- [ ] Configure all settings
- [ ] Quit app completely (⌘Q)
- [ ] Restart app
- [ ] Verify all settings are preserved:
  - [ ] Last.fm authentication
  - [ ] ListenBrainz instances
  - [ ] Refresh interval
  - [ ] Scrobble threshold
  - [ ] App filters
  - [ ] Text cleanup patterns
  - [ ] UI preferences

### 16. Menu Bar Integration

- [ ] Verify menu bar icon always visible
- [ ] Test clicking icon (should open menu)
- [ ] Test right-clicking icon (should open menu)
- [ ] Verify menu stays open until clicked away
- [ ] Check menu appearance in light mode
- [ ] Check menu appearance in dark mode

### 17. Error Handling

**Network Errors:**
- [ ] Disconnect from internet
- [ ] Play a track
- [ ] Verify app handles gracefully (no crashes)
- [ ] Reconnect internet
- [ ] Verify app recovers and scrobbles

**API Errors:**
- [ ] Temporarily use invalid credentials
- [ ] Verify error messages are clear
- [ ] Check app doesn't crash on API failures

**Media Player Errors:**
- [ ] Quit all media players
- [ ] Verify app handles "no playing media" state
- [ ] Open player and verify detection resumes

### 18. Performance Testing

- [ ] Monitor CPU usage (should be minimal when idle)
- [ ] Check CPU usage during active playback
- [ ] Monitor memory usage over extended period
- [ ] Play 20+ tracks and verify no memory leaks
- [ ] Check app responsiveness
- [ ] Verify UI doesn't freeze during operations

### 19. Long-Running Stability

- [ ] Run app for 1 hour with continuous playback
- [ ] Run app for 4+ hours with intermittent playback
- [ ] Leave app running overnight
- [ ] Verify no crashes or hangs
- [ ] Check all features still work after extended use

### 20. Edge Cases

- [ ] Play podcast (typically shouldn't scrobble)
- [ ] Play audiobook (typically shouldn't scrobble)
- [ ] Play local file with no metadata
- [ ] Play streaming radio (behavior may vary)
- [ ] Switch between multiple players rapidly
- [ ] Test with Unicode characters in track names
- [ ] Test with very long track/artist names
- [ ] Test with special characters in metadata

## Compatibility Testing

### 21. macOS Versions

- [ ] Test on macOS 13.0 (Ventura)
- [ ] Test on macOS 14.0 (Sonoma)
- [ ] Test on macOS 15.0 (Sequoia) if available
- [ ] Verify MediaRemote API works on all versions

### 22. Music Players

Test with as many players as possible:
- [ ] Apple Music (Music.app)
- [ ] Spotify
- [ ] iTunes (if still used)
- [ ] VOX
- [ ] Swinsian
- [ ] Any other media player

## Release Build Testing

### 23. Release Configuration

- [ ] Build with Release configuration
- [ ] Verify optimizations don't break functionality
- [ ] Check release build size is reasonable
- [ ] Test all features with release build
- [ ] Verify performance is good or better

### 24. Archive and Export

- [ ] Product > Archive
- [ ] Verify archive builds successfully
- [ ] Export as macOS App
- [ ] Test exported app outside Xcode
- [ ] Verify all features work in exported app

## Known Issues to Verify

- [ ] MediaRemote is private API (expected limitation)
- [ ] App not suitable for App Store (private API usage)
- [ ] Requires macOS 13.0+ (by design)

## Pre-Distribution Checklist

Before releasing:
- [ ] All critical tests pass
- [ ] No known crash bugs
- [ ] Authentication works reliably
- [ ] Scrobbling is accurate
- [ ] Performance is acceptable
- [ ] Documentation is complete
- [ ] README has correct information
- [ ] CHANGELOG is updated
- [ ] License information is correct
- [ ] Version number is set correctly

## Bug Reporting Template

When you find issues, document them with:

```
**Bug Title:** Brief description

**Steps to Reproduce:**
1. Step one
2. Step two
3. Step three

**Expected Behavior:**
What should happen

**Actual Behavior:**
What actually happens

**Environment:**
- macOS version:
- App version:
- Music player:

**Logs/Screenshots:**
(Attach if available)
```

## Test Results Log

Keep a log of your test results:

```
Date: YYYY-MM-DD
Tester: [Name]
macOS Version: [version]
Build: [version]

Tests Passed: X/Y
Critical Issues: [count]
Minor Issues: [count]

Notes:
- [Any important observations]
```
