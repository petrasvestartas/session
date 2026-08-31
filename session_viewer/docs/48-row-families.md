# 48 One row, two shaders

> Lesson [56](56-trimmed.md) adds a trimmed surface whose edges are already `CylinderSegment`s,
> [65](65-screen-to-ray.md) picks against those same rows, [111](111-meshlets.md) re-batches them.
> None touches a shader, because after this lesson a row and the programs that read it live in one
> file and the choice between them is one argument.
> Nothing visible changes: same ink, same draw count, same object count, on every scene and config.
> Answer key: `git diff end-of-47..end-of-48 -- session_viewer/src` is this lesson as one patch.
>
> **Lessons 45-51 move code. Every body you cut is pasted byte-identical except for path
> re-roots inside ONE file; if you find yourself improving a line while moving it, stop — the
> deferral list at the end says which lesson owns that change.**

## 1. Why this seam

### 1a. The evidence — run it on your own tree

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE 'pipe_|segment_|sphere_|glyph_|cyl_|sph_'
grep -c 'CylinderSegment' src/engine/gpu/mod.rs
grep -c 'include_str' src/engine/pipelines/mod.rs
grep -c 'build(device, t' src/engine/pipelines/mod.rs
```

```text
63   fields on Gpu
22   of them are the two ink lanes, twice over
 5   mentions of CylinderSegment in one file that also holds the frame
 8   shaders `include_str!`'d by a file that itself draws nothing
12   pipelines built there, of which nine read a row that file has never heard of
```

Twenty-two fields, and they are two copies of one shape, written out twice each:

```text
  cyl_template_vbo  cyl_template_ibo  cyl_index_count      sph_template_vbo  sph_template_ibo  sph_index_count
  pipe_buffer    pipe_bind_group    pipe_count    pipe_cap      sphere_buffer  sphere_bind_group  sphere_count  sphere_cap
  segment_buffer segment_bind_group segment_count segment_cap   glyph_buffer   glyph_bind_group   glyph_count   glyph_cap
```

Now read the `ribbon` and `ribbon_solid` descs in `pipelines/mod.rs` next to each other: the SAME
`.wgsl` compiled twice, differing in one field — `depth_compare` — over two tables of the SAME row
type. The `cylinder` pipeline is a third program over the first of those tables, chosen at run
time by one `match` on `LineStyle`. That is not three families. It is **one row, and a choice**.

`sphere`/`glyph` is the identical shape: one 48-byte `GlyphPoint`, two tables, two programs, two
prepasses — which is why you write that half yourself.

### 1b. The law this enforces, stated as what it forbids

**F5 — a module is defined by the ROW it owns, not by the shader that draws it.** A file owns one
row format, every table of that row, and every pipeline that reads it. It follows that no
`.wgsl` may be named in two files, and no file may name a row it does not own.

Testable: after this lesson every shader a PIPELINE compiles is named in exactly one file, and
`pipelines/mod.rs` names four rather than today's eight. (`frame.rs` and `instance.rs` also
`include_str!` shaders, but only inside `#[cfg(test)]` — reading a shader as text is not owning
it, and the litmus below distinguishes them.)

### 1c. The rejected alternative

The obvious cut is one `InkLane` with a style flag: both rows are ink, both screen-constant, both
take a prepass. Do not make it. `CylinderSegment` is 40 bytes with two endpoints, `GlyphPoint` is
48 with a centre and six face normals; they share a bind-group layout by coincidence, not by
contract, and the moment lesson **107** gives glyphs an atlas index the shared lane becomes a
struct with two dead fields in half its rows. Two files that look alike are cheaper than one file
with a discriminant.

## 2. Where the code lives after this lesson

| symbol | today's home | new home | who may touch it |
|---|---|---|---|
| `CylinderSegment`, `LineStyle`, `CYL_SIDES`, `unit_cylinder` | `gpu/mod.rs` | `gpu/segments.rs` | the walk WRITES rows; only this file draws them |
| `FACING_UNKNOWN` | `app/scene.rs` | `gpu/segments.rs` | the walk reads it; the row format owns it |
| the 11 `pipe_*`/`segment_*`/`cyl_template_*` fields | `Gpu` | `segments::SegmentLane` | `segments.rs` only |
| `ribbon.wgsl`, `cylinder.wgsl` + their 5 descs | `pipelines/mod.rs` | `segments::Pipes` | `segments.rs` only |
| `GlyphPoint`, `unit_quad` | `gpu/mod.rs` | `gpu/glyphs.rs` | as above |
| the 11 `sphere_*`/`glyph_*`/`sph_template_*` fields | `Gpu` | `glyphs::GlyphLane` | `glyphs.rs` only |
| `sphere.wgsl`, `glyph.wgsl` + their 4 descs | `pipelines/mod.rs` | `glyphs::Pipes` | `glyphs.rs` only |
| `Upload.pipes/segments` | `Upload` | `Upload.seg: SegRows` | the walk writes, `SegmentLane::append` reads |
| `Upload.spheres/glyphs` | `Upload` | `Upload.glyph: GlyphRows` | the walk writes, `GlyphLane::append` reads |

```text
                  segments.rs                          glyphs.rs
        +--------------------------+          +--------------------------+
  rows  | SegRows { pipes,ribbons }|          | GlyphRows{spheres,dots}  |
        +--------------------------+          +--------------------------+
  gpu   | SegmentLane              |          | GlyphLane                |
        |   template  (unit_cylinder)         |   template  (unit_quad)  |
        |   pipes   GrowBuf+group  |          |   spheres GrowBuf+group  |
        |   ribbons GrowBuf+group  |          |   dots    GrowBuf+group  |
        +--------------------------+          +--------------------------+
  pipes | cylinder  ribbon         |          | sphere    glyph          |
        | ribbon_solid             |          | sphere_depth glyph_depth |
        | ribbon_depth ribbon_solid_depth     |                          |
        +--------------------------+          +--------------------------+
  draws | draw_solid(style)        |          | draw_markers()           |
        | draw_flat_depth()        |          | draw_dots_depth()        |
        | draw_flat()              |          | draw_dots()              |
        +--------------------------+          +--------------------------+
             ^ &GpuCtx down, draw count up        ^ same contract, same shape
```

**Exit litmus:** `grep -c 'include_str!("../../shaders' src/engine/gpu/*.rs src/engine/pipelines/mod.rs`
gives `arena.rs 1 · segments.rs 2 · glyphs.rs 2 · pipelines/mod.rs 4` — every compiled shader named
once, in the file that owns the row it reads. (`frame.rs 5` and `instance.rs 5` are the
`#[cfg(test)]` mirrors, reading them as text.)

The chain table, extended:

| geometry | walk writes | engine sink | family | shader |
|---|---|---|---|---|
| Mesh/BRep edges | `seg.pipes` | `SegmentLane.pipes` | `segments` | `cylinder.wgsl` **or** `ribbon.wgsl` (`LineStyle`) |
| Line · Polyline · NurbsCurve · Plane · OBB | `seg.ribbons` | `SegmentLane.ribbons` | `segments` | `ribbon.wgsl` |
| Mesh/BRep vertices | `glyph.spheres` | `GlyphLane.spheres` | `glyphs` | `sphere.wgsl` |
| Point | `glyph.dots` | `GlyphLane.dots` | `glyphs` | `glyph.wgsl` |

## 3. Files we touch

| file | what | step | why |
|---|---|---|---|
| `src/engine/gpu/segments.rs` | **NEW**, 338 lines | 4.1 | the row, both tables, five pipelines, three draws |
| `src/engine/gpu/glyphs.rs` | **NEW**, 251 lines | 4.2 | the same shape for point-like ink |
| `src/engine/gpu/buffers.rs` | 132 → 177 | 6.1 | `GrowBuf::append` and `Template`; `append_index_run` goes |
| `src/engine/gpu/upload.rs` | 98 → 100 | 6.2 | four flat columns become two groups |
| `src/engine/pipelines/mod.rs` | 130 → **67** | 6.3 | nine descs and five shader constants leave |
| `src/engine/gpu/mod.rs` | 1,335 → **1,055** | 6.4-6.5 | 22 fields become 2; six draw sites become six calls |
| `src/app/scene.rs` | 1,341 → 1,335 | 6.6 | eight `Replace-all`s, and `FACING_UNKNOWN` goes to the row |
| `src/selftest.rs`, `examples/check_*.rs` | small | 6.6 | the harnesses follow the columns |

## 4. The two destination files, created first

### 4.1 `src/engine/gpu/segments.rs`

Header, imports, and the two shader constants. The file's whole claim is in the header — read it
before you paste it.

**Create `src/engine/gpu/segments.rs`**

