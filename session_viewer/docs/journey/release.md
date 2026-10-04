# Course release: command-line checkpoints

**All 149 current checkpoints build and pass their scripted checks in visible Chrome 153.0.8010.36.** They cover lessons 01–32 plus placement, source-ownership, save/reopen, loading/recovery, GPU ownership and document-close follow-ups, and the short dock sequence from font setup through command handoff. Every interactive feature check types into the viewer’s actual egui command field. Canvas picking, dragging and wheel navigation retain their natural input paths. Keyboard feature shortcuts are absent.

This is the opening of the full viewer course. The [remaining lesson checklist](roadmap.md) and [destination contract](destination.md) still govern completion.

## Lesson clarity revision

Each current lesson has one focused exercise. All main explanations were reviewed; this batch shortens 49 further explanations after the earlier chapter-34 revision. Main introductions stay within 90 words, followed by typed changes and one result check. Experiments, implementation context and extended acceptance evidence remain expandable supporting notes. Every lesson links to the detailed to-do list.

The first GPU lesson is now two complete runnable steps: paint a white GPU frame (30–59 minutes of typing), then separate presentation with bounded sizing and error feedback (28–56 minutes). That first GPU split preserved all 120 then-published code endpoints. Consecutive white pictures are explicitly declared and independently captured; no artificial view change is used to suggest new behavior.

Five checkpoints still exceed one hour of typing and need meaningful splits. Short explanations alone do not make those long code listings finished lessons. The roadmap lists each one and its estimate.

## Command-panel typing split

The former 03a checkpoint required 81–162 minutes of typing. It is now three complete runnable steps: prepare the fonts and GPU painter (30–60 minutes), paint the Command label over the scene (25–49), then lay out the production field (30–60). Their main explanations are short; field input is explicitly deferred to the input lesson.

Four affected endpoints passed fresh WebAssembly, Trunk, native rendering/state checks and visible Chrome at that split. Chrome observes the actual egui shader/pipeline owner, checks Noto label ink and exact preservation of the scene above the white dock, and retains the production field checks. One repeated layout-setting call was removed from the panel/state introduction; 119 other endpoints remained byte-identical then.

## Direct instructions and command-state split

One hundred long or generic code instructions now name their exact change in at most 25 words. The main explanations still teach the Rust needed for the step; detailed checks remain expandable.

The dock is now taught through separate working steps: history, immediate typing, submission, editing, Help, inline completion, acceptance, key handling, suggestions, browsing, prepared output, control bounds, pointer ownership, wheel browsing, shared layout and application handoff. Each of the 17 new steps has 11–58 minutes of estimated typing, a focused result check, a white diagram and its own build and Chrome evidence. The production dock is split into the same functions; interactive lessons retain exact shared-source parity.

The two 03b model/helper checkpoints pass fresh WebAssembly, Trunk, native rendering/state checks and visible Chrome. Native checks cover retained text/history, borrowed hint priorities, actual egui inspection bounds and JSON, Unicode prefixes and stored caret ranges. Chrome verifies the changed model hint and exact scene preservation, then an unchanged full frame after the helpers. These two preparation stages have no keyboard event handling; immediate typing begins in 03ca.

## Scene, identity, picking and history splits

Four long checkpoints are now nine runnable steps: validated CPU mesh/scene owners (24–47 minutes), GPU scene drawing (23–46), stable object IDs (29–57), selection display (16–32), inverse view coordinates (10–19), triangle coverage/depth (28–56), actual canvas picking (14–27), shared history snapshots (28–55) and typed Undo/Redo (8–16). Each fits the one-hour typing limit.

The first model steps have focused native checks; their browser captures explicitly describe retained drawing until input is connected. Later steps connect the actual command or mouse path. Existing application, renderer and browser behavior is unchanged. The coordinate lesson adds a native round-trip test retained through history. A redundant dependency-feature reordering was removed from picking; only manifest ordering differs in 13–20, whose fresh native, WebAssembly and Chrome checks pass. No feature binding was added or removed.

## Perspective typing split

