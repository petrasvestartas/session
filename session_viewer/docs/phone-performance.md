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
- Phone opacity defaults to 1; desktop stays 0.95. Remove automatic element dimming so explicit URL opacity is retained; browser acceptance pending.
- Completed: publish scenes up to 32 MiB with deterministic gzip, immutable compressed-byte revisions and decoded manifest size. The live floor now transfers 811,523 bytes (83.69% smaller); decoded SHA-256 stays unchanged. Chrome checks HTTP gzip and headerless fallback without any range probe. Cloudflare serves decoded bytes to clients without Accept-Encoding, so publisher verification explicitly requests gzip.
- Polling now compares actual response bytes while retaining ETags for conditional downloads. A headed regression changes strong/weak/missing validators across four successful polls: the old viewer replaced the same scene four times; the fix replaces it once. Native tests cover changed bytes under an unchanged validator and cache reset.
- Pending: first-frame pipeline preparation.
- Retain close-up creases, round bores and tree colours; verify deployment workflows.

## Solid-opacity GPU measurement

Native optimized selftest, 900×700, 20 frames, VIEWER_PROFILE=1 and VIEWER_GPU_TIMING=1; CPU frequency cap 5.2 GHz matches hardware maximum. Compare the same GPU, not cargo's NVIDIA preference against the native executable's integrated default:

| GPU | Ink at opacity 0.95 | Ink at opacity 1 | Reduction |
| --- | ---: | ---: | ---: |
| NVIDIA RTX 4080 Laptop | 8.458 ms | 3.445 ms | 59.3% |
| Intel RPL-S integrated | 77.695 ms | 48.528 ms | 37.5% |

Profiled native geometry walking is about 248 ms. The selftest's outer walk timing includes its separate profiling walk, so it is not the scene walk measurement. The requested ~200 ms desktop goal is not established yet.
