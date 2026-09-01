# 90 Render-on-demand — the biggest win never touches the image

> **Big picture.** *Phase 11.* Every frame so far was drawn at 60 fps whether anything changed or
> not — the archive did the same (`request_redraw()` unconditionally), which is why its heavy post
> stack burned laptops on *static* scenes. The fix is philosophical before it's technical: **games
> render time, CAD renders state.** If nothing changed, the last frame is still correct — draw
> nothing. The frame that *is* drawn stays full quality, always: this lesson changes **when** we
> draw, never **what** we draw. That's the user rule (no quality drop while interacting) honored by
> architecture instead of compromise.

<svg viewBox="0 0 680 120" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="input camera scene and ui changes set a dirty flag; the loop draws a full quality frame only when dirty, otherwise skips entirely" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.2"><rect x="10" y="14" width="120" height="22"/><rect x="10" y="42" width="120" height="22"/><rect x="10" y="70" width="120" height="22"/></g>
  <g fill="#d7dae0" font-size="10"><text x="20" y="29">camera moved</text><text x="20" y="57">scene mutated</text><text x="20" y="85">UI interacted</text></g>
  <g stroke="#6fb3ff" stroke-width="1.2"><line x1="130" y1="25" x2="200" y2="50" marker-end="url(#ah66)"/><line x1="130" y1="53" x2="200" y2="53" marker-end="url(#ah66)"/><line x1="130" y1="81" x2="200" y2="56" marker-end="url(#ah66)"/></g>
  <rect x="204" y="40" width="110" height="26" fill="none" stroke="#6fb3ff"/><text x="259" y="57" fill="#d7dae0" text-anchor="middle">dirty = true</text>
  <path d="M 314,53 h 40" stroke="#6fb3ff" stroke-width="1.2" marker-end="url(#ah66)"/>
  <rect x="358" y="18" width="160" height="26" fill="none" stroke="#6fb3ff"/><text x="438" y="35" fill="#d7dae0" text-anchor="middle">dirty → FULL frame</text>
  <rect x="358" y="62" width="160" height="26" fill="none" stroke="#3a3a3a"/><text x="438" y="79" fill="#888" text-anchor="middle">clean → SKIP entirely</text>
  <text x="600" y="35" fill="#666" font-size="10">identical quality,</text>
  <text x="600" y="49" fill="#666" font-size="10">moving or still</text>
  <text x="600" y="79" fill="#666" font-size="10">0% GPU idle</text>
  <defs><marker id="ah66" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/state.rs   # the dirty flag; mark_dirty() at every state-change site; render() gates on it
src/lib.rs     # RedrawRequested stops unconditionally re-requesting
src/ui/mod.rs  # HUD gains "frames drawn/s" beside fps — the number that proves it works (52 creates ui/)
```

## Step 1 — the flag and its sources: `src/state.rs`

One boolean, one setter, and an audit of *everything that changes what a frame would show*:

```rust
    // find `struct State { … }` → add four fields:
    pub dirty: bool,                       // init true (the first frame must draw)
    pub frames_drawn: u32,                 // ticked once per drawn frame (Step 3)
    pub frames_drawn_last_sec: u32,        // snapshotted for the HUD (Step 3)
    pub frames_sec_t0: f64,                // the snapshot clock (now_ms at last rollover)

    // in `State::new`, seed them in the returned `Ok(Self { … })`:
    //   dirty: true, frames_drawn: 0, frames_drawn_last_sec: 0, frames_sec_t0: now_ms(),

    pub fn mark_dirty(&mut self) { self.dirty = true; }