The perspective checkpoint is now three focused steps: drawing (20–40 minutes of typing), converting a screen point to a bounded ray (14–28), and actual ray picking (30–59). At the perspective split, all 144 existing endpoints remained byte-identical. The two preparation checkpoints deliberately pause the old flat-coordinate canvas query while retaining typed selection; the final step restores mouse selection through the matching ray.

Fresh native, WebAssembly, Trunk and Chrome checks pass. The new browser acceptance checks the visible overlap, zoom/reset aspect, then actual click selection, Delete, Undo and Redo after zooming.

## Action-route typing split

State ownership (14–27 minutes), Rust actions (27–53) and actual browser routing (29–57) now form three runnable steps. Native ownership, history, selection and picking checks pass; Chrome verifies typed actions, reset aspect, undo/redo, empty Delete, and selecting the box after panning.

The two model checkpoints independently capture the retained lighting route until the browser adapter is connected. Reset now preserves the current aspect when the editor is introduced, using the implementation and test previously taught in resize. That fixes the earlier reset regression; all 20 and later code endpoints remain unchanged.

## Resize typing split

The viewport model (19–37 minutes of typing) and the connected resize transaction (29–57) are separate runnable steps. The new CPU model has density, GPU-limit and hidden-canvas tests; the following step updates canvas, surface, depth and aspect together. All existing code endpoints are unchanged at this split.

Native/WebAssembly/Trunk/Chrome checks pass. Chrome acceptance changes wide/tall window sizes and emulates density two, verifies physical pixel dimensions and CSS geometry bounds, then restores the default capture view.

## What was checked

