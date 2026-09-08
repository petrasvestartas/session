# Maintained browser checks

Serve the production build with `REGEN_PROTO=0 NO_COLOR=true trunk serve` on port 8770.
Open `/text-quality.html` for the same-font white-on-black comparison. The displayed
sizes and metrics come from the Rust fixture; the browser reference loads the same
bundled font bytes and explicitly enables kerning/common ligatures.

The text browser check needs Playwright 1.58.2 and Chrome. Dependencies and screenshots
can live outside the repository:

```sh
npm install --prefix /tmp/viewer-browser-test playwright@1.58.2
NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/text-quality.cjs
```

`CHROME_BIN` selects an installed browser; `VIEWER_URL` selects the viewer origin;
`VIEWER_TEST_OUTPUT` selects the capture directory (default `/tmp/session-viewer-text-quality`).
`VIEWER_HEADLESS=1` is supported only where that browser exposes a usable WebGPU adapter.
`VIEWER_CHROME_ARGS` accepts a JSON array for a recorded test-environment configuration;
these flags are not application requirements.

The check uses DPR 1, 1.25 and 2, then forces raster scales 1, 1.25, 1.5 and 2, verifies
that changing scale does not reshape, exercises selection color, gray compositing,
resource disposal and resize, and captures normal-size images and measured metrics.
Same-font browser/shaper line widths must agree within 0.2 CSS pixels (floating-point
font metric differences, not a raster similarity tolerance). There is no automatic
claim about subjective clarity, real monitor changes, Firefox/Safari, world-label
occlusion or imported PDF outlines. Inspect the captured normal-size images, and record
those additional cases only when actually tested. Browser zoom and forced raster scale
are different checks.

`node tests/nameplate.cjs` selects the maintained object-nameplate specimen and checks
13.5 CSS pixel white text, an opaque black background with maximum rounded corners,
centering, DPR 1/2, cached preparation and release. `node tests/nameplate-scene.cjs`
uses the interaction fixture below to verify square 18px document titles and rounded
selected source names in the real canvas, including source bounds centers, zoom,
F10/Escape restoration and the persistent T toggle for selected names. Both tests measure rendered pixels; annotations are GPU text
projected from world anchors with an explicit overlay depth policy.

`node tests/world-text.cjs` loads a manifest-authored fixed plane, verifies white glyphs
on black, checks its world axes through camera changes at DPR 1/2, and confirms that
`T` leaves it visible. The native ignored GPU test `fixed_plane_obeys_solid_depth_orientation_cache_and_release`
checks actual solid occlusion, projection, cache reuse and texture release.

`node tests/teapot.cjs` uses the intentionally retained `assets/pb/view_mixed_teapot.pb`:
the existing 32-patch Utah teapot, preserved byte-for-byte from the archive worktree.
It checks source hash, GUID, visible surfaces and all 512 surface controls at DPR 1/2.
The asset has no verified 3ds Max export provenance. Its original four open shells are retained.

The native ignored GPU test `selected_silhouette_is_black_visible_only_and_releases_coverage`
checks black selected-surface silhouettes against physical occlusion, unchanged picking
IDs, 1x/4x transitions and immediate coverage-texture release when selection clears.

`node tests/pdf-text-quality.cjs` compares the checked local PDF asset at forced 1x and
budget-selected 4x. It checks the source SHA-256, automatic sheet coverage, and three
normal-size glyph crops for partial coverage, retained ink mass, bounds and word position.
The test preserves the original PDF outline/font geometry. It captures both full drawings
and crops; it is deliberately tied to that source and camera. Run on a stable production
bundle to avoid a development rebuild overlay contaminating the screenshots.

Generate the seven-object interaction source fixture outside the repository, then drive
real canvas clicks and keyboard events against the opt-in read-only inspection snapshot:

```sh
REGEN_PROTO=0 cargo run --example interaction_fixture --target x86_64-unknown-linux-gnu -- /tmp/viewer-interaction.pb
NODE_PATH=/tmp/viewer-browser-test/node_modules node tests/interaction.cjs
```

`VIEWER_INTERACTION_FIXTURE` can select another generated fixture path (the adjacent
`.json` provides original source identities and world-space targets). The runner verifies
mesh, line, polyline, NURBS curve/surface, BRep and resident cloud selection at DPR 1 and 2,
visible yellow pixels, repeated F10 disposal behavior, Escape, parent replacement and
five/ten-CSS-pixel edge tolerance. Ctrl edges apply to mesh, BRep and NURBS surface
boundaries; standalone curves retain whole-object selection and source-control behavior,
without inventing an edge for every displayed chord. This fixture is not a streamed-cloud completeness test.

