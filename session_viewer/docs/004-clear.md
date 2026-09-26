# 004 · Clear to a colour

The GPU does not run calls one by one. We record commands into an encoder, submit the recording to the queue, and the canvas shows the result when the GPU is done. This lesson records the smallest frame there is: open a pass, clear the colour and depth textures, close it, submit, present. Then it wires that frame into the event loop, so the browser asks for it and gets it.

## Two more files

`lessons/004/src/engine/gpu/mod.rs` · type this, the lines tagged `register:present` and `register:render`

```rust
--8<-- "lessons/004/src/engine/gpu/mod.rs:004-modules"
```

```rust
--8<-- "lessons/004/src/engine/gpu/mod.rs:004-use"
```

## Present one frame

`lessons/004/src/engine/gpu/present.rs` · type this, new file

`FrameInput` is what the caller gives each frame; today the clear colour and the time. `present` asks the surface for this frame's canvas texture; a lost texture reconfigures the surface and returns `None`, and the caller tries again next frame. It makes a view of the texture, records the frame into an encoder, submits the recording, and presents the texture. The two clock reads measure how long the recording took, in milliseconds, which is the number `?perf` shows later.

```rust
--8<-- "lessons/004/src/engine/gpu/present.rs:004-present"
```

## Record the frame

`lessons/004/src/engine/gpu/render.rs` · type this, new file

`Frame` is what every pass of one frame shares: the canvas view and the clear colour. `encode_frame` records the passes in order and returns the draw count; today there is one pass and no draw. `face_passes` opens it with `begin_faces` and closes it when `pass` goes out of scope. The tagged lines are where every later lesson joins: the ink pass, the pick pass, the panels.

```rust
--8<-- "lessons/004/src/engine/gpu/render.rs:004-encode"
```

## The state draws

`lessons/004/src/state.rs` · type this, the line tagged `register:render`, then append

`render` is the frame loop's heart. It clears `needs_frame`, builds the `FrameInput` with the background colour, and presents when the picture is dirty. A dropped frame stays dirty, so the next redraw tries again. `gpu_failed` reads the error slot from lesson 002: the first GPU error stops drawing and shows the error box, so a broken shader is one message, not a black screen.

```rust
--8<-- "lessons/004/src/state.rs:004-use-frame"
```

```rust
--8<-- "lessons/004/src/state.rs:004-render"
```

## The browser asks for a frame

`lessons/004/src/lib.rs` · type this, the two lines tagged `register:redraw` in `window_event`, then append `redraw`

`RedrawRequested` is the browser saying "draw now"; `Resized` says the canvas changed, and returning true marks the picture dirty. `redraw` fetches the state and renders. The tagged lines in it are for the panels of a later part.

```rust
--8<-- "lessons/004/src/lib.rs:004-events"
```

```rust
--8<-- "lessons/004/src/lib.rs:004-redraw"
```

## Checkpoint

Run `trunk serve`. The canvas is now painted by the GPU: the background colour 0.9 grey, which the sRGB canvas shows as `#f3f3f3`, a shade lighter than the page's own `#f0f0f0`. To see the frame with your own eyes, change `CLEAR` in `state.rs` to `r: 1.0, g: 0.0, b: 0.0` and save: Trunk rebuilds and the canvas turns red. Put the three values back to 0.9 before you go on; the course never changes a line it has written.

## Recap

A frame is a recording: encoder, one pass that clears colour and depth, submit, present. `render` makes the recording only when the picture is dirty; the browser asks for it through `RedrawRequested`. Every later lesson draws inside the pass this one opened.

Next: [005 · Resize and device pixels](005-resize.md)
