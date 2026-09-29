# Viewer course — resume memory, 2026-09-28

## Current work and latest instructions

The user resumed tutorial development after the allowance pause. The course remains unfinished; the old pause request is revoked. They now request a GitHub push after each completed tutorial and a URL for reviewing each one. They prefer the existing http://127.0.0.1:8788/docs/#/course address; replace an old server if it is running, rebuild, and supply actual tutorial URLs.

Completed in the resumed turn: 18-light (flat shader lighting, 1–2 h), 19-actions (browser-independent Editor/Action/Change, 3–5 h), 20-resize (synchronized canvas/surface/depth/camera and dense displays, 3–5 h). All 20 checkpoints passed a full fresh verification batch, then verify --stored passed all 20. Six TypeScript tests, structure (43 reference checkpoints / 351 files), and source parity passed. D2 diagrams through 20 are generated. The strict Vue build through lesson 20 passed; the packaged Trunk release build passed. Its dist/docs index matches the current Vue build and its assets contain tutorial 20.

GitHub publishing: command-line Git cannot resolve github.com. The GitHub connector can read remote main, currently 55ad395f1dc0c74e94da0826f10bad393f1dc17d, which is ahead of local c4a2ba0d by a separate earlier course effort. Do not overwrite those remote changes. Creating a separate tutorials/cumulative-viewer branch was attempted but automatic approval review rejected the action: this session cannot request approval. No branch, commit or push was created. Do not retry mutations through another route to evade that rejection. Lessons 18–20 are ready locally, not published.

Server: no viewer server appeared in the process list. Socket inspection was denied, and Vite preview failed with listen EPERM: operation not permitted 127.0.0.1:8788. No process was killed. The user can start the already compiled docs outside the restricted tool environment with ./docs/serve.sh preview from session_viewer. The preview mode serves /docs/ at port 8788. The user later reported an existing listener and explicitly requested automatic shutdown: serve.sh now sends SIGTERM to TCP listeners on its selected port (including a --port override), waits up to five seconds, then launches Vite. Build mode does not touch ports. Shell syntax and mocked process-order checks passed; actual listening remains unavailable in this environment. URLs /docs/#/course/journey/18-light, /19-actions and /20-resize are valid routes but are NOT live until that command can run.

Browser captures remain pending. Native images are labelled accurately. No application behavior or unrelated source was changed in the resumed turn.

The full course is NOT finished. Most of the conversion to a clear cumulative course remains. There is no reliable completion-time estimate. Do not confuse the preserved complete reference with the new beginner course, and do not mark the goal complete just because the opening works.

This file supersedes the old workflow in docs-handoff.md. Current user instructions override historical requirements for Python, immutable write-once files, a hard one-hour limit, or automatic pushes.

## User's destination and teaching requirements

Deliver the complete Rust/wgpu viewer, with ALL current features and its crafted rendering quality, through Vue documentation. The learner knows no Rust, wants to type all implementation, and needs concise teacher-like explanations, local ownership and a global architecture map. Include an approximate time at the top, meaningful diagrams and distinct evidence of each visible step. Each completed lesson must build and work. Longer lessons and cumulative edits were accepted; do not split files mid-function or reset the project halfway. Keep useful comments and blank lines. Avoid supplied application code, repetitive pictures, huge fragment indexes and mechanical prose.

The user explicitly rejected Python orchestration. Application and executable checks are Rust; course tooling lives in the existing Vue/TypeScript project. The final feature destination is mandatory, not an optional follow-up.

## What exists now

Twenty cumulative lessons are authored in session_viewer/docs/journey/course.json:

1. 01-canvas — HTML canvas and Rust entry point (1–2 h)
2. 02-clear — device, surface, first GPU clear (2–3 h)
3. 03-triangle — shader and pipeline (1–2 h)
4. 04-input — background controls (1–2 h)
5. 05-vertices — vertex buffer (1–2 h)
6. 06-indices — shared vertices and indices (1–2 h)
7. 07-uniforms — GPU transform data (2–3 h)
8. 08-camera — flat camera state (2–3 h)
9. 09-matrices — matrix transforms (2–4 h)
10. 10-depth — depth buffer and overlapping geometry (2–4 h)
11. 11-scene — CPU scene versus GPU resources (3–5 h)
12. 12-identity — stable IDs, selection and deletion (3–5 h)
13. 13-picking — pointer coordinates and CPU picking (2–4 h)
14. 14-history — shared snapshots, undo and redo (2–4 h)
15. 15-perspective — kernel dependency, 3D camera and ray picking (4–7 h)
16. 16-orbit — quaternion orbit, target, isometric view (2–4 h)
17. 17-solid — kernel box, mesh adapter, fallible edit transaction (3–5 h)
18. 18-light — world-space flat normals and lighting (1–2 h)
19. 19-actions — shared editor actions independent of HTML (3–5 h)
20. 20-resize — synchronized drawing size, depth and aspect (3–5 h)

