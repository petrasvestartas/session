# Measurements and verification scope

## Which version do these numbers describe?

The loading, memory and latency measurements below were collected on 8 September for the earlier production checkpoint `210120d0…`. They remain useful evidence for those specific changes. They are **not measurements of the later finite-triangle visibility pass**. The current rendering checks are recorded separately below; do not label an old timing table “current FPS.”

## Current boundary and selection checks

The final renderer passes 89 native tests including explicit GPU tests. The integrated image checks pass all 54 hidden-line cases, all 21 covered-floor camera cases, 40 joined-stroke assertions and 40 selected-overlap assertions. The selected-overlap fixture retains all 69,820 reference yellow core pixels; genuinely covered spans contain zero yellow. The 2800×1800, four-sample floor check also passes with the adaptive eight-pixel tile grid.

The finite-triangle counterexample retains all 766 visible core samples when a narrow neighboring solid is introduced, and zero black pixels when that solid truly covers the edge. Ordinary silhouette drawing is disabled in that particular measurement: a silhouette is legitimate black ink and must not be counted as a leaked hidden source edge. The ordinary-outline-on floor and selection checks remain separate.

The selected cone/cylinder and both source-text orientations were exercised in Chrome at DPR 1 and 2. The seven geometry families pass real object, original-edge, original-face and source-control gestures. Streamed source picking still finds the original ID beyond row six million after the state coordinator split.

The finite visibility fallback costs GPU work. A prototype comparison at 1400×900 measured about 15.5 ms per offscreen floor frame including readback after depth-bound filtering, versus 17.1 ms before that filter. This compares two versions of the new algorithm, **not the old production renderer**, and is not a browser navigation benchmark. Teapot timings varied too much to support an improvement claim. The final cache skips projection/binning when camera and geometry inputs are unchanged; static-frame reuse must not be presented as moving-camera performance.

Chrome on the recorded Ubuntu host and native Intel Vulkan are the tested environments. Firefox, Safari, physical monitor migration, mobile/touch hardware and physical display latency remain unverified. The source teapot is the existing Newell/GLUT Utah patch model; a 3ds Max export provenance is unverified.

## Earlier consolidation measurements

In the following historical tables, “current” means the 8 September consolidation checkpoint identified above. The later outline, source-face, source-text and finite-triangle additions require their own performance comparison.

## Measured browser results — 8 September 2026

`tests/scenes.cjs` ran all seven public scenes once in a fresh browser context and once by reloading that context: these are the **cold/warm** labels below, not proof of a cold CDN or OS cache. Both builds used visible Chrome 152 on Linux, a 1400×900 CSS canvas at DPR 1, and the same navigation sequence. The host has an Intel i9-13900HX; the browser reports an anonymized BrowserWebGpu adapter. Chrome's Vulkan-enabling launch switches belong to this Linux test setup, not to application requirements. The controlled baseline reconstructs viewer `8ca5e78` and kernel `a59c412`, preserving the user's initial camera change, changing only the incompatible adapter preference, and adding read-only inspection. Its original untracked dependency lock was unavailable: the reconstruction reuses the resolved dependency versions and records the pruned lock. This is a controlled source comparison, not a recovered historical binary.

Raw evidence lives under `/tmp/viewer-astra-y8kd1hog/`: `controlled-baseline/provenance.json` records source, patches and build hashes; `scenes-before/` and `scenes-final/` contain all 28 captures; `scene-revisions.json` records 36 successful source HEAD responses with ETags and lengths. Their latest Last-Modified is 7 September 2026 23:42:25 UTC, before these runs. The current seven-scene captures precede the final bounded metadata-window optimization described below. The final source freeze and subsequent verification are recorded in `docs/reconstruction/baseline.json` and `docs/16-verification.md`.

Times are seconds, **baseline → current**. “First geometry” is the navigation-clock rAF observation of the first inspected color submission containing an object; it is not GPU completion or measured presentation. “Settled” includes the harness's 200 ms readiness polling and deliberate 700 ms settling delay after the scene-posted signal.

