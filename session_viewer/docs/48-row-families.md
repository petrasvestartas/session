# 48 Row families — one file per row format

> Third refactor lesson. Start from the end of lesson 47. Pixels stay identical on every
> mandatory scene; one advisory row moves, for a reason stated in Check.

<svg viewBox="0 0 720 300" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="one row family: CPU rows append into a GrowBuf, a bind group exposes it, the family's pipelines draw it and return a draw count; instantiated for arena, segments and glyphs" style="max-width:100%;height:auto;font:12px ui-monospace,monospace">
  <defs><marker id="qb" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
  <text x="360" y="16" fill="#888" font-size="11" text-anchor="middle">one row family — engine/gpu/&lt;family&gt;.rs (lesson 48)</text>
  <g fill="none">
    <rect x="14" y="30" width="110" height="40" stroke="#f0b35c"/><rect x="160" y="30" width="120" height="40" stroke="#6fb3ff"/>
    <rect x="316" y="30" width="110" height="40" stroke="#6fb3ff"/><rect x="462" y="30" width="100" height="40" stroke="#6fb3ff"/>
    <rect x="598" y="30" width="108" height="40" stroke="#7ed37e" stroke-width="1.3"/>
  </g>
  <g fill="#d7dae0" font-size="10" text-anchor="middle">
    <text x="69" y="47">Rows</text><text x="220" y="47">GrowBuf</text><text x="371" y="47">bind group</text><text x="512" y="47">Pipelines.*</text><text x="652" y="47">draw_*() -&gt; u32</text>
  </g>
  <g fill="#888" font-size="9" text-anchor="middle">
    <text x="69" y="62">CPU delta, per upload</text><text x="220" y="62">GPU table, append</text><text x="371" y="62">rows_group (seg, glyph)</text><text x="512" y="62">colour · prepass</text><text x="652" y="62">into draws +=</text>
  </g>
  <g stroke="#6fb3ff" marker-end="url(#qb)">
    <line x1="124" y1="50" x2="158" y2="50"/><line x1="280" y1="50" x2="314" y2="50"/><line x1="426" y1="50" x2="460" y2="50"/><line x1="562" y1="50" x2="596" y2="50"/>
  </g>
  <line x1="14" y1="86" x2="706" y2="86" stroke="#3a3a3a"/>
  <g fill="none">
    <rect x="14" y="100" width="210" height="40" stroke="#f0b35c"/><rect x="260" y="100" width="250" height="40" stroke="#6fb3ff"/><rect x="546" y="100" width="160" height="40" stroke="#7ed37e"/>
    <rect x="14" y="156" width="210" height="40" stroke="#f0b35c"/><rect x="260" y="156" width="250" height="40" stroke="#6fb3ff"/><rect x="546" y="156" width="160" height="40" stroke="#7ed37e"/>
    <rect x="14" y="212" width="210" height="40" stroke="#f0b35c"/><rect x="260" y="212" width="250" height="40" stroke="#6fb3ff"/><rect x="546" y="212" width="160" height="40" stroke="#7ed37e"/>
  </g>
  <g stroke="#6fb3ff" marker-end="url(#qb)">
    <line x1="224" y1="120" x2="258" y2="120"/><line x1="510" y1="120" x2="544" y2="120"/>
    <line x1="224" y1="176" x2="258" y2="176"/><line x1="510" y1="176" x2="544" y2="176"/>
    <line x1="224" y1="232" x2="258" y2="232"/><line x1="510" y1="232" x2="544" y2="232"/>
  </g>
  <g fill="#d7dae0" font-size="10">
    <text x="22" y="116">ArenaRows</text><text x="268" y="116">Arena</text><text x="554" y="116">draw_faces</text>
    <text x="22" y="172">SegRows</text><text x="268" y="172">SegmentLane</text><text x="554" y="172">draw_pipes(style)</text>
    <text x="22" y="228">GlyphRows</text><text x="268" y="228">GlyphLane</text><text x="554" y="228">draw_spheres</text>
  </g>
  <g fill="#888" font-size="9">
    <text x="22" y="131">verts · vids · idx · idx_print · idx_text</text>
    <text x="268" y="131">5 GrowBufs: verts vids faces print text</text>
    <text x="554" y="131">draw_print · draw_text</text>
    <text x="22" y="187">pipes, ribbons: CylinderSegment</text>
    <text x="268" y="187">pipes, ribbons: GrowBuf · groups · Template</text>
    <text x="554" y="187">draw_ribbons · ribbon_depth</text>
    <text x="22" y="243">spheres, dots: GlyphPoint</text>
    <text x="268" y="243">spheres, dots: GrowBuf · groups · Template</text>
    <text x="554" y="243">draw_dots · draw_dot_depth</text>
  </g>
  <g fill="#888" font-size="10">
    <text x="14" y="276">objects.rs: ObjectRows → InstanceTable (append · rebase_anchor · update_inside) — every instance_id indexes it</text>
    <text x="14" y="292">instance.rs: Instance + FLAG_* + wgsl_fields — 4 mirror tests (Rust struct ↔ WGSL struct) land in cargo xtest</text>
  </g>
</svg>

## Goal

Five new files under `gpu/`: `instance.rs` (the object row every other row points at),
`objects.rs` (its table), `arena.rs` (triangles), `segments.rs` (pipes and ribbons), `glyphs.rs`
(markers and dots). A family owns its CPU rows, its GPU tables, its bind groups and its draws,
and returns the number of draws it issued. `Gpu` goes from 64 fields to 39; the first four
tests appear.

## Why

A family is defined by the row it owns, not by a shader: `ribbon.wgsl` is compiled twice, for
two tables. Putting the table, its growth, its bind group and its draw calls in one file means a
wrong-pixel bug has one file to open. The mirror tests pin the Rust structs to the WGSL structs
field by field, so a drift names the file instead of misreading every row.

## Files

| file | change | lines after |
|---|---|---|
| `src/engine/gpu/instance.rs` | created | 165 |
| `src/engine/gpu/objects.rs` | created | 254 |
| `src/engine/gpu/arena.rs` | created | 123 |
| `src/engine/gpu/segments.rs` | created | 214 |
| `src/engine/gpu/glyphs.rs` | created | 168 |
| `src/engine/gpu/buffers.rs`, `frame.rs`, `view.rs`, `src/math.rs` | edited | — |
| `src/engine/gpu/upload.rs` | rewritten | 80 |
| `src/engine/gpu/mod.rs` | rewritten | 799 (was 1379) |
| `src/app/scene.rs`, `src/selftest.rs`, `examples/check_determinism.rs` | edited | — |

Steps 1-5 add files no module declares yet, Steps 6-10 edit files that now reference them, and
Steps 11-12 replace `upload.rs` and `gpu/mod.rs` whole (a `Create` on an existing path means:
delete every line, then paste). The first `cargo check` is in Check.

## Step 1 — `src/engine/gpu/instance.rs`

`Instance`, its flag bits, `wgsl_fields` (a fifteen-line parser for the field names of a WGSL
struct) and the four mirror tests. The three `use` lines in the test module resolve after Steps
4, 5 and 8.

**Create `src/engine/gpu/instance.rs`**

```rust
//! `Instance` - the one object row every instance-reading shader indexes by `instance_id`,
//! its flag bits, and the WGSL field parser the mirror tests use to prove the five shaders
//! declare the same row. No buffer and no bind group here: `objects.rs` owns the table.

use session_rust::Xform;

/// One object row as the five instance-reading shaders see it: the anchored model matrix,
/// the tint, the flag bits and two scalars the ink lanes read. 96 B, the storage stride.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub(crate) model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
    pub(crate) color: [f32; 4], // 16 B
    pub(crate) flags: u32, // 4 B - bit 0 reserved for FLAG_SELECTED
    /// This object's world AABB diagonal, in world units. The ink lanes CLAMP their lift to a
    /// fraction of it - see `LIFT_MAX_EXTENT` in ribbon.wgsl. 0.0 = unknown, no clamp.
    ///
    /// Without it the lift is a fraction of EYE DEPTH, so its world size grows with camera
    /// distance while an object's front-to-back size does not: past some distance the back
    /// wireframe is lifted in front of the front faces and the object goes see-through. Measured
    /// on a 1000 mm box at a 2px pen, that distance is 242 m for a band and 91 m for a marker -
    /// ordinary zoom-out in a scene spanning tens of metres.
    pub(crate) extent: f32, // 4 B
    /// Vertex spacing in world units (see `ObjectRows::spacing`). The ink lanes drop
    /// markers once this projects below a few pixels; 0 = unknown, never culled.
    pub(crate) spacing: f32, // 4 B
    pub(crate) _pad: u32, // 4 B - pad the row to 96 B (storage array stride)
}

impl Instance {
    /// The row is skipped by every draw. Bit 1; bit 0 is reserved for FLAG_SELECTED.
    pub const FLAG_HIDDEN: u32 = 1 << 1;
    /// The eye is inside this object's bounds (per-frame CPU test, see `update_inside_flags`).
    /// Both edge lanes then skip the facing cull - from inside a solid every face points away -
    /// and the flat lane hugs BOTH adjacent face planes, since the back-facing ones are the
    /// visible surface from in there. Bit 2, matching FLAG_INSIDE in ribbon.wgsl/cylinder.wgsl.
    pub const FLAG_INSIDE: u32 = 1 << 2;

    /// The mesh broadcast a zero edge width: it is PRINT, not surface - a PDF glyph, a poché
    /// region, any triangulated fill. triangle.wgsl lights such faces flat (lit = 1.0), so the
    /// authored colour reads the same from the back of the sheet as from the front. Bit 3.
    pub const FLAG_PRINT: u32 = 1 << 3;

    /// The mesh is NOT closed (boundary edges exist), so the facing cull's premise - both
    /// adjacent faces away = far side of a solid, hidden - is void: an interior surface can be
    /// genuinely visible through the hole, faces drawn but its wireframe culled (the bunny's
    /// open base). Set once at build time from Mesh::is_closed(); the edge lanes then skip the
    /// facing cull exactly as FLAG_INSIDE does and occlusion falls to the depth test, which
    /// both lanes already write honestly. Bit 4.
    pub const FLAG_OPEN: u32 = 1 << 4;

    /// This row belongs to a PLANAR file - a drawing sheet. Its fills write no depth (they are
    /// exactly coplanar and composite in document order instead), so the sheet's ink has nothing
    /// to fight and takes NO lift: ribbon.wgsl reads this bit and keeps the pen on the page. That
    /// is what lets the lettering pass, drawn last with a >= depth test, land on top of the
    /// linework the way the page draws it.
    pub const FLAG_SHEET: u32 = 1 << 5;

    /// The one-row placeholder an empty scene draws from: identity, mid grey, no flags.
    pub(crate) fn placeholder() -> Self {
        Self { model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0 }
    }
}

/// The field names of a WGSL `struct <name> { .. }`, in declaration order: `//` comments
/// stripped, fields split on `,` and newlines, the name taken before its `:`. Test-only.
#[cfg(test)]
pub(crate) fn wgsl_fields(src: &str, struct_name: &str) -> Vec<String> {
    let at = src.find(&format!("struct {struct_name}")).expect("struct declared in the shader");
    let rest = &src[at..];
    let open = rest.find('{').expect("struct body opens");
    let close = rest.find('}').expect("struct body closes");

    rest[open + 1..close]
        .lines()
        .map(|l| l.split("//").next().unwrap_or(""))
        .flat_map(|l| l.split(','))
        .map(|f| f.split(':').next().unwrap_or("").trim())
        .filter(|n| !n.is_empty())
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::frame::LineUniform;
    use crate::engine::gpu::glyphs::GlyphPoint;
    use crate::engine::gpu::segments::CylinderSegment;

    /// The Rust row's field names; `_pad` is layout-only and WGSL pads implicitly.
    const INSTANCE_FIELDS: [&str; 6] = ["model", "color", "flags", "extent", "spacing", "_pad"];

    /// Every instance-reading shader declares `Instance` with exactly the Rust fields, in order,
    /// and the Rust row is the 96 B stride the storage array uses.
    #[test]
    fn instance_mirror() {
        let shaders = [
            ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl")),
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
        ];
        let rust: Vec<&str> = INSTANCE_FIELDS.iter().copied().filter(|n| !n.starts_with('_')).collect();

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "Instance"), rust, "{name}: Instance fields");
        }
        assert_eq!(std::mem::size_of::<Instance>(), 96);
    }

    /// Five shaders declare `LineUniform`. The names cannot match 1:1: Rust's `eye: [f32; 3]`
    /// is `eye_x/eye_y/eye_z` in WGSL (three scalars fill the pad before `anchor`'s 16 B
    /// alignment) and `_pad1` is layout-only - so the comparison goes through that mapping,
    /// and the 48 B size is asserted on the Rust side.
    #[test]
    fn line_uniform_mirror() {
        let shaders = [
            ("grid.wgsl", include_str!("../../shaders/grid.wgsl")),
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
        ];
        let rust = ["thickness", "proj_y", "ortho_h", "vp_h", "vp_w", "eye_x", "eye_y", "eye_z", "anchor"];

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "LineUniform"), rust, "{name}: LineUniform fields");
        }
        assert_eq!(std::mem::size_of::<LineUniform>(), 48);
    }

    /// cylinder.wgsl and ribbon.wgsl read the same 40 B segment row. The ends are three scalars
    /// each in WGSL (a `vec3<f32>` would pad the row to 48), so Rust's `p0`/`p1` map to
    /// `p0x/p0y/p0z` and `p1x/p1y/p1z`.
    #[test]
    fn cylinder_segment_mirror() {
        let shaders = [
            ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
            ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
        ];
        let rust = ["p0x", "p0y", "p0z", "radius", "p1x", "p1y", "p1z", "instance_id", "color", "facing"];

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "CylinderSegment"), rust, "{name}: CylinderSegment fields");
        }
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40);
    }

    /// sphere.wgsl and glyph.wgsl read the same 48 B glyph row, field for field.
    #[test]
    fn glyph_point_mirror() {
        let shaders = [
            ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
            ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
        ];
        let rust = ["center", "radius", "color", "instance_id", "facing", "facing_ext"];

        for (name, src) in shaders {
            assert_eq!(wgsl_fields(src, "GlyphPoint"), rust, "{name}: GlyphPoint fields");
        }
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48);
    }
}
```

## Step 2 — `src/engine/gpu/objects.rs`

`ObjectRows` is the per-object columns a walk fills; `InstanceTable` is the one owner of the
instance rows: the true f64 transforms they are rebased from, the world boxes the inside test
walks, the buffer and its bind group. `rebase_anchor` now returns whether it rebuilt, so `Gpu`
can invalidate the splats.

**Create `src/engine/gpu/objects.rs`**

```rust
//! `ObjectRows` - the per-object columns a walk fills (true placement, tint, flags, local
//! AABB, vertex spacing) - and `InstanceTable`, the one owner of the instance rows the GPU
//! reads: their f64 mirrors, the re-anchor, the inside test, the buffer and its bind group.

