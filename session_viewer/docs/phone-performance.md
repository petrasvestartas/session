# Phone rendering and loading

The supplied CODEX_PROMPT.md references another machine's review and patch. They are absent here; this work verifies the local code and published timber-floor bytes independently.

## Load diagnostics

Type `Diagnostic Report` to download the current run, including successful loads. The error panel separately offers the latest and previous report. Reports record download decoded bytes and timing, manifest/decode/walk/upload timing, pipeline creation, first-frame submission/completion, live replacements, visibility/freeze/resume, resource transfer sizes (zero when CORS timing is unavailable), and the actual WebGPU adapter's vendor and architecture. Old failures beyond two hours no longer raise the previous-run banner.

Headed Chrome, NVIDIA Lovelace, real published 4,975,446-byte floor, local optimized viewer (2026-10-03):

| Setup | Navigation to geometry visible | Decode | Walk | Upload |
| --- | ---: | ---: | ---: | ---: |
| Desktop 1200×800 | 1063 ms | 37 ms | 300 ms | 15 ms |
| Phone CSS 412×915, DPR 2.625 capped to 2 | 1239 ms | 36 ms | 295 ms | 13 ms |
| Same phone dimensions, 6× CPU throttle | 3143 ms | 225 ms | 1815 ms | 99 ms |

These are desktop GPU emulations, not measurements of the user's Qualcomm phone or weak 5G connection. All three downloaded reports include every required phase, GPUAdapterInfo and a single live replacement after the next polling interval. This establishes that CPU slowdown alone does not reproduce the reported minutes here. The phone's new report will distinguish network delay, blocking geometry work and GPU completion.

Local checks: 500 native tests passed, 53 ignored; wasm check and optimized Trunk build passed; seven diagnostic unit tests passed. `tests/phone-load.cjs` runs the three headed cases and saves the actual latest reports.

## Remaining work

- Completed: hybrid depth anchors draw 100% in all six far-floor cases; 501 native tests pass, including existing close-up hidden-ink checks. Chrome passes 24 close-up views.
- Completed: phone opacity defaults to 1; desktop stays 0.95. Automatic element dimming is removed. Headed Chrome checks default and explicit opacity 1/0.6 on desktop, phone and throttled phone; all nine cases pass.
- Completed: publish scenes up to 32 MiB with deterministic gzip, immutable compressed-byte revisions and decoded manifest size. The live floor now transfers 811,523 bytes (83.69% smaller); decoded SHA-256 stays unchanged. Chrome checks HTTP gzip and headerless fallback without any range probe. Cloudflare serves decoded bytes to clients without Accept-Encoding, so publisher verification explicitly requests gzip.
- Polling now compares actual response bytes while retaining ETags for conditional downloads. A headed regression changes strong/weak/missing validators across four successful polls: the old viewer replaced the same scene four times; the fix replaces it once. Native tests cover changed bytes under an unchanged validator and cache reset.
- Completed locally: first-frame pipelines are prepared during the early download, using the capped canvas size and correct sample count. A delayed-download phone test proves preparation precedes decode, the opaque face pipeline compiles once, inactive Arctic stays cold, and explicit Arctic remains usable. Reduced-quality fallback waits for GPU completion of the first geometry frame.
- Published edge, opacity, diagnostics and compression fixes passed viewer-check, viewer-pages and the broader Session mini tests. Duplicate-reload and startup fixes are now deployed with viewer-check, viewer-pages and Session mini tests green.
- Completed and deployed: a stationary view continues until its asynchronous visibility-capacity report is consumed, so overflow grows/rebuilds the lists without needing another mouse gesture. It then stops drawing.
- Pending robustness: adjacent face ownership in stroke visibility, idle tap/selection preparation and ribbon variant reduction.
- CPU restructuring is conditional on actual phone diagnostics. The phone itself has not yet been measured here.

## Solid-opacity GPU measurement

Native optimized selftest, 900×700, 20 frames, VIEWER_PROFILE=1 and VIEWER_GPU_TIMING=1; CPU frequency cap 5.2 GHz matches hardware maximum. Compare the same GPU, not cargo's NVIDIA preference against the native executable's integrated default:

