# Swift Rewrite Architecture

## Overview
Native macOS app using SwiftUI for a modern, maintainable scrobbler with proper UI configuration.

## Technology Stack

### Core
- **Swift 5.9+** (macOS 13.0+ target)
- **SwiftUI** for UI
- **Combine** for reactive data flow
- **MediaRemote.framework** (private API via bridging header)

### Key Features
- Menu bar app (NSStatusItem)
- Settings window with SwiftUI
- Background media monitoring
- Multi-service scrobbling (Last.fm, ListenBrainz)
- Album art display
- UserDefaults for configuration

## Project Structure

```
UniversalScrobbler/
├── UniversalScrobbler.xcodeproj
├── UniversalScrobbler/
│   ├── App/
│   │   ├── UniversalScrobblerApp.swift          # Main app entry point
│   │   ├── AppDelegate.swift                     # Menu bar setup
│   │   └── Info.plist
│   │
│   ├── Models/
│   │   ├── Track.swift                           # Track data model
│   │   ├── Config.swift                          # App configuration (Codable)
│   │   └── ScrobbleService.swift                 # Service protocol
│   │
│   ├── Services/
│   │   ├── MediaMonitor.swift                    # MediaRemote integration
│   │   ├── LastFmService.swift                   # Last.fm API client
│   │   ├── ListenBrainzService.swift            # ListenBrainz API client
│   │   ├── MetadataEnricher.swift               # MusicBrainz/Last.fm metadata
│   │   └── ScrobbleManager.swift                # Coordinates all services
│   │
│   ├── Views/
│   │   ├── MenuBarView.swift                    # Menu bar content
│   │   ├── SettingsView.swift                   # Main settings window
│   │   ├── ServicesSettingsView.swift           # Last.fm/LB config
│   │   ├── AppFilteringView.swift               # App allow/block list
│   │   ├── TextCleanupView.swift                # Regex patterns editor
│   │   └── AlbumArtWindow.swift                 # Full-size album art
│   │
│   ├── Utilities/
│   │   ├── TextCleaner.swift                    # Regex text cleanup
│   │   ├── ImageCache.swift                     # Album art caching
│   │   └── Extensions.swift                     # Swift extensions
│   │
│   └── Resources/
│       ├── Assets.xcassets
│       │   ├── AppIcon.appiconset/
│       │   └── MenuBarIcon.imageset/
│       └── MediaRemote.h                         # Private framework bridge
```

## Key Components

### 1. MediaMonitor
Wraps MediaRemote.framework to get now playing info:
- MRMediaRemoteGetNowPlayingInfo
- MRMediaRemoteRegisterForNowPlayingNotifications
- Publishes updates via Combine

### 2. ScrobbleManager
Central coordinator:
- Receives track updates from MediaMonitor
- Manages scrobble timing (50% threshold)
- Distributes to enabled services (Last.fm, ListenBrainz)
- Handles enrichment pipeline

### 3. Configuration (UserDefaults)
```swift
struct AppConfig: Codable {
    var refreshInterval: Int = 5
    var scrobbleThreshold: Int = 50
    var textCleanup: TextCleanupConfig
    var appFiltering: AppFilteringConfig
    var lastfm: LastFmConfig?
    var listenbrainz: [ListenBrainzConfig]
    var idagio: IdagioConfig?
}
```

### 4. Settings Window
Modern SwiftUI with tabs:
- **General**: Refresh interval, scrobble threshold
- **Services**: Last.fm and ListenBrainz setup
- **Apps**: Allow/block app list
- **Text Cleanup**: Regex pattern editor
- **About**: Version, links

### 5. Menu Bar
```
🎵 Universal Scrobbler
─────────────────────
▶ Now Playing: Artist - Title
  [Album Art Thumbnail]
◀ Last Scrobbled: Artist - Title
─────────────────────
❤️ Love Track
⚙️ Settings...
─────────────────────
Quit
```

## MediaRemote Bridge

Since MediaRemote is private API, we need a bridging header:

```objective-c
// MediaRemote.h
#import <Foundation/Foundation.h>

extern NSString *kMRMediaRemoteNowPlayingInfoDidChangeNotification;

extern CFDictionaryRef MRMediaRemoteGetNowPlayingInfo(void);

extern void MRMediaRemoteRegisterForNowPlayingNotifications(dispatch_queue_t queue);
```

## Data Flow

```
macOS Media System
    ↓ (MediaRemote)
MediaMonitor (notifications)
    ↓ (Combine Publisher)
ScrobbleManager
    ↓
├─→ Check app filtering → Prompt if new app
├─→ Apply text cleanup
├─→ Enrich metadata (async)
├─→ Send now_playing to services
└─→ Schedule scrobble at 50% threshold
        ↓
    LastFmService / ListenBrainzService
        ↓
    API Calls
```

## Benefits Over Rust

### Development Experience
✅ Native Xcode tooling (debugging, profiling)
✅ SwiftUI hot reload
✅ Built-in macOS integration (no objc2 FFI)
✅ Type-safe async/await

### User Experience
✅ Native settings window (no manual .conf editing)
✅ macOS standard UI patterns
✅ Better sandboxing support
✅ App Store distribution possible

### Code Quality
✅ Less boilerplate than objc2 bindings
✅ Automatic memory management
✅ Modern concurrency (async/await, actors)
✅ Combine for reactive updates

## Migration Plan

### Phase 1: Core Functionality
1. Set up Xcode project with menu bar app
2. MediaRemote integration
3. Basic track detection
4. UserDefaults configuration

### Phase 2: Scrobbling
1. Last.fm service (auth, now_playing, scrobble, love)
2. ListenBrainz service
3. Scrobble timing logic
4. Error handling and retry logic

### Phase 3: UI
1. Settings window (all tabs)
2. Menu bar status updates
3. App filtering prompts
4. Album art display

### Phase 4: Advanced Features
1. Metadata enrichment (MusicBrainz)
2. Text cleanup with regex
3. Album art caching
4. IDAGIO special handling

### Phase 5: Polish
1. App icon and menu bar icon
2. Logging improvements
3. Launch at login support
4. Notarization for distribution

## Next Steps

1. Create Xcode project structure
2. Set up MediaRemote bridging header
3. Implement basic MediaMonitor
4. Test media detection with Music.app

Would you like me to start creating the Swift project files?