```

The setter call sites — this list *is* the lesson; each gets one call to the `poke()` helper we
build in Step 2 (`mark_dirty` + a redraw request), at code that already exists:

| source | where |
|---|---|
| camera orbit / pan / zoom / fit / projection | every handler that touches `Camera` (10–16) |
| resize | `resize()` |
| first file adopted (`Msg::Ready`) | `lib.rs` `user_event`, the `Ready` arm |
| progressive file appended (`Msg::File`) | `lib.rs` `user_event`, the `File` arm |
| selection & hover changes | 58's gestures, 66's gumball hover |
| any Command executed / undone / redone | `commit()`, the undo/redo verbs (64) |
| live gumball drag | `set_live_model` callers (67) |
| ghost preview updates | `set_preview` / `clear_preview` callers (71) |
| reconcile applied (watch/reload) | `apply_session` (49) |
| settings toggles, thickness slider | the apply-intent block (60) |
| egui needing a repaint (animations, cursor blink) | after `build_ui` returns (code below) |

The loader rows are the classic miss: both `user_event` arms already call `request_redraw()`, but
under the dirty gate that draw renders **nothing** unless the flag is set — poke there too, or the
2nd..Nth progressively loaded sheet never appears until the user happens to move the mouse.

> **An 11-row table is a liability — shrink it to choke points.** Every row above is a place a
> future edit can forget, and a missed poke fails *silently* (a stale frame looks plausible).
> Two defenses, both cheap:
> **Poke at choke points, not call sites.** All Commands already funnel through `commit()` (70)
> and undo/redo through `history` — one poke inside each retires whole rows of the table. Same
> for the camera: if its mutators go through one method (or `set_scene` for scene swaps), the
> poke rides along and the table shrinks to the few rows that *don't* pass a choke point.
> **A debug-mode missed-poke detector.** Keep a `scene_gen: u32` that the scene's mutating
> methods bump, and snapshot `drawn_gen` at each drawn frame. In debug builds, a *skipped* render
> with `scene_gen != drawn_gen` logs `missed poke: scene changed while clean` — turning the
> silent failure into a console line at development time, at zero release cost.

That last row matters: egui reports whether *it* wants another frame (a blinking CLI cursor does);
respect it or the text caret freezes. In `render()`, right after 60's
`let full_out = crate::ui::build_ui(…)` line, insert:

```rust
        // egui wants another frame (caret blink, animations) → poke like any other source
        if full_out.viewport_output.values().any(|v| v.repaint_delay.is_zero()) {
            self.poke();
        }
```

(This is egui-version-sensitive API — `viewport_output`/`repaint_delay` have shifted between
releases. If an egui upgrade changes the shape, the fallback that keeps the caret alive without
chasing the API: poke unconditionally while the CLI has keyboard focus — typing and a blinking
caret are the only *continuous* egui repaint sources this app has.)

## Step 2 — the gate: `src/state.rs` + `src/lib.rs`

The current loop *opens* `render()` with `self.window.request_redraw()` (schedule-the-next-frame,
then draw) — the unconditional 60 fps treadmill. Delete that first line and invert the logic:
`RedrawRequested` draws only if dirty, and only *input* (or egui) re-arms it:

```rust
    pub fn render(&mut self) -> anyhow::Result<()> {
        if !self.dirty {
            return Ok(());                          // clean → the old frame stands. NOTHING runs.
        }
        self.dirty = false;
        self.frames_drawn += 1;                     // the HUD counter (Step 3)
        // …the entire existing frame: cull (53), UI build (60), gpu.clear(…) — unchanged…
        // NOTE: no unconditional request_redraw() here anymore.
        Ok(())
    }
```

Every source in the table needs `mark_dirty()` *and* a redraw request — cheapest done together, one
helper on `impl State` that all the call sites use (`lib.rs`'s input arms call it as
`state.poke()`):

```rust
    pub fn poke(&mut self) { self.mark_dirty(); self.window.request_redraw(); }
