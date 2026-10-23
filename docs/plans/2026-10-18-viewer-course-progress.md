# Session Viewer course: progress and remaining work

Updated 2026-10-23 on Windows. This is the current execution plan, not a declaration that the viewer course is finished.

The book now has **22 chapters and 111 reader-facing lessons**. All **1,360 coding checkpoints**, plus the maintained-viewer reference comparison, remain recoverable. The previous Windows checkout had 18 integrated chapters and 287 reader-facing entries; the older Linux state file reported only 14 chapters and 691 entries.

Open the locally built tutorials at **http://127.0.0.1:8788/docs/#/course**. An isolated, visible Chrome window has been opened there. The local preview server must remain running.

## Stage totals

| Stage | Accomplished | Left | Status |
| --- | --- | --- | --- |
| Source synchronization | All 16 GitHub checkouts pulled to remote branch tips; all 25,829 tutorial files unchanged at the pull boundary | No source repositories awaiting this pull | Done |
| Windows tooling and missing inputs | Cross-platform pull entry points; guarded Windows builds; reproducible fixtures; WARP canvas presentation; actual docs build and cached no-op verified | Windows capture host ports authored and five focused native checks pass; validate course tooling, actual Windows frames and 4/4 regenerated inputs with the appropriate checkpoint contracts | In progress |
| A. Course consolidation | 22/22 chapters integrated; 111 lessons; all 1,360 coding IDs retained; strict docs build and changed-page UI checks pass | 0 chapters awaiting editorial integration | Done for editorial structure; runtime acceptance is separate |
| B1. Production display reconciliation | Ribbon/cloud paired evidence; Matrix five core plus three compact-owner tests; Plane four focused tests; hidden-range GPU proof; two actual 2M cloud tests and 18 production LOD comparisons | Point/glyph/sphere five tests and printed-fill two tests pass; printed alpha and all eight matched mesh images pass; finish matched CAD/vector/Plane, controls and browser evidence; 5/5 full groups remain open | In progress |
| B2. Runtime and performance | Maintained dependency detached from normal native builds; integrated frame coalescing compiles natively and in WASM; 3/3 scheduler semantic tests pass | Browser lifecycle acceptance and 5/5 measured categories; 3/3 full implementation groups remain open | In progress |
| B3. Reload camera and base grid | Both implementations updated; 4/4 course native regressions and 3/3 maintained grid checks pass | Actual deferred-load browser checks; 4/4 full combinations remain open | In progress |
| C. Final integration | All 1,360 checkpoint IDs and snippet references are present; prior docs/navigation baseline is built and checked | Latest full native run: 1,055/1,059 passed; all 4 failures subsequently passed in focused checks, including two corrected whole-owner fixtures and two fixture-directory errors. A clean final full run, 5/5 final gates and feature evidence for 19/22 chapters remain | In progress |

**Remaining work is five bounded batches:** missing inputs/environment, display reconciliation, runtime/performance, reload/grid, and final integration. A batch is not a promised one-session duration. Build and GPU defects are not yet measured well enough to give a reliable number of working sessions. The counts above describe accepted work, rather than treating existing source as verified behavior.

## Source synchronization — done

- [x] Run `.claude/hooks/pull_mono.sh` through Git Bash.
- [x] Pull `session`, `wood`, `wood_nano`, `compas_wood` and `compas_tf`.
- [x] Initialize/update the C++, Python and Rust kernels, schemas and actual nested data submodules.
- [x] Skip stale `.gitmodules` entries without a Git link.
- [x] Add `.cmd`/PowerShell launchers selecting Git Bash explicitly; retain the Bash entry point for Linux/macOS.
- [x] Add `--all-sources` and resumable nested-only synchronization.
- [x] Check Bash syntax, hook parsing and the Windows launcher. Linux/macOS execution has not been tested here.

The first pull was interrupted when its running script was edited. The nested-only run subsequently completed successfully. Pull remains a force sync; it discards local work in repositories it resets. Nothing has been committed or pushed.

## Windows tooling and missing inputs — in progress

