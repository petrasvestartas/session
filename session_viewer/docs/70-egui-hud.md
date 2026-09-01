# 70 egui overlay — the HUD and first settings

> **Big picture.** *Phase 8 — the interface (60–64).* The locked design is **commands-only**: no
> toolbar forest, a command line like Rhino's. Before the CLI can exist there must be a UI layer to
> type into — that's egui, an immediate-mode GUI drawn as one extra render pass over the 3-D frame.
> This lesson wires it in and pays rent immediately: the perf HUD graduates from the console to a
> panel, and the first settings (grid, edges, projection, line thickness) become checkboxes. 61 puts
> the command line in the same overlay.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="winit events go to egui first and are dropped if consumed; each frame the 3D pass renders, then the egui pass renders over it on the same encoder" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <text x="10" y="20" fill="#888">input — every winit event:</text>
  <rect x="10" y="30" width="90" height="28" fill="none" stroke="#6fb3ff"/><text x="55" y="48" fill="#d7dae0" text-anchor="middle">event</text>
  <rect x="140" y="30" width="120" height="28" fill="none" stroke="#6fb3ff"/><text x="200" y="48" fill="#d7dae0" text-anchor="middle">egui FIRST</text>
  <rect x="330" y="12" width="150" height="24" fill="none" stroke="#3a3a3a"/><text x="405" y="28" fill="#888" text-anchor="middle">consumed → stop</text>
  <rect x="330" y="46" width="150" height="24" fill="none" stroke="#6fb3ff"/><text x="405" y="62" fill="#d7dae0" text-anchor="middle">else → camera/pick</text>
  <line x1="100" y1="44" x2="138" y2="44" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah47)"/>
  <line x1="260" y1="38" x2="328" y2="26" stroke="#3a3a3a" stroke-width="1.1" marker-end="url(#ah47)"/>
  <line x1="260" y1="50" x2="328" y2="58" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah47)"/>
  <text x="10" y="106" fill="#888">frame — one encoder:</text>
  <rect x="10" y="116" width="150" height="26" fill="none" stroke="#6fb3ff"/><text x="85" y="133" fill="#d7dae0" text-anchor="middle">3-D passes (clear…)</text>
  <rect x="200" y="116" width="170" height="26" fill="none" stroke="#6fb3ff"/><text x="285" y="133" fill="#d7dae0" text-anchor="middle">egui pass (LoadOp::Load)</text>
  <line x1="160" y1="129" x2="198" y2="129" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah47)"/>
  <text x="420" y="133" fill="#666">→ present — UI always on top</text>
  <defs><marker id="ah47" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/ui/mod.rs     # NEW — Shell (egui ctx/state/renderer), UiState (settings), build_ui
src/lib.rs        # mod ui; route every winit event to egui FIRST
src/state.rs      # State.shell + State.ui; render() builds the UI then hands it to the gpu
src/engine/gpu/mod.rs  # clear() gains the egui pass (same encoder, after the 3-D passes)
```

The deps are already pinned from lesson 02 (`egui`, `egui-wgpu`, `egui-winit`, `egui_extras`, all
0.34 in `Cargo.toml`) — nothing to add. All call shapes below are ported from the archive's working
wiring, not reconstructed from docs.

## Step 1 — the Shell: `src/ui/mod.rs` (NEW)

Three objects: the egui **context** (layout + theming), the **winit bridge** (translates window
events), and the **wgpu renderer** (draws egui's triangles). Plus `UiState` — the plain struct the
widgets bind to:

```rust
//! The egui overlay. Rule: widgets bind to the plain `UiState` struct inside the closure;
//! `State` APPLIES those values after the closure returns — never mutate State mid-layout
//! (can't borrow it).

pub struct Shell {
    pub ctx: egui::Context,
    pub state: egui_winit::State,
    pub renderer: egui_wgpu::Renderer,
}

pub struct UiState {
    pub show_grid: bool,
    pub show_edges: bool,
    pub ortho: bool,
    pub thickness: f32,
    // HUD inputs — State copies the latest numbers in before build_ui each frame:
    pub fps: f32,
    pub frame_ms: f32,
    pub draws: u32,
    pub drawn: u32,
    pub total: u32,
    // 58's rubber band — Some((press, cursor)) in screen points while a marquee drag is live:
    pub marquee: Option<((f32, f32), (f32, f32))>,
}

