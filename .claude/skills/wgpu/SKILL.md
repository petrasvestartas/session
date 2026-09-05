---
name: wgpu
description: Read before writing or changing ANY wgpu, WGSL, WebGPU, naga, or session_viewer GPU code. wgpu ships breaking changes nearly every release, so the fluent-looking call a model recalls from training is usually from an older version and will not compile here; WGSL layout bugs are worse because they produce a black canvas instead of an error. Use this whenever the task touches shaders, pipelines, bind groups, buffers, textures, render passes, surface/device setup, GPU picking, alignment/padding of host-shareable structs, a "why is nothing drawing / everything is black" symptom, a wgpu version bump, or any file under session_viewer/src/engine/ or session_viewer/src/shaders/. Use it even when the change looks like a one-liner, and even when a tutorial or an earlier answer already supplied code.
---

# wgpu / WebGPU

## Why this exists

Two failure modes, both expensive, both invisible to the normal agent loop:

1. **Stale API.** Any training snapshot mixes many wgpu versions. The pattern a
   model recalls most often is fluent, idiomatic, and wrong for the pin. Being
   told "wgpu changed" is not enough — the correct signature has to be in
   context *before* the first line is written.
2. **Silent WGSL bugs.** A misaligned struct raises no error. It renders black,
   or writes garbage into a storage buffer. "Run it, read the error, fix it"
   has no error to read. The feedback-loop section below is what restores the
   signal.

## Step 1 — state the pin, out loud, before writing code

```bash
grep -rn '^wgpu' session_viewer/Cargo.toml session_rust/Cargo.toml
grep -A1 'name = "wgpu"' session_viewer/Cargo.lock
```

At the time this file was written: `wgpu = "29.0"` in both manifests,
resolving to **29.0.4** (viewer) and 29.0.3 (kernel). Everything below is
scoped to 29. If the lock disagrees with this paragraph, the lock wins — fix
this paragraph and re-read the changelog delta before touching code.

## Step 2 — read the vendored source, not a web page

The exact API this build resolves to is already on disk. It cannot be a version
mismatch, which is a promise no URL can make.

```bash
ls -d ~/.cargo/registry/src/*/wgpu-*/          # e.g. .../wgpu-29.0.4/
rg 'pub fn create_bind_group' ~/.cargo/registry/src/*/wgpu-29.0.4/src/
```

`src/api/*.rs` holds the signatures; `wgpu-types-*/src/` holds `Limits`,
`Features`, `TextureFormat`. **Never write a wgpu call from memory** — grep it
and quote the line you found, with its path.

The second ground truth is this repo. `session_viewer/src/engine/` compiles
today against the pin, so a pattern copied from there is correct by
construction. Prefer it over anything external.

## Step 3 — verified facts at the pin

These are the calls a model most reliably gets wrong. Each was grepped out of
`wgpu-29.0.4` or `session_viewer/src`; `references/verified_29.md` has the file
and line for every one, plus the command to re-verify after a bump.

- `bind_group_layouts: &'a [Option<&'a BindGroupLayout>]` — every entry wraps
  in `Some`. A bare `&[&layout]` is the pre-29 shape and will not compile.
- `PipelineLayoutDescriptor` has an `immediate_size: u32` field (push constants
  became "immediate data"). `..Default::default()` does not cover it in a
  struct literal that names the other fields.
- `Instance::new(desc)` takes `InstanceDescriptor` **by value**, not by
  reference, and `InstanceDescriptor` has a `display` field.
- `request_adapter` / `request_device` return
  `impl Future<Output = Result<..>>` — `.await?`, no `pollster::block_on`, no
  `.unwrap()` on an `Option`.
- `max_inter_stage_shader_components` is gone; it is
  `max_inter_stage_shader_variables` and it counts locations, not scalars.
- `MapMode::Write` mappings do not deref to `&mut [u8]`. `BufferViewMut::slice`
  returns `wgpu::WriteOnly<'_, [u8]>`.
- `@builtin(primitive_index)` needs `enable primitive_index;` in the WGSL and
  the `PRIMITIVE_INDEX` feature, which lives in the WebGPU feature set.
- `TextureFormat::describe().srgb()` is long gone — `format.is_srgb()`.
- This viewer is **WebGPU-only in the browser**: `Backends::BROWSER_WEBGPU`,
  and the `webgl` cargo feature is deliberately *not* enabled. Do not
  reintroduce a WebGL2 fallback path or `downlevel_webgl2_defaults()`.

## Step 4 — host-shareable structs

`vec3<T>` has size 12 and alignment 16. That one fact causes more lost hours
than the rest of the API combined, and it fails silently.

