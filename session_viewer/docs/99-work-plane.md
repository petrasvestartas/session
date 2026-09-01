# 98 Work plane — "the ground" becomes a choice

> **Big picture.** *Phase 13.* Every tool so far draws on `z = 0` — fine for floor plans, useless for
> drawing on a wall. The **construction plane** (Rhino's CPlane) makes the drawing surface a piece of
> state: set it by three points or from a face, and everything that assumed the ground — the empty-
> click resolver, the grid, snapping, `rect`/`box` — follows automatically. The architecture did the
> work already: all of those go through **one function** (`cursor_world_point`, 53/63), so this
> lesson mostly replaces a hardcoded plane with a stored one.

<svg viewBox="0 0 680 130" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="the work plane is stored state; the click resolver, grid, snap, and draw tools all read it; set by three points or a picked face" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g transform="translate(30,16)">
    <path d="M 0,70 L 90,90 L 150,50 L 60,32 Z" fill="none" stroke="#6fb3ff" stroke-width="1.8"/>
    <path d="M 30,63 L 120,80 M 18,74 L 105,52 M 45,44 L 135,62" stroke="#6fb3ff" stroke-width="0.6" opacity="0.5"/>
    <text x="75" y="112" fill="#888" text-anchor="middle">a tilted CPlane, grid riding it</text>
  </g>
  <rect x="250" y="40" width="150" height="34" fill="none" stroke="#6fb3ff" stroke-width="1.4"/>
  <text x="325" y="61" fill="#d7dae0" text-anchor="middle">work_plane: Plane</text>
  <g stroke="#6fb3ff" stroke-width="1.1">
    <line x1="400" y1="47" x2="460" y2="26" marker-end="url(#ah75)"/><line x1="400" y1="57" x2="460" y2="57" marker-end="url(#ah75)"/><line x1="400" y1="67" x2="460" y2="88" marker-end="url(#ah75)"/>
  </g>
  <g fill="#d7dae0" font-size="10">
    <text x="466" y="30">empty-click resolver (61) → ray ∩ plane</text>
    <text x="466" y="61">grid shader (70) → plane frame</text>
    <text x="466" y="92">rect/box (71), grid snap (72) → plane uv</text>
  </g>
  <defs><marker id="ah75" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/state.rs          # work_plane: Plane (kernel type) — default world-XY; the resolver uses it
src/app/commands.rs   # `cplane` command: 3 points / `cplane face` / `cplane world` (Get-loop, 54)
src/shaders/ground.wgsl # plane frame uniform — ground + infinite grid render ON the plane
src/app/snap.rs       # grid snap in plane coordinates
```

## Step 1 — the state + the resolver: `src/state.rs`

The kernel's `Plane` (origin + x/y/z axes) is exactly a CPlane. Store one; default = world XY:

```rust
    pub work_plane: Plane,     // ← ADD; Plane::default() is world XY at the origin
```

A new field means a new initializer — add it to `State::new`'s struct literal (else the build
fails with "missing field `work_plane`"):

```rust
    work_plane: Plane::default(),   // ← ADD in State::new — world XY at the origin
```

`cursor_world_point` (61/72) swaps its hardcoded `z = 0` intersection for the plane:

```rust
        // was: t = -ray.origin[2] / ray.dir[2]      (the z=0 special case)
        let n = self.work_plane.z_axis();                                   // plane normal
        let denom = ray.dir.dot(&n);
        if denom.abs() > 1e-9 {
            let t = (self.work_plane.origin() - ray.origin.clone()).dot(&n) / denom;
            if t > 0.0 { return Some(ray.origin + &ray.dir * t); }
        }
```

Because every tool's empty-space clicks flow through this one function, `line`, `polyline`, `point`,
and the Get-loop are now plane-aware — **zero tool edits**, third time the single-resolver design
pays (53 → 64 → here).

One more `State` method now, so Step 2's commands compile against it — Step 3 grows it:

```rust
    /// The work plane changed: redraw (78). Defined NOW so the Step 2 verbs build; Step 3 extends
    /// it to push the new frame into the ground shader's uniform.
    pub fn on_cplane_changed(&mut self) { self.poke(); }
```

## Step 2 — the command: `src/app/commands.rs`

A three-way verb, all Get-loop (62's grammar). The dispatch arm (`commands.rs` needs
`use session_rust::Plane;`):

```rust
        "cplane" => match parts.next() {
            Some("world") => {
                state.work_plane = Plane::default();
                state.on_cplane_changed();
                Dispatch::Instant("cplane: world XY".into())
            }
            Some("face") => { let (cmd, get) = CplaneFace::start(); Dispatch::Start(cmd, get) }
            _            => { let (cmd, get) = Cplane3Points::start(); Dispatch::Start(cmd, get) }
        }
```

(`on_cplane_changed()` was **defined in Step 1** (poke-only) precisely so this dispatch compiles the
moment you add it; Step 3 extends the method to also push the plane frame to the ground shader. The
build stays green at the end of every step.)

The two commands, below `dispatch` in `commands.rs` — 62's ActiveCommand shape, nothing new:

```rust
pub struct Cplane3Points {
    pts: Vec<Point>,
}

impl Cplane3Points {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(Cplane3Points { pts: Vec::new() }),
         GetState::WaitingPoint { prompt: "cplane: pick ORIGIN".into() })
    }
    fn ask(&self) -> GetState {
        let what = match self.pts.len() {
            0 => "cplane: pick ORIGIN",
            1 => "cplane: pick a point on the X axis",
            _ => "cplane: pick a point on the +Y side",
        };
        GetState::WaitingPoint { prompt: what.into() }
    }
}

