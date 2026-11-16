# Run the application

Load a document into the maintained viewer, navigate its geometry and issue commands through the dock.

This ninth cumulative checkpoint is under verification. Its source endpoint matches production; teaching and browser acceptance are separate requirements.

## Prerequisites and ownership

Keep the eight earlier subjects: frame, painting, mathematics, dock, camera, rows, scene rendering and decoding. This integration owns the application lifecycle in `app/runtime.rs`, the concrete host in `state.rs`, and their module registrations in `lib.rs` and `app/mod.rs`. The application chapters cover the document, interaction, loading and stage adapters that this host connects. Those chapters remain required; simply including their files does not teach them.

The completed renderer, decoder, dock and mathematical layout remain in place. The final checkpoint supplies their application adapters and test registrations. The existing fixture page remains runnable. A second entry, `production.html`, starts the actual application from the checkpoint's own source, without depending on a built viewer crate. Continue with [documents, hierarchy and history](completion-documents.md), [commands and editing](completion-commands.md), [loading](completion-loading.md) and [input and selection](completion-input.md), [scene labels](completion-text.md) and [streaming and live updates](completion-streaming.md), then [mathematics in the real scene](stage-scene.md) and [debugging and self-tests](completion-debugging.md).

## Build

Add the concrete application host and its declared adapters, retaining the implementations from the earlier subjects.

### One owner for changing state

`App` implements winit's `ApplicationHandler<Msg>`. It owns an optional `State`, input tracking and the panels. Optional state represents a real startup phase: the window can exist before asynchronous device creation finishes. `Msg::Ready` transfers the completed state into the event loop. Other messages are handled only after that state exists.

`State` owns the window handle, GPU, camera, loaded scene and selection. These objects need a common lifetime: a document upload changes both scene rows and GPU buffers, and a selection result changes both application state and its next image. `Arc<Window>` keeps the window alive while asynchronous setup and redraw callbacks retain handles to it; it does not make the scene a shared mutable object.

Loader futures send typed messages through `EventLoopProxy<Msg>`. They do not borrow the application across an await. A file message reaches `State::append`, which adds a document, uploads its rows, updates titles and layers, expands camera bounds and requests a frame. A fit message respects the camera pose recorded when loading began, so an arriving file does not undo navigation the user has already performed.

### Route an event once

For the normal viewer, panels receive a window event first. If they consume it, scene input does not process it again. This matters when typing a command: a letter intended for the dock must not also trigger a scene shortcut.

Pressed keyboard events require viewer focus. Normal scene input rejects key repeats; a stage mode receives its own key events and controls its navigation policy. Browser pointer cancellation clears the active gesture and wheel interaction so losing focus cannot leave a permanent drag.

The active mode is an optional boxed trait object. `Box<dyn Mode<State>>` owns a value whose concrete type is chosen at runtime. The host calls the trait's methods without knowing which mode implements them. `Loaded<State>` separates fetched and parsed resources from the clock-dependent act of opening a mode.

```rust
--8<-- "session_viewer/src/app/mode.rs:application-mode"
```

During a mode's pointer callback, the host temporarily takes the mode out of its option, passes mutable state to the callback, then puts the mode back. This avoids simultaneously borrowing a field and the whole containing state mutably. The callback still operates on the same GPU and camera used by the normal viewer.

### Draw only when work is ready

A redraw first rejects a hidden page or unavailable canvas size. It then checks GPU pacing. When a submission is still outstanding, the completion callback will wake the window; accumulating more submissions would increase latency without producing a newer camera state sooner.

Resize happens before drawing. `State::resize` limits target reconstruction to one operation per 100 milliseconds during a window drag. A deferred resize retains the request for a later frame. Logical dimensions and device-pixel dimensions remain distinct so handles and labels keep their CSS size.

Panels or the active mode lay out before `State::render`. The renderer can then use the canvas region left by the UI. The submission fence includes UI uploads and picking work as well as scene drawing; otherwise an apparently completed frame could leave work outside the pacing decision.

`dirty` means the image needs replacement. `needs_frame` means another redraw is needed. They are related but not interchangeable: pending asynchronous picking or display refinement can require a later frame even after the current image is presented. `touch` invalidates pending picks as well as marking the image dirty, because an answer computed for the old scene or camera can be stale.

`State::render` applies completed pick results before drawing, runs declared feature hooks, updates camera-relative coordinates and builds `FrameInput`. A still, fully refined scene asks for no continuous redraw. A dropped surface frame is retried. A terminal GPU failure stops submission rather than repeatedly using invalid resources.

### Build the browser entry

