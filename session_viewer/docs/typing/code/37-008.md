# Viewer review · 2026-09-13

Scope: maintained viewer source and course, with focused checks of rendering ownership, input routing, editing, scene replacement and tutorial replay. Kernel repositories were not changed by this review.

## Changes

- Geometry commands use existing upload lanes and kernel transactions. Placed controls convert world targets back to local source coordinates; previews commit once or restore on cancellation.
- Nested tree/graph controls share selection and visibility actions. Index construction is revision-cached and bounded; oversized documents report the limit without exposing partial groups.
- The gumball uses one fixed cylindrical/conical mesh, unlit colors and a bounded antialiasing tile. Ctrl bypasses edit handles for source edge/face picking. Its composite pipeline is reused when only scene MSAA changes.
- egui 0.34.3 matches the archive framework. Explicit Light theme preserves white backgrounds and black text. Deferred actions avoid overlapping UI-state borrows. Inspection-only hit boxes avoid production JSON work; platform output moves without cloning.
- Owned attachments and replaced buffers are explicitly destroyed. The first WebGPU error survives subsequent invalid-command-buffer errors.
- Five independent lessons and seven integrated checkpoints show exact edits, compile commands, visible answers, expected results and next steps. Real command-window screenshots and a matching return triangle are included. The section-plane proposal is explicitly separate from the supported implementation.

## Validation

| Check | Result |
|---|---|
| Maintained WASM release build and clippy with warnings denied | Passed |
| Native library tests | 152 passed; 13 GPU tests separately passed |
| Committed kernel `06807badd20a0a5386501f95112992eb34203c93` | WASM check, native library tests and native all-targets/examples passed in an isolated copy |
| Independent and integrated lesson replay | All 17 compile checkpoints passed; final native tests passed for each sequence |
| Final integrated source comparison | Exact match for 114 runtime/build files |
| Frozen course replay audit and step/locator checks | Passed |
| Built course | 82 pages; 460 exact downloads; local links and code lexers passed |
| Diagrams | 213 current; three new extension diagrams passed overflow, collision and contrast checks |
| Browser loading | Malformed replacement preserves the last scene; newer route wins; six replacement/resource cycles passed |
| Browser lifecycle | Focus, pointer cancellation, hidden canvas and unchanged-framebuffer DPR passed |
| Browser source interaction | Seven geometry families at DPR 1 and 2; identity, object/edge/face/control picks, repeated F10 and tolerance passed |
| Browser editing | Five configurations; commands, nested/graph hide/select, four gumball gestures, control commit/cancel and tile release passed; see `docs/extensions/rounds.json` |
| Tutorial navigation | Home and nested-page triangles: focus, hover and return destination passed |

Browser checks used headed Chrome with the working NVIDIA Vulkan configuration recorded in `docs/extensions/screenshots.json`. Tests close their own browser contexts. Two outline assertions were corrected to preserve reference-mask coverage and the renderer's existing equal selected/unselected border width; rendering behavior was not changed to satisfy them. The interaction test now waits for input processing and fits each specimen so the fixed-size gumball cannot cover an entire short line during color checks.

## Limits and follow-up

The architecture keeps source, display rows and GPU ownership separate; no new general renderer framework was needed. The gumball retains 371,616 buffer bytes and at most 36 MiB of temporary textures. Deselect releases its tile. Panel limits are 200,000 nodes and 1,000,000 row references, with 128 visible entries per page.

These are named-resource bounds, not total-browser memory bounds. Undo snapshots, browser/driver overhead, private egui/glyphon capacity and WebAssembly's memory high-water mark remain separate. The application has one-page lifetime, not a repeated mount/unmount API. Large flag-write batching and undo limits require separate measurements and behavior decisions.

Trim/extend use normalized line/NURBS parameters; explode supports polylines. Group transforms, mesh/surface control writes, reparenting and cutting-object trims are not implemented. Geometry commits reject streamed scenes that the rebuild path cannot preserve.

The requested monorepo push script ran without arguments and refused the dirty sibling repositories before pushing anything. Viewer changes are committed and pushed separately; unrelated working changes and submodule pointers stay outside that commit. CI results are reported after the push.