| GPU | Ink at opacity 0.95 | Ink at opacity 1 | Reduction |
| --- | ---: | ---: | ---: |
| NVIDIA RTX 4080 Laptop | 8.458 ms | 3.445 ms | 59.3% |
| Intel RPL-S integrated | 77.695 ms | 48.528 ms | 37.5% |

Profiled native geometry walking is about 248 ms. The selftest's outer walk timing includes its separate profiling walk, so it is not the scene walk measurement. The requested ~200 ms desktop goal is not established yet.

## First-frame preparation

The headed phone startup test reports 1.1 ms of pipeline-constructor preparation before scene decode; this is CPU constructor time, not a claim that all GPU compilation takes 1.1 ms. First geometry submission and GPU completion are recorded separately. Native checks pass 504 tests with 54 ignored; the GPU-only prewarm/cache test passes when run explicitly. WebAssembly and optimized Trunk builds pass. `tests/startup.cjs` exercises the delayed download and subsequent typed `Arctic On` command.

## Visibility capacity recovery

A 48-triangle overlapping fixture overflows the initial tile-reference pool. The GPU regression proves that pending reports request another frame, the pool grows until the full list fits, and the view returns to demand-driven drawing. Disabling all stroke readers releases the tables and cannot keep the view awake. A headed phone test verifies stationary frame counters after initial load, zoom, hide-lines/edges and restore/Fit; all four stop, with no GPU errors. The test dismisses the editable command-field caret before measuring an idle scene. Native checks pass 504 tests with 55 ignored, with this GPU case run explicitly; WebAssembly and optimized Trunk checks pass.

Public acceptance after overflow deployment also passes: headed phone frame counts settle at 11/45/53/72 across initial floor, zoom, hidden stroke readers and restored Fit. No GPU errors occur. The primary deployment and viewer-check are green; the broader test matrix remains in progress at this measurement.

## Navigation quality

A deterministic headed Chrome regression advances the frame clock during 48 real mouse-drag updates. This exercises the sustained-slow-navigation detector without pretending that desktop GPU emulation measures phone performance. The previous viewer permanently reduced a phone canvas from 824×1830 to 412×915 and left it there after release. Ordinary slow navigation now retains the chosen canvas resolution and MSAA; temporary ink/outline tiers still recover full detail at rest. Device-loss recovery retains its separate conservative fallback.

Headed Chrome checks now pass for the phone default (824×1830, 1× MSAA), explicit phone settings (618×1373, 4× MSAA), and desktop (1200×800, 4× MSAA). Each check proves the slow-navigation detector fires before asserting the dimensions and sample count remain unchanged. The WebAssembly check and optimized Trunk build pass; native tests pass 505 cases with 55 ignored.

A new real-floor CPU ray oracle checks six close-up views at desktop and portrait phone sizes, zoom scales 0.3/0.1 and grazing angles 3°/12°. All sampled visible edges draw, with no detected hidden-edge leaks. This supplements the six permanent far-view cases and the close-up plate cases; it does not establish coverage of an unspecified user camera view.

The 900×700 NVIDIA selftest (20 frames, solid opacity) remains comparable: previous median frame 4.4 ms / ink 3.445 ms; current median frame 4.6 ms / ink 3.588 ms. GPU allocation estimates and non-background pixel count are unchanged. This fixes a navigation quality policy; these runs do not demonstrate a rendering speedup.

## Off-screen stroke work

At close zoom the renderer still visited off-screen stroke vertices and calculated their sag, visibility and neighbouring joints before raster clipping discarded them. A conservative expanded screen box now rejects a stroke only when its full width and end-cap/filter expansion cannot reach the viewport. Thick world-width strokes and selected/ordinary pen widths retain their margin; no visibility tolerance, tessellation or pixel resolution changes.

NVIDIA/Intel are desktop measurements, not Qualcomm phone acceptance. On Intel RPL-S at the phone canvas size 824×1830, solid opacity, 1× MSAA and 20 frames:

| View | Previous median frame | Current median frame | Previous ink | Current ink |
| --- | ---: | ---: | ---: | ---: |
| Fitted floor | 31.4 ms | 29.9 ms | 26.088 ms | 25.384 ms |
| Zoom to 0.3× fit distance | 24.2 ms | 14.2 ms | 18.914 ms | 9.252 ms |

Both before/after PPMs are byte-identical and allocation estimates are unchanged. The close-view median frame improves about 41%; fitted-view change is small. This improves work during navigation; it does not establish a shorter initial phone load. A GPU regression keeps visible stroke fringes when the axis itself is outside each of the four viewport sides, using two wide pen sizes.

Verification: 506 native tests passed, 55 ignored; WebAssembly check and optimized Trunk build passed. Headed Chrome retained visible box edges in all 24 close-up cases and canvas/MSAA quality after slow drags in default phone, explicit phone and desktop cases, with no browser errors.

Deployed as `4f254643`; viewer-check, Pages and Session mini tests are green. Public headed Chrome navigation checks preserve all three quality configurations. The real published floor loads in 1149 ms desktop, 875 ms at phone dimensions, and 3076 ms with 6× CPU throttling, with one scene replacement per run; explicit opacity checks also pass. These use the desktop NVIDIA GPU and local network, not the user's phone or its connection.

The user confirmed the appearance is correct and that both loading and camera movement remain extremely slow on their phone. Actual current phone diagnostics are still needed to distinguish network, CPU conversion and GPU startup delays. Phase 3 CPU restructuring remains conditional on that evidence.

## Bound camera frames by GPU completion

The user confirmed faster loading but still reported extremely slow phone rotation. Browser redraws previously submitted camera frames independently of GPU completion. Headed Chrome using the Intel integrated GPU reproduced a queue of eight outstanding submissions during 180 right-drag movements at the phone canvas size, 824×1830, opacity 1 and MSAA 1.

The browser now waits for the preceding redraw batch, including egui uploads and pick work, before encoding another. Mouse and touch input continue updating the camera; completion wakes the latest state only if another redraw was requested. This also makes the existing drag-tier timing follow GPU throughput instead of the CPU submission rate. No canvas-resolution, antialiasing or edge-visibility settings change. Native synchronous offscreen rendering is unchanged.

| Actual Intel WebGPU queue measurement | Before | After |
| --- | ---: | ---: |
| Maximum outstanding submissions | 8 | 2 |
| Median submission-to-completion time | 60.9 ms | 14.9 ms |
| 95th percentile completion time | 155.1 ms | 39.4 ms |

Two submissions are the UI upload and scene within one redraw batch. The final camera matrix and named GPU allocation estimates match. These are actual GPU completion timings without artificial delay, measured on Intel gen-12lp; they do not establish performance on the user's Qualcomm phone.

The same deterministic regression fails on the old deployed viewer, which makes 108 GPU submissions while completion is held. The corrected build passes all four gestures. The deterministic headed regression separately holds completion promises. Mouse orbit, touch orbit, two-finger pan/pinch and touch cancellation preserve the final pose, resume without another input, retain the sharp canvas and stop drawing at idle. WebAssembly and optimized Trunk builds pass. Native tests pass 506 cases with 55 ignored when run serially; the initial parallel run crashed with SIGSEGV during GPU tests and is not counted as passing.

The existing sharpness regression passes for default phone, explicit phone and desktop; all 24 headed close-up views pass. Local load checks take 815 ms desktop, 829 ms phone dimensions and 2983 ms at 6× CPU throttle, with one live replacement and all explicit-opacity checks passing. Device loss still stops every GPU operation and recovers its saved report after reload. Its stale test selector was updated to the current “Download previous report” button.

Native 40-frame orbit measurements produce byte-identical final images and unchanged GPU allocations. The existing adaptive tier alternates full and plane visibility: before p50/p95 were 20.4/44.1 ms, after 40.0/44.6 ms, with tier-1 medians 20.0/17.3 ms. The overall medians reflect different tier occupancy; no native speed improvement is claimed for this browser-only scheduling change.

The maintained command-input check also passes immediate typing, named View commands, fresh right-click Move repeat, mouse orbit/jitter and phone navigation.
