# Hidden-line implementation and verification

2026-09-04. This is a verification record for the changes in the **main**
`session_viewer` worktree. It does not replace [_HIDDEN_LINE_TASK.md](_HIDDEN_LINE_TASK.md).

**Status: final acceptance remains pending.** The latest `/tmp/hl-bounds` candidate passes
all 42 original-scene census cases, 108 synthetic renders and lifecycle checks. Final
native/wasm checks, shader/layout tests, the gate and the main Trunk build pass. Performance
acceptance fails on the available software renderer, and browser runtime verification remains
blocked. Measured regressions are recorded below, not treated as passes.

The main source now contains a subsequent performance candidate under verification. It moves
the same angular predicate to a cached compute index filter, changes the face attachment to
`Rg16Uint`, and shares support-region/plane decisions between ink queries. The screenshots and
completed correctness results below still belong to `/tmp/hl-bounds`; they do not yet certify
these follow-up edits. TUBE scratch-depth and guarded back-shell prototypes remain isolated
under `/tmp` and have not been applied.

## Visibility rule

The physical face pass writes unbiased reverse-Z depth and an exact face token at each raster
sample. The subsequent ink pass samples these immutable attachments. Faces and ink have no
world-distance push/lift or rasterizer bias. The ink pass retains a read-only depth attachment
for sheet compositing; it does not sample a simultaneously writable depth attachment. The face
pass still clears depth and tokens in scenes containing only lines and points.

Each mesh edge has a variable-length list of actual supporting face tokens. Adjacent faces
support the stroke; other incident faces support only the corresponding round endpoint.
Markers retain their incident faces. A common object ID alone never grants visibility.
Separately authored line/polyline spans, sampled NURBS spans and free point glyphs are
associated with bounded physical triangles using original f64 geometry and placement
transforms. The NURBS walk and host sweep share one f64 sampler. These associations are built
within each file; cross-file coincident ink does not receive this explicit support guarantee.
They do not use a fixed absolute GPU depth tolerance. The new authored-ink fixture verifies
Point/NURBS coverage and hidden controls in 36 additional renders.

Connected coplanar regions share tokens. Warped polygons use the planes of their actual rendered
triangles, with triangle-specific edge/vertex incidence. Large raw meshes receive triangle
planes without constructing topology: indices rotate cyclically to choose an available first
vertex, copying that vertex only when necessary. `face_id` and the physical-area flag both use
`@interpolate(flat, first)`, so unrelated tokens on the other two shared vertices do not change
the triangle's identity. See [WGSL interpolation](https://www.w3.org/TR/2026/CRD-WGSL-20260831/#interpolation).

Ink retains its full footprint over explicitly supporting faces. For a foreign face, visibility
compares the underlying edge axis with that face's physical plane at the axis. A depth value
sampled sideways from a wide stroke is insufficient on a sloping cover. The shader also tests
the projected centreline, which prevents hidden strokes spilling across covering silhouettes
or across different planes in concave geometry. Visibility normals use the inverse transpose;
orthographic views use parallel viewing directions. Plane points and unit normals receive their fixed linear placement once before upload, using
f64 arithmetic over the instance's rounded matrix. Dynamic translation remains in the
re-anchored instance table. This avoids repeated per-fragment matrix/normal transforms without
baking large absolute translations into f32 plane positions.