impl ActiveCommand for Cplane3Points {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep {
        self.pts.push(p);
        if self.pts.len() < 3 { return CmdStep::Prompt(self.ask()); }
        let (o, px, py) = (self.pts[0].clone(), self.pts[1].clone(), self.pts[2].clone());
        // x toward the second pick; z = x × (toward the third); y closes the frame
        let x = (px - o.clone()).normalized();
        let mut z = x.cross(&(py - o.clone()));
        if z.magnitude() < 1e-12 { return CmdStep::Done("points are collinear — unchanged".into()); }
        z.normalize_self();
        let y = z.cross(&x);
        state.work_plane = Plane::from_axes(o, x, y, z);
        state.on_cplane_changed();
        CmdStep::Done("cplane set".into())
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        let n: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if n.len() == 3 { return self.feed_point(state, Point::new(n[0], n[1], n[2])); }
        CmdStep::Prompt(self.ask())
    }
    fn back(&mut self) -> CmdStep { self.pts.pop(); CmdStep::Prompt(self.ask()) }
    fn prompt(&self) -> GetState { self.ask() }
}

pub struct CplaneFace;

impl CplaneFace {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        (Box::new(CplaneFace),
         GetState::WaitingPoint { prompt: "cplane: click a mesh face".into() })
    }
}

impl ActiveCommand for CplaneFace {
    fn feed_point(&mut self, state: &mut crate::state::State, _p: Point) -> CmdStep {
        // the fed point is the snapped click; the FACE comes from 56's sub-object resolve
        match state.face_plane_under_cursor() {
            Some(pl) => {
                state.work_plane = pl;
                state.on_cplane_changed();
                CmdStep::Done("cplane: face".into())
            }
            None => CmdStep::Prompt(self.prompt()),
        }
    }
    fn feed_text(&mut self, _state: &mut crate::state::State, _s: &str) -> CmdStep {
        CmdStep::Prompt(self.prompt())
    }
    fn back(&mut self) -> CmdStep { CmdStep::Cancel }
    fn prompt(&self) -> GetState {
        GetState::WaitingPoint { prompt: "cplane: click a mesh face".into() }
    }
}
```

`face_plane_under_cursor` composes machinery from 47/48 — add to `impl State` (`state.rs`):

```rust
    /// The plane of the mesh face under the cursor (56's resolve → kernel face normal).
    pub fn face_plane_under_cursor(&mut self) -> Option<Plane> {
        let ray = self.cursor_ray()?;
        let vp = self.camera.view_proj(self.aspect());
        let origin = self.camera.origin();
        let viewport = (0.0, 0.0, self.gpu.config.width as f64, self.gpu.config.height as f64);
        let hit = self.scene.pick_mesh(&ray)?;
        let sub = self.scene.resolve_subobject(&hit.guid, &hit, self.cursor,
                                               &vp, &origin, viewport)?;
        let crate::app::pick::SubKind::Face(fk) = sub.kind else { return None };
        // multi-doc: resolve the OWNING doc through the row map — `scene.session` (singular)
        // doesn't exist since 35, and the wrong doc's lookup returns None or the WRONG mesh
        let Some(&row) = self.scene.guid_to_row.get(&hit.guid) else { return None };
        let d = self.scene.doc_of_row(row);
        let Some(Geometry::Mesh(m)) = self.scene.docs[d].session.lookup.get(&hit.guid)
            else { return None };
        let n = m.xform.transform_vector(&m.face_normal(fk)?);     // local normal → world
        Some(Plane::from_point_normal(hit.point.clone(), n))
    }
