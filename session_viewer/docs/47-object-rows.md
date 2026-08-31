# 47 One row per object

> Lesson [64](64-raycast-meshes.md) asks "which object did the ray hit"; lesson [86](86-ground-grid.md) asks
> "where is this object in the world"; lesson [114](114-id-buffer-picking.md) writes an object id
> into a render target. All three get the same answer from the same place, because after this
> lesson there is exactly ONE table of objects and everything else holds an index into it.
> Nothing you can see changes: same ink, same draw count, same object count, on every scene and
> every config.
> Answer key: the block's snapshot branch `end-of-47`, so
> `git diff end-of-46..end-of-47 -- session_viewer/src` is this whole lesson as one patch.
>
> **Lessons 45-51 move code. Every body you cut is pasted byte-identical except for path
> re-roots inside ONE file; if you find yourself improving a line while moving it, stop — the
> deferral list at the end says which lesson owns that change.**

## 1. Why this seam

### 1a. The evidence — run it on your own tree

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -c 'arena_'
grep -c 'instance_id' src/engine/gpu/mod.rs src/app/scene.rs
grep -cE 't\.(objects|object_bounds|object_spacing|verts|vids|idx)' src/app/scene.rs
grep -c 'pub fn append_index_run' -A6 src/engine/gpu/buffers.rs
```

```text
86   fields on Gpu
13   of them begin `arena_`
 3   `instance_id` in gpu/mod.rs, 18 in app/scene.rs
48   sites in the walk that write one of six Upload columns by name
 1   append_index_run — at SIX parameters, five of which are one value
```

Two shapes are hiding in those 86 fields, and both are the same mistake made twice.

The first is the **object table**. Eleven fields — `instances`, `last_origin`, `objects_base`,
`base_f32`, `bounded_rows`, `object_bounds_world`, `inside`, `instance_buffer`, `instance_rows`,
`instance_cap`, `instance_bind_group` — plus the `last_rebase_ms` throttle that only they use.
Every one of them is indexed by the same row number. Add an object and you must push to six
vectors in the right order; forget one and every later row reads the previous row's bounds, with
no error anywhere. Four of the eleven are the `(buffer, rows-on-GPU, capacity, bind group)`
quadruple lesson 46 already named.

The second is the **arena**: thirteen fields for one vertex table and three index runs, where the
runs are three copies of `(buffer, count, cap)` — the exact triple `buffers.rs` typed as `GrowBuf`
at lesson 46 and then left with `#[allow(dead_code)]` on it, because nothing had been folded into
it yet.

And the walk writes both by naming their columns one at a time: forty-eight sites reaching into an
`Upload` with nineteen flat columns, free to push a vertex without its instance id.

### 1b. The law this enforces, stated as what it forbids

**F2 — a family may not build or renumber an object row.** There is exactly one
`(model, tint, flags)` per guid, `objects.rs` owns it, and everything else — a segment row, a
glyph row, a vertex in the arena, a splat record — carries an `instance_id` that *indexes* it.
No family may push to it, reorder it, or keep a second copy of any part of it.

That is testable, and the test is a grep: after this lesson, `arena_ibo`, `objects_base` and
`bounded_rows` appear in exactly one file.

### 1c. The rejected alternative

The obvious cut is to let `Instance` live inside `objects.rs` — one file, the row and its table,
nothing else to open. Do not make it. `Instance` is declared FIVE more times, in five `.wgsl`
files, and those copies are the ones that break: a field added on the Rust side reads the next
field's bytes on the GPU side, silently, at the right size and in the wrong place. Splitting the
row into its own file gives that hazard a home and a **test**, and it puts the flag-bit table
where a shader author looks for it. Fold it into `objects.rs` and lesson 114, which adds a
sixth shader reading `instances[]`, has nowhere to add itself to.

## 2. Where the code lives after this lesson

| symbol | today's home | new home | who may touch it |
|---|---|---|---|
| `Instance`, `FLAG_*`, `REANCHOR_*` | `gpu/mod.rs` | `gpu/instance.rs` | anyone may READ a row; only `objects.rs` writes one |
| `instances`, `objects_base`, `base_f32` | `Gpu` | `objects::InstanceTable` | `objects.rs` only |
| `bounded_rows`, `object_bounds_world`, `inside` | `Gpu` | `objects::InstanceTable` | `objects.rs` only |
| `instance_buffer/_rows/_cap/_bind_group` | `Gpu` | `objects::InstanceTable` | `objects.rs`; `Gpu` reads `.buffer` for the splat groups |
| `rebase_anchor`, `rebuild_instances`, `update_inside_flags` | `impl Gpu` | `impl InstanceTable` | `Gpu` forwards, and owns the `splat_state` invalidation |
| `Upload.objects/_bounds/_spacing` | `Upload` | `Upload.obj: ObjectRows` | the walk writes, `InstanceTable::append` reads |
| the 13 `arena_*` fields | `Gpu` | `arena::Arena` | `arena.rs` only |
| `Upload.verts/vids/idx/idx_print/idx_text` | `Upload` | `Upload.arena: ArenaRows` | the walk writes, `Arena::append` reads |
| `Pipelines.triangle`, `.triangle_sheet`, `TRIANGLE` | `pipelines/mod.rs` | `arena::Pipes` + `arena::descs` | `arena.rs` only |
| `INSTANCE_ID_ATTRIBS`, `instance_id_layout` | `pipelines/build.rs` | `gpu/arena.rs` | `arena.rs` only — `vids` is its buffer |

```text
                      +-------------------------------+
   the walk  --rows-->|  Upload { obj, arena, ... }   |
                      +-------------------------------+
                            |               |
                    &ObjectRows          &ArenaRows
                            v               v
        +---------------------+     +----------------------+
        | objects.rs          |     | arena.rs             |
        |  InstanceTable      |<-id-|  Arena  (3 GrowBufs) |
        |  Instance rows      |     |  Pipes{triangle,     |
        |  rebase / inside    |     |        sheet}        |
        +---------------------+     +----------------------+
                  |                            |
             &GpuCtx down                  &GpuCtx down
             bind_group up                 draws returned up
```

**Exit litmus, grep it when you are done:** `grep -rn 'objects_base\|bounded_rows\|arena_ibo' src/`
names `src/engine/gpu/objects.rs` and `src/engine/gpu/arena.rs` — plus the one doc comment in
`src/math.rs` that still quotes the old field name — and nothing else.

The chain table, as far as this lesson takes it:

| geometry | walk writes | engine sink | family | shader |
|---|---|---|---|---|
| Mesh faces | `verts`+`vids`+`idx` | `Upload.arena` | `Arena` (Solid) | `triangle.wgsl` |
| PDF fills | `idx_print` | `Upload.arena` | `Arena` (Print) | `triangle.wgsl`, depth write off |
| PDF lettering | `idx_text` | `Upload.arena` | `Arena` (Text) | `triangle.wgsl`, depth write off, last |
| every geometry | one `ObjectBase` | `Upload.obj` | `InstanceTable` | `instances[]` in all five |

## 3. Files we touch

| file | what | step | why |
|---|---|---|---|
| `src/engine/gpu/instance.rs` | **NEW**, 159 lines | 4.1 | the row, the flag table, and the test that keeps five shaders honest |
| `src/engine/gpu/objects.rs` | **NEW**, 341 lines | 4.2 | the object table and everything that keeps it current |
| `src/engine/gpu/arena.rs` | **NEW**, 303 lines | 4.3 | the triangle family: rows, buffers, pipelines, draws |
| `src/engine/gpu/buffers.rs` | 140 → 132 | 6.1 | `GrowBuf` stops being dead; `append_index_run` 6 params → 3 |
| `src/engine/gpu/upload.rs` | 110 → 98 | 6.2 | eight flat columns become two groups |
| `src/engine/pipelines/mod.rs` | 148 → 130 | 6.3 | the triangle descs leave for the family that owns them |
| `src/engine/pipelines/build.rs` | 215 → 199 | 4.3 | `instance_id_layout` leaves with them |
| `src/engine/gpu/mod.rs` | 1,691 → 1,335 | 6.4-6.6 | 25 fields become 2 |
| `src/app/scene.rs` | 1,340 → 1,341 | 6.7 | the walk writes into two sinks instead of eight columns |
| `src/engine/gpu/frame.rs` | 177 → 224 | 4.1 | gains `line_uniform_mirror` |
| `src/selftest.rs`, `examples/check_*.rs` | small | 6.8 | the harnesses follow the columns |
| the five `.wgsl` | comments only | 6.8 | the paths they cite moved |

## 4. The three destination files, created first

An empty `impl` compiles, and a file created first makes every later step an append instead of a
splice. Create all three before you cut anything out of `gpu/mod.rs`.

### 4.1 `src/engine/gpu/instance.rs`

Start with the header. It is the file's argument: this type belongs to no family, five shaders
declare it, and a test below is the only thing checking those five.


**Create `src/engine/gpu/instance.rs`** with the header:

```rust
//! `instance.rs` - the object row, and the flag bits every lane reads off it.
//!
//! One `Instance` per object row: 96 B of rebased model matrix, tint, flag word, and the two
//! world-space budgets the ink lanes clamp themselves against. It belongs to no family, because
//! it is what the families POINT AT: every row struct in the program - `CylinderSegment`,
//! `GlyphPoint`, the arena's vertex-id lane, a splat record - carries an `instance_id` that
//! indexes `instances[]`, and five `.wgsl` files declare this struct to read it:
//!
//!   triangle.wgsl · cylinder.wgsl · ribbon.wgsl · sphere.wgsl · glyph.wgsl
//!
//! Those five copies are the only place in the viewer where the same layout is written twice in
//! two languages, and nothing in the toolchain checks them: a field added on one side reads the
//! next field's bytes on the other, silently, at the right size and in the wrong place. The
//! `instance_mirror` test at the bottom of this file is that check. Add a field here, add it to
//! all five, or the test names the file you forgot.
//!
//! The TABLE of these rows - building it, rebasing it, uploading it - is `objects.rs`.
```

**How to type a Move.** This is the first lesson in the curriculum that uses it, and it is worth
one paragraph. `**Move** <file A> <first line> **through** <last line> **to** <file B> **at the
end**` cuts WHOLE LINES: `first line` must match exactly once in A, and `last line` is the first
line at or after it that matches exactly. Nothing is retyped, so a Move cannot introduce a typo —
which is the entire reason to prefer it over copy-and-paste for a body you are not changing.

The re-anchor constants first. They are the object table's own tuning and nothing else reads them:


**Move** `src/engine/gpu/mod.rs` `/// Re-anchor distance: the instance table is rebased about a snapped anchor.` **through** `const REANCHOR_MAX: f64 = 1.0e5;` **to** `src/engine/gpu/instance.rs` **at the end**

**Replace-all** `src/engine/gpu/instance.rs` `const` -> `pub(crate) const` (2 hits)

The block that arrived is two drafts of one paragraph stacked on the first constant, with nothing
on the second. Give each its own.

**Find** in `src/engine/gpu/instance.rs`:

```rust
/// Re-anchor distance: the instance table is rebased about a snapped anchor.
/// The camera can drift this far (mm) before a full rebuild.
/// Within it, pan/zoon only changes the view matrix.
/// f32 error at 1e5 mm from the achor = 6e-3 mm - far below a pixel.
/// Re-anchor threshold, WORLD units (mm): a quarter of the current view distance, so a zoomed-out
/// pan does not rebuild constantly while a zoomed-IN pan re-anchors early enough that world
/// coordinates never regain the magnitude that eats f32 precision. Clamped to a sane band.
pub(crate) const REANCHOR_MIN: f64 = 1.0e3;
pub(crate) const REANCHOR_MAX: f64 = 1.0e5;
```

**Replace with:**

```rust
/// Re-anchor threshold band, WORLD units (mm). The instance table is rebased about a snapped
/// anchor, and the camera may drift a quarter of its view distance - clamped into this band -
/// before a full rebuild. Within the threshold, pan and zoom only change the view matrix. f32
/// error at 1e5 mm from the anchor is 6e-3 mm, far below a pixel.
pub(crate) const REANCHOR_MIN: f64 = 1.0e3;
/// The upper clamp. A zoomed-OUT pan must not rebuild constantly, but world coordinates must
/// never regain the magnitude that eats f32 precision either.
pub(crate) const REANCHOR_MAX: f64 = 1.0e5;
```

Now the row itself. `#[repr(C)]` appears four times in `gpu/mod.rs`, so it cannot be a Move's
first line — the Move takes the struct and the impl, and the two attribute lines are re-typed
here and deleted there. That asymmetry is the one thing to watch when you cut a region: the
anchor has to be **unique**, not merely correct.


**Move** `src/engine/gpu/mod.rs` `pub struct Instance {` **through** `}` **to** `src/engine/gpu/instance.rs` **at the end**

**Move** `src/engine/gpu/mod.rs` `impl Instance {` **through** `}` **to** `src/engine/gpu/instance.rs` **at the end**

**Find** in `src/engine/gpu/instance.rs`:

```rust
pub struct Instance {
```

**Add above it:**

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
```

`objects.rs` builds and rebases these rows, so the fields have to be visible to it. Six one-line
edits, and they are the whole reason `Instance` can leave `Gpu`'s file at all:


**Find** in `src/engine/gpu/instance.rs`:

```rust
    model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
```

**Replace with:**

```rust
    pub(crate) model: [f32; 16], // 64 B - column-major, from Xform::to_f32()
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    color: [f32; 4], // 16 B
```

**Replace with:**

```rust
    pub(crate) color: [f32; 4], // 16 B
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    flags: u32, // 4 B - reserved (selection)
```

**Replace with:**

```rust
    /// The flag word. Nine bits, and it is only readable if the whole word is on one screen -
    /// bit 0 and bits 6-8 are the budget, three free bits before it needs a second word.
    ///
    /// | bit | const | set by | read by |
    /// |---|---|---|---|
    /// | 0 | (reserved: FLAG_SELECTED) | - | - |
    /// | 1 | `FLAG_HIDDEN` | the tree's visibility toggle | every draw, CPU-side |
    /// | 2 | `FLAG_INSIDE` | `InstanceTable::update_inside_flags`, per frame | ribbon.wgsl, cylinder.wgsl |
    /// | 3 | `FLAG_PRINT` | the walk, from a zero edge width | triangle.wgsl |
    /// | 4 | `FLAG_OPEN` | the walk, from `Mesh::is_closed()` | ribbon.wgsl, cylinder.wgsl |
    /// | 5 | `FLAG_SHEET` | the walk's per-file planar sweep | ribbon.wgsl |
    /// | 6-8 | free | | |
    ///
    /// A new bit is a row here, a `pub const` below, and the matching `const FLAG_X = Nu;` in
    /// every shader that reads it - `instance_mirror` checks the struct, never the bit values.
    pub(crate) flags: u32, // 4 B
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    extent: f32, // 4 B
```

**Replace with:**

```rust
    pub(crate) extent: f32, // 4 B
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    /// Vertex spacing in world units (see `Upload::object_spacing`). The ink lanes drop
```

**Replace with:**

```rust
    /// Vertex spacing in world units (see `ObjectRows::spacing`). The ink lanes drop
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    spacing: f32, // 4 B
```

**Replace with:**

```rust
    pub(crate) spacing: f32, // 4 B
```

**Find** in `src/engine/gpu/instance.rs`:

```rust
    _pad: u32, // 4 B - pad the row to 96 B (storage array stride)
```

**Replace with:**

```rust
    pub(crate) _pad: u32, // 4 B - pad the row to 96 B (storage array stride)
```

Last, the parts that are NEW: the nine-bit flag table on one screen, the WGSL struct parser, and
the mirror test. Nine bits, six used, three free — a flag word is only readable if the whole word
is in one place, and "which bit is free" is a question this table answers in a second.


**Find** in `src/engine/gpu/instance.rs`:

```rust
    pub const FLAG_SHEET: u32 = 1 << 5;
}
```

**Add below it:**

```rust

