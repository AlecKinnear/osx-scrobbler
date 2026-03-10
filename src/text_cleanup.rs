// Text cleanup module
// Applies regex patterns to clean up track/album/artist names

use crate::config::CleanupConfig;
use regex::Regex;

pub struct TextCleaner {
    enabled: bool,
    patterns: Vec<Regex>,
}

impl TextCleaner {
    /// Create a new text cleaner from config
    pub fn new(config: &CleanupConfig) -> Self {
        let patterns = if config.enabled {
            config
                .patterns
                .iter()
                .filter_map(|pattern| match Regex::new(pattern) {
                    Ok(re) => Some(re),
                    Err(e) => {
                        log::warn!("Invalid regex pattern '{}': {}", pattern, e);
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        Self {
            enabled: config.enabled,
            patterns,
        }
    }

    /// Clean a text string by applying all patterns
    pub fn clean(&self, text: &str) -> String {
        if !self.enabled {
            return text.to_string();
        }

        let mut result = text.to_string();
        for pattern in &self.patterns {
            result = pattern.replace_all(&result, "").to_string();
        }

        // Trim any extra whitespace
        result.trim().to_string()
    }

    /// Clean an optional string
    pub fn clean_option(&self, text: Option<String>) -> Option<String> {
        text.map(|s| self.clean(&s))
    }
}

/// Parse classical music metadata from sources like IDAGIO
/// Extracts composer as artist, cleans up title, and returns UPC if found
/// Returns (artist, title, upc)
pub fn parse_classical_metadata(artist: &str, title: &str) -> (String, String, Option<String>) {
    log::debug!(
        "parse_classical_metadata called - artist: {:?}, title: {:?}",
        artist,
        title
    );

    // If artist is empty/blank, try to extract from title
    if artist.trim().is_empty() {
        log::debug!("Artist is empty/blank, attempting to extract from title");

        let mut clean_title = title.to_string();
        let mut upc: Option<String> = None;
        
        // Remove pipe suffixes like " | Stream on IDAGIO | IDAGIO"
        if let Some(pipe_pos) = clean_title.find(" | Stream on IDAGIO") {
            clean_title.truncate(pipe_pos);
        } else if let Some(pipe_pos) = clean_title.find(" |") {
            // Fallback: remove any pipe and everything after
            clean_title.truncate(pipe_pos);
        }
        
        // Extract and remove IDAGIO UPC codes (digit sequences at the end)
        // Pattern: space followed by digits at the end of string
        let trimmed = clean_title.trim_end_matches(char::is_numeric);
        if trimmed.len() < clean_title.len() {
            // We have trailing digits - extract them as UPC
            let upc_str = clean_title[trimmed.len()..].to_string();
            if !upc_str.is_empty() {
                upc = Some(upc_str);
                log::debug!("Extracted UPC: {:?}", upc);
            }
            clean_title = trimmed.to_string();
        }
        clean_title = clean_title.trim_end().to_string();

        // Remove leading " - " if present
        let clean_title = if clean_title.starts_with(" - ") {
            clean_title[3..].to_string()
        } else {
            clean_title
        };

        log::debug!("Cleaned IDAGIO title (album name): {:?}, UPC: {:?}", clean_title, upc);

        // For Idagio, the cleaned title is the album name. We use this for the track title
        // and keep the artist empty, as requested. This is for display only and will not
        // be scrobbled.
        let result = ("".to_string(), clean_title, upc);
        log::debug!("Final parsed Idagio data: artist: '{}', title: '{}'", result.0, result.1);
        return result;
    } else {
        log::debug!("Artist is not empty, skipping classical parsing");
    }

    // Otherwise, return as-is
    (artist.to_string(), title.to_string(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disabled_cleaner_returns_unchanged() {
        let config = CleanupConfig {
            enabled: false,
            patterns: vec![r"\s*\[Explicit\]".to_string()],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(cleaner.clean("Song [Explicit]"), "Song [Explicit]");
    }

    #[test]
    fn test_removes_explicit_tags() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![
                r"\s*\[Explicit\]".to_string(),
                r"\s*\(Explicit\)".to_string(),
            ],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(cleaner.clean("Song [Explicit]"), "Song");
        assert_eq!(cleaner.clean("Song (Explicit)"), "Song");
        assert_eq!(cleaner.clean("Song [Explicit] (Explicit)"), "Song");
    }

    #[test]
    fn test_removes_clean_tags() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![r"\s*\[Clean\]".to_string()],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(cleaner.clean("Song [Clean]"), "Song");
    }

    #[test]
    fn test_trims_whitespace() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![r"\s*\[Explicit\]".to_string()],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(cleaner.clean("  Song [Explicit]  "), "Song");
    }

    #[test]
    fn test_multiple_patterns() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![
                r"\s*\[Explicit\]".to_string(),
                r"\s*- Remastered.*".to_string(),
            ],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(cleaner.clean("Song [Explicit] - Remastered 2020"), "Song");
    }

    #[test]
    fn test_clean_option_with_some() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![r"\s*\[Explicit\]".to_string()],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(
            cleaner.clean_option(Some("Song [Explicit]".to_string())),
            Some("Song".to_string())
        );
    }

    #[test]
    fn test_clean_option_with_none() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![r"\s*\[Explicit\]".to_string()],
        };
        let cleaner = TextCleaner::new(&config);

        assert_eq!(cleaner.clean_option(None), None);
    }