FLAT remains a camera-facing capsule quad. TUBE retains the cylinder representation but tests
visibility at its underlying axis, not its nearer shell. Ribbons, markers and picking share
the visibility rule. Markers draw after strokes. MSAA visibility is computed separately for
each sample and returned through `sample_mask`; repeated nonzero face decisions are cached
separately for the centreline and fragment queries. This avoids repeating identical plane work
without merging distinct sample coverage. The renderer also skips
texture queries outside a conservative screen rectangle containing physical mesh faces and
resident cloud splat footprints. Near/far/eye clipping or numerical uncertainty falls back to
the full viewport; both centreline and fragment queries must be outside before bypassing both.
These optimizations passed the latest final-source renders; timing remains pending. See [WGSL sample masks](https://www.w3.org/TR/2026/CRD-WGSL-20260831/#builtin-values).

### Numerical edge-on faces

On the real floor, geometrically edge-on triangles acquired small rasterized slivers after f32
projection/subpixel rounding. Those slivers supplied invalid covering depth at isolated pixels.
The physical face and face-ID passes now reject the angular band

```text
abs(dot(normal, view_ray)) <= 32 * f32::EPSILON * length(normal) * length(view_ray)
```

The shader evaluates its squared equivalent. This is approximately 3.815e-6 radians from
edge-on. It is **not an exact zero-projected-area predicate**, and it does not establish a
universal rounding bound for arbitrary ill-conditioned transforms. It can reject genuinely
grazing faces. In perspective, the ray is measured at the plane's representative point, so a
very extended face can have nearer parts at a different viewing angle. The acceptance renders
establish the behavior of the tested scenes and cameras; they are not a proof for every such
configuration. Neither positions nor depth values are offset by this rule.

The rejected in-face implementation (`deb11910`, reverted by `4dda30a5`) folded/recessed stroke
coverage and damaged close-up corners. The present implementation keeps the full visible ink
footprint. The previously tested clamped hardware-bias settings also failed the rendered probes.
Those failures do not prove that all hardware-bias designs are impossible; a depth-bias clamp
is not a universal guarantee against crossing a small world-space joint.

## Resolved GPU version and layouts

`Cargo.toml` requests `wgpu = "29.0"`; `Cargo.lock` resolves **wgpu 29.0.4**. Implementation used
the installed registry source for that exact release, including
`wgpu-29.0.4/src/api/render_pass.rs`. The corresponding [versioned API source](https://github.com/gfx-rs/wgpu/blob/v29.0.4/wgpu/src/api/render_pass.rs#L625)
declares the optional depth/stencil operations. The ink pass uses `depth_ops: None` and disables
depth writes. The [WebGPU depth/stencil attachment rules](https://www.w3.org/TR/webgpu/#depth-stencil-attachments)
are the API-level contract; headless validation also executes this configuration.

Offsets and storage-array strides below are bytes. Rust `repr(C)` layouts are compared against
Naga's parsed/validated WGSL member offsets and structure spans by
`shader_validation_and_layouts`, following [WGSL alignment and size rules](https://www.w3.org/TR/2026/CRD-WGSL-20260831/#alignment-and-size).

| Structure | Field offsets | Stride |
|---|---|---:|
| `CylinderSegment` | `p0` 0/4/8; `radius` 12; `p1` 16/20/24; `instance_id` 28; `color` 32; `facing` 36; `support_start` 40; `support_count` 44 | 48 |
| `GlyphPoint` | `center` 0/4/8; `radius` 12; `color` 16; `instance_id` 32; `facing` 36; `facing_ext` 40/44; `support_start` 48; `support_count` 52; padding 56/60 | 64 |
| `InkSupport` | `face` 0; `region` 4 (`0` whole stroke, `1` first endpoint, `2` second endpoint) | 8 |
| `FacePlane` | `point` 0/4/8; `instance_id` 12; `normal` 16/20/24; padding 28 | 32 |
| `Instance` (unchanged) | `model` 0; `color` 64; `flags` 80; `thickness` 84; `spacing` 88; tail padding 92 | 96 |
| `LineUniform` | `thickness` 0; `proj_y` 4; `ortho_h` 8; `vp_h` 12; `vp_w` 16; eye 20/24/28; anchor 32/36/40; `feather` 44; `occluder_rect` 48/52/56/60 | 64 |
| `FaceFilterParams` (follow-up candidate) | `index_count` 0; `row_width` 4; two padding words 8/12 | 16 |

The earlier physical checkpoint used a 48-byte `LineUniform`; the final candidate appends a
16-byte `[left, top, right, bottom]` rectangle. All WGSL declarations and the Naga offset test
have been updated and validated by the final test suite and headless renders. `FacePlane` retains its 32-byte layout
while its point/normal acquire the baked-placement meaning described above.

Each arena vertex also has a four-byte face token at vertex location 4. Segment rows grew from
40 to 48 bytes, rather than adopting the experimental 96-byte row. The separate support lists,
face planes, tokens and necessary crease/raw-vertex copies are additional memory costs; the
fixed-row increase alone is not the total scene-memory increase. Support ranges and face bases
are rebased on append and reset on rebuild. Plane-buffer growth/release and target changes
rebind the current GPU resources.

Uncaptured GPU validation errors now panic. The ignored native test
`invalid_gpu_shader_is_fatal` intentionally creates an invalid shader and requires that failure;
a printed validation error followed by a successful image is no longer an acceptable result.

## Confirmed rendered evidence

The latest checked candidate is `/tmp/hl-bounds`; its executable hashes are in
`/tmp/hl-bounds/sha256.txt`. The selftest SHA-256 is
`74c0aeec9171ed3581c26ea00038fa743a27d13a88eba4434d22bad53fded3bc`. These temporary artifacts are evidence from this session, not
portable fixtures or substitutes for rebuilding the main source.

### Synthetic coverage and continuity

`mk_hidden_line_probe` generates separate-object covers, a same-object nonadjacent cover,
nonuniform placement, concave coverage and a warped-polygon variant. Hidden mesh edges,
markers and authored polylines are magenta; legitimate visible strokes are blue and covers
are grey. The nominal hidden clearance is 4 mm before the nonuniform instance transform.

The matrix is two fixtures × top/down/iso × distance factors 1/4/16 × FLAT/TUBE × MSAA 1/4:
**72 renders at 1400×900**. Top is orthographic; down and iso are perspective. Each render
retains the actual harness camera in its log.

| Measurement | FLAT | TUBE |
|---|---:|---:|
| Completed synthetic cases | 36 | 36 |
| Magenta pixels, even `R > G && B > G` by one channel level | 0 | 0 |
| Blue pixels lost/added versus the physical checkpoint | 0 / 0 | 0 / 0 |
| Changed RGB pixels over the complete images versus that checkpoint | 1 | 1 |

The intermediate sample-mask and physical-area checkpoints were byte-identical to the
centreline checkpoint across all 72 cases. The final candidate changes one nonblue pixel in
each style's iso/1×/MSAA4 image: `(857, 413)`, `[203, 197, 197]` becomes neutral grey
`[206, 206, 206]`. Blue strokes lose/gain no pixels. The 1×/4× regular fixtures retain five
connected blue outlines; warped fixtures retain one. At 16× some oblique projections merge
nearby outlines without adding fragmented components. Images and results are in
`/tmp/hl-probe-bounds`, `/tmp/hl-warp-bounds` and `/tmp/hl-probe/bounds_spatial.json`.

The authored Point/NURBS fixture adds another **36 renders** with the same camera, distance,
style and MSAA matrix. Every image has zero exact magenta pixels, exactly **one connected blue
curve**, and exactly **five connected green dots**. The hidden curve and dot controls sit 4 mm
below the cover. Top-fit, oblique 4× and both perspective/orthographic 16× crops were inspected;
visible ink remains continuous and the dots remain distinct. Evidence:
`/tmp/hl-authored-bounds`, including `top_1_flat_4.png`, `down_4_flat_4_crop.png`,
`iso_16_tubes_4_crop.png` and `top_16_flat_1_crop.png`. Thus the final colored matrix contains
**108 cases, 54 per style, with zero exact magenta pixels in every case**.

### Close-up

`assets/pb/view_local_boxes.pb`, `VIEWER_ZOOM=5`, 1400×900, MSAA 4. Compare against the freshly
built baseline on the **same llvmpipe adapter**, rather than treating the historical Intel
coverage count of 262,685 as a cross-driver golden image.

| Style | Fresh baseline non-background pixels | Final candidate | Spatially changed RGB pixels |
|---|---:|---:|---:|
| FLAT | 264,662 | 264,644 | 1,604 |
| TUBE | 264,073 | 264,332 | 1,837 |

These close-ups are not pixel-identical. Full images and enlarged corner comparisons were
inspected: red edges retain their width through corners, markers remain on top, and caps and
intersections remain continuous in both styles. This judgment is based on spatial inspection,
not equal total coverage. The final red stroke masks are identical to the accepted physical
checkpoint in both styles. The final Point support extension changes 97 FLAT / 95 TUBE pixels
near gradient-box markers; the whole image comparison is therefore not byte-identical.
Evidence: `/tmp/hl-final-bounds-views/close_{flat,tubes}.ppm`, corresponding logs,
`spatial.json` and `corners_{flat,tubes}.png` (fresh baseline top, current bottom). The full
unrecolored mixed scene was also rendered and inspected in both styles in that directory.

### Lifecycle and picking

`check_hidden_line_lifecycle` passed on the final candidate. It generates a scene with no
faces and also loads all three probe documents, including authored Point/NURBS controls. On one live device it checks perspective and
orthographic cameras, FLAT→TUBE→FLAT, MSAA 1→4→1, resize/restore, `Scene::rebuild`, release,
and incremental versus batched uploads. Every matching state has identical complete RGBA and
object/segment-ID buffers. Visible IDs resolve to retained object/ribbon ranges. Evidence:
`/tmp/hl-final-bounds-lifecycle/lifecycle.log` and images in that directory.

`check_determinism` also compares the new face IDs, planes and support lists. Both probe
fixtures passed; evidence is `/tmp/hl-determinism.log`.

## Original floor census — final candidate passed

The task's legacy analytic estimate records 4/19,440 surfacing/covered samples at fit and
508/16,804 at approximately 2.6× fit. These explain the original offset defect; they are not
the new rendered-ID metric and must not be compared as if the two measurements were identical.

[_hidden_line_matrix.py](_hidden_line_matrix.py) renders the original, unrecolored floor and
passes each **actually logged camera** to `examples/census_plates.rs`. It records executable and
fixture SHA-256 hashes and runs seven task cameras × 1/4/16 × two styles = **42 cases** at
1800×1400, MSAA 4. The two named far cameras retain their original zoom and additionally apply
the requested distance multiplier.

The census attributes pixels by exact original object and segment ID from the picking pass.
It distinguishes other polyline legs, visible represented axes and partially covered pixels
from an opaque ink-core pixel whose represented axis and whole pixel are covered by physical
geometry. Picking's alpha threshold is 0.5; the metric therefore does not certify every faint
AA fringe. The separate full-color synthetic census checks those fringes. Analytic legacy
offset constants remain labeled as a BEFORE estimate; setting them to zero cannot make the
rendered-ID check pass.

All **42 cases passed** for `/tmp/hl-bounds/selftest`, checked by the corresponding
`/tmp/hl-bounds/census_plates`. The previous physical checkpoint also passed the same matrix.
The source-only denominator is identical for both styles and for baseline/current; the counts
aggregate samples over camera cases, not unique physical points.

| Style | Cases | Independent covered-sample denominator | Baseline fully-covered-pixel matches | Final candidate | Baseline worst case → current |
|---|---:|---:|---:|---:|---|
| FLAT | 21 | 344,840 | 162 | 0 | `down`, 1×: 75 / 16,750 → 0 |
| TUBE | 21 | 344,840 | 19 | 0 | `side`, 1×: 8 / 17,361 → 0 |

At the representative `far2p6`, 1× setting, FLAT fell from 42 / 16,627 to zero;
TUBE was zero before and after at that particular camera. The fit iso baseline had four FLAT
and five TUBE matches, illustrating why neither one camera nor one style is sufficient.

Final checkpoint metadata, logged cameras, hashes and all per-case results are in
`/tmp/hl-final-bounds-ids/matrix.json`. The previous checkpoint is recorded in
`/tmp/hl-final-ids/matrix.json`. Baseline results using the same independent classifier are
`/tmp/hl-baseline-segment-ids/pixel_results.json`; original camera/settings are in that
directory's `id_results.json`, with detailed `*_pixel_census.log` files.

The strict zero requirement was exercised against known failures: the baseline `down`, 1×
case exited 101, and the earlier centreline checkpoint still failed on its two top-view ghost
pixels. Evidence: `/tmp/hl-baseline-segment-ids/expected_failure.log` and
`/tmp/hl-centre-focus/physical_expected_failure.log`. Both the physical and final candidates
removed those failures.

## Performance and environment — acceptance pending

The available adapter is **llvmpipe (LLVM 21.1.8, 256 bits), CPU/Vulkan**. `/dev/dri` is absent.
These are software-renderer measurements, not measurements of the user's Intel/browser GPU.
Run timing without simultaneous builds, renders or CPU image analysis.

A clean physical-checkpoint FLAT pair for `view_mixed`, 1400×900, 15 timed frames per leg plus
five warmup frames, measured the following. This exceeds the performance budget and is a
**failed checkpoint**, not a final accepted result:

| Style/scene | Baseline still/moving ms | Candidate still/moving ms | Ratios |
|---|---:|---:|---:|
| FLAT / cached exact `view_mixed` | 920.71 / 1,414.07 | 1,436.70 / 1,887.21 | 1.560 / 1.335 |

Recorded by `/tmp/hl-perf/compare.py --candidate /tmp/hl-perf-candidate/bench_frame --out
/tmp/hl-perf/physical-initial --rounds 1 --frames 15 --scenes view_mixed --styles flat`.
Raw results and individual logs are under `/tmp/hl-perf/physical-initial`.

The final correctness checkpoint, `/tmp/hl-bounds`, reduced the mixed-scene cost but still
fails the approximately 20% budget. The following clean pairs used 15 timed frames after five
warmup frames per still/moving leg at 1400×900, MSAA4:

| Style/scene | Baseline still/moving ms | Candidate still/moving ms | Increase |
|---|---:|---:|---:|
| FLAT / exact `view_mixed` | 943.13 / 1,428.57 | 1,275.10 / 1,693.26 | 35.2% / 18.5% |
| FLAT / exact `view_meshes` | 1,603.54 / 1,639.49 | 3,194.76 / 3,233.18 | 99.2% / 97.2% |
| TUBE / exact `view_meshes` | 2,982.62 / 3,003.70 | 10,435.31 / 10,352.54 | 249.9% / 244.7% |

These results are in `/tmp/hl-perf/bounds-initial` and
`/tmp/hl-perf/bounds-other-scenes/raw.json`. They are single paired runs, sufficient to
reject these large regressions but not presented as a repeated final benchmark. The candidate
`bench_frame` SHA-256 is
`6c864e29176e8e10b4358b3295531622a59530c6dcfa5bc8a02d193115d7ae89`.
All correctness results above belong to this same source checkpoint. Performance diagnostics
do not weaken its default visibility rule.

Controlled same-binary diagnostics subsequently isolated the main FLAT costs. On `view_meshes`,
the normal shader measured 3,160.11 / 3,295.03 ms; disabling the ink visibility helper measured
2,472.06 / 2,421.38 ms. Keeping the angular-area calculation live while removing only its
fragment discard measured 2,303.51 / 2,295.34 ms. Disabling both required paths measured
1,472.09 / 1,446.11 ms. These deliberately incorrect controls are diagnostic evidence only.
They motivated moving the unchanged angular predicate ahead of rasterization and reducing
repeated ink queries. The analogous TUBE visibility-off control still cost
8,993.97 / 9,227.74 ms, so that style has additional costs requiring investigation.

The exact requested five-sheet `view_lines` fixture could not be obtained in this restricted
session. Cached `/home/petras/.cache/v45c2/assets/scenes/drawings.toml` contains **ten sheets**
and is a labeled substitute, not an equivalent reproduction. Keep its timings separate from
the historical five-sheet numbers. Hardware/browser timing remains unverified here.

Logical live geometry/ink row memory was also measured after the final CPU changes:

| Cached scene | Baseline payload | Final payload | Increase |
|---|---:|---:|---:|
| Exact `view_mixed` | 41.677 MiB | 45.938 MiB | 10.22% |
| Exact `view_meshes` | 55.283 MiB | 152.069 MiB | 175.07% |

These exclude unchanged instance/cloud buffers, CPU mirrors, allocation slack and render
attachments. The additional face-token target costs 19.226 MiB at 1400×900, MSAA4. Conservative
bounds also retain a 24-byte local CPU AABB per object: 2.453 MiB logical payload for the mixed
scene, plus a sparse physical-owner map. The raw triangle-plane/token representation accounts
for the substantial mesh-scene increase; segment stride alone understates the memory cost.
Evidence: `/tmp/hl-perf/memory.md`, `mixed_rows_final.log`, `meshes_rows_final.log`.

**TODO — FINAL PERFORMANCE:** record final source/binary hashes, repeated clean paired still/
moving results for both styles and available scenes, and budget decision. Retain any
outstanding fixture/hardware limitation explicitly.

## Reproduction from the main worktree

```bash
cd /home/petras/code/code_rust/session/session_viewer
export CARGO_TARGET_DIR="$PWD/target" REGEN_PROTO=0
cargo build --release --target x86_64-unknown-linux-gnu \
  --example selftest --example census_plates --example bench_frame \
  --example mk_hidden_line_probe --example check_hidden_line_lifecycle \
  --example check_determinism
HL_BIN="$CARGO_TARGET_DIR/x86_64-unknown-linux-gnu/release/examples"

python3 docs/_hidden_line_matrix.py \
  "$HL_BIN/selftest" "$HL_BIN/census_plates" \
  /tmp/view_mixed_floor_model.pb /tmp/hl-final-matrix --require-zero

for style in flat tubes; do
  env VIEWER_W=1400 VIEWER_H=900 VIEWER_ZOOM=5 VIEWER_MSAA=4 \
    VIEWER_LINE_STYLE="$style" "$HL_BIN/selftest" \
    "/tmp/hl-close-$style.ppm" assets/pb/view_local_boxes.pb
done

mkdir -p /tmp/hl-probes
"$HL_BIN/mk_hidden_line_probe" /tmp/hl-probes/regular.pb
HIDDEN_LINE_PROBE_WARPED=1 "$HL_BIN/mk_hidden_line_probe" /tmp/hl-probes/warped.pb
HIDDEN_LINE_PROBE_AUTHORED=1 "$HL_BIN/mk_hidden_line_probe" /tmp/hl-probes/authored.pb
"$HL_BIN/check_hidden_line_lifecycle" /tmp/hl-lifecycle-final \
  /tmp/hl-probes/regular.pb /tmp/hl-probes/warped.pb /tmp/hl-probes/authored.pb
"$HL_BIN/check_determinism" /tmp/hl-probes/regular.pb /tmp/hl-probes/warped.pb
```

The new `authored.pb` variant contains five visible green coplanar free dots and a visible
blue NURBS, with magenta controls 4 mm below the cover. All 36 additional cases passed. The
complete 108-case color probe can be reproduced without the session's temporary helper scripts:

```bash
python3 - "$HL_BIN/selftest" /tmp/hl-probes <<'PY'
import json, os, pathlib, subprocess, sys
sys.path.insert(0, 'docs')
from _count_colors import read_ppm
binary, root = sys.argv[1], pathlib.Path(sys.argv[2])
base = {k: v for k, v in os.environ.items()
        if not k.startswith(('VIEWER_', 'CENSUS_', 'BENCH_'))}
results = []
for fixture in ('regular', 'warped', 'authored'):
    for camera, settings in [('top', {'VIEWER_VIEW': 'top'}),
                             ('down', {'VIEWER_ORBIT': '0,209'}), ('iso', {})]:
        for distance in (1, 4, 16):
            for style in ('flat', 'tubes'):
                for msaa in (1, 4):
                    name = f'{fixture}_{camera}_{distance}_{style}_{msaa}'
                    output = root / f'{name}.ppm'
                    knobs = dict(VIEWER_W='1400', VIEWER_H='900', VIEWER_NO_GRID='1',
                                 VIEWER_DISTANCE_SCALE=str(distance), VIEWER_LINE_STYLE=style,
                                 VIEWER_MSAA=str(msaa), **settings)
                    run = subprocess.run([binary, str(output), str(root / f'{fixture}.pb')],
                                         env=dict(base, **knobs), capture_output=True, text=True)
                    (root / f'{name}.log').write_text(run.stdout + run.stderr)
                    run.check_returncode()
                    _, _, rgb = read_ppm(str(output))
                    magenta = sum(r > g and b > g for r, g, b
                                  in zip(rgb[::3], rgb[1::3], rgb[2::3]))
                    results.append(dict(name=name, exact_magenta=magenta))
                    assert magenta == 0, (name, magenta)
(root / 'results.json').write_text(json.dumps(results, indent=2))
print('zero-magenta cases:', len(results))
PY
```

Compare full images/corner crops and blue connected strokes separately; the color assertion
alone does not prove that visible lines were retained.

Required checks and the explicit fatal-validation test:

```bash
env CARGO_TARGET_DIR="$PWD/target" REGEN_PROTO=0 docs/_gate.sh
env CARGO_TARGET_DIR="$PWD/target" REGEN_PROTO=0 cargo xtest
env CARGO_TARGET_DIR="$PWD/target" REGEN_PROTO=0 cargo xtest invalid_gpu_shader_is_fatal -- --ignored
env CARGO_TARGET_DIR="$PWD/target" REGEN_PROTO=0 cargo check --target x86_64-unknown-linux-gnu
env CARGO_TARGET_DIR="$PWD/target" REGEN_PROTO=0 cargo check --target wasm32-unknown-unknown
```

Final-source checks passed on the `/tmp/hl-bounds` candidate:

- `docs/_gate.sh`: `gate OK`, local ink 62,944; `/tmp/hl-final-gate.log`.
- `cargo xtest`: 20 passed, one intentionally ignored fatal-validation test; the GPU agent
  repeated the suite after the complete candidate build.
- Explicit `invalid_gpu_shader_is_fatal -- --ignored`: expected validation panic accepted;
  `/tmp/hl-final-fatal-validation.log`.
- Native and wasm `cargo check`: `/tmp/hl-final-native.log`, `/tmp/hl-final-wasm.log`.
- Release/all-target Clippy with `-D warnings`: `/tmp/hl-final-clippy.log`.
- `census_plates` example unit tests: eight passed; `/tmp/hl-final-census-tests.log`.

The main-tree Trunk build completed at **22:26:12 Europe/Zurich** on 2026-09-04;
`/tmp/hl-final-trunk.log`. It generated
`dist/session_viewer-9924773efb92c29c_bg.wasm`, SHA-256
`c0f8072909ce9b9b9e87e3263ce579fea8f1a3cc341ec4535e3cc7cd8f5c544f`.
An actual `trunk serve` rebuilt successfully at 22:25:29 but could not start its server:
`Operation not permitted (os error 1)`, logged in `/tmp/hl-final-trunk-serve.log`.
`curl` likewise could not open a socket under this environment restriction. The **main browser
artifact has changed**, but serving `http://localhost:8770/?scene=view_mixed` and browser GPU
runtime verification are blocked here; no successful browser run is claimed.
