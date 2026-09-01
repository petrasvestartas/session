# 46 A pipeline is data, not a function

> Lesson [116](116-id-buffer-picking.md) re-runs this frame's whole draw list against a second set
> of pipelines that write object ids instead of colour. It costs one preset and one extra
> `Pipelines::new` call — because of this lesson. Nothing you can see changes: same ink, same draw
> count, same object count, on every scene and every config. Answer key: branch `end-of-45`, so
> `git diff end-of-44..end-of-45 -- session_viewer/src` is this lesson as one patch.
>
> **Lessons 45-51 move code. Every body you cut is pasted byte-identical except for path
> re-roots inside ONE file; if you find yourself improving a line while moving it, stop.**

## 1. Why this seam

### 1a. The evidence — run it on your own tree

```bash
cd session_viewer
wc -l src/engine/pipelines/build.rs                                    # 845
grep -c '^pub fn build_' src/engine/pipelines/build.rs                 # 11
grep -c 'device.create_render_pipeline' src/engine/pipelines/build.rs  # 11
awk '/^pub fn build_/{f=$3; c=0; g=1; next} g&&/^\)/{print c, f; g=0} g&&/:/{c++}' \
    src/engine/pipelines/build.rs
grep -c 'create_bind_group_layout' src/engine/gpu/mod.rs               # 9
grep -rn 'pipelines\.edges' src/ | wc -l                               # 0
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
                                                                       # 116
```

The `awk` prints a parameter count per builder:

```text
7 build_triangle_pipeline(
5 build_grid_pipeline(
4 build_edges_pipeline(
7 build_cylinder_pipeline(
3 build_background_pipeline(
7 build_sphere_pipeline(
11 build_ink_depth_pipeline(
7 build_ribbon_solid_pipeline(
7 build_ribbon_pipeline(
7 build_glyph_pipeline(
5 build_splat_resolve_pipeline(
```

Eleven functions, 3 to 11 parameters, six of them at exactly seven, **eleven**
`create_render_pipeline` calls. Diff any two:

```bash
diff <(sed -n '563,632p' src/engine/pipelines/build.rs)      <(sed -n '634,702p' src/engine/pipelines/build.rs)
```

`build_ribbon_solid_pipeline` and `build_ribbon_pipeline` are 70 and 69 lines, and the only
*setting* that differs is one `depth_compare` expression. Of a `RenderPipelineDescriptor`'s two
dozen leaf settings, exactly eleven ever vary across the fourteen pipelines; the rest are
copy-pasted eleven times. Same story for the nine bind-group layouts.

### 1b. The law this enforces, stated as what it forbids

> **A new pipeline is ONE `PipelineDesc` literal in the list that owns it. A new layout is ONE
> field on `Layouts`. Neither may be a new function, and neither may be a new parameter.**

`Pipelines::new` is frozen at three parameters forever, so no later lesson can add a layout by
threading it through fourteen desc literals — and 82-85, 88, 90, 93, 106 and 108-114 all add one.

### 1c. The rejected alternative

The obvious cut is a builder file per family — `triangle.rs`, `ribbon.rs`, `splat.rs`. **Do not
make it.** It files the eleven near-identical `create_render_pipeline` calls instead of removing
them, and it breaks on the first shape that does not fit: at **48**, `ribbon.wgsl` and
`cylinder.wgsl` read one identical row through one layout, so "the cylinder builder" would need
two files. A desc is data, and data sits in a list without owning a file.

## 2. Where the code lives after this lesson

| symbol | today's home | new home | who may touch it |
|---|---|---|---|
| `Mat4`, `mat_mul`, `mat_to_f32` | `engine/gpu/mod.rs` (top level) | **`src/math.rs`** | anyone — no wgpu, no `self` |
| `eye_from_view_proj`, `ortho_half_height` | `impl Gpu` | **`src/math.rs`** | anyone, incl. the headless harness |
| `xform_point`, `grow_bounds` | `app/scene.rs` | **`src/math.rs`** | anyone |
| `Bounds`, `Aabb64` | nowhere — written out longhand | **`src/math.rs`** | anyone (lessons 63, 64, 69, 109, 112, 114) |
| the 9 `create_bind_group_layout` blocks | `Gpu::build`, lines 506-810 | **`pipelines/layouts.rs`** | `Layouts::new` ONLY |
| `Self::splat_entry` | `impl Gpu` | **`pipelines/layouts.rs::compute_entry`** | `Layouts::new` ONLY |
| the 11 `build_*_pipeline` fns | `pipelines/build.rs` | **gone** — `PipelineDesc` + `build` | nobody adds a twelfth |
| `Target { samples, format }` | `samples` + `color_format`, threaded | **`pipelines/build.rs`** | every desc reads it |
| the 14 pipeline recipes | eleven builder bodies | **`Pipelines::new`, as literals** | the family that draws them (47-49) |
| `splat_depth_pipeline`, `splat_color_pipeline` | two `Gpu` fields | **`Pipelines.splat_depth/_color`** | `render`, through `self.pipelines` |
| `edges` pipeline, `edges.wgsl`, `storage_buffer` | built, compiled, drawn 0 times | **deleted** | — |

The compartment, and what crosses each boundary:

```text
                    +--------------------------------------------------+
                    |  src/math.rs        no wgpu, no self, no kernel   |
                    |  Mat4 · mat_mul · mat_to_f32 · eye_from_view_proj |
                    |  ortho_half_height · xform_point · grow_bounds    |
                    |  Bounds · Aabb64                                  |
                    +--------------------------------------------------+
                        ^  [f64;16] in, [f32;3] out — values only
                        |
   +--------------------+----------------------------------------------+
   |  engine/pipelines/                                                |
   |                                                                   |
   |   layouts.rs  ---- &BindGroupLayout ---->  mod.rs                 |
   |   Layouts::new(device)                     Pipelines::new(        |
   |   9 layouts, one owner                       device, t, &l)       |
   |                                              14 PipelineDesc      |
   |                                              literals + 2 compute |
   |                                                    |              |
   |                                          &PipelineDesc            |
   |                                                    v              |
   |                                            build.rs               |
   |                                            Target · PipelineDesc  |
   |                                            opaque/ink/sheet/      |
   |                                            depth_only · build     |
   |                                            build_compute          |
   +-------------------------------------------------------------------+
                        ^  Layouts::new(&device) once; Pipelines::new(device, t, &l)
                        |  again on every MSAA flip
                    +---+----------------------------------------------+
                    |  engine/gpu/mod.rs   Gpu { layouts, pipelines }  |
                    |  106 fields (was 116)                            |
                    +--------------------------------------------------+
```

**Exit litmus, grep it when you are done:**
`grep -rln 'create_render_pipeline\|create_compute_pipeline\|create_bind_group_layout' src/`
names exactly two files — `pipelines/build.rs` and `pipelines/layouts.rs` — and `build.rs` holds
exactly one `device.create_render_pipeline` and one `device.create_compute_pipeline`.

## 3. Files we touch

| file | what | step | why |
|---|---|---|---|
| `src/math.rs` | **NEW, 123 lines** | 4.1, 5.2 | free-function math has no business inside a wgpu handle |
| `src/lib.rs` | one line | 4.1 | `pub mod math;` |
| `src/engine/pipelines/layouts.rs` | **NEW, 181 lines** | 4.2, 5.3 | one owner for all nine bind-group layouts |
| `src/engine/pipelines/build.rs` | **REWRITTEN, 845 → 215** | 5.1 (delete), 5.4 | the block's one sanctioned rewrite |
| `src/engine/pipelines/mod.rs` | 80 → 148 lines | 5.1, 5.4, 5.5 | fourteen descs + two compute, `new` frozen at 3 params |
| `src/engine/gpu/mod.rs` | 2447 → 2139 lines | 5.1-5.5 | loses the math, the layouts, the two compute pipelines |
| `src/app/scene.rs` | 1382 → 1365 lines | 5.2 | `xform_point`/`grow_bounds` move out |
| `src/selftest.rs` | one line | 5.2 | `Gpu::eye_from_view_proj` → `crate::math::` |
| `src/shaders/edges.wgsl` | **DELETED, 24 lines** | 5.1 | zero draw sites |

**Line budgets.** A bad paste is visible by size alone: `src/math.rs` = **123** lines,
`layouts.rs` = **181**, `build.rs` = **215**, `pipelines/mod.rs` = **148**. If any is off by more
than a line or two, do not run the gate — re-read the file.