Every lesson has individually passed its applicable wasm build, Trunk build, native tests and native GPU pixel checks. Lesson 20 was the last completed check. These are NOT browser verification results. The current full batch and all stored fingerprints passed after the harness changes.

Latest reconstructed application: session_viewer/target/course-20-resize. Canonical source is course.json plus docs/journey/code snippets, not generated target files. Final application is still a small educational viewer, far from production feature parity.

## Code ownership and behavior through lesson 20

- Mesh owns validated CPU positions/colours and u16 triangle indices. from_kernel adapts session_rust::Mesh::to_render with checked index conversion.
- Scene owns Objects with stable ObjectId and Rc<Mesh>. IDs are monotonic and remain unreused across undo branches. Scene::add_box creates/translates/colours a kernel box.
- GpuMesh owns uploaded buffers; selection changes upload colour without altering document geometry.
- Renderer owns device, queue, pipeline, GPU meshes, a 64-byte matrix uniform and a fixed 640×480 Depth32Float attachment. Lesson 18 passes world positions to the fragment shader, derives a flat normal and applies fixed two-sided lighting. Lesson 20 recreates the depth attachment on resize.
- Camera owns target, distance, quaternion orientation and aspect. Uses kernel look-at and perspective transforms; ray() unprojects near/far points. Orbit maintains orthonormal axes.
- Picking intersects CPU ray/triangles and returns the nearest ObjectId within the clipped ray. Later GPU ID picking is still required for final accuracy/scale.
- History stores at most 64 Scene snapshots with shared Rc meshes. try_edit rolls back a failing edit while preserving redo; successful edits clear redo.
- Editor (lesson 19) owns document, selection, history, camera and background; Action requests changes, Change tells the browser whether GPU uploads are needed. Browser owns Editor and Renderer. HTML controls, canvas click picking, undo/redo, box creation and isometric view are wired. Listener lifetime is currently page lifetime. Lesson 20 uses one Event callback for clicks and window resize, preserving camera aspect on reset. Viewport converts CSS size and density to bounded physical size. Gestures, element ResizeObserver and event lifecycle remain to teach.
- Browser canvas surface and pipeline use an sRGB view to match native colour encoding. CSS outline avoids border-coordinate mismatch.

## Maintained tooling and canonical files

Under session/:

- session_viewer/docs/journey/course.json — ordered lessons and prose, exact before/after edits, hours, checks, browser actions.
- session_viewer/docs/journey/code/ — authored snippets. before:null creates a file; other replacements must match exactly once; empty snippet deletes the block.
- session_viewer/docs/journey/checks/ID.rs — native behavior/pixel checks.
- session_viewer/docs/journey/frame.rs — shared native frame harness.
- session_viewer/docs/diagrams/journey-NN.d2 — diagram sources; generate SVG with D2, not manually placed text.
- session_viewer/docs/journey/destination.json — preserved final features/source contract.
- session_tests/scripts/course.ts — CLI, loaded through scripts/typescript.mjs and existing esbuild.
- session_tests/scripts/course/{model,checkpoints,render,verify,png,reference,reference-checks,diagrams,foundations,serve}.ts — focused tooling modules.
- session_tests/scripts/course/checkpoints.test.ts — six passing tests, including save/restore, symlink-safe exports, dependency updates and portable kernel paths.
- session_viewer/docs/capture_journey.cjs — Playwright startup/actions/screenshots; supports exact button names and normalized canvas clicks. No browser run has succeeded in this environment.
- session_viewer/workspace/journey — learner workspace. Never overwrite personal work. Save/restore backs up unfinished edits; reference export requires a separate safe destination.

Dependencies are pinned. Initial lock: docs/journey/release/Cargo.lock. Lesson 15 introduces docs/journey/release/15-perspective.lock and the session_rust path dependency plus getrandom js feature. The dependencies ID command checks the expected Cargo.toml, backs up the old lock and changes only the lock. Exports relocate the kernel path correctly. The dependency is the existing geometry kernel, not hidden viewer implementation.