| Scene (`view_` prefix omitted) | First geometry, cold | First geometry, warm | Settled, cold | Settled, warm |
|---|---:|---:|---:|---:|
| lines | 2.05 → 2.06 | 1.81 → 2.10 | 20.31 → 21.84 | 20.58 → 20.93 |
| lines_rotated | 2.15 → 1.97 | 1.78 → 1.78 | 7.41 → 7.22 | 6.71 → 6.97 |
| meshes | 1.20 → 1.27 | 1.02 → 1.04 | 7.24 → 7.28 | 7.55 → 6.74 |
| mixed | 0.71 → 0.86 | 0.68 → 0.68 | 7.12 → 7.66 | 7.22 → 7.30 |
| mixed_solids | 1.02 → 1.14 | 0.59 → 0.77 | 1.74 → 1.94 | 1.39 → 1.60 |
| pointclouds | 3.76 → 5.28 | 3.93 → 5.01 | 23.23 → 31.88 | 23.79 → 29.04 |
| live | 1.03 → 1.00 | 0.57 → 0.71 | 1.75 → 1.73 | 1.39 → 1.61 |

The following sums use the actual `loaded` and `appended` log messages for the six scenes other than `pointclouds`. They are logged phases, not a complete partition of elapsed time: streamed range fetch/parse work, browser scheduling and setup are not all included. Identical source scenes do not imply identical generated geometry or coverage: `mixed` and `mixed_solids` each gained five generated vertices, and `lines` restored four-sample coverage.

| Logged phase / observation | Cold baseline → current | Warm baseline → current |
|---|---:|---:|
| Fetch sum | 27.735 → 27.871 s | 27.804 → 27.042 s |
| Parse sum | 5.440 → 6.598 s | 5.414 → 6.074 s |
| Walk sum | 2.507 → 1.985 s | 2.374 → 1.864 s |
| Upload sum | 0.285 → 0.310 s | 0.260 → 0.302 s |
| Settled sum | 45.570 → 47.659 s | 44.843 → 45.151 s |
| Long tasks: count / summed duration | 37 / 8.194 → 38 / 8.970 s | 40 / 8.297 → 44 / 8.804 s |

The observed walk reduction is 20.8% cold and 21.5% warm. These runs do **not** establish a broad loading or frame-time improvement: parse time and retained memory increased, and total settled time did not improve. Long tasks cover loading through the end of scripted navigation. Across all 28 runs, navigation rAF median was 16.7 ms and p95 rounded to 16.7–16.8 ms; every final 1.2 s idle window submitted zero additional frames. rAF cadence is a main-thread scheduling proxy, not a GPU timestamp measurement or proof of a 16.7 ms GPU-frame budget. One cold and one warm run per scene do not establish statistical significance.

Current submitted inventories below come from the inspection snapshot, after navigation. These are actual uploaded rows and recorded draw calls, not source file sizes or theoretical batching counts. Baseline draw-call instrumentation was unavailable, so no draw-call reduction is claimed.

| Scene | Objects | Triangle vertices | Pipe segments | Ribbon segments | Resident cloud points | Draw calls |
|---|---:|---:|---:|---:|---:|---:|
| lines | 744,035 | 4,206,207 | 0 | 1,393,487 | 0 | 4 |
| lines_rotated | 155,465 | 1,585,686 | 36 | 345,602 | 0 | 8 |
| meshes | 4 | 682,513 | 207,962 | 0 | 0 | 5 |
| mixed | 107,186 | 691,468 | 8,528 | 282,039 | 741,989 | 9 |
| mixed_solids | 14 | 9,725 | 680 | 304 | 0 | 6 |
| pointclouds | 10 | 0 | 0 | 0 | 10,781,672 | 3 |
| live | 237 | 8,644 | 12,833 | 0 | 0 | 5 |