/// Pull one `struct <name> { .. }` out of a `.wgsl` source as `(field, type)` pairs, comments and
/// blank lines dropped. Shared by `instance_mirror` here and `line_uniform_mirror` in `frame.rs`:
/// the two structs the CPU and the GPU both declare are the two that need it, and one parser is
/// easier to trust than two.
#[cfg(test)]
pub(crate) fn wgsl_fields(src: &str, name: &str) -> Vec<(String, String)> {
    let head = format!("struct {name}");
    let start = src.find(&head).unwrap_or_else(|| panic!("no `{head}` in this shader"));
    let body = &src[start + src[start..].find('{').unwrap() + 1..];
    let body = &body[..body.find('}').expect("unterminated struct")];
    body.lines()
        // Comment FIRST, then commas: `CylinderSegment` packs `p0x: f32, p0y: f32, p0z: f32,`
        // onto one line, and a trailing `// ... always draw.` would otherwise split into junk.
        .map(|l| l.split("//").next().expect("split always yields one element"))
        .flat_map(|l| l.split(','))
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (f, t) = l.split_once(':').unwrap_or_else(|| panic!("not a field: `{l}`"));
            (f.trim().to_string(), t.trim().to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `.wgsl` that declares `struct Instance`. The list is the contract: a sixth shader
    /// that reads `instances[]` adds itself here.
    const MIRRORS: [(&str, &str); 5] = [
        ("triangle.wgsl", include_str!("../../shaders/triangle.wgsl")),
        ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
        ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
        ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
        ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
    ];

    /// The Rust row, field by field, with the WGSL type each one must be declared as. `_pad` has
    /// no WGSL counterpart: the shader language rounds the struct up to its 16 B alignment by
    /// itself, which is exactly the byte the Rust side has to spell out.
    #[test]
    fn instance_mirror() {
        assert_eq!(std::mem::size_of::<Instance>(), 96, "the storage array stride is 96 B");
        assert_eq!(std::mem::offset_of!(Instance, color), 64);
        assert_eq!(std::mem::offset_of!(Instance, flags), 80);
        assert_eq!(std::mem::offset_of!(Instance, extent), 84);
        assert_eq!(std::mem::offset_of!(Instance, spacing), 88);

        let want: Vec<(String, String)> = [
            ("model", "mat4x4<f32>"),
            ("color", "vec4<f32>"),
            ("flags", "u32"),
            ("extent", "f32"),
            ("spacing", "f32"),
        ]
        .iter()
        .map(|(f, t)| (f.to_string(), t.to_string()))
        .collect();

        for (file, src) in MIRRORS {
            assert_eq!(
                wgsl_fields(src, "Instance"), want,
                "{file} declares `Instance` differently from instance.rs - a field added on one \
                 side reads the next field's bytes on the other",
            );
        }
    }
}
```

And the second mirror, in `frame.rs`. `LineUniform` is declared by five shaders too — but NOT
the same five: `grid.wgsl` reads the pen and no object row, `triangle.wgsl` reads the object row
and no pen. It reuses the parser from `instance.rs`, because one parser is easier to trust than
two.


**Find** in `src/engine/gpu/frame.rs`:

```rust
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::env::var("VIEWER_THICKNESS").ok().and_then(|v| v.parse().ok()).unwrap_or(2.0)
    }
}
```

**Add below it:**

```rust

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::gpu::instance::wgsl_fields;

    /// Every `.wgsl` that declares `struct LineUniform`. Five of them, and NOT the same five
    /// that declare `Instance`: grid.wgsl reads the pen and no object row, triangle.wgsl reads
    /// the object row and no pen.
    const MIRRORS: [(&str, &str); 5] = [
        ("grid.wgsl", include_str!("../../shaders/grid.wgsl")),
        ("cylinder.wgsl", include_str!("../../shaders/cylinder.wgsl")),
        ("ribbon.wgsl", include_str!("../../shaders/ribbon.wgsl")),
        ("sphere.wgsl", include_str!("../../shaders/sphere.wgsl")),
        ("glyph.wgsl", include_str!("../../shaders/glyph.wgsl")),
    ];

    /// The uniform is 48 B on both sides, and it gets there DIFFERENTLY: Rust spells out a
    /// trailing `_pad1`, WGSL spells out three scalar `eye_*` fields because a `vec3<f32>` there
    /// would align to 16 and shove `anchor` four bytes down. So the two field lists are not the
    /// same list, and the test checks each side against what it must be, plus the offsets that
    /// are the actual contract.
    #[test]
    fn line_uniform_mirror() {
        assert_eq!(std::mem::size_of::<LineUniform>(), 48);
        assert_eq!(std::mem::offset_of!(LineUniform, eye), 20);
        assert_eq!(std::mem::offset_of!(LineUniform, anchor), 32);

        let want: Vec<(String, String)> = [
            ("thickness", "f32"), ("proj_y", "f32"), ("ortho_h", "f32"),
            ("vp_h", "f32"), ("vp_w", "f32"),
            ("eye_x", "f32"), ("eye_y", "f32"), ("eye_z", "f32"),
            ("anchor", "vec3<f32>"),
        ]
        .iter()
        .map(|(f, t)| (f.to_string(), t.to_string()))
        .collect();

        for (file, src) in MIRRORS {
            assert_eq!(
                wgsl_fields(src, "LineUniform"), want,
                "{file} declares `LineUniform` differently from frame.rs - `anchor` lands at a \
                 different offset and the pen reads the eye's bytes",
            );
        }
    }
}
```

Clean up what the two Moves left behind in `gpu/mod.rs` — the orphaned attribute lines. Note the
Find carries the banner line under them: a **Delete** would leave a blank line where the anchor
was, so the way to remove lines cleanly is to Find them WITH a neighbour and Replace with the
neighbour alone.


**Find** in `src/engine/gpu/mod.rs`:

```rust
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]


//////////////////////////////////////////////////////////////////////////////////////////////////
```

**Replace with:**

```rust
//////////////////////////////////////////////////////////////////////////////////////////////////
```

**Gate.**

```bash
cargo check --target wasm32-unknown-unknown --lib      # 0 errors
cargo xtest                                            # 2 passed
```

Two tests, where `src/` had none. Prove they bite before you trust them: add a line
`drift: f32,` to `struct Instance` in `src/shaders/triangle.wgsl`, run `cargo xtest` again, and
read what it says — it names the file. Then take the line out.

### 4.2 `src/engine/gpu/objects.rs`

The whole table, in one file. The bodies are `set_scene`'s object block, `rebase_anchor`,
`rebuild_instances` and `update_inside_flags`, re-rooted onto the table's own field names —
`self.instances` → `self.rows`, `self.objects_base` → `self.base`,
`self.object_bounds_world` → `self.bounds_world`, `self.instance_buffer` → `self.buffer`. They
leave `gpu/mod.rs` in step 6.5; create the destination now.


**Create `src/engine/gpu/objects.rs`**

```rust
//! `objects.rs` - the object table: one row per guid, and everything that keeps it current.
//!
//! `instance.rs` owns the ROW. This owns the TABLE, and the table is the seam the whole engine
//! hangs on: every family draws rows that carry an `instance_id`, and that id is an index into
//! exactly this vector. A family never builds an object row and never renumbers one.
//!
//! Three vectors arrive from the walk (`ObjectRows`, a group of `Upload`) and seven live here:
//!
//! ```text
//!   walk ->  ObjectRows { rows: ObjectBase, bounds: local AABB, spacing }
//!              |  append()
//!              v
//!   base ------+--> base_f32   (the 13 floats a rebase does NOT touch, cast once)
//!              +--> bounds_world (the local AABB through the TRUE transform)
//!              +--> bounded_rows (which rows have one - the only rows the eye test walks)
//!              +--> inside      (last frame's FLAG_INSIDE, for change detection)
//!              v
//!   rows: Vec<Instance>  --write_buffer-->  buffer -> bind_group -> group(2) of five shaders
//! ```
//!
//! `base` holds f64 world transforms; `rows` holds f32 rebased ones. That split is the whole
//! reason this file exists: an f32 matrix cannot hold a coordinate 100 m from the origin AND a
//! millimetre of detail, so the GPU is only ever shown coordinates measured from an anchor near
//! the camera, and the true placement stays here in f64 to be re-differenced when the anchor
//! moves. Lose `base` and the scene cannot be re-anchored; lose the anchor and it jitters.

use session_rust::{Point, Xform};

use crate::engine::pipelines::layouts::Layouts;
use crate::math::{Mat4, eye_from_view_proj, mat_to_f32};

use super::buffers::{GpuCtx, append_rows, mk_rows_group, zeroed_buffer};
use super::instance::{Instance, REANCHOR_MAX, REANCHOR_MIN};

/// The two labels the instance table is built and rebound under. Spelled once: a grown buffer
/// must keep its name, or the binding-size error a scene throws on crossing a cap is anonymous.
const BUFFER_LABEL: &str = "instance.buffer";
const GROUP_LABEL: &str = "instances.bind_group";

/// The one row an empty table keeps: identity placement, mid grey, no flags. WebGPU zeroes the
/// buffer and `on_gpu` stays 0, so it is never drawn - it exists because wgpu cannot bind a
/// 0-byte buffer, and the first `append` clears it.
fn placeholder_row() -> Instance {
    Instance { model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0 }
}

/// One object's TRUE placement, in world units and f64 - the row the walk writes and the row a
/// rebase reads. It was a `(Mat4, [f32; 4], u32)` tuple, indexed `.0`/`.1`/`.2` in eleven places
/// across two files; the flags field in particular was `.2 |= Instance::FLAG_PRINT`, which reads
/// as nothing at all.
pub struct ObjectBase {
    pub model: Mat4,
    pub color: [f32; 4],
    pub flags: u32,
}

/// The object group of `Upload`: the three columns the walk fills per object, aligned by row.
///
/// They are one group because they are written together - a producer that pushes a row without
/// pushing its bounds and its spacing shifts every later row's data by one - and `append` below
/// is the only reader, which is what a sink means.
pub struct ObjectRows {
    pub rows: Vec<ObjectBase>,
    /// Mesh-LOCAL AABB per row. None for linework/points/clouds: only the solid lane's facing
    /// cull needs it (see `Instance::FLAG_INSIDE`).
    pub bounds: Vec<Option<([f32; 3], [f32; 3])>>,
    /// Vertex spacing per row, world units. 0 = unknown (linework, points, clouds), which the
    /// ink lanes read as "never density-cull".
    pub spacing: Vec<f32>,
}