Verification hashes lesson source, selected lock, frame helper/constructor/size, common harness, checker and (when used) kernel source/Cargo.toml/build.rs. It compares materialized code with displayed source before compiling. The field is frame_constructor, not constructor (Object prototype collision was fixed).

Seventeen obsolete Python course generators/checkers were removed. Vue launchers replace MkDocs execution; mkdocs.yml is navigation data only. Old illustration generators and archived reference Python test utilities still exist but are not the new course workflow. Missing Vue snippet includes are fatal.

## Verification evidence and limitations

- Exact results/logs: session_viewer/target/course-checks/results.json and adjacent ID-{wasm,tests,web,frame}.log files.
- All 20 checkpoints passed a fresh batch and current stored-evidence validation. Do not repeat that batch unless source/checker changes require it.
- Six TypeScript tooling tests passed before lesson 17. No heavy command is currently running.
- Native images and D2 diagrams exist through lesson 20. The shared native harness now handles arbitrary dimensions and 256-byte padded readback rows. Lesson 20 checks a 770 × 385 image so row padding is exercised.
- Browser status is explicitly 'not verified'. No connected browser; installed Chrome launch failed with setsockopt: Operation not permitted. Local listening sockets are restricted. Do not claim browser interaction, browser screenshots or a working localhost link.
- Strict Vue and packaged Trunk release builds now cover lessons 01–20; both passed.
- Full reference preserved separately: 43 checkpoints, 351 text files, 271 Rust/WGSL source/test files, 77 commands. All stored reference proofs were current when last checked.
- Reference sections 01 and 37 were rebuilt using the TypeScript orchestration and Rust reference_pixels.rs checker. Section 37 passed 487 tests, 53 ignored; rendered scene had 92,899 ink pixels and five visible object IDs. This does not establish completion of the cumulative course.

## First actions after the user resumes

1. Read this memory, inspect git status and course.json; preserve all existing changes. Do not rerun temporary author scripts from /tmp: they can overwrite later fixes.
2. Lessons 18–20 are done locally. Continue with focused input/gesture lessons, then document loading and geometry growth. Keep using the shared Editor action path instead of duplicating editing behavior inside browser handlers.
3. Overview, release and README now describe the current 20-lesson coverage. Generate/check pages and diagrams after new lessons. Publish each completed tutorial and give its review URL when the environment permits; disclose publishing/server blocks accurately.
4. Existing lessons have current build evidence. Verify new checkpoints and repeat older ones only when an edit changes their source or harness. Build the Vue site and packaged viewer at each review milestone.
5. Continue the SAME project toward resize/gestures, full documents and loading, geometry/CAD rendering, curves, text, tools, panels, commands, GPU picking and final visual quality. A continuous bridge to all current production features has not been designed/implemented yet; this is the largest remaining work.
6. Capture actual browser interaction/screenshots when a usable browser/server is available. Native readbacks remain clearly labelled meanwhile.

## Commands and resource limits

Run from session/session_tests:

```sh
npm run course -- generate
npm run course -- diagrams
npm run course -- structure
npm run course:test
npm run course -- verify --stored
npm run course -- verify
npm run course -- parity
npm run course -- verify-reference --stored
npm run course -- foundations
```

Individual lesson: npm run course -- verify 20-resize.
Full verification log: redirect to ../session_viewer/target/course-checks/all-lessons.log.

Strict Vue build from session/session_tests:

```sh
buildslot timeout 10m prlimit --data=6442450944 -- env DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ node --disable-wasm-trap-handler --max-old-space-size=2048 node_modules/vite/bin/vite.js build --outDir ../session_viewer/target/docs/vue --emptyOutDir
```

Packaged viewer build from session/session_viewer:

```sh
buildslot timeout 10m prlimit --data=6442450944 -- env NO_COLOR=true CARGO_BUILD_JOBS=4 CARGO_NET_OFFLINE=true RUSTC_WRAPPER= REGEN_PROTO=0 trunk build --release
```