The runner foregrounds and focuses the canvas, then waits for a newer submitted observation
with completed picking after each action. A fixed delay followed by an already-idle snapshot
can otherwise observe the state before the browser dispatches the action.

`node tests/text-zoom.cjs` uses Chrome's actual Settings → Appearance → Page zoom control
at 100%, 125%, 150% and 200% in a disposable persistent profile. It checks actual
`devicePixelRatio` and canvas scale changes, retains logical shaping, compares same-font
metrics and captures visible white glyph pixels. This is separate from emulated DPR and
forced raster-scale checks; the runner deletes its own temporary profile on completion.

Native GPU regressions run explicitly on a machine with a supported adapter:

```sh
REGEN_PROTO=0 cargo xtest actual_glyph_coverage_obeys_depth_clip_motion_and_release -- --ignored
REGEN_PROTO=0 cargo xtest continuous_world_scale_evicts_without_reshaping_or_stale_instances -- --ignored
REGEN_PROTO=0 cargo xtest isolated_outline_keeps_coverage_selection_and_identity -- --ignored
REGEN_PROTO=0 cargo xtest canceled_completion_is_discarded_and_next_pick_still_completes -- --ignored
```

These verify actual glyph occlusion/foreground/billboard/overlay coverage, clip bounds,
placement changes without reshaping, cache eviction, release, exact imported outline IDs,
selection/hidden behavior, legacy mixed-print equivalence, and canceled GPU completions.

`node tests/streamed-controls.cjs` serves a virtual ranged protobuf source with 6,065,539
points and caps display residency at 250,000. It holds the second detail page to verify
that an early visible hit is not committed before every eligible source page finishes.
Overlapping points within and across pages must resolve to the frontmost source point;
its original fixed32 ID is fetched with a four-byte range, and its exact source position
remains as one yellow marker. A delayed range followed by Escape checks cancellation.
The fixture writes no large point-cloud file and retains no full-cloud CPU/GPU copy.
It uses the same Chrome/Playwright environment variables as the text checks; the default
capture directory is `/tmp/session-viewer-streamed-controls-dpr1`. Run again with
`VIEWER_DPR=2` to check physical/CSS conversion and point visibility at DPR 2. Display LOD and residency
remain bounded while F10 source queries examine every intersecting source node.

`python3 tests/modularity.py` prepares a scratch copy, removes the standalone point
producer and its registration, and applies an explicit skip-before-row-allocation policy.
It compiles the WASM build and runs the same seven-family interaction checks with one
additional standalone-point probe in the source. Exactly seven drawable rows must remain.
The maintained production source is never edited; shared GPU markers still serve controls
and other geometry. `--target-dir /path/to/cargo/target` can reuse build artifacts, and
`--prepare-only` stops after compilation when another GPU regression is running.

CAD fixtures use the production native render path and preserve source topology through
rotated, mirrored and nonuniform instance transforms:

```sh
REGEN_PROTO=0 cargo build --target x86_64-unknown-linux-gnu --example selftest --example cad_fixture --example check_cad_fixture
VIEWER_ADAPTER=Intel python3 tests/cad-quality.py
```

The runner writes twelve 700×520 PPM images, adapter/validation logs and metrics under
`/tmp/cad-fixture`. Inspect the source and affine cylinder, sphere, hole and C0 patch at
native size; the fill-only and unlit pairs distinguish shading from outlines. Numerical
checks cover smooth sphere variation, unlit uniformity, reversed-face diagnostics and
six post-upload source-edge mappings. Shared geometry minitests separately verify
one-sided C0 normals, trimmed holes and exact shared boundary XYZ. This is a Session
rendering regression, not an OCCT pixel comparison.

`python3 tests/parity.py` rebuilds and runs the existing shared CAD mini-tests without a
GPU: `RemeshNurbsSurfaceGrid`, `NurbsSurfaceTrimmed` and `BRep`. The expected totals are
51 Rust, 59 C++ and 51 Python tests; C++ has eight additional preexisting trimmed-surface
tests. Rust uses the maintained `check_shared_geometry` example and the normal Cargo
dependency resolver. C++ uses the existing `point_minitest` CMake target's compiler flags
and link inputs, selecting only those three test registration objects. A generated main
calls the existing registry, with only report paths redirected outside the source tree.
Python runs the three original modules with their report destination redirected.

The default C++ build directory is `../session_cpp/build` and must use Unix Makefiles;
`--cpp-build /tmp/cad-cpp-build` creates a separate build with the same generator. Building
from scratch requires the shared package's documented compiler and dependency tooling,
including network access for CMake's pinned dependencies. `--jobs 4` controls C++ build
parallelism. Commands, compiler versions, assertions, counts and logs go under
`/tmp/session-viewer-parity` (override with `--output`); no test report is written into
the source tree. A changed or missing suite count fails the gate for review.