impl Default for ObjectRows {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjectRows {
    pub fn new() -> Self {
        Self { rows: Vec::new(), bounds: Vec::new(), spacing: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// The GPU-side object table, plus the CPU state a rebase needs.
pub struct InstanceTable {
    /// What the GPU sees: rebased about `last_origin`, f32, uploaded whole on a rebase and
    /// appended to on a new file.
    rows: Vec<Instance>,
    /// The TRUE world placements, f64. Never uploaded.
    base: Vec<ObjectBase>,
    /// `base`'s rotation/scale cast to f32 ONCE, here, instead of per re-anchor: a rebase then
    /// re-patches three floats per row instead of casting sixteen. At 210k objects that is the
    /// difference between a 20 ms CPU loop and a copy.
    base_f32: Vec<[f32; 16]>,
    /// Per-row WORLD AABB - the local `ObjectRows::bounds` through the true transform.
    bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    /// The rows that HAVE one. Derived from `bounds_world`, so the two are cleared together.
    bounded_rows: Vec<u32>,
    /// Last frame's FLAG_INSIDE per row, so an unchanged frame uploads nothing.
    inside: Vec<bool>,
    pub(super) buffer: wgpu::Buffer,
    /// Rows already ON the buffer - the base for the next append.
    on_gpu: u32,
    cap: u64,
    pub bind_group: wgpu::BindGroup,
    /// The anchor `rows` is currently rebased about. None = the next frame must rebuild.
    last_origin: Option<Point>,
    /// Throttle. A 210k-row rebase costs ~25 ms and one per frame is the motion jank the
    /// constant-quality rule forbids.
    last_rebase_ms: f64,
}

impl InstanceTable {
    pub fn new(device: &wgpu::Device, layouts: &Layouts) -> Self {
        // COPY_SRC because the table GROWS by appending: when it outgrows its buffer the prefix
        // is copied GPU-side into the bigger one, and a buffer without COPY_SRC cannot be the
        // source of that copy.
        let buffer = zeroed_buffer(
            device,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let bind_group = mk_rows_group(device, &layouts.instance, "instances.bind_group", &buffer);
        Self {
            // The scene-shaped state starts as an empty placeholder. WebGPU zero-initializes
            // buffers and `on_gpu` is 0, so the first frame draws nothing; the first `append`
            // clears this row and starts the real table.
            rows: vec![Instance {
                model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0,
            }],
            base: Vec::new(),
            base_f32: Vec::new(),
            bounds_world: Vec::new(),
            bounded_rows: Vec::new(),
            inside: Vec::new(),
            buffer,
            on_gpu: 0,
            cap: 1,
            bind_group,
            last_origin: None,
            last_rebase_ms: 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// One rebased row, for a lane that needs its model/flags/spacing without touching the table.
    pub fn row(&self, i: usize) -> Option<&Instance> {
        self.rows.get(i)
    }

    /// The anchor `rows` is currently rebased about, for anything drawn OUTSIDE the object table
    /// - the grid, the axes - which has to subtract the same origin or it drifts away.
    pub fn anchor(&self) -> Option<&Point> {
        self.last_origin.as_ref()
    }

    /// This row's world AABB, if it has one. The only way out of `bounds_world`.
    pub fn bounds_world(&self, row: usize) -> Option<([f64; 3], [f64; 3])> {
        self.bounds_world.get(row).copied().flatten()
    }

    /// Turn the NEW object rows into instance rows and send them.
    ///
    /// `up.rows` is the ONE table the walk keeps cumulative - the bounds sweep and the per-file
    /// sheet pass both index it by global row - so this is the one lane that gets a full table
    /// every time instead of a delta. Only the rows past `self.base.len()` are converted and
    /// sent: cloning 148k rows per file was 22 MB of memcpy and a full re-upload, for a tail
    /// that had not changed since the file before.
    pub fn append(&mut self, ctx: &GpuCtx, layouts: &Layouts, up: &ObjectRows) {
        let base = self.base.len();
        if base == 0 {
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.rows.clear();
            self.on_gpu = 0;
        }
        debug_assert_eq!(up.rows.len(), up.bounds.len());
        debug_assert!(up.rows.len() >= base, "the object table only ever grows");
        self.base.extend(up.rows[base..].iter().map(|o| ObjectBase { model: o.model, color: o.color, flags: o.flags }));
        self.base_f32.extend(up.rows[base..].iter().map(|o| mat_to_f32(&o.model)));
        self.bounds_world.extend(up.rows[base..].iter().zip(&up.bounds[base..]).map(|(o, b)| {
            let m = &o.model;
            b.map(|(lo, hi)| {
                // World AABB of the local box: the 8 corners through the true transform.
                // Conservative for rotated placements - FLAG_INSIDE is a hint, not a cull.
                let xp = |x: f64, y: f64, z: f64| [
                    m[0] * x + m[4] * y + m[8] * z + m[12],
                    m[1] * x + m[5] * y + m[9] * z + m[13],
                    m[2] * x + m[6] * y + m[10] * z + m[14],
                ];
                let mut wlo = [f64::INFINITY; 3];
                let mut whi = [f64::NEG_INFINITY; 3];
                for c in 0..8 {
                    let p = xp(
                        (if c & 1 == 0 { lo[0] } else { hi[0] }) as f64,
                        (if c & 2 == 0 { lo[1] } else { hi[1] }) as f64,
                        (if c & 4 == 0 { lo[2] } else { hi[2] }) as f64,
                    );
                    for k in 0..3 { wlo[k] = wlo[k].min(p[k]); whi[k] = whi[k].max(p[k]); }
                }
                (wlo, whi)
            })
        }));
        self.inside.resize(self.base.len(), false);
        self.bounded_rows = self.bounds_world.iter().enumerate().filter_map(|(i, b)| b.map(|_| i as u32)).collect();
        // `bounds_world` was just extended above, so each row's extent comes from the same
        // AABB FLAG_INSIDE uses. The diagonal, not an axis: a flat sheet has a zero-thickness axis
        // and would clamp its ink lift to nothing.
        let bounds = &self.bounds_world;
        self.rows.extend(up.rows[base..].iter().enumerate().map(|(i, o)| Instance {
            model: mat_to_f32(&o.model),
            color: o.color,
            flags: o.flags,
            extent: bounds.get(base + i).and_then(|b| *b).map_or(0.0, |(lo, hi)| {
                ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
            }),
            spacing: up.spacing.get(base + i).copied().unwrap_or(0.0),
            _pad: 0,
        }));

        if self.rows.is_empty(){
            self.rows.push(Instance {model: Xform::identity().to_f32(), color: [0.5,0.5,0.5,1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0 });
        }

        let mut on_gpu = self.on_gpu;
        let fresh = &self.rows[on_gpu as usize..];
        if append_rows(ctx, "instance.buffer", &mut self.buffer, &mut on_gpu, &mut self.cap, fresh) {
            self.bind_group = mk_rows_group(&ctx.device, &layouts.instance, "instances.bind_group", &self.buffer);
        }
        self.on_gpu = on_gpu;

        // The table just grew, so the anchor it was rebased about no longer covers every row.
        self.last_origin = None;
    }

    /// The anchor the instance table is rebased about, and whether this call rebuilt it.
    ///
    /// A full rebuild (42 000 x at stress scale) runs only when the camera target strays past
    /// `thresh` - a quarter of the view distance, clamped into [`REANCHOR_MIN`, `REANCHOR_MAX`].
    /// Orbit never moves the target, and pan/zoom within the budget just changes the view matrix.
    ///
    /// `origin` and `view_dist` are both in WORLD units (mm) - the same units as the instance
    /// table's translations. Feeding metres here (the camera's internal unit) makes the subtract
    /// in `rebuild` a no-op at 1/1000 scale, which silently turns camera-relative rendering off:
    /// the symptom is geometry that jitters and then clips away entirely as you zoom in, because
    /// the f32 mvp is differencing two large world magnitudes.
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
        // Between rebuilds the old anchor stays valid - it is just farther from the eye than the threshold likes, which costs f32 precision
        // only past the threshold distance, never a wrong image.
        let now = crate::engine::performance::now_ms();
        let rebuilt = need && (now - self.last_rebase_ms > 200.0 || self.last_origin.is_none());
        if rebuilt {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }
        (self.last_origin.clone().unwrap(), rebuilt)
    }

    /// Rebase every instance's translation around 'origin' - an f64 subtract against the TRUE world transform in 'base'
    /// Then cast to f32.
    /// 'rows', what GPU actually sees, never holds a coordinate bigger than the camera's distance from 'origin',
    /// no matter how far the scene sits from world (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point){
        self.last_origin = Some(origin.clone());
        for (i, o) in self.base.iter().enumerate() {
            let mut m = self.base_f32[i]; // rotation / scale cast once at set_scene
            m[12] = (o.model[12] - origin[0]) as f32;
            m[13] = (o.model[13] - origin[1]) as f32;
            m[14] = (o.model[14] - origin[2]) as f32;
            self.rows[i].model = m;
        }
        ctx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.rows));
    }

    /// Set FLAG_INSIDE on every row whose world AABB contains the eye, clear it on the rest, and
    /// upload only if something actually flipped.
    pub fn update_inside_flags(&mut self, ctx: &GpuCtx, view_proj: &Xform, scene_min: [f32; 3], scene_max: [f32; 3]) {
        if self.bounded_rows.is_empty(){
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let eye = eye_from_view_proj(view_proj); // anchored world units, like rows[]
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= scene_min[k] as f64 && ew[k] <= scene_max[k] as f64);
        let mut dirty = false;
        for &row in &self.bounded_rows{
            let i = row as usize;
            let b = &self.bounds_world[i];
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
            if self.inside.get(i).copied().unwrap_or(false) == inside {
                continue;
            }
            if let Some(row) = self.rows.get_mut(i) {
                row.flags = if inside { row.flags | Instance::FLAG_INSIDE } else { row.flags & !Instance::FLAG_INSIDE };
            }
            if i < self.inside.len() { self.inside[i] = inside; }
            dirty = true;
        }
        if dirty {
            ctx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.rows));
        }
    }

    /// Rewind the whole table so the next upload writes from row 0 again. The buffer and its
    /// capacity stay - only the counters and the CPU vectors move.
    ///
    /// `bounded_rows` is DERIVED from `bounds_world`, so leaving it behind holds row indices
    /// into a vector that is now empty. `rebuild` hides that by re-walking immediately, but a
    /// scene that is cleared and then DRAWN before the next upload - reload_scene between Clear
    /// and the first File - panics in `update_inside_flags` on the stale rows.
    ///
    /// `last_origin` deliberately STAYS: nothing is drawn from the table until the next `append`,
    /// which nulls it, and the grid still needs a valid anchor to subtract in between.
    pub fn reset(&mut self) {
        self.base.clear();
        self.base_f32.clear();
        self.bounds_world.clear();
        self.bounded_rows.clear();
        self.inside.clear();
        self.rows.clear();
        self.on_gpu = 0;
    }
}
```text
//!   walk ->  ObjectRows { rows: ObjectBase, bounds: local AABB, spacing }
//!              |  append()
//!              v
//!   base ------+--> base_f32   (the 13 floats a rebase does NOT touch, cast once)
//!              +--> bounds_world (the local AABB through the TRUE transform)
//!              +--> bounded_rows (which rows have one - the only rows the eye test walks)
//!              +--> inside      (last frame's FLAG_INSIDE, for change detection)
//!              v
//!   rows: Vec<Instance>  --write_buffer-->  buffer -> bind_group -> group(2) of five shaders
//! ```
//!
//! `base` holds f64 world transforms; `rows` holds f32 rebased ones. That split is the whole
//! reason this file exists: an f32 matrix cannot hold a coordinate 100 m from the origin AND a
//! millimetre of detail, so the GPU is only ever shown coordinates measured from an anchor near
//! the camera, and the true placement stays here in f64 to be re-differenced when the anchor
//! moves. Lose `base` and the scene cannot be re-anchored; lose the anchor and it jitters.

use session_rust::{Point, Xform};

use crate::engine::pipelines::layouts::Layouts;
use crate::math::{Mat4, eye_from_view_proj, mat_to_f32};

use super::buffers::{GpuCtx, append_rows, mk_rows_group, zeroed_buffer};
use super::instance::{Instance, REANCHOR_MAX, REANCHOR_MIN};

/// One object's TRUE placement, in world units and f64 - the row the walk writes and the row a
/// rebase reads. It was a `(Mat4, [f32; 4], u32)` tuple, indexed `.0`/`.1`/`.2` in eleven places
/// across two files; the flags field in particular was `.2 |= Instance::FLAG_PRINT`, which reads
/// as nothing at all.
pub struct ObjectBase {
    pub model: Mat4,
    pub color: [f32; 4],
    pub flags: u32,
}

/// The object group of `Upload`: the three columns the walk fills per object, aligned by row.
///
/// They are one group because they are written together - a producer that pushes a row without
/// pushing its bounds and its spacing shifts every later row's data by one - and `append` below
/// is the only reader, which is what a sink means.
pub struct ObjectRows {
    pub rows: Vec<ObjectBase>,
    /// Mesh-LOCAL AABB per row. None for linework/points/clouds: only the solid lane's facing
    /// cull needs it (see `Instance::FLAG_INSIDE`).
    pub bounds: Vec<Option<([f32; 3], [f32; 3])>>,
    /// Vertex spacing per row, world units. 0 = unknown (linework, points, clouds), which the
    /// ink lanes read as "never density-cull".
    pub spacing: Vec<f32>,
}

impl ObjectRows {
    pub fn new() -> Self {
        Self { rows: Vec::new(), bounds: Vec::new(), spacing: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

/// The GPU-side object table, plus the CPU state a rebase needs.
pub struct InstanceTable {
    /// What the GPU sees: rebased about `last_origin`, f32, uploaded whole on a rebase and
    /// appended to on a new file.
    rows: Vec<Instance>,
    /// The TRUE world placements, f64. Never uploaded.
    base: Vec<ObjectBase>,
    /// `base`'s rotation/scale cast to f32 ONCE, here, instead of per re-anchor: a rebase then
    /// re-patches three floats per row instead of casting sixteen. At 210k objects that is the
    /// difference between a 20 ms CPU loop and a copy.
    base_f32: Vec<[f32; 16]>,
    /// Per-row WORLD AABB - the local `ObjectRows::bounds` through the true transform.
    bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    /// The rows that HAVE one. Derived from `bounds_world`, so the two are cleared together.
    bounded_rows: Vec<u32>,
    /// Last frame's FLAG_INSIDE per row, so an unchanged frame uploads nothing.
    inside: Vec<bool>,
    pub(super) buffer: wgpu::Buffer,
    /// Rows already ON the buffer - the base for the next append.
    on_gpu: u32,
    cap: u64,
    pub bind_group: wgpu::BindGroup,
    /// The anchor `rows` is currently rebased about. None = the next frame must rebuild.
    last_origin: Option<Point>,
    /// Throttle. A 210k-row rebase costs ~25 ms and one per frame is the motion jank the
    /// constant-quality rule forbids.
    last_rebase_ms: f64,
}

impl InstanceTable {
    pub fn new(device: &wgpu::Device, layouts: &Layouts) -> Self {
        // COPY_SRC because the table GROWS by appending: when it outgrows its buffer the prefix
        // is copied GPU-side into the bigger one, and a buffer without COPY_SRC cannot be the
        // source of that copy.
        let buffer = zeroed_buffer(
            device,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let bind_group = mk_rows_group(device, &layouts.instance, "instances.bind_group", &buffer);
        Self {
            // The scene-shaped state starts as an empty placeholder. WebGPU zero-initializes
            // buffers and `on_gpu` is 0, so the first frame draws nothing; the first `append`
            // clears this row and starts the real table.
            rows: vec![Instance {
                model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0,
            }],
            base: Vec::new(),
            base_f32: Vec::new(),
            bounds_world: Vec::new(),
            bounded_rows: Vec::new(),
            inside: Vec::new(),
            buffer,
            on_gpu: 0,
            cap: 1,
            bind_group,
            last_origin: None,
            last_rebase_ms: 0.0,
        }
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    /// One rebased row, for a lane that needs its model/flags/spacing without touching the table.
    pub fn row(&self, i: usize) -> Option<&Instance> {
        self.rows.get(i)
    }

    /// The anchor `rows` is currently rebased about, for anything drawn OUTSIDE the object table
    /// - the grid, the axes - which has to subtract the same origin or it drifts away.
    pub fn anchor(&self) -> Option<&Point> {
        self.last_origin.as_ref()
    }

    /// This row's world AABB, if it has one. The only way out of `bounds_world`.
    pub fn bounds_world(&self, row: usize) -> Option<([f64; 3], [f64; 3])> {
        self.bounds_world.get(row).copied().flatten()
    }

    /// Turn the NEW object rows into instance rows and send them.
    ///
    /// `up.rows` is the ONE table the walk keeps cumulative - the bounds sweep and the per-file
    /// sheet pass both index it by global row - so this is the one lane that gets a full table
    /// every time instead of a delta. Only the rows past `self.base.len()` are converted and
    /// sent: cloning 148k rows per file was 22 MB of memcpy and a full re-upload, for a tail
    /// that had not changed since the file before.
    pub fn append(&mut self, ctx: &GpuCtx, layouts: &Layouts, up: &ObjectRows) {
        let base = self.base.len();
        if base == 0 {
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.rows.clear();
            self.on_gpu = 0;
        }
        debug_assert_eq!(up.rows.len(), up.bounds.len());
        debug_assert!(up.rows.len() >= base, "the object table only ever grows");
        self.base.extend(up.rows[base..].iter().map(|o| ObjectBase { model: o.model, color: o.color, flags: o.flags }));
        self.base_f32.extend(up.rows[base..].iter().map(|o| mat_to_f32(&o.model)));
        self.bounds_world.extend(up.rows[base..].iter().zip(&up.bounds[base..]).map(|(o, b)| {
            let m = &o.model;
            b.map(|(lo, hi)| {
                // World AABB of the local box: the 8 corners through the true transform.
                // Conservative for rotated placements - FLAG_INSIDE is a hint, not a cull.
                let xp = |x: f64, y: f64, z: f64| [
                    m[0] * x + m[4] * y + m[8] * z + m[12],
                    m[1] * x + m[5] * y + m[9] * z + m[13],
                    m[2] * x + m[6] * y + m[10] * z + m[14],
                ];
                let mut wlo = [f64::INFINITY; 3];
                let mut whi = [f64::NEG_INFINITY; 3];
                for c in 0..8 {
                    let p = xp(
                        (if c & 1 == 0 { lo[0] } else { hi[0] }) as f64,
                        (if c & 2 == 0 { lo[1] } else { hi[1] }) as f64,
                        (if c & 4 == 0 { lo[2] } else { hi[2] }) as f64,
                    );
                    for k in 0..3 { wlo[k] = wlo[k].min(p[k]); whi[k] = whi[k].max(p[k]); }
                }
                (wlo, whi)
            })
        }));
        self.inside.resize(self.base.len(), false);
        self.bounded_rows = self.bounds_world.iter().enumerate().filter_map(|(i, b)| b.map(|_| i as u32)).collect();
        // `bounds_world` was just extended above, so each row's extent comes from the same
        // AABB FLAG_INSIDE uses. The diagonal, not an axis: a flat sheet has a zero-thickness axis
        // and would clamp its ink lift to nothing.
        let bounds = &self.bounds_world;
        self.rows.extend(up.rows[base..].iter().enumerate().map(|(i, o)| Instance {
            model: mat_to_f32(&o.model),
            color: o.color,
            flags: o.flags,
            extent: bounds.get(base + i).and_then(|b| *b).map_or(0.0, |(lo, hi)| {
                ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
            }),
            spacing: up.spacing.get(base + i).copied().unwrap_or(0.0),
            _pad: 0,
        }));

        if self.rows.is_empty(){
            self.rows.push(Instance {model: Xform::identity().to_f32(), color: [0.5,0.5,0.5,1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0 });
        }

        let mut on_gpu = self.on_gpu;
        let fresh = &self.rows[on_gpu as usize..];
        if append_rows(ctx, "instance.buffer", &mut self.buffer, &mut on_gpu, &mut self.cap, fresh) {
            self.bind_group = mk_rows_group(&ctx.device, &layouts.instance, "instances.bind_group", &self.buffer);
        }
        self.on_gpu = on_gpu;

        // The table just grew, so the anchor it was rebased about no longer covers every row.
        self.last_origin = None;
    }

    /// The anchor the instance table is rebased about, and whether this call rebuilt it.
    ///
    /// A full rebuild (42 000 x at stress scale) runs only when the camera target strays
    /// REANCHOR_DIST from the current anchor - orbit never moves the target, and pan/zoom within
    /// the budget just changes the view matrix.
    ///
    /// `origin` and `view_dist` are both in WORLD units (mm) - the same units as the instance
    /// table's translations. Feeding metres here (the camera's internal unit) makes the subtract
    /// in `rebuild` a no-op at 1/1000 scale, which silently turns camera-relative rendering off:
    /// the symptom is geometry that jitters and then clips away entirely as you zoom in, because
    /// the f32 mvp is differencing two large world magnitudes.
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
        let now = crate::engine::performance::now_ms();
        let rebuilt = need && (now - self.last_rebase_ms > 200.0 || self.last_origin.is_none());
        if rebuilt {
            self.rebuild(ctx, origin);
            self.last_rebase_ms = now;
        }
        (self.last_origin.clone().unwrap(), rebuilt)
    }

    /// Rebase every instance's translation around 'origin' - an f64 subtract agains the TRUE world transfrom in 'base'
    /// Then cast to f32.
    /// 'rows', what GPU actually sees, never holds a coordinate bigger than the camera's distnace from 'origin',
    /// no matter how fas the scene fists from world (0,0,0).
    fn rebuild(&mut self, ctx: &GpuCtx, origin: &Point){
        self.last_origin = Some(origin.clone());
        for (i, o) in self.base.iter().enumerate() {
            let mut m = self.base_f32[i]; // rotation / scale casr once at set_scene
            m[12] = (o.model[12] - origin[0]) as f32;
            m[13] = (o.model[13] - origin[1]) as f32;
            m[14] = (o.model[14] - origin[2]) as f32;
            self.rows[i].model = m;
        }
        ctx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.rows));
    }

    /// Set FLAG_INSIDE on every row whose world AABB contains the eye, clear it on the rest, and
    /// upload only if something actually flipped.
    pub fn update_inside_flags(&mut self, ctx: &GpuCtx, view_proj: &Xform, scene_min: [f32; 3], scene_max: [f32; 3]) {
        if self.bounded_rows.is_empty(){
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let eye = eye_from_view_proj(view_proj); // anchored world units, like rows[]
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= scene_min[k] as f64 && ew[k] <= scene_max[k] as f64);
        let mut dirty = false;
        for &row in &self.bounded_rows{
            let i = row as usize;
            let b = &self.bounds_world[i];
            let inside = in_scene && b.is_some_and(|(lo, hi)| (0..3).all(|k| ew[k] >= lo[k] && ew[k] <= hi[k]));
            if self.inside.get(i).copied().unwrap_or(false) == inside {
                continue;
            }
            if let Some(row) = self.rows.get_mut(i) {
                row.flags = if inside { row.flags | Instance::FLAG_INSIDE } else { row.flags & !Instance::FLAG_INSIDE };
            }
            if i < self.inside.len() { self.inside[i] = inside; }
            dirty = true;
        }
        if dirty {
            ctx.queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.rows));
        }
    }

    /// Rewind the whole table so the next upload writes from row 0 again. The buffer and its
    /// capacity stay - only the counters and the CPU vectors move.
    ///
    /// `bounded_rows` is DERIVED from `bounds_world`, so leaving it behind holds row indices
    /// into a vector that is now empty. `rebuild` hides that by re-walking immediately, but a
    /// scene that is cleared and then DRAWN before the next upload - reload_scene between Clear
    /// and the first File - panics in `update_inside_flags` on the stale rows.
    pub fn clear(&mut self) {
        self.base.clear();
        self.base_f32.clear();
        self.bounds_world.clear();
        self.bounded_rows.clear();
        self.inside.clear();
        self.rows.clear();
        self.on_gpu = 0;
    }
}
```

Three things in there are NOT a move, and each is deliberate:

- **`ObjectBase` names the tuple.** `Vec<(Mat4, [f32; 4], u32)>` was indexed `.0`/`.1`/`.2` in
  eleven places across two files, and the flags one read `t.objects.last_mut().unwrap().2 |=
  Instance::FLAG_PRINT`. That line is now `.flags |=`.
- **`ObjectRows` is a sink.** The three columns are written together — push a row without its
  bounds and every later row is off by one — so they travel as one value and `append` is their
  only reader.
- **`clear()` clears `base_f32`.** The old `reset_arena` cleared five vectors and forgot this one,
  so after a rebuild `base_f32` was twice as long as `base` and `rebuild` read the PREVIOUS
  scene's rotation and scale. No golden catches it, because a rebuild re-walks identical content
  and the stale rows happen to be equal. It is still a leak and still wrong, and a method called
  `clear` on a struct that owns all six vectors is where it stops being possible.

### 4.3 `src/engine/gpu/arena.rs`

One vertex table, three index runs, two pipelines, three draws.


**Create `src/engine/gpu/arena.rs`**

```rust
//! `arena.rs` - the triangle family: one vertex table, three index runs, `triangle.wgsl`.
//!
//! Every surface in the viewer - mesh faces, BRep faces, a tessellated NURBS patch, a PDF's
//! filled regions, a PDF's lettering - is triangles over ONE shared vertex table, and the only
//! thing that separates them is WHICH INDICES are drawn and in WHAT ORDER:
//!
//! ```text
//!   vbo   RenderVertex  position+normal+colour  ]  one table, appended per file
//!   vids  u32           instance_id per vertex  ]  slot 1, so a vertex knows its object row
//!
//!   solid -> triangle        depth write ON    drawn with the 3D geometry
//!   print -> triangle_sheet  depth write OFF   a page's fills, in document order
//!   text  -> triangle_sheet  depth write OFF   the lettering, LAST of everything
//! ```
//!
//! That is the family contract in one file: the rows (`ArenaRows`), the buffers (`Arena`), the
//! pipelines that read them (`Pipes` + `descs`), and the draws. Nothing outside this file names
//! `triangle.wgsl`, and nothing in this file names a `Geometry::` variant - the walk decides
//! which run an index lands in, the family decides what a run means.

use session_rust::RenderVertex;

use crate::engine::pipelines::layouts::Layouts;
use crate::engine::pipelines::{PipelineDesc, Target, build::build};

use super::buffers::{GpuCtx, GrowBuf, append_index_run, zeroed_buffer};
use super::frame::Binds;

const TRIANGLE: &str = include_str!("../../shaders/triangle.wgsl");

/// The arena group of `Upload`: this file's rows, as the walk hands them over.
///
/// `verts`/`vids` are parallel - one instance id per vertex - and the three index runs all index
/// the SAME vertex table, which is what makes splitting the sheet lanes free: one buffer each,
/// no duplicated geometry.
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    /// Sheet lanes. A PDF's fills are exactly coplanar, so they must NOT arbitrate by depth -
    /// they are split off the solid index run and drawn in document order with depth write off.
    /// `idx_text` is the lettering, drawn LAST of all, after the ink lanes, because a page puts
    /// its text on top of both its hatching and its linework.
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

impl Default for ArenaRows {
    fn default() -> Self {
        Self::new()
    }
}

impl ArenaRows {
    pub fn new() -> Self {
        Self { verts: Vec::new(), vids: Vec::new(), idx: Vec::new(), idx_print: Vec::new(), idx_text: Vec::new() }
    }
}

/// Which index run. The three differ in one pipeline and one draw position; naming them is what
/// keeps `run(Text)` from being `self.arena_ibo_text`, `self.arena_text_count` and
/// `self.arena_text_cap` spelled out at every site.
#[derive(Clone, Copy)]
pub enum IdxLane {
    Solid,
    Print,
    Text,
}

/// The shared vertex table and its three index runs.
pub struct Arena {
    vbo: wgpu::Buffer,
    vids: wgpu::Buffer,
    /// Vertices already on the GPU - the base for the next append, and the row every index is
    /// relative to.
    verts: u32,
    vert_cap: u64,
    solid: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
}

impl Arena {
    pub fn new(device: &wgpu::Device) -> Self {
        // One zeroed row each - wgpu cannot bind a 0-byte buffer, and every count starts at 0 so
        // nothing is drawn from them until real geometry appends.
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        Self {
            vbo: zeroed_buffer(device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu),
            vids: zeroed_buffer(device, "arena.vids", 4, vu),
            verts: 0,
            vert_cap: 1,
            solid: GrowBuf { buf: zeroed_buffer(device, "arena.ibo", 4, iu), count: 0, cap: 1, usage: iu, label: "arena.ibo" },
            print: GrowBuf { buf: zeroed_buffer(device, "arena.ibo.print", 4, iu), count: 0, cap: 1, usage: iu, label: "arena.ibo.print" },
            text: GrowBuf { buf: zeroed_buffer(device, "arena.ibo.text", 4, iu), count: 0, cap: 1, usage: iu, label: "arena.ibo.text" },
        }
    }