use crate::engine::performance::now_ms;
use crate::engine::pipelines::Layouts;
use crate::math::{mat_to_f32, xform_point_f64, Aabb, Mat4};
use session_rust::Point;
use super::buffers::{rows_group, GpuCtx, GrowBuf};
use super::instance::Instance;

/// Re-anchor distance: the instance table is rebased about a snapped anchor.
/// The camera can drift this far (mm) before a full rebuild.
/// Within it, pan/zoon only changes the view matrix.
/// f32 error at 1e5 mm from the achor = 6e-3 mm - far below a pixel.
/// Re-anchor threshold, WORLD units (mm): a quarter of the current view distance, so a zoomed-out
/// pan does not rebuild constantly while a zoomed-IN pan re-anchors early enough that world
/// coordinates never regain the magnitude that eats f32 precision. Clamped to a sane band.
const REANCHOR_MIN: f64 = 1.0e3;
const REANCHOR_MAX: f64 = 1.0e5;

/// The object columns of one upload, aligned by row. `rows` is the ONE table the walk keeps
/// cumulative (the bounds sweep and the sheet pass index it by global row); `InstanceTable::append`
/// takes only the rows past what it already holds.
#[derive(Default)]
pub struct ObjectRows {
    /// TRUE world model + tint + flags per object; the instance rows are rebased from it.
    pub rows: Vec<(Mat4, [f32; 4], u32)>,
    /// Mesh-local AABB per object row. None for linework/points/clouds: only the solid lane's
    /// facing cull needs it (see `Instance::FLAG_INSIDE`).
    pub bounds: Vec<Option<([f32; 3], [f32; 3])>>,
    /// Vertex spacing per object row, world units. 0 = unknown (linework, points, clouds),
    /// which the ink lanes read as "never density-cull".
    pub spacing: Vec<f32>,
}

/// The instance rows and everything that rewrites them: the true f64 transforms they are
/// rebased from, the world AABBs the inside test walks, and the GPU table itself.
pub struct InstanceTable {
    instances: Vec<Instance>,
    last_origin: Option<Point>, // rebuild skips when the camera target did not move
    objects_base: Vec<(Mat4, [f32; 4], u32)>, // TRUE world model+color; instances[] is rebased from this
    base_f32: Vec<[f32; 16]>, // model.to_f32() cached once - rebase only re-patches 3 slots
    bounded_rows: Vec<u32>, // rows with Some(world AABB) - the only ones the inside test walks
    /// Per-object WORLD AABB (`ObjectRows::bounds` through the true transform), aligned with
    /// `instances`. Drives FLAG_INSIDE - see `update_inside`.
    object_bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    inside: Vec<bool>, // current FLAG_INSIDE state per instance row, for change detection
    buffer: GrowBuf, // `rebuild` rewrites it whole on every re-anchor
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    /// Group 2 of every instance-reading pipeline; rebuilt when the buffer grows.
    pub group: wgpu::BindGroup,
}

impl InstanceTable {
    /// One placeholder row, so the first frame binds a real buffer and draws nothing from it.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let instances: Vec<Instance> = vec![Instance::placeholder()];
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let buffer = GrowBuf::new(ctx, "instance.buffer", std::mem::size_of::<Instance>() as u64, rows);
        let group = rows_group(ctx, &l.instance, "instances.bind_group", &buffer.buf);

        Self {
            instances,
            last_origin: None,
            objects_base: Vec::new(),
            base_f32: Vec::new(),
            bounded_rows: Vec::new(),
            object_bounds_world: Vec::new(),
            inside: Vec::new(),
            buffer,
            last_rebase_ms: 0.0,
            group,
        }
    }

    /// Take the rows past what the table already holds, mirror them, and send only those.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &ObjectRows) {
        // `up.rows` is the ONE table the walk keeps cumulative - the bounds sweep and the
        // per-file sheet pass both index it by global row - so this is the one lane that gets a
        // full table every time instead of a delta. Only the NEW rows are turned into instances
        // and sent: cloning 148k rows per file was 22 MB of memcpy and a full re-upload, for a
        // tail that had not changed since the file before.
        let base = self.objects_base.len();
        if base == 0 {
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.instances.clear();
            self.buffer.reset();
        }
        debug_assert_eq!(up.rows.len(), up.bounds.len());
        debug_assert!(up.rows.len() >= base, "the object table only ever grows");
        self.objects_base.extend_from_slice(&up.rows[base..]);
        // Rebase re-patches only translations, so the 13 other floats can be cast once here
        // instead of per re-achor: at 210k objects that turns a 20+ msCPU loop into a copy
        self.base_f32.extend(up.rows[base..].iter().map(|(m, _, _)| mat_to_f32(m)));
        self.object_bounds_world.extend(up.rows[base..].iter().zip(&up.bounds[base..]).map(|((m, _, _), b)| b.map(|(lo, hi)| world_aabb(m, lo, hi))));
        self.inside.resize(self.objects_base.len(), false);
        self.bounded_rows = self.object_bounds_world.iter().enumerate().filter_map(|(i, b)| b.map(|_| i as u32)).collect();
        // `object_bounds_world` was just extended above, so each row's extent comes from the same
        // AABB FLAG_INSIDE uses. The diagonal, not an axis: a flat sheet has a zero-thickness axis
        // and would clamp its ink lift to nothing.
        let bounds = &self.object_bounds_world;
        self.instances.extend(up.rows[base..].iter().enumerate().map(|(i, (m, c, f))| Instance {
            model: mat_to_f32(m),
            color: *c,
            flags: *f,
            extent: bounds.get(base + i).and_then(|b| *b).map_or(0.0, |(lo, hi)| diagonal(lo, hi)),
            spacing: up.spacing.get(base + i).copied().unwrap_or(0.0),
            _pad: 0,
        }));

        if self.instances.is_empty() {
            self.instances.push(Instance::placeholder());
        }

        let fresh = &self.instances[self.buffer.len() as usize..];
        if self.buffer.append(ctx, fresh) {
            self.group = rows_group(ctx, &l.instance, "instances.bind_group", &self.buffer.buf);
        }
        self.last_origin = None; // force the next frame to rebase against the new table
    }

    /// The anchor the instance table is rebased about.
    /// A full rebuild (42 000 x at stress scale) runs
    /// only when the camera target strays REANCHOR_DIST from the current anchor - orbit newer moves the target.
    /// And pan/zoom within the budget just changes the view matrix
    /// `origin` and `view_dist` are both in WORLD units (mm) - the same units as the instance
    /// table's translations. Feeding metres here (the camera's internal unit) makes the subtract
    /// below a no-op at 1/1000 scale, which silently turns camera-relative rendering off: the
    /// symptom is geometry that jitters and then clips away entirely as you zoom in, because the
    /// f32 mvp is differencing two large world magnitudes.
    pub fn rebase_anchor(&mut self, ctx: &GpuCtx, origin: &Point, view_dist: f64) -> (Point, bool) {
        let thresh = (view_dist * 0.25).clamp(REANCHOR_MIN, REANCHOR_MAX);
        let need = match &self.last_origin {
            None => true,
            Some(a) => {
                let (dx, dy, dz) = (a[0] - origin[0], a[1] - origin[1], a[2] - origin[2]);
                (dx * dx + dy * dy + dz * dz).sqrt() > thresh
            }
        };
        // Throttled: during a wheel-zoom gesture the target moves every tick,
        // and an every-frame rebuild is the motion jank the rule forbids.
        // Between rebuulds the old achor stays valid - it is just farther from the eye than the threshold likes, which costs f32 precision
        // only past the threshold distance, never a wrong image.
        let now = now_ms();
        let moved = need && (now - self.last_rebase_ms > 200.0 || self.last_origin.is_none());
        if moved {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }
        (self.last_origin.clone().unwrap(), moved)
    }

    /// Rebase every instance's translation around 'origin' - an f64 subtract agains the TRUE world transfrom in 'objects_base'
    /// Then cast to f32.
    /// 'instances', what GPU actually sees, never holds a coordinate bigger than the camera's distnace from 'origin',
    /// no matter how fas the scene fists from world (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point) {
        self.last_origin = Some(origin.clone());
        for (i, (model, _, _)) in self.objects_base.iter().enumerate() {
            let mut m = self.base_f32[i]; // rotation / scale casr once at set_scene
            m[12] = (model[12] - origin[0]) as f32;
            m[13] = (model[13] - origin[1]) as f32;
            m[14] = (model[14] - origin[2]) as f32;
            self.instances[i].model = m;
        }
        ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.instances));
    }

    /// Per-frame refresh of Instance::FLAG_INSIDE. The facing cull in both edge lanes assumes the
    /// eye is OUTSIDE the solid (both adjacent faces turned away = hidden edge); from inside, every
    /// face points away and the whole object loses its wireframe the moment the camera crosses a
    /// face. Per-edge data cannot tell "far side of the solid" from "eye inside it" - that
    /// difference is global - so the CPU answers it per object from the world AABBs, and the answer
    /// rides the instance row. One containment test per object per frame; the instance buffer is
    /// rewritten only when some answer flips, which orbit/zoom almost never does.
    pub fn update_inside(&mut self, ctx: &GpuCtx, eye: [f32; 3], scene: &Aabb) {
        if self.bounded_rows.is_empty() {
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= scene.min[k] as f64 && ew[k] <= scene.max[k] as f64);
        let mut dirty = false;
        for &row in &self.bounded_rows {
            let i = row as usize;
            let b = &self.object_bounds_world[i];
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
            if self.inside.get(i).copied().unwrap_or(false) == inside {
                continue;
            }
            if let Some(row) = self.instances.get_mut(i) {
                row.flags = if inside { row.flags | Instance::FLAG_INSIDE } else { row.flags & !Instance::FLAG_INSIDE };
            }
            if i < self.inside.len() { self.inside[i] = inside; }
            dirty = true;
        }
        if dirty {
            ctx.queue.write_buffer(&self.buffer.buf, 0, bytemuck::cast_slice(&self.instances));
        }
    }

    /// Forget every row; the buffer keeps its capacity.
    pub fn reset(&mut self) {
        self.objects_base.clear();
        self.base_f32.clear();
        self.object_bounds_world.clear();
        self.inside.clear();
        self.instances.clear();
        self.buffer.reset();
        // DERIVED from object_bounds_world (rebuilt in append), so leaving it behind holds row
        // indices into a vector that is now empty: a scene cleared and then DRAWN before the
        // next upload would panic in update_inside on the stale rows.
        self.bounded_rows.clear();
    }

    /// One instance row, as the GPU sees it (rebased about the anchor).
    pub fn row(&self, i: u32) -> Option<&Instance> {
        self.instances.get(i as usize)
    }

    /// Rows in the table - the frame's object count.
    pub fn len(&self) -> u32 {
        self.instances.len() as u32
    }

    /// The anchor the rows are rebased about, as the shaders read it; zero before the first frame.
    pub fn anchor_f32(&self) -> [f32; 3] {
        self.last_origin.as_ref().map(|o| [o[0] as f32, o[1] as f32, o[2] as f32]).unwrap_or([0.0; 3])
    }
}

/// The AABB diagonal, world units - the lift clamp `Instance::extent` carries.
fn diagonal(lo: [f64; 3], hi: [f64; 3]) -> f32 {
    ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
}

/// World AABB of a local box: the 8 corners through the true transform. Conservative for
/// rotated placements - FLAG_INSIDE is a hint, not a cull.
fn world_aabb(m: &Mat4, lo: [f32; 3], hi: [f32; 3]) -> ([f64; 3], [f64; 3]) {
    let mut wlo = [f64::INFINITY; 3];
    let mut whi = [f64::NEG_INFINITY; 3];
    for c in 0..8 {
        let p = xform_point_f64(m, [
            (if c & 1 == 0 { lo[0] } else { hi[0] }) as f64,
            (if c & 2 == 0 { lo[1] } else { hi[1] }) as f64,
            (if c & 4 == 0 { lo[2] } else { hi[2] }) as f64,
        ]);
        for k in 0..3 { wlo[k] = wlo[k].min(p[k]); whi[k] = whi[k].max(p[k]); }
    }
    (wlo, whi)
}
```

## Step 3 — `src/engine/gpu/arena.rs`

The mesh arena: one vertex table and three index runs (faces, sheet fills, lettering).
`verts`/`vids`/`faces` use `GrowBuf::new_exact`, which Step 6 adds; `draw_faces` returns 1 even
when empty, because the draw-count goldens record it that way.

**Create `src/engine/gpu/arena.rs`**

```rust
//! The mesh arena - one vertex table every mesh, BRep and sheet fill shares, and the three
//! index runs drawn from it: solid faces, sheet fills (depth write off), lettering (last of
//! all). `ArenaRows` is one upload's delta; `Arena` is the GPU side. No ink lives here.