- [x] Create a Windows Job Object guard: one heavy job, 2 GiB total job commit, two allowed processors, timeout and child cleanup. The 2026-10-20 user instruction authorizes more memory: browser jobs now use 6 GiB after a 4 GiB startup stall; current builds fit 4 GiB. Historical 2 GiB records remain unchanged.
- [x] Verify argument passing, exit codes, timeout cleanup and exclusion of another heavy job. JSON command transport also preserves Cargo's `--`, flags, spaces, empty strings, quotes and trailing backslashes.
- [x] Normalize course/source text to LF without changing production source content; preserve that behavior in Git attributes.
- [x] Detect the installed native Rust target and route Windows course commands through the guard.
- [x] Confirm Rust native/WASM targets and Trunk are installed.
- [x] Port the maintained docs prebuild lock for Windows Git Bash, which lacks `flock`; preserve its existing Linux path. Seven isolated mock checks pass, including inherited exclusive lock lifetime, concurrent writers, cached no-op, failed-output preservation and junction escape rejection.
- [x] Run the actual maintained docs prebuild through that Windows fallback and verify its cached no-op: 115.996 s build; 0.324 s no-op; output hash and timestamp unchanged.
- [x] Install the contract's pinned Playwright 1.58.2 and pngjs 7.0.0 locally under `target/course-tools`.
- [x] Author a kernel-based deterministic generator for all four missing fixture inputs, with fresh identities and hashes.
- [x] Compile the fixture generator and produce all four inputs under the Windows guard (196.956 seconds, 1,513,250,816 bytes peak job commit).
- [x] Verify protobuf round trips, two constructions per run and identical bytes/hashes in a second process.
- [ ] Validate `course-comparison.pb` against the unchanged course assertions.
- [ ] Validate regenerated `course-metadata.pb` against the unchanged course assertions.
- [ ] Validate regenerated `course-instance-features.pb` against unchanged source/ownership assertions.
- [ ] Validate regenerated `course-feature-pick.pb` against unchanged source/picking assertions.
- [x] Establish a working isolated software WebGPU context and actual readback on Windows. Chromium 149 with ANGLE D3D11 WARP and SwiftShader WebGPU reports `google/swiftshader`, fallback=true, and reads the expected RGBA8 pixel `[51,102,153,255]`. The guarded probe completed successfully; evidence is `software-webgpu-warp.cjs.json`, its log and resource manifest. Earlier failed probes remain preserved. This accepts the software context, not the course rendering scenarios.

Windows does not expose Linux's separate per-job zero-swap quota. The guard records that limitation and does not change global swap settings. Reconstructed fixture bytes are new inputs; historical images cannot validate them.

## A. Course consolidation — editorial structure done

- [x] Retain all 22 main chapters and integrate their 111 consolidated lessons.
- [x] Integrate the final four chapters: 21 lessons retaining 197 original checkpoints.
- [x] Check their 4,840 original code blocks, 2,346 snippet includes and 247 relative links.
- [x] Retain chapter.lesson numbering, checkpoint order, runnable endpoint mappings and old URLs.
- [x] Refresh the contents, daily plan, source map progress and course index.
- [x] Pass all 21 course tooling tests, including source replay and shared dock correspondence.
- [x] Pass the strict local Vite build under the Windows guard: 417.698 seconds and 1,713,123,328 bytes peak job commit.
- [x] Check all 21 newly integrated pages, index, contents, daily plan, legacy redirect and phone layout in an isolated browser.
- [x] Inspect all 26 final captures. Fix the checker to await the exact destination heading and decoded images; preserve earlier captures separately.
- [x] Open the locally built course in a visible, separate browser window.

This completes the book's consolidation, not its production-renderer reconstruction. No new microtutorial IDs were created and no production shader was replaced.

## B1. Display reconciliation — 0/5 groups freshly accepted on Windows

- [ ] Reconcile ribbon/vector stroke construction, joins, pen widths, terminal coverage and AA with actual production shaders.
- [ ] Verify production cloud octree/LOD/splat/depth/EDL and marker-density behavior with matched inputs.
- [ ] Verify CAD tessellation, real BRep holes, borders, NURBS isolines and mirrored/nonuniform placements independently.
- [ ] Complete original Plane, pen, point/cloud and marker display controls through actual semantic owners.
- [ ] Verify shared clipping, selection, family visibility, original colours and placed-instance behavior across the preceding groups.

