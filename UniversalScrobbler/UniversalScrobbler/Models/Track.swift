import Foundation

struct Track: Equatable, Codable {
    var title: String
    var artist: String
    var album: String?
    var duration: UInt64?
    var upc: String?
    var lastfmAlbumArtUrl: String?

    init(title: String, artist: String, album: String? = nil, duration: UInt64? = nil, upc: String? = nil, lastfmAlbumArtUrl: String? = nil) {
        self.title = title
        self.artist = artist
        self.album = album
        self.duration = duration
        self.upc = upc
        self.lastfmAlbumArtUrl = lastfmAlbumArtUrl
    }

    static func == (lhs: Track, rhs: Track) -> Bool {
        return lhs.title == rhs.title &&
               lhs.artist == rhs.artist &&
               lhs.album == rhs.album
    }
}
