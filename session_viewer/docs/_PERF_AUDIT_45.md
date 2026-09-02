# Performance and memory audit of the end-of-45 viewer — ranked synthesis

Measured on the end-of-45 tree (`base45`), native harness, Intel iGPU (RPL-S) via
`PowerPreference::LowPower` unless stated; every number taken at least twice. Full evidence:
`audit_{cpu-memory,gpu-memory,frame-cpu,gpu-frame,load-path}.md`, census in `census_*.md`.
Verification: 5 skeptic passes completed (gpu-memory 1,2,3,5,9: all CONFIRMED with corrected
numbers); the remaining findings were cross-checked by hand against the code and against each
other (the same facts appear independently in 2-3 reports: the object mirror, the MSAA policy,
the instance re-upload, the continuous redraw).

Ranking is by cost on a weaker machine: a laptop iGPU at 2560x1440 DPR2 with 8 GB RAM.

## A. Fixed in the refactor (lesson 51 unless noted) — ranked

| # | finding | measured | fix | pixels |
|---|---|---|---|---|
| 1 | **Continuous redraw**: `State::render` requests the next frame unconditionally; a still scene renders at vsync forever. Idle `drawings` = 9-13 fps with the iGPU 100% busy. | still frame: bunny 6.7-7.1 ms, drawings_rotated 18.4, drawings 85-106 ms — every frame, forever | render on demand: `State.needs_frame`, set by input/Msg/stream/resize; `render()` never requests itself | none |
| 2 | **~1,034 B per object of CPU mirrors** (Gpu: instances 96 + objects_base 152 + base_f32 64 + object_bounds_world 56 + inside 1; Scene: objects 152 + bounds 28 + spacing 4 + order String + guid_to_row String). Only 3 of 744,040 rows have bounds. | drawings_rotated 153 MB, drawings 556 MB for ~210 MB of GPU geometry | one owner (`InstanceTable`): `rows: Vec<Instance>` + `translation: Vec<[f64;3]>` + SPARSE `bounded: Vec<BoundedRow>`; Scene's object columns become a per-upload delta; guids shared via `Rc<str>` (one string per object) | none |
| 3 | **MSAA 4x on the whole frame whenever ANY arena vertex exists** — pure PDF sheets included (the "sheets pay nothing" comment is false). 40% of a sheet frame; 4x colour + 4x depth targets scale with DPR². The 1x `msaa_color` texture is allocated anyway. | drawings_rotated 20.5 -> 11.7 ms at 1x; 7680x4320 4x: depth 524 + colour 540 MiB; 3840x2160: 131 + 135 MiB vs 36 at 1x; dead 1x texture 8-127 MiB | samples keyed on SOLID content (`arena.faces > 0 \|\| pipes > 0 \|\| spheres > 0`, never on vertex count), capped to 1x above 4.2 Mpx, `?msaa=` override; `Targets.msaa: Option` | only for pure sheets and for canvases > 4.2 Mpx (gate scenes unchanged) |
| 4 | **Whole instance table re-uploaded on every re-anchor (throttled 5/s) and on any inside-flag flip**; the CPU loop reads 192 B/row to change 12 B | drawings: 68 MiB per write; loop 30-72 ms + write 14-17 ms + submit 6-8 ms | translation lives in its own 16 B/row buffer (`translations: array<vec4<f32>>`, group 2 binding 1); re-anchor rewrites 12 MB not 68; flag flips write only the flipped rows | declared: 1-ulp vertex differences possible, goldens re-recorded with the reason |
| 5 | **Kernel `Session` retained after the walk for every file** (display_only defaults false); tree+graph+lookup are 52% of a sheet's session and are never read (no scene has xforms) | drawings_rotated docs 360 MB -> 0; bunny 17 MB; lion 24 MB | `display_only = true` on sheet and cloud items in the shipped manifests; for display_only files skip the tree+graph decode | none |
| 6 | **Load peak = bytes + proto + kernel coexisting**; the raw file bytes outlive the decode | big sheet: 566 MB peak on a 117 MB file (4.85x) | pass the bytes by value and drop them right after `prost` decode; `reserve()` the walk tables from known counts (up to 2x slack today) | none |
| 7 | **Arena grows EXACT** — every appended file reallocates and copies the whole vbo/vids/ibo; the ink lanes double | drawings per-file upload 65-525 ms exact vs 9-101 ms with growth | one `GrowBuf` policy for every lane: grow to `max(need, cap * 3/2)`; streamed clouds keep the exact reserve (count known) | none |
| 8 | **Streaming colours fetched whole** while coords are sliced | lidar_14m: 148 MB transient (95 MB raw + 53 MB Vec) | slice the colour run like the coords; carry the varint tail between slices | none (advisory scenes) |
| 9 | `splat_records` built every frame BEFORE the static-skip test; 14 allocs / 65 KB per still frame discarded | lion still: 100% of the work discarded | test `is_current` first; reuse the record Vec | none |
| 10 | **2D dispatch rounds the last row to a full 4096-wide row** | lion: 524,288 threads for 341,989 points (+53%), each passing two barriers | `gy = g.div_ceil(4096); gx = g.div_ceil(gy)` | none |
| 11 | `time` uniform written every frame, bound by two pipelines, read by no shader; `instances_unused` bound to the splat compute (68 MiB on drawings) | 1 write + 1 bind group per frame; a 128 MiB binding limit reachable at 1.4 M objects | delete the time buffer/layout/group and the `@group(1)` line in triangle.wgsl; drop the splat binding | none |
| 12 | `std::env::var("BENCH_NO_MARKERS")` every frame in `encode_frame`, `VIEWER_THICKNESS` every frame natively; eye/ortho computed twice; clock read twice | < 0.001 ms — hygiene | `View` knobs read once; `FrameUniforms::write` computes eye/ortho once; one timestamp per frame | none |
| 13 | `reset_arena` / `Clear` never shrink GPU buffers or CPU mirrors | drawings: 408 MiB stays resident after Clear | `Gpu::release()` used by `Scene::clear`: 1-row placeholders again | none |
| 14 | `line_to_segment` builds two kernel `Point`s per line (4 String allocs) | 947k allocs on one sheet; 22 ms vs 3 ms | read coordinates through `Line`'s index accessor | none |
| 15 | Shaders compiled 2-4x (ribbon 4 modules), rebuilt on every MSAA flip | startup only | one `ShaderModule` per source in `Pipelines::new` | none |
| 16 | Harness blind spots: `bench_scene` prints `0 edges 0 markers 0 verts` (tables already dropped); `VIEWER_FRAMES` median hides the splatter (static skip); no GPU allocator report | 1.8 ms vs 31 ms on lion | print GPU-side counts; add `VIEWER_GPU_REPORT` (native allocator report per label); note the still-camera caveat in the docs | none |