New code this lesson may invent: **14 lines** — `Target`, `PipelineDesc.target`, `Bounds`/`Aabb64`
with their doc lines, and `build_compute`'s signature. Everything else already existed somewhere.
Shape taken while a body is moving is free; anything else is deferred to the lesson §9 names.

## 4. The destination files, created first

Both new files are created before anything is cut, so every later step is a deletion plus a
re-point, not a two-ended edit you cannot compile in the middle of. Neither knows about `Gpu`.

### 4.1 `src/math.rs`

Printed in full rather than moved: the two camera solves leave `impl Gpu`, so they lose four
spaces of indent and `ortho_half_height` gains a `pub`. `Bounds` and `Aabb64` are the only new
lines — lessons 63, 64, 69, 109, 112 and 114 all pass a box around and spell it out longhand.

**Create `src/math.rs`**

```rust
//! Pure math the viewer shares — no wgpu, no kernel state, no `self`.
//!
//! Matrices, the two camera solves, and the f32/f64 box aliases. Everything here is a free
//! function over plain arrays, so an engine lane, an app producer and a headless example can all
//! call it without pulling one another in.

use session_rust::Xform;

/// A world-space axis-aligned box in the arena's f32 units: `(min, max)`.
pub type Bounds = ([f32; 3], [f32; 3]);
/// The same box in the kernel's f64 units, as the object table keeps it.
pub type Aabb64 = ([f64; 3], [f64; 3]);

/// One object's world placement as the 16 raw column-major doubles the GPU row needs.
///
/// NOT a kernel `Xform`: that struct carries `typ`/`name` Strings and a guid `OnceLock`, so
/// `Xform::identity()` heap-allocates TWICE per call and every arena row cost two more on the
/// clone into `objects_base`. On a 90k-line sheet that was ~400k allocations - 300 ms of the
/// walk - to carry 128 bytes of numbers nothing downstream ever reads a name off.
pub type Mat4 = [f64; 16];

/// `a * b` in the kernel's convention: column-major, index = col * 4 + row.
/// Matches `impl Mul for &Xform` element for element - and allocates nothing.
pub fn mat_mul(a: &Mat4, b: &Mat4) -> Mat4 {
    let mut out = [0.0f64; 16];
    for i in 0..4 {
        for j in 0..4 {
            let mut sum = 0.0;
            for k in 0..4 {
                sum += a[k * 4 + i] * b[j * 4 + k];
            }
            out[j * 4 + i] = sum;
        }
    }
    out
}

/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
pub fn mat_to_f32(m: &Mat4) -> [f32; 16] {
    std::array::from_fn(|i| m[i] as f32)
}

/// The camera position, recovered from the combined view-projection alone.
///
/// The eye is the one point that projects to nothing: it is where the clip x, y and w all
/// vanish at once, because every view ray passes through it. Three rows of the matrix, three
/// unknowns, one 3x3 solve - no camera struct needed, so this works for any caller that can
/// produce a view-projection, including the headless harness.
///
/// Orthographic has no eye: rows 0, 1 and 3 are linearly dependent there (w is constant 1),
/// the determinant collapses, and the fallback is the view direction pushed a long way back -
/// which is exactly what an orthographic "eye at infinity" means.
pub fn eye_from_view_proj(vp: &Xform) -> [f32; 3] {
    let r = |i: usize| [vp[(i, 0)], vp[(i, 1)], vp[(i, 2)], vp[(i, 3)]];
    let (a, b, c) = (r(0), r(1), r(3));

    // Cramer on [a b c] . p = -[a3 b3 c3]
    let det3 = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
            - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let rows = [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
    let rhs = [-a[3], -b[3], -c[3]];
    let d = det3(rows);

    // Scale-free singularity test: compare against the product of the row magnitudes, so it
    // fires on genuine dependence rather than on a scene whose units make everything small.
    let norm: f64 = rows.iter().map(|r| (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt()).product();
    if d.abs() <= 1e-9 * norm.max(1e-30) {
        // Orthographic: row 3 carries no direction, so take the view axis from row 2 (depth)
        // and stand a long way back along it.
        let f = [vp[(2, 0)], vp[(2, 1)], vp[(2, 2)]];
        let len = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt().max(1e-30);
        return [0, 1, 2].map(|k| (f[k] / len * 1.0e9) as f32);
    }

    [0, 1, 2].map(|k| {
        let mut m = rows;
        for row in 0..3 {
            m[row][k] = rhs[row];
        }
        (det3(m) / d) as f32
    })
}

/// Ortho half-height in world units (mm), 0.0 in perspective. The w row of the composed
/// matrix says which projection this is: perspective carries the view direction there
/// (magnitude 1), orthographic is all zeros (w is constant 1). Row 1 of the matrix is the
/// y basis scaled by s/h, so 1/|row1.xyz| IS the world half-height - rotation and the
/// anchor (translation lives in column 3) drop out. Left as 0.0, every ink lane falls back
/// to the perspective pen formula with clip.w = 1, which pins pens to a zoom-independent
/// world size: zoom out in ortho and the density taper never fires and far-side ink
/// bleeds through faces.
pub fn ortho_half_height(vp: &Xform) -> f32 {
    let w2 = vp[(3, 0)].powi(2) + vp[(3, 1)].powi(2) + vp[(3, 2)].powi(2);
    if w2 > 1e-12 {
        return 0.0;
    }
    let r1 = vp[(1, 0)].powi(2) + vp[(1, 1)].powi(2) + vp[(1, 2)].powi(2);
    if r1 <= 1e-30 {
        return 0.0;
    }
    (1.0 / r1.sqrt()) as f32
}
```

**Find** in `src/lib.rs`:

```rust
mod engine;
```

**Add below it:**

```rust
pub mod math; // shared free-function math: matrices, the camera solves, the box aliases
```

Gate — an unused module compiles, and `math.rs` is `pub` at the crate root so nothing is dead:

```bash
cargo check --target wasm32-unknown-unknown --lib
wc -l src/math.rs        # 105 for now; 123 after step 5.2
```

### 4.2 `src/engine/pipelines/layouts.rs`

Nine layouts describe this entire viewer, each wedged into `Gpu::build` beside the buffer it
happens to precede. Collected, they are one value and one editable list.

The file is created **whole**, not assembled by nine Moves, because every body changes as it
lands: `let mvp_layout` becomes `let mvp`. Step 5.3 then cuts each block from `Gpu::build`.

**Create `src/engine/pipelines/layouts.rs`**

