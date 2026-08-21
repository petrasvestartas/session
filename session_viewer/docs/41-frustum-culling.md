# 41 Frustum culling — draw only what's on screen

> **Big picture.** *Phase 5.* The stress file pushes 51k segments through the GPU even when you're
> zoomed into a corner where fifty are visible. Culling is the classic renderer fix: decide per
> object whether it can possibly be on screen, and make the GPU skip the rest — changing neither
> *what* is drawn (still one call) nor *how it looks*, only how much work each frame costs.

Zoomed into one corner of the stress file, the GPU is still processing all 51,166 cylinder segments
every frame — most of them behind you or off the sides. This lesson keeps the ones the camera can't
see out of the rasterizer: extract the view frustum's six planes, test each object's world box against
them, and flag the off-screen ones **culled** (vertex work still runs — Step 3 is honest about what
that means). The perf HUD's `drawn / total` split — flat since 34 — finally moves.

The scan is linear (every object's flag must be decided each frame), and that's exactly what the
archive ships — cheap at this scale. 40's BVH stays in reserve for **picking** (47) and
**box-select** (50), where a per-object test genuinely can't run N times. What makes the cull cheap
*here* isn't a fancier data structure — it's only re-uploading the rows whose visibility actually
flipped since last frame.

## The frame mismatch you have to fix

Lesson 33 made `view_proj` **camera-relative** — it maps `world − origin` to clip, so nothing far from
the world origin jitters. But 40's BVH boxes are **absolute world**. Frustum planes pulled straight
from that `view_proj` therefore live in camera-relative space and can't be tested against world boxes
as-is — every result would be off by `origin`.

Two ways to reconcile; one is one line. A plane `[a,b,c,d]` extracted from the camera-relative matrix
tests `a·x' + b·y' + c·z' + d ≥ 0` on a *relative* point `x' = x − origin`. Substitute and it's a
**world** plane with the same normal and a shifted `d`:

```
a(x−oₓ) + b(y−oy) + c(z−o_z) + d  =  a·x + b·y + c·z + (d − (a·oₓ + b·oy + c·o_z))
```

So rebasing the whole frustum to world is: for each plane, `d -= dot(normal, origin)`. Six subtractions,
done once per frame in f64 — then planes and boxes share the world frame and the test is exact.

One refinement since 34c: the matrix the frame *actually* draws with is the **anchored** one —
`view_proj_anchored(aspect, &anchor)`, with the anchor `rebase_anchor` picks (near the origin, but
sticky between frames). Same math, one substitution: the rebase point is that `anchor`, not the raw
camera `origin`. Extract the planes from the anchored matrix and subtract `dot(normal, anchor)`.

<svg viewBox="0 0 680 180" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="per frame: extract 6 planes from the f64 camera-relative view_proj, rebase them to world, plane-test every object's world AABB, set FLAG_CULLED on flipped rows; the shader collapses culled instances so one draw call remains" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <g fill="none" stroke="#6fb3ff" stroke-width="1.3">
    <rect x="10" y="30" width="140" height="40"/>
    <rect x="184" y="30" width="140" height="40"/>
    <rect x="358" y="30" width="150" height="40"/>
    <rect x="542" y="30" width="128" height="40"/>
  </g>
  <g fill="#d7dae0" text-anchor="middle">
    <text x="80" y="46">view_proj (f64)</text><text x="80" y="61" fill="#666" font-size="9">camera-relative</text>
    <text x="254" y="46">6 planes → world</text><text x="254" y="61" fill="#666" font-size="9">d −= n·anchor</text>
    <text x="433" y="46">plane test</text><text x="433" y="61" fill="#666" font-size="9">every world AABB</text>
    <text x="606" y="46">FLAG_CULLED</text><text x="606" y="61" fill="#666" font-size="9">flipped rows only</text>
  </g>
  <g stroke="#6fb3ff" stroke-width="1.4">
    <line x1="150" y1="50" x2="182" y2="50" marker-end="url(#ah37)"/>
    <line x1="324" y1="50" x2="356" y2="50" marker-end="url(#ah37)"/>
    <line x1="508" y1="50" x2="540" y2="50" marker-end="url(#ah37)"/>
  </g>
  <text x="340" y="110" fill="#888" text-anchor="middle">shader collapses a culled instance to a degenerate vertex → the whole arena still draws in ONE call</text>
  <text x="340" y="132" fill="#666" text-anchor="middle">culling changes the instance BUFFER, not the draw count</text>
  <defs><marker id="ah37" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
# Frustum: from_view_proj (Gribb–Hartmann, f64) + rebase-to-world + aabb_visible
src/camera.rs
# world_aabb(guid) → the object's world AABB extents (reads 40's world_boxes cache)
src/app/scene.rs
# apply_frustum_cull(): plane-test each world AABB → FLAG_CULLED on flipped rows; culled_now set
src/engine/gpu/mod.rs
# every vs that reads an instance row collapses a culled one to a clipped vertex
src/shaders/*.wgsl
src/state.rs          # render() builds the frustum, calls apply_frustum_cull before clear()
```

## Step 1 — the frustum: `src/camera.rs`

Gribb–Hartmann pulls the six planes straight out of a view-projection matrix. Build them in **f64**
from the same `view_proj` the renderer uses (before its `to_f32`), so the cull frustum matches the
rendered frame exactly. Add at the bottom of `camera.rs`:

```rust
/// Six inward-facing frustum planes [a,b,c,d]; a point is inside when a·x+b·y+c·z+d ≥ 0.
/// Extracted for wgpu clip space (z ∈ [0, w]). Built from the camera-relative view_proj (33),
/// then `rebased_to_world` so it can be tested against 40's absolute world boxes.
pub struct Frustum {
    pub planes: [[f64; 4]; 6],   // left, right, bottom, top, then the two z planes (26's
                                 // reverse-z puts FAR at z=0, so r2 is far and r3−r2 near)
}

impl Frustum {
    /// `m` is column-major (`m[col][row]`), as `Xform` stores it.
    pub fn from_view_proj(m: &[[f64; 4]; 4]) -> Self {
        let row = |r: usize| [m[0][r], m[1][r], m[2][r], m[3][r]];
        let (r0, r1, r2, r3) = (row(0), row(1), row(2), row(3));
        let add = |a: [f64; 4], b: [f64; 4]| [a[0]+b[0], a[1]+b[1], a[2]+b[2], a[3]+b[3]];
        let sub = |a: [f64; 4], b: [f64; 4]| [a[0]-b[0], a[1]-b[1], a[2]-b[2], a[3]-b[3]];
        let mut planes = [add(r3,r0), sub(r3,r0), add(r3,r1), sub(r3,r1), r2, sub(r3,r2)];
        for p in &mut planes {
            let n = (p[0]*p[0] + p[1]*p[1] + p[2]*p[2]).sqrt();
            // normalize → plane·point is a signed distance
            if n > 1e-20 { for c in p.iter_mut() { *c /= n; } }
        }
        Self { planes }
    }

    /// Shift each plane from camera-relative to world: d -= normal·origin (see the box above).
    /// `origin` is the render loop's rebase point — since 34c that's the ANCHOR, not the raw origin.
    pub fn rebased_to_world(mut self, origin: &Point) -> Self {
        for p in &mut self.planes {
            p[3] -= p[0]*origin[0] + p[1]*origin[1] + p[2]*origin[2];
        }
        self
    }

    /// Positive-vertex (n-vertex) test: pick each box corner on the plane's positive side; if even
    /// that corner is behind a plane, the box is outside. Conservative — never culls a visible box.
    pub fn aabb_visible(&self, min: [f64; 3], max: [f64; 3]) -> bool {
        for p in &self.planes {
            let px = if p[0] >= 0.0 { max[0] } else { min[0] };
            let py = if p[1] >= 0.0 { max[1] } else { min[1] };
            let pz = if p[2] >= 0.0 { max[2] } else { min[2] };
            if p[0]*px + p[1]*py + p[2]*pz + p[3] < 0.0 { return false; }
        }
        true
    }
}
```

`Point` is already imported in `camera.rs` (33's `origin()` returns one).

A plane test is exactly the kind of math that fails silently (an inverted plane culls the *visible*
half and the screen just looks empty), so pin it headless — at the bottom of `camera.rs`:

```rust
#[cfg(test)]
mod frustum_tests {
    use super::*;

    /// One real plane, five degenerate pass-throughs (a zero plane never culls: 0 < 0 is false).
    #[test]
    fn aabb_visible_plane_sides() {
        let mut planes = [[0.0; 4]; 6];
        planes[0] = [1.0, 0.0, 0.0, 0.0];   // inside when x ≥ 0
        let f = Frustum { planes };
        assert!(f.aabb_visible([1.0, -1.0, -1.0], [2.0, 1.0, 1.0]));     // fully inside
        assert!(f.aabb_visible([-1.0, -1.0, -1.0], [2.0, 1.0, 1.0]));    // straddling → conservative
        assert!(!f.aabb_visible([-3.0, -1.0, -1.0], [-1.5, 1.0, 1.0]));  // fully outside
    }

    /// Identity view_proj → clip space IS the world: the frustum must be exactly the wgpu clip
    /// box x,y ∈ [-1, 1], z ∈ [0, 1] (reverse-z planes included).
    #[test]
    fn identity_view_proj_gives_the_clip_box() {
        let f = Frustum::from_view_proj(&[[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0],
                                          [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]);
        assert!(f.aabb_visible([-0.5; 3], [0.5; 3]));                    // at the origin → visible
        assert!(!f.aabb_visible([2.0, 0.0, 0.2], [3.0, 1.0, 0.8]));      // past x = 1 → culled
        assert!(!f.aabb_visible([0.0, 0.0, 2.0], [0.5, 0.5, 3.0]));      // past far z = 1 → culled
        assert!(!f.aabb_visible([0.0, 0.0, -2.0], [0.5, 0.5, -1.0]));    // behind near z = 0 → culled
    }
}
```

Runs with the same wasm-override as 40's test: `cargo test -p session_viewer frustum --target
x86_64-unknown-linux-gnu`.

## Step 2 — plane-test each object, flag the losers: `src/engine/gpu/mod.rs`

The instance flag field gains one bit. It has to coexist with 35's `FLAG_HIDDEN` — so cull *sets and
clears its own bit*, never overwrites the whole field:

```rust
impl Instance {
    pub const FLAG_HIDDEN: u32 = 1 << 1;   // (35)
    pub const FLAG_CULLED: u32 = 1 << 7;   // ← ADD — off-screen this frame; independent of HIDDEN
}
```

First the world-AABB helper `Scene` owes us — add it to `impl Scene` in `app/scene.rs`, beside 40's
`objects_in` (the `OBB::aabb()` collapse already happened when 40's `add_file` filled the cache):

```rust
    /// The object's world AABB extents, in f64 — what the frustum plane test consumes.
    /// A CACHE READ, not a computation: 40's add_file stored every box's extents when it walked
    /// the vertices. Recomputing here would put an O(total vertices) walk inside the PER-FRAME
    /// cull — the classic hidden cost. The cache invalidates exactly when the BVH does: rebuilt or
    /// EXTENDED per `add_file`, since rows are appended globally across docs and the cache must stay
    /// row-indexed in lockstep (40's row-indexed cache).
    pub fn world_aabb(&self, guid: &str) -> ([f64; 3], [f64; 3]) {
        self.world_boxes[self.guid_to_row[guid] as usize]   // 40's row-indexed cache: rows are GLOBAL across docs
    }
```

Add a `culled_now: std::collections::HashSet<u32>` field to `Gpu` (init empty in `new`) — it remembers
which rows are currently culled so each frame only re-uploads the ones whose state **flipped**, not all
N. Then the per-frame cull (right after `new`, near 33's `rebuild_instances`):

```rust
    /// Plane-test every object's world AABB, set/clear FLAG_CULLED. Returns (drawn, culled) for the
    /// HUD. The scan is O(N), but only rows whose state CHANGED since last frame hit the GPU.
    /// Iterates the extents cache BY ROW — `world_boxes` is row-indexed (40), so a guid round-trip
    /// through `guid_to_row` here would be a double hash lookup per object per frame (and a panic
    /// on any stale guid). `world_aabb(guid)` stays for one-off lookups (57, 76).
    pub fn apply_frustum_cull(&mut self, scene: &crate::app::scene::Scene,
                              frustum: &crate::camera::Frustum) -> (u32, u32) {
        let (mut drawn, mut culled) = (0u32, 0u32);
        let mut flipped: Vec<(u32, Instance)> = Vec::new();
        for (row, &(lo, hi)) in scene.world_boxes.iter().enumerate() {
            let row = row as u32;
            let cull = !frustum.aabb_visible(lo, hi);
            let was = self.culled_now.contains(&row);
            if cull != was {
                let inst = &mut self.instances[row as usize];
                if cull { inst.flags |= Instance::FLAG_CULLED; }
                else { inst.flags &= !Instance::FLAG_CULLED; }
                flipped.push((row, *inst));
                if cull { self.culled_now.insert(row); } else { self.culled_now.remove(&row); }
            }
            if cull { culled += 1; } else { drawn += 1; }
        }
        // ONE upload per frame, not one write_buffer per flipped row: pack the flipped rows into
        // a staging buffer, batch one copy per row into a single encoder, submit once. (Each
        // write_buffer stages its own copy — hundreds of staging allocations on a fast orbit.)
        if !flipped.is_empty() {
            use wgpu::util::DeviceExt;
            let data: Vec<Instance> = flipped.iter().map(|(_, i)| *i).collect();
            let staging = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("cull.staging"), contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::COPY_SRC,
            });
            let sz = std::mem::size_of::<Instance>() as u64;
            let mut enc = self.device.create_command_encoder(&Default::default());
            for (k, (row, _)) in flipped.iter().enumerate() {
                enc.copy_buffer_to_buffer(&staging, k as u64 * sz,
                    &self.instance_buffer, *row as u64 * sz, sz);
            }
            self.queue.submit([enc.finish()]);
        }
        (drawn, culled)
    }
```

(The instance buffer already carries `COPY_DST` — 43b's `write_row` writes into it — so the batch
copies need no usage change. Rows are distinct, so the copies never overlap.)

> **Linear, and that's fine here — so why did 40 build a BVH?** Frustum cull *must* decide every
> object's flag every frame, so the scan is inherently O(N); the real optimization is the flip-tracking
> (`culled_now`), which keeps GPU traffic proportional to what *moved*, not to N. The BVH earns its keep
> in **picking (47)** and **box-select (50)**, where the alternative is a per-object triangle test that
> genuinely can't run N times per click. (A million-object scene could also BVH-query the frustum box to
> skip the plane test on distant objects — 81 picks that up; linear holds comfortably at the stress
> file's 42k.)

> **Why set/clear, not rebuild.** 33's `rebuild_instances` rewrites every row's model+color each frame
> but never touches `flags`; this writes only `flags`, only on the rows that flipped. The two never
> collide, and a hidden object (35) stays hidden whether or not it's also culled — different bits.

> **Cull bits are transient.** FLAG_CULLED lives only in the live instance buffer — deliberately *not*
> in the viewer-state flags 35's rewrite keeps in `scene.tables.objects[row].2` (SELECTED/HIDDEN), the
> truth `set_scene` re-derives instances from. So every `set_scene` — each `Msg::File` append rebuilds
> the instances from the tables — wipes the bit: clear `culled_now` there (one line in `set_scene`:
> `self.culled_now.clear();`) and let the next frame's cull recompute. The rule: persistent state lives
> in the tables; transient per-frame state like cull bits is recomputed after camera moves *and* after
> every `set_scene`.

## Step 3 — the shader collapses a culled instance: `src/shaders/*.wgsl`

One rule, applied in **every** vertex shader that reads an instance row for a table that has data
(`triangle.wgsl`, `cylinder.wgsl`, `sphere.wgsl`, plus 34f's `ribbon.wgsl`/`glyph.wgsl` — same
edit, same anchors as cylinder/sphere; `point.wgsl` also reads the row, but its cloud table stays
empty — 35 routes PointCloud through the GLYPH lane — add the same collapse there only if the
dormant cloud path ever wakes): if the row is
culled (or hidden), output a vertex
the rasterizer throws away, so the primitive vanishes without a branch on the CPU or a second draw call.
Each shader reaches the instance differently, so the anchor and the flag read differ per file — but the
collapse is identical: set the builtin position (`VsOut.pos`, **not** `o.clip` — no such field) to a
`w = 0` vector and return early. `var o: VsOut;` zero-inits the rest, so only `pos` matters.

In `triangle.wgsl`, right after `let inst = instances[in.inst_id];`:

```wgsl
if ((inst.flags & FLAG_CULLED) != 0u || (inst.flags & FLAG_HIDDEN) != 0u) {
    var o: VsOut; o.pos = vec4<f32>(0.0); return o;   // w = 0 → clipped away; the triangle collapses
}
```

In `cylinder.wgsl`, right after `let seg = segments[si];` (the segment carries `instance_id`):

```wgsl
let flags = instances[seg.instance_id].flags;
if ((flags & FLAG_CULLED) != 0u || (flags & FLAG_HIDDEN) != 0u) {
    var o: VsOut; o.pos = vec4<f32>(0.0); return o;   // the whole segment collapses
}
```

In `sphere.wgsl`, right after `let g = glyphs[gi];`:

```wgsl
let flags = instances[g.instance_id].flags;
if ((flags & FLAG_CULLED) != 0u || (flags & FLAG_HIDDEN) != 0u) {
    var o: VsOut; o.pos = vec4<f32>(0.0); return o;   // the glyph collapses
}
```

In `ribbon.wgsl`, right after `let seg = segments[vid / 6u];` — and in `glyph.wgsl`, right after
`let g = glyphs[vid / 3u];` — the same two lines as cylinder/sphere respectively (`seg.instance_id`
/ `g.instance_id`).

Declare the two consts at the top of each shader — `const FLAG_HIDDEN: u32 = 2u;
const FLAG_CULLED: u32 = 128u;` — matching the Rust bit values. **One draw call stays one draw call**:
31's `draw_indexed(0..N, 0, 0..segments.len())` fires for the whole arena; the GPU simply discards the
collapsed instances. Culling changes the *buffer*, never the *draw*.

> **What the collapse actually saves.** A `w = 0` vertex still *runs* the vertex shader — every culled
> instance pays its full vertex cost, and only then does the rasterizer throw the degenerate primitive
> away. So the win is fragment + depth work (and the bandwidth those would have burned), not vertex
> invocations; for the tube lanes (cylinder/sphere), which are vertex-bound, the saving is real but
> modest. Skipping the vertex work too means compacting the draw or going indirect — 81's territory.

## Step 4 — run it each frame: `src/state.rs`

In `render`, build the frustum from the **anchored** `view_proj` the frame actually draws with (34c),
rebase it to world, cull, then draw. `render` already computes the anchor and the matrix — the frustum
must come from that **same** matrix, and the rebase point is the `anchor`, not the raw `origin` — so
what's culled matches what's drawn. First bring the type in: extend the import at the top from
`use crate::camera::Camera;` to `use crate::camera::{Camera, Frustum};`.

The scan is O(N) over every row — pointless when the camera hasn't moved since the last scan (the
cull result depends only on the camera and the boxes). So gate it on a cheap camera signature, kept
on `Gpu` beside `culled_now` (add the field, init `None`; a `set_scene` invalidates the cull bits, so
it must reset the gate too — the Step-2 one-liner becomes `self.culled_now.clear();
self.last_cull_sig = None;`):

```rust
    // add to struct Gpu, beside culled_now (pub — state.rs's render gate reads/writes it):
    pub last_cull_sig: Option<([f64; 3], [f64; 3], f64, f64, bool)>,   // camera at the last scan
```

Then:

```rust
        // find, in render() — these three lines already exist (34c):
        let origin = self.camera.origin();
        let anchor = self.gpu.rebase_anchor(&origin, self.camera.distance_world());
        let view_proj = self.camera.view_proj_anchored(aspect, &anchor);
        // insert AFTER them, BEFORE the clear() call. Gate: re-cull only when the camera moved
        // (or set_scene reset the sig) — the O(N) scan's inputs are exactly these.
        let sig = ([origin[0], origin[1], origin[2]], self.camera.target,
                   self.camera.distance, aspect, self.camera.perspective);
        if self.gpu.last_cull_sig != Some(sig) {
            let frustum = Frustum::from_view_proj(&view_proj.to_cols())   // f64, anchor-relative
                // world frame — matches 40's boxes
                .rebased_to_world(&anchor);
            let (drawn, culled) = self.gpu.apply_frustum_cull(&self.scene, &frustum);
            self.gpu.perf_set_drawn(drawn, drawn + culled);
            self.gpu.last_cull_sig = Some(sig);
        }
        // the clear() line itself is unchanged (3-arg since 34d: color, view_proj, origin):
        self.gpu.clear(wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 }, &view_proj, &origin)
```

`perf_set_drawn` doesn't exist yet — 28's counter never had a drawn/total split. Give `Gpu` the
two numbers (the HUD lesson, 52, reads them): add `pub perf_drawn: u32, pub perf_total: u32,` to
`struct Gpu`, initialize both to `0` in the `Ok(Self { … })`, and add next to
`apply_frustum_cull`:

```rust
    /// HUD feed: how many objects survived the cull this frame, out of how many total.
    pub fn perf_set_drawn(&mut self, drawn: u32, total: u32) {
        self.perf_drawn = drawn;
        self.perf_total = total;
    }
```

(`Xform::to_cols()` is the kernel's f64 column-major accessor — `m[col][row]`, exactly what
`Frustum::from_view_proj` takes; this is its first viewer use, and 46's ray / 50's marquee lean on
it later.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

Load the stress file (34), press `F`, then zoom into one corner:

- **`drawn / total` drops** as objects leave the view — the whole point. Zoom back out and it
  climbs to the full count. (Until 52's HUD panel exists, log it: add
  `log::info!("cull: {} / {}", drawn, drawn + culled);` after the `apply_frustum_cull` call —
  or fold the two numbers into 28's once-a-second perf line.)
- **Slow-orbit at the screen edge and nothing pops in late.** This is the test that catches an
  inverted plane or a too-tight box: an object whose center is just outside the frustum but whose body
  still crosses the edge must stay drawn. The positive-vertex test (Step 1) guarantees it — if you see
  popping, a plane's sign is flipped or `aabb_visible` is using the wrong corner.
- **Draw count on the HUD is unchanged** — still the same handful of `draw_indexed` calls from 34. Only
  the segment/triangle *counts* inside them fall. That's the invariant: cull trims the buffer, not the
  pipeline.

## Recap

```
Ch 40: Scene.bvh + the world_boxes extents cache — world boxes per object (the row's placed
       frame — manifest place × session world xform — baked in).
Ch 41: FRUSTUM CULL. Per frame: 6 planes out of the f64 ANCHORED view_proj the frame draws with
       (view_proj_anchored, 34c; Gribb–Hartmann), rebased to WORLD (d −= n·anchor) so they match
       40's world boxes — the one subtlety camera-relative rendering forces. Positive-vertex test on every object's world AABB (linear, like
       the archive; cheap here — and gated on a camera signature, so a still camera skips the scan).
       Set/clear FLAG_CULLED (bit 7) — own bit, so 35's HIDDEN (bit 1) is
       untouched — and re-upload ONLY the rows whose state flipped (culled_now set), batched into
       ONE staging buffer + one submit per frame: flip-tracking, not a fancier structure, keeps it
       cheap. Every instance-reading vertex shader collapses a
       culled/hidden row to a w=0 vertex, so the arena still draws in ONE call: culling changes the
       BUFFER, not the DRAW COUNT — and the collapse saves fragment/depth work, not the vertex
       shader (tube lanes are vertex-bound; compaction/indirect is 81). HUD drawn/total finally
       moves; nothing pops at the screen edge; two #[cfg(test)]s pin the plane math.
```

Edited: `camera.rs` (`Frustum` + `from_view_proj` + `rebased_to_world` + `aabb_visible` +
`#[cfg(test)]` plane tests), `app/scene.rs` (`world_aabb(guid)` → world AABB extents from 40's cache),
`engine/gpu/mod.rs` (`FLAG_CULLED`, `culled_now`, `last_cull_sig`, `apply_frustum_cull` — row-indexed
scan, one batched staging upload), `shaders/*.wgsl` (collapse
culled/hidden instances), `state.rs` (build frustum, rebase, cull when the camera moved, feed the HUD).

## Next

`43a-gpu-arena.md` — Phase 6 opens: the `.pb` file becomes a live source. Reloading a file today
would rebuild the entire scene (35's `add_file` walk from scratch, plus a whole-buffer `set_scene`).
The next lesson diffs the incoming `Session`
against the current one by `guid` — added / removed / content-changed / unchanged — and re-flattens
**only** the objects that actually changed, replacing their arena slots in place and re-walking only
their boxes in 40's extents cache (the tree itself rebuilds from that cache — cheap), instead of
starting from zero.
