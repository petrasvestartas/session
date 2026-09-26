# 006 · The pipeline cache

A pipeline is the GPU's recipe for one kind of draw: which shader, which vertex layout, which blend, which depth test. Compiling one takes milliseconds to seconds, and the viewer needs dozens of them, several per lane, and every one again whenever the sample count flips. So no lane compiles a pipeline itself. It describes one, and a cache hands back the same object for the same description, compiled once, on its first use.

This lesson writes the cache, the lazy object it stores, and the two small enums a description is made of.

## Count what was compiled

`lessons/006/src/engine/pipelines/mod.rs` · type this, new file

Two thread-local counters, one for pipelines and one for shader modules. The tests read them: a frame that compiles nothing new must leave them unchanged.

`Lazy<T>` is a GPU object made on its first use. `LazyCell` holds the closure that makes it; `Rc` lets clones share the one object; `Deref` makes it now if it has not been made yet. Equality means the same object, and `Hash` follows, so a `Lazy` can be a key.

`Cache` is three maps: shaders by label and source hash, bind group layouts by label and entries, pipelines by everything they compile from.

`DepthMode` names the depth tests the viewer uses. Depth is reverse-Z, so nearer is greater: `Opaque` writes and lets the nearer fragment win, `ReadOnly` tests without writing, `Always` ignores depth, `Detached` has no depth texture at all. `ColorWrite` names the blends: overwrite, alpha blend, or nothing when the shader has side effects. Each has a `state` that says the pair wgpu wants.

`PipelineKey` is everything a pipeline compiles from, owned, so the compile can wait. `Target` is where a pipeline draws: colour format and sample count; a flip from 1 to 4 samples is a new key, and back again is the old one, already compiled. The test at the end proves that a `Lazy` is made once and shared.

```rust
--8<-- "lessons/006/src/engine/pipelines/mod.rs:006-cache"
```

## Plug it in

`lessons/006/src/engine/mod.rs` · type this, the line tagged `register:pipelines`

```rust
--8<-- "lessons/006/src/engine/mod.rs"
```

`lessons/006/src/engine/gpu/buffers.rs` · type this, the two lines tagged `register:pipelines`

The context owns the cache, so every lane reaches it through the `ctx` it already has.

```rust
--8<-- "lessons/006/src/engine/gpu/buffers.rs:002-ctx"
```

`lessons/006/src/engine/gpu/mod.rs` · type this, the lines tagged `register:pipelines`

The import, and the `Target` every lane will be built for: the canvas format at 1 sample.

```rust
--8<-- "lessons/006/src/engine/gpu/mod.rs:006-use"
```

```rust
--8<-- "lessons/006/src/engine/gpu/mod.rs:006-target"
```

## Checkpoint

Run `cargo xtest`. One test runs, `lazy_objects_are_made_once_on_first_use`, and passes: the closure ran once, both clones saw 7, and a second `Lazy` is not equal to the first. `trunk serve` shows the same grey canvas; the cache is empty until the next lesson describes a pipeline.

## Recap

A pipeline is compiled once per description and shared through `Lazy`. The cache lives in the context, keyed by `PipelineKey` and `Target`. Next we write the description itself and the compile behind it.

Next: [007 · Describe and build a pipeline](007-build.md)