- Every displayed checkpoint reconstructs, builds for WebAssembly, and produces a Trunk browser bundle. Native state tests and GPU readbacks run where applicable.
- Source completion and automatic-command acceptance checkpoints finish with 111 native checks. Complete body pairs, current origins/epochs and file versions are validated before captured Move/Delete/Save replay. Visible Chrome holds actual source responses, changes selection and camera, checks original-target replay and one-step Undo, verifies a single Save download without consuming Undo, supersedes older pending edits, and rejects late replies after Cancel Reload, Undo and Close. Chrome also compares the actual warm and restored Save downloads with a source double that cannot survive an f32 round trip. Combined HTTP/length/body/read/network/type/version failures preserve drawing, placement, selection, camera, source residency and GPU counters; a second-source failure installs neither source and downloads nothing. Native missing-body/stale/duplicate-key checks and held-reply source URL release complete this bounded restoration acceptance.
- Browser-lifetime checkpoints retain the 111 inherited native checks and add actual WebAssembly/Chrome ownership proofs: two real EventTargets stop invoking a dropped callback and its captured Rc is released; the main viewer removes eighteen listeners on deferred page exit, cancels held source/file authority, releases source URLs and submits no further frames for old input or completions. These checks establish owned lifetime and cancellation, not instantaneous browser/driver memory reclamation. Document Close keeps the viewer available for Open. Cached-transition checks cancel unfinished gestures while retaining state, drawing and usable commands. A real same-tab away/Back run loaded a fresh document on this Chrome; persisted event checks establish the retained-runtime behavior separately, without claiming actual browser cache eligibility.
- The first-failure signal checkpoint passes 114 native tests, including competing callback writers, shared clone identity and independent replacement-device state. Chrome verifies inherited input/loading/lifetime behavior; actual GPU-loss callback integration, stopped submissions and recovery are still the following lessons.
- The GPU-stop checkpoint retains 114 native tests and connects both real error callbacks. Chrome calls actual GPUDevice.destroy(), checks visible first-loss feedback, eighteen listener removals, source abort/URL release, ignored late completion and zero post-loss queue writes/submissions, allocations or surface work after attempted input and resizing. A normal same-tab reload restarts the viewer; diagnostics, bounded automatic recovery and unsaved-edit recovery are not established by this checkpoint.
- Diagnostic lessons finish with 118 native tests: actual JSON schema/round trip, independent snapshots, 24-event rotation, capped Unicode text, finite timing and first-failure retention. Chrome verifies actual browser metadata without query/fragment, fresh dimensions and run identity, read-only snapshot counters, typed ready-report downloads, real GPU-loss and rejected-startup failure downloads, unchanged scene/history/camera/geometry GPU counters, no post-loss GPU work and delayed download URL cleanup. Save still downloads its original document format. Automatic report downloads are attempts; saved previous reports, detailed phases/resources/adapter telemetry and bounded recovery are still next. Reports retain no document/GPU handles and do not recover unsaved edits.
- Every checkpoint fills the browser content area from its first frame. Chrome checks the canvas position and dimensions, rejects visible headings or teaching paragraphs, and checks that the GPU image matches the initial window size.
- The course’s dock model, layout and theme match the production source. An automated check rejects HTML feature buttons and button-based lesson actions.
- Chrome types commands through actual keyboard events. Enter clears the field and records exactly one command. Completion and Escape are checked separately.
- Lesson 04 submits Background twice and compares every scene pixel above the folded dock. The second submission restores the original drawing exactly.
- Later checks cover command actions, picking, resizing, orbit and release, wheel without page scrolling, immediate command-field focus without losing the first character, and Open with whole-import Undo/Redo.
- Fit is checked for repeatability, recovery after panning, document history, and geometry margins in wide and tall windows.
- Placement lessons keep local mesh coordinates separate from model matrices; bounds, ray picking and GPU drawing share world placement. Typed Move checks world-axis composition, immutable shared geometry, Undo/Redo, finite arguments and zero-offset history. Chrome checks rendered interior picking and visible command errors; exact raster-edge ownership is deferred to GPU picking.
- The source-ownership lessons retain original kernel meshes beside their derived display arrays. The final endpoint passes 51 native tests, including double-coordinate preservation, adapter failure, imported allocation identity and ownership across Move/Undo/Redo. Chrome checks import, placed drawing, history and interior picking; visibility and locking policies are still later work.
- Save/reopen lessons preserve exact source doubles, mesh names and visibility/locking attributes, stored object identities and separate placements. The final checkpoint passes 57 native tests. Chrome downloads the real file, proves Save does not consume Undo, refuses an empty Save without a download, checks delayed URL cleanup, and reopens into a cleared scene with identical scene pixels. The bounded flat-mesh subset accepts 1–64 meshes and files up to 4 MiB; nested trees, tree colour policy, definitions and other geometry remain later course work.
- Loading/recovery lessons pass 64 native tests at the final endpoint. Chrome checks malformed/oversized files, rejected promises, held reads completed out of order, Cancel Open, newer-picker cancellation, asynchronous history ownership and atomic Open Replace with pixel-identical Undo/Redo. The cancel-event case dispatches the input event because Playwright intercepts the OS picker. The final check forces adapter unavailability, confirms visible startup feedback without feature controls, then reloads to a working viewer.
- GPU ownership lessons separate original vertex colours from object uniforms, retain shared source/display identity and weak cache entries, and update only changed settings ranges. Native GPU checks prove shared ownership, cache expiration and allocation/write counts. Chrome verifies selection and Move retain geometry, final object uniforms are retained, selection transitions write sixteen bytes per changed row, Move/Undo write sixty-four bytes per changed matrix, and deletion/Undo reuploads only the released row. Document Undo leaves selection cleared; the checks explicitly reselect before comparing the original selected pixels. These counters are renderer calls and bytes, not driver-memory or performance measurements.
- Document-close lessons finish with 71 native state tests plus GPU ownership/readback checks. They count shared CPU displays once across editor/history/GPU roots and count document GPU vertex/index/settings buffer sizes separately from cumulative uploads. Native weak observers verify source/document values disappear at editor close and GPU-held CPU displays disappear at empty synchronization. Chrome checks late successful/rejected reads, opaque-white empty drawing, zero live document ledgers, unchanged cumulative counters, cleared Undo/Redo, empty Save refusal and reopening in the same page. The ledgers deliberately exclude full kernel payload, renderer infrastructure, staging, allocator/driver overhead and pooling; source-only unloading/rehydration and production accounting categories remain future work.
- Source-residency preparation adds independent row metadata and a private editable-owner boundary. Each of the three new endpoints passes 73 native state tests plus WebAssembly/Trunk/native GPU checks. Native Weak checks prove metadata alone does not retain a closed kernel mesh; import/history checks preserve original source identity, names and visibility/locking values. Chrome verifies hidden metadata through Move/Undo and duplicate imports, with distinct saved identities and unchanged live allocation counts. These endpoints prepare source-only unloading; they do not yet unload, rehydrate or enforce the full visibility/locking policy.
- Source-unloading lessons finish with82native state checks plus actual GPU ownership/readback and Chrome checks. A geometry-free original header, import UUID, exact selected-file SHA-256 version and owned Blob URL support source residency independently of drawing. The reader allocates URLs only at accepted File adoption, rejects stale/cancelled reads, cleans failed imports and revokes each owned URL once after its final active/history owner drops. Native proofs protect modified Redo rows and independent duplicate imports, reject epoch exhaustion without mutation, and retain issuance across Close. Unloading drops eligible kernel owners across active/Undo/Redo while preserving metadata, displays, placements, camera, GPU allocations and exact scene pixels. Cold Move/Delete/Save currently return explicit reload-required errors; guarded hydration and automatic edit replay are still next. Source accounting remains scoped and does not measure backing File bytes, metadata heap, process RSS or driver reclamation. The later mutable HTTP publication flow needs its own source-version policy.
- Source-rehydration lessons finish with 87 native state checks and GPU ownership/readback proofs. Reload keys identify an imported Origin allocation and its current release epoch. Bounded immutable-file restoration prepares original kernel Sessions without creating new displays. Adoption validates all requested candidates and matching active/Undo/Redo rows before changing any editable owner; IDs, metadata/display Rcs, placements, camera and history remain. Checks retain exact source doubles through Save and Undo/Redo, keep GPU allocations/counters unchanged, reject a bad second version without partially restoring the first, and ignore duplicate/closed/previous-epoch keys before decoding. All six Chrome checkpoints verify the existing command-only unload behavior and retained drawing, not browser hydration: recorded-URL fetch, cancellation and automatic Move/Delete/Save replay remain the next lessons. The overview now counts lesson IDs with multiple suffix letters consistently with the full roadmap.
- Browser reload lessons finish with 90 native state checks and real Chrome Reload Sources/Cancel Reload checks. A checked ticket owner retains release keys only while current; abandoned work holds URL strings and an empty shared flight. AbortController cancellation pairs with ticket checks, and accepted results use a scoped Rust delivery slot. Fetch validates status/length and enforces 4 MiB per source while streaming, with reader cancellation/lock release. Real Blob reloads restore original sources with unchanged pixels, metadata, origins and GPU allocations/counters; Save and placement Undo/Redo work afterward. A typed native example validates actual original/browser-saved vertices, topology, flags, names and saved GUIDs. Held successes/failures, newer requests overtaking older ones, intervening camera commands, Cancel Reload, Close, Undo and replacement-picker cancellation are checked. Current HTTP, header/body size, stream and immutable-version failures leave the display cold and unchanged, then a real retry restores it. Source URLs revoke after Close even while an abandoned task remains. Automatic Move/Delete/Save-triggered reload and captured-target replay remain next; more combined timing, multi-origin failures, missing-body/current-fetch-rejection and invalid-length coverage remain on the roadmap.
- The captured-edit checkpoint introduces a value-only Intent with the original ObjectId and Move arguments. Its 93 native tests include captured Move/Delete targets after selection changes, finite/no-op handling and source-owner release on Close. Chrome checks retained drawing through the existing explicit reload; automatic edit interception and replay are still future work.
- Source-scope selection resolves captured Move/Delete ObjectIds independently of later selection. Save gathers each active cold origin once and excludes history-only sources. The final endpoint passes 96 native tests; Chrome retains the existing explicit reload path and normal Move. Browser interception/replay remains future work.
- Explicit-target Move composes with the row’s current model and keeps later selection/camera. Native tests prove one Undo/Redo transaction and preservation of Redo for no-op/refused moves; normal browser Move uses the same method and its pixel round trip passes. Browser ticket ownership and automatic replay remain future work.
- Lesson 25 checks the perspective/orthographic pixel round trip and selects a visible imported beam in each projection. Its 39 Rust tests also check camera, fitting, picking and document invariants.

