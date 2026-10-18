# Local validation and remaining course work

## October 9 source reconciliation

The viewer and Wood checkouts are updated to the revisions in [latest source audit](latest-commit-audit.md). Canonical layer/Element behavior, final browser commands and dependency fingerprints changed. Prior whole-course records below are historical; they are not fresh signatures for the revised course. Targeted new verification is recorded in the next-session handoff and release notes. Regenerated pages explicitly label older images as pending revalidation. The preserved historical-reference parity check remains separate.

Current source-matching endpoint evidence: **53u: 315 native tests; 92o: 784; 96i–96p: 804 each; 96q: 808; 96r and 97: 809 each**. Each also passed WebAssembly, its optimized bundle and an actual native GPU frame. All thirteen stored signatures pass. Ten authoring tests and the complete 990-step structure check pass, accounting for 301 current viewer source files. The remaining earlier build records require revalidation against the new fingerprints; they were not re-signed.

Logs on this authoring machine: `/tmp/viewer-sync-final-canonical.log`, `/tmp/viewer-sync-stored.log`, `/tmp/viewer-sync-authoring-final.log`, `/tmp/viewer-sync-generate.log` and `/tmp/viewer-sync-structure.log`. Native repeats exposed by falling back from older Chrome captures were declared at 03a-paint, 03a-panel and 03b-memory against their actual preceding identical frames; image bytes and assertions were preserved.

The final maintained-runtime Chrome contract now passes at **DPR 1 and 2** in visible Chrome 153.0.8010.36. The final endpoint is one of **3/990** fresh browser records; the two handwritten interaction endpoints are described below. The initial timeout was an incorrect expected object count: the maintained loader adds a nameplate to the four original solids, producing five selectable objects. The corrected check explicitly verifies the nameplate and four original geometry values. The screenshot reload also needed URI-encoded spaces (`%20`), because the maintained query reader does not decode form-style `+` separators. Pointer delivery and Fit now finish before the next interaction; single selection and measurement replies use their actual inspection/command contracts. No production source or pixel threshold changed.

The full browser run covers Element visibility defaults/toggles, command text ownership, selection/hide/show/Fit, Element and bounding-box Delete/Undo/Redo, identity restoration, world extents, panel resizing, orbit/wheel navigation, rejection of Close outside drawing, and zero retained scene payload after empty-manifest reload. Both screenshots pass the existing blue-geometry and black-boundary pixel assertions. Record and image hashes: `screenshots/journey/browser.json`; successful log: `/tmp/viewer-startup-corrected9.log`; startup probe: `/tmp/viewer-startup-probe.json`. Browser failures now retain console/network messages and inspected state under `target/course-checks/*-browser-dpr*-failure.json`.

A separate browser-act window at 5053×1321 reported a Vulkan external-image allocation error. That observation is distinct from the controlled 900×760 acceptance, which passed with the recorded Chrome flags; it does not establish a hardware cause for the earlier timeout. Earlier failed capture logs remain preserved at `/tmp/viewer-sync-browser{,-diagnostic,-final}.log`. Lenovo measurements, broader visual cases and the other 987 browser checkpoints remain pending.

## October 11 imported-feature bridge and command completion

Ten new canonical checkpoints, [96i](96i-separate-own-and-hosted-element-features.md) through [96r](96r-preserve-exact-command-completions.md), bring the course to **990 checkpoints** while preserving every previous ID. Each new checkpoint passed WebAssembly, its native test suite, an optimized browser bundle and an independently rendered native frame. The preparation frames deliberately retain the original comparison scene; the connected imported specimen is shown in the actual Chrome captures at 96q and 96r. Their implementation typing estimates remain below one hour; supplied regression fixtures are marked as copied tests.

The handwritten bridge groups original graph features by full interaction type and name/source, retains unmatched features, projects read-only pieces and keeps nested child Elements editable. Three native archive cycles preserve decoded original Element fields and opaque interactions, save no generated polylines, and regenerate exactly six feature paths. Tests also cover large-coordinate placement, combined parent/child transforms, independent visibility, Delete/Undo with original identity, and atomic rejection of malformed outlines. A regression caught and fixed lazy GUID allocation during display grouping: unnamed original feature GUIDs remain empty.

