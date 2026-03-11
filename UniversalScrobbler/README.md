# macOS Scrobbler

A native macOS music scrobbler application built with Swift and SwiftUI.

## Overview

This is a native macOS menu bar application that scrobbles your music listening to Last.fm and ListenBrainz. Built with modern Swift patterns and SwiftUI, it provides a clean, native macOS experience with comprehensive configuration options.

## Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/swift-scrobbler.git
   cd swift-scrobbler
   ```

2. Open `UniversalScrobbler.xcodeproj` in Xcode

3. Build and run (⌘R)

4. Configure Last.fm authentication:
   - Get API credentials from https://www.last.fm/api/account/create
   - Click the menu bar icon and go to Settings > Services
   - Follow the authentication flow

5. Start listening to music - your scrobbles will appear automatically!

## Features

- **Native macOS Menu Bar App**: Runs in the menu bar with no dock icon
- **User-Friendly Authentication**: Streamlined Last.fm OAuth flow with visual guidance
- **Multi-Service Support**: Scrobbles to Last.fm and ListenBrainz
- **Media Detection**: Monitors now playing from Music.app, Spotify, and other media players
- **Metadata Enrichment**: Enhances track metadata using MusicBrainz API
- **Album Art Display**: Shows album art thumbnails and full-size viewer
- **Text Cleanup**: Configurable regex patterns to clean track metadata
- **App Filtering**: Control which apps to scrobble from
- **Love Tracks**: Support for loving tracks on Last.fm
- **Light/Dark Mode**: Full support for macOS system appearance
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
- **Last.fm**: User-friendly authentication interface with web-based OAuth flow
- **ListenBrainz**: Multiple instances supported with tokens
- **App Filtering**: Control which apps to scrobble from
- **Text Cleanup**: Regex patterns for cleaning metadata

### Last.fm Authentication

The app now includes a streamlined authentication interface:
1. Get API credentials from https://www.last.fm/api/account/create
2. Enter credentials in Settings > Services
3. Click "Authorize with Last.fm" to open the authorization page
4. Grant permission on Last.fm website
5. Return to the app and click "Complete Authorization"

See [AUTHENTICATION.md](AUTHENTICATION.md) for detailed authentication documentation.

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
- [ ] ListenBrainz authentication UI (similar to Last.fm)
- [ ] Session key storage in Keychain

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork and clone the repository
2. Open in Xcode 14.0 or later
3. Make your changes
4. Test thoroughly on macOS 13.0+
5. Submit a pull request

See [XCODE_SETUP.md](UniversalScrobbler/XCODE_SETUP.md) for detailed setup instructions.

## License

Same as the original Rust project.

## Credits

Swift rewrite of the Universal Scrobbler.