The scene comparison excludes the command strip. When a key opens completion over the drawing, the test dismisses that overlay before checking camera pixels. Full-page screenshots still show the real interface.

## Screenshot evidence

Each lesson includes its own Chrome capture from the reconstructed browser bundle. The capture file records source and bundle fingerprints, common and optional checkpoint checker hashes, including their local helper dependencies, Chrome version, timestamp, viewport conditions and scene hash in `screenshots/journey/browser.json`. Screenshots are included only while their source and checker fingerprints match.

The initial browser viewport is 900 × 760 CSS pixels at display density 1. The evidence records that viewport, the canvas bounds and the final screenshot viewport. Resize and Fit checks also change the window size. Linux WebGPU uses the flags documented in `open-chrome.sh`. This verifies those scripted cases on this machine; it does not establish complete browser or hardware coverage.

The full-window white canvas starts in lesson 01; GPU sizing starts in lesson 02. The first two screenshots intentionally look alike: lesson 02 replaces the CSS-only background with a verified GPU clear. Each is captured from its own bundle. Lesson 10 gives the depth image matching dimensions before the first draw. Lesson 20 adds resizing after startup and display-density handling. Earlier checkpoints should be reloaded after changing the window size.

The early dock stages have different purposes: 03a draws the styling, 03b adds the model, 03ca connects immediate typing, and the following small steps add submission, completion, pointer input and application handoff. Their captions state when input becomes usable. Native GPU images are separate evidence and are never labelled browser screenshots.

