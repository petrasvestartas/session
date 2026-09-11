# The map

Learn this one picture and the rest of the course has an address.

![The whole viewer as one map: the top row is how documents come in, the bottom row is how a frame is drawn, and a pick answer travels back up.](illustrations/map.svg)

Every step of every lesson opens with this same map, with one box filled pink: that is where the code on the page lives. A solid box is something you have already built. A dashed box is still ahead of you. Nothing else on the map ever moves, so after a lesson or two you stop reading it and start *seeing* it.

## The two paths

There are only two journeys in this viewer, and every file serves one of them.

**Documents come in along the top row.** A file arrives over the network, is decoded into kernel documents that hold exact f64 geometry, and the walk turns those documents into rows — plain arrays with no wgpu types in them. That path runs when a scene loads, and then stops.

**A frame is drawn along the bottom row.** The browser asks for a frame, the shell routes the event, input may have moved the camera, state decides what the frame shows, the GPU core writes the per-frame uniforms and hands each lane its turn, the lanes record draw calls, and the shaders turn those into pixels. That path runs sixty times a second and touches no document at all.

The two meet in one place: the rows the walk produced are uploaded once, and from then on the frame path reads them. That single junction is why the viewer can hold a large model and still draw quickly — drawing never re-reads the documents.

**One arrow goes backwards.** A pick is the only upward flow: the lanes draw object ids into a small offscreen window, the answer is read back, and `Scene` turns a row number into the document object it came from. Everything else in the viewer points downward.

## The zones, one line each

| Zone | What lives there | Why it is separate |
|---|---|---|
| **Page** | `index.html`, `Trunk.toml`, `Cargo.toml` | The browser's side of the contract: what gets loaded before any Rust runs. |
| **Network** | `fetch`, `manifest`, `validate`, `decode`, `stream`, `live`, `loader` | The only code that touches bytes you did not create. All validation happens here. |
| **Kernel** | `session_rust` | Shared with the C++ and Python kernels. Exact f64 geometry and identity; it knows nothing about drawing. |
| **Scene + walk** | `app/scene.rs`, `app/walk/*` | Turns one document into rows. A producer per geometry kind, and none of them knows about files, selection or the camera. |
| **Shell** | `lib.rs`, `app/mod.rs` | The window, the event loop, and the one place a redraw is asked for. |
| **Input** | `app/input.rs`, `app/touch.rs` | Gestures become intentions. It never touches a buffer; it asks `State`. |
| **State** | `state.rs`, `camera.rs` | Camera and selection transitions, and the demand for the next frame. |
| **GPU core** | `device`, `present`, `render`, `frame`, `objects`, `targets`, `pipelines` | One device, one set of uniforms, one list of passes. Everything a lane needs but no lane should own. |
| **Lanes** | `arena`, `segments`, `glyphs`, `cloud`, `splat`, `text*`, `surface_outline`, `triangle_tiles`, `pick` | One per kind of thing drawn. Lanes never see each other; that is what makes a new primitive an addition rather than an edit. |
| **Shaders** | `src/shaders/*.wgsl` | The code that runs on the GPU, compiled against the scene contract in `scene.wgsl`. |
| **Pixels** | the canvas | Where it all ends up. |

## How to use it while you type

When a step lights **Lanes**, you are adding a way to draw something, and the question to ask is: what rows does it read, and which shader does it feed?

When a step lights **GPU core**, you are changing something *every* lane sees — a uniform field, a pass, a pipeline rule. Those are the changes that need the three declarations to agree, so expect a validation error if you miss one.

When a step lights **Scene + walk**, you are deciding what a document *becomes*. Nothing here can see the camera, and if you find yourself wanting to, the design is telling you the work belongs one row down.

When a step lights two zones at once, that is worth noticing: it means the change crosses a boundary, and those are the steps where a file is written across several steps and the build goes red in between. The **Does it compile yet?** line at the top of each lesson tells you exactly which ones.

## Next

[00 · Empty project to a WASM message](00-environment.md): the first box on the map, and the only one you can build without a GPU.
