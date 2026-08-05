# 78 Section planes — cut the building open

> **Big picture.** *Phase 14 — CAD completeness.* The capstone proved the loop; this phase adds what
> a plan review found missing — ranked, and this is #1. For an AEC-adjacent kernel (floors, beams,
> plates), **sectioning is table stakes**: no one evaluates a building model without cutting it.
> The implementation is almost embarrassingly cheap on this architecture: N world planes in one
> uniform, **one `discard` line** in each fragment shader, and the command/drag machinery from
> Phases 8–9 reused verbatim. Picking must respect the cut, or you select walls you can't see.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a section plane cuts the model; fragments on the negative side are discarded per pixel; the plane is set by three points or the work plane and dragged along its normal" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(30,20)">
    <rect x="0" y="30" width="200" height="80" fill="none" stroke="#6fb3ff" stroke-width="1.6"/>
    <line x1="60" y1="30" x2="60" y2="110" stroke="#3a3a3a"/>
    <line x1="130" y1="30" x2="130" y2="110" stroke="#3a3a3a"/>
    <line x1="95" y1="10" x2="95" y2="130" stroke="#e0b040" stroke-width="2"/>
    <path d="M 95,20 h 20" stroke="#e0b040" stroke-width="1.4" marker-end="url(#ah78)"/>
    <text x="95" y="146" fill="#e0b040" text-anchor="middle">section plane · drag along normal</text>
    <g opacity="0.25" fill="#6fb3ff">
      <rect x="95" y="31" width="105" height="78"/>
    </g>
    <text x="150" y="75" fill="#888" text-anchor="middle" font-size="10">discarded side</text>
  </g>
  <g transform="translate(330,24)">
    <text x="0" y="14" fill="#d7dae0">fs, every geometry shader:</text>
    <text x="0" y="34" fill="#888" font-size="10">if dot(p, n) + d &lt; 0 → discard</text>
    <text x="0" y="58" fill="#d7dae0">set: 3 points · work plane (75)</text>
    <text x="0" y="78" fill="#d7dae0">move: drag = 54's axis math</text>
    <text x="0" y="98" fill="#d7dae0">pick: hits behind the cut rejected</text>
    <text x="0" y="122" fill="#666" font-size="10">count = 0 → zero cost; planes rebased like 37's frustum</text>
  </g>
  <defs><marker id="ah78" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
    <path d="M0,0 L6,3 L0,6 Z" fill="#e0b040"/></marker></defs>
</svg>

## Files we touch

```
src/engine/gpu/mod.rs   # SectionUniform (4 planes max) + per-frame rebase + upload
src/shaders/*.wgsl      # triangle/cylinder/sphere/point/ground fs: the discard line
src/app/scene.rs        # pick filter: reject hits behind active sections
src/app/commands.rs     # `section` verb: 3 points / wp / off / flip / drag
src/state.rs            # sections: Vec<Plane> (kernel type) — viewer state, like the camera
```