Use the same original bytes, camera, viewport/DPR, samples, settings and output encoding for matched images. Existing code and earlier partial proofs do not count as new acceptance. Production remains the implementation authority.

## B2. Runtime and performance — measurements 0/5; implementation groups 0/3 accepted

Measurements on consistent scenes/settings:

- [ ] Cold load.
- [ ] Input-to-frame delay.
- [ ] Steady interaction frame time.
- [ ] Upload/allocation activity.
- [ ] Idle behavior.

Implementation and regression checks:

- [ ] Coalesce redundant input renders while preserving single-flight ownership and lifetime.
- [ ] Finish startup/loading and remove unwanted maintained-viewer dependencies from the final learner implementation.
- [ ] Reuse geometry/buffers and verify before/after costs; view-only settings must avoid rebuilding or cloning scenes.

Software GPU measurements must be labelled as such; they are not hardware FPS claims.

## B3. Reload and grid — 0/4 combinations accepted

- [ ] Maintained viewer: reload retains the latest camera, including motion during loading.
- [ ] Course viewer: the same camera guarantee.
- [ ] Maintained viewer: the world base grid changes only after an explicit plane-changing construction command.
- [ ] Course viewer: the same grid guarantee, including floating-origin rebases.

Complete implementation coverage before this batch; update existing lessons instead of adding repair-history tutorials.

## C. Final integration — 0/5 final gates passed

- [ ] Freeze the integrated runtime and pass the final Native/WASM/optimized build checks.
- [ ] Pass all declared runnable-checkpoint checks against their actual final inputs.
- [ ] Close every feature row in the production/lesson coverage map with truthful source and visual evidence.
- [ ] Verify final navigation, aliases and complete source history/checkpoint preservation.
- [ ] Publish locally, inspect the final book and report counts, measurements and remaining limitations.

The current docs build is a valid baseline for stage A. It is not a substitute for this final pass after runtime changes.

## Update rule and current evidence

After each bounded batch, update this plan and `session_viewer/docs/journey/consolidation/current-state.json`: mark only work whose stated checks passed; record counts completed/remaining, the current defect, actual commands/evidence, and the next deliverable. Preserve failed logs and images. Do not promote historical evidence or uncompiled proposals to done.

Current evidence is under `session_viewer/target/course-windows-20261018/`:

- `course-tests.log`
- `docs-build.json` and `docs-build.log`
- `docs-browser-verified.json` and `ui-verified/inspection.json`
- `software-webgpu.resources.json` and its failed-probe log

The first native fixture-check attempt reached the 1,200-second guard timeout (exit 124), with peak job commit 1,560,637,440 bytes. Its cached retry compiled and passed both copy tests, then exposed an MSVC helper keeping the guard alive after Cargo exit. The runner now waits for root-command completion, preserves its exit status and reaps descendants before releasing the mutex. Focused child-cleanup/timeout regression passes; the accepted cached copy run is `fixture-course-native-4.json` (exit 0). Metadata-filter checks pass 7/7 (including two renderer metadata checks); feature-picking source checks pass 1/1. These tests do not replace browser acceptance of the four regenerated inputs.

The corrected software-GPU launch also passes with the required pinned Playwright 1.58.2 and installed Chrome 155 (`software-webgpu-chrome.cjs.json`), so the unchanged canonical browser harness needs no browser-launch rewrite.

The current endpoint WebAssembly build passes (`endpoint-wasm.json`, 169.466 seconds, peak commit 1,399,296,000 bytes). The optimized Trunk build also passes (`endpoint-web-2.json`, 590.012 seconds, peak commit 1,574,285,312 bytes). These are baseline builds, not final release gates after runtime freeze.

The native-only stroke witness helper/example compile. Windows WARP with a task-local DXC 1.9 compiler renders all eight paired production/course scenarios on one shared CPU device (`stroke-witness-warp-dxc.json`, 45.590 seconds, peak commit 719,675,392 bytes). Preserve the raw images and settings in `stroke-baseline-warp-dxc`. Measurable differences are under review; requested pen width versus actual uploaded width must be checked before interpreting them as shader-only differences. No display group is accepted yet. Earlier SwiftShader render stalls and the FXC compiler failure remain preserved.