## Fixed inputs and remaining scope

The captured Delete checkpoint passes 100 native tests. It deletes the original identity in one history transaction, preserves later selection and camera, clears selection only for the removed target, and preserves Redo when the target is cold/missing or no row is selected. Chrome checks ordinary Delete/Undo/Redo through that same helper; native tests exercise the delayed identity. Automatic asynchronous replay remains pending.

The captured reply checkpoint passes 102 native tests. Move/Delete return a scene change; Save returns original-source snapshot bytes without history, selection or camera changes. Exact saved doubles and Move Undo after Save are checked natively; cold Save preserves Redo. Chrome checks the existing normal download and Move Undo paths. The next lessons connect replies to current asynchronous owners; this checkpoint does not yet perform automatic browser replay.

The intent-owner checkpoint passes 104 native tests. A generic pending value owns keys and intent together; stale replies preserve newer work, current replies consume the pair once and cancellation releases metadata without retaining the kernel source. Existing explicit reload callers use a unit payload and retain their browser behavior. Intent-bearing browser delivery and context revocation remain future work; the browser bridge is a separate planned checkpoint to keep typing below one hour.

The course pins wgpu 29.0.4 and egui/egui-wgpu 0.34.3. Locks are introduced for the initial project, egui drawing, dock inspection dependencies, the Rust geometry kernel, protobuf import and explicit UUID generation. The dependency command preserves the previous lock and installs binary fonts; all implementation code is displayed for typing.

Early commands such as Background, Pan and Example are teaching vocabulary. The component and its styling are the real viewer dock. Later command chapters build the complete production vocabulary, argument handling, clipboard, composition and touch integration.

The original production destination remains fingerprinted separately. Browser diagnostics, device-loss guards and the shared command dock are newer than that reference, so the full source-parity audit still reports differences. The course has not yet reached production feature parity. Old reference tutorials remain until their complete replacement is verified.

The production viewer separately passed a Chrome device-loss check: stop GPU work, attempt a diagnostic download, and recover the report on reload. The reported Firefox QueueId driver crash has not been reproduced or proven fixed. A web application cannot guarantee a dump when the browser process itself crashes.