impl Shell {
    pub fn new(window: &winit::window::Window, device: &wgpu::Device,
               format: wgpu::TextureFormat) -> Self {
        let ctx = egui::Context::default();
        let mut vis = egui::Visuals::light();
        vis.selection.bg_fill = egui::Color32::BLACK;              // selected row: black bg…
        vis.selection.stroke  = egui::Stroke::new(1.0, egui::Color32::WHITE);
        vis.override_text_color = Some(egui::Color32::BLACK);      // …never white-on-white
        ctx.set_visuals(vis);
        let renderer = egui_wgpu::Renderer::new(device, format,
            egui_wgpu::RendererOptions::default());
        let state = egui_winit::State::new(ctx.clone(), egui::ViewportId::ROOT,
            window, None, None, None);
        Self { ctx, state, renderer }
    }
}

/// Lay out the whole overlay for this frame. Widgets mutate `ui_state` (a plain struct — fine);
/// anything that must touch State/Gpu is applied by the CALLER from ui_state afterwards.
pub fn build_ui(shell: &mut Shell, window: &winit::window::Window,
                ui_state: &mut UiState) -> egui::FullOutput {
    let raw_input = shell.state.take_egui_input(window);
    shell.ctx.run(raw_input, |ctx| {
        egui::Window::new("perf").default_pos([8.0, 8.0]).resizable(false).show(ctx, |ui| {
            ui.label(format!("{:>5.1} fps   {:>5.2} ms", ui_state.fps, ui_state.frame_ms));
            ui.label(format!("{} draws   {} / {} drawn",
                ui_state.draws, ui_state.drawn, ui_state.total));
        });
        egui::Window::new("settings").default_pos([8.0, 96.0]).resizable(false).show(ctx, |ui| {
            ui.checkbox(&mut ui_state.show_grid, "grid");
            ui.checkbox(&mut ui_state.show_edges, "edges");
            ui.checkbox(&mut ui_state.ortho, "orthographic");
            ui.add(egui::Slider::new(&mut ui_state.thickness, 0.5..=6.0).text("line px"));
        });
        // 58's rubber band, at last: one painter rect over everything while the drag is live.
        if let Some((a, b)) = ui_state.marquee {
            let rect = egui::Rect::from_two_pos(egui::pos2(a.0, a.1), egui::pos2(b.0, b.1));
            ctx.layer_painter(egui::LayerId::new(
                egui::Order::Foreground, egui::Id::new("marquee")))
                .rect(rect, 0.0, egui::Color32::from_rgba_unmultiplied(111, 179, 255, 24),
                      egui::Stroke::new(1.0, egui::Color32::from_rgb(111, 179, 255)));
        }
    })
}
```

(A free extra if you want it: `scene.docs.len()` and each `Doc.name` already exist — one more
label in the perf window shows exactly which files are loaded.)

Add `mod ui;` in `lib.rs`, and to `struct State`: `pub shell: crate::ui::Shell, pub ui:
crate::ui::UiState`. Build the `Shell` in `State::new` right after the two real lines

```rust
        let mut gpu = Gpu::new(window.clone()).await?;
        gpu.set_scene(&scene.tables);
```

(it needs the device and surface format), and seed `UiState` from the current camera/thickness
values — the `ortho` flag is the inverse of the camera's `perspective`. One catch: `State::new` has
**no `camera` binding** to read — 35 constructs it inline in the `Ok(Self { … })` literal
(`camera: Camera::new()`). Hoist it into a binding first, change the literal's
`camera: Camera::new()` to plain `camera,`, and add both new fields (`shell`/`ui`) to the literal:

```rust
        // in State::new, right after gpu.set_scene(&scene.tables):
        let camera = Camera::new();              // hoisted out of the Ok(Self { … }) literal
        let shell = crate::ui::Shell::new(&window, &gpu.device, gpu.config.format);
        let ui = crate::ui::UiState {
            show_grid: gpu.show_grid, show_edges: gpu.show_edges,
            ortho: !camera.perspective,          // camera stores `perspective`; UI shows its inverse
            thickness: gpu.thickness,
            fps: 0.0, frame_ms: 0.0, draws: 0, drawn: 0, total: 0,
            marquee: None,
        };
