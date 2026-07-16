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
src/shaders/*.wgsl      # mesh/cylinder/sphere/point/ground fs: the discard line
src/app/scene.rs        # pick filter: reject hits behind active sections
src/app/commands.rs     # `section` verb: 3 points / wp / off / flip / drag
src/state.rs            # sections: Vec<Plane> (kernel type) — viewer state, like the camera
```

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
    /// Called in clear(), beside the frustum build. `sections` are WORLD planes from State.
    fn upload_sections(&mut self, sections: &[Plane], origin: &Point) {
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

(The buffer + bind group are 07's uniform pattern; bind it in every geometry pipeline at the next
free group slot, or — cheaper — fold the four vec4s + count into the existing line uniform's buffer
as a second struct member, since every geometry shader already binds group 1.)

## Step 2 — one line per shader: `src/shaders/*.wgsl`

Each geometry fs (mesh, cylinder, sphere, point billboards, ground) gains the loop — first thing in
`fs_main`, before any lighting. The shaders that don't yet pass `world_pos` to the fragment stage
(cylinder/sphere) add it to `VsOut` — the vs already computes it:

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
            Some("flip") => { for pl in &mut state.sections { pl.flip(); }   // or rebuild with -n
                              state.poke(); Dispatch::Instant("sections flipped".into()) }
            Some("wp")   => { state.sections.push(state.work_plane.clone()); state.poke();
                              Dispatch::Instant("section from work plane".into()) }
            Some("drag") => { let (cmd, get) = SectionDrag::start(&state.sections);
                              Dispatch::Start(cmd, get) }
            _ => { let (cmd, get) = SectionBy3Points::start();   // 75's 3-point flow, verbatim
                   Dispatch::Start(cmd, get) }
        }
```

The drag is 54's skeleton pointed at a plane instead of an object: `SectionDrag`'s `on_move` runs
`closest_param_on_axis(ray, plane.origin, plane.normal)` and translates the plane's origin along its
normal by `t − t0` — live, `poke()` per move; Enter/click commits, Esc restores the stashed plane.
No Command/undo: sections are **viewer state like the camera**, not document state — they don't
save, don't hash, don't undo (Rhino draws the same line).

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