    fn run(&self, lane: IdxLane) -> &GrowBuf {
        match lane {
            IdxLane::Solid => &self.solid,
            IdxLane::Print => &self.print,
            IdxLane::Text => &self.text,
        }
    }

    fn run_mut(&mut self, lane: IdxLane) -> &mut GrowBuf {
        match lane {
            IdxLane::Solid => &mut self.solid,
            IdxLane::Print => &mut self.print,
            IdxLane::Text => &mut self.text,
        }
    }

    /// Vertices on the GPU - a COUNT, not the table. `msaa_now` reads it to decide whether the
    /// scene holds solids at all.
    pub fn verts(&self) -> u32 {
        self.verts
    }

    /// Append one file's worth of triangles.
    ///
    /// Like the cloud lane, `up.verts/vids/idx` are a DELTA - the caller clears them after
    /// upload (`Scene::upload_to`), because nothing reads them back: picking goes through the
    /// kernel Meshes in Doc.session, never through these flattened rows.
    ///
    /// Appending rather than rebuilding is worth two separate things. It stops re-sending the
    /// whole arena on every file (six files meant the 64 MB vertex table travelled six times),
    /// and it lets the CPU-side Vecs go, which is ~70 MB of wasm heap that was being held for
    /// the sole purpose of feeding the next rebuild.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        if up.verts.is_empty() {
            // The three index runs are appended BELOW this return, so indices without vertices
            // would vanish - and `Upload::drop_uploaded` frees them right after, with no second
            // chance. The walk always pushes both; this says so out loud.
            debug_assert!(
                up.idx.is_empty() && up.idx_print.is_empty() && up.idx_text.is_empty(),
                "index rows arrived with no vertex rows; this early return would drop them",
            );
            return;
        }
        debug_assert_eq!(up.verts.len(), up.vids.len(), "one instance id per vertex, or slot 1 reads the wrong row");
        let vstride = std::mem::size_of::<RenderVertex>() as u64;
        let need_v = self.verts as u64 + up.verts.len() as u64;
        let need_i = self.solid.count as u64 + up.idx.len() as u64;

        if need_v > self.vert_cap || need_i > self.solid.cap {
            // EXACT fit, not doubling - and unlike `GrowBuf::append` that is deliberate here.
            // This is the biggest table in the viewer (64 MB on the mesh-stress scene, and a
            // lidar scene puts hundreds of MB behind it), so a doubling would cost more VRAM
            // than the re-copy costs time: growth happens once per FILE, not per row.
            let cap_v = need_v.max(self.vert_cap);
            let cap_i = need_i.max(self.solid.cap);
            let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
            let vbo = zeroed_buffer(&ctx.device, "arena.vbo", cap_v * vstride, vu);
            let vids = zeroed_buffer(&ctx.device, "arena.vids", cap_v * 4, vu);
            let ibo = zeroed_buffer(&ctx.device, self.solid.label, cap_i * 4, self.solid.usage);
            if self.verts > 0 {
                // the prefix moves GPU-side; it never travels back through wasm memory
                let mut enc = ctx.device.create_command_encoder(&Default::default());
                enc.copy_buffer_to_buffer(&self.vbo, 0, &vbo, 0, self.verts as u64 * vstride);
                enc.copy_buffer_to_buffer(&self.vids, 0, &vids, 0, self.verts as u64 * 4);
                enc.copy_buffer_to_buffer(&self.solid.buf, 0, &ibo, 0, self.solid.count as u64 * 4);
                ctx.queue.submit([enc.finish()]);
            }
            self.vbo = vbo;
            self.vids = vids;
            self.solid.buf = ibo;
            self.vert_cap = cap_v;
            self.solid.cap = cap_i;
        }

        ctx.queue.write_buffer(&self.vbo, self.verts as u64 * vstride, bytemuck::cast_slice(&up.verts));
        ctx.queue.write_buffer(&self.vids, self.verts as u64 * 4, bytemuck::cast_slice(&up.vids));
        ctx.queue.write_buffer(&self.solid.buf, self.solid.count as u64 * 4, bytemuck::cast_slice(&up.idx));
        self.verts += up.verts.len() as u32;
        self.solid.count += up.idx.len() as u32;

        // The sheet runs grow and append the same way; they index the SAME vertex table, so
        // splitting them costs one buffer each and no duplicated geometry.
        append_index_run(ctx, self.run_mut(IdxLane::Print), &up.idx_print);
        append_index_run(ctx, self.run_mut(IdxLane::Text), &up.idx_text);
    }

    /// Forget what the arena holds, so the next upload writes from row 0 again. The buffers and
    /// their capacity stay - only the counters move - so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.verts = 0;
        self.solid.count = 0;
        self.print.count = 0;
        self.text.count = 0;
    }

    /// Bind the shared vertex table and draw one index run. The CALLER sets the pipeline and the
    /// bind groups, because the three lanes sit at three different points of the frame's order
    /// and that order is the whole reason they are three lanes.
    fn draw(&self, pass: &mut wgpu::RenderPass, lane: IdxLane) {
        let run = self.run(lane);
        pass.set_vertex_buffer(0, self.vbo.slice(..)); // slot 0 - vertices
        pass.set_vertex_buffer(1, self.vids.slice(..)); // slot 1 - per-vertex row ids
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.count, 0, 0..1); // whole scene, one call
    }

    /// The solid faces.
    ///
    /// PRECONDITION: the pipeline and groups 0-2 (mvp, time, instances) are set by the frame
    /// immediately above the call, whether or not this run is empty.
    ///
    /// Returns 1 always, matching its siblings' contract - the frame's draw count is of
    /// PIPELINES SET, not of non-empty runs, and the goldens record that number.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass) -> u32 {
        if self.solid.count > 0 {
            self.draw(pass, IdxLane::Solid);
        }
        1
    }

    /// SHEET FILLS. Same vertex table, depth WRITE off, so a page's exactly coplanar regions
    /// composite in document order instead of flickering over one shared depth value. They still
    /// depth-TEST, so 3D geometry in front of the sheet occludes. Returns the draws it issued.
    ///
    /// PRECONDITION: groups 0-2 are still bound from `draw_faces`'s call site - only the pipeline
    /// changes. `draw_text` below re-binds them, because the ink lanes drawn in between put the
    /// pen uniform in group 1.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.print.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.arena.sheet);
        self.draw(pass, IdxLane::Print);
        1
    }

    /// LETTERING, last of everything. A page paints its text on top of its hatching AND its
    /// linework, so it lands after the ink lanes - the one thing draw order can express that a
    /// depth buffer cannot, since all of it is coplanar at z = 0.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.text.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.arena.sheet);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.time, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        self.draw(pass, IdxLane::Text);
        1
    }
}

/// The family's pipelines. Two, and they differ in ONE field.
pub struct Pipes {
    pub triangle: wgpu::RenderPipeline,
    /// Same program, depth WRITE off: the sheet lanes (print fills, then lettering) composite in
    /// draw order instead of fighting over one coplanar depth value.
    pub sheet: wgpu::RenderPipeline,
}

impl Pipes {
    /// A family builds its own pipelines from the shared layouts. `Pipelines::new` calls this
    /// and never sees `TRIANGLE`, which is what keeps the shader constant in the file that owns
    /// the rows it reads.
    pub fn descs(device: &wgpu::Device, t: Target, l: &Layouts) -> Self {
        Self {
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
            sheet: build(device, t, &PipelineDesc {
                vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                depth_write: false,
                ..PipelineDesc::sheet("triangle.sheet", TRIANGLE, &[&l.mvp, &l.time, &l.instance])
            }),
        }
    }
}
```text
//!   vbo   RenderVertex  position+normal+colour  ]  one table, appended per file
//!   vids  u32           instance_id per vertex  ]  slot 1, so a vertex knows its object row
//!
//!   solid -> triangle        depth write ON    drawn with the 3D geometry
//!   print -> triangle_sheet  depth write OFF   a page's fills, in document order
//!   text  -> triangle_sheet  depth write OFF   the lettering, LAST of everything
//! ```
//!
//! That is the family contract in one file: the rows (`ArenaRows`), the buffers (`Arena`), the
//! pipelines that read them (`Pipes` + `descs`), and the draws. Nothing outside this file names
//! `triangle.wgsl`, and nothing in this file names a `Geometry::` variant - the walk decides
//! which run an index lands in, the family decides what a run means.

use session_rust::RenderVertex;

use crate::engine::pipelines::layouts::Layouts;
use crate::engine::pipelines::{PipelineDesc, Target, build::build};

use super::buffers::{GpuCtx, GrowBuf, append_index_run, zeroed_buffer};
use super::frame::Binds;

const TRIANGLE: &str = include_str!("../../shaders/triangle.wgsl");

/// The arena group of `Upload`: this file's rows, as the walk hands them over.
///
/// `verts`/`vids` are parallel - one instance id per vertex - and the three index runs all index
/// the SAME vertex table, which is what makes splitting the sheet lanes free: one buffer each,
/// no duplicated geometry.
pub struct ArenaRows {
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    /// Sheet lanes. A PDF's fills are exactly coplanar, so they must NOT arbitrate by depth -
    /// they are split off the solid index run and drawn in document order with depth write off.
    /// `idx_text` is the lettering, drawn LAST of all, after the ink lanes, because a page puts
    /// its text on top of both its hatching and its linework.
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
}

impl ArenaRows {
    pub fn new() -> Self {
        Self { verts: Vec::new(), vids: Vec::new(), idx: Vec::new(), idx_print: Vec::new(), idx_text: Vec::new() }
    }
}

/// Which index run. The three differ in one pipeline and one draw position; naming them is what
/// keeps `run(Text)` from being `self.arena_ibo_text`, `self.arena_text_count` and
/// `self.arena_text_cap` spelled out at every site.
#[derive(Clone, Copy)]
pub enum IdxLane {
    Solid,
    Print,
    Text,
}

/// The shared vertex table and its three index runs.
pub struct Arena {
    vbo: wgpu::Buffer,
    vids: wgpu::Buffer,
    /// Vertices already on the GPU - the base for the next append, and the row every index is
    /// relative to.
    verts: u32,
    vert_cap: u64,
    solid: GrowBuf,
    print: GrowBuf,
    text: GrowBuf,
}

impl Arena {
    pub fn new(device: &wgpu::Device) -> Self {
        // One zeroed row each - wgpu cannot bind a 0-byte buffer, and every count starts at 0 so
        // nothing is drawn from them until real geometry appends.
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        Self {
            vbo: zeroed_buffer(device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu),
            vids: zeroed_buffer(device, "arena.vids", 4, vu),
            verts: 0,
            vert_cap: 1,
            solid: GrowBuf { buf: zeroed_buffer(device, "arena.ibo", 4, iu), count: 0, cap: 1, usage: iu, label: "arena.ibo" },
            print: GrowBuf { buf: zeroed_buffer(device, "arena.ibo.print", 4, iu), count: 0, cap: 1, usage: iu, label: "arena.ibo.print" },
            text: GrowBuf { buf: zeroed_buffer(device, "arena.ibo.text", 4, iu), count: 0, cap: 1, usage: iu, label: "arena.ibo.text" },
        }
    }

    pub fn run(&self, lane: IdxLane) -> &GrowBuf {
        match lane {
            IdxLane::Solid => &self.solid,
            IdxLane::Print => &self.print,
            IdxLane::Text => &self.text,
        }
    }

    fn run_mut(&mut self, lane: IdxLane) -> &mut GrowBuf {
        match lane {
            IdxLane::Solid => &mut self.solid,
            IdxLane::Print => &mut self.print,
            IdxLane::Text => &mut self.text,
        }
    }

    /// Vertices on the GPU. `msaa_now` reads it to decide whether the scene holds solids at all.
    pub fn verts(&self) -> u32 {
        self.verts
    }

