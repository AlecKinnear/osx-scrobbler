# macOS Scrobbler

A native macOS music scrobbler application built with Swift and SwiftUI.

![Platform](https://img.shields.io/badge/platform-macOS%2013.0%2B-lightgrey)
![Swift](https://img.shields.io/badge/Swift-5.9%2B-orange)
![License](https://img.shields.io/badge/license-Apache%202.0-blue)

## Overview

macOS Scrobbler is a native menu bar application that scrobbles your music listening to Last.fm and ListenBrainz. Built with modern Swift patterns and SwiftUI, it provides a clean, native macOS experience with comprehensive configuration options.

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
├── UniversalScrobbler.xcodeproj/    # Xcode project
└── UniversalScrobbler/
    ├── App/                          # Application entry and delegate
    ├── Models/                       # Data models
    ├── Services/                     # Business logic
    ├── Views/                        # SwiftUI views
    ├── Utilities/                    # Helper utilities
    └── Resources/                    # Assets and resources
```

See [UniversalScrobbler/README.md](UniversalScrobbler/README.md) for detailed project documentation.

## Configuration

Configuration is stored in UserDefaults and can be accessed through the Settings window:

- **Refresh Interval**: How often to check for media updates (default: 5 seconds)
- **Scrobble Threshold**: Percentage of track to play before scrobbling (default: 50%)
- **Last.fm**: User-friendly authentication interface with web-based OAuth flow
- **ListenBrainz**: Multiple instances supported with tokens
- **App Filtering**: Control which apps to scrobble from
- **Text Cleanup**: Regex patterns for cleaning metadata

### Last.fm Authentication

The app includes a streamlined authentication interface:
1. Get API credentials from https://www.last.fm/api/account/create
2. Enter credentials in Settings > Services
3. Click "Authorize with Last.fm" to open the authorization page
4. Grant permission on Last.fm website
5. Return to the app and click "Complete Authorization"

See [UniversalScrobbler/AUTHENTICATION.md](UniversalScrobbler/AUTHENTICATION.md) for detailed authentication documentation.

## Architecture

Built with modern Swift and SwiftUI:

- **MediaRemote Framework**: Uses Apple's private MediaRemote framework for system-wide media detection
- **Async/Await**: Modern Swift concurrency for network calls
- **UserDefaults**: Standard macOS configuration storage
- **SwiftUI**: Declarative UI with native macOS controls
- **Combine**: Reactive updates for real-time scrobbling

See [UniversalScrobbler/ARCHITECTURE.md](UniversalScrobbler/ARCHITECTURE.md) for detailed architecture documentation.

## Known Limitations

- MediaRemote is a private API (may break in future macOS versions)
- Requires macOS 13.0+ (uses modern Swift concurrency)
- Not suitable for App Store distribution due to private API usage

## Future Enhancements

- [ ] Launch at login support
- [ ] Keyboard shortcuts
- [ ] Notification center integration
- [ ] Export scrobble history
- [ ] Statistics and charts
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

See [CONTRIBUTING.md](CONTRIBUTING.md) and [UniversalScrobbler/XCODE_SETUP.md](UniversalScrobbler/XCODE_SETUP.md) for detailed setup instructions.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.

## Credits

A native Swift implementation of a macOS music scrobbler.
