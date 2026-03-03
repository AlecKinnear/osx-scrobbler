# IDAGIO Track Processing Pipeline

Complete flow for detecting and enriching IDAGIO tracks:

## Step 1: Detection
**Location:** `src/media_monitor.rs:media_info_to_track()`

Detects IDAGIO by checking for "IDAGIO" string in metadata:
```rust
let is_idagio = title.contains("IDAGIO") 
             || artist.contains("IDAGIO") 
             || album.contains("IDAGIO");
```

**Why this works:** IDAGIO is a proprietary brand name, not an English word. Its presence in metadata reliably indicates the track source.

## Step 2: Metadata Cleaning
**Location:** `src/text_cleanup.rs:parse_classical_metadata()`

For IDAGIO tracks with empty artist:
- Remove IDAGIO brand suffixes (e.g., " | Stream on IDAGIO")
- Extract trailing digits as catalog ID (UPC field)
- Parse "by [Composer]" pattern to extract composer
- Return cleaned: `(artist, title, upc)`

**Output:** Track struct with:
- `artist`: Extracted composer name (if found)
- `title`: Cleaned piece name
- `upc`: Catalog ID (13 digits, e.g., "5020926113221")

## Step 3: Enrichment Trigger
**Location:** `src/main.rs:now_playing event handler`

If UPC is present and 13 digits:
```rust
if let Some(ref upc) = track.upc {
    if upc.len() == 13 && upc.chars().all(|c| c.is_numeric()) {
        // Spawn enrichment in background thread
        metadata_enricher::enrich_from_musicbrainz(&mut track, config);
    }
}
```

## Step 4: IDAGIO-Specific Enrichment
**Location:** `src/metadata_enricher.rs:enrich_idagio_track()`

1. **Search via Bing:** `search_idagio_album_page(catalog_id)`
   - Query: `"site:idagio.com intitle:{catalog_id}"`
   - Finds album page URL

2. **Fetch from API:** `fetch_and_parse_idagio_album(album_url, duration)`
   - Calls `https://api.idagio.com/v2.0/metadata/albums/{album_id}`
   - Requires bearer token (from config)
   - Matches track by duration
   - Returns: artist name, correct title

3. **Update Track:** 
   - Sets `track.artist` from API response
   - Sets `track.title` from matched track
   - UPC preserved for album art

## Step 5: Album Art
**Location:** `src/scrobbler.rs:Track::idagio_album_art_url()`

Constructs URL from catalog ID:
```rust
pub fn idagio_album_art_url(&self) -> Option<String> {
    self.upc.as_ref().map(|upc| {
        format!("https://idagio-images.global.ssl.fastly.net/albums/{}/main.jpg", upc)
    })
}
```

**Output:** Direct image URL ready for display
- Example: `https://idagio-images.global.ssl.fastly.net/albums/5020926113221/main.jpg`

## Complete Example Flow

**Input (from IDAGIO):**
```
Title:  " - Canon in D major 5020926113221"
Artist: "" (empty)
Album:  None
```

**After Detection & Cleaning:**
```
is_idagio: true
title:  "Canon in D major"
artist: "" (no "by" pattern found)
upc:    Some("5020926113221")
album:  Some("Canon in D major") (fallback)
```

**After Bing Search:**
- Found: `https://app.idagio.com/albums/canon-in-d-major-{uuid}`

**After API Fetch:**
```
artist: "Johann Pachelbel"
title:  "Canon in D Major"
album:  "Canon in D Major"
```

**Final Album Art:**
```
URL: https://idagio-images.global.ssl.fastly.net/albums/5020926113221/main.jpg
```

## Error Handling

Each step is resilient:
- Missing UPC → Skip enrichment, scrobble as-is
- Bing search fails → Log debug, continue
- API unreachable/token invalid → Log debug, continue
- Duration no match → Log debug, continue

Result: App always scrobbles something, even if enrichment fails.

## Configuration Required

In `~/.config/osx_scrobbler.conf`:
```toml
[idagio]
enabled = true
bearer_token = "your_token_here"
```

Without token: enrichment disabled (logs warning at startup)

## Testing Checklist

- [ ] IDAGIO track with "IDAGIO" in metadata
- [ ] UPC extraction (13 digits)
- [ ] Bing search working (no 403 rate limit)
- [ ] API call succeeds (token valid)
- [ ] Track matched by duration
- [ ] Artist/title updated
- [ ] Album art URL generated
- [ ] Non-IDAGIO tracks unaffected (Yandex, Spotify, etc.)