When a change touches a struct shared between Rust and WGSL, write the computed
offsets next to it in the diff so the reader can check them without running
anything:

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Instance {   // offset  size
    model: [[f32; 4]; 4],  //   0    64
    color: [f32; 4],       //  64    16
    flags: u32,            //  80     4
    thickness: f32,        //  84     4
    spacing: f32,          //  88     4
    _pad: f32,             //  92     4   -> 96, a multiple of 16
}
```

Prefer `vec4`/`mat4` over `vec3`/`mat3` in anything host-shareable; the padding
you would otherwise hand-maintain disappears. Flat scalar runs (as in
`LineUniform` in `src/shaders/triangle.wgsl`) are also safe — the trap is
specifically a `vec3` followed by anything.

Check with <https://www.w3.org/2025/webgpu/wgsl-align.html> (paste the WGSL,
read the offsets). It ignores arrays and nested user structs, so it confirms a
layout rather than proving one.

## Step 5 — run the loop that already exists

This repo has the guardrails the general advice asks you to build. Use them
instead of proposing `wgsl_to_wgpu` or a new harness.

```bash
cd session_viewer
cargo check                       # defaults to wasm32 via .cargo/config.toml
cargo xtest                       # native tests (alias for --target x86_64-...)
cargo run --example selftest --target x86_64-unknown-linux-gnu --release -- out.ppm assets/view_local.yaml
```

`selftest` renders one headless frame, writes a PPM and prints an ink count, so
a shader regression shows up as a number and an image rather than a shrug.
`VIEWER_FRAMES`, `VIEWER_W`/`VIEWER_H` and `VIEWER_PICK="x,y"` steer it.
`examples/selftest.rs` installs a `log` sink on purpose: **wgpu reports
validation errors through `log`, and without a logger a broken shader just
renders black.** Keep that in any new harness.

Run `cargo check` after every edit, before proposing the next one. Rust's type
system catches most wgpu misuse for free, but only if the loop is actually run.

If something fails silently, add the check that makes it fail loudly *before*
attempting the fix — a pixel readback, a known-answer compute dispatch, an ink
count. A fix validated by "it looks right now" is not validated.

## Sources, ranked

1. Vendored source at the pin — `~/.cargo/registry/src/*/wgpu-29.0.4/`, plus
   its own `examples/`.
2. This repo — `session_viewer/src/engine/`, `src/shaders/`.
3. Changelog, read **from the pin forward only**, never the whole file:
   <https://github.com/gfx-rs/wgpu/blob/trunk/CHANGELOG.md>. It is written as
   before/after diffs, which makes it the best migration document there is.
   Per-version chunks: <https://github.com/gfx-rs/wgpu/releases>.
4. Docs at the pinned version: `https://docs.rs/wgpu/29.0.4/wgpu/`. Never
   `/latest/` — that reintroduces exactly the mismatch this file prevents.
5. WGSL spec <https://www.w3.org/TR/WGSL/> (the layout and alignment sections)
   and WebGPU spec <https://www.w3.org/TR/webgpu/> for *why* something is
   rejected.
6. Trunk examples <https://github.com/gfx-rs/wgpu/tree/trunk/examples> — good
   for compute and wasm, but they track trunk, so cross-check every call
   against the vendored source before copying.

### Do not read

**learn-wgpu** (`sotrh.github.io/learn-wgpu`) and tutorial blog posts. They are
the most SEO-visible wgpu resources, therefore the most heavily represented in
training data, and they are pinned to versions from years ago. Reading them
reinforces precisely the stale patterns this file exists to correct. If a
tutorial is genuinely the only source for a technique, take the *idea* from it
and verify every call against the vendored source.

## Rules

- State the pinned version before writing code. If the lock cannot be read,
  stop and ask rather than guessing.
- Quote the signature you grepped, with its path. No calls from memory.
- Show computed offsets alongside any host-shareable struct you touch.
- Say so explicitly when you are unsure whether an API is current. A hedge
  costs one line; confident wrong code costs a debugging session.
- On a version bump: read the changelog from the old pin forward, then
  re-verify every entry in `references/verified_29.md` and rewrite it.

## Reference files

- `references/verified_29.md` — every claim above with its source file and
  line, and the commands to re-verify them after a bump.
- `references/patterns.md` — viewer-specific rendering techniques researched
  for this project: thick lines, point clouds, GPU colour-ID picking, gizmos,
  egui overlay, instancing and culling, wasm gotchas, kernel integration. Read
  it when implementing one of those; the API snippets in it are older than the
  pin and must be checked against the vendored source.
