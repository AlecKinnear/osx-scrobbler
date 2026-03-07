import Foundation

protocol ScrobbleService {
    var name: String { get }
    func nowPlaying(_ track: Track) async throws
    func scrobble(_ track: Track, timestamp: Date) async throws
    func love(_ track: Track) async throws
    func isLoved(_ track: Track) async throws -> Bool
}

enum ScrobbleError: Error, LocalizedError {
    case networkError(Error)
    case authenticationFailed
    case invalidResponse
    case apiError(String)

    var errorDescription: String? {
        switch self {
        case .networkError(let error):
            return "Network error: \(error.localizedDescription)"
        case .authenticationFailed:
            return "Authentication failed"
        case .invalidResponse:
            return "Invalid response from server"
        case .apiError(let message):
            return "API error: \(message)"
        }
    }
}
