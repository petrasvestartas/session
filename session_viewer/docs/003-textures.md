# 003 · The frame's textures

A frame is drawn into textures: images on the GPU. The canvas gives us one, the colour we see. A second one we make ourselves: the depth texture, one number per pixel that says how near the nearest thing drawn there was. When two triangles cover a pixel, depth decides which one you see. Both textures must match the canvas size, so they are remade on every resize.

This lesson makes `Targets`, the pair of textures, and the two helpers that create any texture. Nothing on screen changes yet.

## A texture and its view

`lessons/003/src/engine/gpu/targets.rs` · type this, new file

`TextureSpec` is the four facts a texture needs: size, pixel format, samples per pixel and what the GPU may do with it. `texture` makes one. `Attachment` is a texture with its view, the handle a render pass draws through; `Deref` lets an `Attachment` stand wherever a view is wanted, and `Drop` frees the memory the moment it goes away.

`Targets` holds the depth texture and the sample count. `begin_faces` opens the first render pass of a frame: the canvas texture as colour, cleared to the background, and the depth texture cleared to 0. Depth is reverse-Z: 1 is the eye and 0 is infinitely far, which keeps precision where the geometry is. `None` for `clear` keeps what an earlier pass drew.

```rust
--8<-- "lessons/003/src/engine/gpu/targets.rs:003-targets"
```

## Register the file

`lessons/003/src/engine/gpu/mod.rs` · type this, the lines tagged `register:targets`

One line in the file list, one import, one field, two lines in `build` that make the textures without multisampling, and one line that stores them.

```rust
--8<-- "lessons/003/src/engine/gpu/mod.rs:003-module"
```

```rust
--8<-- "lessons/003/src/engine/gpu/mod.rs:003-use"
```

```rust
--8<-- "lessons/003/src/engine/gpu/mod.rs:003-field"
```

```rust
--8<-- "lessons/003/src/engine/gpu/mod.rs:003-build"
```

```rust
--8<-- "lessons/003/src/engine/gpu/mod.rs:003-init"
```

## Checkpoint

Run `cargo check`. It passes, and `trunk serve` shows the same page with the same three console lines. The depth texture exists on the GPU; the next lesson opens a pass over it and clears both textures for real.

## Recap

A frame draws into two textures, colour and depth, both the size of the canvas. `Attachment` pairs a texture with its view and frees it on drop. `begin_faces` opens the pass that clears them. Next: the frame loop that calls it.

Next: [004 · Clear to a colour](004-clear.md)
