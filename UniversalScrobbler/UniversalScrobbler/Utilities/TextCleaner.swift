import Foundation

class TextCleaner {
    private let enabled: Bool
    private let patterns: [NSRegularExpression]

    init(config: TextCleanupConfig) {
        self.enabled = config.enabled

        if config.enabled {
            self.patterns = config.patterns.compactMap { pattern in
                try? NSRegularExpression(pattern: pattern, options: [])
            }
        } else {
            self.patterns = []
        }
    }

    func clean(_ text: String) -> String {
        guard enabled else { return text }

        var result = text
        for pattern in patterns {
            let range = NSRange(result.startIndex..., in: result)
            result = pattern.stringByReplacingMatches(
                in: result,
                options: [],
                range: range,
                withTemplate: ""
            )
        }

        return result.trimmingCharacters(in: .whitespaces)
    }
}
