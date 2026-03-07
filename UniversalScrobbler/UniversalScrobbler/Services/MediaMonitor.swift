import Foundation
import AppKit

class MediaMonitor: ObservableObject {
    @Published var currentTrack: Track?
    @Published var isPlaying: Bool = false

    private let scrobbleThreshold: Int
    private let textCleaner: TextCleaner
    private var currentSession: PlaySession?
    private var distributedNotificationObserver: Any?

    struct PlaySession {
        var track: Track
        var bundleId: String?
        var startedAt: Date
        var duration: UInt64
        var scrobbled: Bool = false
        var nowPlayingSent: Bool = false

        func elapsedSeconds() -> UInt64 {
            return UInt64(Date().timeIntervalSince(startedAt))
        }

        func shouldScrobble(threshold: Int) -> Bool {
            guard !scrobbled else { return false }
            guard duration >= 30 else { return false }

            let elapsed = elapsedSeconds()
            let thresholdTime = (duration * UInt64(threshold)) / 100
            let scrobbleAt = min(thresholdTime, 240)

            return elapsed >= scrobbleAt
        }

        func shouldSendNowPlaying() -> Bool {
            return !nowPlayingSent
        }
    }

    init(scrobbleThreshold: Int, textCleaner: TextCleaner) {
        self.scrobbleThreshold = scrobbleThreshold
        self.textCleaner = textCleaner
        setupMediaNotifications()
    }

    deinit {
        if let observer = distributedNotificationObserver {
            DistributedNotificationCenter.default().removeObserver(observer)
        }
    }

    private func setupMediaNotifications() {
        distributedNotificationObserver = DistributedNotificationCenter.default().addObserver(
            forName: NSNotification.Name("com.apple.Music.playerInfo"),
            object: nil,
            queue: .main
        ) { [weak self] notification in
            self?.handleMediaNotification(notification)
        }

        distributedNotificationObserver = DistributedNotificationCenter.default().addObserver(
            forName: NSNotification.Name("com.spotify.client.PlaybackStateChanged"),
            object: nil,
            queue: .main
        ) { [weak self] notification in
            self?.handleMediaNotification(notification)
        }
    }

    private func handleMediaNotification(_ notification: Notification) {
        guard let userInfo = notification.userInfo as? [String: Any] else { return }

        let playbackState = userInfo["Player State"] as? String ?? userInfo["State"] as? String
        isPlaying = (playbackState == "Playing")

        guard isPlaying else { return }

        if let track = extractTrackInfo(from: userInfo) {
            handleTrackChange(track: track, bundleId: extractBundleId(from: notification))
        }
    }

    private func extractTrackInfo(from userInfo: [String: Any]) -> Track? {
        guard let title = userInfo["Name"] as? String ?? userInfo["Title"] as? String,
              let artist = userInfo["Artist"] as? String else {
            return nil
        }

        let album = userInfo["Album"] as? String
        let duration = userInfo["Total Time"] as? UInt64 ?? userInfo["Duration"] as? UInt64

        if title.contains("IDAGIO") || artist.contains("IDAGIO") || album?.contains("IDAGIO") == true {
            NSLog("[MediaMonitor] IDAGIO detected - skipping")
            return nil
        }

        let cleanedTitle = textCleaner.clean(title)
        let cleanedArtist = textCleaner.clean(artist)
        let cleanedAlbum = album.map { textCleaner.clean($0) }

        return Track(
            title: cleanedTitle,
            artist: cleanedArtist,
            album: cleanedAlbum,
            duration: duration.map { $0 / 1000 }
        )
    }

    private func extractBundleId(from notification: Notification) -> String? {
        return notification.object as? String
    }

    func poll(appFiltering: AppFilteringConfig) -> MediaEvents {
        var events = MediaEvents()

        guard let workspace = NSWorkspace.shared.frontmostApplication,
              isPlaying else {
            if currentSession != nil {
                NSLog("[MediaMonitor] Media stopped, clearing session")
                currentSession = nil
            }
            return events
        }

        guard let track = currentTrack else { return events }

        let bundleId = workspace.bundleIdentifier

        switch shouldScrobbleApp(bundleId: bundleId, appFiltering: appFiltering) {
        case .ignore:
            NSLog("[MediaMonitor] Ignoring playback from \(bundleId ?? "unknown")")
            return events
        case .promptUser:
            if let id = bundleId {
                events.unknownApp = id
            }
            return events
        case .allow:
            break
        }

        let isNewTrack = currentSession == nil || currentSession?.track != track

        if isNewTrack {
            NSLog("[MediaMonitor] New track: \(track.artist) - \(track.title)")
            var newSession = PlaySession(
                track: track,
                bundleId: bundleId,
                startedAt: Date(),
                duration: track.duration ?? 0,
                scrobbled: false,
                nowPlayingSent: true
            )
            currentSession = newSession
            events.nowPlaying = (track, bundleId)
        } else if var session = currentSession {
            if session.shouldScrobble(threshold: scrobbleThreshold) {
                NSLog("[MediaMonitor] Scrobbling: \(session.track.artist) - \(session.track.title)")
                events.scrobble = (session.track, session.startedAt, session.bundleId)
                session.scrobbled = true
                currentSession = session
            } else if session.shouldSendNowPlaying() {
                events.nowPlaying = (session.track, session.bundleId)
                session.nowPlayingSent = true
                currentSession = session
            }
        }

        return events
    }

    private func shouldScrobbleApp(bundleId: String?, appFiltering: AppFilteringConfig) -> AppFilterAction {
        guard let id = bundleId, !id.isEmpty else {
            return appFiltering.scrobbleUnknown ? .allow : .ignore
        }

        if appFiltering.allowedApps.contains(id) {
            return .allow
        }

        if appFiltering.ignoredApps.contains(id) {
            return .ignore
        }

        return appFiltering.promptForNewApps ? .promptUser : .allow
    }

    enum AppFilterAction {
        case allow
        case ignore
        case promptUser
    }
}

struct MediaEvents {
    var nowPlaying: (Track, String?)?
    var scrobble: (Track, Date, String?)?
    var unknownApp: String?
}