One heavy job at a time; four Cargo jobs, 6 GiB data cap, ten-minute timeout. A virtual-address cap breaks V8; use the data cap. Shell login:false. Network/approval unavailable in this environment. No subagents unless explicitly authorized. Do not touch unrelated .claude/agents/session-reviewer.md, compas_tf or kernel changes. Do not git add -A. The user now authorizes tutorial commits/pushes; preserve unrelated work and remote changes. GitHub mutation approval is currently blocked.

## Required final cleanup — latest user instruction

After the entire replacement course is complete and verified, DELETE the old tutorials. The user explicitly requested this; do not leave competing legacy courses or an archive presented as another learning path. Keep the old reference only while it is needed to verify the replacement. Before deleting it, prove all destination features, source coverage and visual-quality cases against production directly. Then remove obsolete lesson pages/crates, legacy snippet manifests, redundant images, old course navigation and tooling that serves only those tutorials. Update links and rerun the strict Vue build. This cleanup is a completion criterion, not authorization to delete the reference prematurely.


## September 29: browser crash and interaction ownership work

Latest request adds Firefox crash diagnostics, an actual Chrome viewer check, and argument-order ownership to the active course/viewer task. Firefox 156.0.1 on Intel Mesa/iris 25.2.8 crashed in the main process with Cannot get non-existent resource QueueId(0,2). Do not claim its root cause is proven or that a page can write a native dump after the process dies.

Production edits now in the working tree:
- assets/diagnostics.js starts before Wasm, keeps four bounded local records and a 15-second heartbeat, saves the first error, attempts a JSON download, offers an explicit download banner and previous-run recovery. JavaScript errors, unhandled rejections, Rust panics and WebGPU failures feed it. Geometry and keystrokes are not recorded.
- State::gpu_failed gates App adoption, loader messages, UI/window events and render before GPU work. It handles failure only once. Avoid cancel_cloud_query here: that normal path uploads controls; failure instead drops the pending CPU query directly. Uncaptured GPU errors now request a redraw, including when the viewer was idle.
- docs/open-chrome.sh uses a separate profile with Vulkan, DefaultANGLEVulkan, VulkanFromANGLE, disable-vulkan-surface, enable-unsafe-webgpu and ozone-platform=x11. Never change the user's ordinary Chrome profile.
- tests/diagnostics.test.cjs has four passing persistence/export/retention/storage-denial/startup checks. tests/device_loss.cjs instruments real GPU methods, destroys a device, requires zero later GPU work, and checks download/reload recovery; browser execution still pending.
- docs/debugging.md teaches browser diagnosis and ownership, with browser-recovery.d2/SVG. README links it. Legacy reference and destination pages now explicitly disclose that September 29 production fixes are ahead of their September 28 snapshots. Do not merely rehash destination.json to conceal stale parity; propagate fixes into course code and reverify later.

Browser evidence: during a temporarily unrestricted period, headed Chrome 153.0.8010.36 rendered the full five-object viewer. target/course-checks/chrome-viewer.png and chrome-viewer.json record that pre-fix run (one console 404, likely favicon; explicit empty favicon is now in index.html). Lessons 01–03 passed headed capture. Lesson 04 failed its keyboard pixel round-trip; investigate initial presentation timing versus input delivery, do not weaken the assertion. No complete browser.json exists. Headless screenshots were misleadingly blank, so capture_journey.cjs now defaults to headed.

Permissions reverted to workspace-write, no network/escalation. Fresh Vite listener on 8789 fails EPERM; fresh Chrome launch fails setsockopt Operation not permitted. Log: target/course-checks/chrome-launch-latest.log. Earlier server handles are gone; do not give localhost URLs as currently running. GitHub branch creation was already rejected by automatic approval review with approval policy never; no commit/push has occurred.

Wood ownership: viewer reads element.features(), not interactions. WoodSession::add_interaction was hosting contact/beam geometry on graph edge.v0. It now hosts on the call's source, removes the previous endpoint copy, preserves graph ordering, flips contact face indices appropriately, and reuses a beam joint record without mutating caller volume order. Plate joints retain their two explicit sides. Existing instance features retain GUIDs during removal. Beam recomputation cleans both endpoints. Regression: wood/tests/interaction_ownership.cpp, CMake target interaction_ownership.

IMPORTANT: wood contains concurrent user edits to a joint-class/protobuf refactor. Preserve them; do not add all files or revert them. Current whole-Wood build stopped on missing element_joint.pb.h. A separate baseline under /tmp/wood-ownership-check uses git HEAD source plus just our ownership changes and the shared kernel. Baseline regression built and FAILED as expected with old contact host must be cleared. Fixed isolated build/test is in progress; update evidence below when complete. Disable compiler cache in that temporary configure with -DSCCACHE_PROGRAM= -DCCACHE_PROGRAM= because sccache sockets are denied. Always use buildslot for C++ builds and wood/tools/run_guarded.sh for test execution.

