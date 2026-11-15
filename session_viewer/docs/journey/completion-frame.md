# Open a GPU frame

Draw a white browser canvas with the production device setup and a small executable frame experiment.

This is a candidate replacement subject. Its partial project passes wasm checks and the four-candidate bundle passes browser interaction checks; full subject acceptance remains pending.

## Prerequisites and ownership

Start with Rust installed, the `wasm32-unknown-unknown` target, Trunk and a browser supporting WebGPU. Use the repository's recorded Cargo lock and shared build cache. No earlier viewer lesson is required.

This subject owns `src/engine/gpu/device.rs`, `src/app/launch.rs`, and the frame experiment under `examples/course`. The example canvas adapter remains a test fixture throughout the course. The production application's scene messages belong to the later application subject; the fixture does not stand in for that implementation.

Rust groups source into modules. A `mod` declaration includes another module; `use` brings a name into scope. `pub` exposes an item to callers. The module indices declare dependencies without storing their implementations. Later subjects add declarations and entries to `registration.rs`; they retain the launcher, device setup and frame experiment.

## Build

Assemble the candidate, then read and type the named source sections below.

From `session_tests`, choose an output directory that does not exist:

```sh
node --import ./scripts/typescript.mjs scripts/course.ts completion-checkpoint frame ../session_viewer/target/learn-frame
```

The command supplies the recorded dependency lock and links the shared fonts and licences. It refuses to overwrite an existing project. The complete dependency manifest is supplied once; subsequent subjects use the same dependency versions.

### Browser and event ownership

The HTML canvas is the destination for the GPU surface. CSS sets its displayed size; the configured surface determines the texture's pixel dimensions. Initialize those dimensions from the canvas's CSS size multiplied by the browser's pixel ratio. The first winit resize notification can arrive after device creation, so it cannot supply the initial dimensions reliably.

```html
--8<-- "session_viewer/examples/course/index.html:course-page"
```

Read [the launcher](../../src/app/launch.rs). Its type parameter `Message` names events sent back to the loop, and `App: ApplicationHandler<Message>` requires the application to implement winit's event callbacks. `FnOnce` allows the factory to consume captured initialization data. The event proxy is created before the application, so asynchronous device setup can deliver its result without borrowing the event loop across an `await`.

In the fixture, `Arc<Window>` keeps the window alive while both the application and surface need it. `Box<dyn Drawing>` owns a drawing implementation whose concrete type is selected at startup. A shared reference `&DeviceSetup` lets drawing use the handles without taking their ownership. `Result<T, String>` represents either a usable value or a contextual startup error.

```rust
--8<-- "session_viewer/examples/course/canvas.rs:course-canvas"
```

`resumed` takes the proxy once, creates the browser window and starts the asynchronous adapter request. `user_event` installs the completed window, GPU and drawing together. Until then, window events have no active drawing to access. A resize clamps texture dimensions to the device limit and skips surface configuration when that pixel size is unchanged. A redraw acquires a surface texture, records drawing, submits it and presents it. Animation requests another redraw; a static frame settles until input or resizing invalidates it.

### Device, queue and failure

Read [device setup](../../src/engine/gpu/device.rs). An instance exposes backends; a surface connects a window to one of them; an adapter describes available hardware and limits. The device creates resources, and its queue accepts commands. These are separate responsibilities: creating an encoder does not execute its commands.

`Options` supplies power preference, optional timing and a diagnostic callback. This keeps device selection independent of application settings and scene types. A function pointer can report diagnostics without requiring the device module to know about the command dock. `Default` supplies an inert reporter for callers that do not need one.

`open` checks adapter capabilities, bounds requested storage-buffer limits, requests the device, and configures a nonzero surface size. Its `Option<Surface>` also permits native headless use. `?` returns a failed request to the caller instead of constructing a partially initialized GPU. Fatal callbacks retain the first error in shared synchronized state; the application can stop submitting work and report that original failure.

### Record, submit, present

A render pass borrows the encoder while it is active. Binding the pass to `_pass` keeps that borrow until the helper returns. Dropping the pass ends recording; `encoder.finish()` produces a command buffer. `queue.submit` schedules it, and the canvas adapter presents the acquired texture afterwards.

```rust
--8<-- "session_viewer/examples/course/frame.rs:course-frame"
```

`LoadOp::Clear` initializes every pixel. `StoreOp::Store` retains the result for presentation. There is no vertex buffer or shader pipeline yet: clearing an attachment is enough to produce this subject's frame.

### Startup registration

The unchanged entry function reads `?tutorial=`, chooses exactly one registered function and calls it. Without a name it uses the last entry. An unknown name returns an error rather than choosing another application. In this first partial project, the supplied registration list contains only `frame`; later checkpoints add entries to that list.

```rust
--8<-- "session_viewer/examples/course/entry.rs:course-entry"
```

## Run and observe

From the assembled directory, run `trunk serve --port 8780` through the workspace storage guard and build slot. Keep `CARGO_TARGET_DIR` pointed to the shared course cache. Open `http://127.0.0.1:8780/?tutorial=frame`.

Expect a completely white canvas with no triangle or text. Resize to a narrow upright window: the canvas must continue filling the viewport, with no stale strip from the old surface size. A white screenshot alone is insufficient evidence; the browser check must also establish successful GPU startup and absence of the fixture's `data-course-error` attribute.

## Predict and try

Predict the result of changing the clear colour to black before rebuilding. Every pixel should become black, including after a resize. Restore white. This is a learner experiment, not a later subject's required rewrite.

## Question and answer

Why does creating an encoder not change the canvas?

An encoder records commands on the CPU. The queue must receive its finished command buffer, and the acquired surface texture must be presented, before the browser can display that work.

## Validation

The partial frame project passes its locked wasm Cargo check. Three focused tooling tests check unchanged earlier implementations, exact retained production source and omission of later application entries. The linked candidate passes GPU startup, physical canvas-size, white-frame and resize checks at desktop and upright dimensions.

The [current interaction receipt](release/completion-partials-windows-20261115.json) records Chrome 157 and the Intel Gen-12LP adapter. Upright viewport emulation is not a physical-phone test, and these checks do not accept the remaining cumulative course.
