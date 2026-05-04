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

/// Media-activity probe for the OSR sleep policy.
///
/// Returns the string `"true"` when at least one of the following is detected:
/// - `navigator.mediaSession.playbackState === 'playing'`  (YouTube, Spotify, etc.)
/// - `document.fullscreenElement instanceof HTMLVideoElement`  (fullscreen video)
///
/// The probe is fire-and-forget: the result is written to
/// `window.__buffr_media_active` and read by a follow-up JS call on the next
/// tick.  A return-value path (e.g. V8 context eval) is not exposed by
/// cef-rs's `Frame::execute_java_script`; using the window property sidesteps
/// that limitation without requiring a custom scheme bridge.
///
/// Scope: v1 covers the two cheapest signal sources.  Silent video, WebRTC,
/// Web Audio, and WakeLock detection are deferred to v2.
pub const MEDIA_PROBE_JS: &str = "(() => {\n  const ms = navigator.mediaSession?.playbackState === 'playing';\n  const fsv = document.fullscreenElement instanceof HTMLVideoElement;\n  window.__buffr_media_active = ms || fsv;\n})();";
