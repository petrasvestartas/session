# Refactoring updates — 2026-09-02

What went into `src/` **after** the lessons were written, and what a future session must do to
fold it into the 44–51 chain. Nothing in `docs/` was touched by this work: every lesson file,
44–51 included, is byte-for-byte what it was before (the four already-dirty docs — 44, 46, 47,
48 — still carry only the earlier, unrelated edits from 13:55–14:30 today).

The chain's problem is structural, not accidental. Lessons 46–51 rebuild the tree by **whole-file
`Create` ops**: lesson 50 alone re-creates `src/lib.rs` (`docs/50-walk-and-shell.md:2558`),
`src/app/mod.rs` (`:2430`) and `src/app/scene.rs` (`:2209`) from scratch. Anything typed into
those files after the lessons were written is not *conflicted* by the chain — it is **silently
deleted** by it. No error, no warning: an undeclared `.rs` simply stops being compiled.

---

## 1. What is new in the tree

### `src/app/touch.rs` — new file, 199 lines

The gesture state machine for touchscreens. Before it, `WindowEvent::Touch` fell through
`window_event`'s `_ => {}` arm and a phone could not move the camera at all.

| gesture | camera call | the mouse binding it mirrors |
|---|---|---|
| one finger, drag | `Camera::orbit` | right-drag |
| two fingers, slide | `Camera::pan` | middle-drag |
| two fingers, spread / close | `Camera::zoom_at` their midpoint | wheel |
| double tap | `Camera::fit` | `F` |

Public surface: `pub struct Touches` (`new`, `event`), `pub enum Act { None, Moved, Fit }`.
It depends on `winit::event::{Touch, TouchPhase}`, `crate::camera::Camera` and
`crate::engine::performance::now_ms` — nothing else, and nothing platform-specific, so it
compiles on the native target the pixel gate uses.

Three decisions worth carrying forward, because they are the parts that are easy to get wrong:

- **Gestures are converted to CSS pixels** (divide by `devicePixelRatio`). winit reports
  PHYSICAL pixels (`to_physical(scale_factor)` in `web_sys/pointer.rs`), so the same centimetre
  of finger travel is 3× the number on a dpr-3 phone that it is on a dpr-1 laptop. Orbit is a
  fixed radians-per-unit, so the raw figure spins the model at a different speed on every device.
- **Two-finger pan is finger-exact.** `Camera::pan` scales by a hard-coded `distance * 0.0015`,
  which equals the `2·tan(30°)` the projection really spans only at a viewport 769.80 px tall
  (`2 · tan 30° / 0.0015 = 769.8003589`). `PAN_PER_PX / viewport_height` corrects it. Exact in
  both projections — the orthographic branch of `view_proj_anchored` uses the same
  `distance · tan(30°)` half-height as the perspective one.
- **Pinch inverts the wheel detent.** `zoom_at` scales distance by `1 - amount·0.1`; a pinch
  gives a ratio `r`, so `1 - amount·0.1 = 1/r` ⇒ `amount = 10·(1 - 1/r)`, clamped to `r ∈ [½, 2]`
  per event so a finger the browser loses and re-delivers cannot teleport the camera.

### `src/lib.rs` — five additions

| line | what |
|---|---|
| 139 | `use crate::app::touch::{Act, Touches};` |
| 155 | `touch: Touches,` field on `struct App` (inserted **below** `ctrl: bool,`) |
| 172 | `touch: Touches::new(),` in `App::run` |
| 474–484 | the `WindowEvent::Touch(t)` arm |
| 503 | `fn device_pixel_ratio()` |

Plus one unrelated repair: the doc comment above `spin_mode` said "The canvas's pixel size (CSS
size × device-pixel-ratio)" — it belonged to `desired_canvas_size`, two functions down.

### `src/app/mod.rs` — one line

`pub mod touch;`

### `index.html` — the page-level opt-out

A canvas gets no gestures until the page stops the browser claiming them first.

- `<meta name="viewport" …, maximum-scale=1.0, user-scalable=no, viewport-fit=cover>` — the
  page's own pinch-zoom would fight the model's. iOS Safari has ignored `user-scalable` since
  iOS 10, which is why the canvas rule below matters more.
- `html, body { height: 100%; overscroll-behavior: none; }` — no pull-to-refresh, no rubber band
  under a downward drag that is meant to be an orbit.
- `canvas { … height: 100dvh; touch-action: none; -webkit-touch-callout: none;
  -webkit-user-select: none; user-select: none; -webkit-tap-highlight-color: transparent; }`

