# 03 · Object rows and identity

## You are building

![Diagram: SourceObject\ guid · revision · Instance\ model · color · flags · instances[] · vs_main(instance_index) · draw(0..3, row..row+1) · two tinted, placed triangles](illustrations/03-01.svg)

![A repr(C) struct is cast to bytes, written to a buffer, attached by a bind group at a group and binding, and declared again in WGSL.](illustrations/gpu-data.svg)

![The 96-byte Instance layout and the vec3 alignment trap.](illustrations/vertex-layout.svg)

## Starting point

- Checkpoint 02: one triangle, one camera uniform.
- Same geometry drawn twice with different placement and tint. Identity lives on the CPU; the GPU only sees rows.

<!-- step-status: start -->

**Does it compile yet?** Yes — `cargo check` was run at the end of every step of this lesson.

<!-- step-status: end -->

## Step 1 · The object row

![Where this step sits in the viewer: GPU core, with 6 of 11 zones built so far.](illustrations/locator-afd0463e4d.svg){ .locator data-strip="illustrations/strip-10d43625b6.svg" }

- One 96-byte record per object, indexed by `instance_index` in every instance-reading shader.
- Flags are bits: selecting sets bit 0 and leaves the rest.
- Here the placement sits in `model`'s translation column. From 04a on, production zeroes that column and moves the anchored translation to its own table (group 2, binding 1).
- The size assertion is compile-time: a wrong stride fails `cargo check`, not the picture.

![Diagram: Instance::placeholder · struct Instance\ 96 B · one object row · flags](illustrations/03-02.svg)

<span class="zone-mark" data-strip="illustrations/strip-10d43625b6.svg" data-zone="GPU core"></span>

<!-- file: 03 session_viewer/src/engine/gpu/instance.rs type lines=1-57 -->

The rest is `#[cfg(test)]`: naga parses every lane shader and checks WGSL member offsets against the Rust ones. The browser build never compiles it.

<span class="zone-mark" data-strip="illustrations/strip-10d43625b6.svg" data-zone="GPU core"></span>

<!-- file: 03 session_viewer/src/engine/gpu/instance.rs copy lines=58-236 -->

## Step 2 · Declare the engine module tree

![Where this step sits in the viewer: GPU core, with 6 of 11 zones built so far.](illustrations/locator-afd0463e4d.svg){ .locator data-strip="illustrations/strip-10d43625b6.svg" }

![Diagram: lib.rs · engine/mod.rs · engine/gpu/mod.rs · instance.rs](illustrations/03-03.svg)

<span class="zone-mark" data-strip="illustrations/strip-10d43625b6.svg" data-zone="GPU core"></span>

<!-- file: 03 session_viewer/src/engine/gpu/mod.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-10d43625b6.svg" data-zone="GPU core"></span>

<!-- file: 03 session_viewer/src/engine/mod.rs type -->

- One line per module: Rust compiles a file only once a `mod` names it, so the row you typed enters the build here.

## Step 3 · Source identity is separate from the row

![Where this step sits in the viewer: Scene + walk, with 7 of 11 zones built so far.](illustrations/locator-f9a61bc664.svg){ .locator data-strip="illustrations/strip-3589a2f087.svg" }

- A `guid` and `revision` identify what the object *is*; the row says how it is drawn this revision.
- Picking returns a row; the scene maps it back. Never search for an object by matching triangle positions.

![Diagram: SourceObject · guid · revision · Instance · scene::objects()](illustrations/03-04.svg)

<span class="zone-mark" data-strip="illustrations/strip-3589a2f087.svg" data-zone="Scene + walk"></span>

<!-- file: 03 session_viewer/src/scene.rs type -->

<!-- check: 03 -->

## Step 4 · Rust layout ↔ WGSL layout

![Where this step sits in the viewer: Shaders, with 7 of 11 zones built so far.](illustrations/locator-1378bac81b.svg){ .locator data-strip="illustrations/strip-9d7fcc8d70.svg" }

Same bytes on both sides, read through different type systems:

```text
Rust `Instance`            offset   WGSL `struct Instance`
model: [f32; 16]              0     model: mat4x4<f32>
color: [f32; 4]              64     color: vec4<f32>
flags: u32                   80     flags: u32
_pad0: f32                   84     thickness: f32
spacing: f32                 88     spacing: f32
_pad: u32                    92     pad: u32
size                         96     array stride
```

- `@builtin(instance_index)` is the `row` of `draw(0..3, row..row + 1)`.
- `@group(0) @binding(1) var<storage, read>` mirrors the `BufferBindingType::Storage { read_only: true }` entry added in the next step.

![Diagram: Instance rows · storage instances[] · instance_index · vs_main · fs_main](illustrations/03-05.svg)

