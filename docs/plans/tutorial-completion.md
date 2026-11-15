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

The unresolved dependency is the application shell: `lib.rs` directly handles concrete scene messages and `State` owns the complete GPU, scene, selection and input features. A command registry alone cannot remove this dependency. The next implementation must establish a stable shell/host boundary and demonstrate an early runnable host plus the production host without changing observable production behavior. Do not advertise the proposed subject ordering as executable until its prerequisites compile.

The placement candidate remains source-only. Its ten existing-file changes must be resolved in the original owners rather than accepted as another finished subject with known rewrites.

The first implemented boundary removes `device.rs`'s dependency on later view settings and application diagnostics. The device accepts explicit power/timing options and a diagnostic callback; the production GPU constructor supplies the existing settings and reporter. Adapter selection, first-error retention, device-loss redraw and the native shared adapter remain owned by device setup. This is an intentional unaccepted production delta until its checks and integration review pass; the destination pin must continue reporting it.

The GPU egui painter now takes a device and queue directly. Its former `GpuCtx` argument pulled in the scene pipeline cache even though painting used only those two handles. Production passes the same handles, with texture upload, tessellation, submission and delayed texture freeing in the same order. Both device setup and this painter compile independently for native and wasm without application or scene shims. These focused checks establish dependency boundaries; full viewer and browser regression checks remain required.

The browser event-loop launcher accepts winit's existing `ApplicationHandler` and a factory receiving its event proxy. The production `App` uses this launcher with the same initialization. This removes concrete scene types from event-loop creation without adding another handler trait; it does not yet decouple the production event handling itself.

## Acceptance and evidence

Keep one source-owner ledger and one evidence index, pointing to existing receipts instead of copying them. A source hash, source include, screenshot or successful site build alone cannot mark a subject accepted. Acceptance requires the complete lesson contract, runnable cumulative source, relevant tests, reviewed browser behavior and satisfied ownership boundaries.

Continue using the existing structural, ownership and endpoint-parity checks unchanged. A destination-pin check supplements them; it never refreshes their accepted hashes. Retain failed evidence with its actual scope.

Use one shared cache per toolchain and one heavy command at a time. Coordinate any precise cache retirement with its owners. The proposed 94-file WASM retirement is unexecuted and is not required for source-only architecture work.

## Continuation

Read [the continuation record](tutorial-continuation.md), run its next command, then implement the current work item. Re-audit unrelated history only if a recorded input changed. Update the record after an accepted milestone or before a limit interrupts work.
