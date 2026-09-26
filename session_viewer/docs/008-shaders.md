# 008 · Shaders in the crate

A shader is a small program that runs on the GPU, written in WGSL, the WebGPU shading language. Ours live in `src/shaders/`, and the build pastes them into the binary. Every scene shader ends with the same shared text: the bind groups every draw starts with, the pen settings, and the helpers. This lesson writes that shared text, the build step that strips it, and the bind group layouts the Rust side fills.

## Strip the shaders before they ship

`lessons/008/build.rs` · type this, new file

A build script is a program cargo runs before compiling the crate. Ours copies each `.wgsl` file into cargo's output folder, `OUT_DIR`, with `#include "file"` lines replaced by that file and comments, indentation and blank lines removed. The shaders reach the browser smaller, while the source keeps its comments.

```rust
--8<-- "lessons/008/build.rs"
```

## Load a shader by name

`lessons/008/src/lib.rs` · type this, the macro tagged `register:shaders`, between the entry function and the modules

`shader!("scene.wgsl")` is the text of that file as `build.rs` wrote it, pasted in at compile time by `include_str!`.

```rust
--8<-- "lessons/008/src/lib.rs:008-macro"
```

## The shared scene code

`lessons/008/src/shaders/scene.wgsl` · type this, new file

Group 0 is the camera matrix, group 1 the pen and view settings. `LineUniform` here mirrors the Rust struct of the next lesson field for field, 80 bytes; the shader reads what the Rust side wrote. Later lessons add the object rows at group 2 and the flags and helpers that read them.

```wgsl
--8<-- "lessons/008/src/shaders/scene.wgsl"
```

## Normals and outputs

`lessons/008/src/shaders/normals.wgsl` · type this, new file

Helpers every shader may need: rotate a normal by an object's matrix, exactly even when the matrix mirrors or scales, and unpack normals stored as two bytes or two halves.

```wgsl
--8<-- "lessons/008/src/shaders/normals.wgsl"
```

`lessons/008/src/shaders/physical.wgsl` · type this, new file

The output of a face fragment: its colour. `@location(0)` is the first colour target of the pipeline.

```wgsl
--8<-- "lessons/008/src/shaders/physical.wgsl"
```

## Assemble a shader

`lessons/008/src/engine/pipelines/mod.rs` · type this, append

`PRELUDE` is the list of shared snippets every scene shader ends with; a shared snippet is one file and one line here. `scene_source` appends them, `shared` appends the normals and outputs, and `module` and `scene_module` hand the text to the cache.

```rust
--8<-- "lessons/008/src/engine/pipelines/mod.rs:008-shaders"
```

## The bind group layouts

`lessons/008/src/engine/pipelines/layouts.rs` · type this, new file

A bind group layout says what a slot holds: a uniform buffer, a storage buffer, a texture, and which shader stages see it. `Layouts` makes each once, for every lane to share: group 0 with the camera matrix, group 1 with the pen settings. The tagged lines are the layouts of later lanes.

```rust
--8<-- "lessons/008/src/engine/pipelines/layouts.rs:008-layouts"
```

## Checkpoint

Run `cargo check`. It runs `build.rs` first; look in `target/wasm32-unknown-unknown/debug/build/session_viewer-*/out/shaders/` and open `scene.wgsl`: your shader without a comment or an indent. The canvas is still grey; nothing draws with these yet.

## Recap

Shaders are WGSL files that `build.rs` strips and `shader!` pastes into the binary. Every scene shader ends with the shared prelude: groups 0 and 1, `LineUniform`, normals, outputs. `Layouts` makes the bind group layouts once. Next: the buffers behind group 0 and 1, written every frame.

Next: [04a · Meshes on the GPU](04a-meshes.md)
