import Foundation

class ListenBrainzService: ScrobbleService {
    let name: String
    private let token: String
    private let apiUrl: String

    init(name: String, token: String, apiUrl: String) {
        self.name = name
        self.token = token
        self.apiUrl = apiUrl
    }

    func nowPlaying(_ track: Track) async throws {
        let payload: [String: Any] = [
            "listen_type": "playing_now",
            "payload": [[
                "track_metadata": [
                    "artist_name": track.artist,
                    "track_name": track.title,
                    "release_name": track.album ?? ""
                ]
            ]]
        ]

        try await performRequest(endpoint: "/1/submit-listens", payload: payload)
        NSLog("[ListenBrainz (\(name))] Now playing updated")
    }

    func scrobble(_ track: Track, timestamp: Date) async throws {
        let payload: [String: Any] = [
            "listen_type": "single",
            "payload": [[
                "listened_at": Int(timestamp.timeIntervalSince1970),
                "track_metadata": [
                    "artist_name": track.artist,
                    "track_name": track.title,
                    "release_name": track.album ?? ""
                ]
            ]]
        ]

        try await performRequest(endpoint: "/1/submit-listens", payload: payload)
        NSLog("[ListenBrainz (\(name))] Scrobbled successfully")
    }

    func love(_ track: Track) async throws {
        NSLog("[ListenBrainz (\(name))] Love/feedback not yet implemented")
    }

    func isLoved(_ track: Track) async throws -> Bool {
        return false
    }

    private func performRequest(endpoint: String, payload: [String: Any]) async throws {
        guard let url = URL(string: apiUrl + endpoint) else {
            throw ScrobbleError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/json", forHTTPHeaderField: "Content-Type")
        request.setValue("Token \(token)", forHTTPHeaderField: "Authorization")

        request.httpBody = try JSONSerialization.data(withJSONObject: payload)

        let (_, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw ScrobbleError.apiError("HTTP error")
        }
    }

    static func validate(token: String, apiUrl: String) async throws {
        guard let url = URL(string: apiUrl + "/1/validate-token") else {
            throw ScrobbleError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.httpMethod = "GET"
        request.setValue("Token \(token)", forHTTPHeaderField: "Authorization")

        let (_, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw ScrobbleError.authenticationFailed
        }
    }
}
