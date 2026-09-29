# Viewer refactor and beginner course

The requested result is a smaller, readable viewer with explicit owners and a Vue course for someone learning both Rust and wgpu. Preserve working features, useful comments, blank lines, existing edits, and the runnable lesson chain.

## Baseline

- Production tree: 86,142 physical lines in 248 Rust/WGSL files, including embedded tests.
- wgpu: 29.0.4 in Cargo.lock.
- Browser library check: passed before edits (`REGEN_PROTO=0 cargo check --locked -j4 --lib`).
- Existing changes: kernel submodules, three command lessons, and command files in later checkpoints. A local patch and source hashes are in `session_viewer/target/refactor/`.
- The largest source file mixes synchronization, transaction notes, preview state and tests. GPU passes repeat binding descriptors. Command tools duplicate option parsing.
- Lesson 01 currently introduces most of the engine and delays its visible window until lesson 12. It cannot serve as a one-hour first lesson.

## Work sequence

1. Establish native tests and browser capture tooling; record failures before edits.
2. Consolidate repeated command parsing and GPU resource descriptions using small named functions.
3. Separate scene synchronization responsibilities and large pass implementation details; retain behavioral tests and public entry points.
4. Review remaining repeated source operations, shaders and lifecycle code; measure net changes without removing comments or tests to improve a count.
5. Rebuild the course around short, explicit typing tasks, supplied infrastructure, Rust explanations, graphical concepts and verifiable outcomes. Keep code includes tied to runnable sources.
6. Capture actual step outcomes; build and inspect the Vue site, check lesson source parity and compile the checkpoints sequentially.
7. Run browser/native verification, update the architecture reference and report measured results and any remaining limitations.

## Acceptance

- Native and wasm builds pass; meaningful existing behavior tests pass.
- Shared behavior has one implementation; functions and files have a clear responsibility.
- Comments explain contracts, numerical details or unfamiliar concepts; blank lines separate logical steps.
- Each tutorial states prerequisites, the result, what to type/copy, a workload below one hour, checks and recovery advice.
- Diagrams explain the concept; screenshots show the actual outcome at each runnable step and carry useful captions.
- The Vue build resolves snippets, assets and links; the final checkpoint agrees with maintained source.

## Completed

Shared binding-layout/resource helpers now own repeated GPU descriptors, shared tool options own common parsing, and `Spec::new` owns command defaults. Scene synchronization has separate notes, nodes, allocation, tombs, compaction and preview modules. Clip/SSAO pipeline construction and substantial GPU/command/panel tests are separate modules with unchanged test paths. Useful comments and whitespace remain.

Production `src` measures **85,830 physical Rust/WGSL lines in 271 files**, including tests, versus **86,142 lines in 248 files** at the captured baseline: **312 fewer lines**. Moving code is not counted as deletion. The synchronization coordinator fell from 3,881 to 614 lines; clip from 2,737 to 935, and SSAO from 1,867 to 876. The largest source file is now 1,704 lines.

All 43 lessons now have a bounded typing task (12–63 lines, at most 2,349 characters), Rust explanations, a prediction, a diagram, checks and recovery guidance. Each budgets 55 minutes of human work, with installation and machine build time separate; these are teaching estimates, not measured learner trials. Long support files are supplied by `practice.py`. Thirteen new D2 diagrams and 137 source/check/rendered screenshots support the course. Captions distinguish recorded source/check snapshots from live viewer captures and label finished-viewer references used in earlier lessons. The command-line walkthrough is current too.

Practice folders include a portable box scene and the required native test fixtures. The checker uses an isolated Cargo target and cleans viewer artifacts between checkpoints; initial results from a shared target were discarded after detecting cross-checkpoint artifact reuse. Actual fresh checks exposed and fixed early shader dependencies, missing tile bindings, late fixtures and tests introduced before their commands. Native practice tests run serially to avoid overlapping GPU-device creation.

## Verification

- All 43 restored practice crates: wasm library check and native test suite passed.
- Production: 487 native tests passed; 53 explicitly ignored by that run.
- Additional GPU run: 50 passed. Three external-data checks were excluded: `bench_frames`, `instanced_file_against_its_baked_twin`, and `benchmark_arctic` require separately configured datasets.
- Production wasm check, native tests/examples compilation and optimized Trunk build passed.
- Native selftest rendered the supplied box fixture: five objects, seven draws, 900 × 700 pixels.
- Browser exercised Point/Undo, layers, object selection, Arctic and opacity; camera, selection and object count were preserved by display changes. No browser errors. Capture metadata identifies the built viewer index.
- White and grey lesson-01 frames were rendered by changing the actual shader, with differing GPU pixel readbacks.
- Course checks: zero write-once violations; final checkpoint equals production after removing teaching comments; all snippets, diagrams and step captures present; screenshot source hashes match current listings.
- D2 diagrams reproduce from source. New diagrams passed Chrome overflow, collision and contrast checks.
- Strict Vue build passed. Site crawl checked 60 pages, 195 assets and 594 internal links, search, kernel deep links and phone widths without errors. Desktop and phone screenshots were inspected; the first-frame diagram was shortened after that review.

Logs and review screenshots are under `session_viewer/target/refactor/current`. Reproducible course tools and capture manifests are in `session_viewer/docs`. Unrelated kernel and reviewer-instruction edits were preserved. No commit or publication was made.