The first comparison-fixture browser attempt did not reach its assertions: startup readiness timed out. A supplemental probe verifies DOM capture but stalls at WebGPU canvas queue completion; ordinary JavaScript timers continue while animation frames stop. Offscreen texture readback alone therefore does not establish working canvas presentation. A fresh single-flag DirectComposition diagnosis is in progress; do not claim any regenerated fixture browser acceptance.

The native-only witness intentionally changes the selftest source inventory, without changing production shader/render paths. The earlier 301-file hash check remains evidence of the pre-witness baseline, not a claim that every current file is unchanged.

Next deliverable: finish the native fixture checks using the retained build cache, establish software WebGPU readback, then run the unchanged semantic checks for all four regenerated inputs and the canonical production rendering witnesses.

The first instrumented stroke run found automatic planar-sheet classification: production converted the flat fixture's pen to a positive 0.5 world radius. Those diagonal images are diagnostic, not an ordinary-pen match. The revised source varies endpoint depth by 1 mm; actual GPU readback confirms sheet=false, radius=0, thickness=1 and opaque black. The corrected eight-case run is pending. DirectComposition-off did not fix the browser stall; the alternative WARP browser probe stopped at its adapter-identity guard because Chrome still selected SwiftShader. WSL2 and its kernel are already installed, but there is no distribution; a fresh task-owned Linux environment is being assessed. No display group or regenerated fixture browser gate is accepted.

Corrected ordinary-pen witness: all eight scenarios pass rendering and actual GPU-input assertions (stroke-ordinary-run.json; 45.845 s, peak commit 719,409,152 bytes). Eight representative production/course images individually inspected; coverage differences persist. The official Ubuntu image checksum passed, but task-specific WSL2 import failed with HCS_E_HYPERV_NOT_INSTALLED: virtualization is unavailable. No reboot or firmware/global configuration change was attempted. Windows native work continues; browser acceptance remains blocked.

Windows WARP canvas acceptance now passes: verified adapter LUID, actual Microsoft/warp fallback identity, all 16 offscreen pixels and presented sRGB pixels [124,170,203,255] (software-warp-check.resources.json; 1.795 s, 422,383,616 bytes). All16 corrected stroke images individually inspected. Actual endpoint startup remains unaccepted: a fresh browser also exhausts the aggregate 2 GiB cap while eagerly compiling finite pipelines (fixture-browser-warp-comparison.resources.json exit134,15.74 s); no fixture callback ran. Production ordinary-ribbon integration and lazy pipeline allocation are in progress. Maintained docs prebuild passed123.862 s/1,889,464,320 bytes; cached GNU-find scan timed out, its native NTFS replacement passed seven mock checks and awaits actual rebuild/no-op after authoring changes.

Ordinary-line extraction: eight unselected production/course pairs match all pixels (stroke-integrated-run.json;85.637 s,792,297,472 bytes);16 PNGs match already individually inspected production frames. Finite variant/pixel-query checks pass on explicit CPU/WARP (finite-sample-warp.json,finite-pixels-warp.json). The unchanged round-cap test fails at its flat near-camera-cut assertion. A separate exact-source production witness confirms BOTH original production and course draw the same erroneous black cap pixel. Production now marks camera-created endpoints flat using its existing coverage, and the course extracts that same revised shader; post-fix checks are pending. No canonical assertion was altered. Ordinary selection was also moved entirely to production flags, removing its legacy duplicate underlay; source review passes, GPU checks pending.

2026-10-20 resumed: exact final replay contains666files. Current84016b80 ink-specific clipping shader compiles. Seven focused native CPU/WARP tests pass: coplanar owner/full width, sloping section, fresh placed/mirrored/far-rebased Line picks/source/archive preservation, camera near-cut, selection/rebase, mixed-family visibility and opacity. Evidence: ribbon-final-{section-owner,section-sloping,placement-pick,near-cut,selection,visibility,opacity}.json. Actual line-ROI pixels recover0?1348 in maintained and course renderers; all four clipped before/after images individually inspected (section-image-inspection.json). This diagnoses clipping, not full scene parity: plane details differ. Current-shader eight-pair regression and browser runtime rebuild are next. All five complete display groups remain open.