```

Three of those sources don't exist yet — small enablers, all in `engine/`:

- **`Gpu.thickness`** — `thickness: 2.0` has been hardcoded in TWO places since 31: `Gpu::new`'s
  initial `LineUniform` and `clear()`'s per-frame write. Add `pub thickness: f32,` to `struct Gpu`,
  `thickness: 2.0,` to its `Ok(Self { … })`, and in **`clear()`**'s per-frame `LineUniform` replace
  `thickness: 2.0,` with `thickness: self.thickness,` — that per-frame write is the one the slider
  drives; `Gpu::new`'s init value is overwritten on frame 1, so it can stay.
- **`Gpu.show_grid` / `show_edges`** — Step 4's gates; add the fields now (see the note there).
- **Perf readouts** — 28's `Performance` keeps its numbers private and never stored the draw
  count. In `engine/performance.rs`, add `pub last_draws: u32,` to the struct (init `0` in
  `Performance::new`), set it in `frame()` (first line: `self.last_draws = draws;`), and add two
  accessors to `impl Performance`:

```rust
    pub fn fps(&self) -> f32 {
        if self.frame_ms > 0.0 { (1000.0 / self.frame_ms) as f32 } else { 0.0 }
    }
    pub fn ms(&self) -> f32 { self.frame_ms as f32 }
```

(`drawn`/`total` come from 53's `perf_drawn`/`perf_total` on `Gpu`.)

## Step 2 — events go to egui first: `src/lib.rs`

Find the `window_event` handler (where orbit/pan/keys live). Before any 3-D handling:

```rust
        // Route the event to egui FIRST; if it consumed it (typing in a field, dragging a slider),
        // the camera must NOT also see it — otherwise typing 'f' fits the view mid-sentence.
        let resp = state.shell.state.on_window_event(&state.window, &event);
        if resp.consumed {
            match &event {   // egui never owns these two, even when focused:
                winit::event::WindowEvent::Resized(s) => state.resize(s.width, s.height),
                winit::event::WindowEvent::RedrawRequested => { let _ = state.render(); }
                _ => {}
            }
            return;
        }
```

This ordering is the whole input contract: **egui first, 3-D second**. Every later UI lesson (CLI,
tree, numeric entry) rides on it for free. (lib.rs's `App` is an `ApplicationHandler<Msg>` — the
`Ready`/`File` messages arrive through `user_event`, not here; 47 touches only `window_event`, so
the two never collide.)

One raw input must be tracked **above** that gate: the left button. egui can consume the release
(clicking a panel), and a stale "down" would stick 58's marquee on screen — and later start a
phantom gumball drag (69's gotcha #1). Add `pub lmb_down: bool` to `struct State` (init `false` in
`State::new`), and in lib.rs insert **above** the `let resp = state.shell.state.on_window_event(…)`
line:

```rust
        // raw button state — egui may consume the release, never let a stale press linger
        if let winit::event::WindowEvent::MouseInput {
            state: s, button: winit::event::MouseButton::Left, .. } = &event {
            state.lmb_down = *s == winit::event::ElementState::Pressed;
        }