```rust
//! `segments.rs` - the segment family: one row, two shaders.
//!
//! Every straight run of ink in the viewer is a `CylinderSegment`: two endpoints, a radius, an
//! `instance_id`, a packed colour and one word of face adjacency. There is ONE row format and
//! TWO tables of it, split by what the ink decorates rather than by what it is:
//!
//! ```text
//!   pipes    mesh / BRep edges    ink that lies ON a surface   -> cylinder.wgsl  or  ribbon.wgsl
//!   ribbons  line / polyline /    ink that floats free         -> ribbon.wgsl
//!            NURBS / plane / OBB
//! ```
//!
//! The split is a DEPTH argument, not a geometry one. A mesh edge lies exactly on the boundary
//! of the two faces that meet there, so it needs either a tube whose radius lifts it off that
//! boundary (`cylinder.wgsl`) or a ribbon that hugs the two face planes it knows about
//! (`ribbon.wgsl`, reading `facing`). A polyline in mid-air has nothing to fight and takes the
//! cheap camera-facing quad. That is why `LineStyle` can flip the solid lane between two shaders
//! at ONE draw site and cost nothing in memory: both shaders read the same forty bytes.
//!
//! Five pipelines over those two tables - `cylinder`, `ribbon`, `ribbon_solid`, and the two
//! depth-only prepasses - and three draws. `glyphs.rs` is the same shape for point-like ink.

use crate::engine::pipelines::layouts::Layouts;
use crate::engine::pipelines::{PipelineDesc, Target, build::{build, cyl_template_layout}};

use super::buffers::{GpuCtx, GrowBuf, Template, mk_rows_group, zeroed_buffer};
use super::frame::Binds;

const RIBBON: &str = include_str!("../../shaders/ribbon.wgsl");
const CYLINDER: &str = include_str!("../../shaders/cylinder.wgsl");
```

Then the constants and the row come over: four Moves and one Remove. `FACING_UNKNOWN` has lived in
`app/scene.rs` since the facing cull was written, but it is a property of the ROW, not of the walk.
It moves.

**Move** `src/engine/gpu/mod.rs` `/// const for the unit_cylinder method` **through** `const CYL_SIDES: u32 = 6;` **to** `src/engine/gpu/segments.rs` **at the end**

**Remove** `src/app/scene.rs`

```rust
/// `facing` value meaning "this edge has no adjacency, always draw it".
```

```rust
pub const FACING_UNKNOWN: u32 = u32::MAX;
```

**Find** in `src/engine/gpu/segments.rs`:

```rust
const CYL_SIDES: u32 = 6;
```

**Add below it:**

```rust

/// `facing` value meaning "this edge has no adjacency, always draw it".
///
/// It cannot be 0: (0, 0) is the honest encoding of +Z. All four corners of the octahedral square
/// collapse onto -Z, so the all-ones word is a value the encoder can produce but never needs, which
/// makes it the one safe sentinel here.
pub const FACING_UNKNOWN: u32 = u32::MAX;
```

**A note on the two Moves below.** Their first line contains a backtick, and the region verb reads
inline `code spans` — so the anchor is written as two fenced blocks, first line and last line,
which the checker treats identically. **Any anchor with a backtick in it must be written this
way.**

**Move** `src/engine/gpu/mod.rs` **to** `src/engine/gpu/segments.rs` **at the end**

```rust
/// How the SOLID lane draws mesh/BRep edges. Both read the SAME `CylinderSegment` table, so
```

```rust
}
```

**Move** `src/engine/gpu/mod.rs` `// Memory layout is 16 (12+4), 16 (12+4) and 16` **through** `}                       // 40 B` **to** `src/engine/gpu/segments.rs` **at the end**

Now the NEW parts: the size assert, the `SegRows` sink, the `Template` helper both families share,
`SegmentLane` with its three draws, and `Pipes` with the five descs.

**Find** in `src/engine/gpu/segments.rs`:

```rust
}                       // 40 B
```

**Add below it:**

```rust

const _: () = assert!(std::mem::size_of::<CylinderSegment>() == 40);

/// The segment group of `Upload`: the two tables the walk fills, by what the ink decorates.
pub struct SegRows {
    /// Mesh/BRep edges - ink that lies ON a surface.
    pub pipes: Vec<CylinderSegment>,
    /// Line/polyline/NURBS/plane/OBB - ink that floats free.
    pub ribbons: Vec<CylinderSegment>,
}

impl SegRows {
    pub fn new() -> Self {
        Self { pipes: Vec::new(), ribbons: Vec::new() }
    }
}

/// The two segment tables on the GPU, their bind groups, and the tube template.
pub struct SegmentLane {
    template: Template,
    pipes: GrowBuf,
    pipes_group: wgpu::BindGroup,
    ribbons: GrowBuf,
    ribbons_group: wgpu::BindGroup,
}

impl SegmentLane {
    pub fn new(device: &wgpu::Device, layouts: &Layouts) -> Self {
        // One storage row per edge (VERTEX-visible, read-only) - the two segment tables. Both
        // start at one row and grow by appending; COPY_SRC lets a grown buffer take the old
        // prefix straight from the old one without a round trip through wasm memory.
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stride = std::mem::size_of::<CylinderSegment>() as u64;
        let pipes = GrowBuf { buf: zeroed_buffer(device, "pipes.buffer", stride, usage), count: 0, cap: 1, usage, label: "pipes.buffer" };
        let ribbons = GrowBuf { buf: zeroed_buffer(device, "segments.buffer", stride, usage), count: 0, cap: 1, usage, label: "segments.buffer" };
        Self {
            // Unit-cylinder template (positions only) - one mesh, instance per edge.
            template: Template::new(device, "cyl.template", &cyl_v, &cyl_i),
            pipes_group: mk_rows_group(device, &layouts.segment, "pipes.bind_group", &pipes.buf),
            ribbons_group: mk_rows_group(device, &layouts.segment, "segments.bind_group", &ribbons.buf),
            pipes,
            ribbons,
        }
    }

    /// Rows on the GPU - a COUNT, not the table.
    pub fn pipes(&self) -> u32 {
        self.pipes.count
    }

    /// Rows on the GPU - a COUNT, not the table.
    pub fn ribbons(&self) -> u32 {
        self.ribbons.count
    }

    /// Append one file's rows to each table. A DELTA like every other lane: only this file's rows
    /// travel, and the bind group is rebuilt only when the buffer behind it actually grew.
    pub fn append(&mut self, ctx: &GpuCtx, layouts: &Layouts, up: &SegRows) {
        if self.pipes.append(ctx, &up.pipes) {
            self.pipes_group = mk_rows_group(&ctx.device, &layouts.segment, "pipes.bind_group", &self.pipes.buf);
        }
        if self.ribbons.append(ctx, &up.ribbons) {
            self.ribbons_group = mk_rows_group(&ctx.device, &layouts.segment, "segments.bind_group", &self.ribbons.buf);
        }
    }

    /// Rewind both tables. Capacity stays, so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.pipes.count = 0;
        self.ribbons.count = 0;
    }

    /// The SOLID lane: mesh/BRep edges, in whichever of the two shaders `View` is asking for.
    /// Returns the draws it issued - `Flat` costs two, because its colour pass writes no depth.
    pub fn draw_solid(&self, pass: &mut wgpu::RenderPass, b: &Binds, style: LineStyle) -> u32 {
        if self.pipes.count == 0 {
            return 0;
        }
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.pipes_group, &[]);
        match style {
            LineStyle::Tubes => {
                pass.set_pipeline(&b.p.seg.cylinder);
                self.template.draw(pass, self.pipes.count); // one template, N edges
                1
            }
            // The flat lane's own shader over the SOLID table. DEPTH PREPASS
            // first (binary at half coverage): the blended colour pass writes no depth,
            // so its AA feather can never depth-reject a later stroke's opaque core -
            // that rejection read as pale flecks inside the bunny's wireframe.
            LineStyle::Flat => {
                pass.set_pipeline(&b.p.seg.ribbon_solid_depth);
                pass.draw(0..4, 0..self.pipes.count);
                pass.set_pipeline(&b.p.seg.ribbon_solid);
                pass.draw(0..4, 0..self.pipes.count);
                2
            }
        }
    }

    /// The FLAT lane's depth prepass. Off by default - see `INK_DEPTH_PREPASS` at the call site.
    pub fn draw_flat_depth(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.ribbons.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.seg.ribbon_depth);
        self.bind_ribbons(pass, b);
        pass.draw(0..4, 0..self.ribbons.count);
        1
    }

    /// The FLAT lane: line/polyline ink, camera-facing ribbons, screen-constant and cheap.
    pub fn draw_flat(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.ribbons.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.seg.ribbon);
        self.bind_ribbons(pass, b);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.count);
        1
    }

    fn bind_ribbons(&self, pass: &mut wgpu::RenderPass, b: &Binds) {
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.ribbons_group, &[]);
    }
}

/// The family's five pipelines: two programs over one row, plus the two depth-only prepasses and
/// the tube.
pub struct Pipes {
    pub cylinder: wgpu::RenderPipeline,
    pub ribbon: wgpu::RenderPipeline,
    pub ribbon_solid: wgpu::RenderPipeline,
    pub ribbon_depth: wgpu::RenderPipeline, // depth-only prepass, so flat ink occludes flat ink
    // Depth-only prepass for the SOLID flat lane: binary at half coverage, so the blended colour
    // pass never writes depth and the AA feather cannot leave pale flecks by depth-rejecting a
    // later stroke's opaque core.
    pub ribbon_solid_depth: wgpu::RenderPipeline,
}

impl Pipes {
    pub fn descs(device: &wgpu::Device, t: Target, l: &Layouts) -> Self {
        Self {
            // Linework tubes: one unit-cylinder template instanced per segment. Solid, so it
            // occludes correctly and needs no bias at all.
            cylinder: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()], // slot 0 - the unit-cylinder positions
                ..PipelineDesc::opaque("cylinder", CYLINDER, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // Flat capsule ribbons: buffer-less, 4 verts per quad, one instance per segment.
            ribbon: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..PipelineDesc::ink("ribbon", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // The SAME shader aimed at the SOLID lane (mesh/BRep edges). GreaterEqual is
            // load-bearing here: a mesh edge lies EXACTLY on the boundary of the two faces that
            // meet there, so strict Greater discards the line and float precision decides which
            // pixels survive - the edge reads offset, ragged and asymmetric along its length.
            ribbon_solid: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                depth_compare: if std::env::var("VIEWER_NO_DEPTH").is_ok() { wgpu::CompareFunction::Always } else { wgpu::CompareFunction::GreaterEqual },
                ..PipelineDesc::ink("ribbon.solid", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // The depth-only prepasses. `fs_depth` is binary at half coverage, so the blended
            // colour passes above never write depth and the AA feather cannot leave pale flecks
            // by depth-rejecting a later stroke's opaque core. Without them, ink never writes
            // depth and draw order alone decides who wins - and draw order here is HashMap order,
            // so "who is in front" was effectively random.
            ribbon_depth: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..PipelineDesc::depth_only("ribbon.depth", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            ribbon_solid_depth: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..PipelineDesc::depth_only("ribbon.solid.depth", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
        }
    }
}
```

