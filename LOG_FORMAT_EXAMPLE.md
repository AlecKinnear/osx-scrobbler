# Classical Track Logging Format

This document shows the expected log output when processing classical tracks from IDAGIO with MusicBrainz enrichment.

## Successful UPC Barcode Lookup

```
[2026-03-03 10:53:38] INFO - == Classical Track Processing ==
  Information from IDAGIO: artist="" title=" - Cannon: Lord of Light, String Quartet & 5 Chansons de femme 5020926113221" album=None
[2026-03-03 10:53:38] INFO -   Cleaned information: artist="" title="Cannon: Lord of Light, String Quartet & 5 Chansons de femme" upc=Some("5020926113221")
[2026-03-03 10:53:39] INFO -   Attempting MusicBrainz barcode lookup: UPC="5020926113221"
[2026-03-03 10:53:39] INFO -   Identified album: "Canon and Gigue in D Major & Other Works"
  Found matching length track: "Gigue in D Major"
[2026-03-03 10:53:39] INFO -   Final output: [menu bar] "Johann Pachelbel - Gigue in D Major"
```

## Fallback Album Name Lookup

When UPC lookup fails or no UPC is present:

```
[2026-03-03 10:54:08] INFO - == Classical Track Processing ==
  Information from IDAGIO: artist="" title=" - El último aliento 00028948583034" album=None
[2026-03-03 10:54:08] INFO -   Cleaned information: artist="" title="El último aliento" upc=Some("00028948583034")
[2026-03-03 10:54:08] INFO -   Attempting MusicBrainz barcode lookup: UPC="00028948583034"
[2026-03-03 10:54:09] WARN -   MusicBrainz barcode lookup: no match for UPC="00028948583034" at duration=325s
[2026-03-03 10:54:09] INFO -   Attempting MusicBrainz album name lookup: album="El último aliento"
[2026-03-03 10:54:10] INFO -   Identified album: "Cuartetas"
  Found matching length track: "El último aliento"
[2026-03-03 10:54:10] INFO -   Final output: [menu bar] "Composer Name - El último aliento"
```

## Lookup Failures

### No Match Found

```
[2026-03-03 10:55:00] INFO - == Classical Track Processing ==
  Information from IDAGIO: artist="" title=" - Unknown Track 1234567890123" album=None
[2026-03-03 10:55:00] INFO -   Cleaned information: artist="" title="Unknown Track" upc=Some("1234567890123")
[2026-03-03 10:55:00] INFO -   Attempting MusicBrainz barcode lookup: UPC="1234567890123"
[2026-03-03 10:55:01] WARN -   MusicBrainz barcode lookup: no match for UPC="1234567890123" at duration=300s
[2026-03-03 10:55:01] INFO -   Attempting MusicBrainz album name lookup: album="Unknown Track"
[2026-03-03 10:55:02] WARN -   MusicBrainz album lookup: no match for album="Unknown Track" at duration=300s
```

### API Failure

```
[2026-03-03 10:55:30] INFO - == Classical Track Processing ==
  Information from IDAGIO: artist="" title=" - Track 5020926113221" album=None
[2026-03-03 10:55:30] INFO -   Cleaned information: artist="" title="Track" upc=Some("5020926113221")
[2026-03-03 10:55:30] INFO -   Attempting MusicBrainz barcode lookup: UPC="5020926113221"
[2026-03-03 10:55:31] WARN -   MusicBrainz barcode lookup: FAILED - HTTP 503
```

## Log Levels

- **INFO**: Major steps in the processing pipeline
  - Raw information from IDAGIO
  - Cleaned information after parsing UPC
  - MusicBrainz lookup attempts
  - Identified album and track name
  - Final output for display

- **WARN**: Non-fatal issues or lookup failures
  - No match found in MusicBrainz
  - API errors
  - Missing required information (no duration, no album, no UPC)

- **DEBUG**: Implementation details
  - Text cleanup results
  - Release checking
  - Unused for simplified classical track pipeline

## Debugging Tips

1. **Track not being enriched?** Look for WARN logs about lookup failures
2. **Wrong artist/title?** Check the "Information from IDAGIO" line for what was extracted
3. **UPC not extracted?** Look at "Cleaned information" to see if UPC was found
4. **Album not found?** Check which lookup strategy was attempted and why it failed