Isolated ownership regression: PASS after the old baseline failed its ownership assertion. Tested exact current add_interaction and instance-feature removal functions plus both-endpoint recomputation cleanup on Wood baseline a286722ca52d463a1a94d3785c665d735173c299 and shared kernel 70a1ebe918c04200b24ba2daf6e0815d1d9511e4. Additional assertions preserve canonical volume order and remove rehosted features during recomputation. This is not a claim that the concurrently edited full Wood checkout builds. Build logs /tmp/ownership-{baseline,fixed}-build.log.

Viewer verification update: wasm cargo check passed. Native default parallel suite crashed with SIGSEGV after 513 reported tests; a fresh serial run passed all 487 ordinary tests, 53 ignored, in 14.71 seconds. Do not hide the parallel failure or claim its cause established. Logs target/course-checks/viewer-fixes-{tests,serial-tests}.log. Four diagnostic JavaScript tests pass (run directly with node tests/diagnostics.test.cjs to see all four). Strict Vue build passed and includes the browser-recovery diagram and troubleshooting/ownership documentation. Trunk release rebuild is underway; record result before handing off.

Final local verification for this fixes pass: optimized Trunk/Wasm build PASS; strict Vue build PASS; packaged diagnostics.js byte-equals the source, packaged docs index byte-equals the current Vue output and the docs bundle contains the Firefox QueueId signature. Diagnostic tests now FIVE passed, including corrupt/older local-storage records. Device-loss browser test now routes a controlled boxes-only manifest and checks initial console errors with URLs before destroying the device. Its actual browser run is still blocked. The browser-control tool also reports Browser is not available: chrome. No new cumulative lesson was added in this pass; the course remains at 20. No GitHub push or live review server.


## Next continuation: lesson 21 built locally

User now requires a visible bullet checklist with accomplished items checked, reposted after every completed item. Keep the full-course scope on that checklist, including browser screenshots, all current features, parity, publishing and final legacy removal. Latest turn is PROGRESS: authoritative lesson code/docs/diagram/render and verification changed; not a no-progress turn.

Course now has 21 lessons, 44–78 estimated study hours. New 21-gestures: Remember a press until it ends, 219 lines to type, 3–5 study hours. Source is in journey/code/21-gestures-01..17, course.json owns its edits, generated page journey/21-gestures.md. Gesture holds one Drag (ID/button/start/last/moved); Motion converts CSS positions/deltas into existing Editor Actions. Left release picks only under four CSS pixels, remembers any excursion even after returning; right drag orbits and consumes the last release delta; another pointer cannot finish it. Browser captures pointers, cancels on lost capture/cancel/blur/resize, borrows MouseEvent rather than moving Event, and does not redraw idle moves. Current buttons remain available. Mobile orbit/wheel/keyboard and listener lifetime cleanup are later work.

Verification: 24 native Rust tests passed; wasm build and Trunk lesson bundle passed; native rendering checks passed (rotated box face position/shade and area). Screenshot journey/21-gestures.png visually inspected. D2 journey-21.d2 rendered to its SVG. Six TypeScript tooling tests passed. Verify 21-gestures --stored confirms exact current source. Strict Vue build and packaged production Trunk release passed; dist/docs contains lesson 21. Logs target/course-checks/21-{verify,docs,packaged}.log and 21-gestures-{wasm,tests,web,frame}.log. No need to repeat these absent new source changes.

Browser capture tool now supports drag actions and asserts both changed orbit pixels and an unchanged image after release. It has NOT run for lesson 21 because Chrome/server access remains blocked; no browser screenshot is claimed. Review URL when the user runs docs/serve.sh preview: http://127.0.0.1:8788/docs/#/course/journey/21-gestures . There is no confirmed live review server and no GitHub push.

Known full-course structure failure remains explicit: Reference changed: src/app/feedback.rs (September 29 production diagnostics ahead of September 28 frozen snapshots). New lesson listings/image uniqueness checks pass before that destination fingerprint gate. Do not update fingerprints alone to pretend parity. Source integration into reference/replacement and browser verification are unfinished completion requirements.