    /// Append one file's worth of triangles.
    ///
    /// Like the cloud lane, `up.verts/vids/idx` are a DELTA - the caller clears them after
    /// upload (`Scene::upload_to`), because nothing reads them back: picking goes through the
    /// kernel Meshes in Doc.session, never through these flattened rows.
    ///
    /// Appending rather than rebuilding is worth two separate things. It stops re-sending the
    /// whole arena on every file (six files meant the 64 MB vertex table travelled six times),
    /// and it lets the CPU-side Vecs go, which is ~70 MB of wasm heap that was being held for
    /// the sole purpose of feeding the next rebuild.
    pub fn append(&mut self, ctx: &GpuCtx, up: &ArenaRows) {
        if up.verts.is_empty() {
            return;
        }
        debug_assert_eq!(up.verts.len(), up.vids.len(), "one instance id per vertex, or slot 1 reads the wrong row");
        let vstride = std::mem::size_of::<RenderVertex>() as u64;
        let need_v = self.verts as u64 + up.verts.len() as u64;
        let need_i = self.solid.count as u64 + up.idx.len() as u64;

        if need_v > self.vert_cap || need_i > self.solid.cap {
            let cap_v = need_v.max(self.vert_cap);
            let cap_i = need_i.max(self.solid.cap);
            let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
            let vbo = zeroed_buffer(&ctx.device, "arena.vbo", cap_v * vstride, vu);
            let vids = zeroed_buffer(&ctx.device, "arena.vids", cap_v * 4, vu);
            let ibo = zeroed_buffer(&ctx.device, self.solid.label, cap_i * 4, self.solid.usage);
            if self.verts > 0 {
                // the prefix moves GPU-side; it never travels back through wasm memory
                let mut enc = ctx.device.create_command_encoder(&Default::default());
                enc.copy_buffer_to_buffer(&self.vbo, 0, &vbo, 0, self.verts as u64 * vstride);
                enc.copy_buffer_to_buffer(&self.vids, 0, &vids, 0, self.verts as u64 * 4);
                enc.copy_buffer_to_buffer(&self.solid.buf, 0, &ibo, 0, self.solid.count as u64 * 4);
                ctx.queue.submit([enc.finish()]);
            }
            self.vbo = vbo;
            self.vids = vids;
            self.solid.buf = ibo;
            self.vert_cap = cap_v;
            self.solid.cap = cap_i;
        }

        ctx.queue.write_buffer(&self.vbo, self.verts as u64 * vstride, bytemuck::cast_slice(&up.verts));
        ctx.queue.write_buffer(&self.vids, self.verts as u64 * 4, bytemuck::cast_slice(&up.vids));
        ctx.queue.write_buffer(&self.solid.buf, self.solid.count as u64 * 4, bytemuck::cast_slice(&up.idx));
        self.verts += up.verts.len() as u32;
        self.solid.count += up.idx.len() as u32;

        // The sheet runs grow and append the same way; they index the SAME vertex table, so
        // splitting them costs one buffer each and no duplicated geometry.
        append_index_run(ctx, self.run_mut(IdxLane::Print), &up.idx_print);
        append_index_run(ctx, self.run_mut(IdxLane::Text), &up.idx_text);
    }

    /// Forget what the arena holds, so the next upload writes from row 0 again. The buffers and
    /// their capacity stay - only the counters move - so a rebuild costs no allocation.
    pub fn reset(&mut self) {
        self.verts = 0;
        self.solid.count = 0;
        self.print.count = 0;
        self.text.count = 0;
    }

    /// Bind the shared vertex table and draw one index run. The CALLER sets the pipeline and the
    /// bind groups, because the three lanes sit at three different points of the frame's order
    /// and that order is the whole reason they are three lanes.
    fn draw(&self, pass: &mut wgpu::RenderPass, lane: IdxLane) {
        let run = self.run(lane);
        pass.set_vertex_buffer(0, self.vbo.slice(..)); // slot 0 - vertices
        pass.set_vertex_buffer(1, self.vids.slice(..)); // slot 1 - per-vertex row ids
        pass.set_index_buffer(run.buf.slice(..), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..run.count, 0, 0..1); // whole scene, one call
    }

    /// The solid faces. The pipeline and groups 0-2 are already set by the frame.
    pub fn draw_faces(&self, pass: &mut wgpu::RenderPass) {
        if self.solid.count > 0 {
            self.draw(pass, IdxLane::Solid);
        }
    }

    /// SHEET FILLS. Same vertex table, depth WRITE off, so a page's exactly coplanar regions
    /// composite in document order instead of flickering over one shared depth value. They still
    /// depth-TEST, so 3D geometry in front of the sheet occludes. Returns the draws it issued.
    pub fn draw_print(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.print.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.arena.sheet);
        self.draw(pass, IdxLane::Print);
        1
    }

    /// LETTERING, last of everything. A page paints its text on top of its hatching AND its
    /// linework, so it lands after the ink lanes - the one thing draw order can express that a
    /// depth buffer cannot, since all of it is coplanar at z = 0.
    pub fn draw_text(&self, pass: &mut wgpu::RenderPass, b: &Binds) -> u32 {
        if self.text.count == 0 {
            return 0;
        }
        pass.set_pipeline(&b.p.arena.sheet);
        pass.set_bind_group(0, b.mvp, &[]);
        pass.set_bind_group(1, b.time, &[]);
        pass.set_bind_group(2, b.instances, &[]);
        self.draw(pass, IdxLane::Text);
        1
    }
}

/// The family's pipelines. Two, and they differ in ONE field.
pub struct Pipes {
    pub triangle: wgpu::RenderPipeline,
    /// Same program, depth WRITE off: the sheet lanes (print fills, then lettering) composite in
    /// draw order instead of fighting over one coplanar depth value.
    pub sheet: wgpu::RenderPipeline,
}

impl Pipes {
    /// A family builds its own pipelines from the shared layouts. `Pipelines::new` calls this
    /// and never sees `TRIANGLE`, which is what keeps the shader constant in the file that owns
    /// the rows it reads.
    pub fn descs(device: &wgpu::Device, t: Target, l: &Layouts) -> Self {
        Self {
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
            sheet: build(device, t, &PipelineDesc {
                vertex_buffers: &[RenderVertex::layout(), instance_id_layout()],
                depth_write: false,
                ..PipelineDesc::sheet("triangle.sheet", TRIANGLE, &[&l.mvp, &l.time, &l.instance])
            }),
        }
    }
}
```

Then the vertex-id layout comes over from `pipelines/build.rs` — by Move, because it is
byte-identical and this is the case where that is worth demonstrating. `vids` is this family's
second vertex buffer and nothing else in the program has one, so the layout that describes it
belongs beside the buffer, not in a file of generic builders.


**Move** `src/engine/pipelines/build.rs` `const INSTANCE_ID_ATTRIBS: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {` **through** `}];` **to** `src/engine/gpu/arena.rs` **after** `const TRIANGLE: &str = include_str!("../../shaders/triangle.wgsl");`

**Move** `src/engine/pipelines/build.rs` `// This helps the GPU to read the second vertex buffer - the instance row id.` **through** `}` **to** `src/engine/gpu/arena.rs` **after** `}];`

**Find** in `src/engine/gpu/arena.rs`:

```rust
// This helps the GPU to read the second vertex buffer - the instance row id.
// Without a layout description, the pipeline doesn' know those bytes exists and in what shape they are.
/// Vertex-buffer layout for the per-vertex instance-row id (`@location(3)`, one `u32` per vertex).
pub fn instance_id_layout() -> wgpu::VertexBufferLayout<'static>{
```

**Replace with:**

```rust
/// Vertex-buffer layout for the per-vertex instance-row id (`@location(3)`, one `u32` per vertex).
/// Without it the pipeline does not know those four bytes exist, or what shape they are.
/// It belongs here and nowhere else: `vids` is this family's second vertex buffer.
fn instance_id_layout() -> wgpu::VertexBufferLayout<'static>{
```

**Find** in `src/engine/pipelines/build.rs`:

```rust
    format: wgpu::VertexFormat::Float32x3,
}];


/// Vertex-buffer layout for the unit-cylinder/-sphere template positions (`@location(0)`, one `vec3<f32>`).
```

**Replace with:**

```rust
    format: wgpu::VertexFormat::Float32x3,
}];

/// Vertex-buffer layout for the unit-cylinder/-sphere template positions (`@location(0)`, one `vec3<f32>`).
```

**Gate.**

```bash
cargo check --target wasm32-unknown-unknown --lib      # 0 errors: nothing calls them yet
wc -l src/engine/gpu/instance.rs src/engine/gpu/objects.rs src/engine/gpu/arena.rs
```

```text
 159 src/engine/gpu/instance.rs
 341 src/engine/gpu/objects.rs
 303 src/engine/gpu/arena.rs
```

If a count is far off, a paste went wrong and it is cheaper to find out now than after 25 fields
have moved.

## 5. Where the borrow checker bites — B2, and it bites in step 6.5

> ```rust
> self.objects.append(&self.ctx, &self.layouts, &up.obj);
> ```
>
> reads fine. But write the same call as a method that takes `&mut self` on `Gpu` and reaches
> for two of its own fields —
>
> ```rust
> fn append_objects(&mut self) {
>     self.objects.append(&self.ctx, &self.layouts, &self.upload.obj);
> }                    // ^^^^^^^^^^^^ E0499: cannot borrow `self.objects` as mutable
> ```                  //                      while `self` is also borrowed as immutable
>
> — and it stops compiling. The fix is the one lesson 46 introduced as B1, applied one level
> down: **the sub-struct takes what it needs as parameters, and the caller does the field
> access at the call site.** `&self.ctx` and `&self.layouts` are disjoint borrows of `Gpu`;
> `self.<method that borrows all of self>` is not. It recurs every time a family method needs
> the floor, which is every family method in lessons 48 and 49.

## 6. The steps

### 6.1 `buffers.rs` — `GrowBuf` stops being dead

Lesson 46 typed `GrowBuf` and put `#[allow(dead_code)]` on it, with a comment promising the
attribute would come off at 47 with `Arena`. `Arena` has three of them, so it comes off now — and
`append_index_run`, which took the same triple spelled out plus a label and the data, drops from
six parameters to three.


**Find** in `src/engine/gpu/buffers.rs`:

```rust
#[allow(dead_code)]
pub struct GrowBuf {
```

**Replace with:**

```rust
pub struct GrowBuf {
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// Nothing constructs one YET: `Gpu` still carries the twelve triples spread flat, and each is
/// folded here by the lesson that creates its family - so the attribute comes off at 47, with
/// `Arena`.
```

**Replace with:**

```rust
/// `Arena` folds the first three at 47; the ink lanes follow at 48 and the point lanes at 49.
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
pub fn append_index_run(
    ctx: &GpuCtx,
    label: &str,
    ibo: &mut wgpu::Buffer,
    count: &mut u32,
    cap: &mut u64,
    data: &[u32],
) {
    if data.is_empty() {
        return;
    }
    let need = *count as u64 + data.len() as u64;
    if need > *cap {
        let new_cap = need.max(*cap * 2);
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let nb = zeroed_buffer(&ctx.device, label, new_cap * 4, iu);
        if *count > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(ibo, 0, &nb, 0, *count as u64 * 4);
            ctx.queue.submit([enc.finish()]);
        }
        *ibo = nb;
        *cap = new_cap;
    }
    ctx.queue.write_buffer(ibo, *count as u64 * 4, bytemuck::cast_slice(data));
    *count += data.len() as u32;
}
```

**Replace with:**

```rust
pub fn append_index_run(ctx: &GpuCtx, run: &mut GrowBuf, data: &[u32]) {
    if data.is_empty() {
        return;
    }
    let need = run.count as u64 + data.len() as u64;
    if need > run.cap {
        let new_cap = need.max(run.cap * 2);
        let nb = zeroed_buffer(&ctx.device, run.label, new_cap * 4, run.usage);
        if run.count > 0 {
            let mut enc = ctx.device.create_command_encoder(&Default::default());
            enc.copy_buffer_to_buffer(&run.buf, 0, &nb, 0, run.count as u64 * 4);
            ctx.queue.submit([enc.finish()]);
        }
        run.buf = nb;
        run.cap = new_cap;
    }
    ctx.queue.write_buffer(&run.buf, run.count as u64 * 4, bytemuck::cast_slice(data));
    run.count += data.len() as u32;
}
```

**Gate.** `cargo check --target wasm32-unknown-unknown --lib` — one error, at the two old call
sites in `set_scene`, which step 6.5 deletes. That is the shape of a refactor's error wall:
`cargo check 2>&1 | grep -c '^error'` is your progress bar, and you fix only the FIRST error and
re-run, because two hundred `E0609`s are usually one missing rename.

### 6.2 `Upload` — eight flat columns become two groups

A producer should be handed the columns it may write, not all nineteen. The arena's five and the
object table's three go first; `seg`, `glyph` and `cloud` follow at 48 and 49.


**Find** in `src/engine/gpu/upload.rs`:

```rust
use crate::math::Mat4;
use session_rust::RenderVertex;

use super::{CloudDraw, CylinderSegment, GlyphPoint, LodNode};
```

**Replace with:**

```rust
use super::arena::ArenaRows;
use super::objects::ObjectRows;
use super::{CloudDraw, CylinderSegment, GlyphPoint, LodNode};
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
    pub verts: Vec<RenderVertex>,
    pub vids: Vec<u32>,
    pub idx: Vec<u32>,
    pub pipes
```

**Replace with:**

```rust
    /// The triangle family's rows: one vertex table and three index runs (`arena.rs`).
    pub arena: ArenaRows,
    pub pipes
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
    /// Sheet lanes. A PDF's fills are exactly coplanar, so they must NOT arbitrate by depth -
    /// they are split off the solid index run and drawn in document order with depth write off.
    /// `idx_text` is the lettering, drawn LAST of all, after the ink lanes, because a page puts
    /// its text on top of both its hatching and its linework.
    pub idx_print: Vec<u32>,
    pub idx_text: Vec<u32>,
    pub objects: Vec<(Mat4, [f32; 4], u32)>,
    /// Mesh-local AABB per object row, aligned with `objects`. None for linework/points/clouds:
    /// only the solid lane's facing cull needs it (see `Instance::FLAG_INSIDE`).
    pub object_bounds: Vec<Option<([f32; 3], [f32; 3])>>,
    /// Vertex spacing per object row, world units, aligned with `objects`. 0 = unknown (linework,
    /// points, clouds), which the ink lanes read as "never density-cull".
    pub object_spacing: Vec<f32>,
    pub min
```

**Replace with:**

```rust
    /// The object rows: one `(model, tint, flags)` per guid, plus the two per-row columns that
    /// ride with it (`objects.rs`). Every `instance_id` in every other column indexes this one.
    pub obj: ObjectRows,
    pub min
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
            verts: Vec::new(),
            vids: Vec::new(),
            idx: Vec::new(),
            pipes: Vec::new(),
```

**Replace with:**

```rust
            arena: ArenaRows::new(),
            pipes: Vec::new(),
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
            idx_print: Vec::new(),
            idx_text: Vec::new(),
            objects: Vec::new(),
            object_bounds: Vec::new(),
            object_spacing: Vec::new(),
```

**Replace with:**

```rust
            obj: ObjectRows::new(),
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
        drop_rows(&mut self.verts);
        drop_rows(&mut self.vids);
        drop_rows(&mut self.idx);
        drop_rows(&mut self.idx_print);
        drop_rows(&mut self.idx_text);
```

**Replace with:**

```rust
        drop_rows(&mut self.arena.verts);
        drop_rows(&mut self.arena.vids);
        drop_rows(&mut self.arena.idx);
        drop_rows(&mut self.arena.idx_print);
        drop_rows(&mut self.arena.idx_text);
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
//! It arrives here FLAT - one struct, nineteen columns, today's names. Each family regroups its
//! own columns into a `<Family>Rows` sink as it is created (47-49), so a producer can be handed
//! the two columns it may write instead of all nineteen.
```

**Replace with:**

```rust
//! It arrived FLAT - one struct, nineteen columns. Each family regroups its own columns into a
//! `<Family>Rows` sink as it is created: `arena` and `obj` here at 47, `seg`/`glyph` at 48,
//! `cloud` at 49 - so a producer can be handed the two columns it may write, not all nineteen.
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
    /// Fourteen of the nineteen columns; `objects`, `object_bounds` and `object_spacing` stay,
    /// and `min`/`max` are not rows.
```

**Replace with:**

```rust
    /// Fourteen of the nineteen columns; the three `obj` columns stay,
    /// and `min`/`max` are not rows.
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
        // `objects`, `object_bounds` and `object_spacing` STAY: they are per-object rows the
```

**Replace with:**

```rust
        // The three `obj` columns STAY: they are per-object rows the
```

### 6.3 The triangle family takes its own pipelines

`Pipelines` keeps the LIST. The descs, and the `include_str!` of the shader they compile, go to
the file that owns the rows they read — which is what makes "add a shader" a change in one file
instead of three.


**Find** in `src/engine/pipelines/mod.rs`:

```rust
use build::{build, build_compute, cyl_template_layout, instance_id_layout};
use session_rust::RenderVertex;
```

**Replace with:**

```rust
use build::{build, build_compute, cyl_template_layout};
use crate::engine::gpu::arena;
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
const TRIANGLE: &str = include_str!("../../shaders/triangle.wgsl");
const SPLAT: &str = include_str!("../../shaders/splat.wgsl");
```

**Replace with:**

```rust
const SPLAT: &str = include_str!("../../shaders/splat.wgsl");
```

**Find** in `src/engine/pipelines/mod.rs`:

```rust
/// Every pipeline the viewer draws with, built once at startup and rebuilt whole on an MSAA
/// flip. Fourteen render pipelines and two compute.
pub struct Pipelines{
    pub triangle: wgpu::RenderPipeline,
    /// Same program, depth WRITE off: the sheet lanes (print fills, then lettering) composite in
    /// draw order instead of fighting over one coplanar depth value. See its desc below.
    pub triangle_sheet: wgpu::RenderPipeline,
    pub grid: wgpu::RenderPipeline,
```

**Replace with:**

```rust
/// Every pipeline the viewer draws with, built once at startup and rebuilt whole on an MSAA
/// flip. Fourteen render pipelines and two compute - the triangle family's two now live behind
/// `arena`, which is where lessons 48 and 49 take the other twelve.
pub struct Pipelines{
    /// The triangle family's own two, built by `gpu::arena` from the layouts below. A family
    /// owns the pipelines that read its rows; this struct owns only the list.
    pub arena: arena::Pipes,
    pub grid: wgpu::RenderPipeline,
```

**Find** in `src/engine/pipelines/mod.rs`:

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

**Replace with:**

```rust
            arena: arena::Pipes::descs(device, t, l),
```

### 6.4 `Gpu`'s head — 25 fields become 2

Leaves before roots: the modules and imports first, then the field list, then the constructor.


**Find** in `src/engine/gpu/mod.rs`:

```rust
pub mod buffers;
pub mod frame;
pub mod present;
pub mod targets;
pub mod upload;
pub mod view;
```

**Replace with:**

```rust
pub mod arena;
pub mod buffers;
pub mod frame;
pub mod instance;
pub mod objects;
pub mod present;
pub mod targets;
pub mod upload;
pub mod view;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use frame::{CloudUniform, FrameInput, FrameUniforms, LineUniform};
```

**Replace with:**

```rust
use frame::{CloudUniform, FrameInput, FrameUniforms, LineUniform};
pub use instance::Instance;
use arena::Arena;
use objects::InstanceTable;
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use buffers::{GpuCtx, append_index_run, append_rows, mk_rows_group, zeroed_buffer};
```

**Replace with:**

```rust
use buffers::{GpuCtx, append_rows, mk_rows_group, zeroed_buffer};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
pub use crate::math::{Mat4, mat_mul, mat_to_f32, eye_from_view_proj, ortho_half_height};
```

**Replace with:**

```rust
pub use crate::math::{mat_mul, eye_from_view_proj, ortho_half_height};
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use session_rust::{Xform, RenderVertex, Point};
```

**Replace with:**

```rust
use session_rust::{Xform, Point};
```

Now the field list itself. Twenty-five lines out, two in — and read the two comments, because
after this the only way to find out what is in the object table is to open `objects.rs`, which is
the point.


**Find** in `src/engine/gpu/mod.rs`:

```rust
    pub arena_vbo: wgpu::Buffer,
    pub arena_vids: wgpu::Buffer,
    pub arena_ibo: wgpu::Buffer,
    pub arena_index_count: u32,
    // The two sheet index runs, appended exactly like the solid one.
    arena_ibo_print: wgpu::Buffer,
    arena_print_count: u32,
    arena_print_cap: u64,
    arena_ibo_text: wgpu::Buffer,
    arena_text_count: u32,
    arena_text_cap: u64,
    pub arena_vert_count: u32,   // rows already on the GPU - the base for the next append
    pub arena_vert_cap: u64,
    pub arena_index_cap: u64,
    instances: Vec<Instance>,
    last_origin: Option<Point>, // rebuild_instances skips when the camera target did not move
    objects_base: Vec<(Mat4, [f32; 4], u32)>, // TRUE world model+color; isntance[] is rebased from this
    base_f32: Vec<[f32; 16]>, // mode.to_f32() cached once - rebase only re-patches 3 slots
    bounded_rows: Vec<u32>, // rows with Some(world AABB) - the only onces the inside test walks
    /// Per-object WORLD AABB (Upload.object_bounds through the true transform), aligned with
    /// `instances`. Drives FLAG_INSIDE - see update_inside_flags.
    object_bounds_world: Vec<Option<([f64; 3], [f64; 3])>>,
    inside: Vec<bool>, // current FLAG_INSIDE state per instance row, for change detection

    instance_buffer: wgpu::Buffer, // new() builds this storage buffer as a local and drops it, only the bidn group survives; rebuild_instances() reuploads into it every frame, so the buffer handle itself must live on GPU, not vanish atht eh of new()
    instance_rows: u32, // instance rows already ON the GPU - the base for the next append
    instance_cap: u64,
    pub instance_bind_group: wgpu::BindGroup,
```

**Replace with:**

```rust
    /// The object table: one row per guid, rebased about the camera anchor (`objects.rs`).
    /// Every `instance_id` in every other lane indexes it.
    pub objects: InstanceTable,
    /// The triangle family: one vertex table, three index runs (`arena.rs`).
    pub arena: Arena,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    last_rebase_ms: f64, // throttle - a 210k-row rebase costs ~25 ms, one per frame is jank
    pub performance: Performance,
```

**Replace with:**

```rust
    pub performance: Performance,
```

And the constructor. Both sub-structs build themselves; `Gpu::build` stops knowing that an
instance buffer needs `COPY_SRC` or that an index run starts at capacity 1.


**Find** in `src/engine/gpu/mod.rs`:

```rust
        // The scene-shaped fields start as empty placeholders
        // WebGPU zero-initializes buffers, and every *_count is 0, so the first frame draws nothing.
        // The loader calls set_scene the moment the first file's tables exist.
        let instances: Vec<Instance> = vec![Instance{
            model: Xform::identity().to_f32(), color: [0.5, 0.5, 0.5, 1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0,
        }];

        // COPY_SRC because the table GROWS by appending: when it outgrows its buffer the prefix
        // is copied GPU-side into the bigger one, and a buffer without COPY_SRC cannot be the
        // source of that copy.
        let instance_buffer = zeroed_buffer(
            &device,
            "instance.buffer",
            std::mem::size_of::<Instance>() as u64,
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC);
        let objects_base: Vec<(Mat4, [f32; 4], u32)> = Vec::new();
        let base_f32: Vec<[f32; 16]> = Vec::new();
        let bounded_rows: Vec<u32> = Vec::new();
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let arena_index_count = 0u32;
        let iu_sheet = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_ibo_print = zeroed_buffer(&device, "arena.ibo.print", 4, iu_sheet);
        let arena_ibo_text = zeroed_buffer(&device, "arena.ibo.text", 4, iu_sheet);
        let (arena_print_count, arena_print_cap) = (0u32, 1u64);
        let (arena_text_count, arena_text_cap) = (0u32, 1u64);
        let arena_vert_count = 0u32;
        let (arena_vert_cap, arena_index_cap) = (1u64, 1u64);
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);
```

**Replace with:**

```rust
        // The scene-shaped fields start as empty placeholders. WebGPU zero-initializes buffers,
        // and every *_count is 0, so the first frame draws nothing. The loader calls set_scene
        // the moment the first file's tables exist.
        let objects = InstanceTable::new(&device, &layouts);
        let arena = Arena::new(&device);
        let (pipe_count, segment_count, sphere_count, glyph_count) = (0u32, 0u32, 0u32, 0u32);
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("instances.bind_group"),
            layout: &layouts.instance,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: instance_buffer.as_entire_binding()}],
        });

        // One zeroed row each - wgpu cannot bind a 0-byte buffer, and arena_index_count starts
        // at 0 so nothing is drawn from them until real geometry appends.
        let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
        let arena_vbo = zeroed_buffer(&device, "arena.vbo", std::mem::size_of::<RenderVertex>() as u64, vu);
        let arena_vids = zeroed_buffer(&device, "arena.vids", 4, vu);
        let arena_ibo = zeroed_buffer(&device, "arena.ibo", 4, iu);

```

**Replace with:**

```rust

```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            &instance_buffer,
```

**Replace with:**

```rust
            &objects.buffer,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let splat_group0_stream = Self::mk_splat_group0(&device, &layouts.splat_group0, &mvp_buffer, &cloud_buffer, &instance_buffer, &splat_stream_recs);
```

**Replace with:**

```rust
        let splat_group0_stream = Self::mk_splat_group0(&device, &layouts.splat_group0, &mvp_buffer, &cloud_buffer, &objects.buffer, &splat_stream_recs);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            arena_vbo,
            arena_vids,
            arena_ibo,
            arena_index_count,
            arena_ibo_print,
            arena_print_count,
            arena_print_cap,
            arena_ibo_text,
            arena_text_count,
            arena_text_cap,
            arena_vert_count,
            arena_vert_cap,
            arena_index_cap,
            instances,
            last_origin: None,
            objects_base,
            base_f32,
            bounded_rows,
            object_bounds_world: Vec::new(),
            inside: Vec::new(),
            instance_buffer, // was a dropped local in new(), now moved onto GPU so rebuild_instances() can write into every frame
            instance_rows: 0,
            instance_cap: 1,
            instance_bind_group,
```

**Replace with:**

```rust
            objects,
            arena,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            view: View::from_env(),
            last_rebase_ms: 0.0,
            targets,
```

**Replace with:**

```rust
            view: View::from_env(),
            targets,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
use session_rust::{Xform, Point};


/// const for the unit_cylinder method
```

**Replace with:**

```rust
use session_rust::{Xform, Point};

/// const for the unit_cylinder method
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);


        // Unit-cylinder tempalte (positions only) - one mesh, instance per edge.
```

**Replace with:**

```rust
        let (scene_min, scene_max) = ([0.0f32; 3], [0.0f32; 3]);

        // Unit-cylinder tempalte (positions only) - one mesh, instance per edge.
```

**Gate.** `cargo check --target wasm32-unknown-unknown --lib` — a wall of `E0609`, one per
remaining call site. Steps 6.5 and 6.6 walk it down to zero.

### 6.5 `set_scene` — forty lines become two calls

The object table first, because every other lane's rows carry an index into it.


**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Instance rows: rebuilt from the true transforms (rebase state, must live CPU-side).
        //
        // `up.objects` is the ONE table the walk keeps cumulative - the bounds sweep and the
        // per-file sheet pass both index it by global row - so this is the one lane that gets a
        // full table every time instead of a delta. Only the NEW rows are turned into instances
        // and sent: cloning 148k rows per file was 22 MB of memcpy and a full re-upload, for a
        // tail that had not changed since the file before.
        let base = self.objects_base.len();
        if base == 0 {
            // First upload, or a rebuild that rewound everything: start the GPU table over too,
            // which also drops the one-row placeholder an empty scene leaves behind.
            self.instances.clear();
            self.instance_rows = 0;
        }
        debug_assert_eq!(up.objects.len(), up.object_bounds.len());
        debug_assert!(up.objects.len() >= base, "the object table only ever grows");
        self.objects_base.extend_from_slice(&up.objects[base..]);
        // Rebase re-patches only translations, so the 13 other floats can be cast once here
        // instead of per re-achor: at 210k objects that turns a 20+ msCPU loop into a copy
        self.base_f32.extend(up.objects[base..].iter().map(|(m, _, _)| mat_to_f32(m)));
        self.object_bounds_world.extend(up.objects[base..].iter().zip(&up.object_bounds[base..]).map(|((m, _, _), b)| {
            b.map(|(lo, hi)| {
                // World AABB of the local box: the 8 corners through the true transform.
                // Conservative for rotated placements - FLAG_INSIDE is a hint, not a cull.
                let xp = |x: f64, y: f64, z: f64| [
                    m[0] * x + m[4] * y + m[8] * z + m[12],
                    m[1] * x + m[5] * y + m[9] * z + m[13],
                    m[2] * x + m[6] * y + m[10] * z + m[14],
                ];
                let mut wlo = [f64::INFINITY; 3];
                let mut whi = [f64::NEG_INFINITY; 3];
                for c in 0..8 {
                    let p = xp(
                        (if c & 1 == 0 { lo[0] } else { hi[0] }) as f64,
                        (if c & 2 == 0 { lo[1] } else { hi[1] }) as f64,
                        (if c & 4 == 0 { lo[2] } else { hi[2] }) as f64,
                    );
                    for k in 0..3 { wlo[k] = wlo[k].min(p[k]); whi[k] = whi[k].max(p[k]); }
                }
                (wlo, whi)
            })
        }));
        self.inside.resize(self.objects_base.len(), false);
        self.bounded_rows = self.object_bounds_world.iter().enumerate().filter_map(|(i, b)| b.map(|_| i as u32)).collect();
        // `object_bounds_world` was just extended above, so each row's extent comes from the same
        // AABB FLAG_INSIDE uses. The diagonal, not an axis: a flat sheet has a zero-thickness axis
        // and would clamp its ink lift to nothing.
        let bounds = &self.object_bounds_world;
        self.instances.extend(up.objects[base..].iter().enumerate().map(|(i, (m, c, f))| Instance {
            model: mat_to_f32(m),
            color: *c,
            flags: *f,
            extent: bounds.get(base + i).and_then(|b| *b).map_or(0.0, |(lo, hi)| {
                ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2) + (hi[2] - lo[2]).powi(2)).sqrt() as f32
            }),
            spacing: up.object_spacing.get(base + i).copied().unwrap_or(0.0),
            _pad: 0,
        }));

        if self.instances.is_empty(){
            self.instances.push(Instance {model: Xform::identity().to_f32(), color: [0.5,0.5,0.5,1.0], flags: 0, extent: 0.0, spacing: 0.0, _pad: 0 });
        }

        let mut rows = self.instance_rows;
        let fresh = &self.instances[rows as usize..];
        if append_rows(&self.ctx, "instance.buffer",
            &mut self.instance_buffer, &mut rows, &mut self.instance_cap, fresh) {
            self.instance_bind_group = mk_rows_group(&self.ctx.device, &self.layouts.instance, "instances.bind_group", &self.instance_buffer);
        }
        self.instance_rows = rows;
```

**Replace with:**

```rust
        // The object table first: every other lane's rows carry an `instance_id` into it.
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        // Mesh arena. Like the cloud lane, `up.verts/vids/idx` are a DELTA - the caller clears
        // them after upload (Scene::upload_to), because nothing reads them back: picking goes
        // through the kernel Meshes in Doc.session, never through these flattened rows.
        //
        // Appending rather than rebuilding is worth two separate things. It stops re-sending the
        // whole arena on every file (six files meant the 64 MB vertex table travelled six times),
        // and it lets the CPU-side Vecs go, which is ~70 MB of wasm heap that was being held for
        // the sole purpose of feeding the next rebuild.
        if !up.verts.is_empty() {
            let vstride = std::mem::size_of::<RenderVertex>() as u64;
            let need_v = self.arena_vert_count as u64 + up.verts.len() as u64;
            let need_i = self.arena_index_count as u64 + up.idx.len() as u64;

            if need_v > self.arena_vert_cap || need_i > self.arena_index_cap {
                let cap_v = need_v.max(self.arena_vert_cap);
                let cap_i = need_i.max(self.arena_index_cap);
                let vu = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
                let iu = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC;
                let vbo = zeroed_buffer(&self.ctx.device, "arena.vbo", cap_v * vstride, vu);
                let vids = zeroed_buffer(&self.ctx.device, "arena.vids", cap_v * 4, vu);
                let ibo = zeroed_buffer(&self.ctx.device, "arena.ibo", cap_i * 4, iu);
                if self.arena_vert_count > 0 {
                    // the prefix moves GPU-side; it never travels back through wasm memory
                    let mut enc = self.ctx.device.create_command_encoder(&Default::default());
                    enc.copy_buffer_to_buffer(&self.arena_vbo, 0, &vbo, 0, self.arena_vert_count as u64 * vstride);
                    enc.copy_buffer_to_buffer(&self.arena_vids, 0, &vids, 0, self.arena_vert_count as u64 * 4);
                    enc.copy_buffer_to_buffer(&self.arena_ibo, 0, &ibo, 0, self.arena_index_count as u64 * 4);
                    self.ctx.queue.submit([enc.finish()]);
                }
                self.arena_vbo = vbo;
                self.arena_vids = vids;
                self.arena_ibo = ibo;
                self.arena_vert_cap = cap_v;
                self.arena_index_cap = cap_i;
            }

            self.ctx.queue.write_buffer(&self.arena_vbo, self.arena_vert_count as u64 * vstride, bytemuck::cast_slice(&up.verts));
            self.ctx.queue.write_buffer(&self.arena_vids, self.arena_vert_count as u64 * 4, bytemuck::cast_slice(&up.vids));
            self.ctx.queue.write_buffer(&self.arena_ibo, self.arena_index_count as u64 * 4, bytemuck::cast_slice(&up.idx));
            self.arena_vert_count += up.verts.len() as u32;
            self.arena_index_count += up.idx.len() as u32;

            // The sheet runs grow and append the same way; they index the SAME vertex table, so
            // splitting them costs one buffer each and no duplicated geometry.
            append_index_run(&self.ctx, "arena.ibo.print",
                &mut self.arena_ibo_print, &mut self.arena_print_count, &mut self.arena_print_cap, &up.idx_print);
            append_index_run(&self.ctx, "arena.ibo.text",
                &mut self.arena_ibo_text, &mut self.arena_text_count, &mut self.arena_text_cap, &up.idx_text);
        }
```