```rust
//! `Layouts` — the single owner of every bind-group layout the viewer binds.
//!
//! A bind-group layout is the shape of one `@group(n)` block: which bindings exist, what type
//! each one is, and which shader stages may read it. Nine of them describe this whole viewer,
//! every pipeline picks from those nine, and a shader and a Rust binding that disagree fail at
//! pipeline creation — so there is exactly one place to look and exactly one place to edit.
//!
//! `Layouts::new` is the editable list: adding a uniform is one entry in one `entries: &[..]`,
//! adding a layout is one field here and one block below. Nothing threads a layout through a
//! parameter list — `Pipelines::new(device, t, &l)` takes the whole set, and is frozen at three
//! parameters for exactly that reason.

/// One `COMPUTE`-visible buffer binding. The two splat groups are nine entries that differ only
/// in their binding index and their buffer type, so they are written as a list, not as nine
/// eleven-line literals.
pub fn compute_entry(
    binding: u32,
    ty: wgpu::BufferBindingType) -> wgpu::BindGroupLayoutEntry{
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer { ty, has_dynamic_offset: false, min_binding_size: None },
        count: None }
}

/// Every bind-group layout, built once and kept for the life of the `Gpu`: an MSAA flip rebuilds
/// every pipeline and every rows bind group from these, so they must outlive both.
pub struct Layouts {
    /// group 0 of everything that reads the camera: the `mvp` uniform, VERTEX only.
    pub mvp: wgpu::BindGroupLayout,
    /// group 1 of `triangle.wgsl`: the animation clock, FRAGMENT only.
    pub time: wgpu::BindGroupLayout,
    /// group 2 everywhere: the object table (`Instance` rows), VERTEX storage.
    pub instance: wgpu::BindGroupLayout,
    /// group 3 of the segment lanes: `CylinderSegment` rows, VERTEX storage.
    pub segment: wgpu::BindGroupLayout,
    /// group 3 of the glyph lanes: `GlyphPoint` rows. Byte-identical to `segment`, which is why
    /// the `glyph` pipeline has always been built against the WRONG one of the two without
    /// anything noticing - see the desc in `mod.rs`.
    pub glyph: wgpu::BindGroupLayout,
    /// group 1 of every ink lane AND group 0 of the splat compute: the pen/viewport uniform.
    /// VERTEX + FRAGMENT, and `cloud_bind_group` is a second bind group over this same layout.
    pub line: wgpu::BindGroupLayout,
    /// The splat compute's two groups: camera + records, then the point columns + pixel buffers.
    pub splat_group0: wgpu::BindGroupLayout,
    pub splat_group1: wgpu::BindGroupLayout,
    /// The fullscreen resolve reads the two per-pixel buffers from the FRAGMENT stage.
    pub splat_resolve: wgpu::BindGroupLayout,
}

impl Layouts {
    /// Build all nine. Order is the order they were created in `Gpu::build`.
    pub fn new(device: &wgpu::Device) -> Self {
        let mvp = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("mvp.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu:: BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                count: None,
            }],
        });

        let time = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("time.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None},
                count: None,
            }],
        });

        let instance = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("instance.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let segment = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("segments.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false, min_binding_size: None,
                },
                count: None,
            }],
        });

        let glyph = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("glyphs.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let line = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("line.layout"),
            entries: &[wgpu::BindGroupLayoutEntry{
                binding: 0,
                // FRAGMENT too: the flat lane's fragment stage reads the viewport size to
                // recover the fragment's ndc for the face-plane depth solve (ribbon.wgsl
                // `ink_depth`). Everything else still only touches it from the vertex stage.
                visibility: wgpu::ShaderStages::VERTEX.union(wgpu::ShaderStages::FRAGMENT),
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count:None
            }],
        });

        let splat_group0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.group0.layout"),
            entries: &[
                compute_entry(0, wgpu::BufferBindingType::Uniform),
                compute_entry(1, wgpu::BufferBindingType::Uniform),
                compute_entry(2, wgpu::BufferBindingType::Storage { read_only: true }),
                compute_entry(3, wgpu::BufferBindingType::Storage { read_only: true }),
            ],
        });

        let splat_group1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.group1.layout"),
            entries: &[
                compute_entry(0, wgpu::BufferBindingType::Storage { read_only: true }), // pos
                compute_entry(1, wgpu::BufferBindingType::Storage { read_only: true }), // col
                compute_entry(2, wgpu::BufferBindingType::Storage { read_only: false }), // sdepth
                compute_entry(3, wgpu::BufferBindingType::Storage { read_only: false }), // scolor
                compute_entry(4, wgpu::BufferBindingType::Storage { read_only: true }),
            ],
        });

        let splat_resolve = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("splat.resolve.layout"),
            entries: & [
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry{
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None,
                },
            ],
        });

        Self { mvp, time, instance, segment, glyph, line, splat_group0, splat_group1, splat_resolve }
    }
}
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
pub mod build;
```

**Add above it:**

```rust
pub mod layouts;
```

Gate:

```bash
cargo check --target wasm32-unknown-unknown --lib
wc -l src/engine/pipelines/layouts.rs    # 181
```

`Layouts` is not used yet, so `cargo check` warns `struct \`Layouts\` is never constructed`.
That warning is the progress bar for the next three steps; it goes away at 5.3.

## 5. The steps

Order is leaves before roots: delete what nothing calls, then the free functions, then the
layouts, then the pipeline recipes, then the two compute pipelines that hang off them. Every step
gets a `cargo check`; steps 5.1, 5.3, 5.4 and 5.5 get the pixel gate as well.

### 5.1 Delete before you move

`edges` is built by a 67-line builder, compiles a 24-line shader and is drawn **zero** times —
`grep -rn 'pipelines\.edges' src/` returns nothing, and `storage_buffer` has zero callers. Neither
is moved: faithfully relocating dead code is how it survives another five lessons.

**Remove** `src/engine/pipelines/build.rs`

```rust
/// Pipeline for mesh edges — `LineList` over the mesh vertices, depth-tested but not written.
```

**through**

```rust
}
```

Five anchors in this lesson start on a comment line — this one and four in 5.2 — because the
comment dies with the function it documents. If your copy of one was ever reworded, anchor on the
`pub fn` below it and delete the stranded comment by hand.

Three one-line sites in `src/engine/pipelines/mod.rs`, each anchored together with the surviving
line above it — the shape to use whenever you drop a single line.

**Find** in `src/engine/pipelines/mod.rs`:

```rust
use build::build_grid_pipeline;
use build::build_edges_pipeline;
```

**Replace with:**

```rust
use build::build_grid_pipeline;
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub grid: wgpu::RenderPipeline,
    pub edges: wgpu::RenderPipeline,
```

**Replace with:**

```rust
    pub grid: wgpu::RenderPipeline,
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
            grid: build_grid_pipeline(device, samples, color_format, aspect_layout, line_layout),
            edges: build_edges_pipeline(device, samples, color_format, aspect_layout),
```

**Replace with:**

```rust
            grid: build_grid_pipeline(device, samples, color_format, aspect_layout, line_layout),
```

`storage_buffer` sits at the tail of `src/engine/gpu/mod.rs`, after `zeroed_buffer`, so the
anchor is the end of the file and what survives is `zeroed_buffer`'s last two lines.

**Find** in `src/engine/gpu/mod.rs`:

```rust
        })
}

/// A storage bufffer filled by  `write buffer`, not `create_buffer_init`: init maps the whole buffer at a creation
/// and on wgpu's web backend that allocates a full-size mirror of the contents in the wasm heap costs three times per scene load.
/// `ẁrite_buffer` stages through the queue instead
/// empty data leaves the minimum buffer zeri-initialized.
fn storage_buffer<T: bytemuck::Pod>(device: &wgpu::Device, queue: &wgpu::Queue, label: &str, data: &[T]) -> wgpu::Buffer{
    let size = (data.len() * std::mem::size_of::<T>()).max(std::mem::size_of::<T>()).max(4) as u64;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    if !data.is_empty(){
        queue.write_buffer(&buf, 0, bytemuck::cast_slice(data));
    }
    buf
}
```

**Replace with:**

```rust
        })
}
```

Then the shader itself, which nothing includes any more:

```bash
rm src/shaders/edges.wgsl
```

Gate — the warning count drops by one and the frame does not move:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
./docs/_gate.sh
```

`gate OK`. That is **89 lines of Rust** gone plus a 24-line shader, and one fewer WGSL module
compiled at every startup.

### 5.2 `src/math.rs` — the bodies leave, the paths do not

`gpu/mod.rs` re-exports the five names it used to define, so every `engine::gpu::` caller keeps
the path it already types; `scene.rs` does the same for its two.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::pipelines::Pipelines;
```

**Add below it:**

```rust
/// The shared math lives in `crate::math`; re-exported here so every `engine::gpu::` caller
/// keeps the path it already types.
pub use crate::math::{Mat4, mat_mul, mat_to_f32, eye_from_view_proj, ortho_half_height};
```

Now the matrix block. Its first seven lines are a doc comment describing `ArenaUpload`, stranded
above `Mat4` by an old edit; it comes back below.

**Remove** `src/engine/gpu/mod.rs`

```rust
/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
```

**through**

```rust
}
```

**Remove** `src/engine/gpu/mod.rs`

```rust
/// The GPU edge: f64 world math stays CPU-side, the instance row is f32.
```

**through**

```rust
}
```

**Find** in `src/engine/gpu/mod.rs` — the two cuts left three blank lines:

```rust
const INK_DEPTH_PREPASS: bool = false;




```

**Replace with:**

```rust
const INK_DEPTH_PREPASS: bool = false;


```

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub struct ArenaUpload{
```

**Add above it:**

```rust
/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
/// the engine borrows it, uploads, and forgets.
/// Lanes stay apart (SOLID pipes/spheres vs flat segments/glyphs)
/// and are spliced solid-first at upload.
/// `objects` holds the TRUE per-object transfrom + tint + flags.
/// `Gpu` builds instance rows from it and rebases them as the camera moves.
/// No Mesh, no Session, no wgpu type on the app side of this line.
```

Next the two camera solves — `impl Gpu` methods that never touch `self`, the giveaway that they
were never engine code.

**Remove** `src/engine/gpu/mod.rs`

```rust
    /// The camera position, recovered from the combined view-projection alone.
```

**through**

```rust
    }
```

**Remove** `src/engine/gpu/mod.rs`

```rust
    /// Ortho half-height in world units (mm), 0.0 in perspective. The w row of the composed
```

**through**

```rust
    }
```

**Find** in `src/engine/gpu/mod.rs` — again, three blank lines where two functions were:

```rust
        t0.elapsed().as_secs_f64()
    }




```

**Replace with:**

```rust
        t0.elapsed().as_secs_f64()
    }