And the template mesh itself, which nothing outside this file has ever needed.

**Move** `src/engine/gpu/mod.rs` `/// Unit-cylinder template mesh (positions + indices) along +Z, radius 1, z in [0,1], with cap fans.` **through** `}` **to** `src/engine/gpu/segments.rs` **at the end**

**Find** in `src/engine/gpu/segments.rs`:

```rust
/// const for the unit_cylinder method
```

**Replace with:**

```rust
/// Sides on the unit-cylinder template. Six, because at the pen widths this viewer draws a tube
/// covers two or three pixels across, where a hexagon and a circle resolve to the same pixels -
/// and every extra side is twelve more triangles on the biggest instanced draw in the frame.
```

**Find** in `src/engine/gpu/segments.rs`:

```rust
// Memory layout is 16 (12+4), 16 (12+4) and 16
```

**Replace with:**

```rust
// Memory layout is 16 (12+4), 16 (12+4) and 16
//
// The fields are `pub`, not `pub(crate)` like `Instance`'s, and that is not an oversight:
// `examples/check_lean.rs` dumps a differing row field by field when the determinism harness
// finds one, and an example is a separate crate.
```

**Find** in `src/engine/gpu/segments.rs`:

```rust
    pub radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 0 -> wolrd mm override
```

**Replace with:**

```rust
    pub radius: f32,    // 4 B - 0.0 to screen-constant px (default); > 0 -> world mm override
```

Last, the mirror test this family has been missing: `CylinderSegment` is declared twice more, in
`cylinder.wgsl` and `ribbon.wgsl`, with only a size assert between them until now.

**Find** in `src/engine/gpu/segments.rs`:

```rust
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]); // bottom + top fan
    }
    (v, idx)
}
```

**Replace with:**

```rust
        idx.extend_from_slice(&[cb, b1, b0, ct, b0 + 1, b1 + 1]); // bottom + top fan
    }
    (v, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// The two shaders that declare `struct CylinderSegment`. Both read the SAME table, which is
    /// the whole argument of this file - so both have to agree with it, and with each other.
    const MIRRORS: [(&str, &str); 2] = [("cylinder.wgsl", CYLINDER), ("ribbon.wgsl", RIBBON)];

    /// The ends are three SCALARS on both sides, not a `vec3<f32>`. That is the one thing this
    /// test exists to hold: WGSL aligns a `vec3<f32>` to 16, so writing the obvious thing on the
    /// shader side takes the stride from 40 to 48 and every row after the first is misread - at
    /// the right size, in the wrong place, with no error anywhere.
    #[test]
    fn cylinder_segment_mirror() {
        assert_eq!(std::mem::size_of::<CylinderSegment>(), 40, "the storage array stride is 40 B");
        assert_eq!(std::mem::offset_of!(CylinderSegment, radius), 12);
        assert_eq!(std::mem::offset_of!(CylinderSegment, p1), 16);
        assert_eq!(std::mem::offset_of!(CylinderSegment, instance_id), 28);
        assert_eq!(std::mem::offset_of!(CylinderSegment, color), 32);
        assert_eq!(std::mem::offset_of!(CylinderSegment, facing), 36);

        let want: Vec<(String, String)> = [
            ("p0x", "f32"), ("p0y", "f32"), ("p0z", "f32"), ("radius", "f32"),
            ("p1x", "f32"), ("p1y", "f32"), ("p1z", "f32"), ("instance_id", "u32"),
            ("color", "u32"), ("facing", "u32"),
        ]
        .iter()
        .map(|(f, t)| (f.to_string(), t.to_string()))
        .collect();

        for (file, src) in MIRRORS {
            assert_eq!(
                wgsl_fields(src, "CylinderSegment"), want,
                "{file} declares `CylinderSegment` differently from segments.rs",
            );
        }
    }
}
```

**Gate.** `cargo check --target wasm32-unknown-unknown --lib` — errors, because `gpu/mod.rs` still
declares the types you just took. Expected until 6.4; all of them should be `cannot find type`, and
none inside `segments.rs`.

### 4.2 `src/engine/gpu/glyphs.rs`

Same shape, same order. The header says what differs: the flat half draws without a template.

**Create `src/engine/gpu/glyphs.rs`**

```rust
//! `glyphs.rs` - the point family: one row, two shaders, the same shape as `segments.rs`.
//!
//! Every point-like mark in the viewer is a `GlyphPoint`: a centre, a radius, a colour, an
//! `instance_id`, and up to six incident face normals. One row format, two tables, split by the
//! same depth argument the segment family is split by:
//!
//! ```text
//!   spheres  mesh / BRep vertices   marks that sit ON a surface   -> sphere.wgsl
//!   dots     Point geometry         marks that float free         -> glyph.wgsl
//! ```
//!
//! `sphere.wgsl` expands a camera-facing quad template and trims it to a disc; `glyph.wgsl`
//! builds its own triangle per dot and needs no template at all. Both read the same forty-eight
//! bytes through the same layout, which is the family contract stated once more: a row is a row,
//! and the shader that reads it is a choice made at the draw site.
//!
//! Four pipelines and four draws. Read `segments.rs` first - this file is its mirror, and the
//! only structural difference is that the flat half here draws without a template.

use crate::engine::pipelines::layouts::Layouts;
use crate::engine::pipelines::{PipelineDesc, Target, build::{build, cyl_template_layout}};

use super::buffers::{GpuCtx, GrowBuf, Template, mk_rows_group, zeroed_buffer};
use super::frame::Binds;

const SPHERE: &str = include_str!("../../shaders/sphere.wgsl");
const GLYPH: &str = include_str!("../../shaders/glyph.wgsl");
```

**Move** `src/engine/gpu/mod.rs` `// One instance of the unit-sphere template.` **through** `} // 48 B total, three 16-byte rows` **to** `src/engine/gpu/glyphs.rs` **at the end**

**Move** `src/engine/gpu/mod.rs` `// The WGSL GlyphPoint (glyph.wgsl AND sphere.wgsl - same table) is exactly this layout; the` **through** `const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);` **to** `src/engine/gpu/glyphs.rs` **at the end**

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
const _: () = assert!(std::mem::size_of::<GlyphPoint>() == 48);
```

**Add below it:**

```rust

/// The glyph group of `Upload`: the two tables the walk fills, by what the mark decorates.
pub struct GlyphRows {
    /// Mesh/BRep vertices - markers that sit ON a surface, radius matched to the pipes.
    pub spheres: Vec<GlyphPoint>,
    /// Point geometry - flat SDF dots.
    pub dots: Vec<GlyphPoint>,
}

impl GlyphRows {
    pub fn new() -> Self {
        Self { spheres: Vec::new(), dots: Vec::new() }
    }
}

/// The two glyph tables on the GPU, their bind groups, and the marker template.
pub struct GlyphLane {
    template: Template,
    spheres: GrowBuf,
    spheres_group: wgpu::BindGroup,
    dots: GrowBuf,
    dots_group: wgpu::BindGroup,
}

impl GlyphLane {
    pub fn new(device: &wgpu::Device, layouts: &Layouts) -> Self {
        let (quad_v, quad_i) = unit_quad();
        let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let stride = std::mem::size_of::<GlyphPoint>() as u64;
        let spheres = GrowBuf { buf: zeroed_buffer(device, "spheres.buffer", stride, usage), count: 0, cap: 1, usage, label: "spheres.buffer" };
        let dots = GrowBuf { buf: zeroed_buffer(device, "glyphs.buffer", stride, usage), count: 0, cap: 1, usage, label: "glyphs.buffer" };
        Self {
            // Camera-facing quad template (positions-only) - one mesh, instance per marker
            template: Template::new(device, "sph.template", &quad_v, &quad_i),
            spheres_group: mk_rows_group(device, &layouts.glyph, "spheres.bind_group", &spheres.buf),
            dots_group: mk_rows_group(device, &layouts.glyph, "glyphs.bind_group", &dots.buf),
            spheres,
            dots,
        }
    }

