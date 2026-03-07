import Foundation

class ScrobbleManager: ObservableObject {
    @Published var nowPlayingTrack: String?
    @Published var lastScrobbledTrack: String?
    @Published var albumArtUrl: String?
    @Published var isLoved: Bool = false

    private var services: [ScrobbleService] = []
    private let config: AppConfig

    init(config: AppConfig) {
        self.config = config
        setupServices()
    }

    private func setupServices() {
        services.removeAll()

        if let lastfm = config.lastfm, lastfm.enabled, !lastfm.sessionKey.isEmpty {
            NSLog("[ScrobbleManager] Last.fm scrobbler enabled")
            let service = LastFmService(
                apiKey: lastfm.apiKey,
                apiSecret: lastfm.apiSecret,
                sessionKey: lastfm.sessionKey
            )
            services.append(service)
        }

        for lb in config.listenbrainz where lb.enabled {
            NSLog("[ScrobbleManager] ListenBrainz scrobbler enabled: \(lb.name)")
            let service = ListenBrainzService(
                name: lb.name,
                token: lb.token,
                apiUrl: lb.apiUrl
            )
            services.append(service)
        }

        if services.isEmpty {
            NSLog("[ScrobbleManager] No scrobblers enabled")
        }
    }

    func handleNowPlaying(_ track: Track) async {
        nowPlayingTrack = "\(track.artist) - \(track.title)"
        albumArtUrl = track.lastfmAlbumArtUrl

        for service in services {
            do {
                try await service.nowPlaying(track)
            } catch {
                NSLog("[ScrobbleManager] Failed to send now playing to \(service.name): \(error)")
            }
        }

        if let lastfm = config.lastfm, lastfm.enabled, !lastfm.sessionKey.isEmpty {
            if let service = services.first(where: { $0 is LastFmService }) as? LastFmService {
                isLoved = (try? await service.isLoved(track)) ?? false
            }
        }
    }

    func handleScrobble(_ track: Track, timestamp: Date) async {
        lastScrobbledTrack = "\(track.artist) - \(track.title)"

        for service in services {
            do {
                try await service.scrobble(track, timestamp: timestamp)
            } catch {
                NSLog("[ScrobbleManager] Failed to scrobble to \(service.name): \(error)")
            }
        }
    }

    func loveTrack(_ track: Track) async {
        guard let lastfm = config.lastfm, lastfm.enabled else {
            NSLog("[ScrobbleManager] Last.fm not configured")
            return
        }

        if let service = services.first(where: { $0 is LastFmService }) as? LastFmService {
            do {
                try await service.love(track)
                isLoved = true
                NSLog("[ScrobbleManager] Track loved successfully")
            } catch {
                NSLog("[ScrobbleManager] Failed to love track: \(error)")
            }
        }
    }
}