```

They were called as associated functions; they are free functions now. Both counts are asserted —
if yours differ, you cut the wrong region:

**Replace-all** `src/engine/gpu/mod.rs` `Self::ortho_half_height` → `ortho_half_height` (2 hits)

**Replace-all** `src/engine/gpu/mod.rs` `Self::eye_from_view_proj` → `eye_from_view_proj` (2 hits)

The headless harness called it through `Gpu`; it needs a matrix, not a graphics card:

**Find** in `src/selftest.rs`:

```rust
        let solved = Gpu::eye_from_view_proj(&view_proj);
```

**Replace with:**

```rust
        let solved = crate::math::eye_from_view_proj(&view_proj);
```

Now the app side. `Mat4` leaves the import list with the body in `scene.rs` that named it:

**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint, Mat4, mat_mul};
```

**Replace with:**

```rust
use crate::engine::gpu::{ArenaUpload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint, mat_mul};
pub use crate::math::{grow_bounds, xform_point};
```

Byte-identical in both files, so they are **moved**, not retyped — the verb lesson **47** leans
on. `docs/_replay_check.py --moves` proves a move did not quietly lose a line:

**Move** `src/app/scene.rs` `pub fn xform_point(m: &Mat4, p: [f32; 3]) -> [f32; 3] {` **through** `}` **to** `src/math.rs` **at the end**

**Move** `src/app/scene.rs` `fn grow_bounds(min: &mut [f32; 3], max: &mut [f32; 3], p: [f32; 3]) {` **through** `}` **to** `src/math.rs` **at the end**

**Replace-all** `src/math.rs` `fn grow_bounds` → `pub fn grow_bounds` (1 hit)

**Find** in `src/app/scene.rs` — the two Moves left three blank lines:

```rust
    v.shrink_to_fit();
}



/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
```

**Replace with:**

```rust
    v.shrink_to_fit();
}

/// A plane is infinite - draw a fix sqzare around its origin, spanned by its x/y axes
```

`src/math.rs` is complete — 123 lines, in this order:

```text
  1- 12  header + Bounds + Aabb64          (new: 2 aliases, 2 doc lines)
 14- 41  Mat4 + mat_mul + mat_to_f32       (from gpu/mod.rs, unchanged)
 43-105  eye_from_view_proj + ortho_half_height   (from impl Gpu, dedented)
107-123  xform_point + grow_bounds         (moved from scene.rs)
```

Gate:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
wc -l src/math.rs src/app/scene.rs      # 123, 1365
```

### 5.3 The nine layouts leave `Gpu::build`

`Layouts` is one field where nine were. Each block is cut out of `Gpu::build`, most with the blank
line that follows, and every reference renamed by a counted `Replace-all` — the count is the proof.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::pipelines::Pipelines;
```

**Replace with:**

```rust
use crate::engine::pipelines::Pipelines;
use crate::engine::pipelines::layouts::Layouts;
```

`Layouts::new` runs once, before the first buffer that needs one:

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let msaa_view = Self::create_msaa_view(&device, &config, samples);
```

**Add below it:**

```rust

        // Every bind-group layout, in one value. They outlive the pipelines they were built
        // for: an MSAA flip rebuilds those from these.
        let layouts = Layouts::new(&device);
```

Now the nine blocks. Seven of them are followed by a blank line and take it with them; `glyph`
and `splat_group0` are followed immediately by the next statement and stop at their own `});`.

**Remove** `src/engine/gpu/mod.rs` `        let mvp_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

**Remove** `src/engine/gpu/mod.rs` `        let time_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

**Remove** `src/engine/gpu/mod.rs` `        let instance_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

**Remove** `src/engine/gpu/mod.rs` `        let segment_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

**Remove** `src/engine/gpu/mod.rs` `        let glyph_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** `        });`

**Remove** `src/engine/gpu/mod.rs` `        let line_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

**Remove** `src/engine/gpu/mod.rs` `        let splat_group0_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** `        });`