```

## Step 3 — the egui pass: `src/engine/gpu/mod.rs`

egui hands back triangles; drawing them is four calls on the **same encoder** as the 3-D passes, in a
second render pass that loads (not clears) the color target. The split matters: *tessellation* needs
the egui `Context`, which lives in `Shell` — so the **caller tessellates**, and `Gpu` only ever sees
finished triangles. (`Gpu` knowing `egui_wgpu` is fine — it's a GPU library, not a document type;
35's litmus is about `Session`/`Mesh`.) `clear()` gains one parameter:

```rust
    pub fn clear(&mut self, color: wgpu::Color, view_proj: &Xform, origin: &Point,
                 ui: Option<crate::ui::UiFrame>) -> anyhow::Result<()> {
```

then, after the last 3-D pass's closing `}` — the block below ENDS with the submit, so **delete
the existing `self.queue.submit([encoder.finish()]);` line** (the `output.present();` after it
stays):

```rust
        // ---- egui pass (after the 3-D passes, same encoder) ----
        let mut extra_cmds = Vec::new();
        if let Some(f) = ui {
            let screen = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [self.config.width, self.config.height],
                pixels_per_point: f.pixels_per_point,
            };
            for (id, delta) in &f.textures_delta.set {
                f.renderer.update_texture(&self.device, &self.queue, *id, delta);
            }
            extra_cmds = f.renderer.update_buffers(&self.device, &self.queue,
                &mut encoder, &f.tris, &screen);
            {
                let epass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("egui"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        depth_slice: None,
                        // LOAD — draw over the scene
                        ops: wgpu::Operations { load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store },
                    })],
                    depth_stencil_attachment: None,                      // UI ignores scene depth
                    occlusion_query_set: None, timestamp_writes: None, multiview_mask: None,
                });
                f.renderer.render(&mut epass.forget_lifetime(), &f.tris, &screen);
            }
            for id in &f.textures_delta.free { f.renderer.free_texture(id); }
        }
        self.queue.submit(extra_cmds.into_iter().chain(std::iter::once(encoder.finish())));
```

with the carrier type in `src/ui/mod.rs`:

```rust
/// Everything Gpu needs to draw one frame of UI — tessellated by the caller (the Context stays
/// in Shell).
pub struct UiFrame<'a> {
    pub renderer: &'a mut egui_wgpu::Renderer,
    pub tris: Vec<egui::ClippedPrimitive>,
    pub textures_delta: egui::TexturesDelta,
    pub pixels_per_point: f32,
}
```

> **Pass-ordering contract.** The egui pass draws onto the *resolved* `view` with `LoadOp::Load`
> and no depth — so it must be the LAST thing on the encoder. Every 3-D pass precedes it, and any
> later overlay pass that resolves MSAA into `view` (65's gumball: color Load on `msaa_view`,
> resolve into `view`) must run *before* it too — a resolve after egui would blit the MSAA buffer
> right over the UI. Encoder order is the whole guarantee; there is no depth test to save you.

> **Perf note — don't format what hasn't changed.** The two HUD `format!`s (and the tessellation
> after them) run every frame even on a static scene — small, but pure churn, and `format!` allocs.
> The cheap gate: cache the two label strings in `UiState` and rebuild them only when the numbers
> they show actually change (`draws`/`drawn`/`total`/`frame_ms` quantized to 0.01 ms) — build_ui
> then does `ui.label(&ui_state.hud_line1)` and the allocs vanish from the steady state. 71 goes
> the whole way: nothing changed → no UI build, no tessellation, no frame at all.

## Step 4 — the frame ties together: `src/state.rs`

`render()` now: copy HUD numbers in → build the UI → **apply** the settings → tessellate → draw.
Find the final `self.gpu.clear(wgpu::Color { r: 0.9, … }, &view_proj)` expression at the
end of `render()` and replace it with:

```rust
        // 1. feed the HUD (the Step-1 accessors + 53's cull counters)
        self.ui.fps = self.gpu.performance.fps();
        self.ui.frame_ms = self.gpu.performance.ms();
        self.ui.draws = self.gpu.performance.last_draws;
        self.ui.drawn = self.gpu.perf_drawn;
        self.ui.total = self.gpu.perf_total;
        // …and 58's rubber band: live while the left button is down past the click threshold
        let (px, py) = self.drag_start;
        let drag = (self.cursor.0 - px).hypot(self.cursor.1 - py);
        self.ui.marquee = if self.lmb_down && drag >= 3.0 {
            Some(((px as f32, py as f32), (self.cursor.0 as f32, self.cursor.1 as f32)))
        } else { None };

        // 2. lay out; widgets mutate self.ui only
        let full_out = crate::ui::build_ui(&mut self.shell, &self.window, &mut self.ui);
        self.shell.state.handle_platform_output(&self.window, full_out.platform_output);

        // 3. APPLY intent — after the closure, where &mut self is free again
        self.gpu.show_grid = self.ui.show_grid;
        self.gpu.show_edges = self.ui.show_edges;
        self.gpu.thickness = self.ui.thickness;
        self.camera.set_ortho(self.ui.ortho);            // 16's projection toggle, now data-driven

        // 4. tessellate + hand to the gpu (the same clear call, one new argument)
        let tris = self.shell.ctx.tessellate(full_out.shapes, full_out.pixels_per_point);
        let ui_frame = crate::ui::UiFrame { renderer: &mut self.shell.renderer, tris,
            textures_delta: full_out.textures_delta, pixels_per_point: full_out.pixels_per_point };
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj, &origin,
            Some(ui_frame))