`touch-action: none` is the load-bearing one. winit *does* call `preventDefault`, on
`pointerdown` and on a non-passive `touchstart` (`winit-0.30.13` `web_sys/pointer.rs:115`,
`web_sys/canvas.rs:245`), but that is a late veto — the compositor has already begun deciding
whether the gesture is a scroll, and where it decides yes the app gets a `pointercancel` and
nothing more.

---

## 2. Why the touch code is where it is

`src/app/touch.rs` was chosen over the two alternatives on measurement, not taste:

- **Not inline in `lib.rs`.** That file carries 15 anchored ops before lesson 50 and 5 after it,
  and the pre-50 anchors are single lines distinguished only by indentation — e.g.
  `docs/46-pipeline-descs.md:1854` anchors `state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);`
  at 28 spaces, `:1828` the same call at 20 spaces preceded by its `let aspect`. A double-tap-fit
  written inline inside `window_event` lands at 16–20 spaces and duplicates the `:1828` anchor,
  which makes lesson 46 fail. Inline placement is one indentation level from breaking the chain.
- **Not top-level `src/touch.rs`.** No lesson would touch it, but at end-of-51 there are exactly
  five top-level modules (`lib`, `state`, `camera`, `math`, `selftest`) and every input concern
  lives under `app/`. `ARCHITECTURE.md:47` names the seam: *"a drag, a wheel, a key →
  `app/input.rs`"*.
- **`src/app/touch.rs`** survives all eight lessons byte-identical (no lesson creates, edits,
  deletes or anchors on that path) and lands one step from its final home, beside the
  `src/app/input.rs` that lesson 50 creates.

**Anchors checked, and clear.** Nothing in 46–51 quotes `struct App`, `orbiting`, `panning`,
`last_cursor`, `ctrl`, `MouseInput`, `CursorMoved`, `MouseWheel`, `ModifiersChanged`,
`ApplicationHandler` or the `fn window_event` signature as a `Find` anchor — those strings occur
in lesson 50 only as `Create` **output**. Lesson 48's one "panning" hit is the word *spanning*.
Neither `index.html` nor `Cargo.toml` is the target of any op in 46–51 (grep: 0 hits in all six).

**One anchor that survives by luck, and must keep surviving.** Lesson 44 *does* anchor on
today's `App`, twice: `docs/44-streaming-cloud.md:1087` (`    ctrl: bool,`) and `:1099`
(`            ctrl: false,`), each adding a `fitted` line below. `touch: Touches,` was inserted
**after** `ctrl: bool,`, so both anchors still match exactly once (verified: `grep -c` = 1 each).
They would NOT survive renaming `ctrl`, a second `ctrl: bool,` line, or reordering the field list.

---

## 3. The fold-in, for a future session

Three items. All mechanical; none needs a design decision.

### (a) Lesson 46 — one line, a hard compile break

`src/lib.rs:481` reads

```rust
state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, vp.0 / vp.1);
```

Lesson 46 replaces `Gpu`'s two `scene_min` / `scene_max` fields with a single `bounds: Aabb`
(`docs/51_refactored/src/engine/gpu/mod.rs:76`). No op covers the touch arm, so after lesson 46
the tree carries a dangling `state.gpu.scene_min` — a straight `E0609` at that lesson's **Check**
step, and it stays red through 47, 48 and 49 until lesson 50's whole-file `Create` removes the
line altogether.

**Fix:** `state.gpu.bounds.min, state.gpu.bounds.max`, as a new op in lesson 46 (or stated in its
Check prose). The break is intrinsic to double-tap-fit — any fit needs scene bounds — not to the
file choice.

### (b) Lesson 50 — the wiring is deleted in silence

Lesson 50's two `Create`s wipe every reference to touch: after it,
`grep -ci touch src/lib.rs` = 0 and `grep -c 'mod touch' src/app/mod.rs` = 0, while
`src/app/touch.rs` sits on disk whole and orphaned. Four things to re-add:

1. `docs/50-walk-and-shell.md:2430`, the `Create src/app/mod.rs` block: add `pub mod touch;`
   to the nine-module list. **`src/app/live.rs` is missing from that list too** — the live-data
   loader post-dates the refactor block and is orphaned by exactly the same mechanism. Fold both
   in together.
2. `docs/50-walk-and-shell.md:2076`, Step 19's `Create src/app/input.rs`: give `struct Input`
   a `pub touch: Touches` field and `Input::new()` its initialiser, and add a
   `WindowEvent::Touch(t) => { … }` arm to `Input::mouse`.