    /// Rows on the GPU - a COUNT, not the table.
    pub fn spheres(&self) -> u32 {
        self.spheres.count
    }

    /// Rows on the GPU - a COUNT, not the table.
    pub fn dots(&self) -> u32 {
        self.dots.count
    }

    /// Append one file's rows to each table. A DELTA like every other lane: only this file's
    /// rows travel, and a bind group is rebuilt only when the buffer behind it actually grew.
    pub fn append(&mut self, ctx: &GpuCtx, layouts: &Layouts, up: &GlyphRows) {
        if self.spheres.append(ctx, &up.spheres) {
            self.spheres_group = mk_rows_group(&ctx.device, &layouts.glyph, "spheres.bind_group", &self.spheres.buf);
        }
        if self.dots.append(ctx, &up.dots) {
            self.dots_group = mk_rows_group(&ctx.device, &layouts.glyph, "glyphs.bind_group", &self.dots.buf);
        }
    }

    /// Rewind both tables. Capacity stays, so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.spheres.count = 0;
        self.dots.count = 0;
    }

    /// Vertex markers, the solid half. Two draws: the prepass then the disc - the same split the
    /// solid ribbons take, and for the same reason.
    pub fn draw_markers(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.spheres.count == 0 {
            return 0;
        }
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.spheres_group, &[]);
        // Same prepass split as the solid ribbons - see LineStyle::Flat in segments.rs.
        pass.set_pipeline(&b.p.glyphs.sphere_depth);
        self.template.draw(pass, self.spheres.count);
        pass.set_pipeline(&b.p.glyphs.sphere);
        self.template.draw(pass, self.spheres.count); // one template, N glyphs
        2
    }

    /// The flat half's depth prepass. Off by default - see `INK_DEPTH_PREPASS` at the call site.
    pub fn draw_dots_depth(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.dots.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.glyphs.glyph_depth);
        self.bind_dots(pass, b);
        pass.draw(0..3 * self.dots.count, 0..1);
        1
    }

    /// Flat SDF dots. No template: three vertices per dot, built in the shader.
    pub fn draw_dots(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.dots.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.glyphs.glyph);
        self.bind_dots(pass, b);
        pass.draw(0..3 * self.dots.count, 0..1); // 3 verts/dot, no template
        1
    }

    fn bind_dots(&self, pass: &mut wgpu::RenderPass, b: &Binds) {
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.line, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        pass.set_bind_group(3, &self.dots_group, &[]);
    }
}

/// The family's four pipelines: two programs over one row, each with its depth-only prepass.
pub struct Pipes {
    pub sphere: wgpu::RenderPipeline,
    pub glyph: wgpu::RenderPipeline,
    /// The only ink-depth pipeline with a vertex buffer: the marker prepass runs the same quad
    /// template its colour pass does.
    pub sphere_depth: wgpu::RenderPipeline,
    pub glyph_depth: wgpu::RenderPipeline,
}

impl Pipes {
    pub fn descs(device: &wgpu::Device, t: Target, l: &Layouts) -> Self {
        Self {
            // A camera-facing quad template instanced per marker, trimmed to a circle by the
            // fragment SDF. Its depth comes from the `sphere_depth` prepass; GreaterEqual lets a
            // marker drawn AFTER a band still keep the rim the band's cap overlaps.
            sphere: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()], // reused - position only, stride 12
                depth_compare: if std::env::var("VIEWER_NO_DEPTH").is_ok() { wgpu::CompareFunction::Always } else { wgpu::CompareFunction::GreaterEqual },
                ..PipelineDesc::ink("sphere", SPHERE, &[&l.mvp, &l.line, &l.instance, &l.glyph])
            }),
            // Group 3 is `l.glyph`, matching `dots_group` and `glyph_depth` below. It read
            // `l.segment` for a long time - the old builder's parameter was named `glyph_layout`
            // and was handed the segment one - and worked only because the two layouts are
            // byte-identical, so wgpu deduplicates them. Named honestly now.
            glyph: build(device, t, &PipelineDesc::ink("glyph", GLYPH, &[&l.mvp, &l.line, &l.instance, &l.glyph])),
            sphere_depth: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()],
                ..PipelineDesc::depth_only("sphere.depth", SPHERE, &[&l.mvp, &l.line, &l.instance, &l.glyph])
            }),
            glyph_depth: build(device, t, &PipelineDesc::depth_only("glyph.depth", GLYPH, &[&l.mvp, &l.line, &l.instance, &l.glyph])),
        }
    }
}
```

**Move** `src/engine/gpu/mod.rs` `/// Camera-facing quad template (positions + indices) for the instanced vertex markers. The` **through** `}` **to** `src/engine/gpu/glyphs.rs` **at the end**

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
// One instance of the unit-sphere template.
```

**Replace with:**

```rust
// One instance of the camera-facing quad template, trimmed to a disc in the fragment shader.
```

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    pub radius: f32, // 4 B - 0.0 - screen-constant px; 0 - world mm
```

**Replace with:**

```rust
    pub radius: f32, // 4 B - 0.0 = screen-constant px (default); > 0 = world mm override
```

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    pub instance_id: u32, // 4 B - row insntaces
```

**Replace with:**

```rust
    pub instance_id: u32, // 4 B - row in instances[]
```

**Find** in `src/engine/gpu/glyphs.rs`:

```rust
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}
```

**Replace with:**

```rust
    let idx = vec![0u32, 1, 2, 0, 2, 3];
    (v, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// The two shaders that declare `struct GlyphPoint` — one per half of the family.
    const MIRRORS: [(&str, &str); 2] = [("sphere.wgsl", SPHERE), ("glyph.wgsl", GLYPH)];

    /// Unlike `CylinderSegment`, this row CAN carry `vec3`/`vec4` on the shader side: `center`
    /// sits at offset 0 and `color` at 16, both already 16-aligned, so WGSL's rules and Rust's
    /// agree by luck rather than by design. The test is what turns that luck into a contract.
    #[test]
    fn glyph_point_mirror() {
        assert_eq!(std::mem::size_of::<GlyphPoint>(), 48, "three 16-byte rows");
        assert_eq!(std::mem::offset_of!(GlyphPoint, radius), 12);
        assert_eq!(std::mem::offset_of!(GlyphPoint, color), 16);
        assert_eq!(std::mem::offset_of!(GlyphPoint, instance_id), 32);
        assert_eq!(std::mem::offset_of!(GlyphPoint, facing), 36);
        assert_eq!(std::mem::offset_of!(GlyphPoint, facing_ext), 40);

        let want: Vec<(String, String)> = [
            ("center", "vec3<f32>"), ("radius", "f32"), ("color", "vec4<f32>"),
            ("instance_id", "u32"), ("facing", "u32"), ("facing_ext", "vec2<u32>"),
        ]
        .iter()
        .map(|(f, t)| (f.to_string(), t.to_string()))
        .collect();

        for (file, src) in MIRRORS {
            assert_eq!(
                wgsl_fields(src, "GlyphPoint"), want,
                "{file} declares `GlyphPoint` differently from glyphs.rs",
            );
        }
    }
}
```

**Gate.** `wc -l src/engine/gpu/segments.rs src/engine/gpu/glyphs.rs` — **338** and **251**. A
count far off means a paste went wrong.

### 4.3 The two banners `gpu/mod.rs` no longer needs

Both "Individual type memory layouts" and "Primitives" are empty now.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        if solid { 4 } else { 1 }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////
/// Individual type memory layouts
//////////////////////////////////////////////////////////////////////////////////////////////////




//////////////////////////////////////////////////////////////////////////////////////////////////
/// Primitives
//////////////////////////////////////////////////////////////////////////////////////////////////



```

**Replace with:**

```rust
        if solid { 4 } else { 1 }
    }
}
```

## 5. Where the borrow checker bites — B2 again, and why the draws return a count

> A draw method cannot take `&mut Gpu` and a `&Binds` built from `Gpu` at the same time, and it
> cannot increment `Gpu`'s `draws` counter from inside itself either:
>
> ```rust
> pub fn draw_flat(&self, pass: &mut wgpu::RenderPass, b: &Binds, gpu: &mut Gpu) { .. }
> //                                                              ^^^^^^^^^^^^ E0502
> ```
>
> `self` is already a borrow of `gpu`. The fix is the contract every draw in this block uses:
> **a draw returns the number of draws it issued, and the caller adds it up.** `draws +=
> self.seg.draw_flat(&mut pass, &b);` — no shared counter, and the number is visible at the call
> site where the frame's order is read.

## 6. The steps

### 6.1 `buffers.rs` — a table appends to itself

