# Load without blocking the viewer

Show a document or a mathematical deck while keeping startup, fetching and decoding responsive and reporting failures with context.

## Prerequisites and ownership

Complete decoding, the application lifecycle and document ownership. This application chapter teaches `app/route.rs`, `manifest.rs`, `fetch.rs`, `loader.rs` and `state/hydrate.rs`, together with the early-fetch part of `index.html`. The decoder and its browser-yield helper remain the implementations introduced earlier. Stage parsing and painting remain in the mathematics subject; this chapter supplies their asynchronous loading prerequisites.

## Build

Connect route selection, early requests and loader messages to the completed decoder and application host.

### Start independent work together

The startup page begins requests before wasm is ready. It records each request's promise, abort controller and start time in `window.__viewerEarly`. For ordinary scenes it fetches the manifest and probes the first discoverable file. For a stage route it fetches four fonts, the index and discoverable chapter files instead. The HTML scan is an optimization, not the authoritative YAML parser: Rust still validates and loads anything the early scan misses.

```rust
--8<-- "session_viewer/src/app/fetch.rs:loading-early"
```

`early` removes the matching entry when taking ownership. A whole-file request and the initial 8192-byte probe have different keys. A later range or conditional request cannot accidentally consume that probe. Reusing the promise avoids a second network request; reusing its controller and timestamp preserves cancellation and timing ownership.

`loader::boot` starts manifest work while `State::new` awaits GPU setup. Once setup succeeds, it sizes and prewarms the GPU and posts `Msg::Ready`. Subsequent completed documents enter through messages. A stage route opens the stage and returns; it does not also start ordinary scene loading.

### Routes and manifests describe different things

`route.rs` interprets the page location and query. A scene name must remain inside the allowed path form: empty names, parent traversal, leading slashes and URL-like names are refused. `data=off` uses this origin; HTTPS and local development URLs can supply a different data base. `join` then resolves each file against that base.

`Manifest` describes the scene's files, placements, text, view and grid. A full `xform` takes precedence over the simpler `at` translation. This metadata is separate from each decoded kernel document: the same geometry bytes can be displayed under a different placement without rewriting them. The obsolete `display_only` manifest field no longer determines editability; streamed shells and released geometry have explicit runtime states.

The loader probes a protobuf head before deciding whether to read it whole or stream a recognized cloud or sheet. It shares available point and segment budgets across files and overlaps the next file's probe with the current load. Background sheet slices wait for initial manifest loads so one large sheet does not prevent other files reaching the screen.

### Bound memory and distinguish failures

`fetch::get_buffer` leaves the body in a JavaScript `Uint8Array` until a consumer needs wasm-owned bytes. `get` explicitly copies it into a Rust vector. This distinction matters for large files: retaining two full copies can cost more memory than decoding itself.

The body reader grows its buffer as chunks arrive and resets the stall deadline after each chunk. A slow transfer that continues delivering bytes differs from a stalled request. Whole-file bodies are capped at 512 MiB; progress appears for sufficiently large transfers. Compressed payloads use the browser's decompression stream before decoding.

HTTP responses and network failures are separate. The low-level GET returns the HTTP status, allowing conditional and range callers to interpret it. Higher-level whole-file and range helpers enforce their own response requirements. A server ignoring a range is not proof that the returned bytes represent the requested slice. Contextual status messages identify manifest, document or stage failures rather than treating empty bytes as success.

### Discard work from an obsolete scene

```rust
--8<-- "session_viewer/src/app/loader.rs:loading-generations"
```

The loader keeps separate generations for streams and replacement requests. Clearing the scene advances the stream generation and resets its budgets. Each reload advances the load generation; a delayed result checks whether its generation is still current before adoption. Asynchronous completion order must not decide which scene wins.

The `Loading` guard increments the active-load count on construction and decrements it in `Drop`. Early returns therefore release that count too. This is Rust resource ownership applied to scheduling: forgetting a decrement would leave background sheet work waiting forever.

### Restore editable geometry only when needed

```rust
--8<-- "session_viewer/src/state/hydrate.rs:loading-hydration"
```

A display can retain rows after releasing its kernel geometry. `locked_reason` distinguishes a permanently read-only streamed shell from a released document whose source can be fetched again. Requested operations enter a small `Resume` queue. The returned `Hydrated` value carries a document index and token; an old token is ignored after replacement or another release.

On success, the host restores the kernel document and attempts waiting operations. Controls resume only if the relevant row is still selected; Save and a complete rewalk wait until all needed documents are resident. On failure, it clears pending resumptions and shows the document name with the error. This prevents an old fetch from executing an edit against a different current selection.

## Run and observe

Build and serve the cumulative `application` checkpoint's `production.html`. Open `/?stage=decks/equilibrium/index.yaml&data=off`. The stage loads fonts and chapter content, then displays its first slide. Advance and step backward without reloading the page. Open the ordinary comparison-scene route separately: it should show the document and dock rather than a stage.

In browser developer tools, inspect the Network panel during a reload. Font/index requests can begin before wasm startup completes. Block one stage font URL and reload: the visible error names that font instead of presenting an empty slide as ready. Remove the block before continuing.

## Predict and try

Predict whether enabling a slow network should cause the first completed request to choose the active scene. Start loading one scene, request another, and let the first finish last. The newer load generation remains authoritative; obsolete results must not replace it. Check the visible scene and request timings together, because a completed request alone does not prove adoption.

## Question and answer

Why does taking an early request delete its entry?

A response body and abort controller need one consumer responsible for completion and cancellation. Taking the entry transfers that ownership to Rust. A later caller must either use an appropriate cache path or start its own request, rather than consuming the same body twice.

## Validation

The cumulative application compiles for wasm and passes its startup, document and deck browser checks. Six injected stage failures cover a font, malformed chapter and picture at both desktop and upright sizes; each reports its contextual filename. These checks do not establish delayed replacement races, range streaming, hydration recovery or physical-phone memory behavior. Those remain separate acceptance cases.