Current84016b80 ordinary8/8pairedregressionPASS;16PNGs match individually inspected prior renders. Actual-camera cloud baseline then completed8cases in12.314s/609828864bytes: DPR1fourpairs exact;DPR2course radius0.5?production with3736/3813changedpixels (EDL0/.25). Fouruniqueimagesinspected. Cloud extraction must preserve real shader, strictdepth,worldclipping and separate pageownership. Canonical immutable ribbon web snapshot WASMbuildPASS;optimizedTrunkbuild running. New cloud integration will have separate validation, and final release remains open.

2026-10-20 memory authorization: user requested more memory. Guard now accepts an explicit per-job MemoryLimitMiB override; next runs use4096MiB, one job and two CPUs. Regression confirms default2GiB, requested4GiB manifest, timeout and child cleanup. Retrying the unchanged optimized ribbon browser snapshot at4GiB isolates the previous memory cap from the newly authored cloud/lazy changes.

Current accepted subchecks (2026-10-20): ordinary lines8/8 matched images and7/7 focused semantic checks; cloud splats8/8 DPR/sample/EDL pairs,3/3 new ownership/depth/source tests and7/8 existing cloud regression filters. The failing cloud/Line visibility case is retained: depth and cleared primitive IDs are correct but hidden red ink remains visible. Two unchanged legacy-stroke cache/pick tests pass. Browser trace shows render-pipeline creations fall45?22, but readiness and all4 fixture callbacks remain unaccepted. Full display groups remain0/5; B2,B3,C totals remain open.

Memory follow-up: the integrated browser snapshot still misses readiness after 180 seconds at 4 GiB. A same-source, same-settings 6 GiB run is now in progress. Native cloud image parity remains 8/8; browser readiness and fixture acceptance remain open.


Latest source sync (2026-10-20): all16 GitHub checkouts updated, root95a14e08/sessiona391b104. Complete backup: C:/Users/Petras/Downloads/viewer-course-source-sync-20261020. All25,829 tutorial files were byte-identical at the pull boundary. Windows Bash hook merge and LF policy retain cross-platform launchers. No commits or pushes made.

Latest Matrix integration remains in existing lesson22.6/final checkpoint: exact composed production shader, component packing, original identity, whole-object picks and independent plate-less title. Native build passes37.482s at2,118,852,608bytes. Packing/archive/provenance/backward compatibility checks pass; GPU digit assertion is under investigation. Eight historical kernel-bearing locks receive only the new already-locked getrandom0.2.17 dependency edge; package versions stay fixed.

The6GiB browser attempt finished without reaching readiness (peak5,774,598,144bytes). Actual production cloud/Line witness confirms hidden-ink regression; shared six-line visibility guard and exact shader extracts updated, after-fix verification underway. Full stage counters remain unchanged; no full display group/final gate accepted.


Matrix native acceptance update (2026-10-20): all five new Matrix tests pass (plus two existing normal-matrix tests), and all four selected/unselected one/four-sample images were inspected. Actual digits, empty zero, color interpolation, object picks, sample return, GUID/archive placement and separate plate-less title are checked. Shared cloud/Line correction passes the canonical regression and12/12 actual production/course focused color controls; remaining mesh-border differences prevent whole-image parity claims. Existing finite shader recipes and sloping-point color/pick regression pass after fallback-call consolidation. Browser startup benefit remains unmeasured.

2026-10-20 integrated verification: native B1/B3 + dependency detachment + frame scheduling compile together (37.284 s, 2,155,061,248-byte peak). Three scheduler tests pass; normal dependency tree excludes session_viewer. All 18 actual production/course LOD comparisons pass on the original 2,000,064-point cloud. Mixed vector GPU check progressed beyond initial head/source probes but timed out in a later GPU submission; trace refinement pending. Pinned browser 1x/4x startup still unaccepted; counters remain honest full-group acceptance totals.

