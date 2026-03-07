import Foundation
import AppKit

class ImageCache {
    static let shared = ImageCache()

    private var cache: [String: NSImage] = [:]
    private let cacheQueue = DispatchQueue(label: "com.universalscrobbler.imagecache")

    private init() {}

    func image(for url: String) -> NSImage? {
        return cacheQueue.sync {
            return cache[url]
        }
    }

    func setImage(_ image: NSImage, for url: String) {
        cacheQueue.async { [weak self] in
            self?.cache[url] = image
        }
    }

    func loadImage(from urlString: String) async -> NSImage? {
        if let cached = image(for: urlString) {
            return cached
        }

        guard let url = URL(string: urlString) else { return nil }

        do {
            let (data, _) = try await URLSession.shared.data(from: url)
            if let image = NSImage(data: data) {
                setImage(image, for: urlString)
                return image
            }
        } catch {
            NSLog("[ImageCache] Failed to load image from \(urlString): \(error)")
        }

        return nil
    }

    func clearCache() {
        cacheQueue.async { [weak self] in
            self?.cache.removeAll()
        }
    }
}
