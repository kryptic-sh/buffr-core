# Changelog

All notable changes to `buffr-core` are documented here. Format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), versioning follows
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] — 2026-05-03

### Changed

- **`profile_paths()` migrated to `hjkl-config` 0.2 (XDG-everywhere).** Cache +
  data dirs now come from `hjkl_config::cache_dir("buffr")` /
  `hjkl_config::data_dir("buffr")` instead of `directories::ProjectDirs`. Fixes
  a split-brain on macOS/Windows where `buffr-config` already routed through
  `hjkl-config` (writing `~/.config/buffr/config.toml`) but `buffr-core` was
  still resolving cache + data via the old `sh.kryptic.buffr` Bundle ID layout.
  Now every dir buffr touches is XDG-everywhere.
- macOS users: cache moves from `~/Library/Caches/sh.kryptic.buffr/` to
  `~/.cache/buffr/`; data moves from
  `~/Library/Application Support/sh.kryptic.buffr/` to `~/.local/share/buffr/`.
- Windows users: cache moves from `%LOCALAPPDATA%\kryptic\buffr\cache\` to
  `~/.cache/buffr/`; data moves from `%APPDATA%\kryptic\buffr\data\` to
  `~/.local/share/buffr/`.
- Linux users: paths unchanged (`~/.cache/buffr/`, `~/.local/share/buffr/`).
- Replaced `directories` dep with `hjkl-config = "0.2"`.

`CoreError::NoProjectDirs` variant name preserved for back-compat; semantics
widen slightly to "no XDG home dir resolvable" (only fires in sandboxed envs
without `$HOME`).

## [0.2.0] — 2026-05-03

### Added

- **`ClipboardReader`** opaque newtype + `BrowserHost::clipboard_handle()` so
  embedders can read the system clipboard from a worker thread without depending
  on `hjkl-clipboard` directly. `read_text()` performs the blocking Wayland read
  off the CEF UI thread to avoid the self-deadlock when Chromium owns the
  selection.
- **`BrowserHost::is_loading()`** flag, set by `BuffrLoadHandler::on_load_start`
  on main-frame loads and cleared by the next successful
  `OsrPaintHandler::on_paint`. Lets the embedder keep a loading animation
  playing across the navigation gap until the first contentful frame.
- **`BrowserHost::force_repaint_active`** atomic flag for embedder watchdogs to
  nudge a stuck CEF renderer via a `was_hidden` cycle.
- **`OsrFrame::needs_fresh`** flag set by `osr_resize` and cleared by the next
  successful main-frame paint. Lets the embedder's freshness gate reject
  persisted-but-stale paints after a resize burst.
- `RenderHandler::screen_info` plumbing for live device-scale changes
  (per-monitor HiDPI, fractional scaling toggle).

### Changed

- **`hjkl-clipboard` 0.3 → 0.4.** `Clipboard` becomes `Clone + Send + Sync`,
  enabling the worker-thread read pattern. New `Selection` / `MimeType` API.
- All paint / load handlers now plumb `loading_busy: Arc<AtomicBool>` through
  the factory functions.
- `OsrPaintHandler::on_paint` clears `needs_fresh` and `loading_busy` on
  successful main-frame paints.
- `osr_resize` invalidates the OSR view (`invalidate(VIEW)`) after tab
  activation so newly-fronted tabs commit a fresh paint.

### Fixed

- **Persistent letterbox / "two sizes behind" paint after rapid resize.**
  Before: the freshness gate accepted any paint at the right dims even if it was
  buffered from before the resize. After: `needs_fresh` requires a post-resize
  paint before re-presenting.

## [0.1.3] — 2026-04-30

### Fixed

- `build.rs` stages all CEF `Release/` DLLs and JSONs on Windows. Previously the
  build script missed runtime files needed by `cargo run` from a fresh checkout.

## [0.1.2] — 2026-04-30

### Changed

- `hjkl-clipboard` dep relaxed from exact-pin to caret `0.3` so consumers can
  pick up patch fixes without a buffr-core re-publish.

## [0.1.1] — 2026-04-30

### Changed

- Extracted from the `kryptic-sh/buffr` umbrella into a standalone repository
  with full git history preserved via `git subtree split`.
- Added per-repo CI (fmt / clippy / test matrix / cargo-deny) and a tag-driven
  release workflow that publishes idempotently to crates.io.

[Unreleased]: https://github.com/kryptic-sh/buffr-core/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/kryptic-sh/buffr-core/releases/tag/v0.3.0
[0.2.0]: https://github.com/kryptic-sh/buffr-core/releases/tag/v0.2.0
[0.1.3]: https://github.com/kryptic-sh/buffr-core/releases/tag/v0.1.3
[0.1.2]: https://github.com/kryptic-sh/buffr-core/releases/tag/v0.1.2
[0.1.1]: https://github.com/kryptic-sh/buffr-core/releases/tag/v0.1.1