Memory is MiB after navigation, with matching cold/warm capacity observations. **WASM capacity, known source CPU payload, and GPU allocation are different measurements; do not add WASM capacity to its owned source payload.** The source payload column is available only in the current build and excludes private nested caches, allocator/map overhead and transient staging. GPU buffers are owned capacities; textures are estimated payloads with the exclusions in “Allocation and lifecycle boundaries.”

| Scene | WASM baseline → current | Current known source CPU payload | GPU buffers baseline → current | GPU textures baseline → current |
|---|---:|---:|---:|---:|
| lines | 559.4 → 1197.4 | 434.1 | 396.8 → 403.1 | 4.8 → 38.5 |
| lines_rotated | 214.6 → 379.2 | 136.8 | 144.8 → 146.5 | 38.5 → 38.5 |
| meshes | 465.1 → 469.8 | 274.7 | 55.9 → 56.7 | 38.5 → 38.5 |
| mixed | 304.0 → 339.6 | 121.0 | 99.5 → 101.2 | 48.1 → 48.1 |
| mixed_solids | 11.4 → 14.8 | 0.1 | 1.3 → 1.3 | 38.5 → 38.5 |
| pointclouds | 261.7 → 248.9 | 158.7 | 204.3 → 204.3 | 14.4 → 14.4 |
| live | 8.8 → 12.3 | 2.8 | 2.1 → 2.1 | 38.5 → 38.5 |

The lines scene's 559 → 1197 MiB WASM-capacity increase is a material tradeoff of retaining source geometry for F10 and the current allocation path, not a memory optimization; the 434 MiB known live source payload does not account for every byte of linear-memory growth. Its larger texture estimate reflects restored four-sample outline coverage. Default `pointclouds` resident rows differ (10,431,672 baseline versus 10,781,672 current) because existing asynchronous budget timing changes the loaded remainder. Its default frame and memory observations are therefore **not a same-geometry performance comparison**. The baseline also recorded one HTTP 416 per cloud run; current runs recorded none. Static-server live-reload/WebSocket and favicon noise are excluded by the harness and are not renderer validation results.

`tests/streamed-controls.cjs` separately verifies the final metadata read window at DPR 1 and 2: one 16-byte color header plus one 64 KiB cached tail range locates all seven small LOD arrays and the ID header, replacing the serial header/body requests. Larger arrays keep the existing 64 MiB individual/128 MiB total bounds; skipped large geometry/ID fields never cause whole-payload reads. Exact exposed ETag validation remains. The fixture still queries all relevant source pages, selects original ID `0xfedcba98` beyond row six million, verifies occlusion across pages and the actual yellow source marker, and cancels delayed reads while retaining only 250,000 displayed points. The optional `?points=0` setting selects the minimum 250,000-point display prefix per streamed file; it does not restrict F10 source queries.


The final matched cloud benchmark (`points-controlled-before/` and `points-controlled-final/`) uses that same display-prefix setting and confirms **4,931,672 resident points in both versions**, including whole-file clouds. Cold settled time falls from **21.132 to 16.732 s (20.8%)**; warm falls from **20.771 to 15.740 s (24.2%)**. Observed first-geometry time falls from 3.685 to 2.633 s cold and 3.020 to 1.905 s warm. Source HTTP resource requests, including the manifest, fall from **118 to 51** in each cache condition (117 → 50 protobuf requests). WASM capacity rises slightly, 265.375 → 268.813 MiB; GPU buffer capacities are effectively unchanged at 95,778,728 → 95,779,024 bytes, with identical 15,120,016-byte texture estimates. The current known source CPU payload is 158.739 MiB; that measurement is unavailable for the baseline. Navigation rAF remains 16.7 ms median / 16.8 ms p95. This is an observed loading improvement at matched point density, not evidence of faster GPU frames. The baseline retains its one HTTP 416 per run; the current build has none.