2026-10-20 further verification: compact Matrix3/3 and ordinary Plane4/4 focused tests pass; hidden-range actualGPU test passes1x/4x without uploads or source changes. Both existing actual2M cloud GPU filters pass; all18actualproduction LOD pairs pass. Integrated WASM compiles after a Send-safe active-runtime completion callback correction. First4x mixed scene requires23.7s joined/head plus13.5s sheet deferred first-use work; later stages~1ms. Isolated staged run passes unchanged pixel/pick/sample-return assertions, but is diagnostic rather than unsplit acceptance. Native optimized-compiler counterexperiment exhausts6GiB, so retain normal compiler and useexplicitbounded60s software readback override (defaults unchanged). Actualpaired Plane case exposes zero-width printed-fill depth divergence, now a concrete reconciliation task. All full-group/final counters remain open.

## 2026-10-23 integrated boundary

- [x] Compile the current production glyph/mesh/BRep, printed-alpha, outline-policy and empty-pass MSAA source with all native diagnostic examples and library tests.
- [x] Pass all three printed-fill regressions in the full suite, including real shared instances, composed alpha, sample changes, exact owner queries and independent cap-token storage.
- [x] Pass all eight actual-production Point frame pairs at DPR 1/2, one/four samples and both selection states.
- [x] Pass all four unselected production mesh frame pairs after production edge integration.
- [x] Pass the unchanged mixed joined/vector/sheet/Matrix regression with the explicit 60-second software-GPU wait budget.
- [ ] Finish the 1,059-test native boundary run and resolve its observed source/instrumentation failures; the run is not accepted yet.
- [ ] Rerun selected production mesh frames after removing the obsolete always-on black selection contour.
- [ ] Refresh the final endpoint's stale current-source inventory at maintained-source freeze; the prepared inventory contains 312 viewer, 109 kernel and 12 Wood files.
- [ ] Verify chapter-specific visible feature evidence for the 19 chapters without an accepted image record. Historical captures remain candidates, not current proof.

All original checkpoint history remains available. Final-checkpoint production policies and validation fixtures must identify the active production lanes; the frozen canonical browser callbacks remain unchanged.

## 2026-10-23 native reconciliation

- [x] Complete the integrated native suite: 1,030 passed, 29 failed, no GPU timeouts or memory-limit failures (1,358 seconds; 1.49 GB peak job commit).
- [x] Pass all eight actual-production mesh image comparisons, including selected meshes, DPR 1/2 and samples 1/4: zero changed RGBA pixels.
- [x] Pass the first focused repair batch: 5/7 tests, including shared-frame picking, mesh display/archive, cloud controls, Plane ownership and cleared metadata.
- [x] Diagnose altered grid fixtures as Windows CRLF conversion; restore both exact recorded hashes and exclude PPM assets from text conversion.
- [x] Correct integer picking pass order so foreground Point glyphs follow mesh-edge ink, matching colour-pass order.
- [x] Integrate source-address fixtures using the actual production curve walk and stable translated owner addresses.
- [ ] Verify the next focused batch, including explicit outline enablement and the refreshed current-source inventory.
- [ ] Finish the remaining native failures before rerunning the full suite. Focused fixes are not a new full-suite pass.

Evidence: `target/course-windows-20261018/integrated-full-native-tests.json`, `full-native-failure-summary.json`, `pick-priority-native-tests.json`, and `mesh-production-pairs-outline-fixed.json`.

### Repair verification boundary

Of the original 29 failing tests, 28 have now passed in focused runs after correction; the remaining near-clipped headed-ribbon fixture is being checked against actual production terminal/filter semantics. This is not a complete new 1,059-test suite pass. Newly introduced regressions must still be caught by the final full run.

- [x] Windows grid: two actual maintained/course pairs, 256/512 physical pixels, same device/queue, zero changed RGBA pixels. Preserve earlier Linux references separately.
- [x] Bounded contacts: actual production joined outline walk, contact sentinel and packed normal; complete perimeter pixels and original picks match across both projections, four angles, mirrored placements and both densities.
- [x] Foreground glyph/frame picking, source-address translation, actual mesh/glyph ABI checks, retained generic range fixtures, printed whole-object queries and original highlights.
- [x] Camera-lens adapter respects normalized perspective matrices; the existing shifted-panel sheet pen check passes.
- [ ] Finish the terminal fixture check, refresh local docs/visible browser, and run the final integrated native suite.

The current native source inventory includes 312 viewer files, 109 kernel files and 12 wood files. It hashes actual working-tree bytes; Git heads are baseline identifiers, not a clean-tree assertion.