`GrowBuf` has held the buffer, the count and the cap since lesson 46, but callers still passed all
three by hand. Four tables in two new files is enough evidence for the method.

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// Grow-and-append one index run. Same shape as the solid arena's own append: the existing
/// prefix is copied GPU-side, never back through wasm memory.
/// Append rows to a growable STORAGE buffer
```

**Replace with:**

```rust
impl GrowBuf {
    /// Append rows to this table, growing it if they do not fit. Returns `true` when the buffer
    /// was replaced, so the caller knows to rebuild the bind group pointing at it.
    ///
    /// Growth RE-CREATES the buffer with THIS table's `usage`, which is the whole reason that
    /// field is carried: the arena's index runs are INDEX buffers and the ink tables are STORAGE
    /// ones, and an index run re-made as STORAGE fails validation at the next `set_index_buffer`.
    pub fn append<T: bytemuck::Pod>(&mut self, ctx: &GpuCtx, data: &[T]) -> bool {
        if data.is_empty() {
            return false;
        }
        let stride = std::mem::size_of::<T>() as u64;
        let need = self.count as u64 + data.len() as u64;
        let mut grew = false;
        if need > self.cap {
            let new_cap = need.max(self.cap * 2);
            let nb = zeroed_buffer(&ctx.device, self.label, new_cap * stride, self.usage);
            if self.count > 0 {
                // the prefix moves GPU-side; it never travels back through wasm memory
                let mut enc = ctx.device.create_command_encoder(&Default::default());
                enc.copy_buffer_to_buffer(&self.buf, 0, &nb, 0, self.count as u64 * stride);
                ctx.queue.submit([enc.finish()]);
            }
            self.buf = nb;
            self.cap = new_cap;
            grew = true;
        }
        ctx.queue.write_buffer(&self.buf, self.count as u64 * stride, bytemuck::cast_slice(data));
        self.count += data.len() as u32;
        grew
    }
}

/// Append rows to a growable STORAGE buffer
```

`GrowBuf::append` is the real implementation, not a wrapper: growth RE-CREATES the buffer, with
**this table's** `usage`. Delegating to `append_rows` would hard-code `STORAGE`, and the arena's
three index runs are `INDEX` buffers — a grown index run re-made as storage fails validation at the
next `set_index_buffer`. `append_index_run` is now that method with one type fixed, so it goes.

**Remove** `src/engine/gpu/buffers.rs` `pub fn append_index_run(ctx: &GpuCtx, run: &mut GrowBuf, data: &[u32]) {` **through** `}`

**Find** in `src/engine/gpu/buffers.rs`:

```rust
    grew
}


/// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
```

**Replace with:**

```rust
    grew
}

/// One read-only storage buffer at binding 0 - the shape every ink lane's bind group has.
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// cloud lanes in `mod.rs` are still a loose (buffer, count, cap) triple rather than a `GrowBuf`.
```

**Replace with:**

```rust
/// cloud lanes in `mod.rs` are still a loose (buffer, count, cap) triple rather than a `GrowBuf`.
/// Anything that HAS a `GrowBuf` calls `GrowBuf::append` above instead.
```

**Find** in `src/engine/gpu/arena.rs`:

```rust
use super::buffers::{GpuCtx, GrowBuf, append_index_run, zeroed_buffer};
```

**Replace with:**

```rust
use super::buffers::{GpuCtx, GrowBuf, zeroed_buffer};
```

**Find** in `src/engine/gpu/arena.rs`:

```rust
        append_index_run(ctx, self.run_mut(IdxLane::Print), &up.idx_print);
        append_index_run(ctx, self.run_mut(IdxLane::Text), &up.idx_text);
```

**Replace with:**

```rust
        self.run_mut(IdxLane::Print).append(ctx, &up.idx_print);
        self.run_mut(IdxLane::Text).append(ctx, &up.idx_text);
```

Last for this file, `Template` — a positions-only mesh uploaded once and instanced per row. Both
ink families need one and neither owns it, which is what the floor is for.

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// A fresh buffer of `size` bytes,
```

**Add above it:**

```rust
/// A template mesh instanced once per row: positions only, uploaded once at startup.
pub struct Template {
    vbo: wgpu::Buffer,
    ibo: wgpu::Buffer,
    count: u32,
}

impl Template {
    pub fn new(device: &wgpu::Device, label: &str, verts: &[[f32; 3]], idx: &[u32]) -> Self {
        use wgpu::util::DeviceExt;
        Self {
            count: idx.len() as u32,
            vbo: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}.vbo")),
                contents: bytemuck::cast_slice(verts),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            ibo: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{label}.ibo")),
                contents: bytemuck::cast_slice(idx),
                usage: wgpu::BufferUsages::INDEX,
            }),
        }
    }

    /// Bind the template and draw `n` instances of it.
    pub fn draw(&self, pass: &mut wgpu::RenderPass, n: u32) {
        pass.set_vertex_buffer(0, self.vbo.slice(..));
        pass.set_index_buffer(self.ibo.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.count, 0, 0..n);
    }
}

```

### 6.2 `Upload` — four more columns become two groups

**Find** in `src/engine/gpu/upload.rs`:

```rust
use super::arena::ArenaRows;
use super::objects::ObjectRows;
use super::{CloudDraw, CylinderSegment, GlyphPoint, LodNode};
```

**Replace with:**

```rust
use super::arena::ArenaRows;
use super::glyphs::GlyphRows;
use super::objects::ObjectRows;
use super::segments::SegRows;
use super::{CloudDraw, LodNode};
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
    pub pipes: Vec<CylinderSegment>, // Solid lane: Mesh/Brep edges, drawn as 3D cylinders
    pub spheres: Vec<GlyphPoint>, // Solid lane: Mesh/Brep vertices, radius matched to the pipes
    pub segments: Vec<CylinderSegment>, // Flat lane: line/polyline, drawn as camera-facing ribbons
    pub glyphs: Vec<GlyphPoint>, // Flat lane: points, draw as SDF dots,
```

**Replace with:**

```rust
    /// The segment family's two tables: mesh/BRep edges and free-standing linework
    /// (`segments.rs`). One row format, two shaders.
    pub seg: SegRows,
    /// The glyph family's two tables: mesh/BRep vertex markers and flat dots (`glyphs.rs`).
    pub glyph: GlyphRows,
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
            pipes: Vec::new(),
            spheres: Vec::new(),
            segments: Vec::new(),
            glyphs: Vec::new(),
```

**Replace with:**

```rust
            seg: SegRows::new(),
            glyph: GlyphRows::new(),
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
        drop_rows(&mut self.pipes);
        drop_rows(&mut self.segments);
        drop_rows(&mut self.spheres);
        drop_rows(&mut self.glyphs);
```

**Replace with:**

```rust
        drop_rows(&mut self.seg.pipes);
        drop_rows(&mut self.seg.ribbons);
        drop_rows(&mut self.glyph.spheres);
        drop_rows(&mut self.glyph.dots);
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
//! `cloud` at 49 - so a producer can be handed the two columns it may write, not all nineteen.
```

**Replace with:**

```rust
//! `cloud` at 49 - so a producer can be handed the two columns it may write, not all nineteen.
//! After 48 the flat count is nine, in four groups.
```

### 6.3 `pipelines/mod.rs` — nine descs and five shaders leave

What is left is the LIST plus the three pipelines that belong to no row — grid, background, splat
resolve. The file goes from 130 lines to 67.

**Find** in `src/engine/pipelines/mod.rs`:

```rust
use build::{build, build_compute, cyl_template_layout};
use crate::engine::gpu::arena;
```

**Replace with:**

```rust
use build::{build, build_compute};
use crate::engine::gpu::{arena, glyphs, segments};
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
const RIBBON: &str = include_str!("../../shaders/ribbon.wgsl");
const GLYPH: &str = include_str!("../../shaders/glyph.wgsl");
const SPHERE: &str = include_str!("../../shaders/sphere.wgsl");
const GRID: &str = include_str!("../../shaders/grid.wgsl");
const CYLINDER: &str = include_str!("../../shaders/cylinder.wgsl");
```

**Replace with:**

```rust
const GRID: &str = include_str!("../../shaders/grid.wgsl");
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub arena: arena::Pipes,
    pub grid: wgpu::RenderPipeline,
    pub cylinder: wgpu::RenderPipeline,
    pub sphere: wgpu::RenderPipeline,
    pub ribbon: wgpu::RenderPipeline,
    pub ribbon_solid: wgpu::RenderPipeline,
    pub glyph: wgpu::RenderPipeline,
    pub ribbon_depth: wgpu::RenderPipeline, // depth-only prepass, so flat ink occludes flat ink
    pub glyph_depth: wgpu::RenderPipeline,
    // Depth-only prepasses for the SOLID flat lane (mesh/BRep edge ribbons + vertex markers):
    // binary at half coverage, so the blended colour passes never write depth and the AA
    // feather cannot leave pale flecks by depth-rejecting a later stroke's opaque core.
    pub ribbon_solid_depth: wgpu::RenderPipeline,
    pub sphere_depth: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
```

**Replace with:**

```rust
    pub arena: arena::Pipes,
    /// The segment family's five - two programs over one row, plus two prepasses and the tube.
    pub seg: segments::Pipes,
    /// The glyph family's four.
    pub glyphs: glyphs::Pipes,
    pub grid: wgpu::RenderPipeline,
    pub background: wgpu::RenderPipeline,
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            // Linework tubes: one unit-cylinder template instanced per segment. Solid, so it
            // occludes correctly and needs no bias at all.
            cylinder: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()], // slot 0 - the unit-cylinder positions
                ..PipelineDesc::opaque("cylinder", CYLINDER, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
```

**Replace with:**

