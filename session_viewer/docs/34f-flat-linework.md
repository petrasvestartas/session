# 34f Flat linework at scale — capsule ribbons, glyph dots, paper-space widths

> **Big picture.** 503k objects, 4 draw calls, CPU near-idle — and 10 fps. The arithmetic:
> 598,604 segments × 48 triangles (12-sided capped cylinder) ≈ **29M triangles and 30M
> vertex-shader runs for ~1.2M pixels of actual ink**. One instanced draw kills the CPU cost that
> scales with object count; it does nothing for GPU vertex/fill cost — and no API (wgpu included)
> changes how many triangles silicon rasterizes. The CAD answer is *pay per pixel*: our thickness
> is screen-constant 2–4px at EVERY zoom, so the cylinder's roundness is never visible — a flat
> camera-facing capsule makes the same pixels for 2 triangles. Measured end to end (headless
> probe, same scene):

```
12-sided cylinders   149 ms/frame          6-sided (still 3D)    ~75 ms      (2.0×)
capsule ribbons       28 ms/frame (5.3×)   user's GPU            ~18 ms
```

## Files we touch

```
src/engine/gpu/mod.rs           # CYL_SIDES 6, LINEWORK_SOLID switch, LineUniform + vp_w,
                                # mode-aware draws, planar paper-width pass
src/engine/gpu/adapters.rs      # encode_width — kernel width enters the radius encoding
src/shaders/ribbon.wgsl         # NEW — capsule edge quads (round caps!)
src/shaders/glyph.wgsl          # NEW — SDF circle dots (1 triangle)
src/engine/pipelines/build.rs   # build_ribbon_pipeline + build_glyph_pipeline
src/engine/pipelines/mod.rs     # ribbon + glyph fields; cylinder + sphere STAY
src/shaders/cylinder.wgsl       # LineUniform struct sync (32 B)
src/shaders/sphere.wgsl         # LineUniform struct sync
```

## Step 1 — the free 2×: `CYL_SIDES` 12 → 6

Screen-constant thickness means the cross-section is never visibly round — 12 sides is waste at
every zoom. **One constant** (`gpu/mod.rs`): 48→24 tris/segment, pixels identical, measured 2×.

## Step 2 — `LineUniform` grows `vp_w`

Flat shapes offset corners in CLIP space — pixels→NDC needs BOTH viewport axes. Grow the struct
to two vec4s (uniform sizes are 16-byte multiples) in `gpu/mod.rs`:

```rust
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    _pad: [f32; 3],
} // 32 B - two vec4s
```