Visible Chrome 153.0.8010.36 at 900×760 CSS pixels and DPR 1 passes the 96q and 96r contracts: actual file selection, import Undo/Redo, rendered geometry, full hierarchy labels, independent Element controls, child panel selection/Delete/Undo, and two downloaded archive reopen cycles without accumulated pieces. The 96r contract also types exact View Isometric and Select commands. Native egui text events check the same vocabulary through character-by-character input and Enter. Exact matches and word-boundary continuations now outrank misleading longer completions.

The final maintained-runtime contract was recaptured at DPR 1/2 after the cumulative source changes. It now additionally rejects Close with only two draft points, closes a three-point 300/400/500 mm triangle, measures its exact 1,200 mm perimeter, and checks one-step Undo/Redo with original identity. The existing out-of-drawing rejection and empty-manifest release checks still pass.

Logs: `/tmp/viewer-bridge-96q.log`, `/tmp/viewer-bridge-verify-oct11.log`, `/tmp/viewer-bridge-browser-final.log` (96q/96r), `/tmp/viewer-bridge-close-browser.log` (final corrected DPR 1/2), `/tmp/viewer-bridge-authoring.log`, `/tmp/viewer-bridge-stored.log`, `/tmp/viewer-bridge-generate.log`, `/tmp/viewer-bridge-structure.log`, and `/tmp/viewer-bridge-docs.log`. The successful records and image hashes are in `target/course-checks/results.json` and `screenshots/journey/browser.json`. Earlier failed iteration logs remain diagnostic history.

This is bounded bridge evidence, not complete handwritten parity. Element-instance feature display, copied relationship ownership, projected-piece picking/highlighting, projection-specific presentation persistence, and the camera/rendering/input/runtime differences remain explicit in [the bridge audit](rendering-differences.md). The imported-feature browser exercise still uses the teaching Close-to-clear behavior; the final maintained runtime uses Close to finish drawing. Historical destination inventories and the existing production patch remain unchanged, and the learner workspace was not created.

## Historical whole-course run before this update

All subjects 00–97 are authored, with 980 cumulative checkpoints. On October 9, 2026, the unmodified `course -- verify --stored` checker accepted every checkpoint against its current displayed source. No missing or stale record remained.

| Local evidence | Historical records |
| --- | ---: |
| WebAssembly builds | 980 |
| Optimized browser bundles | 980 |
| Native test suites, where the lesson supplies tests | 949 |
| Actual native GPU frames | 979 |

The canvas-only first checkpoint has no native frame requirement. These counts come from successful commands in `session_viewer/target/course-checks/results.json`; source signatures include reconstructed files, lockfiles, assets, kernel inputs, frame fixtures and the unmodified checker. Native images describe the teaching renderer except where the lesson explicitly uses the maintained reference.

The full-course refresh also checks authoring, cumulative reconstruction, typing bounds, generated pages, structure, diagram reproduction and strict documentation links. Reproduce those checks from the Session root:

```sh
npm --prefix session_tests run course -- verify --stored
npm --prefix session_tests run course:test
npm --prefix session_tests run course -- generate
node --import ./session_tests/scripts/typescript.mjs --input-type=module -e 'import {structure} from "./session_tests/scripts/course/verify.ts"; structure();'
npm --prefix session_tests run course -- diagrams --check
DOCS_STRICT_LINKS=1 npm --prefix session_tests run build
```

Repeated native images are declared against the preceding identical frame and independently rendered. They do not demonstrate a visible change where a lesson adds ownership, bindings or tests. All 980 generated pages have their referenced images.

The command above invokes the unmodified current-course structure checker. The combined `course -- structure` command additionally checks the historical 43-section reference against today's production file list. Its cumulative historical listings and snapshots reconstruct successfully, but that final parity assertion fails because newer viewer source files are absent from the older reference. That separate historical gap remains recorded; it does not invalidate the current course's complete source inventory. Neither checker has been relaxed.

At that earlier source revision, fresh Chrome acceptance was limited to eight opening checkpoints; 972 browser checkpoints remained pending. At that revision, the final browser contract had only been syntaxchecked. Local software-Vulkan frames do not establish Lenovo Chrome or Firefox rotation/zoom performance. Keep that remaining acceptance visible in the [roadmap](roadmap.md) and [final handoff](final-handoff.md).

The [upstream audit](latest-commit-audit.md) checks the current viewer commit and complete linked Wood Rust subtree. The [maintained scene evidence](current-native-scenes.md) separately records original floor and dome renders and their exact local provenance.