**Remove** `src/engine/gpu/mod.rs` `        let splat_group1_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

**Remove** `src/engine/gpu/mod.rs` `        let splat_resolve_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{` **through** the blank line below it:

```rust

```

`splat_entry` was a `COMPUTE`-visible buffer entry written once so the two splat groups could be
lists; it is `compute_entry` in `layouts.rs` now. Its two comment lines above it stay.

**Remove** `src/engine/gpu/mod.rs` `    fn splat_entry(` **through** the blank line below its closing brace:

```rust

```

**Find** in `src/engine/gpu/mod.rs` — the six layout fields and the comment that explains them,
anchored on the field above them:

```rust
    inside: Vec<bool>, // current FLAG_INSIDE state per instance row, for change detection
    // Layouts surfvive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    mvp_layout: wgpu::BindGroupLayout,
    time_layout: wgpu::BindGroupLayout,
    instance_layout: wgpu::BindGroupLayout,
    line_layout: wgpu::BindGroupLayout,
    segment_layout: wgpu::BindGroupLayout,
    glyph_layout: wgpu::BindGroupLayout,
```

**Replace with:**

```rust
    inside: Vec<bool>, // current FLAG_INSIDE state per instance row, for change detection
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    splat_recs: wgpu::Buffer,
    splat_group0_layout: wgpu::BindGroupLayout,
    splat_group1_layout: wgpu::BindGroupLayout,
    splat_resolve_layout: wgpu::BindGroupLayout,
```

**Replace with:**

```rust
    splat_recs: wgpu::Buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub config: wgpu::SurfaceConfiguration,  // Settings for Surface: size, pixel format
```

**Add below it:**

```rust
    // Layouts surfvive so set_scene can rebuild bind groups and pipelines on an MSAA change.
    pub layouts: Layouts,
```

The struct literal in `build` loses the same nine names and gains one:

**Find** in `src/engine/gpu/mod.rs`:

```rust
            inside: Vec::new(),
            mvp_layout,
            time_layout,
            instance_layout,
            line_layout,
            segment_layout,
            glyph_layout,
```

**Replace with:**

```rust
            inside: Vec::new(),
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            splat_recs,
            splat_group0_layout,
            splat_group1_layout,
            splat_resolve_layout,
```

**Replace with:**

```rust
            splat_recs,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            config,
```

**Add below it:**

```rust
            layouts,
```

Nine counted renames, re-pointing every remaining reference at the one owner. If a count differs,
you cut the wrong region:

**Replace-all** `src/engine/gpu/mod.rs` `mvp_layout` → `layouts.mvp` (3 hits)

**Replace-all** `src/engine/gpu/mod.rs` `time_layout` → `layouts.time` (3 hits)

**Replace-all** `src/engine/gpu/mod.rs` `instance_layout` → `layouts.instance` (4 hits)

**Replace-all** `src/engine/gpu/mod.rs` `segment_layout` → `layouts.segment` (6 hits)

**Replace-all** `src/engine/gpu/mod.rs` `glyph_layout` → `layouts.glyph` (6 hits)

**Replace-all** `src/engine/gpu/mod.rs` `line_layout` → `layouts.line` (5 hits)

**Replace-all** `src/engine/gpu/mod.rs` `splat_group0_layout` → `layouts.splat_group0` (5 hits)

**Replace-all** `src/engine/gpu/mod.rs` `splat_group1_layout` → `layouts.splat_group1` (5 hits)

**Replace-all** `src/engine/gpu/mod.rs` `splat_resolve_layout` → `layouts.splat_resolve` (4 hits)

Gate — `Gpu` is down nine fields and up one, and `Layouts` is no longer dead:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -c 'create_bind_group_layout' src/engine/gpu/mod.rs                  # 0
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
                                                                          # 108
./docs/_gate.sh
```

### 5.4 `build.rs` rewritten, and fourteen descs

The block's **one sanctioned rewrite**, and the one step whose halves cannot compile apart: the
moment `build.rs` stops exporting the ten remaining builders, `Pipelines::new` stops resolving.
Type 5.4 end to end before you run `cargo check` — it is red in between, and that is not a mistake.

Two things that are easy to lose:

- **`Pipelines::new` takes `device` here, not `ctx`.** `GpuCtx` does not exist until lesson
  **46**, which renames it with a single Replace-all. Do not invent it now.
- **There are TWO live `VIEWER_NO_DEPTH → CompareFunction::Always` branches**, in
  `build_sphere_pipeline` and `build_ribbon_solid_pipeline` (`build_ribbon_pipeline` has none).
  Both survive as `depth_compare:` overrides on the `sphere` and `ribbon.solid` descs. No golden
  runs with that variable set, so losing one is silent.

`Pipelines` keeps **named fields**, not a map: the MSAA flip assigns a whole new `Pipelines`
mid-session, and lesson 116's id pass re-runs the draw list against a second set.

**Create `src/engine/pipelines/build.rs`**

```rust
//! Building blocks for every pipeline in the viewer. A pipeline is DATA - a `PipelineDesc`
//! literal in `mod.rs` - and this file holds the one function that turns one into a
//! `wgpu::RenderPipeline`, plus the four presets that name the four recipes the viewer actually
//! uses. There is exactly ONE `create_render_pipeline` call and exactly ONE
//! `create_compute_pipeline` call below.

// `samples` (on `Target`) is MSAA. 4 = smooth mesh silhouettes, but it quadruples
// fragment work AND framebuffer bandwidth. Linework does its OWN antialiasing (SDF alpha ramp
// in ribbon/glyph), so on a 2D sheet - 100% linework - MSAA buys nothing and costs everything.
// It cannot be mixed WITHIN a frame: sample count is a property of the render PASS, so every
// pipeline drawn into it must agree. The viewer therefore picks one per SCENE - see
// `Gpu::msaa_for`.
/// What every pipeline drawn into one render pass must agree on. An MSAA flip therefore rebuilds
/// every pipeline - see `Gpu::set_scene`.
#[derive(Clone, Copy)]
pub struct Target {
    pub samples: u32,
    pub format: wgpu::TextureFormat,
}

/// A pipeline as DATA. Eleven near-identical builders existed because a pipeline was modelled as
/// code; they differed in about five of these fields. Everything not named here - `Ccw`, no
/// culling, `Fill`, `Depth32Float`, no stencil, no bias, `mask: !0`, no `alpha_to_coverage` - is
/// the same in all fourteen and lives once, in `build`.
pub struct PipelineDesc<'a> {
    /// Names the shader module, the pipeline layout AND the pipeline; the only string a GPU
    /// error message will hand back, so it is also lesson 106's error-scope label.
    pub label: &'a str,
    /// WGSL source text, normally an `include_str!`.
    pub shader: &'a str,
    /// `fs_main` for a colour pass, `fs_depth` for a prepass.
    pub fs_entry: &'a str,
    pub topology: wgpu::PrimitiveTopology,
    /// Almost always empty: the ink lanes read their rows from storage buffers, not attributes.
    pub vertex_buffers: &'a [wgpu::VertexBufferLayout<'a>],
    /// `@group(0)`, `@group(1)`, ... in order.
    pub bind_groups: &'a [&'a wgpu::BindGroupLayout],
    pub blend: Option<wgpu::BlendState>,
    /// `ColorWrites::empty()` is what makes a depth-only pass legal against a colour attachment.
    pub write_mask: wgpu::ColorWrites,
    pub depth_write: bool,
    pub depth_compare: wgpu::CompareFunction,
    /// Overrides the pass target. `None` = draw into the frame; a value pins this pipeline to its
    /// own attachment (lesson 92's R8Unorm mask, lesson 116's R32Uint id buffer).
    pub target: Option<Target>,
}

impl<'a> PipelineDesc<'a> {
    /// Solid geometry: unblended, writes depth, strict reverse-Z. Tubes, the splat resolve, and -
    /// with two fields flipped - the grid and the background.
    pub fn opaque(label: &'a str, shader: &'a str, bind_groups: &'a [&'a wgpu::BindGroupLayout]) -> Self {
        Self {
            label, shader, bind_groups,
            fs_entry: "fs_main",
            topology: wgpu::PrimitiveTopology::TriangleList,
            vertex_buffers: &[],
            blend: None,
            write_mask: wgpu::ColorWrites::ALL,
            depth_write: true,
            depth_compare: wgpu::CompareFunction::Greater,
            target: None,
        }
    }

    /// Linework and markers: blended for the AA feather, and NOT depth-writing, because a
    /// half-covered feather pixel that wrote depth would reject the next stroke's opaque core.
    /// `GreaterEqual`, not `Greater`: ink sits exactly on the face it annotates, and it has to
    /// survive its OWN depth prepass.
    pub fn ink(label: &'a str, shader: &'a str, bind_groups: &'a [&'a wgpu::BindGroupLayout]) -> Self {
        Self {
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            depth_write: false,
            depth_compare: wgpu::CompareFunction::GreaterEqual,
            ..Self::opaque(label, shader, bind_groups)
        }
    }

    /// Blended fills. A surface can be translucent - a PDF sheet's shaded regions arrive at 5-40%
    /// alpha and unblended render SOLID - and opaque geometry is unaffected, since alpha 1 blends
    /// to itself. Flipping `depth_write` off is the whole of `triangle_sheet`.
    pub fn sheet(label: &'a str, shader: &'a str, bind_groups: &'a [&'a wgpu::BindGroupLayout]) -> Self {
        Self {
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            ..Self::opaque(label, shader, bind_groups)
        }
    }

    /// A depth-only prepass: run `fs_depth`, write depth, write NO colour. The pass has a colour
    /// attachment so the pipeline must declare one too - Dawn rejects an empty target list
    /// against a colour pass - so every channel is masked off instead.
    pub fn depth_only(label: &'a str, shader: &'a str, bind_groups: &'a [&'a wgpu::BindGroupLayout]) -> Self {
        Self {
            fs_entry: "fs_depth",
            write_mask: wgpu::ColorWrites::empty(),
            ..Self::opaque(label, shader, bind_groups)
        }
    }
}

/// The ONE `create_render_pipeline` call in the viewer.
pub fn build(device: &wgpu::Device, t: Target, d: &PipelineDesc) -> wgpu::RenderPipeline {
    let t = d.target.unwrap_or(t);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(d.label),
        source: wgpu::ShaderSource::Wgsl(d.shader.into()),
    });
    let groups: Vec<Option<&wgpu::BindGroupLayout>> = d.bind_groups.iter().map(|g| Some(*g)).collect();
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(d.label),
        bind_group_layouts: &groups,
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(d.label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: d.vertex_buffers,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(d.fs_entry),
            targets: &[Some(wgpu::ColorTargetState {
                format: t.format,
                blend: d.blend,
                write_mask: d.write_mask,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: d.topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(d.depth_write),
            depth_compare: Some(d.depth_compare),
            stencil: wgpu::StencilState::default(),
            // No hardware bias anywhere. The units of `constant` on a float depth format are
            // implementation-defined - a driver may apply less than asked, or nothing - so faces
            // recede in `triangle.wgsl` instead (FACE_PUSH) and the ink lanes lean on
            // `GreaterEqual`.
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState { count: t.samples, mask: !0, alpha_to_coverage_enabled: false },
        multiview_mask: None,
        cache: None,
    })
}

/// The same, for compute. No target, no depth, no blend - a shader, an entry point and its groups.
pub fn build_compute(
    device: &wgpu::Device,
    label: &str,
    wgsl: &str,
    entry: &str,
    bind_groups: &[&wgpu::BindGroupLayout],
) -> wgpu::ComputePipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
    });
    let groups: Vec<Option<&wgpu::BindGroupLayout>> = bind_groups.iter().map(|g| Some(*g)).collect();
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &groups,
        immediate_size: 0,
    });
    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some(label),
        layout: Some(&layout),
        module: &shader,
        entry_point: Some(entry),
        compilation_options: Default::default(),
        cache: None,
    })
}

const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 3,
    format: wgpu::VertexFormat::Uint32,
}];
const CYL_TEMPLATE_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
    offset: 0,
    shader_location: 0,
    format: wgpu::VertexFormat::Float32x3,
}];

// This helps the GPU to read the second vertex buffer - the instance row id.
// Without a layout description, the pipeline doesn' know those bytes exists and in what shape they are.
/// Vertex-buffer layout for the per-vertex instance-row id (`@location(3)`, one `u32` per vertex).
pub fn instance_id_layout() -> wgpu::VertexBufferLayout<'static>{
    wgpu::VertexBufferLayout{
        array_stride: 4,
        step_mode: wgpu::VertexStepMode::Vertex, // one u32 per vertex
        attributes: &INSTANCE_ID_ATTRIBS // advances per-vertex, like position
    }
}

/// Vertex-buffer layout for the unit-cylinder/-sphere template positions (`@location(0)`, one `vec3<f32>`).
pub fn cyl_template_layout() -> wgpu::VertexBufferLayout<'static>{
    wgpu::VertexBufferLayout {
        array_stride: 12, // one vec3<f32> per templete vertex
        step_mode: wgpu::VertexStepMode::Vertex,
        attributes: &CYL_TEMPLATE_ATTRIBS
    }
}
```

`build` holds the only `create_render_pipeline` call in the viewer, `build_compute` the only
`create_compute_pipeline`. Each preset is a `..Self::opaque(..)` spread, so the difference between
them is the lines you can see: `ink` is three fields, `sheet` one, `depth_only` two.

Now the call sites. `pipelines/mod.rs` stops importing eleven builders and starts naming the WGSL
each family compiles; lessons 48-50 move each pair into the family file that owns its row.

**Find** in `src/engine/pipelines/mod.rs`:

```rust
pub mod build;

use build::build_triangle_pipeline;
use build::build_grid_pipeline;
use build::build_sphere_pipeline;
use build::build_ribbon_pipeline;
use build::build_ribbon_solid_pipeline;
use build::build_glyph_pipeline;
use build::build_background_pipeline;
use build::build_splat_resolve_pipeline;
use build::build_ink_depth_pipeline;

use crate::engine::pipelines::build::build_cylinder_pipeline;
```

**Replace with:**

```rust
pub mod build;

pub use build::{PipelineDesc, Target};
use build::{build, cyl_template_layout, instance_id_layout};
use session_rust::RenderVertex;
use layouts::Layouts;

// The WGSL each family compiles. They live beside the descs that name them; lessons 48-50 move
// each pair into the family file that owns the row it draws.
const RIBBON: &str = include_str!("../../shaders/ribbon.wgsl");
const GLYPH: &str = include_str!("../../shaders/glyph.wgsl");
const SPHERE: &str = include_str!("../../shaders/sphere.wgsl");
const GRID: &str = include_str!("../../shaders/grid.wgsl");
const CYLINDER: &str = include_str!("../../shaders/cylinder.wgsl");
const BACKGROUND: &str = include_str!("../../shaders/background.wgsl");
const SPLAT_RESOLVE: &str = include_str!("../../shaders/splat_resolve.wgsl");
const TRIANGLE: &str = include_str!("../../shaders/triangle.wgsl");

```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    /// draw order instead of fighting over one coplanar depth value. See build_triangle_pipeline.
```

**Replace with:**

```rust
    /// draw order instead of fighting over one coplanar depth value. See its desc below.
```

Ten parameters become three, and they are frozen at three:

**Remove** `src/engine/pipelines/mod.rs` `    pub fn new(` **through** `    ) -> Self{`

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    /// Build every render pipeline from the shared bind-group layouts.
        Self {
```

**Replace with:**

```rust
    /// Build every pipeline from the shared bind-group layouts. Each one is a `PipelineDesc`
    /// literal: a NEW pipeline is a literal here, never a new builder function.
    /// FROZEN AT THREE PARAMETERS. A new layout is a field on `Layouts`, never a parameter here -
    /// otherwise every later lesson that adds one threads it through fourteen desc literals.
    pub fn new(device: &wgpu::Device, t: Target, l: &Layouts) -> Self{
        Self {
```

Four groups of descs, in the order they already sit in. What you cannot see in a literal is the
preset, and the preset is identical across all fourteen.

**Find** in `src/engine/pipelines/mod.rs` — the sheet pair:

```rust
            triangle: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout, true),
            triangle_sheet: build_triangle_pipeline(device, samples, color_format, aspect_layout, time_layout, instance_layout, false),
```

**Replace with:**

```rust
            // Solid mesh triangles. Blended, because a surface can be translucent.
            triangle: build(device, t, &PipelineDesc {
                vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                ..PipelineDesc::sheet("triangle", TRIANGLE, &[&l.mvp, &l.time, &l.instance])
            }),
            // The same program with ONE field flipped. A drawing's fills are exactly coplanar -
            // 362,581 vertices of a PDF sheet, ONE distinct z - so the depth buffer cannot order
            // them: equal depth fails a strict Greater, and the depths are not even reliably
            // equal, since positions are camera-relative and re-rounded to f32 every frame.
            // Whichever fill won flipped as the camera moved, and that flip is the flicker
            // between lettering and hatching. With no depth WRITE the fills stop arbitrating and
            // composite in draw order, which is what a page is. They are still depth-TESTED, so
            // 3D geometry in front still occludes the sheet.
            triangle_sheet: build(device, t, &PipelineDesc {
                vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                depth_write: false,
                ..PipelineDesc::sheet("triangle.sheet", TRIANGLE, &[&l.mvp, &l.time, &l.instance])
            }),
```

**Find** in `src/engine/pipelines/mod.rs` — the four opaque lanes:

```rust
            grid: build_grid_pipeline(device, samples, color_format, aspect_layout, line_layout),
            cylinder: build_cylinder_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            background: build_background_pipeline(device, samples, color_format),
            splat_resolve: build_splat_resolve_pipeline(device, samples, color_format, line_layout, splat_resolve_layout),
```

**Replace with:**

```rust
            // Buffer-less LineList - positions come from @builtin(vertex_index). Depth-tested
            // so geometry hides it, never depth-writing, so it hides nothing.
            grid: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::LineList,
                depth_write: false,
                ..PipelineDesc::opaque("grid", GRID, &[&l.mvp, &l.line])
            }),
            // Linework tubes: one unit-cylinder template instanced per segment. Solid, so it
            // occludes correctly and needs no bias at all.
            cylinder: build(device, t, &PipelineDesc {
                vertex_buffers: &[cyl_template_layout()], // slot 0 - the unit-cylinder positions
                ..PipelineDesc::opaque("cylinder", CYLINDER, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
            // A buffer-less triangle at the far plane, with no bind groups at all. `Always`,
            // never depth-writing: it paints under everything and blocks nothing.
            background: build(device, t, &PipelineDesc {
                depth_write: false,
                depth_compare: wgpu::CompareFunction::Always,
                ..PipelineDesc::opaque("background", BACKGROUND, &[])
            }),
            // Fullscreen composite of the two per-pixel splat buffers. Splats occlude like any
            // solid, so this is the `opaque` preset unchanged - the only desc with no overrides.
            splat_resolve: build(device, t, &PipelineDesc::opaque("splat.resolve", SPLAT_RESOLVE, &[&l.line, &l.splat_resolve])),
```

**Find** in `src/engine/pipelines/mod.rs` — the four ink lanes:

```rust
            sphere: build_sphere_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, glyph_layout),
            ribbon: build_ribbon_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            ribbon_solid: build_ribbon_solid_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
            glyph: build_glyph_pipeline(device, samples, color_format, aspect_layout, line_layout, instance_layout, segment_layout),
```

**Replace with:**

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
```

`glyph` binds `l.segment` at group 3, not `l.glyph` — not a typo introduced here: the old
`build_glyph_pipeline` was handed `segment_layout` despite its parameter name, and it works because
the two layouts are byte-identical. A body you are moving is not a body you are fixing.

**Find** in `src/engine/pipelines/mod.rs` — the four depth-only prepasses:

```rust
            ribbon_depth: build_ink_depth_pipeline(device, samples, "ribbon.depth", color_format,
                wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ribbon.wgsl").into()),
                aspect_layout, line_layout, instance_layout, segment_layout, &[], wgpu::PrimitiveTopology::TriangleStrip),
            glyph_depth: build_ink_depth_pipeline(device, samples, "glyph.depth", color_format,
                wgpu::ShaderSource::Wgsl(include_str!("../../shaders/glyph.wgsl").into()),
                aspect_layout, line_layout, instance_layout, glyph_layout, &[], wgpu::PrimitiveTopology::TriangleList),
            ribbon_solid_depth: build_ink_depth_pipeline(device, samples, "ribbon.solid.depth", color_format,
                wgpu::ShaderSource::Wgsl(include_str!("../../shaders/ribbon.wgsl").into()),
                aspect_layout, line_layout, instance_layout, segment_layout, &[], wgpu::PrimitiveTopology::TriangleStrip),
            sphere_depth: build_ink_depth_pipeline(device, samples, "sphere.depth", color_format,
                wgpu::ShaderSource::Wgsl(include_str!("../../shaders/sphere.wgsl").into()),
                aspect_layout, line_layout, instance_layout, glyph_layout,
                &[build::cyl_template_layout()], wgpu::PrimitiveTopology::TriangleList),
```

**Replace with:**

```rust
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
```

Last, the two `Pipelines::new` call sites in `gpu/mod.rs`. `samples` and `color_format` were
threaded as two parameters through eleven builders; they are one `Target` now, and it is `Copy`.

**Find** in `src/engine/gpu/mod.rs`:

```rust
use crate::engine::pipelines::Pipelines;
```

**Replace with:**

```rust
use crate::engine::pipelines::{Pipelines, Target};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let pipelines = Pipelines::new(
            &device,
            samples,
            config.format,
            &layouts.mvp,
            &layouts.time,
            &layouts.instance,
            &layouts.line,
            &layouts.segment,
            &layouts.glyph,
            &layouts.splat_resolve,
        );
```

**Replace with:**

```rust
        let pipelines = Pipelines::new(&device, Target { samples, format: config.format }, &layouts);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.pipelines = Pipelines::new(
                &self.device,
                samples,
                self.config.format,
                &self.layouts.mvp,
                &self.layouts.time,
                &self.layouts.instance,
                &self.layouts.line,
                &self.layouts.segment,
                &self.layouts.glyph,
                &self.layouts.splat_resolve,
            );
```

**Replace with:**

```rust
            self.pipelines = Pipelines::new(&self.device, Target { samples, format: self.config.format }, &self.layouts);
```

Gate — now it should be green, and the frame should not have moved on any config:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
wc -l src/engine/pipelines/build.rs src/engine/pipelines/mod.rs   # 215, 140
grep -c 'device.create_render_pipeline' src/engine/pipelines/build.rs   # 1
./docs/_gate.sh
```

Each config in the gate exercises a different group, which is what makes one gate run cover all
four: the default config draws the ink descs, `VIEWER_LINE_STYLE=tubes` swaps in `cylinder`,
`drawings_rotated` is `triangle_sheet` end to end, and the cloud scenes are `splat_resolve`.

### 5.5 The two compute pipelines come home

The splat rasterizer's two compute pipelines were built inline in `Gpu::build` and stored as two
`Gpu` fields — the only two in the viewer not in `Pipelines`. `build_compute` takes them.

**Find** in `src/engine/pipelines/mod.rs`:

```rust
use build::{build, cyl_template_layout, instance_id_layout};
```

**Replace with:**

```rust
use build::{build, build_compute, cyl_template_layout, instance_id_layout};
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
const TRIANGLE: &str = include_str!("../../shaders/triangle.wgsl");
```

**Add below it:**

```rust
const SPLAT: &str = include_str!("../../shaders/splat.wgsl");
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
/// Every render pipeline the viewer draws with, built once at startup.
pub struct Pipelines{
```

**Replace with:**

```rust
/// Every pipeline the viewer draws with, built once at startup and rebuilt whole on an MSAA
/// flip. Fourteen render pipelines and two compute.
pub struct Pipelines{
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub splat_resolve: wgpu::RenderPipeline, // fullscreen composite of the splat buffers
```

**Add below it:**

```rust
    // The splat rasterizer is COMPUTE: two passes over one shader, depth for every point first,
    // then colour for every point, composing into the two per-pixel atomics buffers.
    pub splat_depth: wgpu::ComputePipeline,
    pub splat_color: wgpu::ComputePipeline,
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
                ..PipelineDesc::depth_only("sphere.depth", SPHERE, &[&l.mvp, &l.line, &l.instance, &l.glyph])
            }),
```

**Add below it:**

```rust
            splat_depth: build_compute(device, "splat.depth", SPLAT, "cs_depth", &[&l.splat_group0, &l.splat_group1]),
            splat_color: build_compute(device, "splat.color", SPLAT, "cs_color", &[&l.splat_group0, &l.splat_group1]),
```

**Find** in `src/engine/gpu/mod.rs` — the inline shader module, the pipeline layout and the two
`create_compute_pipeline` calls, anchored on the comment that follows them:

```rust
        let splat_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor{
            label: Some("splat.shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/splat.wgsl").into()),
        });

        let splat_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label: Some("splat.layout"),
            bind_group_layouts: &[Some(&layouts.splat_group0), Some(&layouts.splat_group1)],
            immediate_size: 0,
        });

        let splat_depth_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{
            label: Some("splat.depth"),
            layout: Some(&splat_layout),
            module: &splat_shader,
            entry_point: Some("cs_depth"),
            compilation_options: Default::default(),
            cache: None,
        });

         let splat_color_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor{
            label: Some("splat.color"),
            layout: Some(&splat_layout),
            module: &splat_shader,
            entry_point: Some("cs_color"),
            compilation_options: Default::default(),
            cache: None,
        });

        // Pipelines
```

**Replace with:**

```rust
        // Pipelines
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    splat_resolve_group: wgpu::BindGroup,
    splat_depth_pipeline: wgpu::ComputePipeline,
    splat_color_pipeline: wgpu::ComputePipeline,
```

**Replace with:**

```rust
    splat_resolve_group: wgpu::BindGroup,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            splat_resolve_group,
            splat_depth_pipeline,
            splat_color_pipeline,
```

**Replace with:**

```rust
            splat_resolve_group,
```

**Replace-all** `src/engine/gpu/mod.rs` `self.splat_depth_pipeline` → `self.pipelines.splat_depth` (1 hit)

**Replace-all** `src/engine/gpu/mod.rs` `self.splat_color_pipeline` → `self.pipelines.splat_color` (1 hit)

Gate — this is the last edit of the lesson:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
grep -rn 'create_compute_pipeline' src/ | grep -c 'device\.'   # 1
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
                                                                # 106
./docs/_gate.sh
```

## 6. Delete before you move — and what looks dead but is not

Three things near `edges` look dead and are **not**, so they stay:

- **`ribbon_depth` and `glyph_depth`** are gated behind `const INK_DEPTH_PREPASS: bool = false;`,
  so no frame in the gate builds them into a draw. They are the depth prepass for the flat lane
  and the constant is a switch, not a tombstone — flip it and flat ink starts occluding flat ink.
- **`ribbon_solid_depth` and `sphere_depth`** are drawn every frame; the SOLID lane's prepass is
  not gated by that constant.
- **the `time` uniform and its layout** feed exactly one shader (`triangle.wgsl`'s animation
  clock) and look like leftovers from lesson 07. They are bound at group 1 of both triangle
  pipelines; removing the layout removes the binding and the shader stops validating.

The test is not "does this look old". It is `grep -rn '<name>' src/` and the answer being zero.

## 7. Proving nothing changed — three ladders

**Ladder 1, the compiler.** Both targets, `--all-targets` natively so the examples and the
headless harness are type-checked too:

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
```

*What it cannot catch:* anything that type-checks. A desc that binds `l.glyph` where the builder
bound `l.segment` compiles perfectly, and so does a `depth_compare` copied from the wrong builder.
Nor can it see a `#[cfg]`-gated arm on the target you did not build.

**Ladder 2, `--moves`.** The only proof a move took its lines byte-identically: the multiset of
stripped, non-blank lines over {source} ∪ {destinations}, before and after, minus every line the
doc declares as added or removed. One source here (`src/app/scene.rs`), two moves — thin on
purpose, so lesson 48's nine bodies across three files are not the first time you run it.

```bash
python3 docs/_replay_check.py --moves <end-of-44 snapshot> /tmp/w45 docs/46-pipeline-descs.md
```

What the other two ladders miss: a line dropped inside a `#[cfg(target_arch = ...)]` arm, which
compiles and renders exactly the frame you expect.

**Ladder 3, the pixel gate, twice.**

```bash
./docs/_gate.sh && ./docs/_gate.sh
```

64 rows: four mandatory scenes × four configs × two passes, plus four advisory scenes when their
gitignored `.pb` assets are present. Every row is gated on **ink, draw count and object count**;
only `drawings_rotated` is gated on the PPM checksum, because the splat lane is a two-pass atomic
compute rasterizer and which point wins a contested pixel is a race. `lion`, `bunny_cloud`,
`cloud_mix`, `lidar14` and `bunny_drawings` record `nondet(splat)`; `bunny` holds no cloud and
still drifts by one pixel — (625, 220), grey 171 ⇄ 170 — and records `nondet(mesh)`. Both are
exempted in `_GOLDENS.tsv` and neither is your bug. The gate runs each row twice and fails on a
disagreement before comparing to the goldens.

*What it cannot catch:* the whole `VIEWER_NO_DEPTH` path — no golden sets it. That is why §5.4
names both branches by hand.

## 8. What you can now do in one line

Add a wireframe pass over every mesh edge. Before this lesson: a ~70-line copy of
`build_ribbon_pipeline`, a `use` line, a struct field and a call. Now one literal.

**Type all six steps below.** The first three add it, the last three take it back out — a
demonstration, not part of the end state, and the file must be back to 148 lines before §10. Do
**not** undo it with `git checkout`: lesson 46 is not committed, and that would throw it all away.

**8a.** **Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub ribbon_solid: wgpu::RenderPipeline,
```

**Add below it:**

```rust
    pub wireframe: wgpu::RenderPipeline,
```

**8b.** One entry in the list, immediately after the `ribbon_solid` desc. **Find** in
`src/engine/pipelines/mod.rs`:

```rust
                ..PipelineDesc::ink("ribbon.solid", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
```

**Add below it:**

```rust
            wireframe: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::LineStrip,
                depth_compare: wgpu::CompareFunction::Always,
                ..PipelineDesc::ink("wireframe", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
```

Three overrides on top of the ink preset: line topology, no depth test, everything else inherited.

**8c.** Now point one draw at it. **Find** in `src/engine/gpu/mod.rs`:

```rust
                        pass.set_pipeline(&self.pipelines.ribbon_solid);
```

**Replace with:**

```rust
                        pass.set_pipeline(&self.pipelines.wireframe);
```

Run it:

```bash
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- \
    /tmp/wire.ppm assets/scenes/bunny.toml
```

Every mesh edge is now a hairline that ignores depth — the bunny becomes a see-through cage. The
point is the diff, not the picture: **six lines** for a genuinely new pipeline, none of them
anywhere near `create_render_pipeline`.

Now put it back, in reverse order.

**8d.** **Find** in `src/engine/gpu/mod.rs`:

```rust
                        pass.set_pipeline(&self.pipelines.wireframe);
```

**Replace with:**

```rust
                        pass.set_pipeline(&self.pipelines.ribbon_solid);
```

**8e.** **Find** in `src/engine/pipelines/mod.rs` (the anchor carries the line above, so no blank
line is left behind):

```rust
            }),
            wireframe: build(device, t, &PipelineDesc {
                topology: wgpu::PrimitiveTopology::LineStrip,
                depth_compare: wgpu::CompareFunction::Always,
                ..PipelineDesc::ink("wireframe", RIBBON, &[&l.mvp, &l.line, &l.instance, &l.segment])
            }),
```

**Replace with:**

```rust
            }),
```

**8f.** **Find** in `src/engine/pipelines/mod.rs`:

```rust
    pub ribbon_solid: wgpu::RenderPipeline,
    pub wireframe: wgpu::RenderPipeline,
```

**Replace with:**

```rust
    pub ribbon_solid: wgpu::RenderPipeline,
```

`wc -l src/engine/pipelines/mod.rs` is back to **148**, and `./docs/_gate.sh` is green again. If it
is not, you removed one line too many or too few — 8e and 8f undo 8a and 8b exactly.

## 9. What is deliberately not here

- **Per-family `descs()`.** The fourteen literals and their `include_str!` lines move to the file
  that owns the row they draw: **47** (`arena.rs`), **48** (`segments.rs`, `glyphs.rs`), **49**
  (`splat.rs`, `backdrop.rs`).
- **`GpuCtx`.** `Pipelines::new` takes `&wgpu::Device`. Lesson **46** introduces
  `GpuCtx { device, queue }` and renames the parameter with one Replace-all; the arity stays 3.
- **A WGSL prelude.** Every `.wgsl` still repeats its own struct definitions.
  `PipelineDesc.shader` is a `&str`, which makes a prelude a one-line change at **111**.
- **MRT colour targets.** `PipelineDesc.target` is `Option<Target>`, one attachment. Lessons
  **90** and **114** pin a pipeline to its own attachment through it; a *second simultaneous*
  attachment is a sibling function, not a wider desc (seam S3c, rejected).
- **`Layouts` entries as data.** Nine literal blocks with one shared helper. The entry lists are
  editable, which is the whole ask; a table buys nothing until something generates it.
- **Fixing the `glyph`/`segment` layout alias.** Preserved exactly, comment and all. A body you
  are moving is not a body you are fixing.

## 10. Expected state

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets
./docs/_gate.sh && ./docs/_gate.sh
```

Both gate runs print `gate OK`.

```bash
wc -l src/math.rs src/engine/pipelines/layouts.rs \
      src/engine/pipelines/build.rs src/engine/pipelines/mod.rs \
      src/engine/gpu/mod.rs src/app/scene.rs
```

```text
  123 src/math.rs                        NEW
  181 src/engine/pipelines/layouts.rs    NEW
  215 src/engine/pipelines/build.rs      was 845
  148 src/engine/pipelines/mod.rs        was  80
 2139 src/engine/gpu/mod.rs              was 2447
 1365 src/app/scene.rs                   was 1382
```

```bash
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
grep -c 'device.create_render_pipeline' src/engine/pipelines/build.rs
grep -c 'device.create_compute_pipeline' src/engine/pipelines/build.rs
grep -rc 'create_bind_group_layout' src/engine/pipelines/layouts.rs
grep -rn 'create_bind_group_layout' src/engine/gpu/mod.rs | wc -l
```

```text
106   Gpu fields          (was 116)
1     create_render_pipeline calls      (was 11)
1     create_compute_pipeline calls     (was 2, both inline in Gpu::build)
9     bind-group layouts, all in Layouts::new
0     bind-group layouts left in gpu/mod.rs
```

`Gpu` 116 → 106: nine bind-group layouts became one `layouts` field, and the two compute pipelines
moved next to the fourteen render pipelines. `build.rs` 845 → 215: eleven builders became one
`build`, one `build_compute`, four presets and a struct.

## Recap

> A pipeline is data. Eleven near-identical builders existed because a pipeline was modelled as
> code, and code cannot be spread with `..`: saying "the ribbon recipe with depth writing on" meant
> copying sixty-nine lines to change one. `PipelineDesc` names the eleven settings that vary, four
> presets name the four recipes, and one `build` holds the single `create_render_pipeline` call.
> `Layouts` does the same one level down, and `Pipelines::new(device, t, &l)` is frozen at three
> parameters so no later lesson can add a layout by threading it through fourteen literals. And
> `edges` was deleted rather than relocated, because faithfully moving dead code is how it
> survives a refactor. **The law: a new pipeline is one literal, a new layout is one field. Never
> a new function, never a new parameter.**

## Edited

`src/math.rs` (NEW — matrices, the camera solves, `Bounds`/`Aabb64`) · `src/lib.rs` (one
`pub mod`) · `src/engine/pipelines/layouts.rs` (NEW — nine layouts + `compute_entry`) ·
`src/engine/pipelines/build.rs` (REWRITTEN — `Target`, `PipelineDesc`, four presets, `build`,
`build_compute`) · `src/engine/pipelines/mod.rs` (fourteen descs + two compute) ·
`src/engine/gpu/mod.rs` (loses the math, the layouts, `splat_entry`, `storage_buffer`, the two
compute pipelines; gains `layouts`) · `src/app/scene.rs` · `src/selftest.rs` ·
`src/shaders/edges.wgsl` (DELETED).

## Reference

Built in nine checkpoints, each compiled and most of them gated:

| checkpoint | what landed |
|---|---|
| 45a | `src/math.rs` — the seven names + `Bounds`/`Aabb64` |
| 45b | `pipelines/layouts.rs` — nine blocks out of `Gpu::build`, `Gpu` gains `layouts` |
| 45c | delete before you move — `edges.wgsl`, `build_edges_pipeline`, `Pipelines.edges`, `storage_buffer` |
| 45d1 | `Target`/`PipelineDesc`/presets/`build`/`build_compute`; `Pipelines::new` 10 params → 3 |
| 45d2 | the four ink descs; four builders deleted (gated with `VIEWER_LINE_STYLE=tubes`) |
| 45d3 | the four opaque descs; four builders deleted |
| 45d4 | the two sheet descs; `build_triangle_pipeline` deleted |
| 45d5 | the four depth-only descs; `build_ink_depth_pipeline` deleted — `build.rs` 845 → 215 |
| 45d6 | the two compute pipelines fold in — `Gpu` 116 → 106 |

45d1-45d5 are merged into step 5.4 so you do not type `PipelineDesc` twice. They are worth reading
if a group refuses to compile — each converts exactly one preset's worth of descs.

`git diff end-of-44..end-of-45 -- session_viewer/src` is the whole lesson as one patch;
`diff -u` any single file against it if a line count comes out wrong.

## Next

Lesson **46** — **the floor is not a lane.** Run the evidence:

```bash
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE '_(cap|capacity):'
```

106 fields, thirteen of them capacities — thirteen buffers written out longhand as a
`(buffer, count, cap)` triple, some forty fields saying one thing thirteen times. A buffer, its
row count and its capacity are one value, and what belongs to no family belongs beneath them all.