use crate::engine::pipelines::Pipelines;
use session_rust::RenderVertex;
use super::buffers::{GpuCtx, GrowBuf};
use super::frame::Binds;

/// One upload's mesh rows: vertices, their instance ids, and the three index runs.
/// Sheet lanes: a PDF's fills are exactly coplanar, so they must NOT arbitrate by depth - they
/// are split off the solid run and drawn in document order with depth write off. `idx_text`
/// is the lettering, drawn LAST of all, after the ink lanes, because a page puts its text on
/// top of both its hatching and its linework.
#[derive(Default)]
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

/// The arena on the GPU. `verts`/`vids`/`faces` grow EXACT-fit: this is the biggest table in
/// the viewer (64 MB of vertices on a six-file scene) and it grows once per file, so doubling
/// would hold up to 2x the geometry for nothing. The two sheet runs double like every lane.
pub struct Arena {
    verts: GrowBuf,
    vids: GrowBuf,
    faces: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
}

impl Arena {
    /// Five one-row tables; the first upload sizes them.
    pub fn new(ctx: &GpuCtx) -> Self {
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;

        Self {
            verts: GrowBuf::new_exact(ctx, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu),
            vids: GrowBuf::new_exact(ctx, "arena.vids", 4, vu),
            faces: GrowBuf::new_exact(ctx, "arena.ibo", 4, iu),
            print: GrowBuf::new(ctx, "arena.ibo.print", 4, iu),
            text: GrowBuf::new(ctx, "arena.ibo.text", 4, iu),
        }
    }

    /// Append one file's rows. They are a DELTA - the caller drops them after upload, because
    /// nothing reads them back (picking goes through the kernel Meshes in `Doc.session`).
    /// The sheet runs index the SAME vertex table, so splitting them costs one buffer each.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        self.verts.append(ctx, &up.verts);
        self.vids.append(ctx, &up.vids);
        self.faces.append(ctx, &up.idx);
        self.print.append(ctx, &up.idx_print);
        self.text.append(ctx, &up.idx_text);
    }

    /// The solid faces, one indexed draw over the whole table. Counts 1 even when the table is
    /// empty - the draw-count goldens record it that way.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        pass.set_pipeline(&p.triangle);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);

        if !self.faces.is_empty() {
            pass.set_vertex_buffer(0, self.verts.buf.slice(..)); // slot 0 - vertices
            pass.set_vertex_buffer(1, self.vids.buf.slice(..)); // slot 1 - per-vertex row ids
            pass.set_index_buffer(self.faces.buf.slice(..), wgpu::IndexFormat::Uint32);
            pass.draw_indexed(0..self.faces.len(), 0, 0..1); // whole scene, one call
        }
        1
    }

    /// SHEET FILLS, second. Same vertex table, depth WRITE off, so a page's exactly coplanar
    /// regions composite in document order instead of flickering over one shared depth value.
    /// They still depth-TEST, so 3D geometry in front of the sheet occludes.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        self.draw_run(pass, p, b, &self.print)
    }

    /// LETTERING, last of everything. A page paints its text on top of its hatching AND its
    /// linework, so it lands after the ink lanes - the one thing draw order can express that a
    /// depth buffer cannot, since all of it is coplanar at z = 0.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        self.draw_run(pass, p, b, &self.text)
    }

    /// One sheet run through the depth-read-only triangle pipeline; 0 draws when it is empty.
    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds, run: &GrowBuf) -> u32 {
        if run.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.triangle_sheet);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_vertex_buffer(0, self.verts.buf.slice(..));
        pass.set_vertex_buffer(1, self.vids.buf.slice(..));
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.len(), 0, 0..1);
        1
    }

    /// Forget what the arena holds, so the next upload writes from row 0 again. The buffers and
    /// their capacity stay - only the counters move - so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.verts.reset();
        self.vids.reset();
        self.faces.reset();
        self.print.reset();
        self.text.reset();
    }

    /// Vertices on the GPU - the MSAA test and the scene log read it.
    pub fn vert_count(&self) -> u32 {
        self.verts.len()
    }
}
```

## Step 4 — `src/engine/gpu/segments.rs`

`CylinderSegment`, `LineStyle`, and `SegmentLane` with its two tables; `draw_pipes` takes the
style and returns 1 for tubes, 2 for the flat prepass and colour pair.

**Create `src/engine/gpu/segments.rs`**

```rust
//! The segment family - every straight piece of ink. Two tables of the same 40 B row: pipes
//! (mesh/BRep edges, the SOLID lane) and ribbons (line/polyline/curve, the FLAT lane), plus
//! the unit cylinder the tube style instances. `SegRows` is one upload; `SegmentLane` the GPU.

use crate::engine::pipelines::{Layouts, Pipelines};
use super::buffers::{rows_group, GpuCtx, GrowBuf, Template};
use super::frame::Binds;

/// Sides of the unit cylinder: six is the fewest that reads as round at pen widths.
const CYL_SIDES: u32 = 6;

/// One segment row, 40 B, the layout cylinder.wgsl and ribbon.wgsl both declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CylinderSegment {
    // The two ends are FLAT f32s, not `[f32; 3]`, and that is deliberate. WGSL gives `vec3<f32>`
    // an alignment of 16, so any struct containing one is padded to a multiple of 16 - this table
    // was 48 B and could not have been 40 whatever else was packed. Scalars align to 4, so the
    // stride is the honest sum of the fields. Costs one `vec3<f32>(..)` per end in the shaders.
    pub p0: [f32; 3],   // 12 B - start point
    pub radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 0 -> wolrd mm override
    pub p1: [f32; 3],   // 12 B - end point (p0..instance_id = 32 B of geometry)
    pub instance_id: u32,  // 4 B - row in instances[]: object model + flags (hide/select later)
    // Was `[f32; 4]` - 16 B carrying what is really 8-bit RGBA. Packing it paid for `facing`
    // AND took 8 B off every segment: 48 -> 40, which is 20% of the biggest table in the viewer
    // (118 MB at mesh-stress scale).
    pub color: u32,     // 4 B - RGBA8, low byte red
    // The two faces this edge belongs to, as octahedral unit normals, 16 bits each - about 1.4
    // degrees, when all that is asked of them is the SIGN of a dot product (the facing cull) and
    // a plane to hug (the flat lane's depth solve). This is what lets the shader answer "is this
    // edge facing the camera" without the depth buffer: both faces facing away means the edge is
    // hidden and must not be drawn at all. FACING_UNKNOWN = unknown, always draw (polylines,
    // drawing linework, BRep edges with no adjacency); 0 is a real value - a +Z/+Z face pair.
    pub facing: u32,    // 4 B
}                       // 40 B

// The WGSL CylinderSegment (cylinder.wgsl AND ribbon.wgsl - same table) is exactly this layout;
// the array stride is the struct's, so a drift here misreads every row.
const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory - which is the whole reason
/// the two lanes were built over one buffer. Easy3D ships exactly this pair
/// (`lines_cylinders_*` against `lines_plain_*_width_control`) and lets you pick.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface it
    /// decorates so silhouette edges never lose the depth test.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader. Cheaper, and it
    /// lies IN the surface rather than proud of it.
    Flat,
}

/// One upload's segments: the solid lane's pipes and the flat lane's ribbons.
#[derive(Default)]
pub struct SegRows {
    /// Solid lane: mesh/BRep edges, drawn as 3D cylinders or as ribbons with a depth prepass.
    pub pipes: Vec<CylinderSegment>,
    /// Flat lane: line/polyline/curve, drawn as camera-facing ribbons.
    pub ribbons: Vec<CylinderSegment>,
}

/// Linework lane is per GEOMETRY TYPE, not global (both stay screen-constant px):
/// SOLID (cylinder + sphere) for mesh/BRep, whose ink lies ON a surface - the tube radius lifts
///   it off that surface, so a silhouette edge cannot lose the depth test to its own face.
/// FLAT (ribbon + glyph) for line/polyline/point, which float free and have nothing to fight.
/// Routing lives in `app::scene::Scene`; one draw per lane here.
///
/// The two tables used to share one buffer, solid rows first. One buffer meant one splice point,
/// and a splice point moves whenever either half grows - so appending a file was impossible and
/// every upload rebuilt the whole table. Two buffers, same layout and same shader (each lane
/// indexes from row 0), and both grow by appending.
pub struct SegmentLane {
    pipes: GrowBuf,
    ribbons: GrowBuf,
    pipe_group: wgpu::BindGroup,
    ribbon_group: wgpu::BindGroup,
    template: Template,
}

impl SegmentLane {
    /// Two one-row tables (VERTEX-visible, read-only storage) and the unit cylinder, uploaded once.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stride = std::mem::size_of::<CylinderSegment>() as u64;
        let pipes = GrowBuf::new(ctx, "pipes.buffer", stride, rows);
        let ribbons = GrowBuf::new(ctx, "segments.buffer", stride, rows);
        let pipe_group = rows_group(ctx, &l.segment, "pipes.bind_group", &pipes.buf);
        let ribbon_group = rows_group(ctx, &l.segment, "segments.bind_group", &ribbons.buf);
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let template = Template::new(ctx, "cyl.template", &cyl_v, &cyl_i);

        Self { pipes, ribbons, pipe_group, ribbon_group, template }
    }

    /// Append one file's rows (a DELTA); a bind group is rebuilt only when its buffer grew.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &SegRows) {
        if self.pipes.append(ctx, &up.pipes) {
            self.pipe_group = rows_group(ctx, &l.segment, "pipes.bind_group", &self.pipes.buf);
        }
        if self.ribbons.append(ctx, &up.ribbons) {
            self.ribbon_group = rows_group(ctx, &l.segment, "segments.bind_group", &self.ribbons.buf);
        }
    }

    /// The solid lane: mesh/BRep edges as real cylinders (the tube radius lifts the ink off the
    /// surface it sits on, so silhouette edges never lose the depth test), or as flat ribbons
    /// with a depth prepass. Tubes = 1 draw, Flat = prepass + colour = 2.
    pub fn draw_pipes(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds, style: LineStyle) -> u32 {
        if self.pipes.is_empty() {
            return 0;
        }

        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.pipe_group, &[]);
        match style {
            LineStyle::Tubes => {
                pass.set_pipeline(&p.cylinder);
                pass.set_vertex_buffer(0, self.template.vbo.slice(..));
                pass.set_index_buffer(self.template.ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.template.index_count, 0, 0..self.pipes.len()); // one template, N edges
                1
            }
            // The flat lane's own shader over the SOLID table. DEPTH PREPASS
            // first (binary at half coverage): the blended colour pass writes no depth,
            // so its AA feather can never depth-reject a later stroke's opaque core -
            // that rejection read as pale flecks inside the bunny's wireframe.
            LineStyle::Flat => {
                pass.set_pipeline(&p.ribbon_solid_depth);
                pass.draw(0..4, 0..self.pipes.len());
                pass.set_pipeline(&p.ribbon_solid);
                pass.draw(0..4, 0..self.pipes.len());
                2
            }
        }
    }

    /// The flat lane's colour pass: line/polyline/curve ribbons, blended, depth read-only.
    pub fn draw_ribbons(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.ribbons.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.ribbon);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.ribbon_group, &[]);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.len());
        1
    }

    /// The flat lane's depth prepass (`INK_DEPTH_PREPASS`): the same ribbons, depth only.
    pub fn draw_ribbon_depth(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.ribbons.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.ribbon_depth);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.ribbon_group, &[]);
        pass.draw(0..4, 0..self.ribbons.len());
        1
    }

    /// Forget every row; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.pipes.reset();
        self.ribbons.reset();
    }

    /// Solid-lane rows on the GPU - the MSAA test reads it.
    pub fn pipe_count(&self) -> u32 {
        self.pipes.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn ribbon_count(&self) -> u32 {
        self.ribbons.len()
    }
}

/// Unit-cylinder template mesh (positions + indices) along +Z, radius 1, z in [0,1], with cap fans.
/// The shader rescales xy by the screen-constant radius and maps z along (p1-p0), so it's registered ONCE.
fn unit_cylinder(sides: u32) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut v: Vec<[f32; 3]> = Vec::new();
    let mut idx: Vec<u32> = Vec::new();
    for s in 0..sides{
        let a = s as f32 / sides as f32 * std::f32::consts::TAU;
        v.push([a.cos(), a.sin(), 0.0]);
        v.push([a.cos(), a.sin(), 1.0]);
    }
    for s in 0..sides{
        let b0 = 2 * s;
        let b1 = 2 * ((s+1) % sides);
        idx.extend_from_slice(&[b0, b1, b1 + 1, b0, b1+1, b0+1]); // Two triangles per side face
    }
    let cb = v.len() as u32;
    v.push([0.0, 0.0, 0.0]);
    let ct = v.len() as u32;
    v.push([0.0, 0.0, 1.0]);
    for s in 0..sides{
        let b0 = 2 * s;
        let b1 = 2 * ((s+1)%sides);
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]); // bottom + top fan
    }
    (v, idx)
}
```

## Step 5 — `src/engine/gpu/glyphs.rs`

`GlyphPoint` and `GlyphLane` with its two tables; `draw_spheres` is the prepass and colour pair.

**Create `src/engine/gpu/glyphs.rs`**

```rust
//! The glyph family - every vertex-sized piece of ink. Two tables of the same 48 B row: spheres
//! (mesh/BRep vertex markers, the SOLID lane, drawn on a quad template) and dots (free points,
//! the FLAT lane, three verts per dot). `GlyphRows` is one upload; `GlyphLane` the GPU.