```rust
            seg: segments::Pipes::descs(device, t, l),
            glyphs: glyphs::Pipes::descs(device, t, l),
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            // A camera-facing quad template instanced per marker, trimmed to a circle by the
            // fragment SDF. Its depth comes from the `sphere_depth` prepass; GreaterEqual lets a
            // marker drawn AFTER a band still keep the rim the band's cap overlaps.
            sphere: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()], // reused - position only, stride 12
                depth_compare: if std::env::var("VIEWER_NO_DEPTH").is_ok() { wgpu::CompareFunction::Always } else { wgpu::CompareFunction::GreaterEqual },
                ..PipelineDesc::ink("sphere", SPHERE, &[&l.mvp, &l.line, &l.instance, &l.glyph])
            }),
            // Flat capsule ribbons: buffer-less, 4 verts per quad, one instance per segment.
            ribbon: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..PipelineDesc::ink("ribbon", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // The SAME shader aimed at the SOLID lane (mesh/BRep edges). GreaterEqual is
            // load-bearing here: a mesh edge lies EXACTLY on the boundary of the two faces that
            // meet there, so strict Greater discards the line and float precision decides which
            // pixels survive - the edge reads offset, ragged and asymmetric along its length.
            ribbon_solid: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                depth_compare: if std::env::var("VIEWER_NO_DEPTH").is_ok() { wgpu::CompareFunction::Always } else { wgpu::CompareFunction::GreaterEqual },
                ..PipelineDesc::ink("ribbon.solid", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // The ribbon recipe with the glyph names. `l.segment` at group 3, NOT `l.glyph`: the
            // old builder named its parameter `glyph_layout` and was handed the segment one, and
            // it has always worked because the two layouts are byte-identical. Preserved as it
            // stands - `glyph_depth` below binds the other one.
            glyph: build(device, t, &PipelineDesc::ink("glyph", GLYPH, &[&l.mvp, &l.line, &l.instance, &l.segment])),
            // The four depth-only prepasses. `fs_depth` is binary at half coverage, so the
            // blended colour passes above never write depth and the AA feather cannot leave pale
            // flecks by depth-rejecting a later stroke's opaque core. Without them, ink never
            // writes depth and draw order alone decides who wins - and draw order here is HashMap
            // order, so "who is in front" was effectively random.
            ribbon_depth: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..PipelineDesc::depth_only("ribbon.depth", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            glyph_depth: build(device, t, &PipelineDesc::depth_only("glyph.depth", GLYPH, &[&l.mvp, &l.line, &l.instance, &l.glyph])),
            ribbon_solid_depth: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..PipelineDesc::depth_only("ribbon.solid.depth", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // The only ink-depth pipeline with a vertex buffer: the marker prepass runs the same
            // quad template its colour pass does.
            sphere_depth: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()],
                ..PipelineDesc::depth_only("sphere.depth", SPHERE, &[&l.mvp, &l.line, &l.instance, &l.glyph])
            }),
            splat_depth: build_compute(device, "splat.depth", SPLAT, "cs_depth", &[&l.splat_group0, &l.splat_group1]),
```

**Replace with:**

```rust
            splat_depth: build_compute(device, "splat.depth", SPLAT, "cs_depth", &[&l.splat_group0, &l.splat_group1]),
```

### 6.4 `Gpu` — 22 fields become 2

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod frame;
pub mod instance;
```

**Replace with:**

```rust
pub mod frame;
pub mod glyphs;
pub mod instance;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod present;
pub mod targets;
```

**Replace with:**

```rust
pub mod present;
pub mod segments;
pub mod targets;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use arena::Arena;
use objects::InstanceTable;
```

**Replace with:**

```rust
use arena::Arena;
use glyphs::GlyphLane;
use objects::InstanceTable;
pub use segments::{CylinderSegment, LineStyle};
use segments::SegmentLane;
pub use glyphs::GlyphPoint;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use buffers::{GpuCtx, append_rows, mk_rows_group, zeroed_buffer};
```

**Replace with:**

```rust
use buffers::{GpuCtx, append_rows, zeroed_buffer};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub cyl_template_vbo: wgpu::Buffer,
    pub cyl_template_ibo: wgpu::Buffer,
    pub cyl_index_count: u32,
    /// The SOLID lane (mesh/BRep edges -> cylinders) and the FLAT lane (line/polyline ->
    /// ribbons) used to share one buffer, solid rows first. One buffer meant one splice point,
    /// and a splice point moves whenever either half grows - so appending a file was impossible
    /// and every upload rebuilt the whole table from the CPU copy. Two buffers, same layout and
    /// same shader (each lane indexes from row 0), and both grow by appending.
    pub pipe_buffer: wgpu::Buffer,
    pub pipe_bind_group: wgpu::BindGroup,
    pub pipe_count: u32,
    pub pipe_cap: u64,
    pub segment_buffer: wgpu::Buffer,
    pub segment_bind_group: wgpu::BindGroup,
    pub segment_count: u32,
    pub segment_cap: u64,
    pub sph_template_vbo: wgpu::Buffer,
    pub sph_template_ibo: wgpu::Buffer,
    pub sph_index_count: u32,
    /// Vertex ink, split the same way: spheres are mesh/BRep vertices, glyphs are flat dots.
    pub sphere_buffer: wgpu::Buffer,
    pub sphere_bind_group: wgpu::BindGroup,
    pub sphere_count: u32,
    pub sphere_cap: u64,
    pub glyph_buffer: wgpu::Buffer,
    pub glyph_bind_group: wgpu::BindGroup,
    pub glyph_count: u32,
    pub glyph_cap: u64,
```

**Replace with:**

```rust
    /// The segment family (`segments.rs`). The SOLID lane (mesh/BRep edges -> cylinders) and
    /// the FLAT lane (line/polyline -> ribbons) used to share one buffer, solid rows first. One
    /// buffer meant one splice point, and a splice point moves whenever either half grows - so
    /// appending a file was impossible and every upload rebuilt the whole table from the CPU
    /// copy. Two buffers, same layout and same shader (each lane indexes from row 0), and both
    /// grow by appending.
    pub seg: SegmentLane,
    /// The glyph family (`glyphs.rs`), split the same way: spheres are mesh/BRep vertices,
    /// dots are flat points.
    pub glyphs: GlyphLane,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);
