import SwiftUI
import AppKit

class AppDelegate: NSObject, NSApplicationDelegate {
    private var statusItem: NSStatusItem?
    private var popover: NSPopover?
    private var mediaMonitor: MediaMonitor?
    private var scrobbleManager: ScrobbleManager?
    private var timer: Timer?
    private var settingsWindow: NSWindow?

    func applicationDidFinishLaunching(_ notification: Notification) {
        NSLog("[AppDelegate] Application starting")

        let config = ConfigManager.shared.config
        let validationErrors = ConfigManager.shared.validate()

        if !validationErrors.isEmpty {
            NSLog("[AppDelegate] Configuration errors: \(validationErrors.joined(separator: ", "))")
        }

        NSLog("[AppDelegate] Refresh interval: \(config.refreshInterval)s")
        NSLog("[AppDelegate] Scrobble threshold: \(config.scrobbleThreshold)%")

        let textCleaner = TextCleaner(config: config.textCleanup)
        if config.textCleanup.enabled {
            NSLog("[AppDelegate] Text cleanup enabled with \(config.textCleanup.patterns.count) patterns")
        }

        mediaMonitor = MediaMonitor(scrobbleThreshold: config.scrobbleThreshold, textCleaner: textCleaner)
        scrobbleManager = ScrobbleManager(config: config)

        setupMenuBar()
        startPolling(interval: TimeInterval(config.refreshInterval))

        NSLog("[AppDelegate] Universal Scrobbler started")
    }

    private func setupMenuBar() {
        statusItem = NSStatusBar.system.statusItem(withLength: NSStatusItem.variableLength)

        if let button = statusItem?.button {
            if let image = NSImage(named: "musical-note") {
                image.isTemplate = true
                button.image = image
            } else {
                button.image = NSImage(systemSymbolName: "music.note", accessibilityDescription: "macOS Scrobbler")
            }
            button.action = #selector(togglePopover)
            button.target = self
        }

        let popover = NSPopover()
        popover.contentSize = NSSize(width: 300, height: 400)
        popover.behavior = .transient

        if let scrobbleManager = scrobbleManager {
            popover.contentViewController = NSHostingController(
                rootView: MenuBarView(
                    scrobbleManager: scrobbleManager,
                    onLove: { [weak self] in self?.loveCurrentTrack() },
                    onShowAlbumArt: { [weak self] in self?.showAlbumArt() },
                    onSettings: { [weak self] in self?.openSettings() },
                    onQuit: { NSApplication.shared.terminate(nil) }
                )
            )
        }

        self.popover = popover

        NSLog("[AppDelegate] Menu bar initialized")
    }

    @objc private func togglePopover() {
        guard let button = statusItem?.button else { return }

        if let popover = popover {
            if popover.isShown {
                popover.performClose(nil)
            } else {
                popover.show(relativeTo: button.bounds, of: button, preferredEdge: .minY)
            }
        }
    }

    private func startPolling(interval: TimeInterval) {
        timer = Timer.scheduledTimer(withTimeInterval: interval, repeats: true) { [weak self] _ in
            self?.pollMedia()
        }
        timer?.tolerance = 0.5
    }

    private func pollMedia() {
        guard let mediaMonitor = mediaMonitor,
              let scrobbleManager = scrobbleManager else { return }

        let config = ConfigManager.shared.config
        let events = mediaMonitor.poll(appFiltering: config.appFiltering)

        if let (track, bundleId) = events.nowPlaying {
            NSLog("[AppDelegate] Now playing: \(track.artist) - \(track.title) from \(bundleId ?? "unknown")")

            Task {
                var enrichedTrack = track
                await MetadataEnricher.enrich(&enrichedTrack, config: config)
                await scrobbleManager.handleNowPlaying(enrichedTrack)
            }
        }

        if let (track, timestamp, bundleId) = events.scrobble {
            NSLog("[AppDelegate] Scrobble: \(track.artist) - \(track.title) at \(timestamp) from \(bundleId ?? "unknown")")

            Task {
                await scrobbleManager.handleScrobble(track, timestamp: timestamp)
            }
        }

        if let bundleId = events.unknownApp {
            handleUnknownApp(bundleId: bundleId)
        }
    }

    private func handleUnknownApp(bundleId: String) {
        NSLog("[AppDelegate] Prompting user for app: \(bundleId)")

        let choice = showAppPromptDialog(bundleId: bundleId)
        var config = ConfigManager.shared.config

        switch choice {
        case .allow:
            NSLog("[AppDelegate] User allowed app: \(bundleId)")
            if !config.appFiltering.allowedApps.contains(bundleId) {
                config.appFiltering.allowedApps.append(bundleId)
                ConfigManager.shared.config = config
                ConfigManager.shared.save()
                NSLog("[AppDelegate] Added \(bundleId) to allowed apps")
            }
        case .ignore:
            NSLog("[AppDelegate] User ignored app: \(bundleId)")
            if !config.appFiltering.ignoredApps.contains(bundleId) {
                config.appFiltering.ignoredApps.append(bundleId)
                ConfigManager.shared.config = config
                ConfigManager.shared.save()
                NSLog("[AppDelegate] Added \(bundleId) to ignored apps")
            }
        }
    }

    private func loveCurrentTrack() {
        guard let scrobbleManager = scrobbleManager,
              let trackString = scrobbleManager.nowPlayingTrack else {
            NSLog("[AppDelegate] No track currently playing to love")
            return
        }

        let parts = trackString.split(separator: " - ", maxSplits: 1)
        guard parts.count == 2 else { return }

        let track = Track(
            title: String(parts[1]),
            artist: String(parts[0])
        )

        Task {
            await scrobbleManager.loveTrack(track)
        }
    }

    private func showAlbumArt() {
        guard let url = scrobbleManager?.albumArtUrl else {
            NSLog("[AppDelegate] No album art available")
            return
        }

        NSLog("[AppDelegate] Opening album art window: \(url)")
        showAlbumArtWindow(imageUrl: url)
    }

    private func openSettings() {
        if settingsWindow == nil {
            let window = NSWindow(
                contentRect: NSRect(x: 0, y: 0, width: 600, height: 500),
                styleMask: [.titled, .closable, .resizable],
                backing: .buffered,
                defer: false
            )
            window.title = "Settings"
            window.center()
            window.contentView = NSHostingView(rootView: SettingsView())
            settingsWindow = window
        }

        settingsWindow?.makeKeyAndOrderFront(nil)
        NSApp.activate(ignoringOtherApps: true)
    }

    func applicationWillTerminate(_ notification: Notification) {
        NSLog("[AppDelegate] Application shutting down")
        timer?.invalidate()
    }

    func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
        if !flag {
            togglePopover()
        }
        return true
    }
}