## Saved diagnostic report admission and recency

Four further checkpoints prepare browser storage. They pass 120, 122, 123 and 125 native tests respectively, plus WebAssembly, Trunk, native GPU frames and headed Chrome. Typed validation bounds metadata and observations and requires consistent failure/outcome state. JSON decoding limits input to one MiB before parsing and rejects unsupported top-level fields. Recency uses the first fatal timestamp for failed runs and a separate heartbeat policy for interrupted running tabs; invalid chronology, active other tabs and healthy/closed runs are excluded. Policy evaluation does not mutate a report.

The native tests use a deterministic timestamp parser. Actual browser Date.parse, storage adoption and previous-report notices are not connected at these checkpoints. Chrome retains the actual current-report downloads, GPU-loss disposal, startup failure, restart, source precision and command/history checks. The four typing estimates are 30–60, 27–53, 25–49 and 13–26 minutes. Storage, lifecycle telemetry and bounded GPU recovery follow.

## Browser storage and previous-report retrieval

Three further checkpoints retain the 125 inherited native state tests and pass WebAssembly, Trunk, native GPU frames and actual headed Chrome. The browser-only store uses stable sessionStorage tab identity and a separate key for each run. Reads examine at most 256 keys and 32 owned candidates through the bounded decoder and real Date.parse policy. Successful writes retain the current run and two newest valid older reports, remove malformed owned values and preserve unrelated keys. Larger namespaces refuse writing; quota failures preserve existing evidence and removal failures return false without claiming rollback.

The final endpoint selects previous metadata before writing its own running report, persists ready/fatal observations independently of GPU disposal, shows failed/interrupted notices in the actual dock and accepts Diagnostic Report Previous. Chrome destroys a real device, reads the actual stored failure, reloads the same test page and downloads unchanged previous JSON from a healthy run. Stable tab/new key, three-report bounds, scene/history invariants, download URL release, quiet healthy/active-other-tab cases and denied-storage current downloads pass. There are no new HTML feature controls. Heartbeat/lifecycle/error observations and full load/adapter/resource telemetry remain upcoming; diagnostics do not restore unsaved edits.

The three typing estimates are 21–41, 18–36 and 25–49 minutes. An initial live test incorrectly assumed the inspector exported status; the final lesson exposes the existing drawn status for acceptance, then passes all checks. One recheck also failed an inherited command-clear assertion; an unchanged confirmation run and the final live run passed. Its cause was not established, and the assertion was not weakened.

## Compatibility-safe retention

The next checkpoint corrects a compatibility gap in the initial writer. Strict admission excluded newer schemas, so using admitted reports as the retention list could delete newer-format evidence. Retention now reads only a bounded positive-version/date header, ranks the two newest older values and leaves their original stored bytes unchanged. Unsupported or typed-invalid values can occupy a retained slot but remain ineligible for notices and typed downloads. Syntax errors, excessive bytes and unusable headers can still be pruned; a failed older-value read aborts before any mutation.

127 native tests, WebAssembly, Trunk, native GPU rendering and real Chrome pass. Chrome preserves exact whitespace-bearing version2 JSON with unknown telemetry, confirms strict selection refuses it, and verifies read denial leaves all values untouched. The inherited real loss/reload/previous-download and scene/history checks also pass. Typing26–51 minutes.

## Heartbeat and callback ownership

Two more checkpoints pass 129 native tests each, WebAssembly, Trunk, native GPU frames and actual headed Chrome. A bounded heartbeat updates lastSeen without appending observations, changing outcome or rewriting the first fatal timestamp. The browser refreshes actual context/UTC and persists only metadata after releasing its report borrow. Chrome checks unchanged scene, selection, camera, history and geometry counters, exact stored JSON, and failure preservation after real GPU destruction.