**Replace with:**

```rust
        // Then the triangle family: one vertex table and three index runs.
        self.arena.append(&self.ctx, &up.arena);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            self.instances.len(), self.arena_vert_count, self.pipe_count + self.segment_count, self.pipe_count,
```

**Replace with:**

```rust
            self.objects.len(), self.arena.verts(), self.pipe_count + self.segment_count, self.pipe_count,
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.last_origin = None; // force the next frame to rebase agains the new table
        if up.min[0].is_finite()
```

**Replace with:**

```rust
        if up.min[0].is_finite()
```

`rebase_anchor` stays on `Gpu`, as a forwarder, for one reason: a rebase moves every row, which
makes every splat record stale, and staleness of the splat lane is the one thing `InstanceTable`
cannot know and `Gpu` can.


**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// The anchor the instance table is rebased about.
    /// A full rebuild (42 000 x at stress scale) runs
    /// only when the camera target strays REANCHOR_DIST from the current anchor - orbit newer moves the target.
    /// And pan/zoom within the budget just changes the view matrix
    /// `origin` and `view_dist` are both in WORLD units (mm) - the same units as the instance
    /// table's translations. Feeding metres here (the camera's internal unit) makes the subtract
    /// below a no-op at 1/1000 scale, which silently turns camera-relative rendering off: the
    /// symptom is geometry that jitters and then clips away entirely as you zoom in, because the
    /// f32 mvp is differencing two large world magnitudes.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64) -> Point{
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
        let now = crate::engine::performance::now_ms();
        if need && (now - self.last_rebase_ms > 200.0 || self.last_origin.is_none()) {
            self.rebuild_instances(origin);
            self.last_rebase_ms = now;
        }
        self.last_origin.clone().unwrap()
    }
```

**Replace with:**

```rust
    /// The anchor the instance table is rebased about (`objects.rs` does the work).
    ///
    /// A rebase moves every row, so any splat record built against the old positions is stale -
    /// which is the one thing `InstanceTable` cannot know and `Gpu` can.
    pub fn rebase_anchor(&mut self, origin: &Point, view_dist: f64) -> Point{
        let (anchor, rebuilt) = self.objects.rebase_anchor(&self.ctx, origin, view_dist);
        if rebuilt {
            self.splat_state = None; // instance model moved - splats are stale
        }
        anchor
    }
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
    /// Rebase every instance's translation around 'origin' - an f64 subtract agains the TRUE world transfrom in 'objects_base'
    /// Then cast to f32.
    /// 'instances', what GPU actually sees, never holds a coordinate bigger than the camera's distnace from 'origin',
    /// no matter how fas the scene fists from world (0,0,0).
    fn rebuild_instances(&mut self, origin: &Point){
        // let shift = Xform::translation(-origin[0], -origin[1], -origin[2]);
        // for (i, (model, color)) in self.objects_base.iter().enumerate() {
        //     self.instances[i].model = (&shift * model).to_f32(); // f64 multiply, f32 cast last
        //     self.instances[i].color = *color;
        // }
        // self.ctx.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
        self.last_origin = Some(origin.clone());
        for (i, (model, _, _)) in self.objects_base.iter().enumerate() {
            let mut m = self.base_f32[i]; // rotation / scale casr once at set_scene
            m[12] = (model[12] - origin[0]) as f32;
            m[13] = (model[13] - origin[1]) as f32;
            m[14] = (model[14] - origin[2]) as f32;
            self.instances[i].model = m;
        }
        self.ctx.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
        self.splat_state = None; // instance model moved - splats are stale

    }

    // splat helpers - one compute-visible buffer entry, and the three bind groups,
```

**Replace with:**

```rust
    // splat helpers - one compute-visible buffer entry, and the three bind groups,
```

### 6.6 The rest of `Gpu` — the frame, the draws, the reset


**Find** in `src/engine/gpu/mod.rs`:

```rust
        let f = FrameInput { config: &self.config, view_proj, anchor: self.last_origin.as_ref(), view: &self.view };
```

**Replace with:**

```rust
        let f = FrameInput { config: &self.config, view_proj, anchor: self.objects.anchor(), view: &self.view };
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        if self.bounded_rows.is_empty(){
            return;
        }
        let Some(origin) = self.last_origin.clone() else { return };
        let eye = eye_from_view_proj(view_proj); // anchored world units, like instances[]
        let ew = [origin[0] + eye[0] as f64, origin[1] + eye[1] as f64, origin[2] + eye[2] as f64];
        // The eye outside the scene's box is outside every object in it.
        let in_scene = (0..3).all(|k| ew[k] >= self.scene_min[k] as f64 && ew[k] <= self.scene_max[k] as f64);
        let mut dirty = false;
        for &row in &self.bounded_rows{
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
            self.ctx.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
        }
    }
```

**Replace with:**

```rust
        self.objects.update_inside_flags(&self.ctx, view_proj, self.scene_min, self.scene_max);
    }
```

**Replace-all** `src/engine/gpu/mod.rs` `self.instance_buffer` -> `self.objects.buffer` (2 hits)

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let Some(row) = self.instances.get(inst as usize) else { continue };
```

**Replace with:**

```rust
            let Some(row) = self.objects.row(inst as usize) else { continue };
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            let b = self.frame.binds(&self.pipelines, &self.instance_bind_group);
```

**Replace with:**

```rust
            let b = self.frame.binds(&self.pipelines, &self.objects.bind_group);
```

The three arena draws. The solid one keeps its `draws += 1` OUTSIDE the emptiness test, exactly
as before — the count is of pipelines set, not of index runs that happened to be non-empty, and
the goldens count it.


**Find** in `src/engine/gpu/mod.rs`:

```rust
            pass.set_pipeline(&b.p.triangle);
            pass.set_bind_group(0, b.mvp, &[]);
            pass.set_bind_group(1, b.time, &[]);
            pass.set_bind_group(2, b.instances, &[]);

            // Arena draw
            if self.arena_index_count > 0 {
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..)); // slot 0 - vertices
                pass.set_vertex_buffer(1, self.arena_vids.slice(..)); // slot 1 - per-vertex row ids
                pass.set_index_buffer(self.arena_ibo.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_index_count, 0, 0..1); // whole scene, one call
            }
            draws += 1;
```

**Replace with:**

```rust
            pass.set_pipeline(&b.p.arena.triangle);
            pass.set_bind_group(0, b.mvp, &[]);
            pass.set_bind_group(1, b.time, &[]);
            pass.set_bind_group(2, b.instances, &[]);
            draws += self.arena.draw_faces(&mut pass);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            // SHEET FILLS, second. Same vertex table, depth WRITE off, so a page's exactly
            // coplanar regions composite in document order instead of flickering over one shared
            // depth value. They still depth-TEST, so 3D geometry in front of the sheet occludes.
            if self.arena_print_count > 0 {
                pass.set_pipeline(&b.p.triangle_sheet);
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..));
                pass.set_vertex_buffer(1, self.arena_vids.slice(..));
                pass.set_index_buffer(self.arena_ibo_print.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_print_count, 0, 0..1);
                draws += 1;
            }
```

**Replace with:**

```rust
            // SHEET FILLS, second - the same vertex table with depth write off. See draw_print.
            draws += self.arena.draw_print(&mut pass, &b);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
            if self.arena_text_count > 0 {
                pass.set_pipeline(&b.p.triangle_sheet);
                pass.set_bind_group(0, b.mvp, &[]);
                pass.set_bind_group(1, b.time, &[]);
                pass.set_bind_group(2, b.instances, &[]);
                pass.set_vertex_buffer(0, self.arena_vbo.slice(..));
                pass.set_vertex_buffer(1, self.arena_vids.slice(..));
                pass.set_index_buffer(self.arena_ibo_text.slice(..), wgpu::IndexFormat::Uint32);
                pass.draw_indexed(0..self.arena_text_count, 0, 0..1);
                draws += 1;
            }
```

**Replace with:**

```rust
            draws += self.arena.draw_text(&mut pass, &b);
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        (draws, self.instances.len() as u32)
```

**Replace with:**

```rust
        (draws, self.objects.len() as u32)
```

And the two resets. Note the second one: eleven lines become one call, and the comment about
`bounded_rows` being derived state moves into `InstanceTable::clear`, where the vector it warns
about actually lives.


**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.arena_vert_count = 0;
        self.arena_index_count = 0;
        self.arena_print_count = 0;
        self.arena_text_count = 0;
```

**Replace with:**

```rust
        self.arena.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        self.objects_base.clear();
        self.object_bounds_world.clear();
        self.inside.clear();
        self.instances.clear();
        self.instance_rows = 0;
        // DERIVED from object_bounds_world (rebuilt in set_scene), so leaving it
        // behind holds row indices into a vector that is now empty. `rebuild`
        // hides that by re-walking immediately, but a scene that is cleared and
        // then DRAWN before the next upload - reload_scene between Clear and the
        // first File - panics in update_inside_flags on the stale rows.
        self.bounded_rows.clear();
```

**Replace with:**

```rust
        self.objects.reset();
```

**Find** in `src/engine/gpu/mod.rs`:

```rust
        let solid = self.arena_vert_count > 0 || self.pipe_count > 0 || self.sphere_count > 0;
```

**Replace with:**

```rust
        let solid = self.arena.verts() > 0 || self.pipe_count > 0 || self.sphere_count > 0;
```

**Gate.**

```bash
cargo check --target wasm32-unknown-unknown --lib
```

Still red — `src/app/scene.rs` writes the old column names. That is step 6.7.

### 6.6b Three stale claims in the comments this lesson is folding

Not a move, and not style: a wrong MEASURED number in a comment is a wrong fact, and `buffers.rs`
carries two. `rebind` has never existed, and twelve triples is thirty-six fields, not thirty-three.
While we are in there, the six-parameter exception gets written where the violation lives.

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// `new`/`append`/`rebind` at three parameters instead of four and lets one `let Gpu { ctx, .. }`
```

**Replace with:**

```rust
/// `new`/`append`/`mk_rows_group` at three parameters instead of four and lets one `let Gpu { ctx, .. }`
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// - and `Gpu` carries that triple twelve times over, which is thirty-three of its fields. Each
```

**Replace with:**

```rust
/// - and `Gpu` carries that triple twelve times over, which is thirty-six of its fields. Each
```

**Find** in `src/engine/gpu/buffers.rs`:

```rust
/// This is the same deal the mesh arena already struck, extended to the lanes that had not taken
```

**Replace with:**

```rust
/// Six parameters, against the house limit of five, and deliberately so until 49: three raw
/// cloud lanes in `mod.rs` are still a loose (buffer, count, cap) triple rather than a `GrowBuf`.
///
/// This is the same deal the mesh arena already struck, extended to the lanes that had not taken
```

And `Upload` gets the `Default` clippy asks for beside its `new`, plus two typos in the doc comment
that now describes the `obj` group.

**Find** in `src/engine/gpu/upload.rs`:

```rust
/// Everything `Gpu` needs to fill its buffers, built and owened by `app::scene::Scene`,
```

**Replace with:**

```rust
/// Everything `Gpu` needs to fill its buffers, built and owned by `app::scene::Scene`,
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
/// `objects` holds the TRUE per-object transfrom + tint + flags.
```

**Replace with:**

```rust
/// `obj.rows` holds the TRUE per-object transform + tint + flags.
```

**Find** in `src/engine/gpu/upload.rs`:

```rust
impl Upload {
    pub fn new() -> Self {
```

**Replace with:**

```rust
impl Default for Upload {
    fn default() -> Self {
        Self::new()
    }
}

impl Upload {
    pub fn new() -> Self {
```

### 6.7 The walk writes into sinks

Two literal conversions, the flag accesses that stop being `.2`, and then twelve `Replace-all`s
with their counts asserted. **If a count differs, you renamed the wrong thing** — stop and look
rather than adjusting the number.


**Find** in `src/app/scene.rs`:

```rust
use crate::engine::gpu::{Upload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint, mat_mul};
```

**Replace with:**

```rust
use crate::engine::gpu::{Upload, CloudDraw, LodNode, Instance, CylinderSegment, GlyphPoint, mat_mul};
use crate::engine::gpu::objects::ObjectBase;
```

**Find** in `src/app/scene.rs`:

```rust
self.tables.objects.push((place.m, [1.0; 4], 0));
```

**Replace with:**

```rust
self.tables.objects.push(ObjectBase { model: place.m, color: [1.0; 4], flags: 0 });
```

**Find** in `src/app/scene.rs`:

```rust
t.objects.push((placed, [1.0; 4], flags));
```

**Replace with:**

```rust
t.objects.push(ObjectBase { model: placed, color: [1.0; 4], flags });
```

**Replace-all** `src/app/scene.rs` `t.objects.last_mut().unwrap().2` -> `t.objects.last_mut().unwrap().flags` (3 hits)

**Find** in `src/app/scene.rs`:

```rust
                o.2 |= Instance::FLAG_SHEET;
```

**Replace with:**

```rust
                o.flags |= Instance::FLAG_SHEET;
```

**Replace-all** `src/app/scene.rs` `Some((xf, _, _)) = t.objects.get` -> `Some(ObjectBase { model: xf, .. }) = t.objects.get` (7 hits)

The renames. Longest key first, always: `t.object_bounds` before `t.objects`, `t.idx_print`
before `t.idx`, or the short one eats the long one's prefix.


**Replace-all** `src/app/scene.rs` `self.tables.object_bounds` -> `self.tables.obj.bounds` (1 hits)

**Replace-all** `src/app/scene.rs` `self.tables.object_spacing` -> `self.tables.obj.spacing` (1 hits)

**Replace-all** `src/app/scene.rs` `self.tables.objects` -> `self.tables.obj.rows` (3 hits)

**Replace-all** `src/app/scene.rs` `self.tables.verts` -> `self.tables.arena.verts` (2 hits)

**Replace-all** `src/app/scene.rs` `t.object_bounds` -> `t.obj.bounds` (13 hits)

**Replace-all** `src/app/scene.rs` `t.object_spacing` -> `t.obj.spacing` (13 hits)

**Replace-all** `src/app/scene.rs` `t.objects` -> `t.obj.rows` (13 hits)

**Replace-all** `src/app/scene.rs` `t.verts` -> `t.arena.verts` (7 hits)

**Replace-all** `src/app/scene.rs` `t.vids` -> `t.arena.vids` (7 hits)

**Replace-all** `src/app/scene.rs` `t.idx_print` -> `t.arena.idx_print` (2 hits)

**Replace-all** `src/app/scene.rs` `t.idx_text` -> `t.arena.idx_text` (2 hits)

**Replace-all** `src/app/scene.rs` `t.idx` -> `t.arena.idx` (5 hits)

### 6.8 The harnesses and the five shaders

`selftest` and the two `check_*` examples read `Upload` directly, so they follow the columns. The
`same!` macro has to widen from one identifier to a dotted path:


**Find** in `src/selftest.rs`:

```rust
        let (v, i) = (t.verts.len(), t.idx.len());
```

**Replace with:**

```rust
        let (v, i) = (t.arena.verts.len(), t.arena.idx.len());
```

**Find** in `src/selftest.rs`:

```rust
t.spheres.len(), t.verts.len());
```

**Replace with:**

```rust
t.spheres.len(), t.arena.verts.len());
```

**Find** in `examples/check_determinism.rs`:

```rust
macro_rules! same { ($f:ident) => {
```

**Replace with:**

```rust
macro_rules! same { ($($f:ident).+) => {
```

**Find** in `examples/check_lean.rs`:

```rust
macro_rules! same { ($f:ident) => {
```

**Replace with:**

```rust
macro_rules! same { ($($f:ident).+) => {
```

**Find** in `examples/check_determinism.rs`:

```rust
            if bytemuck::cast_slice::<_, u8>(&a.tables.$f) != bytemuck::cast_slice::<_, u8>(&b.tables.$f) {
                fails.push(format!("tables.{}", stringify!($f)));
```

**Replace with:**

