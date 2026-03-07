import Foundation

class MetadataEnricher {
    private static let musicBrainzAPI = "https://musicbrainz.org/ws/2"
    private static let lastFmAPI = "https://ws.audioscrobbler.com/2.0/"
    private static let durationToleranceMs: UInt64 = 3000
    private static var albumArtCache: [String: String?] = [:]

    static func enrich(_ track: inout Track, config: AppConfig?) async {
        guard let duration = track.duration else {
            NSLog("[MetadataEnricher] FAILED - no duration available")
            return
        }

        if let upc = track.upc, upc.count >= 12, upc.count <= 14, upc.allSatisfy({ $0.isNumber }) {
            NSLog("[MetadataEnricher] Attempting barcode lookup: UPC=\"\(upc)\"")

            if let (album, trackTitle) = try? await searchByBarcode(upc: upc, durationSecs: duration) {
                NSLog("[MetadataEnricher] Identified album: \"\(album)\"")
                NSLog("[MetadataEnricher] Found matching track: \"\(trackTitle)\"")
                track.album = album
                track.title = trackTitle

                if let cfg = config, let lastfm = cfg.lastfm, lastfm.enabled {
                    if let artUrl = try? await fetchLastFmAlbumArt(artist: track.artist, album: album, apiKey: lastfm.apiKey) {
                        track.lastfmAlbumArtUrl = artUrl
                        NSLog("[MetadataEnricher] Album art (Last.fm): \(artUrl)")
                    }
                }
                return
            }
        }

        guard let albumName = track.album, !albumName.isEmpty else {
            NSLog("[MetadataEnricher] FAILED - no album and no UPC available")
            return
        }

        NSLog("[MetadataEnricher] Attempting album name lookup: album=\"\(albumName)\"")

        if let (album, trackTitle) = try? await searchAlbum(albumName: albumName, durationSecs: duration) {
            NSLog("[MetadataEnricher] Identified album: \"\(album)\"")
            NSLog("[MetadataEnricher] Found matching track: \"\(trackTitle)\"")
            track.album = album
            track.title = trackTitle

            if let cfg = config, let lastfm = cfg.lastfm, lastfm.enabled {
                if let artUrl = try? await fetchLastFmAlbumArt(artist: track.artist, album: album, apiKey: lastfm.apiKey) {
                    track.lastfmAlbumArtUrl = artUrl
                    NSLog("[MetadataEnricher] Album art (Last.fm): \(artUrl)")
                }
            }
        }
    }

    private static func searchByBarcode(upc: String, durationSecs: UInt64) async throws -> (String, String)? {
        let durationMs = durationSecs * 1000
        let query = "barcode:\(upc)"

        guard let url = URL(string: "\(musicBrainzAPI)/release?query=\(query.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? "")&fmt=json&limit=5&inc=recordings") else {
            throw ScrobbleError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.setValue("OSX-Scrobbler/1.0.0 ( https://github.com/aleckinnear/osx-scrobbler )", forHTTPHeaderField: "User-Agent")

        let (data, _) = try await URLSession.shared.data(for: request)
        let result = try JSONDecoder().decode(MBReleaseSearchResult.self, from: data)

        return findMatchingTrack(in: result.releases ?? [], durationMs: durationMs)
    }

    private static func searchAlbum(albumName: String, durationSecs: UInt64) async throws -> (String, String)? {
        let durationMs = durationSecs * 1000
        let query = "release:\"\(albumName.replacingOccurrences(of: "\"", with: "\\\""))\""

        guard let url = URL(string: "\(musicBrainzAPI)/release?query=\(query.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? "")&fmt=json&limit=5&inc=recordings") else {
            throw ScrobbleError.invalidResponse
        }

        var request = URLRequest(url: url)
        request.setValue("OSX-Scrobbler/1.0.0 ( https://github.com/aleckinnear/osx-scrobbler )", forHTTPHeaderField: "User-Agent")

        let (data, _) = try await URLSession.shared.data(for: request)
        let result = try JSONDecoder().decode(MBReleaseSearchResult.self, from: data)

        return findMatchingTrack(in: result.releases ?? [], durationMs: durationMs)
    }

    private static func findMatchingTrack(in releases: [MBRelease], durationMs: UInt64) -> (String, String)? {
        for release in releases {
            guard let albumTitle = release.title else { continue }

            for medium in release.media ?? [] {
                for track in medium.tracks ?? [] {
                    if let mbDuration = track.length {
                        let durationDiff = abs(Int64(mbDuration) - Int64(durationMs))
                        if UInt64(durationDiff) <= durationToleranceMs {
                            if let trackTitle = track.title {
                                return (albumTitle, trackTitle)
                            }
                        }
                    }
                }
            }
        }
        return nil
    }

    static func fetchLastFmAlbumArt(artist: String, album: String, apiKey: String) async throws -> String? {
        let cacheKey = "\(artist)|\(album)"

        if let cached = albumArtCache[cacheKey] {
            return cached
        }

        guard let url = URL(string: "\(lastFmAPI)?method=album.getinfo&artist=\(artist.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? "")&album=\(album.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? "")&api_key=\(apiKey)&format=json") else {
            return nil
        }

        var request = URLRequest(url: url)
        request.setValue("OSX-Scrobbler/1.0.0 ( https://github.com/aleckinnear/osx-scrobbler )", forHTTPHeaderField: "User-Agent")

        let (data, _) = try await URLSession.shared.data(for: request)
        let result = try JSONDecoder().decode(LastFmAlbumResponse.self, from: data)

        if let images = result.album?.image {
            for image in images where image.size == "large" && !(image.url?.isEmpty ?? true) {
                albumArtCache[cacheKey] = image.url
                return image.url
            }
            for image in images where image.size == "extralarge" && !(image.url?.isEmpty ?? true) {
                albumArtCache[cacheKey] = image.url
                return image.url
            }
        }

        albumArtCache[cacheKey] = nil
        return nil
    }
}

struct MBReleaseSearchResult: Codable {
    let releases: [MBRelease]?
}

struct MBRelease: Codable {
    let title: String?
    let id: String?
    let media: [MBMedia]?
}

struct MBMedia: Codable {
    let trackCount: Int?
    let tracks: [MBTrack]?

    enum CodingKeys: String, CodingKey {
        case trackCount = "track-count"
        case tracks
    }
}

struct MBTrack: Codable {
    let title: String?
    let length: UInt64?
}

struct LastFmAlbumResponse: Codable {
    let album: LastFmAlbum?
}

struct LastFmAlbum: Codable {
    let image: [LastFmImage]?
}

struct LastFmImage: Codable {
    let size: String?
    let url: String?

    enum CodingKeys: String, CodingKey {
        case size
        case url = "#text"
    }
}
