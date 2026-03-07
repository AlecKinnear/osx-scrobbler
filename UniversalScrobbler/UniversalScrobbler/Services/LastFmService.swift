import Foundation
import CryptoKit

class LastFmService: ScrobbleService {
    let name = "Last.fm"
    private let apiKey: String
    private let apiSecret: String
    private let sessionKey: String
    private let apiUrl = "https://ws.audioscrobbler.com/2.0/"

    init(apiKey: String, apiSecret: String, sessionKey: String) {
        self.apiKey = apiKey
        self.apiSecret = apiSecret
        self.sessionKey = sessionKey
    }

    func nowPlaying(_ track: Track) async throws {
        var params: [String: String] = [
            "method": "track.updateNowPlaying",
            "artist": track.artist,
            "track": track.title,
            "api_key": apiKey,
            "sk": sessionKey
        ]

        if let album = track.album {
            params["album"] = album
        }

        if let duration = track.duration {
            params["duration"] = String(duration)
        }

        let signature = generateSignature(params: params)
        params["api_sig"] = signature
        params["format"] = "json"

        try await performRequest(params: params)
        NSLog("[Last.fm] Now playing updated")
    }

    func scrobble(_ track: Track, timestamp: Date) async throws {
        var params: [String: String] = [
            "method": "track.scrobble",
            "artist": track.artist,
            "track": track.title,
            "timestamp": String(Int(timestamp.timeIntervalSince1970)),
            "api_key": apiKey,
            "sk": sessionKey
        ]

        if let album = track.album {
            params["album"] = album
        }

        if let duration = track.duration {
            params["duration"] = String(duration)
        }

        let signature = generateSignature(params: params)
        params["api_sig"] = signature
        params["format"] = "json"

        try await performRequest(params: params)
        NSLog("[Last.fm] Scrobbled successfully")
    }

    func love(_ track: Track) async throws {
        let params: [String: String] = [
            "method": "track.love",
            "artist": track.artist,
            "track": track.title,
            "api_key": apiKey,
            "sk": sessionKey
        ]

        let signature = generateSignature(params: params)
        var signedParams = params
        signedParams["api_sig"] = signature

        try await performRequest(params: signedParams)
        NSLog("[Last.fm] Track loved")
    }

    func isLoved(_ track: Track) async throws -> Bool {
        let params: [String: String] = [
            "method": "track.getInfo",
            "artist": track.artist,
            "track": track.title,
            "api_key": apiKey,
            "sk": sessionKey
        ]

        let signature = generateSignature(params: params)
        var signedParams = params
        signedParams["api_sig"] = signature
        signedParams["format"] = "json"

        guard let url = URL(string: apiUrl) else {
            throw ScrobbleError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")
        request.httpBody = signedParams.percentEncoded()

        let (data, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            return false
        }

        if let json = try? JSONSerialization.jsonObject(with: data) as? [String: Any],
           let trackInfo = json["track"] as? [String: Any],
           let userloved = trackInfo["userloved"] as? String,
           userloved == "1" {
            return true
        }

        return false
    }

    private func generateSignature(params: [String: String]) -> String {
        let sorted = params.sorted { $0.key < $1.key }
        let concatenated = sorted.map { "\($0.key)\($0.value)" }.joined()
        let signatureString = concatenated + apiSecret

        let hash = Insecure.MD5.hash(data: Data(signatureString.utf8))
        return hash.map { String(format: "%02hhx", $0) }.joined()
    }

    private func performRequest(params: [String: String]) async throws {
        guard let url = URL(string: apiUrl) else {
            throw ScrobbleError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.httpMethod = "POST"
        request.setValue("application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")
        request.httpBody = params.percentEncoded()

        let (_, response) = try await URLSession.shared.data(for: request)

        guard let httpResponse = response as? HTTPURLResponse,
              (200...299).contains(httpResponse.statusCode) else {
            throw ScrobbleError.apiError("HTTP error")
        }
    }
}

extension Dictionary where Key == String, Value == String {
    func percentEncoded() -> Data? {
        return map { key, value in
            let escapedKey = key.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? key
            let escapedValue = value.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? value
            return "\(escapedKey)=\(escapedValue)"
        }
        .joined(separator: "&")
        .data(using: .utf8)
    }
}
