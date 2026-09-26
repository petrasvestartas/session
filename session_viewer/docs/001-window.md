# 001 · A window on the canvas

The browser sends events: a click, a key, a resize, a request to draw. Something has to receive them in one place and in order. That something is winit, the windowing library, and its event loop: one loop that owns the canvas and hands us every event as a value.

This lesson writes the loop and the shell around it: the `App` that runs it, the `State` it drives, and the message that carries the state into the loop once it exists. Nothing is drawn yet. The loop is the one piece every later lesson hooks into, so it comes first.

## The crate's modules

`lessons/001/src/lib.rs` · type this, append after the entry function

A module is one file. `app` will hold everything that talks to the page: loading, messages, input. `state` holds what the viewer keeps between frames. `pub use` makes `State` reachable as `crate::State` from every file.

```rust
--8<-- "lessons/001/src/lib.rs:001-modules"
```

## One message: the state is ready

`lessons/001/src/lib.rs` · type this, append

The GPU opens asynchronously, so the state cannot be built inside the loop. It is built elsewhere and posted into the loop as a message. `Msg` is the list of messages the loop accepts; today it has one. `Box` puts the state on the heap so the message stays small.

```rust
--8<-- "lessons/001/src/lib.rs:001-messages"
```

## The app that runs the loop

`lessons/001/src/lib.rs` · type this, append

`App` holds the state, once it arrives, and a proxy: a handle that any code can use to post a message into the loop. `run` creates the loop and hands the app to the browser's own loop with `spawn_app`; the browser calls back, our function returns at once. `adopt` takes the ready state and asks for a first frame. `request_if_needed` asks for a frame only when something changed, so a still picture costs nothing.

```rust
--8<-- "lessons/001/src/lib.rs:001-app"
```

## The three events

`lessons/001/src/lib.rs` · type this, append

`ApplicationHandler` is the trait winit calls. `resumed` runs once: it turns the page's canvas into the winit window and starts `boot`, the async task that opens the GPU. `user_event` receives our own messages; `Ready` becomes the state. `window_event` receives the browser's events; the match returns true when the picture must be drawn again, and `touch` records that. Every later lesson adds one line to one of these matches.

```rust
--8<-- "lessons/001/src/lib.rs:001-events"
```

## A pointer event

`lessons/001/src/lib.rs` · type this, append

Any event the match above does not name is a pointer event. `changed |= ...` is the idiom this course uses everywhere: a later lesson adds one line that ORs its own answer into the result, and nothing written before it changes.

```rust
--8<-- "lessons/001/src/lib.rs:001-mouse"
```

## The canvas element

`lessons/001/src/lib.rs` · type this, append

`web_sys` is the browser API in Rust. `dyn_into` turns the generic element with id `canvas` into an `HtmlCanvasElement`, or `None` if the page has no such element.

```rust
--8<-- "lessons/001/src/lib.rs:001-canvas"
```

## Start

`lessons/001/src/lib.rs` · type this, append

`start` runs the app, and shows the error box if the loop cannot be created. The first `if` steps aside on one other page of the site, the text-quality page, which runs its own code; you meet it in the text lessons.

```rust
--8<-- "lessons/001/src/lib.rs:001-start"
```

`lessons/001/src/lib.rs` · type this, the line tagged `register:shell` in `run_web`

One line between the panic hook and `Ok(())` is the whole difference between a module that loads and a viewer that runs.

```rust
--8<-- "lessons/001/src/lib.rs:000-entry"
```

## What the viewer keeps

`lessons/001/src/state.rs` · type this, new file

`State` is everything the viewer keeps between frames. Today: the window and three flags. `needs_frame` says a redraw is wanted; `dirty` says the picture changed; the two times serve the frame loop in lesson 004. `new` is `async` because it will open the GPU, which takes time. `touch` is what every change calls.

```rust
--8<-- "lessons/001/src/state.rs:001-state"
```

## The app module

`lessons/001/src/app/mod.rs` · type this, new file

Two lines, each tagged `register:`: a tag marks a line a later lesson adds to a list, and this file is the list of everything that talks to the page.

```rust
--8<-- "lessons/001/src/app/mod.rs"
```

## Talking to the page

`lessons/001/src/app/feedback.rs` · type this, new file

`status` writes one line into the `viewer-status` corner of the page; `error` fills the `viewer-error` box and shows it, reload button included. Both also log, so the console tells the same story.

```rust
--8<-- "lessons/001/src/app/feedback.rs:001-feedback"
```

## Boot

`lessons/001/src/app/loader.rs` · type this, new file

`boot` is the async task `resumed` started. It keeps the proxy in a thread-local slot, so any loader code can post a message later, builds the state, and posts `Ready`. If the state cannot be built, the error box says why.

```rust
--8<-- "lessons/001/src/app/loader.rs:001-boot"
```

## Checkpoint

Run `trunk serve` and open the page. It looks the same, and the console is still empty: the loop runs, and nothing asks it to draw. To see that winit owns the canvas now, open the developer tools, Elements, and find `<canvas id="canvas">`: it carries `width` and `height` attributes with the size of your screen in pixels. Lesson 000's page had none. winit measured the canvas and sized its drawing surface.

## Recap

winit owns the canvas and hands us every event in one loop. `App` runs the loop; `boot` builds the `State` off the loop and posts it as a message; `touch` marks a change and `request_if_needed` asks for a frame only then. Next we open the GPU and log what we found.

Next: [002 · Open the GPU](002-gpu.md)