> **New `State` field first.** `sections: Vec<Plane>` is viewer state like the camera — add it to
> `struct State` **and** initialize `sections: Vec::new()` in `State::new` (a struct literal, so a missing
> field is an **E0063** build error — same as 75's `work_plane`). Step 4's `state.sections…` code needs it.

## Step 1 — the uniform, camera-relative like everything else: `src/engine/gpu/mod.rs`

A plane is `(normal, d)` with the keep-side test `n·p + d ≥ 0`. The shaders' `world_pos` is
**camera-relative** (33), so the planes rebase per frame exactly like 37's frustum did — same move,
opposite direction:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct SectionUniform {
    planes: [[f32; 4]; 4],   // (n.xyz, d_rebased) — keep where dot(n, p_rel) + d >= 0
    count: u32,
    _pad: [u32; 3],
}
```

```rust
    /// `sections` are WORLD planes from State. Rebase against the same ANCHOR the instance rows
    /// use (34c) — in State::render(), right after `let anchor = self.gpu.rebase_anchor(&origin);`,
    /// call `self.gpu.upload_sections(&self.sections, &anchor);`.
    pub fn upload_sections(&mut self, sections: &[Plane], origin: &Point) {
        let mut u = SectionUniform { planes: [[0.0; 4]; 4], count: sections.len().min(4) as u32,
                                     _pad: [0; 3] };
        for (i, pl) in sections.iter().take(4).enumerate() {
            let n = pl.z_axis();                            // the plane's normal (kernel Plane)
            let d = -n.dot(&(pl.origin() - Point::new(0.0, 0.0, 0.0)));   // world d: n·p + d = 0
            // rebase to camera-relative: n·(p−o) + (d + n·o) == n·p + d  (37's identity, reused)
            let d_rel = d + n.dot(&(origin.clone() - Point::new(0.0, 0.0, 0.0)));
            u.planes[i] = [n[0] as f32, n[1] as f32, n[2] as f32, d_rel as f32];
        }
        self.queue.write_buffer(&self.section_buffer, 0, bytemuck::bytes_of(&u));
    }
```

The buffer is 07's uniform pattern; `SectionUniform` is its **own** uniform with its own buffer —
don't fold the data into the line uniform, sectioning has nothing to do with line width. But
*where* it binds matters: cylinder/sphere/point already use groups 0–3, and WebGPU's default
`max_bind_groups` is **4** — there is no group 4 to claim. The one group every pipeline shares is
**group 0 (mvp)** — so the section uniform becomes `@group(0) @binding(1)`, piggybacking on the one
layout all pipelines already bind. In `Gpu::new`, find the `mvp_layout` creation → add a second
entry to its `entries` array:

```rust
            wgpu::BindGroupLayoutEntry {
                binding: 1,                                    // sections (78) — every shader sees it
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
```

create the buffer just above the `mvp_bind_group` (zeroed = count 0 = sections off):

```rust
        let section_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sections"),
            size: std::mem::size_of::<SectionUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
```

and add `wgpu::BindGroupEntry { binding: 1, resource: section_buffer.as_entire_binding() }` to the
`mvp_bind_group`'s `entries`. (Store `section_buffer` on `Gpu` — `upload_sections` writes it. No
pipeline gains a layout slot, no pass binds anything new: group 0 was already bound everywhere.)

## Step 2 — one line per shader: `src/shaders/*.wgsl`

Each geometry fs (`triangle.wgsl` = meshes; `cylinder.wgsl`; `sphere.wgsl` = spheres; `point.wgsl`
= cloud billboards; `ground.wgsl` = 65's analytic ground, which computes its own `hit`) gains the
loop — first thing in `fs_main`, before any lighting. The background gradient stays uncut — it's
sky. `triangle.wgsl` already carries `world_pos` in `VsOut` (`@location(1)`); the shaders that
don't yet pass it to the fragment stage (cylinder/sphere/point) add it to `VsOut` — the vs already
computes the world position.

First declare the uniform once per shader — same slot everywhere, because Step 1 put it in the
shared mvp group. Add at the top of each file, right below its `@group(0) @binding(0) … mvp` line:

```wgsl
struct SectionUniform {
    planes: array<vec4<f32>, 4>,   // (n.xyz, d_rebased)
    count: u32,
};
@group(0) @binding(1) var<uniform> sec: SectionUniform;
```

Then, as the first statement in each `fs_main`:

```wgsl
    for (var i = 0u; i < sec.count; i = i + 1u) {
        if (dot(in.world_pos, sec.planes[i].xyz) + sec.planes[i].w < 0.0) {
            discard;
        }
    }
```

`count = 0` → the loop body never runs — sections cost nothing until used. The cut face is hollow
(you see the object's inside back-faces); a darker tint on `!front_facing` fragments (21's builtin)
is the classic cheap "cut material" cue — one `if`, worth it. True solid caps are kernel
plane-splits — a later boolean lesson, not a shader trick.

## Step 3 — picking respects the cut: `src/app/scene.rs`

Without this you click *through* the opened wall and select what the eye can't see — the viewport
and the selection disagree, Phase 7's cardinal sin. One world-space filter, applied to both pickers:

```rust
    /// True if a WORLD point survives every active section (the same test the shaders run).
    fn visible_through_sections(&self, p: &Point, sections: &[Plane]) -> bool {
        for pl in sections {
            let n = pl.z_axis();
            let d = -n.dot(&(pl.origin() - Point::new(0.0, 0.0, 0.0)));
            if n.dot(&(p.clone() - Point::new(0.0, 0.0, 0.0))) + d < 0.0 {
                return false;
            }
        }
        true
    }
    // pick_mesh: after a candidate hit → `if !visible_through_sections(&point, sections) { continue; }`
    // pick_thin: filter the RayHit list the same way before choosing the nearest.
```

(A mesh whose *nearest* intersection is cut away but whose farther intersection is visible will read
as unpickable at that pixel — acceptable v1; the exact fix is walking all ray hits, noted not built.)

## Step 4 — the command + the drag: `src/app/commands.rs` + `src/state.rs`

`section` composes machinery you already have — 49's multi-step points, 75's work plane, 54's axis
drag:

```rust
        "section" => match parts.next() {
            Some("off")  => { state.sections.clear(); state.poke();
                              Dispatch::Instant("sections off".into()) }
            Some("flip") => { for pl in &mut state.sections {          // kernel Plane has no flip():
                                  *pl = Plane::from_point_normal(pl.origin(), -pl.z_axis());
                              }
                              state.poke(); Dispatch::Instant("sections flipped".into()) }
            Some("wp")   => { state.sections.push(state.work_plane.clone()); state.poke();
                              Dispatch::Instant("section from work plane".into()) }
            Some("drag") => { let (cmd, get) = SectionDrag::start(&state.sections);
                              Dispatch::Start(cmd, get) }
            _ => { let (cmd, get) = SectionBy3Points::start();   // 75's 3-point flow, verbatim
                   Dispatch::Start(cmd, get) }
        }
```

`SectionBy3Points` is 75's `Cplane3Points` **verbatim** except its last two lines — copy that
struct+impl under a new name and replace the two `state.work_plane = …; state.on_cplane_changed();`
lines of `feed_point` with:

```rust
        state.sections.push(Plane::from_axes(o, x, y, z));
        state.poke();
        return CmdStep::Done("section set".into());
```

The drag is 54's skeleton pointed at a plane instead of an object — live `poke()` per move,
Enter/Esc ends it (no Command/undo: sections are **viewer state like the camera**, not document
state — they don't save, don't hash, don't undo; Rhino draws the same line):

```rust
pub struct SectionDrag {
    t0: Option<f64>,             // axis param at the first on_move (lazy press reference)
}

impl SectionDrag {
    pub fn start(sections: &[Plane]) -> (Box<dyn ActiveCommand>, GetState) {
        let prompt = if sections.is_empty() { "section drag: no section set".into() }
                     else { "section drag: move the mouse, Enter to accept".into() };
        (Box::new(SectionDrag { t0: None }), GetState::WaitingPoint { prompt })
    }
}

impl ActiveCommand for SectionDrag {
    fn on_move(&mut self, state: &mut crate::state::State, _p: Point) {
        let Some(pl) = state.sections.last() else { return };
        let (o, n) = (pl.origin(), pl.z_axis());
        let Some(ray) = state.cursor_ray() else { return };
        let Some(t) = crate::engine::gumball::closest_param_on_axis(&ray, &o, &n) else { return };
        let t0 = *self.t0.get_or_insert(t);
        let dt = t - t0;
        if dt.abs() > 0.0 {
            let moved = Plane::from_point_normal(o + n.clone() * dt, n);
            *state.sections.last_mut().unwrap() = moved;
            self.t0 = Some(t);                       // incremental: origin moved, re-anchor
            state.poke();
        }
    }
    fn feed_point(&mut self, _state: &mut crate::state::State, _p: Point) -> CmdStep {
        CmdStep::Done("section placed".into())       // a click accepts, like Enter
    }
    fn feed_text(&mut self, _state: &mut crate::state::State, _s: &str) -> CmdStep {
        CmdStep::Done("section placed".into())       // Enter on the empty CLI accepts
    }
    fn back(&mut self) -> CmdStep { CmdStep::Cancel }
    fn prompt(&self) -> GetState {
        GetState::WaitingPoint { prompt: "section drag: move the mouse, Enter to accept".into() }
    }
}
```

(Both commands live in `commands.rs` beside 75's; `state.cursor_ray()` goes `pub(crate)` so they
can call it — one-word edit in `state.rs` (54). `Plane::from_point_normal(point, normal)` and
`o + n * dt` (`Point + Vector`) are verified kernel ops.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Load the floor model. `section`, click 3 points on a wall face (endpoint snaps make it exact) →
  everything on the negative side vanishes, live. `section flip` → the *other* half vanishes.
- `section drag`, move the mouse → the cut **sweeps through the building** interactively — the whole
  feature's wow moment, and it's one uniform write per frame.
- Click inside the opened building → you select the visible beam, never the cut-away wall in front
  of it (Step 3). Orbit — the cut is view-independent (world planes, correctly rebased: no swimming
  as the camera pans; if the cut crawls during pan, the `d_rel` rebase is missing).
- `section off` → whole model back; perf HUD identical to before (the count-0 early-out).

## Recap

```
Ch 77: the capstone loop.
Ch 78: SECTIONS. Up to 4 world planes in one uniform, rebased camera-relative per frame (d_rel =
       d + n·origin — 37's identity reused); every geometry fs runs one keep-test loop and discards
       the negative side (count 0 = free; !front_facing tint = cheap cut cue; true caps = kernel
       splits, later). Picking filters hits through the same world test — viewport and selection
       never disagree. Command: `section` = 3 points (49), `wp` (75), off/flip; `section drag` =
       54's closest_param_on_axis aimed at the plane's own normal, live per move. Sections are
       VIEWER state like the camera: no save, no hash, no undo. Total new machinery: one uniform,
       one loop, one filter — everything else was already on the shelf.
```

Edited: `engine/gpu/mod.rs` (`SectionUniform`, `upload_sections`), `shaders/*.wgsl` (keep-test loop,
`world_pos` through cylinder/sphere `VsOut`, back-face tint), `app/scene.rs`
(`visible_through_sections` + two picker filters), `app/commands.rs` (`section`), `state.rs`
(`sections: Vec<Plane>`, `SectionDrag`).

## Next

`79-import-export.md` — the kernel's OBJ codec (and one honest gap about STEP) reaches the browser:
drag a file in, `export obj` out.
