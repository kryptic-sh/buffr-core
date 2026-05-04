//! Embedded JavaScript snippets executed via `run_js` against CEF
//! frames.
//!
//! Every script lives as a `.js` asset under `crates/buffr-core/assets/`
//! and is pulled in here via [`include_str!`]. Keeping the bodies in
//! their own files lets editors apply real syntax highlighting and
//! ESLint checks; the Rust side just wires the bytes through.

/// Focus the first VISIBLE editable text field on the page (`i` / `gi`).
pub const FOCUS_FIRST_INPUT: &str = include_str!("../assets/focus_first_input.js");

/// Exit insert/edit mode: synthesize Escape keydown+keyup then blur.
pub const EXIT_INSERT: &str = include_str!("../assets/exit_insert.js");

/// Media-activity probe — init script (patched-constructor phase, v1.5).
///
/// Injected once per main-frame load via `LoadHandler::on_load_end` by
/// [`crate::handlers`].  Patches three browser APIs so that the poll script
/// can detect media activity that v1's cheap signals miss:
///
/// - **Silent / muted video** — patches `HTMLMediaElement.prototype.play` to
///   track all playing media elements (including muted `<video>` and
///   autoplay GIF-as-video that never set `mediaSession`).
/// - **WebRTC** — patches the `RTCPeerConnection` constructor to register
///   instances; the poll checks `connectionState !== 'closed'`.
/// - **Screen Wake Lock** — patches `navigator.wakeLock.request` to track
///   outstanding sentinels; the poll checks `sentinel.released === false`.
///
/// State is stored under `window.__buffr_media_state.{playingMedia,
/// peerConnections, wakeLocks}`.  An idempotency guard
/// (`window.__buffr_media_probe_installed`) prevents double-installation on
/// SPA soft-navigations.
///
/// Each patch is wrapped in `try { … } catch {}` so a failure in one signal
/// does not affect the others.
pub const MEDIA_PROBE_INIT_JS: &str = include_str!("../assets/media_probe_init.js");

/// Media-activity probe — poll script (v1.5).
///
/// Executed every ~2 s by [`crate::host::BrowserHost::run_media_probe`].
/// Recomputes `window.__buffr_media_active` from all five signal sources and
/// writes a boolean result that the Rust side reads on the next tick via
/// [`crate::host::BrowserHost::read_media_probe_result`].
///
/// Signal sources (combined with `||`):
/// 1. `navigator.mediaSession.playbackState === 'playing'`  (YouTube, Spotify)
/// 2. `document.fullscreenElement instanceof HTMLVideoElement`  (fullscreen video)
/// 3. Any `<video>` / `<audio>` tracked by `__buffr_media_state.playingMedia`
///    that is currently un-paused  (silent / muted video, non-mediaSession audio)
/// 4. Any `RTCPeerConnection` in `__buffr_media_state.peerConnections` with
///    `connectionState !== 'closed'`  (WebRTC calls)
/// 5. Any `WakeLockSentinel` in `__buffr_media_state.wakeLocks` with
///    `released === false`  (Screen Wake Lock API)
///
/// A return-value path (e.g. V8 context eval) is not exposed by
/// cef-rs's `Frame::execute_java_script`; using the `window` property
/// sidesteps that limitation without requiring a custom scheme bridge.
pub const MEDIA_PROBE_POLL_JS: &str = include_str!("../assets/media_probe_poll.js");
