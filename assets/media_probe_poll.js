// buffr media-activity probe — poll script.
//
// Executed every ~2 s by `BrowserHost::run_media_probe`. Recomputes
// `window.__buffr_media_active` from all five signal sources and writes a
// boolean result. The companion init script (`media_probe_init.js`) must
// have run first to install the patched constructors; if it hasn't, the
// three patched-constructor signals simply read as false.
//
// Signal sources (combined with ||):
//   1. navigator.mediaSession.playbackState === 'playing'
//   2. document.fullscreenElement instanceof HTMLVideoElement
//   3. Any <video>/<audio> tracked by __buffr_media_state that is un-paused
//   4. Any RTCPeerConnection with connectionState !== 'closed'
//   5. Any WakeLockSentinel with released === false

(function () {
    'use strict';

    var active = false;

    // ── Signal 1: mediaSession ─────────────────────────────────────────────
    try {
        if (navigator.mediaSession && navigator.mediaSession.playbackState === 'playing') {
            active = true;
        }
    } catch (_e) {}

    // ── Signal 2: fullscreen video ─────────────────────────────────────────
    try {
        if (!active && document.fullscreenElement instanceof HTMLVideoElement) {
            active = true;
        }
    } catch (_e) {}

    // ── Signal 3: playing media elements (patched-ctor) ───────────────────
    // The WeakSet has no iterator; instead we scan all <video> and <audio>
    // elements on the page and check paused + tracked-membership.
    try {
        if (!active) {
            var state = window.__buffr_media_state;
            if (state && state.playingMedia) {
                var els = document.querySelectorAll('video, audio');
                for (var i = 0; i < els.length; i++) {
                    if (!els[i].paused && state.playingMedia.has(els[i])) {
                        active = true;
                        break;
                    }
                }
            }
        }
    } catch (_e) {}

    // ── Signal 4: RTCPeerConnection (patched-ctor) ─────────────────────────
    try {
        if (!active) {
            var state = window.__buffr_media_state;
            if (state && state.peerConnections && state.peerConnections.size > 0) {
                state.peerConnections.forEach(function (pc) {
                    if (!active &&
                        pc.connectionState !== 'closed' &&
                        pc.connectionState !== 'failed' &&
                        pc.iceConnectionState !== 'closed') {
                        active = true;
                    }
                });
            }
        }
    } catch (_e) {}

    // ── Signal 5: Screen Wake Lock (patched-ctor) ──────────────────────────
    try {
        if (!active) {
            var state = window.__buffr_media_state;
            if (state && state.wakeLocks && state.wakeLocks.size > 0) {
                state.wakeLocks.forEach(function (sentinel) {
                    if (!active && !sentinel.released) {
                        active = true;
                    }
                });
            }
        }
    } catch (_e) {}

    window.__buffr_media_active = active;
})();