Latest verification: all 29 original failing tests have now passed in focused runs. The final headed-terminal check passed in 51.702 seconds. A complete new integrated 1,059-test run is still pending. Fair performance baseline before-v3 includes every final feature and repair, with only the 13 scheduling edits omitted; unrelated source bytes are identical. Local docs refresh is the next heavy job.


2026-10-23 resumed browser boundary: current optimized endpoint build PASS (246.781 s, peak 1,367,064,576 bytes). Latest strict docs build PASS, 1,575 pages / 399 assets; all 26 route/layout checks PASS. Current comparison browser acceptance is running against windows-integrated; earlier focused 29 native repairs pass but full 1,059-test rerun remains pending. Remaining counts: 4 fixture callbacks, 5 measurement categories, 4 route/live combinations, 19 chapter feature evidence sets, and the full checkpoint/release gates. These counts overlap and are not a session estimate.


2026-10-23 current acceptance correction: the current production-layout endpoint passes file-read, document phases and recovery callbacks, but the recursive historical stroke callback rejects its shared buffer layout. Preserve its assertion and execute canonical old-checkpoint contracts with their own appropriate source; do not fake 44-byte telemetry. Windows capture host ports and a real streamed-sheet ingress witness are authored, pending execution. All 26 current docs route/layout captures were individually inspected. The full native rerun is still running; its grid fixture failed because root launched from the staging folder rather than viewer root. Preserve this run and rerun from the declared fixture directory. Current dated source inventory preserves the old snapshot and maps the six added maintained files; source mapping is not feature acceptance.


2026-10-23 07:55 UTC: complete native run 1,055 PASS / 4 FAIL, 1,653.797 s, 1,491,730,432-byte peak. All four then PASS plus fresh source inventory in native-owner-sheet-focused.json (13.946 s, 750,432,256 bytes). Two relative-file failures came from staging cwd; two raw whole-owner ID assertions now explicitly verify active ID and resolved original Object without dropping size/distance/no-fake-subpart checks. A clean final full-suite rerun remains required. Actual SheetInit/SheetFields/SheetRows first whole-frame pair PASS, zero RGBA differences; the 32-case projection/placement/DPR/MSAA/selection batch is running. New native witness helper is validation-only. Final source fingerprint 0c6c0ee9, 713 files; the older browser bundle retains its honest previous source freeze.


## 2026-10-23 checkpoint recovery and ownership

- Gemini/Argo owns existing chapters 1?10; Codex owns 11?22, shared runtime, integration and validation. Preserve the 22-chapter structure and all 1,361 checkpoint IDs.
- Recovered 560 saved files after an accidental force sync caused by the older Windows launcher ignoring `--help`. Current changes were snapshotted before restoration; that reset checkout had no additional local file edits. The tested model is restored exactly: 713 files, SHA-256 fingerprint `a45eada2fbdbd940d5e85b50e7e5fe12ccef69e794abd71111264cce0368fe2c`. Native witness source matches saved canonical SHA-256 `ba5124762568cdcfe6a6cca79f7e74b501fd94237b4afd948ecb3087d341d580`.
- Pull launchers now handle help/unknown arguments before invoking Bash; local changes block implicit resets. Explicit `--force` is destructive. Source synchronization is complete; do not rerun it over tutorial edits.
- Recovery structure check PASS: 1,361 checkpoint reconstructions, 307 current viewer source files; 43 retained reference checkpoints and 351 archived final files. Prior exact-model native build PASS; focused native checks 12/12 and tooling 22/22 PASS.
- Actual Sheet ingress comparisons PASS 32/32, zero full-frame differences. Corrected ordinary-family witness requires fresh Point/mesh reruns before accepting their placement/depth evidence.
- Final full native suite, browser fixture contracts, five performance categories, four reload/grid combinations, 19 feature-evidence sets and five release gates remain open. Full-code checkpoint publication is authorized; unfinished gates must remain visible.


### Checkpoint progress ? 2026-10-23