```

Camera only has `toggle_projection` (from 16) — add the data-driven setter next to it in
`src/camera.rs`; the field is `perspective`, the inverse of `ortho`:

```rust
    pub fn set_ortho(&mut self, on: bool) {   // add beside toggle_projection
        self.perspective = !on;
    }
```

(`show_grid`/`show_edges` are two new `pub bool` fields on `Gpu`, gated in `clear()`: wrap the
grid block — `pass.set_pipeline(&self.pipelines.grid)` through its `draws += 1;` — in
`if self.show_grid { … }`, and wrap the whole Edges block — 34f's
`if self.segment_count > 0 { … }` (both SOLID and FLAT branches) — in
`if self.show_edges { … }`. Add both to `struct Gpu` **and** initialize them in `Gpu::new`'s
`Self { … }` (`show_grid: true, show_edges: true`) — a struct literal, so a missing field is an
**E0063** build error. `thickness` drives the line uniform every frame after Step 1's enabler;
the slider just changes the number it uploads.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Two floating panels over the scene; the perf window shows live fps / ms / draws / drawn-vs-total —
  the 28 console counter, graduated.
- **Uncheck `grid`** → the grid vanishes *next frame*; re-check → back. Same for `edges` (the whole
  cylinder pass skips — draw count on the HUD drops by one). Toggle `orthographic` → same switch as
  the Space key. Drag the thickness slider → every line in the scene thickens live, no re-upload
  (that's 31's uniform doing its job).
- **Drag a slider across the viewport** → the camera does *not* orbit while you drag; type in a
  future text field → keys don't trigger shortcuts. That's Step 2's `consumed` gate — the one thing
  that must never regress.

## Recap

```
Ch 59: visibility — Phase 7 closed; the scene is fully interactive but mute.
Ch 60: EGUI OVERLAY. Shell { ctx, winit-state, wgpu-renderer } (archive wiring:
       Renderer::new(device, format, RendererOptions::default()), State::new(ctx,
       ViewportId::ROOT, window, …)). INPUT
       CONTRACT: every winit event → egui first; consumed → the 3-D layer never sees it (the ONE
       exception: lmb_down is tracked raw, above the gate — egui can eat a release).
       FRAME: State copies HUD numbers into UiState → build_ui (widgets bind to the plain
       struct — NEVER mutate State inside the closure) → apply intent after → ctx.tessellate →
       Gpu draws a second pass on the SAME encoder, LoadOp::Load, no depth (UiFrame carries
       renderer+tris+textures_delta; tessellation stays with the Context in Shell) — LAST on the
       encoder, after any MSAA-resolving overlay (65's gumball). First rent:
       perf HUD panel; grid/edges/ortho checkboxes; thickness slider straight into 31's uniform;
       and 58's rubber band, painted as one Foreground-layer rect while lmb_down && drag ≥ 3 px.
```

Edited: `ui/mod.rs` (NEW — `Shell`, `UiState`, `UiFrame`, `build_ui`, the marquee rect), `lib.rs`
(`mod ui;` + egui-first event routing + raw `lmb_down`), `state.rs` (shell + ui fields, the 4-step
frame, marquee feed), `engine/gpu/mod.rs` (`clear(…, ui)` egui pass, `show_grid`/`show_edges` gates).

## Next

`71-command-bus.md` — THE interface arrives: a command line docked at the screen edge, a registry of
verbs, and the **Get-loop** — the little state machine that lets a running command ask for a point or
an option and accept *either* a click *or* typed text, exactly like Rhino's prompt. First verbs:
`hide`, `show`, `zoom`.