<span class="zone-mark" data-strip="illustrations/strip-9d7fcc8d70.svg" data-zone="Shaders"></span>

<!-- file: 03 session_viewer/src/shaders/first.wgsl type -->

## Step 5 · Bind the rows and draw each one

![Where this step sits in the viewer: Page, Shell, with 7 of 11 zones built so far.](illustrations/locator-b32fcaf1b6.svg){ .locator data-strip="illustrations/strip-e2a9d1f0c8.svg" }

- The layout gains binding 1; the bind group supplies the storage buffer; one draw per row.
- `objects` stays on the CPU side of the shell, so the status counts source data, not GPU rows.

![Diagram: scene::objects() · STORAGE buffer · BindGroup · draw(0..3, row..row+1)](illustrations/03-06.svg)

<span class="zone-mark" data-strip="illustrations/strip-3d2a8d385e.svg" data-zone="Shell"></span>

<!-- file: 03 session_viewer/src/lib.rs type -->

<span class="zone-mark" data-strip="illustrations/strip-7b9fa61633.svg" data-zone="Page"></span>

<!-- file: 03 session_viewer/index.html copy -->

![A draw call carries two ranges: vertex_index walks the three corners, instance_index walks the object rows, and every invocation reads only the row its instance_index names - so a hundred objects are one call and one buffer.](illustrations/instancing.svg)

## Check

<!-- checkpoint: 03 -->

Expected:

- Two triangles, one orange on the left, one blue on the right.
- Status reads **Checkpoint 03 · 2 objects**.
- Orbit and zoom: the two keep their relative placement.

A wrong stride shows as a correct first object and a corrupt second one. A wrong source map looks fine until selection returns the other object.

![Checkpoint 03: one triangle geometry drawn twice through two object rows, each with its own placement and tint.](screenshots/03.png)

## What changed

<!-- tree: 03 session_viewer/src -->

- `engine::gpu::instance::Instance` is the row contract between scene and shaders.
- Data flow: `SourceObject.row` → storage buffer → `instances[instance_index]` → placed, tinted vertex.

**Production equivalent:** `src/engine/gpu/instance.rs`. Production keeps the scene in `src/app/scene.rs`.

## Try

- Change the second row's `color` in `scene.rs`: only that triangle changes, because tint lives in the row, not in the geometry.
- Set the same `model[12]` for both rows: they overlap exactly, because nothing about the geometry is per row — both rows drive the same three positions `vs_main` builds from `vertex_index`.
- Draw with `draw(0..3, 0..1)` only: the second row vanishes, because `instance_index` never reaches 1.

## Questions and answers

**`Instance` has a matrix, a colour, some flags and a spacing. Why does it occupy 96 bytes?**

*How to work it out.* Add the fields: 64 for the matrix, 16 for the colour, 4 + 4 + 4 for the rest — 92. A `mat4x4` requires 16-byte alignment, and a storage-array element must start at a multiple of the struct's largest alignment, so the stride rounds to the next multiple of 16.

*The answer.* 96, because 92 rounds up to 96. The explicit padding field on the Rust side makes the round-up deliberate instead of accidental.

**A GPU row is not a source identity. What is the difference, and why keep both?**

*How to work it out.* Ask what each one survives. Rows are rebuilt and renumbered whenever the scene reloads. A guid is written in the document and must mean the same thing next week. Anything a user is told about has to be the second kind.

*The answer.* A row is 96 bytes of drawing state at an index in a buffer; an identity is the guid and revision of a thing in the document. The GPU can only answer with a row, so `Scene` turns that row back into something nameable. Collapse them and either your rows must never move or your identities are not stable — both unacceptable.

**Which index reaches the shader's `instances[]`, and what sets it?**

*How to work it out.* Look at what the draw call takes: a vertex range and an instance range. The shader reads two builtins. One counts vertices; by elimination the other counts instances.

*The answer.* `@builtin(instance_index)`, set by the draw's instance range: `draw(0..3, 1..2)` runs the vertex stage with `instance_index == 1`. Nothing is bound per object — one buffer, one index — so a thousand objects cost one bind and a thousand indices.

**What you should be able to do now**

Write the `#[repr(C)]` row and its size assertion in an empty file without looking, then compare with `instance.rs`. Correct: `model: [f32; 16]`, `color: [f32; 4]`, `flags: u32`, a padding `f32`, `spacing: f32`, a trailing padding `u32`, `#[repr(C)]`, `Clone`/`Copy`/`Pod`/`Zeroable`, and the 96-byte size assertion. If your field order differs, ask whether the shader would still work — `#[repr(C)]` makes that question answerable, because without it Rust may reorder the fields.

## Next

[04a · Meshes on the GPU](04a-meshes.md): vertex and index buffers, the mesh arena, and the first real drawing module.