```

(`state.rs` gains `Plane` in its `session_rust` use. Register the verb: `"cplane"` in `VERBS`,
alias `("cp","cplane")`.)

**`rect`/`box` (71) build in plane uv.** Corner clicks convert to plane coordinates, the rectangle
spans u/v, the result converts back — through one helper pair on `State`, so the tools still never
touch `work_plane` directly:

```rust
    /// World ↔ work-plane coordinates. Tools stay plane-blind: they see (u, v, w) triples.
    pub fn to_plane(&self, p: &Point) -> [f64; 3] {
        let d = p.clone() - self.work_plane.origin();
        [d.dot(&self.work_plane.x_axis()), d.dot(&self.work_plane.y_axis()),
         d.dot(&self.work_plane.z_axis())]
    }
    pub fn from_plane(&self, u: f64, v: f64, w: f64) -> Point {
        let o = self.work_plane.origin();
        let (x, y, z) = (self.work_plane.x_axis(), self.work_plane.y_axis(),
                         self.work_plane.z_axis());
        Point::new(o[0] + x[0]*u + y[0]*v + z[0]*w,
                   o[1] + x[1]*u + y[1]*v + z[1]*w,
                   o[2] + x[2]*u + y[2]*v + z[2]*w)
    }
```

In `rect.rs` (71), find `rect_corners` → replace it and its callers' first lines:

```rust
/// Two picked world corners → the 4 rectangle corners ON the work plane (u/v span, w = 0).
fn rect_corners(state: &crate::state::State, a: &Point, b: &Point) -> [Point; 4] {
    let (pa, pb) = (state.to_plane(a), state.to_plane(b));
    let (u0, u1) = (pa[0].min(pb[0]), pa[0].max(pb[0]));
    let (v0, v1) = (pa[1].min(pb[1]), pa[1].max(pb[1]));
    [state.from_plane(u0, v0, 0.0), state.from_plane(u1, v0, 0.0),
     state.from_plane(u1, v1, 0.0), state.from_plane(u0, v1, 0.0)]
}
```

(`rect_ghost` gains the same `state` parameter and passes it through; the two `rect_corners(&a, &p)`
call sites become `rect_corners(state, &a, &p)`.) `BoxTool::finish` builds its mesh in plane space
and places it with one kernel transform — find its `let (x0, x1) = …` through `m.xform = …` lines →
replace with:

```rust
        let (pa, pb) = (state.to_plane(a), state.to_plane(b));
        let (u0, u1) = (pa[0].min(pb[0]), pa[0].max(pb[0]));
        let (v0, v1) = (pa[1].min(pb[1]), pa[1].max(pb[1]));
        let mut m = Mesh::create_box(u1 - u0, v1 - v0, h);
        // center the box in plane space, then map plane space → the tilted work plane
        let center = Xform::translation((u0 + u1) * 0.5, (v0 + v1) * 0.5, h * 0.5);
        m.xform = Xform::plane_to_plane(&Plane::default(), &state.work_plane) * &center;
```

(`rect.rs` gains `Plane` in its imports. `BoxTool`'s clicked-height arm reads `p[2].abs()` — make
that plane-aware too: `state.to_plane(&p)[2].abs()`.)

## Step 3 — the visible plane: grid + snap

**Ground/grid (70).** The analytic shader gains the plane frame; ray∩plane replaces ray∩z=0 (same
math as Step 1, GPU-side), and the fade measures in plane uv. World XY renders pixel-identical to
before. In `ground.wgsl`, find `GroundUniform` → replace `ground_z` + pads with the frame (the Rust
mirror grows by the same three `[f32; 4]`s — 144 B both sides, no vec3 traps):

```wgsl
struct GroundUniform {
    inv_view_proj: mat4x4<f32>,
    cam_rel_eye: vec4<f32>,       // eye − origin (xyz), fade radius (w)
    plane_o: vec4<f32>,           // work-plane origin, CAMERA-RELATIVE (origin-subtracted)
    plane_x: vec4<f32>,           // unit u axis (xyz)
    plane_y: vec4<f32>,           // unit v axis (xyz)
};
```

and in `fs_main`, find the block from `let denom = dir.z;` through `let d = length(hit.xy -
g.cam_rel_eye.xy);` → replace with:

```wgsl
    let n = cross(g.plane_x.xyz, g.plane_y.xyz);
    let denom = dot(dir, n);
    if (abs(denom) < 1e-7) { discard; }
    let t = dot(g.plane_o.xyz - p0, n) / denom;
    if (t <= 0.0) { discard; }
    let hit = p0 + dir * t;
    // horizon fade, measured in plane uv (world-XY before; identical there)
    let rel = hit - g.plane_o.xyz;
    let uv = vec2<f32>(dot(rel, g.plane_x.xyz), dot(rel, g.plane_y.xyz));
    let eye_rel = g.cam_rel_eye.xyz - g.plane_o.xyz;
    let euv = vec2<f32>(dot(eye_rel, g.plane_x.xyz), dot(eye_rel, g.plane_y.xyz));
    let d = length(uv - euv);