`tests/picking-performance.cjs` verifies visible yellow pixels after ten trusted pointer-up inputs in each of seven geometry families. Current results are 70/70; baseline is 50/70 because lines and clouds never produced the required highlight. Current per-family p95 input-to-selected-color-submission is 32.3–33.3 ms; the next-rAF presentation proxy is 48.3–49.5 ms. Screenshot completion provides only a 134.1–150.3 ms upper bound including capture/RPC overhead, not photon timing. The 50 ms interaction target is met for submission and the next-rAF proxy in this fixture, not established for physical display presentation or network-backed exhaustive F10 queries. The five working baseline families have submission p95 31.6–33.2 ms, so these measurements demonstrate repaired selection coverage, not a convincing speed improvement.


The final default-density cloud run after the metadata-window change settles in **17.282 s cold / 17.131 s warm**, with all ten objects and 10,781,672 resident points, no recorded errors and zero idle frames. These are current-build observations; the original default run had a different display density. The matched native offscreen depth workload measures 8.0 → 8.5 ms median over 30 frames including readback (about 6% cost); it establishes a correctness tradeoff, not a browser GPU speedup.

## Publication, replacement and verification scope

The existing `view_put.sh out/scan.pb` root `view_` naming contract remains. `view_live.sh scene.toml scan.pb` preserves placements/styles, publishes verified immutable `pb/revisions/<sha256>.pb` data first, updates the stable `pb/view_live.pb` alias using server-side CopyObject, then publishes the mutable TOML and compatible YAML manifests. Unchanged content verifies and reuses its revision; browsers reuse decoded immutable geometry when only placements change. Mutable resources revalidate, bounded failures retain the last valid scene, and later request generations supersede earlier ones. Local credentials stay in owned temporary curl configuration; no write secret enters browser assets. Notifications are sent only after verified publication.

Four maintained mock publication tests pass: ordering and placement preservation, failed upload, failed verification, and repeated unchanged revision without another geometry PUT. An authorized end-to-end probe used unique synthetic R2 keys and a 9,815-byte geometry fixture. Geometry upload/verification took 1,119 ms, stable alias copy/verification 1,234 ms, manifest upload/verification 1,219 ms: **3,572 ms total**. Initial cold browser readiness was 955 ms; publication start to the inspected new color submission was 4,113 ms, with 4 ms from final publication verification to that submission. Screenshot completion was a separate 44 ms upper bound. The final object count changed from seven to fourteen; these small-fixture timings do not predict large scan uploads.

A subsequent visible placement-only revision reused the same geometry: revision HEAD verification 516 ms plus manifest upload/verification 1,206 ms, **1,722 ms total, zero geometry PUT bytes and zero new browser protobuf requests**. With one-second polling, publication start to inspected submission was 2,429 ms; final publication verification to submission was 656 ms, and capture completion 710 ms. The normal-size fourteen-object capture was inspected. All four owned remote probe keys were removed and their absence verified; user live keys were untouched and no real notification was sent. Public CORS responses expose Content-Range, Content-Length, Accept-Ranges and ETag. Raw timing, request and cleanup evidence is outside the repository under the measurement directory's `publication/` subdirectory.

`tests/loading.cjs` passes malformed replacement, older delayed response after newer completion, and six alternating scene replacements. Repeated equal workloads retain exactly equal owned GPU capacity; WASM capacity remains 5,505,024 bytes across those cycles. `tests/lifecycle.cjs` passes text-input focus, mouse pointer cancellation, hidden-canvas idle and changed CSS size/DPR with unchanged physical framebuffer dimensions.