```rust
            if bytemuck::cast_slice::<_, u8>(&a.tables.$($f).+) != bytemuck::cast_slice::<_, u8>(&b.tables.$($f).+) {
                fails.push(format!("tables.{}", stringify!($($f).+)));
```

**Find** in `examples/check_lean.rs`:

```rust
            if bytemuck::cast_slice::<_, u8>(&a.$f) != bytemuck::cast_slice::<_, u8>(&b.$f) {
                println!("  MISMATCH {}: {} vs {}", stringify!($f), a.$f.len(), b.$f.len()); ok = false;
```

**Replace with:**

```rust
            if bytemuck::cast_slice::<_, u8>(&a.$($f).+) != bytemuck::cast_slice::<_, u8>(&b.$($f).+) {
                println!("  MISMATCH {}: {} vs {}", stringify!($($f).+), a.$($f).+.len(), b.$($f).+.len()); ok = false;
```

**Find** in `examples/check_determinism.rs`:

```rust
        same!(verts); same!(idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Replace with:**

```rust
        same!(arena.verts); same!(arena.idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Find** in `examples/check_lean.rs`:

```rust
        same!(verts); same!(idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Replace with:**

```rust
        same!(arena.verts); same!(arena.idx); same!(segments); same!(pipes); same!(spheres); same!(glyphs);
```

**Find** in `examples/check_lean.rs`:

```rust
        if a.objects.len() != b.objects.len() { println!("  MISMATCH objects rows"); ok = false; }
```

**Replace with:**

```rust
        if a.obj.len() != b.obj.len() { println!("  MISMATCH objects rows"); ok = false; }
```

**Find** in `examples/check_lean.rs`:

```rust
        for (x, y) in a.objects.iter().zip(&b.objects) {
            if x.0 != y.0 || x.1 != y.1 || x.2 != y.2 { println!("  MISMATCH object row"); ok = false; break }
        }
```

**Replace with:**

```rust
        for (x, y) in a.obj.rows.iter().zip(&b.obj.rows) {
            if x.model != y.model || x.color != y.color || x.flags != y.flags { println!("  MISMATCH object row"); ok = false; break }
        }
```

And the comments in the shaders that cite a Rust path. These are load-bearing now: the mirror
tests make the two-language structs a checked contract, and a comment pointing at the wrong file
is how the next author fails to find the other half.


**Find** in `src/shaders/glyph.wgsl`:

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
```

**Replace with:**

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/frame.rs)
```

**Find** in `src/shaders/cylinder.wgsl`:

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
```

**Replace with:**

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/frame.rs)
```

**Find** in `src/shaders/sphere.wgsl`:

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
```

**Replace with:**

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/frame.rs)
```

**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
```

**Replace with:**

```wgsl
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/frame.rs)
```

**Find** in `src/shaders/ribbon.wgsl`:

```wgsl
// Instance::FLAG_SHEET - the row belongs to a planar drawing sheet (see gpu/mod.rs).
```

**Replace with:**

```wgsl
// Instance::FLAG_SHEET - the row belongs to a planar drawing sheet (see gpu/instance.rs).
```

**Find** in `src/shaders/triangle.wgsl`:

```wgsl
// Instance flag bit 3 (Instance::FLAG_PRINT in gpu/mod.rs): the mesh broadcast a zero edge
```

**Replace with:**

```wgsl
// Instance flag bit 3 (Instance::FLAG_PRINT in gpu/instance.rs): the mesh broadcast a zero edge
```
## 7. Proving nothing changed — four ladders

**(1) The compiler.** Both targets, all targets natively.

```bash
cargo check --target wasm32-unknown-unknown --lib
cargo check --all-targets --target x86_64-unknown-linux-gnu
```

Zero errors, and the warning list must be exactly the one lesson 46 left — nine of them, all in
`lib.rs` and `probe_mem.rs`, all predating this block. A NEW warning here is a real finding: it
means a symbol you moved is no longer reachable from where it is used. Two came up while this
lesson was written (`RenderVertex` and the `Mat4`/`mat_to_f32` re-export in `gpu/mod.rs`, both
now consumed by the new files instead), and both are fixed in step 6.4.

What it cannot catch: a body that lost a line while moving. It type-checks fine.

**(2) The tests.** New this lesson, and the only mechanical check on the Rust↔WGSL boundary.

```bash
cargo xtest
```

```text
test engine::gpu::instance::tests::instance_mirror ... ok
test engine::gpu::frame::tests::line_uniform_mirror ... ok
test result: ok. 2 passed; 0 failed
```

What they cannot catch: a flag BIT that disagrees. `instance_mirror` checks the struct, not
`const FLAG_PRINT = 8u`. Bits are still on you, which is why they are in one table.

**(3) The line multiset.** The compiler proves a Move type-checks and the goldens prove the
pixels agree; neither proves a Move was byte-identical, and a line dropped inside a `#[cfg]` arm
passes both.

```bash
python3 docs/_replay_check.py --moves <end-of-46 tree> /tmp/w47 docs/47-object-rows.md
```

```text
docs/47-object-rows.md: 120 ops, 0 failed
docs/47-object-rows.md: 2 move source(s), 0 not byte-identical
```

The two sources are `gpu/mod.rs` (the row and its constants, to `instance.rs`) and
`pipelines/build.rs` (the vertex-id layout, to `arena.rs`). Everything else this lesson does is a
Create plus a Remove, and `--moves` accounts for those against the Create bodies — which is why
the run also prints a `lost-declared` list of 333 lines and passes: those lines are declared gone
by an op you typed, not dropped on the floor.

**(4) The pixels, and the two harnesses that go where pixels do not.**

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

Four mandatory scenes × four configs × two passes, against `docs/_GOLDENS.tsv`:
`lion` 77543/4/1 · `bunny` 44215/9/6 (tubes 43954/8/6) · `bunny_cloud` 7511/4/1 ·
`drawings_rotated` 25043/10/155465 sha `8c339ef1c45a1e39` (tubes 24970/9 sha `0436f04fe5fc5c7c`).
`drawings_rotated` is the one mandatory row a checksum still gates — the other three carry
`nondet(splat)`/`nondet(mesh)` and are gated on ink, draws and objects. `objects` is the count
this lesson is about, so a wrong number there is a wrong object table, immediately.

## 8. What you can now do in one line

Take a flag bit. Before this lesson that meant finding `Instance` two thirds of the way down a
1,691-line file, guessing which of five shaders declare it, and finding out at runtime — as a
wrong colour, or as nothing at all — whether you guessed right. Now the const, the free-bit
budget and the list of shaders that mirror it are one screen, and a test tells you which shader
you forgot.

**Type all eight steps below.** The first four add the bit, the last four take it back out — this
is a demonstration, not part of the lesson's end state, and the tree must be back to what §6 left
before you read §10. Do **not** undo it with `git checkout`: you have not committed lesson 47 yet,
and that command would throw the whole lesson away.

**8a.** Bit 6, straight off the free budget in the table. **Find** in `src/engine/gpu/instance.rs`:

```rust
    pub const FLAG_SHEET: u32 = 1 << 5;
```

**Replace with:**

```rust
    pub const FLAG_SHEET: u32 = 1 << 5;

    /// Bit 6, taken from the free budget in the table below: light this row's faces green.
    pub const FLAG_DEBUG: u32 = 1 << 6;
```

**8b.** Set it on every row, in the one place rows are built. **Find** in `src/engine/gpu/objects.rs`:

```rust
            flags: o.flags,
```

**Replace with:**

```rust
            flags: o.flags | Instance::FLAG_DEBUG,
```

**8c.** The shader's copy of the bit. **Find** in `src/shaders/triangle.wgsl`:

```wgsl
const FLAG_PRINT = 8u;
```

**Replace with:**

```wgsl
const FLAG_PRINT = 8u;
const FLAG_DEBUG = 64u;
```

**8d.** And read it, in the vertex shader, where the instance row is already in hand. **Find** in
`src/shaders/triangle.wgsl`:

```wgsl
    o.color = in.color.rgb * inst.color.rgb; // baked base color x instance tint (white today)
```

**Replace with:**

```wgsl
    o.color = in.color.rgb * inst.color.rgb * select(vec3<f32>(1.0), vec3<f32>(0.2, 1.0, 0.2), (inst.flags & FLAG_DEBUG) != 0u);
```

Render the bunny:

```bash
cargo run -q --release --example selftest --target x86_64-unknown-linux-gnu -- \
    /tmp/dbg.ppm assets/scenes/bunny.toml
```

```text
[INFO] headless frame: 9 draws, 6 objects, 900x700
wrote /tmp/dbg.ppm  900x700  non-background pixels: 46976 (7.5%)
```

Green bunny. Counted the same way both times — a pixel whose green channel leads both others by
25 — it goes from **2,207 to 30,724**. Four lines: one const in Rust, one const in WGSL, one
field expression, one shader expression. Nothing else in the program knew, and the free-bit
budget in the table is now two instead of three. Put it back:

**8e.** **Find** in `src/shaders/triangle.wgsl`:

```wgsl
    o.color = in.color.rgb * inst.color.rgb * select(vec3<f32>(1.0), vec3<f32>(0.2, 1.0, 0.2), (inst.flags & FLAG_DEBUG) != 0u);
```

**Replace with:**

```wgsl
    o.color = in.color.rgb * inst.color.rgb; // baked base color x instance tint (white today)
```

**8f.** Both lines together, because a delete verb would leave a blank line behind. **Find** in
`src/shaders/triangle.wgsl`:

```wgsl
const FLAG_PRINT = 8u;
const FLAG_DEBUG = 64u;
```

**Replace with:**

```wgsl
const FLAG_PRINT = 8u;
```

**8g.** **Find** in `src/engine/gpu/objects.rs`:

```rust
            flags: o.flags | Instance::FLAG_DEBUG,
```

**Replace with:**

```rust
            flags: o.flags,
```

**8h.** **Find** in `src/engine/gpu/instance.rs`:

```rust
    pub const FLAG_SHEET: u32 = 1 << 5;

    /// Bit 6, taken from the free budget in the table below: light this row's faces green.
    pub const FLAG_DEBUG: u32 = 1 << 6;
```

**Replace with:**

```rust
    pub const FLAG_SHEET: u32 = 1 << 5;
```

The point is that the diff was four lines in four files and the compiler and the test between
them told you about the fifth file you would otherwise have missed.

## 9. What is deliberately not here

- **`RowTable<T>` — the generic growable table.** `GrowBuf` is a struct of three fields, not a
  generic that owns its rows. Three families is not enough evidence for the abstraction; lesson
  **57** has five and takes it then.
- **`upload_rows` as one call.** Each lane still appends itself. Lesson **62**.
- **A `Frustum` on the object table.** `bounded_rows` + `update_inside_flags` is a point-in-box
  test, not a cull. Lesson **62**.
- **The `Upload.seg`, `Upload.glyph` and `Upload.cloud` groups.** Two of eight groups this
  lesson, the rest with the families that consume them — 48 and 49. Regrouping a column before
  its family exists means renaming it twice.
- **`Instance::new`.** The struct literal in `InstanceTable::append` is the only one, and a
  constructor for a single call site is a name to look up, not a simplification. Lesson **59**
  adds the second one, and it can add the constructor.
- **The `objects_base` → `base` rename in `Upload`.** The walk still calls its column
  `obj.rows`. Lesson **50** rewrites those producers wholesale and names them there.
- **`persistence.rs`.** Declared over cap at 453 lines since lesson 43; the three-way split is
  lesson **59**.

And the standing rule: **a body you are moving is not a body you are fixing.** This lesson breaks
it exactly once, in `InstanceTable::clear`, and says so in §4.2 with the reason.

## 10. Expected state

```bash
cd session_viewer
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
wc -l src/engine/gpu/mod.rs src/engine/gpu/instance.rs src/engine/gpu/objects.rs src/engine/gpu/arena.rs
grep -rln 'objects_base\|bounded_rows\|arena_ibo' src/
grep -c 'instance_id_layout\|INSTANCE_ID_ATTRIBS' src/engine/pipelines/build.rs
cargo xtest 2>&1 | tail -3
```

```text
63

  1335 src/engine/gpu/mod.rs
   159 src/engine/gpu/instance.rs
   341 src/engine/gpu/objects.rs
   303 src/engine/gpu/arena.rs

src/math.rs
src/engine/gpu/arena.rs
src/engine/gpu/objects.rs

0

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

| | end-of-46 | end-of-47 |
|---|---|---|
| `Gpu` fields | 86 | **63** |
| `gpu/mod.rs` | 1,691 | **1,335** |
| `gpu/instance.rs` | — | **159** |
| `gpu/objects.rs` | — | **341** |
| `gpu/arena.rs` | — | **303** |
| `gpu/buffers.rs` | 140 | **132** |
| `gpu/upload.rs` | 110 | **98** |
| `gpu/frame.rs` | 177 | **224** |
| `pipelines/mod.rs` | 148 | **130** |
| `pipelines/build.rs` | 215 | **199** |
| `app/scene.rs` | 1,340 | 1,341 |
| `Upload` columns, flat | 19 | **11 + 2 groups** |
| `append_index_run` params | 6 | **3** |
| `#[test]` in `src/` | 0 | **2** |

## Recap

```text
Lesson 45 made a pipeline a value: eleven builders at three to eleven parameters became one
`build` and fourteen `PipelineDesc` literals, and `Gpu` lost ten fields with the layouts.
Lesson 46 put a floor under the families: `GpuCtx` instead of a device and a queue held
separately, `GrowBuf` for the triple every lane spells out, and six files - buffers, upload,
view, frame, targets, present - for everything that belongs to no lane. `Gpu` went from 106
fields to 86.

Lesson 47 is the first FAMILY, and it starts with the thing every family points at. One row
per object, one table that owns it, and an `instance_id` everywhere else. The row went to
`instance.rs` because five shaders declare it and now a test says so; the table went to
`objects.rs` with the rebase, the anchor and the eye-inside test that are the only reasons it
is not just a Vec; and the triangle family - one vertex table, three index runs, two pipelines,
three draws - went to `arena.rs` and took its shader constant and its vertex layout with it.
`Gpu` is 63 fields, and 25 of the ones it lost were two shapes written out longhand.

The law: a family may not build or renumber an object row. Grep for `bounded_rows` and you get
one file.
```

## Edited

`src/engine/gpu/instance.rs` (NEW — `Instance`, the flag table, `wgsl_fields`, `instance_mirror`) ·
`src/engine/gpu/objects.rs` (NEW — `ObjectBase`, `ObjectRows`, `InstanceTable`) ·
`src/engine/gpu/arena.rs` (NEW — `ArenaRows`, `Arena`, `IdxLane`, `Pipes`, the three draws) ·
`src/engine/gpu/mod.rs` (25 fields → 2; `set_scene` → two calls; the draws delegate) ·
`src/engine/gpu/buffers.rs` (`GrowBuf` live; `append_index_run` 6 params → 3) ·
`src/engine/gpu/upload.rs` (two groups) ·
`src/engine/gpu/frame.rs` (`line_uniform_mirror`) ·
`src/engine/pipelines/mod.rs` (`arena: arena::Pipes`) ·
`src/engine/pipelines/build.rs` (the vertex-id layout leaves) ·
`src/app/scene.rs` (twelve `Replace-all`s and the tuple named) ·
`src/selftest.rs`, `examples/check_determinism.rs`, `examples/check_lean.rs` ·
the five `.wgsl` (comments only).

## Reference

The implementation this lesson was written from was built in one sitting and gated twice:

| checkpoint | what landed |
|---|---|
| 47a | `gpu/instance.rs` — the row, the flag table, both mirror tests |
| 47b | `gpu/objects.rs` — `ObjectBase`, `ObjectRows`, `InstanceTable` and its five methods |
| 47c | `gpu/arena.rs` — three `GrowBuf`s, `IdxLane`, the family's `Pipes` and its draws |
| 47d | `Upload`'s two groups and the twelve `Replace-all`s in the walk |
| 47e | `Gpu`'s field list, constructor, `set_scene` and draws |

`git diff end-of-46..end-of-47 -- session_viewer/src` is the whole lesson as one patch; `diff -u`
any single file against it if a line count comes out wrong.

## Next

Lesson **48** — **one row, two shaders.** Run the evidence:

```bash
grep -cE '^\s+(pub )?[a-z_0-9]+\s*:' <(sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs)
sed -n '/^pub struct Gpu/,/^}/p' src/engine/gpu/mod.rs | grep -cE 'pipe_|segment_|sphere_|glyph_|cyl_|sph_'
grep -c 'CylinderSegment' src/engine/gpu/mod.rs src/app/scene.rs
```

63 fields, and 22 of them are two lanes of the same row: `pipe_*` and `segment_*` hold identical
`CylinderSegment` tables read by `ribbon.wgsl` and `cylinder.wgsl` through one layout, with the
choice made at one draw site. That is not two families. It is one module and five pipelines — and
`sphere`/`glyph` is the same shape, which is why you write the second half.