Next work: add wheel zoom and focused keyboard controls using the existing Gesture/Editor route, then loading real documents and geometry growth. Keep the whole scene/document/rendering architecture visible and preserve the final feature contract. Author scripts /tmp/author-viewer-21.mjs and finish-viewer-21.mjs are one-off and MUST NOT be rerun: they would duplicate or overwrite lesson edits. Existing learner workspace/journey remains untouched.

## September 29 continuation: lesson 22 and screenshot audit

Latest user steering questioned whether native images 16-orbit through 21-gestures were the same/correct. Inspected all six actual PNGs visually. They are distinct: 16 orbited triangles; 17 unlit grey box; 18 three shaded box faces; 19 selected yellow box; 20 wide 770x385 viewport; 21 visibly rotated scene. Compared each PNG byte-for-byte with png(saved frame.ppm), raw RGB hash with results.json, and both target/docs/vue + dist/docs assets; all matched. Stored source proofs for all six are current. Curl to the user's exact localhost URL is blocked by socket EPERM, so this does NOT verify live HTTP/cache contents.

Teaching correction: render.ts now puts a bold 'Native render check — not a browser screenshot' label BEFORE unverified native images, not only a small caption after. course.json image_result explains setup, visible difference and what the image cannot prove for 16–22. Regenerated all pages. release.md has a six-lesson comparison table and audit scope. Never claim a single still proves resizing, release/cancel or browser input. Actual browser interaction sequences/screenshots remain an unfinished requirement. BrowserAct CLI was checked and is absent; no installation or browser run occurred.

New checkpoint 22-shortcuts, 'Give the keyboard a place to work': 150 lines, 2–4 active study hours. Total 22 lessons, 46–82 hours. Source snippets 22-shortcuts-01..11 in docs/journey/code; manifest authoritative. Stateless shortcuts::wheel reads pixel/line/page mode (16 CSS pixels per line, canvas height per page), clamps per-event movement to +/-600, exponential positive Zoom factor; invalid/zero inputs yield None. shortcuts::key translates arrows, +/- and Home, Delete, Ctrl/Cmd+Z, Shift+Z/Y into existing Editor actions. Held navigation repeats; document commands do not. Browser adapter skips Alt/composition, Escape cancels Gesture, handles keys only on focusable canvas, listens for canvas blur, explicitly focuses on accepted press, uses non-passive canvas wheel listener. Tab stays browser-owned. No duplicate camera/history path. Existing dependency versions unchanged; added web-sys features only.

Four new Rust tests cover equivalent wheel units/small-step accumulation/inverse motion, bounds and invalid data, repeats/modifiers, and undo/redo keeping zoom independent. 28 Rust tests passed, wasm build passed, lesson Trunk bundle passed, native frame and projected face/area assertions passed. verify 22-shortcuts --stored passed. Six TS tooling tests passed. D2 diagram22 rendered. Native screenshot22 visually inspected: closer box and translated view. Capture tool supports wheel/key actions, asserts changed pixels, no page scrolling and no navigation after focus moves to a button. These browser actions have NOT been executed.

Strict Vue build PASS; built route JS explicitly checked for new evidence labels, lesson-specific captions and lesson22. Packaged Trunk release is currently running (exec session 96478); poll and record completion. Logs target/course-checks/22-{verify,docs,packaged,structure}.log plus 22-shortcuts-{wasm,tests,web,frame}.log. Structure passes the new lesson/content/image checks then still fails the known destination drift at src/app/feedback.rs. Do not hide that discrepancy by merely changing hashes.

No GitHub push; earlier automatic approval review rejection for branch creation still applies. No confirmed accessible localhost server. Review URL when preview runs: http://127.0.0.1:8788/docs/#/course/journey/22-shortcuts . docs/serve.sh preview stops its port's existing listener before restart, already implemented. User wants checklist reposted at each completed item; delivered throughout this continuation. Full-course objective unchanged, old tutorials retained pending full replacement verification.

This continuation is PROGRESS: new executable checkpoint and source, test/build/render evidence, corrected teaching captions, release audit and rebuilt Vue docs. The preceding checklist-only response was no progress; independent course work was available and resumed. Do not mark goal blocked while course implementation can continue. Next work: document loading and geometry growth (all production features still required), plus browser evidence when available. Do NOT rerun /tmp/author-viewer-22.mjs: it is a one-off author script and the manifest now contains later caption edits. No changes to workspace/journey.

