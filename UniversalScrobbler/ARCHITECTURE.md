# Universal Scrobbler Architecture (Swift)

## System Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                      macOS System                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Music.app    │  │ Spotify      │  │ Other Apps   │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                  │               │
│         └──────────────────┴──────────────────┘               │
│                             │                                 │
│              DistributedNotificationCenter                    │
└─────────────────────────────┬───────────────────────────────┘
                              │
┌─────────────────────────────▼───────────────────────────────┐
│              Universal Scrobbler (Swift)                     │
│                                                               │
│  ┌────────────────────────────────────────────────────┐    │
│  │                   AppDelegate                       │    │
│  │           (Main Coordinator & Timer)                │    │
│  └───┬────────────────────────────────────────────┬───┘    │
│      │                                              │        │
│  ┌───▼──────────────┐                  ┌──────────▼─────┐  │
│  │  MediaMonitor    │                  │  Menu Bar UI   │  │
│  │  - Notifications │                  │  - SwiftUI     │  │
│  │  - Text cleanup  │                  │  - Album art   │  │
│  │  - App filtering │                  │  - Settings    │  │
│  └───┬──────────────┘                  └────────────────┘  │
│      │                                                       │
│      │ Track Events                                         │
│      │                                                       │
│  ┌───▼──────────────────────────────────────────────┐      │
│  │          ScrobbleManager                          │      │
│  │     (Coordinates all services)                    │      │
│  └───┬───────────────────────────────────────┬──────┘      │
│      │                                        │              │
│  ┌───▼─────────────┐              ┌─────────▼──────────┐   │
│  │ MetadataEnricher│              │  Scrobble Services │   │
│  │  - MusicBrainz  │              │  - Last.fm         │   │
│  │  - Last.fm art  │              │  - ListenBrainz    │   │
│  └─────────────────┘              └─────────┬──────────┘   │
│                                              │               │
└──────────────────────────────────────────────┼──────────────┘
                                               │
                     ┌─────────────────────────┴─────────────────┐
                     │                                             │
            ┌────────▼─────────┐                    ┌─────────▼──────────┐
            │  Last.fm API     │                    │ ListenBrainz API   │
            │  ws.audioscrobbler│                   │ api.listenbrainz   │
            └──────────────────┘                    └────────────────────┘
```

## Component Details

### 1. AppDelegate (Main Coordinator)

**Responsibility**: Application lifecycle and coordination

```swift
AppDelegate
├── Setup menu bar (NSStatusItem)
├── Start polling timer (5s interval)
├── Coordinate MediaMonitor ←→ ScrobbleManager
├── Handle app filtering dialogs
└── Manage settings window
```

**Flow**:
1. App launches → AppDelegate.applicationDidFinishLaunching()
2. Load config from UserDefaults
3. Initialize services (MediaMonitor, ScrobbleManager)
4. Setup menu bar UI
5. Start timer (polls every 5 seconds)
6. On each tick: call MediaMonitor.poll()
7. Handle events (now playing, scrobble, unknown app)

### 2. MediaMonitor (Media Detection)

**Responsibility**: Detect and track media playback

```swift
MediaMonitor
├── Subscribe to DistributedNotificationCenter
├── Parse media info (artist, title, album, duration)
├── Apply text cleanup (regex patterns)
├── Filter IDAGIO (skip completely)
├── Track play sessions (PlaySession struct)
├── Calculate scrobble timing (50% or 4 min)
└── Return MediaEvents (now_playing, scrobble, unknown_app)
```

**State Machine**:
```
┌──────────────┐
│   No Media   │
└──────┬───────┘
       │ Media starts
       ▼
┌──────────────┐
│  New Track   │◄────────┐
└──────┬───────┘         │
       │ Time passes     │ Track changes
       ▼                 │
┌──────────────┐         │
│  Threshold   ├─────────┘
│   Reached    │
└──────┬───────┘
       │ Scrobble sent
       ▼
┌──────────────┐
│  Scrobbled   │
└──────────────┘
```

### 3. ScrobbleManager (Service Coordinator)

**Responsibility**: Coordinate scrobbling services

```swift
ScrobbleManager
├── Manage multiple services (Last.fm, ListenBrainz)
├── Distribute now_playing to all services
├── Distribute scrobbles to all services
├── Handle love track requests
├── Check loved status (Last.fm only)
└── Update UI state (@Published properties)
```

**Data Flow**:
```
Track Event
    │
    ▼
ScrobbleManager
    │
    ├──► LastFmService.nowPlaying()
    ├──► LastFmService.scrobble()
    ├──► LastFmService.love()
    │
    ├──► ListenBrainzService.nowPlaying()
    ├──► ListenBrainzService.scrobble()
    │
    └──► Update UI (@Published properties)