`node tests/picking-performance.cjs` compares real mouse-release selection on a controlled
baseline (`VIEWER_BASELINE_URL`, default port 45693) and the current viewer (`VIEWER_URL`,
default port 18772). Set `VIEWER_INTERACTION_FIXTURE` to the same generated seven-family
fixture used by `interaction.cjs`; the runner mocks identical YAML and protobuf bytes for
both builds and records their SHA-256. Both builds need the inspection attributes used by
the controlled baseline: selected row, frame count, submission timestamp and camera matrix.
The default is ten measured samples per family after one warmup, using actual Escape and
mouse input. Submission, the following browser frame callback and screenshot completion
are reported separately as median and p95; none measures physical display presentation.
Every sample must also show yellow screenshot pixels. Failed highlights remain failures
and are excluded from successful-highlight timing summaries. There is no fixed sleep in
the measured path. `--self-test` checks timestamp/summary logic without Chrome; browser
runs use the same Playwright installation and Chrome settings as `interaction.cjs`.

`python3 tests/format.py` checks all handwritten Rust files under the viewer's `src/`
and `examples/` with Rust 2024 defaults. Use `--write` to apply the same formatting.
The script passes an explicit file list and `skip_children=true`, so it never traverses
or reformats the shared Session packages.

The maintained depth scripts are in `tests/depth/`; they moved out of the archived lessons.
The strict hidden-ink matrix renders 54 combinations and the original floor census renders
21 views, including distance ×16. Both require zero hidden ink in their stated masks:

```sh
python3 tests/depth/_probe_matrix.py SELFTEST MK_HIDDEN_LINE_PROBE /tmp/depth-probes
python3 tests/depth/_hidden_line_matrix.py SELFTEST CENSUS_PLATES FLOOR.pb /tmp/depth-floor --require-zero
python3 tests/depth/_closeup_box.py /tmp/grey-box.ppm
```

Use the built native `selftest`, `mk_hidden_line_probe` and `census_plates` example paths.
The grey-box image comes from `HIDDEN_LINE_PROBE_CLOSEUP=1 mk_hidden_line_probe` followed
by `selftest`; the checker requires retained red edge chains and seven attached black
vertices. The full `tests/depth/_ink_suite.sh` additionally runs the older local-scene,
shading, boundary and lifecycle checks. It needs the documented local assets and can fetch
the public floor into `${SCRATCH:-/tmp}/pb`; `INK_SUITE_NO_FETCH=1` skips an absent floor,
so use the explicit floor command above when complete floor validation is required.

The CAD audit compares each original BRep edge polygon against every incident face's
actual triangle edges, first in source f64 and then in the uploaded RenderMesh f32.
It also records per-face planarity/normals and exports isolated source objects with
unchanged GUIDs and per-object transforms. Omit the final PB argument to use only the
four generated primitive controls:

```sh
REGEN_PROTO=0 cargo run --locked --target x86_64-unknown-linux-gnu \
  --example cad_boundary_audit -- /tmp/cad-boundary-audit /absolute/path/scene.pb
uv venv /tmp/cad-plot-env
uv pip install --python /tmp/cad-plot-env/bin/python matplotlib
/tmp/cad-plot-env/bin/python tests/cad-boundary-plot.py \
  /tmp/cad-boundary-audit/geometry.json /tmp/cad-boundary-audit/meshing.png
```

The PNG/SVG shows exact triangles, original CAD boundary polygons and their mesh nodes;
its transparent inspection view intentionally includes hidden edges and does not simulate
the viewer's depth test. `geometry.json` distinguishes analytic curve-to-chord deviation
from missing triangle-edge incidence. A nonzero chord deviation is expected on curved
edges; a displayed chord must still be an exact edge of each incident triangle mesh.
The planar pyramid upload and cone seam-facing regressions run with
`cargo test --target x86_64-unknown-linux-gnu --lib app::walk::brep` (set `REGEN_PROTO=0`).
The shared `Singular Planar Normal` cases cover both trimmed and U-collapsed grid surfaces
through `python3 tests/parity.py --output /tmp/cad-parity` (53 Rust / 61 C++ / 53 Python).

The BRep front-meridian unit regression ray-tests the teapot’s exposed authored edge against
all face triangles; it fails with the earlier kernel that buried its lower-body chord.
`cad_boundary_audit` additionally checks exact f64 and uploaded f32 incidence on each face.
The shared finer-grid test retains original samples and checks local normal angles after
canonical boundary refinement. Integer picking and visible lines consume matching physical
depth-gradient attachments. The interaction fixture uses T to expose short selected strokes
and accepts their antialiased yellow coverage; the separate nameplate fixture verifies
the default centered white/black label and T persistence.
