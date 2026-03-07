import Foundation

struct AppConfig: Codable {
    var refreshInterval: Int = 5
    var scrobbleThreshold: Int = 50
    var textCleanup: TextCleanupConfig = TextCleanupConfig()
    var appFiltering: AppFilteringConfig = AppFilteringConfig()
    var lastfm: LastFmConfig? = LastFmConfig()
    var listenbrainz: [ListenBrainzConfig] = [ListenBrainzConfig()]

    static let `default` = AppConfig()
}

struct TextCleanupConfig: Codable {
    var enabled: Bool = true
    var patterns: [String] = [
        "\\s*\\[Explicit\\]",
        "\\s*\\[Clean\\]",
        "\\s*\\(Explicit\\)",
        "\\s*\\(Clean\\)",
        "\\s*- Explicit",
        "\\s*- Clean"
    ]
}

struct LastFmConfig: Codable {
    var enabled: Bool = false
    var apiKey: String = ""
    var apiSecret: String = ""
    var sessionKey: String = ""
}

struct ListenBrainzConfig: Codable {
    var enabled: Bool = false
    var name: String = "Primary"
    var token: String = ""
    var apiUrl: String = "https://api.listenbrainz.org"
}

struct AppFilteringConfig: Codable {
    var promptForNewApps: Bool = true
    var scrobbleUnknown: Bool = true
    var allowedApps: [String] = []
    var ignoredApps: [String] = []
}

class ConfigManager {
    static let shared = ConfigManager()
    private let userDefaults = UserDefaults.standard
    private let configKey = "appConfig"

    private init() {}

    var config: AppConfig {
        get {
            if let data = userDefaults.data(forKey: configKey),
               let decoded = try? JSONDecoder().decode(AppConfig.self, from: data) {
                return decoded
            }
            return AppConfig.default
        }
        set {
            if let encoded = try? JSONEncoder().encode(newValue) {
                userDefaults.set(encoded, forKey: configKey)
            }
        }
    }

    func save() {
        if let encoded = try? JSONEncoder().encode(config) {
            userDefaults.set(encoded, forKey: configKey)
        }
    }

    func validate() -> [String] {
        var errors: [String] = []

        if config.refreshInterval == 0 {
            errors.append("Refresh interval must be greater than 0")
        }

        if config.scrobbleThreshold == 0 || config.scrobbleThreshold > 100 {
            errors.append("Scrobble threshold must be between 1 and 100")
        }

        if let lastfm = config.lastfm, lastfm.enabled {
            if lastfm.apiKey.isEmpty {
                errors.append("Last.fm API key is required when Last.fm is enabled")
            }
            if lastfm.apiSecret.isEmpty {
                errors.append("Last.fm API secret is required when Last.fm is enabled")
            }
        }

        for lb in config.listenbrainz where lb.enabled {
            if lb.token.isEmpty {
                errors.append("ListenBrainz token is required when enabled (instance: \(lb.name))")
            }
            if lb.apiUrl.isEmpty {
                errors.append("ListenBrainz API URL is required (instance: \(lb.name))")
            }
        }

        for bundleId in config.appFiltering.allowedApps {
            if config.appFiltering.ignoredApps.contains(bundleId) {
                errors.append("Bundle ID '\(bundleId)' appears in both allowed and ignored lists")
            }
        }

        return errors
    }
}