```

(Sweep the table's rows to call `poke()`. The watch poll (45) and 50's debounce still need ticks
while idle — drive them from their own timer/`spawn_local` wakeups rather than the render loop, or
accept a low-rate heartbeat: `request_redraw` once per second from a timer, with `render` still
skipping cleanly when nothing is dirty. The heartbeat costs one no-op call, not a frame. It also
covers a CSS-only canvas resize: no winit resize event fires for those — today the treadmill's
`RedrawRequested` size check catches them, and with the treadmill gone only the heartbeat does.
The *right* fix for that last case is a `ResizeObserver` on the canvas that pokes on CSS resizes —
it replaces the heartbeat's resize-detection role entirely (the heartbeat then only services the
watch poll); it costs one more `web-sys` feature flag (`"ResizeObserver"`,
`"ResizeObserverEntry"`), which is why the heartbeat is shown first.)

## Step 3 — prove it on the HUD: `src/ui/mod.rs`

fps says how fast frames *can* draw; the new number says how many *did*. Three mechanical edits:

1. `UiState` (60) gains `pub frames_drawn_last_sec: u32,` (seed `0` in `State::new`'s `UiState`
   literal, beside the other HUD zeros).
2. In `render()`, the snapshot rolls over once a second — insert right after Step 2's
   `self.frames_drawn += 1;` line (`now_ms` is already imported from `engine::performance`):

```rust
        let now = now_ms();
        if now - self.frames_sec_t0 >= 1000.0 {
            self.frames_drawn_last_sec = self.frames_drawn;
            self.frames_drawn = 0;
            self.frames_sec_t0 = now;
        }
        self.ui.frames_drawn_last_sec = self.frames_drawn_last_sec;   // feed the HUD, 60's step 1
```

3. In `build_ui` (60), find the perf window's first label —
   `ui.label(format!("{:>5.1} fps   {:>5.2} ms", ui_state.fps, ui_state.frame_ms));` → replace with:

```rust
            ui.label(format!("{:>5.1} fps   {:>5.2} ms   {:>4} drawn/s",
                ui_state.fps, ui_state.frame_ms, ui_state.frames_drawn_last_sec));
```

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Load the stress file and **let go of the mouse**: `drawn/s` falls to **0** (or 1 with the
  heartbeat). Open the OS GPU monitor — utilization drops to idle. This is the whole win: the most
  expensive scene in the course now costs *nothing* to look at.
- **Orbit**: instant response — the first input event pokes a frame; no perceptible latency versus
  the old loop (input → dirty → redraw is the same frame). `drawn/s` ≈ display rate while moving.
- **The identity check** (the user rule, made falsifiable): screenshot mid-orbit, stop, screenshot at
  rest, diff — pixel-identical rendering path, because there *is* only one path. Nothing in this
  lesson knows how to render a "cheaper" frame.
- Type in the CLI → the caret keeps blinking (egui's repaint request is honored). Gumball drag, ghost
  preview, watch-reload — all animate, because every mutation site pokes.

## Recap

```
Ch 70: the stage — analytic ground.
Ch 71: RENDER-ON-DEMAND. Games render time; CAD renders STATE. One dirty flag; poke() = dirty +
       request_redraw at every site that changes what a frame shows (camera, selection, Commands,
       live drags, ghosts, reconcile, settings, egui's own repaint_delay==0 — the caret!). render()
       returns before touching the GPU when clean; no unconditional request_redraw treadmill. The
       drawn frame is ALWAYS the full-quality frame — this lesson has no second rendering path,
       which is how the no-quality-drop rule survives by construction. HUD: drawn/s beside fps —
       0 idle, display-rate while interacting. Static scenes now cost zero GPU; 79's AO rides on
       this: its cost is per DRAWN frame, and idle frames are free.
```

Edited: `state.rs` (`dirty`, `poke`, gated `render`, drawn/s counter), `lib.rs` (input pokes; no
treadmill), `ui/mod.rs` (drawn/s on the HUD).

## Next

`91-gtao.md` — ambient occlusion, the constant-quality way: half-resolution GTAO with a fixed tap
budget (~12 reads/px vs the archive's ~112), identical every frame moving or still — plus the
archive's hard-won implementation traps (the analytic inverse-projection, the tangent-plane gate, IGN
noise) ported, not re-derived.