use crate::engine::pipelines::{Layouts, Pipelines};
use super::buffers::{rows_group, GpuCtx, GrowBuf, Template};
use super::frame::Binds;

/// One marker or dot row, 48 B (three 16 B rows), the layout sphere.wgsl and glyph.wgsl declare.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GlyphPoint {
    pub center: [f32; 3], // 12 B - mesh-local
    pub radius: f32, // 4 B - 0.0 - screen-constant px; 0 - world mm
    pub color:  [f32; 4],
    pub instance_id: u32, // 4 B - row insntaces
    // Up to SIX incident face normals (oct16 pairs), widest incident edge's two first - the same
    // adjacency CylinderSegment carries one word of. A marker that hugs only the widest edge's
    // two faces still loses a sector of its disc to the THIRD face's band at a trihedral corner
    // (measured on a box corner); all-ones (FACING_UNKNOWN) means "no adjacency / no more".
    pub facing: u32,
    pub facing_ext: [u32; 2],
} // 48 B total, three 16-byte rows

// The WGSL GlyphPoint (glyph.wgsl AND sphere.wgsl - same table) is exactly this layout; the
// array stride is the struct's, so a drift here misreads every row.
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);

/// One upload's glyphs: the solid lane's vertex markers and the flat lane's dots.
#[derive(Default)]
pub struct GlyphRows {
    /// Solid lane: mesh/BRep vertices, radius matched to the pipes.
    pub spheres: Vec<GlyphPoint>,
    /// Flat lane: points, drawn as SDF dots.
    pub dots: Vec<GlyphPoint>,
}

/// Vertex ink, split like the segments: spheres are mesh/BRep vertices, dots are flat points.
/// Same layout and same table shape; each lane indexes from row 0 and grows by appending.
pub struct GlyphLane {
    spheres: GrowBuf,
    dots: GrowBuf,
    sphere_group: wgpu::BindGroup,
    dot_group: wgpu::BindGroup,
    template: Template,
}

impl GlyphLane {
    /// Two one-row tables and the camera-facing quad, uploaded once.
    pub fn new(ctx: &GpuCtx, l: &Layouts) -> Self {
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stride = std::mem::size_of::<GlyphPoint>() as u64;
        let spheres = GrowBuf::new(ctx, "spheres.buffer", stride, rows);
        let dots = GrowBuf::new(ctx, "glyphs.buffer", stride, rows);
        let sphere_group = rows_group(ctx, &l.glyph, "spheres.bind_group", &spheres.buf);
        let dot_group = rows_group(ctx, &l.glyph, "glyphs.bind_group", &dots.buf);
        let (sph_v, sph_i) = unit_quad();
        let template = Template::new(ctx, "sph.template", &sph_v, &sph_i);

        Self { spheres, dots, sphere_group, dot_group, template }
    }

    /// Append one file's rows (a DELTA); a bind group is rebuilt only when its buffer grew.
    pub fn append(&mut self, ctx: &GpuCtx, l: &Layouts, up: &GlyphRows) {
        if self.spheres.append(ctx, &up.spheres) {
            self.sphere_group = rows_group(ctx, &l.glyph, "spheres.bind_group", &self.spheres.buf);
        }
        if self.dots.append(ctx, &up.dots) {
            self.dot_group = rows_group(ctx, &l.glyph, "glyphs.bind_group", &self.dots.buf);
        }
    }

    /// Vertex markers are drawn LAST of the solid lane, after the bands, and their
    /// pipeline compares GreaterEqual. Drawn FIRST (the previous arrangement) the marker
    /// had to win STRICTLY - the band, testing GreaterEqual against the marker's depth,
    /// takes the pixel on any tie - so every pixel where the two computed the same depth
    /// went to the band, and the disc lost a bite of its rim wherever a band cap crossed
    /// it. Ordering it last inverts that: the marker only has to MATCH the band's depth to
    /// keep the pixel, which is a strictly weaker condition, so it can only ever draw more
    /// of the disc. Real occlusion is untouched - anything genuinely nearer still has a
    /// higher depth and still wins.
    ///
    /// Faces are already down by this point, so a vertex hidden inside the solid stays
    /// hidden, which was the reason markers went early in the first place.
    ///
    /// Prepass + colour = 2 draws; the caller gates on `show_mesh_edges && markers`.
    pub fn draw_spheres(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.spheres.is_empty() {
            return 0;
        }

        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.sphere_group, &[]);
        pass.set_vertex_buffer(0, self.template.vbo.slice(..));
        pass.set_index_buffer(self.template.ibo.slice(..), wgpu::IndexFormat::Uint32);
        // Same prepass split as the solid ribbons - see `SegmentLane::draw_pipes`.
        pass.set_pipeline(&p.sphere_depth);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.len());
        pass.set_pipeline(&p.sphere);
        pass.draw_indexed(0..self.template.index_count, 0, 0..self.spheres.len()); // one template, N glyphs
        2
    }

    /// The flat lane's colour pass: SDF dots, three verts each, no template.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.dots.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.glyph);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.dot_group, &[]);
        pass.draw(0..3 * self.dots.len(), 0..1); // 3 verts/dot, no template
        1
    }

    /// The flat lane's depth prepass (`INK_DEPTH_PREPASS`): the same dots, depth only.
    pub fn draw_dot_depth(&self, pass: &mut wgpu::RenderPass<'_>, p: &Pipelines, b: &Binds) -> u32 {
        if self.dots.is_empty() {
            return 0;
        }

        pass.set_pipeline(&p.glyph_depth);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.dot_group, &[]);
        pass.draw(0..3 * self.dots.len(), 0..1);
        1
    }

    /// Forget every row; the buffers keep their capacity.
    pub fn reset(&mut self) {
        self.spheres.reset();
        self.dots.reset();
    }

    /// Solid-lane rows on the GPU - the MSAA test reads it.
    pub fn sphere_count(&self) -> u32 {
        self.spheres.len()
    }

    /// Flat-lane rows on the GPU.
    pub fn dot_count(&self) -> u32 {
        self.dots.len()
    }
}

/// Camera-facing quad template (positions + indices) for the instanced vertex markers. The
/// shader expands it in SCREEN space and trims to a circle in the fragment with a 1px AA ramp,
/// so the silhouette is a perfect circle at any radius. This replaced a tessellated unit sphere:
/// 6x3 segments was a comment-era choice ("a few pixels across") that reads as a hexagon at the
/// sizes world-mm pens reach, and any fixed tessellation is still a polygon when you zoom in -
/// the SDF is exact and cheaper (2 triangles instead of 36+).
fn unit_quad() -> (Vec<[f32; 3]>, Vec<u32>) {
    let v = vec![
        [-1.0, -1.0, 0.0],
        [ 1.0, -1.0, 0.0],
        [ 1.0,  1.0, 0.0],
        [-1.0,  1.0, 0.0],
    ];
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}
```

## Step 6 — `src/engine/gpu/buffers.rs`

`GrowBuf::new_exact`: the arena's three vertex-side tables grow to exactly what they need.

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// A growable GPU table: capacity doubles when it runs out, the live prefix is copied GPU-side
/// and only the new rows are written. Appending is what lets the CPU copy go after upload -
```

**Replace with:**

```rust
/// A growable GPU table: capacity doubles when it runs out (or grows exact-fit, see `new_exact`),
/// the live prefix is copied GPU-side and only the new rows are written. Appending is what lets the CPU copy go after upload -
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
    label: &'static str,
```

**Add below it:**

```rust
    exact: bool,
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
        Self { buf, len: 0, cap: 1, stride, usage, label }
```

**Replace with:**

```rust
        Self { buf, len: 0, cap: 1, stride, usage, label, exact: false }
    }

    /// The same table with EXACT-fit growth: capacity becomes what is needed, never more. For
    /// a table that grows once per file and dwarfs every other (the mesh arena), doubling would
    /// hold up to 2x the geometry for nothing; the price is one GPU-side copy per append.
    pub fn new_exact(ctx: &GpuCtx, label: &'static str, stride: u64, usage: wgpu::BufferUsages) -> Self {
        Self { exact: true, ..Self::new(ctx, label, stride, usage) }
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
            let new_cap = need.max(self.cap * 2);
```

**Replace with:**

```rust
            let new_cap = if self.exact { need } else { need.max(self.cap * 2) };
```

## Step 7 — `src/engine/gpu/upload.rs`

Replace the whole file. The columns are grouped by family: `arena`, `seg`, `glyph`, the cloud
columns, then `obj` and the box.

**Create `src/engine/gpu/upload.rs`**

```rust
//! `Upload` - the walked rows on their way to the GPU: every family's table for one file (a
//! DELTA) plus the cumulative object columns. Built by `app::scene::Scene`, borrowed by
//! `Gpu::set_scene`, then emptied. No wgpu type and no kernel type here.

use crate::math::Aabb;
use super::arena::ArenaRows;
use super::glyphs::GlyphRows;
use super::objects::ObjectRows;
use super::segments::SegRows;
use super::{CloudDraw, LodNode};

/// Everything `Gpu` needs to fill its buffers, built and owned by `app::scene::Scene`;
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs)
/// and are spliced solid-first at upload.
/// `obj` holds the TRUE per-object transform + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
pub struct Upload {
    pub arena: ArenaRows,
    pub seg: SegRows,
    pub glyph: GlyphRows,
    pub cloud_pos: Vec<f32>, // Raw lane: 3 floats per point, 12 B
    pub cloud_col: Vec<u32>, // Raw lane: RBGA8 per point, 4 B
    pub cloud_nrm: Vec<u32>, // Raw lane: oct16 normal per point (u32::MAX = none), 4 B -> 20 B/pt
    pub cloud_nodes: Vec<LodNode>, // every walked cloud's octree nodes; a draw owns one slice
    pub cloud_draws: Vec<CloudDraw>, // first, count, instance, point spacing world units
    pub obj: ObjectRows,
    pub bounds: Aabb,
}

impl Default for Upload {
    /// Every lane empty and the box inverted, ready for the first walk.
    fn default() -> Self {
        Self {
            arena: ArenaRows::default(),
            seg: SegRows::default(),
            glyph: GlyphRows::default(),
            cloud_pos: Vec::new(),
            cloud_col: Vec::new(),
            cloud_nrm: Vec::new(),
            cloud_draws: Vec::new(),
            cloud_nodes: Vec::new(),
            obj: ObjectRows::default(),
            bounds: Aabb::empty(),
        }
    }
}

impl Upload {
    /// Forget the uploaded rows: the GPU is their only holder now. Every drawn table goes -
    /// nothing reads them back (picking goes through the kernel Meshes in `Doc.session`), and a
    /// kept copy is what let lanes rebuild whole buffers per file. `obj` STAYS: the instance
    /// table is rebased from it on every re-anchor, and the walk indexes it by global row.
    pub fn drop_uploaded(&mut self) {
        drop_rows(&mut self.arena.verts);
        drop_rows(&mut self.arena.vids);
        drop_rows(&mut self.arena.idx);
        drop_rows(&mut self.arena.idx_print);
        drop_rows(&mut self.arena.idx_text);
        drop_rows(&mut self.seg.pipes);
        drop_rows(&mut self.seg.ribbons);
        drop_rows(&mut self.glyph.spheres);
        drop_rows(&mut self.glyph.dots);
        drop_rows(&mut self.cloud_pos);
        drop_rows(&mut self.cloud_col);
        drop_rows(&mut self.cloud_nrm);
        drop_rows(&mut self.cloud_draws);
        drop_rows(&mut self.cloud_nodes);
    }
}

/// Empty a table AND hand its allocation back. `clear()` alone keeps the capacity, which on
/// these tables is the whole point of the exercise - a scan's cleared-but-capacious `cloud_pos`
/// holds exactly as much wasm heap as a full one.
fn drop_rows<T>(v: &mut Vec<T>) {
    v.clear();
    v.shrink_to_fit();
}
```

## Step 8 — `src/engine/gpu/frame.rs`

`LineUniform` becomes `pub(crate)` so the mirror test can measure it.

**Find** in `src/engine/gpu/frame.rs`:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LineUniform{
```

**Replace with:**

```rust
/// The line/pen block (group 1), 48 B - three vec4s; the mirror test checks the shaders' copy.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct LineUniform {
```

## Step 9 — `src/engine/gpu/view.rs`

`LineStyle` is imported from `segments.rs` now.

**Find** in `src/engine/gpu/view.rs`:

```rust
/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
/// switching costs one branch at the draw site and nothing in memory - which is the whole reason
/// the two lanes were built over one buffer. Easy3D ships exactly this pair
/// (`lines_cylinders_*` against `lines_plain_*_width_control`) and lets you pick.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LineStyle {
    /// A real 3D tube per edge: 12 triangles, and the radius lifts the ink off the surface it
    /// decorates so silhouette edges never lose the depth test.
    Tubes,
    /// A camera-facing quad per edge: 6 vertices, the flat lane's own shader. Cheaper, and it
    /// lies IN the surface rather than proud of it.
    Flat,
}
```

**Replace with:**

```rust
use super::segments::LineStyle;
```

## Step 10 — `src/math.rs`

`xform_point_f64`, for the world boxes. The fragment splits `xform_point`'s closing brackets
from the new function that reuses them: the `]` and `}` below the anchor now close
`xform_point_f64`.

**Find** in `src/math.rs`:

```rust
        (m[2] * x + m[6] * y + m[10] * z + m[14]) as f32,
