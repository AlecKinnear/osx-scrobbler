# OSX Scrobbler - Agent Instructions

## Build & Test Commands

- **Build**: `cargo build`
- **Run**: `cargo run` or `RUST_LOG=debug cargo run` for debug logging
- **Test**: `cargo test`
- **Single test**: `cargo test <test_name>`
- **Lint**: `cargo clippy`
- **Format**: `cargo fmt`
- **Release build**: `cargo build --release`

## Architecture & Structure

A lightweight macOS menu bar application (no dock icon) that monitors media players and scrobbles to Last.fm and ListenBrainz.

**Core modules** (`src/`):
- `main.rs` - Event loop, tray icon, app bundle installation, Last.fm auth flow
- `config.rs` - Config loading/saving (TOML format, `~/Library/Application Support/osx_scrobbler.conf`)
- `media_monitor.rs` - Polls macOS Media Remote for current track and scrobble eligibility
- `scrobbler.rs` - Last.fm and ListenBrainz HTTP API integrations
- `text_cleanup.rs` - Regex-based text cleanup for track/album/artist names
- `ui/tray.rs` - System tray menu with now playing and last scrobbled tracks

**Key APIs**:
- **Media Remote** (`media-remote` crate) - Detects music player changes across all apps
- **Last.fm** (`rustfm-scrobble-proxy`, manual HTTP via `attohttpc`) - Session-based scrobbling
- **ListenBrainz** (`listenbrainz` crate) - Token-based scrobbling (supports self-hosted)
- **System Tray** (`tray-icon`, `winit`) - Event-based menu bar integration

**Configuration**:
- Global settings: `refresh_interval` (polling), `scrobble_threshold` (% or 4 min)
- Text cleanup: regex patterns applied in order
- App filtering: allowlist/denylist by bundle ID with optional prompts
- Multiple Last.fm and ListenBrainz endpoints supported

## Code Style & Conventions

- **Format**: `cargo fmt` (standard Rust)
- **Linting**: `cargo clippy` - fix all warnings
- **Line length**: Keep under 100 characters when reasonable
- **Error handling**: Prefer `?` operator; use `expect()` with descriptive messages only; avoid `unwrap()` except tests
- **Imports**: Group by standard, external crates, internal modules
- **Naming**: snake_case functions, PascalCase types/traits, SCREAMING_SNAKE_CASE constants
- **Types**: Use `Result<T>` from `anyhow` for error handling
- **Logging**: Use `log::info!`, `log::warn!`, `log::error!` macros
- **Doc comments**: Add for public APIs and complex functions
- **Testing**: Add unit tests for pure functions; use descriptive test names
- **Commits**: Present tense, under 72 chars first line, reference issues/PRs
