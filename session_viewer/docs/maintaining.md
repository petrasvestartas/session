# Maintaining the viewer course

The viewer and its executable checks are Rust. The Vue project's TypeScript tooling assembles lesson edits, renders Markdown, checks coverage and manages learner checkpoints. Use one entry point from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- list
```

## Lesson source

`journey/course.json` holds the cumulative order, teaching, estimates and exact edits. Snippets live in `journey/code`. A replacement must match one earlier block. The tool reconstructs the project from these same snippets; init supplies the dependency lock only.

The modules under `session_tests/scripts/course` have distinct jobs: `model.ts` replays source; `checkpoints.ts` preserves learner files; `render.ts` writes the lesson pages; `verify.ts` invokes Cargo and records evidence. The native Rust harness checks pixel values. The small PNG writer only packages those measured pixels for documentation.

Every lesson needs a visible goal, named owners and values, a prediction experiment, a question and a complete runnable checkpoint. The full viewer is the required destination. Planned lessons must remain visibly unfinished until their cumulative implementation and checks exist.

## Verify a change

```sh
npm --prefix ../session_tests run course -- generate
npm --prefix ../session_tests run course -- structure
npm --prefix ../session_tests run course:test
npm --prefix ../session_tests run course -- verify
npm --prefix ../session_tests run course -- parity
```

For prose-only changes, `verify --stored` checks the recorded source fingerprints without repeating Rust builds. `verify-reference --stored` validates all 43 existing reference checkpoints against their earlier build and render evidence. A source comparison is not a runtime result.

The runtime verifier uses build slots, four Cargo workers, a memory cap and a timeout. Generated reference projects stay under `target`; learner files remain under `workspace/journey`. The Rust pixel checks reject an empty or wrongly coloured result and check the input-state round trip.

## Preserve the destination

`journey/destination.json` fingerprints the current source and key inputs, maps the 43 existing sections to courses and lists quality requirements. `structure` fails on missing or changed source. Review an intentional change before updating the baseline. `parity` independently compares final reference Rust/WGSL with production after removing teaching comments.

The earlier reconstruction in `typing/manifest.json` remains readable without Python. The TypeScript model replays every addition and its tests compare all final file hashes. Its large sections are reference material while the complete smaller sequence is developed. Do not connect incompatible checkpoints through Next links.

The old Python lesson generators and checkers have been retired. `structure` now compares every reference insertion and displayed snippet with all 43 stored source snapshots. Edit the new cumulative sequence for teaching changes; the large reference is retained as a checked destination during conversion. `mkdocs.yml` contains navigation data only. Both documentation launchers build or serve Vue.

## Browser evidence

```sh
npm --prefix ../session_tests run course -- serve
```

Then, in another terminal with Chrome and Playwright installed:

```sh
npm --prefix ../session_tests run course -- capture
npm --prefix ../session_tests run course -- generate
```

The capture tool checks actual startup, mouse and keyboard input, canvas results and the pixel round trip. Native rendering cannot substitute for it. Pages include browser captures only when their source fingerprints match. Otherwise they identify native readbacks explicitly.

## Foundations and diagrams

```sh
npm --prefix ../session_tests run course -- foundations
npm --prefix ../session_tests run course -- diagrams --check
```

Foundation examples are Rust programs with assertions. Flow diagrams use D2 0.9.0 from `target/tools/d2-v0.9.0/bin/d2`; the TypeScript command renders or verifies them. Edit D2 sources, not generated SVG text positions.

## Build the Vue site

From `session_tests`, run `DOCS_STRICT_LINKS=1 DOCS_BASE=/docs/ npm run build -- --outDir ../session_viewer/target/docs/vue --emptyOutDir`, then build the viewer with Trunk. The strict build resolves source includes and checks page links. The sidebar keeps the entry lessons visible and the long reference material collapsible.

To review that compiled documentation alone, run `./docs/serve.sh preview` from `session_viewer`. Open `http://127.0.0.1:8788/docs/#/course`. A lesson has a stable URL such as `http://127.0.0.1:8788/docs/#/course/journey/18-light`. The launcher stops the existing TCP listener on the selected port and waits for it to release the port before starting Vite. `--port NUMBER` overrides the default. It needs `lsof` and permission to stop the listener and listen on localhost.