The next endpoint owns its native interval, Window and Rust Closure together. Startup registers 15000 milliseconds; cancellation clears the native interval before releasing the callback. Explicit stop is idempotent and replacement clears the preceding owner. The timer captures no device, renderer, document or source. Chrome observes the actual requested period and accelerates delivery only in the proof, checking multiple metadata updates, exact handle cancellation/replacement, zero GPU calls inside callbacks, continued diagnostics after GPU loss and ready startup/current download when registration throws. It does not measure real 15-second wall-clock scheduling or phone performance. Automatic lifecycle suspension/final-exit hooks follow next.

Typing estimates are 17–33 and 16–32 minutes. The first timer capture passed its custom assertions but failed the common no-page-error check because its observer accessed GPUQueue on the Back-test navigation target without WebGPU. The helper now guards that interface; a full repeat passes with the original page-error assertion intact. Both final captures retain the white full-window canvas and actual command dock.

## Final-close and page-transition ownership

Two further checkpoints pass 132 native tests each, WebAssembly, Trunk, native GPU frames and actual headed Chrome. Final close validates timestamp text before mutation, marks healthy reports Closed and preserves Failed with its original fatal timestamp and observations. A targeted late-ready test failed before the correction: a milestone reopened Closed as Ready. That promotion is now prevented, and Chrome sends the actual late observation through the debug metadata export to prove Closed remains stored. Scene, placement, selection, camera, history and geometry counters stay unchanged. This operation does not itself cancel an outstanding adapter/device promise.

A separate browser owner retains two metadata bindings and an Rc allocation token independently of the eighteen drawing/input bindings. Cached pagehide records activity and pauses its interval without closing the report; pageshow resumes one interval. Final exit persists close immediately, then disposes listeners on the next microtask only if the allocation still matches. The callback returns before its own Rust Closure is released. Taking owners out of RefCell slots before dropping them avoids reentrant borrows. Denied registration cleans the partial binding, records unavailable lifecycle service, and leaves ready drawing/current downloads usable.

Chrome has focused transition acceptance plus the common real command/camera/input checks; it does not alter the older optional ownership assertions. It separately observes two metadata/eighteen GPU bindings, actual device destruction, retained post-loss observations, healthy/failed final cleanup, zero late GPU work and replacement before an older cleanup microtask. Synthetic persisted transitions establish the branch, not actual browser cache eligibility. Earlier checkpoints retain independent fresh evidence. Hidden/frozen suspension reasons, pending-startup revocation, error observations, complete phases and bounded recovery remain next.

Typing estimates are 22–44 and 26–52 minutes. An initial close capture failed an inherited command-history assertion; its checker now logs the inspector on inherited failure, with the assertion intact. The full repeat and final corrected-policy recheck pass. Its input cause remains unestablished and belongs to the pending dock audit.

Production diagnostics separately corrects cached outcome loss and late-milestone overwrites in 16f01f7e. Three new regression tests failed before and all ten pass after; headed Chrome reproduces the cached failure before and passes all three lifecycle cases afterward. All ten recency cases, the full serial native suite (506 passed, 55 ignored) and WebAssembly pass. The permanent far-floor oracle actually renders all six desktop/portrait configurations: 100.0000% of visible samples remain inked. It is a visible-edge acceptance, not zero hidden-line-leak evidence or a phone timing measurement. Rendering code is unchanged by this metadata correction.

## Reproduce

From `session_viewer`:

```sh
npm --prefix ../session_tests run course -- verify
npm --prefix ../session_tests run course -- verify --stored
npm --prefix ../session_tests run course -- serve 8781
```

Install browser-test dependencies once if needed:

```sh
npm install --prefix target/course-tools playwright@1.58.2 pngjs@7.0.0
```

In another terminal:

```sh
npm --prefix ../session_tests run course -- capture
npm --prefix ../session_tests run course -- generate
```

Capture checks the build fingerprints and runs each uploaded specimen’s handwritten Rust example before launching Chrome. Each checkpoint gets a fresh browser context; reload and Back checks within that checkpoint keep the same page. These commands use `target` and leave `workspace/journey` untouched. The separate `structure` audit includes the unfinished production destination comparison; its remaining differences must be resolved before declaring the entire course complete.

[Return to the course](../journey.md)
