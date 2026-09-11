# The map

Learn this one picture and the rest of the course has an address.

![The whole viewer as one map: the top row is how documents come in, the bottom row is how a frame is drawn, and a pick answer travels back up.](illustrations/map.svg)

Every step that touches a file opens with this map, one box filled pink: where the code on the page lives. (A step that explains rather than edits has no map.)

A compressed copy stays **pinned at the top of the page**, following you from step to step and from code block to code block.

Three fills, and nothing else on the map ever moves:

- **pink** — the code on this page lives here
- **light slate** — you have already built it
- **near-black** — still ahead of you


## The two paths

Every file serves one of two journeys.

**Documents come in along the top row.** A file arrives over the network, decodes into kernel documents holding exact f64 geometry, and the walk turns those into rows — plain arrays with no wgpu types. It runs when a scene loads, then stops.

**A frame is drawn along the bottom row.** The browser asks → the shell routes → input may have moved the camera → state decides what shows → the GPU core writes the per-frame uniforms and hands each lane its turn → lanes record draw calls → shaders make pixels. Sixty times a second, touching no document.

They meet once: the walk's rows are uploaded, and the frame path reads only those. That single junction is why a large model still draws quickly — drawing never re-reads the documents.

**One arrow goes backwards.** A pick: the lanes draw object ids into a small offscreen window, the answer is read back, and `Scene` turns a row number into the document object it came from. Two flows go further than the arrow shows — picking a streamed cloud point or a sheet entity continues left into the network, because the identity was never on this machine; and the tile pool reads its own size report back a frame later, never reaching the scene. Everything else points downward.

## The zones, one line each

| Zone | What lives there | Why it is separate |
|---|---|---|
| **Page** | `index.html`, `Trunk.toml`, `Cargo.toml` | The browser's side of the contract: what gets loaded before any Rust runs. |
| **Network** | `fetch`, `manifest`, `validate`, `decode`, `stream`, `live`, `route`, `loader`, `cloud_query`, `sheet_query` | The only code that touches bytes you did not create. All validation happens here. |
| **Kernel** | `session_rust` | Shared with the C++ and Python kernels: exact f64 geometry and identity, plus the one shared display type, `RenderVertex`. It links wgpu for that and for the GPU buffers a `Mesh` caches, and decides nothing about how the viewer draws. |
| **Scene + walk** | `app/scene.rs`, `app/scene_text.rs`, `app/selection.rs`, `app/walk/*`, `engine/text.rs` | Turns one document into rows and names what can be selected. No producer in `walk/` knows about files, selection or the camera; `Scene` holds the documents and their placements and hands finished rows to the GPU. |
| **Shell** | `lib.rs`, `app/mod.rs`, `app/feedback.rs`, `app/inspection*`, `app/knobs.rs`, `selftest*`, `text_quality.rs`, `engine/performance.rs` | The window, the event loop, the one place a redraw is asked for, and the measurements that observe a frame without changing it. |
| **Input** | `app/input.rs`, `app/touch.rs` | Gestures become intentions. It never touches a buffer or names a wgpu type: scene changes go through `State`. It does flip the view knobs (`view.lit`, `show_outlines`) and read the device scale directly — per-frame view state, not scene state. |
| **State** | `state.rs`, `camera.rs`, `math.rs` | Camera and selection transitions, and the demand for the next frame. |
| **GPU core** | `device`, `present`, `render`, `frame`, `objects`, `targets`, `buffers`, `instance`, `upload`, `view`, `pipelines/` | One device, one set of uniforms, one list of passes, one row type, one growable buffer. Everything a lane needs but no lane should own. |
| **Lanes** | `arena`, `faces`, `segments`, `glyphs`, `cloud`, `splat`, `lod`, `text*`, `surface_outline`, `backdrop`, `triangle_tiles`, `pick` | One per kind of thing drawn, so a new primitive is an addition rather than an edit. Most lanes ignore each other; the exceptions are deliberate and few — outline text borrows the arena's buffers rather than copying the geometry, the splat lane reads the cloud tables and the LOD walk it draws from, and the ink shader reads the projected triangles and the tile pool the arena's tile lane fills. |
| **Shaders** | `src/shaders/*.wgsl` | The code that runs on the GPU, compiled against the scene contract in `scene.wgsl`. |
| **Pixels** | the canvas | Where it all ends up. |

## How to use it while you type

**Lanes** lit: you are adding a way to draw something. Ask what rows it reads and which shader it feeds.

**GPU core** lit: you are changing something *every* lane sees — a uniform field, a pass, a pipeline rule. These need the three declarations to agree; expect a validation error if you miss one.

**Scene + walk** lit: you are deciding what a document *becomes*. Nothing here can see the camera; wanting to is the design telling you the work belongs one row down.

Two zones lit: the change crosses a boundary — a file written across several steps, with the build red in between. The **Does it compile yet?** line at the top of each lesson names them.

## Next

[00 · Empty project to a WASM message](00-environment.md): the first box on the map, and the only one you can build without a GPU.
