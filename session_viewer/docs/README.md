# Build the browser viewer

This course builds Session Viewer from an empty viewer crate to the current Rust/WebGPU implementation. Start with a message produced by WebAssembly, draw a triangle, add the camera and geometry, then implement CAD boundaries, text, selection, loading and the final visibility correction.

You do not need previous graphics-programming experience. You will need time to read code and try each checkpoint. The [first lesson](00-environment.md) explains the Rust syntax and tools used at the beginning; later lessons introduce GPU concepts when you need them.

## How to use a lesson

Every numbered lesson has the same route:

1. Read what the new stage does and which state it starts from.
2. Read the explanation of the important data and functions.
3. Open **Complete file changes**. Each page says **create**, **replace in full**, or **remove**, with the exact path. Follow its Next link until all files are complete.
4. Run the checkpoint commands. Compare the browser result with the expected behavior.
5. Continue only after that stage builds and runs.

The file pages contain complete source, including imports and pipeline descriptors. You never have to guess where a snippet goes or read a diff to find missing code. Long files have a download link and a code-only copy button. Rust, WGSL and TOML have explicit highlighting.

**Type** the viewer logic you want to learn. **Copy** lockfiles, licensed font bytes, test fixtures and independent C++/Python parity implementations. These are clearly marked. Shared Session source is supplied as a pinned dependency; chapters 06–09 reconstruct its changed CAD algorithms. Rewriting every preexisting geometry-library function is outside this viewer course.

The exact patches remain the verification mechanism and an optional automatic route. They are not required reading for the manual route. A Python helper assembles and checks the files; the application you build is Rust and WGSL.

## Prepare one workspace

Run these commands from the maintained `session_viewer` checkout. Choose a **new, empty** path for your work; do not overwrite the viewer you are using.

```sh
export COURSE_REPO="$PWD"
export COURSE_WORK="$HOME/viewer-course"
export RUSTUP_TOOLCHAIN=1.97.1
export REGEN_PROTO=0
export NO_COLOR=true
rustup toolchain install 1.97.1
rustup target add --toolchain 1.97.1 wasm32-unknown-unknown
cargo +1.97.1 install trunk --version 0.21.14 --locked
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$COURSE_WORK" --initialize-only
```

`COURSE_REPO` points at the supplied course and fixtures. `COURSE_WORK` is where you write code. The initializer extracts checksum-verified shared source packages into that workspace; it does not write the viewer for you. Create `session_viewer` beneath it as instructed in lesson 00.

Use Python 3.13 or newer for the archive extraction helper. Cargo obtains the dependencies pinned by the supplied `Cargo.lock`. Git is required by the optional replay/check tools. No personal R2 account or credentials are needed for the local course.

## Build, run and stop

From the lesson's viewer directory:

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. Keep that terminal open while using the page. Stop the server with **Ctrl+C** before advancing the lesson. Port 8780 keeps the course separate from the production viewer on 8771. If the port is occupied, use another free port and open that same number.

A successful Cargo check verifies Rust. A successful Trunk build prepares WASM and browser assets. A visible browser result with no GPU errors verifies initialization and shader use. You need all three; an empty canvas is not a passing checkpoint.

## Optional exact reconstruction and browser checks

To have the driver write a lesson for you, choose another empty workspace and run:

```sh
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$HOME/viewer-course-auto" --through 00
python3 "$COURSE_REPO/docs/reconstruction/replay.py" --output "$HOME/viewer-course-auto" --through 01 --advance
```

`--advance` verifies the previous source hashes before editing. It refuses to overwrite handwritten changes. After manually reproducing an exact checkpoint, `--adopt` checks every required file and records that stage. Different comments/formatting also produce different hashes; use ordinary Cargo/browser checks while experimenting, or compare with the supplied full files when returning to exact reconstruction.

For automated real-browser checks, install the test runner once:

```sh
npm install --prefix "$COURSE_WORK/browser-tools" playwright@1.58.2
export NODE_PATH="$COURSE_WORK/browser-tools/node_modules"
export CHROME_BIN=/usr/bin/google-chrome
export VIEWER_HEADLESS=0
```