Lesson22 packaging completion: Trunk release PASS (session96478 terminal0). Packaged dist/docs index and lesson16/20/21/22 JS modules byte-equal the fresh Vue output. No running build handle remains. Browser verification/publishing remain pending; native evidence and caption improvements are complete locally.

## September 29 continuation: lesson 23, actual mesh-session import

Previous goal turn was PROGRESS (lesson22 and screenshot audit). Current continuation is PROGRESS: added lesson23, executable import and source ownership, tests, distinct native image, generated pages/diagram and passing builds. Full-course scope remains active, not blocked merely because publishing/browser checks are unavailable.

New 23-import, 'Keep the document behind the picture': 258 lines to type, 5–8 study hours. Course total23, 51–90 hours. Nineteen edits in journey/code/23-import-01..19; course.json authoritative, generated page journey/23-import.md, D2/SVG journey-23. New dependency lock23-import.lock adds direct prost0.14.4 and js-sys0.3.105 (same versions as transitive lock); web-sys features File/FileList/Blob/HtmlInputElement/CustomEvent/Init. Existing learner workspace untouched.

Architecture: document::load decodes proto Session, validates supported subset before kernel construction, builds every display Mesh, returns Loaded{Rc<Session>, Vec<(guid,Mesh)>}. Scene Object now has optional Source{Rc<Session>,guid}; demo objects None. Import appends meshes in one existing History::try_edit transaction. Local ObjectId remains distinct from source GUID, so repeated file imports remain independently selectable. Rc source survives Undo/Redo with mesh buffers shared. Failed decoding preserves scene, selection and redo. Current subset intentionally rejects placements/definitions/interactions, nonmesh geometry, nonflat tree rows, face holes, n-gons, non-object colour modes; caps4MiB,64meshes,10000vertices/faces per mesh. This is NOT final production file support; expand it in later lessons, do not relabel it full compatibility.

specimen.rs builds a three-piece orange frame using the real kernel and pb_dumps; examples/sample.rs writes sample.pb. Learner types both. file_input.rs handles native browser file picker: checks size before read, clears input to permit reopening same file, shared request number ignores stale async completion, delivers Uint8Array through viewer-file CustomEvent to the callback owning Editor. No RefCell<Editor> / borrow held across await. Browser callback routes Import action and reports success only after acceptance. Async/browser delivery remains UNTESTED in Chrome.

Verification:32 Rust tests passed; four new tests cover retained source+whole-importUndo/Redo, failed import preserving selected/redo, repeated imports with independent local IDs, malformed/unsupported protobuf checks. wasm build, lesson Trunk bundle, native frame/pixel checks PASS. Native helper writes actual sample.pb then READS that file for rendering; it does not merely generate equivalent bytes in memory. The frame checks projected top beam colour and visible orange area. Image23 visually inspected: distinct orange three-piece frame plus prior pink/cyan triangles. verify23--stored passes final source fingerprint. Initial run was rerun because the helper changed during its first build. No need to repeat again absent changes.

Capture tool adds file actions, setInputFiles, successful import wait, canvas change and whole-importUndo/Redo pixel equality. Model adds browser_result_status because successful file import changes status text. This tool has NOT run under current socket/browser restrictions. Six TypeScript tooling tests pass. Structure passes lesson checks then fails known src/app/feedback.rs destination drift. StrictVue buildPASS. Packaged Trunk release currently running exec session4651. Logs23-{verify,docs,packaged,structure}.log and23-import-{wasm,tests,web,frame}.log. Poll current actual handle and correct memory with terminal result.

User asked how to enable permissions. Read OpenAI Docs SKILL, fetched official https://learn.chatgpt.com/docs/permission-modes and cloud-security docs. Answered: permission menu below composer → Ask for approval to allow approval prompts; optional Settings→General→Permissions→enableFullaccess, then SELECTFullaccess for this chat (enabling alone does not change chat). CLI /permissions. Fullaccess grants broader file/network access. GitHub app action permissions separate and may be admin-restricted. Did not change settings, bypass sandbox or retry previously denied publishing. Current developer profile still workspace-write/networkrestricted/approvalnever. If user changes mode, verify actual new profile/access before claiming launch/push works.

