# The map

Learn this one picture and the rest of the course has an address.

![The whole viewer as one map: the top row is how documents come in, the bottom row is how a frame is drawn, and a pick answer travels back up.](illustrations/map.svg)

Every step that touches a file opens with this same map, with one box filled pink: that is where the code on the page lives. (A step that only explains something has no file, so it has no map.)

A compressed copy of it stays **pinned at the top of the page** while you read, and follows you from step to step, so the answer to "where am I" is never more than a glance away — you never have to scroll back to find it. A solid box is something you have already built. A dashed box is still ahead of you. Nothing else on the map ever moves, so after a lesson or two you stop reading it and start *seeing* it.

## The two paths

There are only two journeys in this viewer, and every file serves one of them.

**Documents come in along the top row.** A file arrives over the network, is decoded into kernel documents that hold exact f64 geometry, and the walk turns those documents into rows — plain arrays with no wgpu types in them. That path runs when a scene loads, and then stops.

**A frame is drawn along the bottom row.** The browser asks for a frame, the shell routes the event, input may have moved the camera, state decides what the frame shows, the GPU core writes the per-frame uniforms and hands each lane its turn, the lanes record draw calls, and the shaders turn those into pixels. That path runs sixty times a second and touches no document at all.

The two meet in one place: the rows the walk produced are uploaded once, and from then on the frame path reads them. That single junction is why the viewer can hold a large model and still draw quickly — drawing never re-reads the documents.

**One arrow goes backwards.** A pick is the upward flow you will feel: the lanes draw object ids into a small offscreen window, the answer is read back, and `Scene` turns a row number into the document object it came from. Two things extend it further than the arrow can show — picking a streamed cloud point or a sheet entity continues left into the network, because the identity was never on this machine; and the tile pool reads its own size report back a frame later, which is a GPU → CPU flow that never reaches the scene. Everything else points downward.

## The zones, one line each

| Zone | What lives there | Why it is separate |
|---|---|---|
| **Page** | `index.html`, `Trunk.toml`, `Cargo.toml` | The browser's side of the contract: what gets loaded before any Rust runs. |
| **Network** | `fetch`, `manifest`, `validate`, `decode`, `stream`, `live`, `loader` | The only code that touches bytes you did not create. All validation happens here. |
| **Kernel** | `session_rust` | Shared with the C++ and Python kernels: exact f64 geometry and identity, and the one shared display type, `RenderVertex`. It does link wgpu for that, but it decides nothing about how the viewer draws. |
| **Scene + walk** | `app/scene.rs`, `app/scene_text.rs`, `app/selection.rs`, `app/walk/*`, `engine/text.rs` | Turns one document into rows, and names what can be selected. No producer in `walk/` knows about files, selection or the camera; `Scene` above them holds the documents and their placements, and hands finished rows to the GPU. |
| **Shell** | `lib.rs`, `app/mod.rs`, `app/feedback.rs`, `app/inspection*`, `engine/performance.rs` | The window, the event loop, the one place a redraw is asked for, and the measurements that observe a frame without changing it. |
| **Input** | `app/input.rs`, `app/touch.rs` | Gestures become intentions. It never touches a buffer or names a wgpu type: anything that changes the scene goes through `State`. It does flip the view knobs directly (`view.lit`, `show_outlines`) and read the device scale, because those are per-frame view state rather than scene state. |
| **State** | `state.rs`, `camera.rs` | Camera and selection transitions, and the demand for the next frame. |
| **GPU core** | `device`, `present`, `render`, `frame`, `objects`, `targets`, `buffers`, `instance`, `upload`, `view`, `pipelines/` | One device, one set of uniforms, one list of passes, one row type, one growable buffer. Everything a lane needs but no lane should own. |
| **Lanes** | `arena`, `segments`, `glyphs`, `cloud`, `splat`, `lod`, `text*`, `surface_outline`, `backdrop`, `triangle_tiles`, `pick` | One per kind of thing drawn, which is what makes a new primitive an addition rather than an edit. Most lanes ignore each other completely; the exceptions are deliberate and few — outline text borrows the arena's buffers rather than copying the geometry, and the splat lane reads the cloud tables and the LOD walk it draws from. |
| **Shaders** | `src/shaders/*.wgsl` | The code that runs on the GPU, compiled against the scene contract in `scene.wgsl`. |
| **Pixels** | the canvas | Where it all ends up. |

## How to use it while you type

When a step lights **Lanes**, you are adding a way to draw something, and the question to ask is: what rows does it read, and which shader does it feed?

When a step lights **GPU core**, you are changing something *every* lane sees — a uniform field, a pass, a pipeline rule. Those are the changes that need the three declarations to agree, so expect a validation error if you miss one.

When a step lights **Scene + walk**, you are deciding what a document *becomes*. Nothing here can see the camera, and if you find yourself wanting to, the design is telling you the work belongs one row down.

When a step lights two zones at once, that is worth noticing: it means the change crosses a boundary, and those are the steps where a file is written across several steps and the build goes red in between. The **Does it compile yet?** line at the top of each lesson tells you exactly which ones.

## Next

[00 · Empty project to a WASM message](00-environment.md): the first box on the map, and the only one you can build without a GPU.