- [x] Full-code Session checkpoint `b7cdd5dc` pushed to `codex/viewer-course-checkpoints-20261023`; GitHub viewer-check run39928419284 pending. Parent hook/pointer checkpoint follows after CI.
- [x] Gemini/Argo ownership confirmed: existing sections1?10 in `session/session_viewer/docs/journey`; its concurrent prose/redraft files remain untouched. Codex chapters11?22:59 authored lessons,912 retained checkpoints, no missing snippet references.
- [x] Corrected actual production initial GPU translation: Point8/8 and Mesh8/8 full-RGBA pairs pass at the recorded placement.193.448s,830099456-byte peak; four representative images individually inspected. New native records and Point figures are embedded in lesson22.6.
- [x] Recovery docs build and visible browser check pass;111 reader lessons, latest Matrix code and decoded images verified.
- [x] Reconciled23 stale chapter11?22 metadata statements, cloud extraction/LOD evidence, current Scene defaults and native reload/grid implementation status.
- [x] Corrected transformed Plane16/16 full-RGBA comparisons PASS337.04s,982036480-byte peak; four representative images inspected. Plain/occlusion/print controls remain open.
- [ ] Chapters11 and13?20 still need complete chapter-specific accepted visual sets:9 of our12 chapters. Existing partial native evidence does not close a whole chapter.
- [ ] Four fixture callbacks; five measured performance categories; four reload/grid browser combinations; final clean native/WASM/optimized acceptance; runnable-checkpoint verification and all five release gates remain.


### Native evidence checkpoint and ongoing full run

- [x] Parent checkpointa574827 pushed; no configured branch workflow runs. Session runtime CI atb7cdd5dc is green:558native tests pass,61GPU tests ignored, plus WASM/native-all-target checks.
- [x] Gemini/Argo has pushed ten section commits throughd4af0558 on the shared branch. Their files and commits are preserved. Runtime and tooling sources are unchanged from the green CI checkpoint.
- [x] Source-bound native records/figures now cover Point8/8,Mesh8/8,and transformed Plane16/16 at the declared poses. Complete arbitrary depth/occlusion or browser acceptance is not inferred.
- [x] Before/after performance sources prepared:710/713files, identical outside13declared scheduling edits; final fingerprinta45eada2fbdbd940d5e85b50e7e5fe12ccef69e794abd71111264cce0368fe2c. Measurements still pending.
- [x] Portable pull launcher safety:8isolated Bash/cmd cases pass (help,invalid arguments,dirty worktree,unpublished commit). Every case preserves HEAD and payload.
- [ ] Complete learner native suite running from correct fixture cwd:integrated-full-native-source-freeze,1800s/6GiB,twoCPUs,one test thread. No acceptance before completion.
- [ ] Fresh strict docs build and browser review are required after Gemini and current visual/prose edits; previous successful docs evidence remains bound to its earlier inputs.


### Clean current learner native suite - completed

- [x] All1,059learner native tests PASS;zero failures andzero ignored tests. Guard1468.642seconds,peak1,480,597,504bytes,twoCPUs,one test thread. Evidence:integrated-full-native-source-freeze.json/log. Runtime/model fingerprinta45eada2fbdbd940d5e85b50e7e5fe12ccef69e794abd71111264cce0368fe2c,713files.
- [x] Allfive latest Matrix tests andthree compact Matrix owner tests pass in this full run. Previously failing fixture/owner checks also pass in the complete run.
- [ ] Combined docs build is running after Gemini sections1-10 and current11-22 visual/prose updates.
- [ ] Final release gates remain open: learner WASM/optimized rebuild,full browser/feature/performance/reload and runnable-checkpoint acceptance are not implied by the native pass.


### Current source freeze and combined browser review

- [x] Current 713-file a45 source passes native 1059/1059, locked WASM (25.384s), and optimized browser build (190.879s). No native failures remain.
- [x] Combined Gemini and Codex docs build passes. Prior 26 route/layout captures were individually reviewed; the subsequent shared SVG parser fix passes five focused tests and a fresh build. Visible Chrome confirms the complete first diagram, current Matrix, decoded Point/Plane images and linked evidence records.
- [x] The EXACT original 2223-byte covered Plane input passes its retained case with zero RGBA differences (6.955s); distinct generated controls remain separate.
- [ ] Performance baseline optimized build running, followed by paired measured categories. Four historical fixture callbacks, four reload/grid combinations, nine owned chapter visual sets, remaining display controls and full runnable-checkpoint/release acceptance remain open.