```

**Replace with:**

```rust
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Unit-cylinder tempalte (positions only) - one mesh, instance per edge.
        let (cyl_v, cyl_i) = unit_cylinder(CYL_SIDES);
        let cyl_index_count = cyl_i.len() as u32;

        let cyl_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("cyl.template.vbo"),
            contents: bytemuck::cast_slice(&cyl_v),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let cyl_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("cyl.template.ibo"),
            contents: bytemuck::cast_slice(&cyl_i),
            usage: wgpu::BufferUsages::INDEX,
        });

        // One storage row per edge (VERTEX-visible, read-only) - the two segment tables. Both
        // start at one row and grow by appending; COPY_SRC lets a grown buffer take the old
        // prefix straight from the old one without a round trip through wasm memory.
        let pipe_cap = 1u64;
        let segment_cap = 1u64;
        let pipe_buffer = zeroed_buffer(
            &device, "pipes.buffer",
            std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let segment_buffer =  zeroed_buffer(
            &device, "segments.buffer",
            std::mem::size_of::<CylinderSegment>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);

        let pipe_bind_group = mk_rows_group(&device, &layouts.segment, "pipes.bind_group", &pipe_buffer);
        let segment_bind_group = mk_rows_group(&device, &layouts.segment, "segments.bind_group", &segment_buffer);
```

**Replace with:**

```rust
        let seg = SegmentLane::new(&device, &layouts);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Camera-facing quad template (positions-only) - one mesh, instance per marker
        let (sph_v, sph_i) = unit_quad();
        let sph_index_count = sph_i.len() as u32;
        let sph_template_vbo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("sph.template.vbo"),
            contents: bytemuck::cast_slice(&sph_v),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let sph_template_ibo = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("sph.template.ibo"),
            contents: bytemuck::cast_slice(&sph_i),
            usage: wgpu::BufferUsages::INDEX,
        });
        let sphere_cap = 1u64;
        let glyph_cap = 1u64;
        let sphere_buffer = zeroed_buffer(
            &device,
            "spheres.buffer",
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let glyph_buffer =  zeroed_buffer(
            &device,
            "glyphs.buffer",
            std::mem::size_of::<GlyphPoint>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let sphere_bind_group = mk_rows_group(&device, &layouts.glyph, "spheres.bind_group", &sphere_buffer);
        let glyph_bind_group = mk_rows_group(&device, &layouts.glyph, "glyphs.bind_group", &glyph_buffer);
```

**Replace with:**

```rust
        let glyphs = GlyphLane::new(&device, &layouts);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            cyl_template_vbo,
            cyl_template_ibo,
            cyl_index_count,
            pipe_buffer,
            pipe_bind_group,
            pipe_count,
            pipe_cap,
            segment_buffer,
            segment_bind_group,
            segment_count,
            segment_cap,
            sph_template_vbo,
            sph_template_ibo,
            sph_index_count,
            sphere_buffer,
            sphere_bind_group,
            sphere_count,
            sphere_cap,
            glyph_buffer,
            glyph_bind_group,
            glyph_count,
            glyph_cap,
```

**Replace with:**

```rust
            seg,
            glyphs,
```

### 6.5 `set_scene`, the log, and the six draws

Four appends become two and the six draw sites become six calls. The `draws +=` counts are the
same as before, including `draw_solid` returning **2** in `Flat` style because its colour pass
needs a depth prepass in front of it. The goldens count these.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // The four ink lanes, each a DELTA like the mesh arena: only this file's rows travel,
        // and the bind group is rebuilt only when the buffer behind it actually grew.
        if append_rows(&self.ctx, "pipes.buffer",
            &mut self.pipe_buffer, &mut self.pipe_count, &mut self.pipe_cap, &up.pipes) {
            self.pipe_bind_group = mk_rows_group(&self.ctx.device, &self.layouts.segment, "pipes.bind_group", &self.pipe_buffer);
        }
        if append_rows(&self.ctx, "segments.buffer",
            &mut self.segment_buffer, &mut self.segment_count, &mut self.segment_cap, &up.segments) {
            self.segment_bind_group = mk_rows_group(&self.ctx.device, &self.layouts.segment, "segments.bind_group", &self.segment_buffer);
        }
        if append_rows(&self.ctx, "spheres.buffer",
            &mut self.sphere_buffer, &mut self.sphere_count, &mut self.sphere_cap, &up.spheres) {
            self.sphere_bind_group = mk_rows_group(&self.ctx.device, &self.layouts.glyph, "spheres.bind_group", &self.sphere_buffer);
        }
        if append_rows(&self.ctx, "glyphs.buffer",
            &mut self.glyph_buffer, &mut self.glyph_count, &mut self.glyph_cap, &up.glyphs) {
            self.glyph_bind_group = mk_rows_group(&self.ctx.device, &self.layouts.glyph, "glyphs.bind_group", &self.glyph_buffer);
        }
```

**Replace with:**

```rust
        // The two ink families, each a DELTA like the mesh arena: only this file's rows travel,
        // and a bind group is rebuilt only when the buffer behind it actually grew.
        self.seg.append(&self.ctx, &self.layouts, &up.seg);
        self.glyphs.append(&self.ctx, &self.layouts, &up.glyph);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.objects.len(), self.arena.verts(), self.pipe_count + self.segment_count, self.pipe_count,
            self.sphere_count + self.glyph_count, self.sphere_count, self.point_count
```

**Replace with:**

```rust
            self.objects.len(), self.arena.verts(), self.seg.pipes() + self.seg.ribbons(), self.seg.pipes(),
            self.glyphs.spheres() + self.glyphs.dots(), self.glyphs.spheres(), self.point_count
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.pipe_count > 0 && self.view.show_mesh_edges {
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.pipe_bind_group, &[]);
                match self.view.line_style {
                    LineStyle::Tubes => {
                        pass.set_pipeline(&b.p.cylinder);
                        pass.set_vertex_buffer(0, self.cyl_template_vbo.slice(..));
                        pass.set_index_buffer(self.cyl_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                        pass.draw_indexed(0..self.cyl_index_count, 0, 0..self.pipe_count); // one template, N edges
                    }
                    // The flat lane's own shader over the SOLID table. DEPTH PREPASS
                    // first (binary at half coverage): the blended colour pass writes no depth,
                    // so its AA feather can never depth-reject a later stroke's opaque core -
                    // that rejection read as pale flecks inside the bunny's wireframe.
                    LineStyle::Flat => {
                        pass.set_pipeline(&b.p.ribbon_solid_depth);
                        pass.draw(0..4, 0..self.pipe_count);
                        pass.set_pipeline(&b.p.ribbon_solid);
                        pass.draw(0..4, 0..self.pipe_count);
                        draws += 1;
                    }
                }
                draws += 1;
            }
```

**Replace with:**

```rust
            draws += self.seg.draw_solid(&mut pass, &b, self.view.line_style);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.sphere_count > 0 && self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.sphere_bind_group, &[]);
                pass.set_vertex_buffer(0, self.sph_template_vbo.slice(..));
                pass.set_index_buffer(self.sph_template_ibo.slice(..), wgpu::IndexFormat::Uint32);
                // Same prepass split as the solid ribbons - see the LineStyle::Flat note above.
                pass.set_pipeline(&b.p.sphere_depth);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count);
                pass.set_pipeline(&b.p.sphere);
                pass.draw_indexed(0..self.sph_index_count, 0, 0..self.sphere_count); // one template, N glyphs
                draws += 2;
            }
```

**Replace with:**

```rust
            if self.view.show_mesh_edges && std::env::var("BENCH_NO_MARKERS").is_err() {
                draws += self.glyphs.draw_markers(&mut pass, &b);
            }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if INK_DEPTH_PREPASS && self.segment_count > 0 && self.view.show_lines {
                pass.set_pipeline(&b.p.ribbon_depth);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                pass.draw(0..4, 0..self.segment_count);
                draws += 1;
            }
```

**Replace with:**

```rust
            if INK_DEPTH_PREPASS && self.view.show_lines {
                draws += self.seg.draw_flat_depth(&mut pass, &b);
            }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if INK_DEPTH_PREPASS && self.glyph_count > 0 && self.view.show_points {
                pass.set_pipeline(&b.p.glyph_depth);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(0..3 * self.glyph_count, 0..1);
                draws += 1;
            }
```

**Replace with:**

```rust
            if INK_DEPTH_PREPASS && self.view.show_points {
                draws += self.glyphs.draw_dots_depth(&mut pass, &b);
            }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.segment_count > 0 && self.view.show_lines {
                pass.set_pipeline(&b.p.ribbon);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.segment_bind_group, &[]);
                // instance_index IS the row: this table holds nothing but flat-lane segments
                pass.draw(0..4, 0..self.segment_count);
                draws += 1;
            }
```

**Replace with:**

```rust
            if self.view.show_lines {
                draws += self.seg.draw_flat(&mut pass, &b);
            }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.glyph_count > 0 && self.view.show_points {
                pass.set_pipeline(&b.p.glyph);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.line, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_bind_group(3, &self.glyph_bind_group, &[]);
                pass.draw(0..3 * self.glyph_count, 0..1); // 3 verts/dot, no template
                draws += 1;
            }
```

**Replace with:**

```rust
            if self.view.show_points {
                draws += self.glyphs.draw_dots(&mut pass, &b);
            }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.pipe_count = 0;
        self.segment_count = 0;
        self.sphere_count = 0;
        self.glyph_count = 0;
```

**Replace with:**

```rust
        self.seg.reset();
        self.glyphs.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let solid = self.arena.verts() > 0 || self.pipe_count > 0 || self.sphere_count > 0;
```

**Replace with:**

```rust
        let solid = self.arena.verts() > 0 || self.seg.pipes() > 0 || self.glyphs.spheres() > 0;
```

### 6.6 The walk, the harnesses, and the constant that went home

**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::objects::ObjectBase;
```

**Replace with:**

```rust
use crate::engine::gpu::objects::ObjectBase;
use crate::engine::gpu::segments::FACING_UNKNOWN;
```

**Replace-all** `src/app/scene.rs` `self.tables.pipes` -> `self.tables.seg.pipes` (1 hits)

**Replace-all** `src/app/scene.rs` `self.tables.segments` -> `self.tables.seg.ribbons` (1 hits)

**Replace-all** `src/app/scene.rs` `self.tables.spheres` -> `self.tables.glyph.spheres` (1 hits)

**Replace-all** `src/app/scene.rs` `self.tables.glyphs` -> `self.tables.glyph.dots` (1 hits)

**Replace-all** `src/app/scene.rs` `t.pipes` -> `t.seg.pipes` (8 hits)

**Replace-all** `src/app/scene.rs` `t.segments` -> `t.seg.ribbons` (8 hits)

**Replace-all** `src/app/scene.rs` `t.spheres` -> `t.glyph.spheres` (7 hits)

**Replace-all** `src/app/scene.rs` `t.glyphs` -> `t.glyph.dots` (3 hits)

**Find** in `src/selftest.rs`:

```rust
        let (pipes, sph) = (t.pipes.len(), t.spheres.len());
```

**Replace with:**

```rust
        let (pipes, sph) = (t.seg.pipes.len(), t.glyph.spheres.len());
```

**Find** in `src/selftest.rs`:

```rust
            t.pipes.len(), t.pipes.len() as f64 * 40.0 / 1.048576e6, t.spheres.len(), t.arena.verts.len());
```

**Replace with:**

```rust
            t.seg.pipes.len(), t.seg.pipes.len() as f64 * 40.0 / 1.048576e6, t.glyph.spheres.len(), t.arena.verts.len());