Review URL when preview runs: http://127.0.0.1:8788/docs/#/course/journey/23-import . No confirmed live server/no push. User checklist repeated at milestones. Do NOT rerun /tmp/author-viewer-23.mjs (one-off; course.json now contains prose edits). Next lessons need document placement/hierarchy, camera fit and readable document/object inspection, then remaining geometry/rendering/features. Editor::apply still has nested wildcard/unreachable view dispatch; architecture cleanup pending as recorded earlier. Full old course preserved until complete parity and browser verification.

Lesson23 packaging completion: Trunk release PASS, session4651 terminal0. Packaged docs index, lesson23 JS and its PNG byte-match current Vue/source artifacts. No live build handle remains. Full course, browser evidence, architecture/parity, publishing and legacy removal remain unfinished.

## September 29: permissions enabled, browser evidence completed for 01–23

User enabled danger-full-access + network and explicitly requested previously blocked work, including Chrome and publishing. Approval policy is still never; no sandbox override arguments needed. Previous turn was progress (lesson23); this one is progress (real browser verification and publishing preparation).

Live docs preview restarted via docs/serve.sh preview: script terminated old PID315352 on8788, new exec2146. HTTP /docs/ returns200. Existing course-bundle server PID318629 on8781 remains live,23-import/dist returns200. Production Vite preview on8770 started as exec58046 and responds200.

Headed Chrome153.0.8010.36 launched with the documented Linux WebGPU/Vulkan flags. All23 lesson browser checks PASS; docs/screenshots/journey/browser.json now has23 source/bundle fingerprints and actual full-page captures. Lesson04 earlier failure diagnosed precisely: keyboard focus outline from adjacent button overlays197 pixels of canvas's top CSS border. Real GPU pixels were exactly restored (native-size640x480 readback hashes equal). capture_journey.cjs now hashes decoded canvas.toDataURL drawing pixels via pngjs, excluding surrounding CSS, while screenshots retain full real page appearance. Probe evidence/logs under target/course-checks/probe04; no application workaround or weakened drawing assertion. Full-run log target/course-checks/chrome-course.log. Browser23 image visually inspected: file import status, native file picker and three orange frame pieces with triangles.

Production tests/device_loss.cjs PASS in headed Chrome against8770: forceddevice.destroy, zero subsequent guarded GPU calls, downloaded report, recovered previous failure onreload. Artifacts target/device-loss/{before.png,report.json,...}; logtarget/course-checks/chrome-device-loss.log. This does NOT prove the Firefox Mesa QueueId crash fixed or reproduce its root cause.

Generated pages prefer real browser screenshots now. Added browser_caption for16–23 to explain visible change and tested interaction. release.md now documents completed Chrome cases and limitations; precise table avoids claiming cancellation/picking-after-resize/DPR2 coverage. Full course/future lessons/parity/Firefox remain incomplete. StrictVue build completed once withbrowserimages; final release text/table/setup instructions changed afterward, so rebuild before end/publishpreview.

Publishing preparation: remote main55ad395f is18 commits ahead of localc4a2ba0d, with another historical course rewrite. No force push or change to remote main. Isolated worktree /tmp/session-course-review, branch tutorials/cumulative-viewer, based at localc4a2ba0d. Copied only task-scoped modified/untracked files from session_viewer, session_tests, and three2026-09-28viewer plan files. Mainworktree and unrelatedsubmodulepointers untouched.6561stagedpaths (~147MiBsource, mostly preserved tutorial snapshots). Includes full pending viewer refactor/diagnostics and current+reference teachingwork; original reference retained peruser untilfinalreplacement. Do not claimfullcoursefinished.

Review branch pins session_rust d390def4245be2469339b3bcea463ee1e5a5ea3b, the exact clean kernel used in all builds. Confirmed commit already exists on GitHub via gh api. Kernel worktree /tmp/session-course-review/session_rust created withgitworktree, no new kernel edits/push. session_proto2cf406ac matchesoriginal parent pin. Leave unrelated mainworkspace session_cpp/py/data pointers and docs/plans/docs-handoff.md alone. gitdiff--check reports intentional snippet blankEOF plus a few existingreferencewhitespace lines; don't trimfragments blindly and invalidate reconstruction. No commit/push yet at this point; complete finaldocsbuild, sync revised files into reviewworktree, commit/push branch, and inspect actualtriggeredworkflows. Main/develop-only push workflows mean reviewbranch maytriggernone; reportthat honestly. Branch is intentionally divergent; integration with newer main remains review work.