Fill `vp_w`/`_pad` at BOTH write sites (init in `new()`, per-frame in `clear()`), and add the two
fields to the `LineUniform` mirror structs in `cylinder.wgsl` and `sphere.wgsl` — a mismatched
uniform struct is a pipeline-creation error, and both 3D pipelines still build (they're kept).

## Step 3 — the capsule ribbon: `src/shaders/ribbon.wgsl` (NEW)

One segment = 6 buffer-less verts (2 triangles) forming a screen-space **capsule**: the quad is
extruded perpendicular to the segment's SCREEN direction (always camera-facing), extended
half-width past both ends, and the fragment shader rounds the caps with an SDF — **round line
ends**, and polyline corners join smoothly because neighbouring caps overlap. Depth comes from
each endpoint's own clip position, so occlusion still works per-pixel.

```wgsl
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{ model: mat4x4<f32>, color: vec4<f32>, flags: u32, };
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment (48 B) — same table the cylinder pipeline reads.
struct CylinderSegment{
    p0: vec3<f32>, radius: f32, p1: vec3<f32>, instance_id: u32, color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32,
    vp_w: f32, _pad0: f32, _pad1: f32, _pad2: f32,
};

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>, // this fragment's screen position, px
    @location(2) @interpolate(flat) a: vec2<f32>,   // segment endpoints on screen, px
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(linear) hw: f32,      // half-width, px
};

//   corner 0: e0−   1: e1−   2: e1+     (tri 1)
//   corner 3: e0−   4: e1+   5: e0+     (tri 2)
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let seg = segments[vid / 6u];
    let corner = vid % 6u;
    let model = instances[seg.instance_id].model;

    let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(seg.p1, 1.0)).xyz;
    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);

    let at_end1 = corner == 1u || corner == 2u || corner == 4u;
    let side = select(-1.0, 1.0, corner == 2u || corner == 4u || corner == 5u);
    let clip = select(c0, c1, at_end1);

    // Endpoints in screen pixels (one consistent mapping, used both ways)
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (c0.xy / max(abs(c0.w), 1e-6) * 0.5 + 0.5) * vp;
    let s1 = (c1.xy / max(abs(c1.w), 1e-6) * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // px half-width at this end: global thickness, or a world radius projected (>0) —
    // the inverse of cylinder.wgsl's screen_radius, solved for pixels.
    var px = line.thickness;
    if (seg.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = seg.radius * line.vp_h / line.ortho_h;
        } else {
            px = seg.radius * line.proj_y * line.vp_h / clip.w;
        }
        px = max(px, 0.5); // paper-space ink never vanishes: floor at ~1px on screen
    }

    // Corner in px: sideways ± half-width, and PAST the end by half-width (cap room)
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * px;

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.zw);
    o.color = seg.color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw = px;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    // Capsule SDF in screen px — rounds both caps; MSAA smooths the rim.
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    if (length(pa - ba * h) > in.hw) { discard; }
    return in.color;
}
```

**`src/shaders/glyph.wgsl` (NEW)** — the dot sibling: 32b's 3-corner triangle whose incircle is
the disc, reading the GLYPH table, `px = line.thickness` with the same world-radius override,
opaque + depth-writing, SDF `discard` past radius 1 in corner space. (It is `point.wgsl` with the
glyph table, no blending, and depth writes on.)

## Step 4 — pipelines + the switch

**4a. `build.rs`**: `build_ribbon_pipeline` + `build_glyph_pipeline` — copies of
`build_point_pipeline` with their own labels/shaders, `blend: None`, `depth_write_enabled:
Some(true)`, and ribbon binding the SEGMENT layout at group 3.
**4b. `pipelines/mod.rs`**: two imports, two fields (`ribbon`, `glyph`), two build calls —
`cylinder` and `sphere` keep theirs.
**4c. `gpu/mod.rs`** — the switch, next to the other consts:

```rust
/// Linework style switch. Thickness is screen-constant px in BOTH modes, so at 2-4px the two are
/// pixel-identical — but not cost-identical:
/// `false` (default): FLAT — camera-facing ribbon edges (2 tris/segment) + circle-glyph dots
///                    (1 tri/dot). 598k segments ≈ 1.2M tris.
/// `true`:            SOLID — 3D cylinder edges + sphere dots. 598k segments ≈ 14M tris —
///                    measured ~5x slower at stress scale; kept for close-up/handle work.
const LINEWORK_SOLID: bool = false;
```

and in `clear()`, both linework blocks branch on it inside their 34b count-guards — SOLID keeps
the exact template draws, FLAT drops the template buffers:

```rust
                } else {
                    pass.set_pipeline(&self.pipelines.ribbon);
                    // …same four bind groups…
                    pass.draw(0..6 * self.segment_count, 0..1); // 6 verts per segment, no template
                }
                // dots: pipelines.glyph, pass.draw(0..3 * self.glyph_count, 0..1)
```

## Step 5 — paper-space lineweights (the 2D/3D split)

CAD convention: **2D drawing sheets have paper-mm lineweights** — zoom out and the ink thins like
a print; **model geometry (3D lines, mesh/BRep edges) stays screen-constant**. The shader already
has both lanes (`radius == 0` px-constant, `> 0` world-mm); route drawing widths into the world
lane:

**5a. `adapters.rs`** — the kernel width enters the encoding (`line_to_segment` and
`polyline_to_segments` set `radius: encode_width(…)`):

```rust
/// Kernel width (dimensionless, default 1.0) → the radius encoding's NEGATIVE lane (px
/// multiplier); 0.0 = plain global default. `walk_session` flips negatives into the POSITIVE
/// (world-mm) lane for planar 2D drawings — paper-space lineweights that scale with zoom.
pub(super) fn encode_width(w: f64) -> f32 {
    if w.is_finite() && w > 0.0 && (w - 1.0).abs() > 1e-9 { -(w as f32) } else { 0.0 }
}
```

**5b. `gpu/mod.rs` `walk_session`**, after the extent pass — planarity IS the discriminator
(every PDF conversion is exactly z ≡ 0; no flag needed in the data):

```rust
        // 2D DRAWING SHEETS (exactly planar, z ≡ 0 — every PDF conversion) get PAPER-SPACE
        // lineweights: kernel width (mm on the sheet) → the radius WORLD lane, so zooming out
        // thins the ink like a real print. 3D model files keep screen-constant px linework.
        let planar = t.min[2].is_finite() && (t.max[2] - t.min[2]).abs() < 1e-3;
        if planar {
            for s in &mut t.segments {
                s.radius = if s.radius < 0.0 { -s.radius * 0.5 } // width mm → half-width mm
                           else { 0.5 };                        // default width 1.0 → 0.5mm pen
            }
        }
```

The `max(px, 0.5)` floor in ribbon.wgsl (Step 3) keeps zoomed-out ink at ~1px instead of
shimmering away — also standard CAD behavior.

## Verify — measured, not vibed

The headless probe (no browser interaction needed — Chrome writes console lines to its debug log):

```bash
chrome --headless=new --enable-unsafe-webgpu --enable-logging --v=0 \
  --user-data-dir=/tmp/hl --window-size=1758,1347 http://127.0.0.1:8770/ &
sleep 100; kill %1; grep -oE '"perf[^"]*"' /tmp/hl/chrome_debug.log | tail -5
```

Same 503k-object wall: `149ms → 28ms` headless (5.3×), `~100ms → ~18ms` on a real GPU. Flip
`LINEWORK_SOLID = true` and diff by eye: identical at 2–4px. Zoom into a drawing cell: pens
fatten like paper (0.28/0.51/0.71mm weights now visibly differ); zoom out: hairlines, floored at
1px. Mesh edges in the floor_model cell stay 2px at every zoom.

## Recap

```
Ch 34f: PAY PER PIXEL. CYL_SIDES 6 (roundness is invisible at screen-constant width — free 2×).
        ribbon.wgsl: screen-space CAPSULE, 2 tris/segment, SDF round caps, per-endpoint depth.
        glyph.wgsl: 1-tri SDF discs. LINEWORK_SOLID keeps cylinders+spheres one constant away —
        same tables, same pixels. LineUniform +vp_w (32 B ×3 mirrors). Paper-space lineweights:
        planar files route kernel width → world-mm lane (zoom-dependent ink, 1px floor); 3D stays
        screen-constant. 149→28ms measured on the 503k wall.
```

## Next

`34g-camera-ux.md` — the wall renders fast; now it has to FEEL like CAD: cursor-centered zoom
with no range stops, and middle-mouse pan.