The materialized Cargo manifest compiles these sources directly. Its `build.rs` produces the shader inputs required by the shader macro; the build must not borrow another crate's `OUT_DIR`. Assets, native fixtures and built documentation use the documented shared directories rather than independent dependency caches.

`production.html` is an exact copy of the production `index.html` source in the materialized application checkpoint. The checkpoint keeps its teaching fixture as `index.html`, copied separately from `examples/course/index.html`; the two entry names do not imply two production implementations. Its Trunk rules package the protobuf fixtures, text resources, stage assets, decks, diagnostics and documentation. Build the strict documentation site before materializing this final checkpoint: packaging an absent site is an error. The page's early requests and the loader's ownership of those requests are covered in the loading chapter.

### Register state and frame responsibilities

`Features` owns optional interaction state across frames: drafts, active drags, pending splits, query generations, snapping and hierarchy state. Ordered function lists declare work before and after pick answers, handlers that may consume a pick before ordinary selection, and hooks that expand a clicked row into a group. A feature registers its state and hooks at these boundaries instead of rewriting the event loop. Order is behavior: a pending drag must receive its delayed answer before ordinary selection interprets it.

### Package the browser entry and its dependencies

Trunk.toml selects the optimized build and a relative public URL, then invokes the documentation builder before packaging. Its explicit watch list includes the sibling kernel and documentation inputs; an omitted directory would stop edits there from triggering a rebuild. The development server binds localhost and marks its mutable index no-store, so the HTML names the current hashed WebAssembly bundle. These server headers concern the development entry page, not a claim that every immutable asset must bypass caches.

The production index owns the canvas, status element, command-input integration and documentation link. Trunk's Rust link builds the viewer library, while copy rules place protobuf files, full text fonts, stage fonts, decks and the built documentation beside it. The early-fetch script runs before wasm; the diagnostics script is deferred until parsing completes. One named context-menu handler suppresses the browser menu over the canvas. The embedding-message handler accepts a reload request from the parent window or the same origin and forwards its scene value to the wasm binding, retaining the existing page and GPU. A missing WebGPU API displays an explicit HTTPS/localhost message. Opening the HTML as a filesystem file is not the supported startup path.

The supplied local manifest is an authored list of geometry-file URLs, display names, placements and per-item settings such as point size. Its files exercise different rendering lanes, but their presence is not guaranteed on another machine. Resolve each relative file against the manifest's route and replace missing local examples with recorded course fixtures. Changing a manifest placement moves the loaded instance; it does not edit the kernel file's coordinates. The manifest is input data whose parser and contextual errors belong to the loading chapter.

## Run and observe

Materialize `application` using the completion-checkpoint command in [Maintaining](../maintaining.md). Build its `production.html` entry with Trunk and serve the emitted `dist/index.html`. Use a recorded scene manifest whose files are available to the server; the repository's default local manifest is not a guarantee that private scene files exist on another machine.

The expected result is the normal viewer: geometry, document controls and command dock on the same canvas. Scroll over the geometry to change its scale. Enter `Fit` in the dock to frame the loaded scene, then `View Orthographic` to switch projection. The command history must show each command. Typing a command must not select all scene objects.

The retained teaching page and its earlier tutorial modes remain separate runnable observations of the same completed components. They are not substitutes for testing this production entry.

## Predict and try

Predict what happens if a file finishes loading after you orbit the camera. Navigate while loading a sufficiently large local fixture, then let loading finish. The saved load-time pose differs from the current pose, so automatic fitting preserves your navigation. Enter `Fit` explicitly when you want to frame the enlarged scene.

## Question and answer

Why send a loaded document as a message instead of letting the fetch future edit `State` directly?

The event loop serializes state changes with input and redraw handling. A future can wait for network or decoding work without retaining a mutable borrow of the application. Its completed document becomes one explicit event, after which the host updates scene data, GPU resources, panels and redraw state together.

## Validation

Five tooling tests verify earlier implementation retention, shader closure and the exact production source endpoint. The complete default native suite passes 609 tests with 60 explicit ignores; ignored GPU/device tests are not acceptance. The cumulative tutorial and default application compile for wasm; the native test binary links. Targeted native runs pass 68 scene, seven hierarchy and seven decoder tests, with 12 explicit ignores. Optimized Cargo and Trunk builds pass. Eight browser captures verify startup, zoom, Fit and projection commands at desktop and upright sizes; all match the reviewed production captures exactly. This is a scoped application smoke check, not full document editing, stage or physical-phone acceptance. The failed native verification attempt with a missing fixture junction remains recorded separately.