```

**Add below it:**

```rust
    ]
}

/// The same placement in f64 end to end - the world AABB corners that FLAG_INSIDE tests.
pub fn xform_point_f64(m: &Mat4, p: [f64; 3]) -> [f64; 3] {
    let [x, y, z] = p;
    [
        m[0] * x + m[4] * y + m[8] * z + m[12],
        m[1] * x + m[5] * y + m[9] * z + m[13],
        m[2] * x + m[6] * y + m[10] * z + m[14],
```

## Step 11 — `src/engine/gpu/mod.rs`

Delete every line and paste; nothing from the old file survives. `Gpu` keeps the floor and the
point lanes; `set_scene` is a list of `append` calls, and `encode_frame`'s draw sites are family
calls that add their returned counts.

**Create `src/engine/gpu/mod.rs`**

```rust
//! `Gpu` - the lowest layer of the viewer (ARCHITECTURE.md §1): the floor (surface, `GpuCtx`,
//! layouts, pipelines, frame uniforms, targets, view), the four row families (objects, arena,
//! segments, glyphs - one file each, one shader row format each), and, until lesson 49, the
//! cloud/stream/splat lanes and the frame list. It knows nothing app-specific.

pub mod arena;
pub mod buffers;
pub mod device;
pub mod frame;
pub mod glyphs;
pub mod instance;
pub mod objects;
pub mod present;
pub mod segments;
pub mod targets;
pub mod upload;
pub mod view;

use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Pipelines, Target};
use crate::math::Aabb;
use session_rust::Point;

use buffers::{zeroed_buffer, GpuCtx, GrowBuf};
use device::DeviceSetup;
use frame::{Binds, FrameCx, FrameUniforms};
use glyphs::GlyphLane;
use segments::SegmentLane;
use targets::Targets;

pub use arena::Arena;
pub use frame::FrameInput;
pub use glyphs::GlyphPoint;
pub use instance::Instance;
pub use objects::InstanceTable;
pub use segments::{CylinderSegment, LineStyle};
pub use upload::Upload;
pub use view::View;

/// Depth prepass for the FLAT lane, so flat ink occludes flat ink (a dot behind a polyline
/// loses to it) instead of pure draw order deciding - and draw order here is HashMap order,
/// so without it "who is in front" is effectively random. Costs a SECOND full pass over every
/// ribbon/dot; set false to trade correct ink ordering for that frame time back.
/// Off: on 2D sheets (600k segments, all ribbons) the second pass doubles the frame.
const INK_DEPTH_PREPASS: bool = false;

/// One cloud's contiguous point range, as the record builder sees it. It was a
/// `(first, count, instance, spacing)` tuple until the octree gave every cloud a second
/// range - its slice of the LOD node table - and six positional fields is where a tuple
/// stops being readable.
#[derive(Clone, Copy)]
pub struct CloudDraw {
    pub first: u32,      // absolute first row in the cloud tables
    pub count: u32,
    pub instance: u32,   // the instance row this cloud draws against
    pub spacing: f32,    // measured point spacing, world units (0 = unknown)
    pub node_first: u32, // first LodNode of this cloud in the nodes table (walked lane)
    pub node_count: u32, // 0 = no octree (streamed clouds) - the record covers everything
}

/// One octree node of a WALKED cloud (kernel `SpatialOctree`): its own spacing-limited
/// subsample as a row range, its cube for the screen-error test, and the accept spacing
/// that drives the attenuated splat radius. `first` is RELATIVE to the cloud's own first
/// point and `children` are indices RELATIVE to the cloud's node slice; -1 = none.
#[derive(Clone, Copy)]
pub struct LodNode {
    pub center: [f32; 3], // cube centre, cloud-LOCAL units
    pub size: f32,        // cube edge, cloud-local units
    pub spacing: f32,     // accept spacing, cloud-local units
    pub first: u32,       // row offset from the draw's own `first`
    pub count: u32,
    pub children: [i32; 8],
}

pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // Screen to draw pixels on; None when headless.
    pub ctx: GpuCtx,                         // Device (makes resources) + queue (submits work).
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
    /// Layouts survive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    pub layouts: Layouts,
    pub pipelines: Pipelines,
    pub frame: FrameUniforms,                // mvp / line / cloud uniforms + this frame's eye and ortho
    pub targets: Targets, // depth + MSAA colour at the sample count this scene chose (see `msaa_now`)
    /// The runtime knobs: what to show, ink style, cloud/EDL/LOD scalars, pen weight.
    pub view: View,
    /// The object rows: instances, their f64 mirrors, the re-anchor and the inside test.
    pub objects: InstanceTable,
    /// The mesh arena: one vertex table, three index runs (faces, sheet fills, lettering).
    pub arena: Arena,
    /// The segment family: pipes (solid lane) and ribbons (flat lane) over one row layout.
    pub segments: SegmentLane,
    /// The glyph family: spheres (solid lane markers) and dots (flat lane) over one row layout.
    pub glyphs: GlyphLane,
    pub point_pos: GrowBuf, // positions, array<f32> - three rows per point
    pub point_col: GrowBuf, // colours, array<u32> RGBA8
    pub point_nrm: GrowBuf, // normals, array<u32> oct16 (u32::MAX = none)
    splat_depth_buf: wgpu::Buffer, // one u32 per pixel: winning reverse-Z bits (0 = empty)
    splat_color_buf: wgpu::Buffer, // one u32 per pixel: winner's RBGA8
    splat_recs: wgpu::Buffer,
    splat_group0: wgpu::BindGroup,
    splat_group1: wgpu::BindGroup,
    splat_resolve_group: wgpu::BindGroup,
    splat_total: u32,
    splat_state: Option<([f32; 16], f32)>, // (mvp, cloud_size) the buffers were build for; None = stale
    cloud_nodes: Vec<LodNode>,
    cloud_draws: Vec<CloudDraw>, // (first, count, instance, spacing)
    pub point_count: u32,
    // The STREAM lane: clouds whose points never existed on the CPU. Their own three buffers
    // and record table - the walked lane above is rebuilt whole by every set_scene, so a
    // streamed cloud cannot live in it. The two lanes meet in the shared per-pixel
    // depth/colour buffers: atomics compose across dispatches.
    stream_pos_buf: wgpu::Buffer,
    stream_col_buf: wgpu::Buffer,
    stream_nrm_buf: wgpu::Buffer,
    stream_capacity: u64, // rows
    stream_count: u32,
    stream_pos_at: u32,
    stream_col_at: u32,
    pub stream_draws: Vec<CloudDraw>, // (first, count, instance, spacing)
    splat_stream_recs: wgpu::Buffer,
    splat_group0_stream: wgpu::BindGroup,
    splat_group1_stream: wgpu::BindGroup,
    pub performance: Performance,
    pub bounds: Aabb,
}

