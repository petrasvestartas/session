# Performance ledger

Measured with `examples/bench_frame.rs` (median frame, 60 frames per leg, 1400 x 900, native
Vulkan through the same tree the page runs; `VIEWER_ADAPTER` picks the GPU). "before" = the
face-identity design (commit 6ec6f1fc), "after" = the depth-buffer rule (commit 0b571aad).
Both binaries carry the `VIEWER_ADAPTER` knob, so each leg ran on the adapter it names -
`Intel(R) Graphics (RPL-S)` and `NVIDIA GeForce RTX 4080 Laptop GPU`, both Vulkan, confirmed
from the `adapter:` line of the matching `selftest` build. Scenes from the R2 bucket. Before
and after ran back to back per scene per GPU on an otherwise idle desktop. Date: 2026-09-08.

| scene | GPU | before still | before moving | after still | after moving |
|---|---|---|---|---|---|
| view_mixed | Intel RPL-S | 37.51 ms | 50.41 ms | 34.46 ms | 46.14 ms |
| view_mixed | RTX 4080 | 2.60 ms | 3.02 ms | 1.91 ms | 2.36 ms |
| view_meshes | Intel RPL-S | 139.58 ms | 142.84 ms | 35.95 ms | 36.06 ms |
| view_meshes | RTX 4080 | 4.06 ms | 3.20 ms | 1.17 ms | 1.42 ms |
| view_lines | Intel RPL-S | 74.63 ms | 75.16 ms | 69.81 ms | 70.05 ms |
| view_lines | RTX 4080 | 4.32 ms | 3.28 ms | 3.70 ms | 3.15 ms |

Every leg is at least as fast after as before; view_meshes is the one that moves, from 139.58
to 35.95 ms still on the iGPU, because the face pass no longer builds or writes face tokens.

Browser heap (Chrome, `?perf=1`, after every file arrived): not measured.

Ink fragment cost is now at most five depth texture reads per sample and no storage reads.
The face pass writes one colour target; there is no compute pass and no face-token attachment
(the previous design's `Rg16Uint` token target is 4 B per texel: 1400 x 900 x 4 samples =
19.2 MiB at this size with 4x MSAA).

Rules for this file: every number is measured on the day it is written, with the command that
produced it; a number that was not re-measured after a change is deleted, not carried over.
