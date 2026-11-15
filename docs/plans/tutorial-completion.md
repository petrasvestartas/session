# Complete the cumulative viewer course

The destination for this batch is session commit `87f844e41d08ef104fc071d5bc18163ffe9a4353`. Preserve concurrent production work and reconcile its source and behavior in a separate final pass. The pinned destination is a comparison target, not accepted tutorial evidence.

## Execution order

1. Record the destination and explicit source owners. Reject missing owners, duplicate ownership and unnoticed destination changes. Planned ownership is not teaching coverage.
2. Resolve the dependency boundaries that prevent early subjects from retaining their implementations. Reuse production's `Mode<Host>`, panels, command registry and drawing lanes. Review the event loop, `State`, GPU construction and loading together before changing their architecture.
3. Implement the smallest coherent cumulative route. Start with Rust and a GPU frame, then egui painting, then the complete 2D mathematics subject. Introduce real-scene stage integration only after rows, camera, strokes, arrows, labels and loading. Retain historical fixtures and stable links.
4. Finish each subject's code and teaching together. Later subjects may register new components through reviewed interfaces; they may not replace finished implementations. Archive displaced active checkpoints only after their replacement runs and its links and fixtures are checked.
5. Validate focused changes first, batch native and wasm builds through the shared cache, and run full structural, ownership, parity, site and browser checks at integration boundaries. Publish coherent batches and watch every triggered workflow to completion.
6. Reconcile production changes since the pin, review every source file's actual teaching or deliberate exclusion, then run final acceptance. Software rendering, browser emulation and physical-device results remain distinct.

## Architecture decisions

`app/mode.rs` already provides the appropriate early mathematics interface. `Presentation` implements `Mode<Host>` without requiring `State`; the later spatial mode uses `State`. Keep the mathematics implementation and add the spatial adapter later.

`app/ui/mod.rs` has a panel registry, `app/command/verbs/mod.rs` has a command registry, and `engine/gpu/lane.rs` has drawing-lane registration. Use these concrete interfaces where they fit. A `register:` comment does not make an arbitrary change to a function an acceptable registration.

The concrete application owns scene messages and `State`, which contains GPU, scene, selection and input features. The shared winit launcher is now separate from that application. Early partial assemblies compile without the concrete application; the complete production application compiles against the same launcher. Browser regression acceptance remains required before treating this boundary as finished.

The placement candidate remains source-only. Its ten existing-file changes must be resolved in the original owners rather than accepted as another finished subject with known rewrites.

The first implemented boundary removes `device.rs`'s dependency on later view settings and application diagnostics. The device accepts explicit power/timing options and a diagnostic callback; the production GPU constructor supplies the existing settings and reporter. Adapter selection, first-error retention, device-loss redraw and the native shared adapter remain owned by device setup. This is an intentional unaccepted production delta until its checks and integration review pass; the destination pin must continue reporting it.

The GPU egui painter now takes a device and queue directly. Its former `GpuCtx` argument pulled in the scene pipeline cache even though painting used only those two handles. Production passes the same handles, with texture upload, tessellation, submission and delayed texture freeing in the same order. Both device setup and this painter compile independently for native and wasm without application or scene shims. These focused checks establish dependency boundaries; full viewer and browser regression checks remain required.

The browser event-loop launcher accepts winit's existing `ApplicationHandler` and a factory receiving its event proxy. The production `App` uses this launcher with the same initialization. This removes concrete scene types from event-loop creation without adding another handler trait; it does not yet decouple the production event handling itself.

The concrete viewer application lives in `app/runtime.rs`; the crate root registers modules and reexports the existing API. The spatial deck application similarly lives in `app/stage/application.rs`, leaving the stage module index available for additive registrations. The pure mathematics implementation remains unchanged. The camera owns its field-of-view constant, which the GPU uniform module reexports, so teaching the camera does not require the complete GPU pipeline. These moves pass full native and wasm typechecks; linked behavior remains unaccepted.

The concrete GPU assembly lives in `engine/gpu/context.rs`. Keeping it in `gpu/mod.rs` would require adding a large implementation to the early device/painter module index. The index now contains declarations and reexports only. Moving the type down one module retains former private fields and helpers at GPU-module visibility, and preserves the original engine-visible `target` method with `pub(in crate::engine)`. Method bodies and command ordering are unchanged. Full native and both default/tutorial wasm typechecks pass after the move.

The cumulative checkpoint assemblies live in `examples/course` behind the `tutorial` Cargo feature. Their startup list appends a frame, painting, mathematics and production-viewer entry; selecting the latest entry does not rewrite an earlier implementation. The examples use the production device, GPU painter and mathematics modules. Their small window adapter is a teaching/test fixture, not the production application. Default builds exclude this adapter and retain the production WebAssembly start function. Each partial source checkpoint must still compile independently; selecting modes in one final bundle alone cannot establish cumulative acceptance.

Before accepting first painting, its fixture must already support the next input-owning subject. Its painter accepts `RawInput` and an egui UI callback; the first painting and mathematics callers supply empty input, while the dock can supply translated events without changing the painter. The fixture factory receives the window alongside the device, allowing a later input owner to construct the existing egui-winit adapter. This keeps window events and widget input outside the finished GPU painter. These fixture APIs are retained through six runnable candidates. The evidence index records independent wasm checks, native tests and actual desktop/upright interactions; full-course acceptance remains pending.

## Acceptance and evidence

Keep one source-owner ledger and one evidence index, pointing to existing receipts instead of copying them. A source hash, source include, screenshot or successful site build alone cannot mark a subject accepted. Acceptance requires the complete lesson contract, runnable cumulative source, relevant tests, reviewed browser behavior and satisfied ownership boundaries.

Continue using the existing structural, ownership and endpoint-parity checks unchanged. A destination-pin check supplements them; it never refreshes their accepted hashes. Retain failed evidence with its actual scope.

Use one shared cache per toolchain and one heavy command at a time. The user authorized 24 GiB generated files; the workspace guard now allows 36 GiB total while retaining CPU, memory and FIFO constraints. Preserve protected artifacts and failed receipts. Current resource and runtime evidence is recorded in the continuation file.

## Continuation

Read [the continuation record](tutorial-continuation.md), run its next command, then implement the current work item. Re-audit unrelated history only if a recorded input changed. Update the record after an accepted milestone or before a limit interrupts work.

### Row storage dependency boundary

The row table was coupled to later ink textures, renderer Lane integration and Upload geometry through three files. Introducing those files whole would pull the complete scene renderer into the storage lesson. Keep the implementations intact in table/layout/geometry/core modules; register ink, Lane, Upload and renderer-validation adapters through module declarations and reexports in their owning later subjects. Diagnostics belongs to the engine, with the existing feedback path reexported for UI callers. This preserves public entry points while avoiding later rewrites of completed storage code. The row checkpoint explicitly labels its egui view as a CPU mirror; scene shaders and picking acceptance belong to their later owners.