impl Gpu {
    /// Set up the five wgpu objects, in order: Instance → Surface → Adapter → Device + Queue → configure.
    /// The scene starts empty - every upload, including the first file, goes through `set_scene`
    /// (progressive loading calls it once per appended file), One code path, not two.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), size.width.max(1), size.height.max(1)).await
    }

    /// Same stack with no window and no surface, rendering into an offscreen texture. Exists so
    /// a shader can be checked against a PNG on this machine instead of against the user's eyes.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, width.max(1), height.max(1)).await
    }

    /// The shared constructor: negotiate the device, make every layout, buffer, bind group and
    /// pipeline, and start with an empty scene.
    async fn build(
        window: Option<std::sync::Arc<winit::window::Window>>,
        width: u32,
        height: u32,
    ) -> anyhow::Result<Self> {
        let DeviceSetup { surface, device, queue, config } = device::open(window, (width, height)).await?;
        let ctx = GpuCtx { device, queue };

        // Depth and MSAA - the empty scene starts flat (1x); set_scene flips to 4x when the
        // first solid geometry arrives.
        let samples = 1;
        let targets = Targets::new(&ctx, &config, samples);

        // Every bind-group layout, once; pipelines and bind groups are made from these.
        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, (config.width, config.height));

        // The four row families start as one zeroed row each: wgpu cannot bind a 0-byte
        // buffer, and every length is 0, so the first frame draws nothing. The loader calls
        // set_scene the moment the first file's tables exist.
        let objects = InstanceTable::new(&ctx, &layouts);
        let arena = Arena::new(&ctx);
        let segments = SegmentLane::new(&ctx, &layouts);
        let glyphs = GlyphLane::new(&ctx, &layouts);

        // Point cloud tables - empty until set_scene fills them from the upload.
        let rows = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let point_count = 0u32;
        let point_pos = GrowBuf::new(&ctx, "points.buffer", 4, rows);
        let point_col = GrowBuf::new(&ctx, "points.col.buffer", 4, rows);
        let point_nrm = GrowBuf::new(&ctx, "points.nrm.buffer", 4, rows);

        // compute splatting - buffers, layouts, groups, pipelines.
        // the per-pixel buffers are framebuffer-sized u32s;
        // clear_buffer COPY_DST
        let pixels = (config.width.max(1) * config.height.max(1)) as u64 * 4;
        let splat_depth_buf = zeroed_buffer(&ctx.device, "splat.depth", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_color_buf = zeroed_buffer(&ctx.device, "splat.color", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_recs = zeroed_buffer(&ctx.device, "splat.rescales", 16 + 256 * 144, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0 = Self::mk_splat_group0(
            &ctx.device,
            &layouts.splat_group0,
            &frame.mvp_buffer,
            &frame.cloud_buffer,
            &splat_recs
        );

        let splat_group1 = Self::mk_splat_group1(
            &ctx.device,
            &layouts.splat_group1,
            &point_pos.buf,
            &point_col.buf,
            &point_nrm.buf,
            &splat_depth_buf,
            &splat_color_buf,
        );

        // stream lane: same layouts, its own buffers; grown for real by stream_reserve
        let stream_usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stream_pos_buf = zeroed_buffer(&ctx.device, "stream.pos", 12, stream_usage);
        let stream_col_buf = zeroed_buffer(&ctx.device, "stream.col", 4, stream_usage);
        let stream_nrm_buf = zeroed_buffer(&ctx.device, "stream.nrm", 4, stream_usage);
        let splat_stream_recs = zeroed_buffer(&ctx.device, "splat.stream.recs", 16 + 256 * 144,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
        let splat_group0_stream = Self::mk_splat_group0(&ctx.device, &layouts.splat_group0, &frame.mvp_buffer, &frame.cloud_buffer, &splat_stream_recs);
        let splat_group1_stream = Self::mk_splat_group1(&ctx.device, &layouts.splat_group1, &stream_pos_buf, &stream_col_buf, &stream_nrm_buf, &splat_depth_buf, &splat_color_buf);
        let splat_resolve_group = Self::mk_splat_resolve_group(
            &ctx.device,
            &layouts.splat_resolve,
            &splat_depth_buf,
            &splat_color_buf,
        );

        // Pipelines - render and compute, one set per sample count.
        let pipelines = Pipelines::new(&ctx.device, Target { format: config.format, samples }, &layouts);

        // Output
        log::info!("viewer init OK — surface {}x{}, format {:?}", config.width, config.height, config.format);
        Ok(Self {
            surface,
            ctx,
            config,
            layouts,
            pipelines,
            frame,
            targets,
            view: View::from_env(),
            objects,
            arena,
            segments,
            glyphs,
            point_pos,
            point_col,
            point_nrm,
            splat_depth_buf,
            splat_color_buf,
            splat_recs,
            splat_group0,
            splat_group1,
            splat_resolve_group,
            splat_total: 0,
            splat_state: None,
            stream_pos_buf,
            stream_col_buf,
            stream_nrm_buf,
            stream_capacity: 1,
            stream_count: 0,
            stream_pos_at: 0,
            stream_col_at: 0,
            stream_draws: Vec::new(),
            splat_stream_recs,
            splat_group0_stream,
            splat_group1_stream,
            cloud_draws: Vec::new(),
            cloud_nodes: Vec::new(),
            point_count,
            performance: Performance::new(),
            bounds: Aabb { min: [0.0; 3], max: [0.0; 3] },
        })
    }

    /// Append one upload to every family - called once per file while progressive loading
    /// appends. Every table but `obj` is a DELTA: only this file's rows travel, and a bind group
    /// is rebuilt only when the buffer behind it grew. An MSAA flip (first solid file after
    /// flat-only ones) also rebuilds the targets and every pipeline: sample count belongs to the PASS.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        self.arena.append(&self.ctx, &up.arena);
        self.segments.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);

        // Raw cloud lane, same deal. `cloud_draws` carries each cloud's absolute first-point
        // offset, which `Scene` keeps running across files - so the draw records append too.
        self.point_pos.append(&self.ctx, &up.cloud_pos);
        self.point_col.append(&self.ctx, &up.cloud_col);
        self.point_nrm.append(&self.ctx, &up.cloud_nrm);
        self.point_count = self.point_pos.len() / 3;
        // The walk numbers a cloud's nodes from the start of ITS upload; the lane's table is
        // cumulative, so every draw's node slice is rebased on the way in - the same thing
        // `Scene::cloud_base` already does for the point rows.
        let node_base = self.cloud_nodes.len() as u32;
        self.cloud_nodes.extend_from_slice(&up.cloud_nodes);
        self.cloud_draws.extend(up.cloud_draws.iter().map(|d| CloudDraw { node_first: d.node_first + node_base, ..*d }));
        self.rebuild_splat_groups();
        self.splat_state = None;

        if up.bounds.is_finite() { // an empty upload (the State boots before any file) knows no box
            self.bounds = up.bounds;
        }

        log::info!(
            "scene: {} objects {} arena verts {} segments ({} pipes) {} glyphs ({} spheres) {} cloud points",
            self.objects.len(), self.arena.vert_count(), self.segments.pipe_count() + self.segments.ribbon_count(), self.segments.pipe_count(),
            self.glyphs.sphere_count() + self.glyphs.dot_count(), self.glyphs.sphere_count(), self.point_count
        );

        let samples = self.msaa_now();
        if samples != self.targets.samples {
            self.targets = Targets::new(&self.ctx, &self.config, samples);
            self.pipelines = Pipelines::new(&self.ctx.device, Target { format: self.config.format, samples }, &self.layouts);
            log::info!("msaa: {}x", samples);
        }
    }

    /// The anchor the instance table is rebased about - see `InstanceTable::rebase_anchor`.
    /// A rebase moves every instance model, so the splats are stale.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64) -> Point {
        let (anchor, moved) = self.objects.rebase_anchor(&self.ctx, origin, view_dist);
        if moved {
            self.splat_state = None;
        }
        anchor
    }

    /// Splat group 0 for one lane: the frame uniforms and that lane's record table. The three
    /// splat groups are rebuilt whenever any bound buffer is recreated (set_scene, resize).
    fn mk_splat_group0(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        mvp: &wgpu::Buffer,
        cloud: &wgpu::Buffer,
        recs: &wgpu::Buffer
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("splat.group0"),
            layout,
            entries: &[
                wgpu::BindGroupEntry{binding: 0, resource: mvp.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 1, resource: cloud.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 2, resource: recs.as_entire_binding()},
            ],
        })
    }

    /// Splat group 1 for one lane: its point buffers and the shared per-pixel depth/colour pair.
    fn mk_splat_group1(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        pos: &wgpu::Buffer,
        col: &wgpu::Buffer,
        nrm: &wgpu::Buffer,
        sdepth: &wgpu::Buffer,
        scolor: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("splat.group1"),
            layout,
            entries: &[
                wgpu::BindGroupEntry{binding: 0, resource: pos.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 1, resource: col.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 2, resource: sdepth.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 3, resource: scolor.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 4, resource: nrm.as_entire_binding()},
            ],
        })
    }

    /// The resolve pass's view of the per-pixel splat buffers.
    fn mk_splat_resolve_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        sdepth: &wgpu::Buffer,
        scolor: &wgpu::Buffer,
    ) -> wgpu::BindGroup{
        device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("splat.resolve.group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry{binding: 0, resource: sdepth.as_entire_binding()},
                wgpu::BindGroupEntry{binding: 1, resource: scolor.as_entire_binding()},
            ],
        })
    }

    /// Re-point the five splat bind groups at the current buffers (set_scene, resize, stream growth).
    fn rebuild_splat_groups(&mut self){
        self.splat_group0 = Self::mk_splat_group0(&self.ctx.device, &self.layouts.splat_group0, &self.frame.mvp_buffer, &self.frame.cloud_buffer, &self.splat_recs);
        self.splat_group1 = Self::mk_splat_group1(&self.ctx.device, &self.layouts.splat_group1, &self.point_pos.buf, &self.point_col.buf, &self.point_nrm.buf, &self.splat_depth_buf, &self.splat_color_buf);
        self.splat_group0_stream = Self::mk_splat_group0(&self.ctx.device, &self.layouts.splat_group0, &self.frame.mvp_buffer, &self.frame.cloud_buffer, &self.splat_stream_recs);
        self.splat_group1_stream = Self::mk_splat_group1(&self.ctx.device, &self.layouts.splat_group1, &self.stream_pos_buf, &self.stream_col_buf, &self.stream_nrm_buf, &self.splat_depth_buf, &self.splat_color_buf);
        self.splat_resolve_group = Self::mk_splat_resolve_group(&self.ctx.device, &self.layouts.splat_resolve, &self.splat_depth_buf, &self.splat_color_buf);

    }

    /// Make room for `need` stream rows total, copying the live prefix GPU-side.
    ///
    /// EXACT, not doubling: appends here are few and huge, so doubling would waste over a
    /// hundred MB on a multi-scan scene AND worsen the worst transient (old+new live at once).
    /// What doubling avoids is a GPU-side copy - the one thing here that never touches wasm.
    fn stream_reserve(&mut self, need: u64) {
        if need <= self.stream_capacity { return }
        let cap = need;
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let pos = zeroed_buffer(&self.ctx.device, "stream.pos", cap * 12, usage);
        let col = zeroed_buffer(&self.ctx.device, "stream.col", cap * 4, usage);
        let nrm = zeroed_buffer(&self.ctx.device, "stream.nrm", cap * 4, usage);
        if self.stream_count > 0 {
            let mut enc = self.ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&self.stream_pos_buf, 0, &pos, 0, self.stream_count as u64 * 12);
            enc.copy_buffer_to_buffer(&self.stream_col_buf, 0, &col, 0, self.stream_count as u64 * 4);
            enc.copy_buffer_to_buffer(&self.stream_nrm_buf, 0, &nrm, 0, self.stream_count as u64 * 4);
            self.ctx.queue.submit([enc.finish()]);
        }
        // The wire has no normals, and a zeroed buffer is NOT "no normal" - oct code 0 decodes
        // to a real direction. Fill the new region with the sentinel, in 1M-row slabs so the
        // staging cost stays bounded.
        let fill = vec![u32::MAX; 1 << 20];
        let mut at = self.stream_count as u64;
        while at < cap {
            let n = (cap - at).min(1 << 20) as usize;
            self.ctx.queue.write_buffer(&nrm, at * 4, bytemuck::cast_slice(&fill[..n]));
            self.ctx.queue.submit([]);
            at += n as u64;
        }
        self.stream_pos_buf = pos;
        self.stream_col_buf = col;
        self.stream_nrm_buf = nrm;
        self.stream_capacity = cap;
        self.rebuild_splat_groups();
        self.splat_state = None;
    }

    /// A cloud is about to STREAM in. The count is known before a single point has been read -
    /// the protobuf packed-double length prefix gives it - so all three buffers are sized once,
    /// exactly, and every slice afterwards lands at a known offset. No growth mid-cloud.
    pub fn cloud_begin(&mut self, count: u32, instance: u32) {
        self.stream_reserve(self.stream_count as u64 + count as u64);
        self.stream_draws.push(CloudDraw { first: self.stream_count, count, instance, spacing: 0.0, node_first: 0, node_count: 0 });
        self.stream_pos_at = self.stream_count;
        self.stream_col_at = self.stream_count;
        self.stream_count += count;
    }

    /// One slice of positions, straight from the socket into GPU memory. `write_buffer` passes
    /// a subarray VIEW of wasm memory - the slice is the only copy that exists. The FIRST slice
    /// also measures the cloud's point spacing (median consecutive distance - scan order is
    /// surface order), which lesson 41's attenuation needs and a streamed cloud cannot get
    /// from the kernel walk.
    pub fn cloud_pos(&mut self, pos: &[f32]) {
        if let Some(d) = self.stream_draws.last_mut() {
            if d.spacing == 0.0 && self.stream_pos_at == d.first && pos.len() >= 6 {
                let n = (pos.len() / 3).min(2048);
                let mut gaps: Vec<f32> = (1..n).map(|i| {
                    let (a, b) = ((i - 1) * 3, i * 3);
                    ((pos[b] - pos[a]).powi(2) + (pos[b + 1] - pos[a + 1]).powi(2) + (pos[b + 2] - pos[a + 2]).powi(2)).sqrt()
                }).filter(|g| *g > 0.0).collect();
                if !gaps.is_empty() {
                    gaps.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    d.spacing = gaps[gaps.len() / 2];
                }
            }
        }
        self.ctx.queue.write_buffer(&self.stream_pos_buf, self.stream_pos_at as u64 * 12, bytemuck::cast_slice(pos));
        self.stream_pos_at += (pos.len() / 3) as u32;
        // Dawn only recycles its upload staging when a submitted serial completes. Without a
        // flush, 165 MB of write_buffer piles 165 MB of staging on top of the destination.
        self.ctx.queue.submit([]);
        self.splat_state = None; // new points - the splat buffers are stale
    }

    /// The colour run, packed to RGBA8.
    pub fn cloud_col(&mut self, col: &[u32]) {
        self.ctx.queue.write_buffer(&self.stream_col_buf, self.stream_col_at as u64 * 4, bytemuck::cast_slice(col));
        self.stream_col_at += col.len() as u32;
        self.ctx.queue.submit([]);
        self.splat_state = None;
    }

    /// Grow the scene box by a streamed cloud's world-space AABB, so the camera can fit it.
    pub fn grow_scene(&mut self, world: &Aabb) {
        if !world.is_finite() { return }
        // an empty scene starts with a zero box; the first cloud replaces it
        if self.bounds.min[0] >= self.bounds.max[0] {
            self.bounds = *world;
            return;
        }
        self.bounds.union(world);
    }

    /// Reconfigure the surface and recreate the depth + MSAA targets for a new canvas size.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            if let Some(s) = &self.surface { s.configure(&self.ctx.device, &self.config); }
            self.targets = Targets::new(&self.ctx, &self.config, self.targets.samples);
            let pixels = (width * height) as u64 * 4;
            self.splat_depth_buf = zeroed_buffer(&self.ctx.device, "splat.depth", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.splat_color_buf = zeroed_buffer(&self.ctx.device, "splat.color", pixels, wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST);
            self.rebuild_splat_groups();
            self.splat_state = None;

        }
    }

    /// Per-frame uniforms through `FrameUniforms::write`, then the inside-flag refresh, which
    /// reads the eye it solved.
    fn write_frame_uniforms(&mut self, input: &FrameInput) {
        let anchor = self.objects.anchor_f32();
        let cx = FrameCx { view: &self.view, anchor, size: (self.config.width, self.config.height) };
        self.frame.write(&self.ctx, input, &cx);
        self.objects.update_inside(&self.ctx, self.frame.eye, &self.bounds);
    }

    /// Build the record table for one cloud lane. A record folds the cloud's whole per-frame
    /// state: mvp x rebased model as ONE matrix, the tint, the attenuation constant and the
    /// model rotation - so a thread does one mat-vec, no instance fetch.
    /// Attenuated (world-sized) dots, Potree-style: the record carries k such that the
    /// shader's radius is clamp(k * vp_h / clip.w, ...) px - a point covers its own
    /// world-space footprint, so near surfaces close up gap-free and far points shrink.
    /// The manifest px is a size FACTOR on the measured spacing.
    fn splat_records(&self, draws: &[CloudDraw], nodes: &[LodNode]) -> ([u32; 4], Vec<u8>, u32) {
        let mut header = [0u32; 4];
        let mut recs: Vec<u8> = Vec::new();
        let mut cum = 0u32;
        let ortho_h = self.frame.ortho_h as f64;
        let vp_h = self.config.height as f64;
        let aspect = self.config.width as f64 / self.config.height as f64;
        let eye = self.frame.eye;
        for &CloudDraw { first, count, instance: inst, spacing, node_first, node_count } in draws {
            let Some(row) = self.objects.row(inst) else { continue };
            if row.flags & Instance::FLAG_HIDDEN != 0 { continue; }
            let px = if row.spacing > 0.0 { row.spacing } else { 3.0 } * self.view.cloud_size;
            if px <= 0.0 || header[0] >= 256 { continue; }
            // column-major 4x4: combined = mvp x model - one per cloud, shared by every
            // record the cloud emits
            let (a, b) = (&self.frame.mvp_f32, &row.model);
            let mut m = [0.0f32; 16];
            for col in 0..4 {
                for r in 0..4 {
                    m[col * 4 + r] = (0..4).map(|k| a[k * 4 + r] * b[col * 4 + k]).sum();
                }
            }
            // tint.a smuggles the MINIMUM radius (the manifest px, halved): without a
            // floor, attenuation turns distant clouds to dust. With octree LOD a far node
            // carries BIGGER spacing (Potree's answer), but the floor still guards leaves.
            let tint = [row.color[0], row.color[1], row.color[2], (px * 0.5).max(0.5)];
            // spacing is in the cloud's LOCAL units; col0's length is the model scale
            let mscale = ((row.model[0] as f64).powi(2) + (row.model[1] as f64).powi(2) + (row.model[2] as f64).powi(2)).sqrt();
            // one record = one contiguous range at one spacing. world radius = spacing x
            // (px/6); k folds the projection so the shader only divides by clip.w:
            //   perspective: r_px = world_r * cot(fov/2) * (vp_h/2) / w
            //   ortho:       r_px = world_r * vp_h / (2*ortho_h), and w = 1
            let emit = |f: u32, c: u32, sp: f32, recs: &mut Vec<u8>, header: &mut [u32; 4], cum: &mut u32| {
                if header[0] >= 256 { return; }
                recs.extend_from_slice(bytemuck::cast_slice(&m));
                recs.extend_from_slice(bytemuck::cast_slice(&tint));
                let world_r = (sp as f64).max(1.0e-9) * mscale * 0.001 * (px as f64) / 6.0; // metres
                let k = if ortho_h > 0.0 { world_r / (2.0 * ortho_h) }
                        else { world_r * 1.7320508 * 0.5 }; // cot(30 deg) / 2
                recs.extend_from_slice(bytemuck::cast_slice(&[f, c, *cum, (k as f32).to_bits()]));
                // the MODEL rotation columns (translation-free), so a cloud with
                // normals can rotate them into world space for the lambert term
                recs.extend_from_slice(bytemuck::cast_slice(&[
                    b[0], b[1], b[2], 0.0f32,
                    b[4], b[5], b[6], 0.0,
                    b[8], b[9], b[10], 0.0,
                ]));
                header[0] += 1;
                *cum += c;
            };
            if self.view.lod_split_px > 0.0 && node_count > 0 {
                // Octree LOD, Potree-style screen-error selection: every VISITED node
                // contributes its own subsample, and the walk descends while the node's
                // projected point spacing is coarser than the cutoff - far nodes stop at
                // the root (a handful of coarse points), near nodes go deep. Coarse nodes
                // carry big spacing, so attenuation grows their dots to close the gaps.
                let slice = &nodes[node_first as usize..(node_first + node_count) as usize];
                let mut stack: Vec<usize> = vec![0];
                while let Some(ni) = stack.pop() {
                    if header[0] >= 256 { break; }
                    let nd = slice[ni];
                    let c = nd.center;
                    // FRUSTUM CULL on the node's bounding sphere, in clip space through the
                    // folded matrix: an off-screen subtree costs nothing - and without this
                    // a close zoom would visit every node and starve the 256-record table.
                    let r_m = nd.size as f64 * 0.8660254 * mscale * 0.001; // sphere radius, metres
                    let cw = (m[3] * c[0] + m[7] * c[1] + m[11] * c[2] + m[15]) as f64;
                    if ortho_h <= 0.0 && cw < -r_m { continue; } // fully behind the eye
                    let cx = (m[0] * c[0] + m[4] * c[1] + m[8] * c[2] + m[12]) as f64;
                    let cy = (m[1] * c[0] + m[5] * c[1] + m[9] * c[2] + m[13]) as f64;
                    let (ndc_x, ndc_y, ry) = if ortho_h > 0.0 {
                        (cx, cy, r_m / ortho_h)
                    } else {
                        let w = cw.max(1.0e-9);
                        (cx / w, cy / w, r_m * 1.7320508 / w)
                    };
                    if ndc_x.abs() > 1.0 + ry / aspect.min(1.0) || ndc_y.abs() > 1.0 + ry {
                        continue; // the whole subtree is outside the view
                    }
                    // node centre in anchored world units - the eye's space
                    let w = [
                        row.model[0] * c[0] + row.model[4] * c[1] + row.model[8] * c[2] + row.model[12],
                        row.model[1] * c[0] + row.model[5] * c[1] + row.model[9] * c[2] + row.model[13],
                        row.model[2] * c[0] + row.model[6] * c[1] + row.model[10] * c[2] + row.model[14],
                    ];
                    let dist_m = (((w[0] - eye[0]).powi(2) + (w[1] - eye[1]).powi(2) + (w[2] - eye[2]).powi(2)) as f64).sqrt() * 0.001;
                    let sp_m = nd.spacing as f64 * mscale * 0.001;
                    let sp_px = if ortho_h > 0.0 { sp_m * vp_h / (2.0 * ortho_h) }
                                else { sp_m * 1.7320508 * 0.5 * vp_h / dist_m.max(1.0e-9) };
                    let leaf = nd.children.iter().all(|&ch| ch < 0);
                    let refine = !leaf && sp_px > self.view.lod_split_px as f64;
                    // Dot size: a REFINED node's region also receives all its deeper
                    // points, so its own subsample renders at the cloud's measured
                    // spacing - otherwise coarse dots blob over the fine layer under
                    // them. Only the unrefined FRINGE keeps its coarse node spacing
                    // (its points are the only ink there - big dots close the gaps);
                    // a node can never be DENSER than the raw cloud, so the measured
                    // spacing is also the floor there. Leaves hold raw points.
                    let sp = if refine || leaf { spacing } else { nd.spacing.max(spacing) };
                    // `nd.first` is relative to this cloud's own first point
                    emit(first + nd.first, nd.count, sp, &mut recs, &mut header, &mut cum);
                    if refine {
                        for &ch in &nd.children {
                            if ch >= 0 { stack.push(ch as usize); }
                        }
                    }
                }
            } else {
                emit(first, count, spacing, &mut recs, &mut header, &mut cum);
            }
        }
        header[1] = cum;
        (header, recs, cum)
    }

    /// Encode the whole frame into `view`. Returns (draws, objects) for the perf counter.
    /// Knows nothing about a surface, so it works headless.
    pub fn encode_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        color: wgpu::Color,
    ) -> (u32, u32) {
        let mut draws = 0u32;

        // Splat the clouds by COMPUTE before the render pass. One thread per point,
        // twice (depth race, then colour claim); the render pass composites the result
        // with one fullscreen triangle. TWO record sets - the walked lane and the stream
        // lane bind different point buffers - but one pixel buffer pair: atomics compose
        // across dispatches, so both lanes contest the same per-pixel depth race.
        {
            let (header, recs, cum) = self.splat_records(&self.cloud_draws, &self.cloud_nodes);
            let (header_s, recs_s, cum_s) = self.splat_records(&self.stream_draws, &[]);
            self.splat_total = cum + cum_s;
            // Static skip: camera still, same scale, nothing rebuilt - the buffers already
            // hold this exact frame's splats, so the whole compute prelude is free.
            let state = (self.frame.mvp_f32, self.view.cloud_size);
            if self.splat_total > 0 && self.splat_state != Some(state) {
                self.ctx.queue.write_buffer(&self.splat_recs, 0, bytemuck::bytes_of(&header));
                self.ctx.queue.write_buffer(&self.splat_recs, 16, &recs);
                self.ctx.queue.write_buffer(&self.splat_stream_recs, 0, bytemuck::bytes_of(&header_s));
                self.ctx.queue.write_buffer(&self.splat_stream_recs, 16, &recs_s);
                encoder.clear_buffer(&self.splat_depth_buf, 0, None); // 0 bits = reverse-Z far = empty
                encoder.clear_buffer(&self.splat_color_buf, 0, None);
                // 2D grid: a 1D dispatch caps at 65535 workgroups (~4.2M threads) and an
                // oversized dispatch invalidates the WHOLE command buffer - the frame
                // silently never draws. 4096-wide rows cover any point count.
                let grid = |n: u32| { let g = n.div_ceil(64); (g.min(4096), g.div_ceil(4096)) };
                let ((gx, gy), (sx, sy)) = (grid(cum), grid(cum_s));
                let mut cp = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor::default());
                // BOTH lanes' depth races must settle before EITHER lane claims colours -
                // dispatches in one pass are ordered, so lane order inside each phase is free.
                cp.set_pipeline(&self.pipelines.splat_depth);
                if cum > 0 {
                    cp.set_bind_group(0, &self.splat_group0, &[]);
                    cp.set_bind_group(1, &self.splat_group1, &[]);
                    cp.dispatch_workgroups(gx, gy, 1);
                }
                if cum_s > 0 {
                    cp.set_bind_group(0, &self.splat_group0_stream, &[]);
                    cp.set_bind_group(1, &self.splat_group1_stream, &[]);
                    cp.dispatch_workgroups(sx, sy, 1);
                }
                cp.set_pipeline(&self.pipelines.splat_color);
                if cum > 0 {
                    cp.set_bind_group(0, &self.splat_group0, &[]);
                    cp.set_bind_group(1, &self.splat_group1, &[]);
                    cp.dispatch_workgroups(gx, gy, 1);
                }
                if cum_s > 0 {
                    cp.set_bind_group(0, &self.splat_group0_stream, &[]);
                    cp.set_bind_group(1, &self.splat_group1_stream, &[]);
                    cp.dispatch_workgroups(sx, sy, 1);
                }
                self.splat_state = Some(state);
            }
        }

        let b = Binds { mvp: &self.frame.mvp_group, line: &self.frame.line_group, instances: &self.objects.group };
        {
            let mut pass = self.targets.begin_pass(encoder, view, color);

            // Pipelines - sequence of drawing is important:
            // background -> grid -> triangles -> sphere markers -> cylinders -> CLOUD -> ink
            // prepass -> ribbon -> glyph. Everything that WRITES depth comes first (the cloud
            // included, since it went opaque); the flat ink lanes read that depth and never
            // write it. The markers go with the solids so the line ink tests against them -
            // a vertex marker is the topmost ink at its own joint.

            // Background
            pass.set_pipeline(&self.pipelines.background);
            pass.draw(0..3, 0..1);
            draws += 1;

            // Grid first as the depth writes are off, all objects paints over it
            pass.set_pipeline(&self.pipelines.grid);
            pass.set_bind_group(0, b.mvp, &[]);
            pass.set_bind_group(1, b.line, &[]);   // for the anchor
            pass.draw(0..50, 0..1);
            draws += 1;

            draws += self.arena.draw_faces(&mut pass, &self.pipelines, &b);
            draws += self.arena.draw_print(&mut pass, &self.pipelines, &b);
            if self.view.show_mesh_edges {
                draws += self.segments.draw_pipes(&mut pass, &self.pipelines, &b, self.view.line_style);
            }

            // The cloud lane. drawn with the solids: the compute splatter already resovled
            // every cloud into the per-pixel depth/color buffers, so the whoel lane is one fullscreen triangle
            // that composites them - depth-writing via frag_depth, so splat and solids occlude each other exactly.
            if self.splat_total > 0 {
                pass.set_pipeline(&self.pipelines.splat_resolve);
                pass.set_bind_group(0, &self.frame.cloud_group, &[]);
                pass.set_bind_group(1, &self.splat_resolve_group, &[]);
                pass.draw(0..3, 0..1);
                draws += 1;
            }

            // Markers go LAST of the solid lane - see `GlyphLane::draw_spheres`.
            if self.view.show_mesh_edges && self.view.markers {
                draws += self.glyphs.draw_spheres(&mut pass, &self.pipelines, &b);
            }

            // FLAT-lane depth prepass, BOTH tables before either colour pass: blended ink cannot
            // write depth (its AA feather would leave halos), so without this nothing in the flat
            // lane occludes anything else in it and pure draw order wins - a point dot then sits
            // on top of a polyline it is behind, at every camera angle.
            // COST: it draws the whole flat lane a SECOND time. On 2D sheets (600k segments, all
            // ribbons) that doubles the frame - so it is off by default and only worth enabling
            // for 3D scenes where ink-vs-ink order is actually visible.
            if INK_DEPTH_PREPASS && self.view.show_lines {
                draws += self.segments.draw_ribbon_depth(&mut pass, &self.pipelines, &b);
            }
            if INK_DEPTH_PREPASS && self.view.show_points {
                draws += self.glyphs.draw_dot_depth(&mut pass, &self.pipelines, &b);
            }

            if self.view.show_lines {
                draws += self.segments.draw_ribbons(&mut pass, &self.pipelines, &b);
            }

            draws += self.arena.draw_text(&mut pass, &self.pipelines, &b);

            if self.view.show_points {
                draws += self.glyphs.draw_dots(&mut pass, &self.pipelines, &b);
            }
        }

        (draws, self.objects.len())
    }

    /// Forget every family's rows, so the next upload writes from row 0 again. Every lane
    /// appends, so a rebuild has to rewind every lane - leaving one set would append the
    /// re-walked scene BEHIND the copy already there. Capacity stays: a rebuild costs no allocation.
    pub fn reset_arena(&mut self) {
        self.objects.reset();
        self.arena.reset();
        self.segments.reset();
        self.glyphs.reset();
        self.point_pos.reset();
        self.point_col.reset();
        self.point_nrm.reset();
        self.point_count = 0;
        self.cloud_draws.clear();
        self.cloud_nodes.clear();
    }

    /// MSAA sample count for a scene. It cannot be chosen per lane: sample count belongs to the
    /// render PASS, and every pipeline drawn into a pass must match it, so 1x linework and 4x
    /// solids in one frame would need two passes and a depth resolve between them. Pick per scene
    /// instead - hard-edged geometry (triangles, tubes, spheres) is the only thing MSAA smooths,
    /// while ribbons and dots antialias themselves in the shader. A 2D sheet therefore pays
    /// nothing, and a model with meshes gets clean silhouettes.
    /// MSAA follows what is ON THE GPU, not what arrived in the latest upload.
    ///
    /// This used to read `up.verts`/`up.pipes`/`up.spheres`, which was correct while every lane
    /// was cumulative. Now that the arena arrives as a DELTA, an upload carrying only cloud rows
    /// has an empty `up.verts` - so it reported "no solids", flipped 4x back to 1x, and rebuilt
    /// every pipeline and both render targets. In the mixed scene that thrashed 4x -> 1x -> 4x
    /// on every single append.
    fn msaa_now(&self) -> u32 {
        let solid = self.arena.vert_count() > 0 || self.segments.pipe_count() > 0 || self.glyphs.sphere_count() > 0;
        if solid { 4 } else { 1 }
    }
}
```

## Step 12 — `src/app/scene.rs`

The walk writes `t.obj.rows`, `t.arena.verts`, `t.seg.pipes`, `t.glyph.spheres` and so on.

**Find** in `src/app/scene.rs`:

```rust
        let row = self.tables.objects.len() as u32;
        self.tables.objects.push((place.m, [1.0; 4], 0));
        self.tables.object_bounds.push(None);
        self.tables.object_spacing.push(px); // the manifest px rides the spacing row, like the walk's clouds