    #[test]
    fn test_invalid_pattern_is_skipped() {
        let config = CleanupConfig {
            enabled: true,
            patterns: vec![
                r"[invalid(".to_string(), // Invalid regex
                r"\s*\[Explicit\]".to_string(),
            ],
        };
        let cleaner = TextCleaner::new(&config);

        // Should still clean with the valid pattern
        assert_eq!(cleaner.clean("Song [Explicit]"), "Song");
    }

    #[test]
    fn test_parse_idagio_classical_metadata() {
        let (artist, title, upc) = parse_classical_metadata(
            "",
            " - Canon and Gigue in D major P 37 by Johann Pachelbel | Stream on IDAGIO | IDAGIO",
        );
        assert_eq!(artist, "");
        assert_eq!(title, "Canon and Gigue in D major P 37 by Johann Pachelbel");
        assert_eq!(upc, None);
    }

    #[test]
    fn test_parse_classical_metadata_without_by() {
        let (artist, title, upc) =
            parse_classical_metadata("", " - Perpetual Night: 17th Century Airs | Stream on IDAGIO | IDAGIO");
        assert_eq!(artist, "");
        assert_eq!(title, "Perpetual Night: 17th Century Airs");
        assert_eq!(upc, None);
    }

    #[test]
    fn test_parse_idagio_without_composer() {
        // IDAGIO track without composer info - just title and UPC
        let (artist, title, upc) =
            parse_classical_metadata("", " - Perpetual Night: 17th Century Airs and Songs 3149020933848");
        assert_eq!(artist, "");
        // Should strip UPC code and leading dash
        assert_eq!(title, "Perpetual Night: 17th Century Airs and Songs");
        assert_eq!(upc, Some("3149020933848".to_string()));
    }

    #[test]
    fn test_parse_idagio_with_upc_no_pipe() {
        // IDAGIO track with just UPC code (new format)
        let (artist, title, upc) =
            parse_classical_metadata("", " - Cannon: Lord of Light, String Quartet & 5 Chansons de femme 5020926113221");
        assert_eq!(artist, "");
        assert_eq!(title, "Cannon: Lord of Light, String Quartet & 5 Chansons de femme");
        assert_eq!(upc, Some("5020926113221".to_string()));
    }

    #[test]
    fn test_parse_classical_metadata_with_existing_artist() {
        let (artist, title, upc) = parse_classical_metadata("Artist", " - Title by Composer");
        assert_eq!(artist, "Artist");
        assert_eq!(title, " - Title by Composer");
        assert_eq!(upc, None);
    }
}