Set `CHROME_BIN` to your installed WebGPU-capable Chromium executable. Add `--verify` to replay/adopt commands to run a locked build and actual browser checkpoint. These checks start an ephemeral local server and save results in the work directory. The recorded Linux test host needed `DISPLAY=:0` and Vulkan/ANGLE launch arguments; see [the tested environment](measurements.md). They are host-specific test settings, not deployment requirements.

## Course map

| Stage | What you add |
|---|---|
| [00 · Environment](00-environment.md) | Cargo, TOML, WASM startup and diagnostics |
| [01 · First frame](01-first-frame.md) | Device, surface, pipeline and first WGSL triangle |
| [02 · Camera](02-camera.md) | Spaces, matrices, orbit/pan/zoom, projection and DPR |
| [03 · Identity](03-identity.md) | Source objects, transforms, styles and GPU rows |
| [04 · Drawing modules](04-modules.md) | Meshes, strokes, points and clouds with one resource owner |
| [05 · Physical visibility](05-visibility.md) | Depth, thick ink, hidden-line and close-up checks |
| [06 · CAD data](06-cad-contract.md) | Source faces, mesh records, UVs and normal contracts |
| [07 · Shared boundaries](07-boundaries.md) | Canonical edge samples and constrained face triangulation |
| [08 · Trims and seams](08-trimming.md) | Holes, periodic surfaces and boundary provenance |
| [09 · Shading](09-normals.md) | Smooth interiors, sharp creases, poles and transforms |
| [10 · Text layout](10-text-layout.md) | Fonts, shaping, advances, offsets and baselines |
| [11 · Text rendering](11-text-rendering.md) | Glyph coverage, black backing, placement, DPI and caches |
| [12 · Picking](12-picking.md) | Production event shell, visible IDs and yellow selection |
| [13 · Source controls](13-controls.md) | F10, original IDs and complete streamed-source queries |
| [14 · Loading](14-loading.md) | Manifests, protobuf, validation and safe replacement |
| [15 · Publication](15-publication.md) | Immutable revisions, loading reuse and bounded reads |
| [16 · Resource accounting](16-verification.md) | Ownership measurements and the full verification harness |
| [17 · Faces, text and outlines](17-source-presentation.md) | Selectable source text/faces, joined ink and one silhouette |
| [18 · Finite visibility](18-finite-visibility.md) | Concave/touching-edge correction and final production convergence |

## Verified scope

The series contains 19 runnable checkpoints. Earlier stages retain their recorded clean-build/browser evidence; the added final stages have separate clean reconstructions. The [verification record](reconstruction/verification.json) identifies the actual checks. The [final inventory](reconstruction/baseline.json) freezes 91 runtime source files; [convergence](reconstruction/convergence.json) verifies that the final lesson uses those same bytes. Local fixture packaging differs from the user's working dataset cache.

The application structure and extension guide are in [Architecture](../ARCHITECTURE.md). [Coverage](coverage.md) maps requirements to source and tests. [Measurements](measurements.md) separates current correctness checks from historical performance evidence and unverified platforms.

## Run this documentation locally

Install [uv](https://docs.astral.sh/uv/getting-started/installation/) if `uvx` is unavailable. From the maintained viewer checkout, build and check the site:

```sh
docs/serve.sh build
python3 docs/check_site.py
docs/serve.sh
```

Open <http://localhost:8772/>. The build generates the linked complete-file pages from the verified patches and supplies exact downloads. Use this rendered site for those links. `check_site.py` compares every rendered listing and download against its checkpoint, checks required edits/removals, local links and explicit language classes.

The site uses [Material code blocks](https://squidfunk.github.io/mkdocs-material/reference/code-blocks/) and [Pygments language lexers](https://pygments.org/languages/). Its build language does not limit the languages it teaches. Generated listings, the site and temporary assembly live under ignored `target/docs`; maintained Markdown and the exact reconstruction inputs remain in Git.