```

**Replace with:**

```rust
        let row = self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push((place.m, [1.0; 4], 0));
        self.tables.obj.bounds.push(None);
        self.tables.obj.spacing.push(px); // the manifest px rides the spacing row, like the walk's clouds
```

**Find** in `src/app/scene.rs`:

```rust
        self.vert_base += self.tables.verts.len() as u32;
```

**Replace with:**

```rust
        self.vert_base += self.tables.arena.verts.len() as u32;
```

**Find** in `src/app/scene.rs`:

```rust
        let seg0 = self.tables.segments.len();
        let pipe0 = self.tables.pipes.len();
        let vert0 = self.tables.verts.len();
        let vb = self.vert_base; // read before `t` borrows self.tables
        let sphere0 = self.tables.spheres.len();
        let glyph0 = self.tables.glyphs.len();
        let obj0 = self.tables.objects.len();
```

**Replace with:**

```rust
        let seg0 = self.tables.seg.ribbons.len();
        let pipe0 = self.tables.seg.pipes.len();
        let vert0 = self.tables.arena.verts.len();
        let vb = self.vert_base; // read before `t` borrows self.tables
        let sphere0 = self.tables.glyph.spheres.len();
        let glyph0 = self.tables.glyph.dots.len();
        let obj0 = self.tables.obj.rows.len();