```

On the Rust side, `Gpu` stores the frame — `pub work_plane: (Point, Vector, Vector)` (origin, x, y;
init `(Point::new(0.0, 0.0, 0.0), Vector::x_axis(), Vector::y_axis())` in `Gpu::new`) — and 70's
per-frame `GroundUniform` fill writes `plane_o = work_plane.0 − origin` (the camera-relative
subtract, f64 then cast) plus the two axes in place of the old `ground_z`. If you took 70's
fract/fwidth infinite-grid upgrade, its lines already key off the hit — express them in `uv` and
they ride the plane for free.

**Snap (72).** The `Grid` candidate quantizes in plane uv instead of world xy; `Endpoint`/`Vertex`
snaps are world-space and unaffected. `snap()` gains a `plane: &Plane` parameter (the call site in
`cursor_world_point` passes `&self.work_plane`; `snap.rs` imports `Plane`). Find 72's grid-candidate
block (`if raw[2].abs() < 1e-6 { … }`) → replace with:

```rust
    // grid candidate — the crossing nearest the raw point, in PLANE uv (87)
    let d = raw.clone() - plane.origin();
    let (u, v, w) = (d.dot(&plane.x_axis()), d.dot(&plane.y_axis()), d.dot(&plane.z_axis()));
    if w.abs() < 1e-6 {
        let (gu, gv) = ((u / GRID_STEP).round() * GRID_STEP, (v / GRID_STEP).round() * GRID_STEP);
        let (o, x, y) = (plane.origin(), plane.x_axis(), plane.y_axis());
        let g = Point::new(o[0] + x[0]*gu + y[0]*gv,
                           o[1] + x[1]*gu + y[1]*gv,
                           o[2] + x[2]*gu + y[2]*gv);
        consider(g, SnapKind::Grid);
    }
```

**The change broadcaster** — grow the Step 1 stub into the real thing (`impl State`):

```rust
    /// The work plane changed: push the frame to the ground shader + redraw (78).
    pub fn on_cplane_changed(&mut self) {
        self.gpu.work_plane = (self.work_plane.origin(),
                               self.work_plane.x_axis(), self.work_plane.y_axis());
        self.poke();
    }
```

> ⚠ The work plane is **viewer state, not document state** — it lives on `State`, so a reload,
> reconcile (46), or a fresh session silently resets it to world XY. That's Rhino's session
> behavior too, but say it in the HUD when a non-default plane is active (`cplane: custom`) so a
> reload doesn't move anyone's drawing surface without a word. Persisting it (a manifest setting
> per doc) is a small, real follow-up: serialize origin + axes next to 50's save path and restore
> in `set_scene`.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- `cplane` → click three points on a box's tilted face (endpoint snap makes this exact, 64) → the
  grid **lies on that face**. Draw a `rect` → the rectangle lands **in the face's plane**, not on the
  floor — the roadmap's acceptance line, verbatim.
- `box` on the tilted plane → extrudes along the plane normal. Grid snap → clicks land on the tilted
  grid's crossings.
- `cplane world` → everything back to the floor; the scene renders exactly as before the lesson
  (default-plane regression check).
- The audit that matters: the empty-click tools (`line`/`polyline`/`point`) didn't change — they
  inherit the plane through `cursor_world_point`. `rect`/`box` read the plane only via `State`'s
  `to_plane`/`from_plane` helpers, never the field directly, so `grep -l "work_plane" src/app/tools/`
  → still empty.

## Recap

```
Ch 79: edit points — modeling on the curve.
Ch 80: CPLANE. work_plane: Plane (kernel type: origin + axes IS a construction plane), default world
       XY. The ONE empty-click resolver (71/72's cursor_world_point) swaps z=0 for ray∩plane — every
       tool becomes plane-aware with zero tool edits (the single-resolver design's third payoff).
       `cplane` verb: 3 points (Get-loop) / face (pick → face plane) / world (reset). Grid + ground
       render on the plane (frame uniform into 70's analytic shader); grid snap quantizes in plane
       uv; rect/box build in uv and extrude along z_axis. Rectangle-on-a-tilted-face: the acceptance
       test, passing.
```

Edited: `state.rs` (`work_plane`, resolver, `on_cplane_changed`, uv helpers), `app/commands.rs`
(`cplane`), `shaders/ground.wgsl` (plane frame), `app/snap.rs` (uv grid snap).

## Next

`99-advanced-perf.md` — headroom for scenes beyond the stress file: LOD/decimation, occlusion
culling, and the GPU-compute cull + indirect draw that lesson 27's WebGPU-only decision unlocked.