## B. Known and deliberately left (documented in ARCHITECTURE.md, not fixed here)

- Kernel decode is 75-79% of native load time (prost 50%); `Mesh` HashMap-of-HashMaps costs 61 B/vertex; `SpatialOctree` build (HashSet accept test) is 80% of a cloud walk. Kernel work, three languages.
- Solid ink drawn twice (depth prepass + colour) = 31% of the flat bunny frame: the prepass is what keeps the AA rim free of flecks; the alpha-to-coverage alternative changes pixels.
- Sheet ribbons are 80% of a sheet frame at 4x MSAA (fixed by A3 at 1x); the vertex stage fetches the instance row five times per vertex — a `step_mode: Instance` table is a shader-side redesign.
- Compute splat: the colour pass re-projects every point (cost = the depth pass); per-thread linear record search. Both are `splat.wgsl` changes; the dispatch fix (A10) is the Rust-side part.
- `blend: ALPHA_BLENDING` on the opaque triangle pipeline (~1 ms on sheets) — dropping it changes translucent 3D faces.
- The background draw duplicates the pass clear (0.1 ms); deleting it changes the `draws` golden column.
- `GlyphPoint` carries 12 B the flat lane never reads; `stream.nrm` is 53 MiB of sentinel on lidar_14m.
- The harness measures the iGPU; the dGPU is 5-20x faster and inverts the Tubes/Flat verdict. Numbers under CPU load disagree by up to 2.3x — every measurement in a lesson states the load average.

## C. Behaviours the audit proved load-bearing (the refactor must keep them)

- The splat static skip (camera still -> no compute).
- Append-only lanes; deltas per file; `drop_rows` after upload.
- The solid-lane depth prepass before the blended colour pass (flecks otherwise).
- `GreaterEqual` on the solid ribbon/marker colour passes; both `VIEWER_NO_DEPTH` branches.
- The `draws` count semantics (`arena` counts 1 even when empty) — the goldens record them.
- Exact-fit reserve for streamed clouds (count known before the first byte).
- The 200 ms re-anchor throttle.
