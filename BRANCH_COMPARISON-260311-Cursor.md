# Branch comparison: rust-home vs rust-office

**Purpose:** Compare the two branches and note where each is stronger, so you can decide what (if anything) to bring from rust-office into rust-home. This is a **comparison and critique**, not a to-do list to execute.

---

## How they get "now playing"

| | rust-home | rust-office |
|--|-----------|-------------|
| **Trigger** | Event-driven: macOS MediaRemote observers (`NowPlayingInfoDidChange`, etc.) push `UserEvent::MediaStateChanged`; 250 ms debounce; timer only for scrobble deadline | Polling: winit wakes at `next_poll_time`; `media_monitor.poll()` every `refresh_interval` (default 20 s) |
| **Data source** | `fetch_media_snapshot()` in main: JXA `get_raw_info()` in a **separate process** (avoids in-process MediaRemote C API and SIGSEGV risk) | `NowPlayingJXA::get_info()` inside the app (media-remote crate, in-process) |
| **Scrobble timing** | Snapshot-based: `PlaySession` keeps `elapsed_secs`, `is_playing`, `last_snapshot_at`; `played_seconds()` extrapolates; `check_scrobble()` + `next_scrobble_deadline()` for timer | Wall-clock: only `started_at`; `elapsed_seconds()` = now − started_at; scrobble checked on each poll |

---

## Where rust-home is stronger

1. **Stability** – JXA runs in another process; a bad MediaRemote state can't SIGSEGV the app. rust-office's `NowPlayingJXA::get_info()` is in-process and may still use the same C API.
2. **Responsiveness** – Events + debounce give quick "now playing" updates; rust-office only sees changes every `refresh_interval`.
3. **Scrobble accuracy** – Snapshot + extrapolation tracks real playback; rust-office can drift if the event loop is late.
4. **IDAGIO** – Only rust-home has full support (config, parsing, enrichment, no-scrobble gating, album art).
5. **Album art** – rust-home has a real album-art channel and `fetch_and_display_album_art`; rust-office only updates tray state and leaves the window unimplemented.
6. **Icons / install** – Optional `.icns`, in-repo `resources/iconset/icon_32.png`, template icon, quarter-note labels; rust-office requires .icns and uses a different path.

---

## Where rust-office is stronger (gaps in rust-home)

1. **Config** – `refresh_interval` in config (default 20 s); rust-home has no user-tunable interval for debounce/timer.
2. **Concurrency** – Plain `Vec<Service>`, Love/is_loved on main; rust-home uses `Arc<Mutex<Vec<Service>>>` and threads + events.
3. **API shape** – `Service::is_loved()` on the enum; rust-home has a standalone `lastfm_is_loved()` and event callback.
4. **Metadata delay** – rust-office retries when `is_playing` but title is empty (Apple Music / Yandex), with short sleeps and a few attempts; rust-home doesn't.
5. **User-Agent** – rust-office uses `env!("CARGO_PKG_NAME")` / `env!("CARGO_PKG_VERSION")`; rust-home uses a hardcoded string in the enricher (and album art).
6. **ListenBrainz** – rust-office skips ListenBrainz when artist is too short in one clear place; rust-home has similar logic in a spawned thread—worth confirming it matches.
7. **Docs** – rust-office has IDAGIO_REMOVED.md, SWIFT_ARCHITECTURE.md, SWIFT_VS_RUST.md and .env in .gitignore; rust-home doesn't.

---

## Summary table (architecture / behavior)

| Aspect | rust-home | rust-office |
|--------|-----------|-------------|
| Media source | Events + debounce; timer for scrobble | Fixed-interval polling |
| Now-playing read | JXA `get_raw_info()` (out-of-process) | `NowPlayingJXA::get_info()` (in-process) |
| Scrobble timing | Snapshot + extrapolation | Wall-clock |
| Config | No `refresh_interval` | `refresh_interval` (default 20) |
| IDAGIO | Full support | Removed |
| Scrobblers | `Arc<Mutex<Vec<Service>>>`, threads + events | `Vec<Service>`, sync on main |
| Album art | Channel + fetch_and_display | Tray only; window "not implemented" |
| now_playing payload | `(Track, Option<String>, bool)` (should_scrobble) | `(Track, Option<String>)` |
| is_loved | `lastfm_is_loved()` + thread + event | `Service::is_loved()` on main |
| Icons | Optional .icns, template, quarter-note labels | Required .icns, different path |

---

## How to use this

- Treat this as **reference material** to decide what you care about improving on rust-home (e.g. metadata-delay retry, User-Agent, `Service::is_loved`, docs, optional `refresh_interval`).
- Any "recommended improvements" from earlier discussion are **suggested direction**, not a checklist you're obliged to implement. You can adopt none, some, or all, depending on your goals.
