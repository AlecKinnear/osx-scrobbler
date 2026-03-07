# macOS Scrobbler

A native macOS music scrobbler application built with Swift and SwiftUI.

## Overview

This is a complete Swift rewrite of the macOS Scrobbler, designed to be a native macOS menu bar application with proper UI configuration and modern Swift patterns.

## Features

- **Native macOS Menu Bar App**: Runs in the menu bar with no dock icon
- **Multi-Service Support**: Scrobbles to Last.fm and ListenBrainz
- **Media Detection**: Monitors now playing from Music.app, Spotify, and other media players
- **Metadata Enrichment**: Enhances track metadata using MusicBrainz API
- **Album Art Display**: Shows album art thumbnails and full-size viewer
- **Text Cleanup**: Configurable regex patterns to clean track metadata
- **App Filtering**: Control which apps to scrobble from
- **Love Tracks**: Support for loving tracks on Last.fm
- **Settings UI**: Native SwiftUI settings window

## Requirements

- macOS 13.0 (Ventura) or later
- Xcode 14.0 or later
- Swift 5.9 or later

## Project Structure

```
UniversalScrobbler/
├── UniversalScrobbler.xcodeproj     # Xcode project file
└── UniversalScrobbler/
    ├── App/                          # Application entry and delegate
    │   ├── UniversalScrobblerApp.swift
    │   ├── AppDelegate.swift
    │   └── Info.plist
    ├── Models/                       # Data models
    │   ├── Track.swift
    │   ├── Config.swift
    │   └── ScrobbleService.swift
    ├── Services/                     # Business logic
    │   ├── MediaMonitor.swift
    │   ├── LastFmService.swift
    │   ├── ListenBrainzService.swift
    │   ├── MetadataEnricher.swift
    │   └── ScrobbleManager.swift
    ├── Views/                        # SwiftUI views
    │   ├── MenuBarView.swift
    │   ├── SettingsView.swift
    │   ├── AlbumArtWindow.swift
    │   └── AppPromptDialog.swift
    ├── Utilities/                    # Helper utilities
    │   ├── TextCleaner.swift
    │   ├── ImageCache.swift
    │   └── Extensions.swift
    └── Resources/                    # Resources and assets
        └── MediaRemote.h             # Private API bridge
```

## Architecture

### Core Components

#### MediaMonitor
- Uses DistributedNotificationCenter to monitor macOS media events
- Tracks play sessions with scrobble timing logic
- Applies text cleanup to metadata
- Filters apps based on user configuration

#### ScrobbleManager
- Central coordinator for all scrobbling services
- Manages Last.fm and ListenBrainz instances
- Handles now playing updates and scrobbles
- Tracks loved status on Last.fm

#### MetadataEnricher
- Enriches track metadata using MusicBrainz API
- Searches by barcode (UPC) or album name
- Matches tracks by duration tolerance
- Fetches album art from Last.fm

#### ConfigManager
- Manages app configuration using UserDefaults
- Validates configuration on save
- Provides reactive config updates

### Services

#### LastFmService
- Implements Last.fm API v2.0
- Handles authentication with session keys
- Supports now playing, scrobble, love, and status checks
- Uses MD5 signature generation

#### ListenBrainzService
- Implements ListenBrainz API v1
- Token-based authentication
- Supports multiple instances
- Now playing and scrobble submissions

### UI Components

#### MenuBarView
- Main menu bar popover interface
- Shows currently playing track
- Displays album art thumbnail
- Love track button
- Access to settings

#### SettingsView
- Tabbed SwiftUI settings interface
- General settings (refresh interval, threshold)
- Service configuration (Last.fm, ListenBrainz)
- App filtering (allow/ignore lists)
- Text cleanup patterns
- About page

#### AlbumArtWindow
- Dedicated window for full-size album art
- Async image loading with cache
- Resizable and closable

### Utilities

#### TextCleaner
- Regex-based text cleanup
- Configurable patterns
- Removes explicit tags, etc.

#### ImageCache
- Thread-safe image caching
- Async image loading
- Reduces network requests

## Building

1. Open `UniversalScrobbler.xcodeproj` in Xcode
2. Select the "UniversalScrobbler" scheme
3. Build and run (⌘R)

The app will appear in the menu bar with a music note icon.

## Configuration

Configuration is stored in UserDefaults and can be accessed through the Settings window:

- **Refresh Interval**: How often to check for media updates (default: 5 seconds)
- **Scrobble Threshold**: Percentage of track to play before scrobbling (default: 50%)
- **Last.fm**: API credentials and session key
- **ListenBrainz**: Multiple instances supported with tokens
- **App Filtering**: Control which apps to scrobble from
- **Text Cleanup**: Regex patterns for cleaning metadata

## MediaRemote Framework

This app uses Apple's private MediaRemote framework for media detection. The `MediaRemote.h` bridging header provides access to:

- `MRMediaRemoteGetNowPlayingInfo()` - Get current media info
- `MRMediaRemoteRegisterForNowPlayingNotifications()` - Subscribe to updates
- Distributed notifications from media apps

## Migration from Rust

This Swift implementation replaces the Rust version with:

- ✅ Native SwiftUI settings interface (no manual config editing)
- ✅ Better macOS integration (notifications, menu bar)
- ✅ Xcode tooling (debugging, profiling, Interface Builder)
- ✅ Type-safe async/await for network calls
- ✅ Combine framework for reactive updates
- ✅ UserDefaults for configuration (standard macOS)
- ✅ Reduced complexity (no FFI, no objc2 bindings)

## Known Limitations

- IDAGIO support has been removed for simplicity
- MediaRemote is a private API (may break in future macOS versions)
- Requires macOS 13.0+ (uses modern Swift concurrency)

## Future Enhancements

- [ ] Launch at login support
- [ ] Keyboard shortcuts
- [ ] Notification center integration
- [ ] Export scrobble history
- [ ] Statistics and charts
- [ ] App Store distribution (requires alternative to MediaRemote)

## License

Same as the original Rust project.

## Credits

Swift rewrite of the Universal Scrobbler by Alec Kinnear.
