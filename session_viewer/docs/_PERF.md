# Performance ledger

Measured with `examples/bench_frame.rs` (median frame, 60 frames per leg, 1400 x 900, Intel iGPU,
native Vulkan through the same tree the page runs). "base" = the tree before the 2026-09-03
ground refactor (commit 10f527c1), "new" = after it. Scenes from the R2 bucket.

| scene | base still | base moving | new still | new moving |
|---|---|---|---|---|
| view_lines (5 sheets) | 81.8 ms | 77.9 ms | 48.1 ms | 48.0 ms |
| view_lines_rotated | 21.4 ms | 21.5 ms | 21.0 ms | 21.1 ms |
| view_meshes | 16.8 ms | 16.6 ms | 17.1 ms | 16.7 ms |
| view_mixed | 10.9 ms | 26.3 ms | 10.9 ms | 22.9 ms |

Browser (Chrome, WebGPU, `?perf=1`, the same laptop), heap after every file has arrived:

| scene | base heap | new heap | note |
|---|---|---|---|
| view_pointclouds | 1168 MB | 264 MB | four scans and the 14 M cloud stream through their octrees instead of decoding whole |
| view_mixed | - | 284 MB | |

Load: view_pointclouds shows its first cloud in under 2 s and every file in about 6 s; the
431 MB cloud is a 250 k-point prefix under the 6 M page budget (`?points=` raises it).

Publish turnaround (`bash/view_live.sh`, one scene + one file, 2026-09-03): 2.46 s before,
1.16 s after (curl SigV4 instead of the aws CLI, verifies overlapped); the open page sees
the relay within its 100 ms tick.

A ribbon depth prepass (lines written to depth before the colour pass, so coincident lines
resolve by depth) was measured and rejected: view_mixed still 10.9 -> 16.1 ms, moving 22.9 -> 27.4 ms.

Rules for this file: every number is measured on the day it is written, with the command that
produced it; a number that was not re-measured after a change is deleted, not carried over.
