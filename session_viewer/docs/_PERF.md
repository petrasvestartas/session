# Performance ledger

Measured with `examples/bench_frame.rs` (median frame, 60 frames per leg, 1400 x 900, native
Vulkan through the same tree the page runs; `VIEWER_ADAPTER` picks the GPU):

```
env VIEWER_ADAPTER=<Intel|NVIDIA> VIEWER_W=1400 VIEWER_H=900 BENCH_FRAMES=60 \
  target/x86_64-unknown-linux-gnu/release/examples/bench_frame <scene>.yaml
```

"before" = the weld design (commit 224a5861), "after" = today's tip with unwelded BRep faces,
BRep edge pipes and their tessellation-winding facing signs (commit ea8509c0). Both binaries
carry the `VIEWER_ADAPTER` knob, so each leg ran on the adapter it names, confirmed from the
`adapter:` line of a `selftest` run under each `VIEWER_ADAPTER`: `Intel(R) Graphics (RPL-S)
(IntegratedGpu, Vulkan)` and `NVIDIA GeForce RTX 4080 Laptop GPU (DiscreteGpu, Vulkan)`. Scenes
from the R2 bucket, in `$S/bench/pb/`. Date: 2026-09-08.

A first, single-shot pass (before and after each run once, back to back per scene per GPU, but
not interleaved with each other) put `view_mixed` on the RTX 4080 moving at 2.85 ms before
against 3.06 ms after, and 3.14 ms on a rerun - both above before - which is why the twelve
legs below were instead measured interleaved, before/after/before/after/before/after (B A B A
B A), three runs of each per scene per GPU on an otherwise idle desktop; the table takes the
median of the three runs on each side.

| scene | GPU | before still | before moving | after still | after moving |
|---|---|---|---|---|---|
| view_mixed | Intel RPL-S | 34.36 ms | 46.21 ms | 34.35 ms | 46.17 ms |
| view_mixed | RTX 4080 | 1.96 ms | 2.37 ms | 1.87 ms | 2.35 ms |
| view_meshes | Intel RPL-S | 37.37 ms | 37.57 ms | 37.31 ms | 37.57 ms |
| view_meshes | RTX 4080 | 1.51 ms | 1.49 ms | 1.17 ms | 1.13 ms |
| view_lines | Intel RPL-S | 70.62 ms | 70.23 ms | 70.28 ms | 70.28 ms |
| view_lines | RTX 4080 | 4.41 ms | 3.18 ms | 4.38 ms | 3.18 ms |

Every after median is at or below its before median, with one exception that is noise:
`view_lines` Intel moving is 70.28 ms after against 70.23 ms before (+0.05 ms), well inside
that leg's own before-side spread of 0.36 ms (70.17-70.53 ms across its three before runs).
`view_mixed` is the only scene with BReps (seven solids in one file of 13 objects); on the
RTX 4080, where the unwelded, per-face upload trades a welded index buffer for more vertices,
it is very slightly faster after, not slower (1.96 -> 1.87 ms still, 2.37 -> 2.35 ms moving);
on the Intel iGPU, which this scene is fill-rate- and CPU-walk-bound on elsewhere, it is flat
(34.36 -> 34.35 ms still, 46.21 -> 46.17 ms moving).

Browser heap (Chrome, `?perf=1`, after every file arrived): not measured.

Ink fragment cost, counted in `ink_visibility.wgsl` at commit ff8f046a: a stroke fragment makes
at most six depth texture reads per sample - its own texel, the two the planarity guard reads
outward, the two it reads inward when the outward pair is rejected, and one re-read of the
accepted neighbour - and a disc fragment at most seven: its own texel, the two the guard reads
on each of its two axes, and a re-read of each axis's neighbour. No storage reads.
The face pass writes one colour target; there is no compute pass and no face-token attachment
(the previous design's `Rg16Uint` token target is 4 B per texel: 1400 x 900 x 4 samples =
19.2 MiB at this size with 4x MSAA).

Rules for this file: every number is measured on the day it is written, with the command that
produced it; a number that was not re-measured after a change is deleted, not carried over.