3. `device_pixel_ratio()` moves with it — into `app/touch.rs` or `app/knobs.rs`.
4. Nothing else. Lesson 50's `window_event` already ends
   `other => self.input.mouse(state, &other)` (`docs/51_refactored/src/lib.rs:139`), so
   `WindowEvent::Touch` **already reaches `Input`** and is merely swallowed by its `_ => false`
   arm (`docs/51_refactored/src/app/input.rs`, last arm). The fold-in touches `input.rs` only;
   `lib.rs` needs no edit at all.

### (c) Post-50 simplification, free

Lesson 51 turns the input handlers into "did anything change?" booleans feeding
`state.needs_frame`. So the three `state.window.request_redraw()` calls in today's `Touch` arm
become `true` returns, and `Act::Fit` resolves as `state.fit_all()` — the helper lesson 50 adds
at `docs/51_refactored/src/state.rs:72` — instead of naming the bounds fields at all, which also
makes item (a) moot from lesson 50 onward.

### Not affected

`src/camera.rs` — lessons 46–50 have **zero** ops on it and lesson 51 has exactly three, all
comment typo fixes (`docs/51-performance-memory.md:478`, `:490`, `:503`). `Camera::orbit`,
`pan`, `zoom_at` and `fit` are the API `touch.rs` calls, and they are the most stable surface in
the tree: touch code written against them today keeps compiling through the whole chain.

`docs/_gate.sh` — a pure input change cannot move a gate number. The gate builds
`examples/selftest.rs` **native**, where `App` and `window_event` are `#[cfg(target_arch =
"wasm32")]` and do not compile in at all; the harness never constructs an event loop. The one way
an input change *can* break the gate is by failing to compile natively, which is why
`app/touch.rs` carries no `web_sys` call.

### Also worth folding in at the same time

The lesson snapshots under `docs/*/index.html` (48 of them) still carry `height: 100vh` and no
`touch-action` — they are frozen Trunk templates inside end-of-lesson trees, so they are correct
as history, but any lesson that re-publishes a page should pick up the mobile rules from §1.

---

## 4. How this was verified

- **Build.** `cargo check --target wasm32-unknown-unknown` and
  `cargo check --target x86_64-unknown-linux-gnu --all-targets` both clean;
  `cargo xtest` 8 passed / 0 failed. `cargo clippy` reports **zero** warnings in `app/touch.rs`
  and no new warning anywhere (the two in `lib.rs` — `Msg`'s enum size and a collapsible `if` in
  the resize block — predate this work).
- **The module is really in the build.** A temporary `compile_error!` appended to
  `src/app/touch.rs` turned the check red, then was removed. (A guard that has never gone red
  proves nothing.)
- **The gestures, in a real browser**, through the real winit path: Chrome at
  `127.0.0.1:8770`, synthetic `PointerEvent`s with `pointerType: "touch"` dispatched on the
  canvas (`button: -1` on moves — otherwise winit reads a chorded button and returns before the
  touch branch; `coalescedEvents` populated, because winit reads `getCoalescedEvents()`).
  - one-finger drag +400 px → orbited ≈ 2 rad, as `−dx·0.005` predicts;
  - two-finger spread 400 → 1000 px → zoomed in, no orbit;
  - two-finger slide −480 px → panned −480 px, **measured −270 screenshot px against −273
    predicted** — pan is finger-exact;
  - double tap → fit;
  - `pointercancel` mid-drag → the finger is dropped and the next one-finger drag still orbits
    (a stranded finger would silently turn every later single-finger drag into a two-finger
    gesture — the highest-impact failure mode in the file, and the reason it is tested).
- **A measurement trap, recorded so it is not re-hit.** The double tap appeared broken at first.
  It was not: the MCP browser tab was in the background, where Chrome throttles timers, and a
  `setTimeout(120)` between the two taps was taking **1000 ms** — past the 320 ms window. Both
  tap clocks are read when the event is *handled*, not when it happened (winit's `Touch` carries
  no timestamp), so any stall longer than the window splits a double tap into two singles. At
  30–60 fps that stall is one frame and does not matter.

## 5. State of the tree

Changed by this work: `src/app/touch.rs` (new), `src/app/mod.rs`, `src/lib.rs`, `index.html`.
Untouched: everything under `docs/`, `src/camera.rs`, `assets/`, `Cargo.toml`.

One thing to watch for: `docs/_replay_check.py` will **write into the live tree** if it is given
`.` as its source. During this session it silently applied lesson 51's `display_only = true` ops
to six `assets/scenes/*.toml`; they were reverted with `git checkout`. Always replay into a copy.