The verified toolchain is Rust/Cargo 1.97.1, viewer edition 2024, shared Rust edition 2021, Trunk 0.21.14, locked wgpu 29.0.4 / winit 0.30.13 / Glyphon 0.11.0 / TOML 0.8.23. Browser evidence uses Ubuntu 26.04.1, Chrome 152.0.7977.82 and Playwright 1.58.2; native GPU evidence identifies Intel RPL-S/Vulkan. The strict WASM/native Clippy gates and viewer-only formatting pass. The initial consolidation passed 68 native unit tests, five explicit GPU tests and 51 Rust / 59 C++ / 51 independent Python CAD tests. The completed boundary and annotation follow-ups increased these to 77 native, eight GPU and 53/61/53 shared tests, all passing; C++ includes eight additional preexisting cases. Depth verification passes 54 hidden-line views and 21 fully covered floor-ID census views with zero hidden leakage, preserves nine close-up red edges/seven dark vertices at 1× and 4×, and leaves 242,720 mixed close-up pixels unchanged.

The same-font white-on-black fixture was checked at 12/14/16/18/24 CSS px, DPR 1/1.25/2 and actual Chrome page zoom 100/125/150/200%; maximum line-width drift was 0.0026 CSS px and no glyph was missing. Native tests cover depth, clipping, motion, cache eviction after 4,096 distinct raster keys and release. Real PDF outline text retains source geometry: automatic 4× coverage restores partial edge pixels for normal-size words while word bounds stay within one pixel and ink mass changes by only 1–2%.

Firefox, Safari, physical monitor migration, mobile/touch hardware, physical display latency and a separate weak browser GPU are **unverified**. The native integrated adapter result does not identify the anonymized browser adapter. `scenes/view_drawings.toml` is absent from the authorized R2 inventory and returns 404; it is not counted as passed. The seven available YAML scenes and local PDF fixtures were tested. Requested TOML aliases currently return 404 and resolve through the maintained YAML fallback. No claim is made about inaccessible source scenes or unsupported environments.


## Earlier CAD rendering observations

These observations belong to the earlier 8 September boundary/annotation checkpoint, before the final finite-triangle pass. In the retained text below, “current” refers to that earlier checkpoint.

Final native CAD verification uses `examples/cad_fixture.rs`, `examples/check_cad_fixture.rs` and `tests/cad-quality.py`. Twelve 700×520 frames were inspected on Intel RPL-S/Vulkan with 4× MSAA: cylinders and spheres shade smoothly; source and rotated/mirrored/nonuniform instances retain correct facing; outer and inner hole boundaries remain attached; the C0 patch retains its sharp geometric fold and crease stroke. Fill-only and unlit pairs separate normals from linework. The sphere interior scanline has a maximum adjacent-channel change of 1/255, while its unlit counterpart is constant. Post-upload source mappings are identical for source/affine pairs: cylinder 147 segments/3 edges, sphere 36/1, hole 159/15. No missing-boundary fallback or GPU validation error occurred. C0 lighting contrast is modest under the default headlight; the separate one-sided-normal tests establish the discontinuity directly.

The follow-up boundary/annotation checks use the updated public mixed scene with the teapot and fixed-plane text. The exact incidence audit covers 13 BReps, including the user’s mixed-scene solids and teapot: no missing f64/f32 segments or empty faces. The current suite passes 77 native unit tests and eight explicit GPU tests; browser source/edge/control gestures, labels/T and fixed-plane text pass at DPR 1/2. The public mixed scene reports the authored world-plane axes and teapot title, with no browser errors and zero idle redraws. Its observed cold/warm settled times are 11.061/9.701 seconds; these are single runs of the changed scene, not a matched loading improvement claim.

The gradient attachment has a measurable cost. Three interleaved native debug runs, each 120 offscreen frames including readback at 1400×900 and 4x MSAA on Intel RPL-S/Vulkan, give median-frame observations of 3.6–3.8 ms before versus 3.8 ms after for the unchanged box, and 4.0–4.2 ms before versus 4.2–4.6 ms after for the teapot. These are CPU wall-clock/readback measurements, not GPU timestamps or browser frame rates. The teapot’s geometry also changed as part of the boundary repair. The earlier broader consolidation measurements below retain their original scene revisions and scope.
