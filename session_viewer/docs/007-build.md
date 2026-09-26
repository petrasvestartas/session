# 007 · Describe and build a pipeline

A lane says what it wants in a few words: this shader, these bind groups, triangles or lines, opaque or blended, write depth or only test it. `PipelineDesc` is those words. `build` turns them into a key, asks the cache, and returns the `Lazy` pipeline. `compile` is the one place in the viewer that calls `create_render_pipeline`.

## The description

`lessons/007/src/engine/pipelines/mod.rs` · type this, append

`PipelineDesc` is the description: label, shader, entry points, bind group layouts in slot order, vertex buffers, topology, colour and depth. `new` is the common base; `with`, `vertex`, `color` and `depth` are copies with one thing changed, so a lane's pipelines read as one line each.

```rust
--8<-- "lessons/007/src/engine/pipelines/mod.rs:007-desc"
```

## Compile through the cache

`lessons/007/src/engine/pipelines/mod.rs` · type this, append

`wgsl` makes a shader module for a source text, once per label and hash, lazily. `layout` makes a bind group layout once per label and entries. `pipeline_layout` puts the group layouts in slot order. `build` copies the description into an owned `PipelineKey`, returns the cached pipeline if the key is known, else stores a `Lazy` that will call `compile`. `compile` fills the wgpu descriptor: vertex state with its buffers, fragment state with one colour target, the primitive state with no culling, the depth state from the mode, and the sample count from the target. `count_pipeline` ticks the counter the tests read.

```rust
--8<-- "lessons/007/src/engine/pipelines/mod.rs:007-build"
```

## Checkpoint

Run `cargo check` and `cargo xtest`: the lazy test still passes and nothing else changed. The canvas is still grey. You have the whole path from a description to a compiled pipeline; the next lesson writes the first shader and the first description, and draws with it.

## Recap

A lane describes a pipeline with `PipelineDesc`; `build` keys the cache with an owned copy and hands back a `Lazy`; `compile` runs once, on first use. Next: shaders in the crate.

Next: [008 · Shaders in the crate](008-shaders.md)