```

### 4. Services (Network Layer)

#### LastFmService
```swift
LastFmService: ScrobbleService
├── Generate MD5 signatures (CryptoKit)
├── POST to ws.audioscrobbler.com/2.0/
├── Methods:
│   ├── nowPlaying(track)
│   ├── scrobble(track, timestamp)
│   ├── love(track)
│   └── isLoved(track) → Bool
└── Error handling (throws ScrobbleError)
```

#### ListenBrainzService
```swift
ListenBrainzService: ScrobbleService
├── Token authentication (Bearer)
├── POST to api.listenbrainz.org/1/
├── JSON payloads
├── Methods:
│   ├── nowPlaying(track)
│   ├── scrobble(track, timestamp)
│   ├── love(track) [not implemented]
│   └── isLoved(track) → false [not supported]
└── Multiple instances supported
```

### 5. MetadataEnricher (Enhancement)

**Responsibility**: Enrich track metadata

```swift
MetadataEnricher
├── MusicBrainz API search
│   ├── Barcode (UPC) lookup
│   ├── Album name search
│   └── Duration matching (±3s tolerance)
├── Last.fm album art
│   ├── album.getinfo API
│   ├── Extract image URLs
│   └── Cache results
└── Async enrichment (don't block scrobbling)
```

**Enrichment Flow**:
```
Track with partial metadata
    │
    ├──► Has UPC? → MusicBrainz barcode search
    │                   │
    │                   ├──► Match by duration
    │                   └──► Get album + track title
    │
    ├──► Has album? → MusicBrainz album search
    │                   │
    │                   ├──► Match by duration
    │                   └──► Get corrected metadata
    │
    └──► Has album? → Last.fm album.getinfo
                        │
                        └──► Get album art URL
```

### 6. UI Layer (SwiftUI + AppKit)

#### Menu Bar View
```swift
MenuBarView (SwiftUI)
├── Now Playing display
├── Album art thumbnail
├── Last Scrobbled display
├── Love button (❤️)
├── Settings button (⚙️)
└── Quit button
```

#### Settings View
```swift
SettingsView (SwiftUI TabView)
├── General Tab
│   ├── Refresh interval
│   └── Scrobble threshold
├── Services Tab
│   ├── Last.fm (API key, secret, session key)
│   └── ListenBrainz (token, API URL, name)
├── App Filtering Tab
│   ├── Prompt for new apps
│   ├── Scrobble unknown
│   ├── Allowed apps list
│   └── Ignored apps list
├── Text Cleanup Tab
│   ├── Enable toggle
│   └── Regex patterns list
└── About Tab
    ├── App name
    ├── Version
    └── Description
```

### 7. Configuration System

```swift
ConfigManager (Singleton)
├── Storage: UserDefaults
├── Format: JSON (Codable)
├── Key: "appConfig"
├── Methods:
│   ├── config: AppConfig (get/set)
│   ├── save()
│   └── validate() → [String]
└── Reactive: @Published for UI updates
```

**Config Structure**:
```swift
AppConfig
├── refreshInterval: Int
├── scrobbleThreshold: Int
├── textCleanup: TextCleanupConfig
│   ├── enabled: Bool
│   └── patterns: [String]
├── appFiltering: AppFilteringConfig
│   ├── promptForNewApps: Bool
│   ├── scrobbleUnknown: Bool
│   ├── allowedApps: [String]
│   └── ignoredApps: [String]
├── lastfm: LastFmConfig?
│   ├── enabled: Bool
│   ├── apiKey: String
│   ├── apiSecret: String
│   └── sessionKey: String
└── listenbrainz: [ListenBrainzConfig]
    ├── enabled: Bool
    ├── name: String
    ├── token: String
    └── apiUrl: String
```

## Data Flow Diagrams

### Complete Scrobble Flow

```
┌─────────────────────────────────────────────────────────────┐
│ 1. User plays music in Music.app                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. macOS DistributedNotificationCenter broadcasts           │
│    "com.apple.Music.playerInfo" notification                │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. MediaMonitor receives notification                       │
│    - Parse artist, title, album, duration                   │
│    - Apply text cleanup (remove [Explicit], etc.)           │
│    - Check if IDAGIO → skip if true                         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Check app filtering                                      │
│    - Is app allowed? → Continue                             │
│    - Is app ignored? → Stop                                 │
│    - Is app unknown? → Prompt user                          │
└────────────────────────┬────────────────────────────────────┘
                         │ Allowed
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 5. Create/update PlaySession                                │
│    - Track metadata                                         │
│    - Start time                                             │
│    - Duration                                               │
│    - Scrobbled flag                                         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 6. Emit "now_playing" event immediately                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 7. ScrobbleManager.handleNowPlaying()                       │
│    ├─► MetadataEnricher.enrich() (async)                   │
│    │   ├─► MusicBrainz search                               │
│    │   └─► Last.fm album art                                │
│    ├─► LastFmService.nowPlaying()                           │
│    └─► ListenBrainzService.nowPlaying()                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 8. Update menu bar UI                                       │
│    - Show "Now Playing: Artist - Title"                     │
│    - Display album art thumbnail                            │
│    - Check loved status on Last.fm                          │
└─────────────────────────────────────────────────────────────┘
                         │
                         │ User continues listening...
                         │ (50% of track plays)
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 9. Scrobble threshold reached (50% or 4 min)                │
│    - PlaySession.shouldScrobble() → true                    │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 10. Emit "scrobble" event                                   │
│     - Track metadata                                        │
│     - Timestamp (when playback started)                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 11. ScrobbleManager.handleScrobble()                        │
│     ├─► LastFmService.scrobble(track, timestamp)            │
│     └─► ListenBrainzService.scrobble(track, timestamp)      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│ 12. Update UI                                               │
│     - Show "Last Scrobbled: Artist - Title"                 │
│     - Mark PlaySession as scrobbled                         │
└─────────────────────────────────────────────────────────────┘
```

### Love Track Flow

```
User clicks "Love" button
        │
        ▼
MenuBarView.onLove()
        │
        ▼
AppDelegate.loveCurrentTrack()
        │
        ├─► Parse "Artist - Title" string
        ├─► Create Track object
        └─► Task { await scrobbleManager.loveTrack(track) }
                │
                ▼
        ScrobbleManager.loveTrack()
                │
                ├─► Find LastFmService
                └─► service.love(track)
                        │
                        ▼
                LastFmService.love()
                        │
                        ├─► Generate MD5 signature
                        ├─► POST to Last.fm track.love API
                        └─► Update @Published isLoved = true
                                │
                                ▼
                        MenuBarView updates heart icon ❤️
```

## Threading & Concurrency

### Main Thread
- UI updates (SwiftUI, AppKit)
- Timer callbacks
- Notification handling
- Configuration changes

### Background Tasks (async/await)
- Network requests (URLSession)
- Metadata enrichment
- Image loading
- Cache updates

### Thread Safety
- `@MainActor` for UI updates
- `ImageCache` uses DispatchQueue for thread-safe access
- `UserDefaults` is thread-safe by default
- No shared mutable state between threads

## Error Handling Strategy

### Network Errors
```swift
do {
    try await service.scrobble(track, timestamp: date)
} catch ScrobbleError.networkError(let error) {
    NSLog("Network error: \(error)")
    // Retry logic could be added here
} catch ScrobbleError.authenticationFailed {
    NSLog("Authentication failed - check credentials")
    // Show alert to user
} catch {
    NSLog("Unknown error: \(error)")
}
```

### Configuration Errors
```swift
let errors = ConfigManager.shared.validate()
if !errors.isEmpty {
    // Show alert with all validation errors
    showAlert(errors.joined(separator: "\n"))
}
```

### Media Detection Errors
```swift
// Gracefully handle missing metadata
guard let track = extractTrackInfo(from: userInfo) else {
    NSLog("Failed to parse media info")
    return // Don't crash, just skip this update
}
```

## Performance Considerations

### Efficient Polling
- Timer with 0.5s tolerance for battery efficiency
- Only process events when media state changes
- Skip processing when no media is playing

### Caching
- Album art cached in memory (ImageCache)
- Last.fm album art results cached (MetadataEnricher)
- Configuration cached in memory (ConfigManager.shared)

### Async Operations
- All network calls are async (don't block UI)
- Metadata enrichment doesn't delay scrobbling
- Image loading doesn't block menu bar

### Memory Management
- ARC handles all memory automatically
- Weak references in closures ([weak self])
- Cache can be cleared if needed

## Security Considerations

### Credentials Storage
- API keys and session keys in UserDefaults
- UserDefaults is sandboxed per-app
- Not encrypted (consider Keychain for production)

### Network Security
- HTTPS for all API calls (Last.fm, MusicBrainz)
- No credential logging
- Signature-based auth for Last.fm (MD5)
- Token-based auth for ListenBrainz (Bearer)

### Code Signing
- Xcode handles signing for development
- Needs proper signing for distribution
- Notarization required for DMG distribution

## Future Improvements

### Performance
- [ ] Use Combine for reactive state management
- [ ] Add request deduplication
- [ ] Implement retry logic with exponential backoff
- [ ] Use CoreData for scrobble history

### Reliability
- [ ] Add offline queue for failed scrobbles
- [ ] Implement crash reporting
- [ ] Add health checks for services
- [ ] Better error recovery

### Features
- [ ] Background app refresh
- [ ] Notification center integration
- [ ] Statistics and charts
- [ ] Export scrobble history

---

This architecture provides a solid foundation for a maintainable, extensible music scrobbling application with clear separation of concerns and modern Swift best practices.
