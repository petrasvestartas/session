# Cumulative viewer course

Build the production viewer through nine executable subjects. Each subject adds its final implementation; the retained-source check permits only declared registration extensions in earlier module indices.

## Subjects

Follow this order; each subject requires the preceding one.

| Subject | Checkpoint |
| --- | --- |
| 1. [Open a GPU frame](completion-frame.md) | `frame` |
| 2. [Paint shapes and text](completion-painting.md) | `painting` |
| 3. [Lay out and animate mathematics](completion-mathematics.md) | `mathematics` |
| 4. [Enter commands](completion-dock.md) | `dock` |
| 5. [Project and fit a scene](completion-camera.md) | `camera` |
| 6. [Place and update object rows](completion-rows.md) | `rows` |
| 7. [Render kernel geometry](completion-geometry.md) | `geometry` |
| 8. [Decode a document](completion-decoding.md) | `decoding` |
| 9. [Run the application](completion-application.md) | `application` |

The application subject includes document/UI behavior, commands, input, loading, text, streaming, real-scene stages and debugging. These chapters share the application checkpoint; they are not extra rewrite checkpoints. The rendering subject includes visibility and resource ownership.

## Run a checkpoint

From `session_tests`, run `npm run course -- list` to see the active executable subjects. Assemble a fresh source project with `npm run course -- checkpoint mathematics ../session_viewer/target/learn-mathematics`. Use another subject ID to choose that cumulative endpoint. Existing output directories are refused; your work is not overwritten.

Build and serve that project using the shared cache and workspace guards described in [Maintaining](../maintaining.md). Follow its lesson for the exact URL, visible result, experiment and checks. For the final application, build its `production.html` entry. Earlier implementations and example entries remain present.

## Acceptance

The route is **incomplete**. Eight of its nine executable subjects are accepted: frame, painting, mathematics, dock, camera, rows, geometry and decoding. Application remains pending. [Evidence](release.md) and each lesson distinguish native tests, wasm checks, browser behavior and physical-device acceptance. A successful source comparison or site build does not establish teaching or rendering quality.

The source teaching ledger currently has 359 paragraph witnesses across 351 Rust/WGSL source files and eight browser/build support files, with none missing. A witness binds an actual explanation to reviewed source; it is not automatic teaching acceptance. `course -- teaching` passes its source and paragraph bindings; executable and human teaching acceptance remain separate. No production source has been deliberately excluded.

Remaining work for whole-course completion is the unresolved [application GPU instancing comparison](release/completion-application-gpu-diagnostic-windows-20261116.json) and physical-device verification; [production reconciliation](release/completion-production-reconciliation-20261118.json) and [stage reconciliation](release/completion-stage-reconciliation-windows-20261118.json) record the reviewed deltas against the planning baseline. Metadata round-trip byte preservation has a recorded failure: current Save normalizes component JSON spacing/key order. No production behavior change or passing byte-parity claim is implied.

## Historical reconstruction

[Historical contents](contents.md), [checkpoint progress](roadmap.md) and the [former route](../archived/cumulative-route-20261115.md) preserve the older sequence. `npm run course -- historical-list` lists its identifiers. Its `course.json`, source fixtures, stored hashes and reconstruction checks remain intact.

The active manifest is `active-course.json`; its IDs select actual cumulative assemblies, not grouped historical edits. Historical `structure`, `ownership` and `parity` checks keep their original meaning and may still fail. They are not relabeled as passes when the new assembly passes retention or endpoint checks.

The support-file scan includes the production Cargo manifest and lock, shader build script, Trunk configuration, browser entry, diagnostics script, text-quality page and local scene manifest. The application checkpoint calls the production entry `production.html`; it retains the original entry bytes. [Non-code bundled assets](completion-asset-exclusions.json) list 17 deliberate exclusions: binary fonts and fixture images/data, their provenance/licences and the asset-directory placeholder. They remain available to the runnable checkpoints.