```

**Find** in `examples/check_determinism.rs`:

```rust
        same!(arena.verts); same!(arena.idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Replace with:**

```rust
        same!(arena.verts); same!(arena.idx); same!(seg.ribbons); same!(seg.pipes); same!(glyph.spheres); same!(glyph.dots);
```

**Find** in `examples/check_lean.rs`:

```rust
        same!(arena.verts); same!(arena.idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Replace with:**

```rust
        same!(arena.verts); same!(arena.idx); same!(seg.ribbons); same!(seg.pipes); same!(glyph.spheres); same!(glyph.dots);
```

**Find** in `examples/check_lean.rs`:

```rust
            for (i, (x, y)) in a.pipes.iter().zip(&b.pipes).enumerate() {
```

**Replace with:**

```rust
            for (i, (x, y)) in a.seg.pipes.iter().zip(&b.seg.pipes).enumerate() {
```

**Find** in `examples/check_lean.rs`:

```rust
            for (i, (x, y)) in a.spheres.iter().zip(&b.spheres).enumerate() {
```

**Replace with:**

```rust
            for (i, (x, y)) in a.glyph.spheres.iter().zip(&b.glyph.spheres).enumerate() {
```

**Find** in `src/app/scene.rs`:

```rust
const BLACK: u32 = 0xff00_0000;


/// The two faces an edge belongs to, packed into one word for the shader's facing test.
```

**Replace with:**

```rust
const BLACK: u32 = 0xff00_0000;

/// The two faces an edge belongs to, packed into one word for the shader's facing test.
```

## 7. Proving nothing changed — four ladders

**(1) The compiler.** Both targets, and exactly the nine warnings lesson 46 left.

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets --target x86_64-unknown-linux-gnu
```

One new warning appears — `mk_rows_group` unused in `gpu/mod.rs`, because both families now build
their own bind groups. Step 6.4 drops it from the import.

**(2) The tests.** `cargo xtest` — **4 passed**: `Instance` and `LineUniform` from 47, plus the two
added in §4.1 and §4.2, `CylinderSegment` (cylinder + ribbon) and `GlyphPoint` (sphere + glyph).

What they cannot catch: the shaders' `const FLAG_X = Nu;` bit values and the `facing` word's oct16
encoding. Structure is checked; meaning is not.

**(3) The line multiset.**

```bash
python3 docs/_replay_check.py --moves <end-of-47 tree> /tmp/w48 docs/48-row-families.md
```

```text
docs/48-row-families.md: 76 ops, 0 failed
docs/48-row-families.md: 1 move source(s), 0 not byte-identical
```

One source: `gpu/mod.rs`, over both new files. `FACING_UNKNOWN` is a **Remove** plus an **Add**,
not a Move, because `--moves` pairs ONE source with its destinations — a second source into the
same destination reads every line of the first as undeclared.

**(4) The pixels, and the two harnesses.**

```bash
./docs/_gate.sh                # twice
cargo run -q --release --example check_determinism --target x86_64-unknown-linux-gnu -- assets/pb/lion.pb
cargo run -q --release --example check_lean        --target x86_64-unknown-linux-gnu -- assets/pb/mesh_bunny.pb
```

```text
gate OK                        (both runs)
lion.pb: DETERMINISTIC
mesh_bunny.pb: IDENTICAL
```

This is the lesson where **Tubes matters**. `bunny` under `VIEWER_LINE_STYLE=tubes` is 43,954 ink
and **8** draws against 44,215 and **9** in Flat — one fewer, because Flat needs its prepass and
Tubes does not. That difference is `draw_solid`'s return value, and it is the only golden proving
the `LineStyle` branch survived the move.

## 8. What you can now do in one line

Give the flat lane a second view of its own rows. `drawings_rotated` holds **191,605** ribbons and
36 pipes, and now that both programs and both tables are in one file, pointing the tube pipeline at
the OTHER table is two lines.

**Type all four steps.** The first two add it, the last two take it back out. Do **not** undo it
with `git checkout` — you have not committed lesson 48 yet.

**8a.** **Find** in `src/engine/gpu/segments.rs`:

```rust
        pass.set_pipeline(&b.p.seg.ribbon);
        self.bind_ribbons(pass, b);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.count);
        1
    }
```

**Replace with:**

```rust
        self.bind_ribbons(pass, b);
        pass.set_pipeline(&b.p.seg.cylinder);
        self.template.draw(pass, self.ribbons.count);
        pass.set_pipeline(&b.p.seg.ribbon);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.count);
        2
    }
```

**8b.** Render the sheet:

```bash
cargo run -q --release --example selftest --target x86_64-unknown-linux-gnu -- \
    /tmp/tube.ppm assets/scenes/drawings_rotated.toml
```

```text
[INFO] headless frame: 11 draws, 155465 objects, 900x700
wrote /tmp/tube.ppm  900x700  non-background pixels: 25255 (4.0%)
```

Every line on the page is a real 3D tube: draws **10 → 11**, ink **25,043 → 25,255**, and 191,605
tubes of twelve triangles each in the same frame. Two lines of Rust, no new pipeline, table or row
— both halves were already in the file, missing only a caller willing to pair them.

**8c.** Put it back. **Find** in `src/engine/gpu/segments.rs`:

```rust
        self.bind_ribbons(pass, b);
        pass.set_pipeline(&b.p.seg.cylinder);
        self.template.draw(pass, self.ribbons.count);
        pass.set_pipeline(&b.p.seg.ribbon);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.count);
        2
    }
```

**Replace with:**

```rust
        pass.set_pipeline(&b.p.seg.ribbon);
        self.bind_ribbons(pass, b);
        // instance_index IS the row: this table holds nothing but flat-lane segments
        pass.draw(0..4, 0..self.ribbons.count);
        1
    }
```

**8d.** Confirm you are back: `./docs/_gate.sh --only drawings_rotated` prints 25043 / 10 /
155465 again.

## 9. What is deliberately not here

- **One `InkLane` over both row types.** §1c. Lesson **107** is where the shapes diverge.
- **`append_rows`'s six-parameter form.** Two lanes — raw points and the streamed ones — are not
  `GrowBuf`s until **49**, and migrating a lane that is about to move anyway costs more.
- **`INK_DEPTH_PREPASS`.** Still a `const` in `gpu/mod.rs`; it belongs to the FRAME, not to
  either family, and it goes to `render.rs` at **49**.
- **The `Spacing` enum.** `radius: 0.0` still means "screen-constant px" and `> 0.0` world mm,
  encoded in a float. The first lesson needing both units in one row names it.

## 10. Expected state

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
wc -l src/engine/gpu/mod.rs src/engine/gpu/segments.rs src/engine/gpu/glyphs.rs src/engine/pipelines/mod.rs
grep -c 'include_str!("../../shaders' src/engine/gpu/arena.rs src/engine/gpu/segments.rs src/engine/gpu/glyphs.rs src/engine/pipelines/mod.rs
grep -c 'build(device, t' src/engine/pipelines/mod.rs
```

```text
43

  1055 src/engine/gpu/mod.rs
   338 src/engine/gpu/segments.rs
   251 src/engine/gpu/glyphs.rs
    67 src/engine/pipelines/mod.rs

src/engine/gpu/arena.rs:1
src/engine/gpu/segments.rs:2
src/engine/gpu/glyphs.rs:2
src/engine/pipelines/mod.rs:4

3
```

| | end-of-47 | end-of-48 |
|---|---|---|
| `Gpu` fields | 63 | **43** |
| `gpu/mod.rs` | 1,335 | **1,055** |
| `gpu/segments.rs` | — | **338** |
| `gpu/glyphs.rs` | — | **251** |
| `pipelines/mod.rs` | 130 | **67** |
| `gpu/buffers.rs` | 132 | **177** |
| shaders named in `pipelines/mod.rs` | 8 | **4** |
| render pipelines built there | 12 | **3** |
| `Upload` flat columns | 11 + 2 groups | **7 + 4 groups** |

## Recap

```text
45 made a pipeline a value. 46 put a floor under the families - GpuCtx, GrowBuf, and the six
files for everything that belongs to no lane. 47 built the first family and the thing every
family points at: one object row per guid, one table that owns it, and arena.rs as the worked
example of the contract.

48 is that contract applied twice, to the two ink families, and the argument it settles is what
a MODULE is. Not a shader: `ribbon.wgsl` is compiled twice, once for each of two tables, and
`cylinder.wgsl` is a third program over the first of them, chosen by one `match`. Not a lane
either: pipes and ribbons are two tables of ONE row. A module is the ROW - the format, every
table of it, every pipeline that reads it, and every draw that issues one. Five pipelines and
three draws in segments.rs; four and three in glyphs.rs; and pipelines/mod.rs, which used to
name all nine shaders, now names four and is half the size.

Gpu is 43 fields. Twenty of the twenty-two it lost were one shape written out four times.
```

## Edited

`src/engine/gpu/segments.rs` (NEW) · `src/engine/gpu/glyphs.rs` (NEW) ·
`src/engine/gpu/buffers.rs` (`GrowBuf::append`) · `src/engine/gpu/upload.rs` (two more groups) ·
`src/engine/pipelines/mod.rs` (nine descs and five shader constants leave) ·
`src/engine/gpu/mod.rs` (22 fields → 2, six draws → six calls, two empty banners removed) ·
`src/app/scene.rs` (eight `Replace-all`s; `FACING_UNKNOWN` goes to the row) ·
`src/selftest.rs`, `examples/check_determinism.rs`, `examples/check_lean.rs`.

## Reference

`git diff end-of-47..end-of-48 -- session_viewer/src` is the whole lesson as one patch.

## Next

Lesson **49** — **the frame is a list you can read.** Run the evidence:

```bash
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
awk '/pub fn encode_frame/,/^    }$/' src/engine/gpu/mod.rs | wc -l
grep -c 'extend_from_slice' src/engine/gpu/mod.rs
```

43 fields, of which 25 are the point lanes: raw positions, colours, normals, the streamed copies of
all three, and eleven fields of splat machinery whose record format has **no Rust type at all** —
36 words packed by four `extend_from_slice` calls and read back by literal index. That record gets
a `#[repr(C)]` struct, the point lanes get their two files, and what is left of `encode_frame`
becomes a list of draws you can read top to bottom.