```

**Find** in `src/app/scene.rs`:

```rust
            let ri = t.objects.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            let placed = placement(&guid);
            t.objects.push((placed, [1.0; 4], flags));
```

**Replace with:**

```rust
            let ri = t.obj.rows.len() as u32;
            let flags = if self.hidden.contains(&guid) { Instance::FLAG_HIDDEN } else { 0 };
            let placed = placement(&guid);
            t.obj.rows.push((placed, [1.0; 4], flags));
```

**Find** in `src/app/scene.rs`:

```rust
                        if m.name == "text" { &mut t.idx_text } else { &mut t.idx_print }
                    } else {
                        &mut t.idx
```

**Replace with:**

```rust
                        if m.name == "text" { &mut t.arena.idx_text } else { &mut t.arena.idx_print }
                    } else {
                        &mut t.arena.idx
```

**Find** in `src/app/scene.rs`:

```rust
                        &mut t.verts,
                        &mut t.vids,
                        idx_lane,
                        &mut t.pipes,
                        &mut t.spheres
```

**Replace with:**

```rust
                        &mut t.arena.verts,
                        &mut t.arena.vids,
                        idx_lane,
                        &mut t.seg.pipes,
                        &mut t.glyph.spheres
```

**Find** in `src/app/scene.rs`:

```rust
                        // The object row for this guid was pushed just above the match - .2 is flags.
                        t.objects.last_mut().unwrap().2 |= Instance::FLAG_PRINT;
```

**Replace with:**

```rust
                        // The object row for this guid was pushed just above the match - .2 is flags.
                        t.obj.rows.last_mut().unwrap().2 |= Instance::FLAG_PRINT;
```

**Find** in `src/app/scene.rs`:

```rust
                        t.objects.last_mut().unwrap().2 |= Instance::FLAG_OPEN;
                    }
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, m.number_of_vertices()));
```

**Replace with:**

```rust
                        t.obj.rows.last_mut().unwrap().2 |= Instance::FLAG_OPEN;
                    }
                    t.obj.bounds.push(b); t.obj.spacing.push(mesh_spacing(b, m.number_of_vertices()));
```

**Find** in `src/app/scene.rs`:

```rust
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(bb); t.object_spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                }
                Geometry::Line(l) => { t.segments.push(line_to_segment(l, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::Polyline(pl) => { t.segments.extend(polyline_to_segments(pl, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::NurbsCurve(c) => { t.segments.extend(nurbscurve_to_segments(c, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::Point(p) => { t.glyphs.push(point_to_glyph(p, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
```

**Replace with:**

```rust
                        &mut t.arena.verts,
                        &mut t.arena.vids,
                        &mut t.arena.idx,
                        &mut t.seg.pipes,
                        &mut t.glyph.spheres
                    );
                    t.obj.bounds.push(bb); t.obj.spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                }
                Geometry::Line(l) => { t.seg.ribbons.push(line_to_segment(l, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::Polyline(pl) => { t.seg.ribbons.extend(polyline_to_segments(pl, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::NurbsCurve(c) => { t.seg.ribbons.extend(nurbscurve_to_segments(c, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::Point(p) => { t.glyph.dots.push(point_to_glyph(p, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
```

**Find** in `src/app/scene.rs`:

```rust
                    t.object_bounds.push(None);
                    t.object_spacing.push(px);
```

**Replace with:**

```rust
                    t.obj.bounds.push(None);
                    t.obj.spacing.push(px);
```

**Find** in `src/app/scene.rs`:

```rust
                        &mut t.verts,
                        &mut t.vids,
                        &mut t.idx,
                        &mut t.pipes,
                        &mut t.spheres
                    );
                    t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
                Geometry::Plane(p) => { t.segments.extend(plane_to_segments(p, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
                Geometry::OBB(b) => { t.segments.extend(obb_to_segments(b, ri)); t.object_bounds.push(None); t.object_spacing.push(0.0); }
```

**Replace with:**

```rust
                        &mut t.arena.verts,
                        &mut t.arena.vids,
                        &mut t.arena.idx,
                        &mut t.seg.pipes,
                        &mut t.glyph.spheres
                    );
                    t.obj.bounds.push(b); t.obj.spacing.push(mesh_spacing(b, sm.number_of_vertices()));
                }
                Geometry::Plane(p) => { t.seg.ribbons.extend(plane_to_segments(p, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
                Geometry::OBB(b) => { t.seg.ribbons.extend(obb_to_segments(b, ri)); t.obj.bounds.push(None); t.obj.spacing.push(0.0); }
```

**Find** in `src/app/scene.rs`:

```rust
                            if m.name == "text" { &mut t.idx_text } else { &mut t.idx_print }
                        } else {
                            &mut t.idx
```

**Replace with:**

```rust
                            if m.name == "text" { &mut t.arena.idx_text } else { &mut t.arena.idx_print }
                        } else {
                            &mut t.arena.idx
```

**Find** in `src/app/scene.rs`:

```rust
                            &mut t.verts,
                            &mut t.vids,
                            idx_lane,
                            &mut t.pipes,
                            &mut t.spheres
                        );
                        if is_print_fill(&m) {
                            t.objects.last_mut().unwrap().2 |= Instance::FLAG_PRINT;
                        }
                        t.object_bounds.push(b); t.object_spacing.push(mesh_spacing(b, m.number_of_vertices()));
```

**Replace with:**

```rust
                            &mut t.arena.verts,
                            &mut t.arena.vids,
                            idx_lane,
                            &mut t.seg.pipes,
                            &mut t.glyph.spheres
                        );
                        if is_print_fill(&m) {
                            t.obj.rows.last_mut().unwrap().2 |= Instance::FLAG_PRINT;
                        }
                        t.obj.bounds.push(b); t.obj.spacing.push(mesh_spacing(b, m.number_of_vertices()));
```

**Find** in `src/app/scene.rs`:

```rust
                            &mut t.verts,
                            &mut t.vids,
                            &mut t.idx,
                            &mut t.pipes,
                            &mut t.spheres
                        );
                        t.object_bounds.push(bb); t.object_spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                    }
                    ElementGeometry::None => { t.object_bounds.push(None); t.object_spacing.push(0.0); },
```

**Replace with:**

```rust
                            &mut t.arena.verts,
                            &mut t.arena.vids,
                            &mut t.arena.idx,
                            &mut t.seg.pipes,
                            &mut t.glyph.spheres
                        );
                        t.obj.bounds.push(bb); t.obj.spacing.push(mesh_spacing(bb, bm.number_of_vertices()));
                    }
                    ElementGeometry::None => { t.obj.bounds.push(None); t.obj.spacing.push(0.0); },
```

**Find** in `src/app/scene.rs`:

```rust
        for (i, v) in t.verts.iter().enumerate().skip(vert0) {
            if let Some(&ri) = t.vids.get(i) {
                if let Some((xf, _, _)) = t.objects.get(ri as usize) {
```

**Replace with:**

```rust
        for (i, v) in t.arena.verts.iter().enumerate().skip(vert0) {
            if let Some(&ri) = t.arena.vids.get(i) {
                if let Some((xf, _, _)) = t.obj.rows.get(ri as usize) {
```

**Find** in `src/app/scene.rs`:

```rust
        for s in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)){
            if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize){
```

**Replace with:**

```rust
        for s in t.seg.pipes.iter().skip(pipe0).chain(t.seg.ribbons.iter().skip(seg0)){
            if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
```

**Find** in `src/app/scene.rs`:

```rust
        for s in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)){
            if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize){
```

**Replace with:**

```rust
        for s in t.glyph.spheres.iter().skip(sphere0).chain(t.glyph.dots.iter().skip(glyph0)){
            if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
```

**Find** in `src/app/scene.rs`:

```rust
            let Some((xf, _, _)) = t.objects.get(inst as usize) else { continue };
```

**Replace with:**

```rust
            let Some((xf, _, _)) = t.obj.rows.get(inst as usize) else { continue };
```

**Find** in `src/app/scene.rs`:

```rust
            for (i, v) in t.verts.iter().enumerate().skip(vert0){
                if let Some(&ri) = t.vids.get(i){
                    if let Some((xf, _, _)) = t.objects.get(ri as usize) {
```

**Replace with:**

```rust
            for (i, v) in t.arena.verts.iter().enumerate().skip(vert0){
                if let Some(&ri) = t.arena.vids.get(i){
                    if let Some((xf, _, _)) = t.obj.rows.get(ri as usize) {
```

**Find** in `src/app/scene.rs`:

```rust
            for s in t.pipes.iter().skip(pipe0).chain(t.segments.iter().skip(seg0)){
                if let Some((xf, _, _)) = t.objects.get(s.instance_id as usize){
```

**Replace with:**

```rust
            for s in t.seg.pipes.iter().skip(pipe0).chain(t.seg.ribbons.iter().skip(seg0)){
                if let Some((xf, _, _)) = t.obj.rows.get(s.instance_id as usize){
```

**Find** in `src/app/scene.rs`:

```rust
            for g in t.spheres.iter().skip(sphere0).chain(t.glyphs.iter().skip(glyph0)){
                if let Some((xf, _, _)) = t.objects.get(g.instance_id as usize) {
```

**Replace with:**

```rust
            for g in t.glyph.spheres.iter().skip(sphere0).chain(t.glyph.dots.iter().skip(glyph0)){
                if let Some((xf, _, _)) = t.obj.rows.get(g.instance_id as usize) {
```

**Find** in `src/app/scene.rs`:

```rust
            for o in t.objects.iter_mut().skip(obj0) {
                o.2 |= Instance::FLAG_SHEET;
            }
            for s in t.pipes.iter_mut().skip(pipe0).chain(t.segments.iter_mut().skip(seg0)){
```

**Replace with:**

```rust
            for o in t.obj.rows.iter_mut().skip(obj0) {
                o.2 |= Instance::FLAG_SHEET;
            }
            for s in t.seg.pipes.iter_mut().skip(pipe0).chain(t.seg.ribbons.iter_mut().skip(seg0)){
```

## Step 13 — `src/selftest.rs`

The table report reads the grouped columns.

**Find** in `src/selftest.rs`:

```rust
        let (v, i) = (t.verts.len(), t.idx.len());
        let (pipes, sph) = (t.pipes.len(), t.spheres.len());
```

**Replace with:**

```rust
        let (v, i) = (t.arena.verts.len(), t.arena.idx.len());
        let (pipes, sph) = (t.seg.pipes.len(), t.glyph.spheres.len());
```

**Find** in `src/selftest.rs`:

```rust
            t.pipes.len(), t.pipes.len() as f64 * 40.0 / 1.048576e6, t.spheres.len(), t.verts.len());
```

**Replace with:**

```rust
            t.seg.pipes.len(), t.seg.pipes.len() as f64 * 40.0 / 1.048576e6, t.glyph.spheres.len(), t.arena.verts.len());
```

## Step 14 — `examples/check_determinism.rs`

The comparison macro takes a field path.

**Find** in `examples/check_determinism.rs`:

```rust
        macro_rules! same { ($f:ident) => {
            if bytemuck::cast_slice::<_, u8>(&a.tables.$f) != bytemuck::cast_slice::<_, u8>(&b.tables.$f) {
                fails.push(format!("tables.{}", stringify!($f)));
            }
        }; }
        same!(verts); same!(idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Replace with:**

```rust
        macro_rules! same { ($($f:tt).+) => {
            if bytemuck::cast_slice::<_, u8>(&a.tables.$($f).+) != bytemuck::cast_slice::<_, u8>(&b.tables.$($f).+) {
                fails.push(format!("tables.{}", stringify!($($f).+)));
            }
        }; }
        same!(arena.verts); same!(arena.idx); same!(seg.ribbons); same!(seg.pipes); same!(glyph.spheres); same!(glyph.dots);
```

## Check

```bash
cargo check --lib --target wasm32-unknown-unknown            # 2 warnings
cargo check --all-targets --target x86_64-unknown-linux-gnu  # 9 warnings
cargo xtest                                                  # 4 passed
./docs/_gate.sh                                              # gate OK
```

`Gpu` has 39 fields (was 64). The mandatory rows do not move. One advisory row does:
`bunny_drawings VIEWER_REBUILD=1` goes from 41997 to 42107 ink pixels, because the old
`Gpu::reset_arena` cleared `objects_base` but not `base_f32`, so a rebuild's rebase read stale
rotation rows; `InstanceTable::reset` clears all of them.

To see a mirror test bite, rename `extent` to `extent_px` in `glyph.wgsl` and run `cargo xtest`:
`instance_mirror` fails naming `glyph.wgsl`. Put it back.

## Recap

- A family owns its rows, its tables, its bind groups and its draws; every draw returns its count.
- The instance table is the one object table; every other row carries an index into it.
- Four mirror tests keep the Rust rows and the WGSL rows the same shape.

## Next

Lesson [49](49-point-lanes.md) — the point lanes and the frame list: `cloud.rs`, `stream.rs`,
`splat.rs`, `backdrop.rs`, `render.rs`; `Gpu` reaches 17 fields.
