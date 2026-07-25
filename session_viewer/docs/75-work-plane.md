# 75 Work plane — "the ground" becomes a choice

> **Big picture.** *Phase 13.* Every tool so far draws on `z = 0` — fine for floor plans, useless for
> drawing on a wall. The **construction plane** (Rhino's CPlane) makes the drawing surface a piece of
> state: set it by three points or from a face, and everything that assumed the ground — the empty-
> click resolver, the grid, snapping, `rect`/`box` — follows automatically. The architecture did the
> work already: all of those go through **one function** (`cursor_world_point`, 48/59), so this
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
    <text x="466" y="30">empty-click resolver (48) → ray ∩ plane</text>
    <text x="466" y="61">grid shader (65) → plane frame</text>
    <text x="466" y="92">rect/box (58), grid snap (59) → plane uv</text>
  </g>
  <defs><marker id="ah75" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/state.rs          # work_plane: Plane (kernel type) — default world-XY; the resolver uses it
src/app/commands.rs   # `cplane` command: 3 points / `cplane face` / `cplane world` (Get-loop, 49)
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

`cursor_world_point` (48/59) swaps its hardcoded `z = 0` intersection for the plane:

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
pays (48 → 59 → here).

## Step 2 — the command: `src/app/commands.rs`

A three-way verb, all Get-loop (49's grammar):

```rust
    "cplane" => match parts.next() {
        Some("world") => { state.work_plane = Plane::default(); state.on_cplane_changed();
                           Dispatch::Instant("cplane: world XY".into()) }
        Some("face") => { /* WaitingPoint; on click: pick_ray → hit mesh face (43) → Plane from the
                            face's plane (kernel face normal + hit point as origin) */ }
        _ => { /* 3-point ActiveCommand: origin → x-direction point → y-side point;
                 Plane::new_origin_x_y(…) — check your kernel's from-points constructor.
                 back/Esc from 49 apply as always. */ }
    }
```

(`on_cplane_changed()` is **defined in Step 3** — it rebuilds the grid uniform and calls `poke()`.
It's called here before you write it; add the verb now, add the method in Step 3, and the build is
green only once both exist.)

(`rect`/`box` (58) build their geometry in plane **uv**: corner clicks convert to plane coordinates
(`plane.x_axis()`/`y_axis()` dot products), the rectangle spans u/v, and the result transforms back —
a `to_plane`/`from_plane` helper pair on `State`. `box` extrudes along `z_axis()`.)

## Step 3 — the visible plane: grid + snap

- **Grid/ground (65):** the analytic shader gains the plane frame (origin + axes as a uniform);
  ray∩plane replaces ray∩z=0 (same math as Step 1, GPU-side), and grid lines run along plane u/v —
  the fade and depth logic don't change. World XY renders pixel-identical to before.
- **Snap (59):** the `Grid` candidate quantizes in plane uv instead of world xy — express the raw
  point in plane coordinates, round u and v to the step, convert back. `Endpoint`/`Vertex` snaps are
  world-space and unaffected.
- `on_cplane_changed()` = update the grid uniform + `poke()` (66).

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- `cplane` → click three points on a box's tilted face (endpoint snap makes this exact, 59) → the
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
Ch 74: edit points — modeling on the curve.
Ch 75: CPLANE. work_plane: Plane (kernel type: origin + axes IS a construction plane), default world
       XY. The ONE empty-click resolver (48/59's cursor_world_point) swaps z=0 for ray∩plane — every
       tool becomes plane-aware with zero tool edits (the single-resolver design's third payoff).
       `cplane` verb: 3 points (Get-loop) / face (pick → face plane) / world (reset). Grid + ground
       render on the plane (frame uniform into 65's analytic shader); grid snap quantizes in plane
       uv; rect/box build in uv and extrude along z_axis. Rectangle-on-a-tilted-face: the acceptance
       test, passing.
```

Edited: `state.rs` (`work_plane`, resolver, `on_cplane_changed`, uv helpers), `app/commands.rs`
(`cplane`), `shaders/ground.wgsl` (plane frame), `app/snap.rs` (uv grid snap).

## Next

`76-advanced-perf.md` — headroom for scenes beyond the stress file: LOD/decimation, occlusion
culling, and the GPU-compute cull + indirect draw that lesson 27's WebGPU-only decision unlocked.
