# Maintaining the viewer course

The viewer and its executable checks are Rust. The Vue project's TypeScript tooling assembles lesson edits, renders Markdown, checks coverage and manages learner checkpoints. Use one entry point from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- list
```

## Lesson source

The learner starts with Rust and wgpu from scratch and ends with the maintained viewer's production tree and behavior. Organize the route into the fewest coherent subject lessons: one visible result and runnable checkpoint per subject. Keep small typing instructions inside that lesson. Do not reduce the count by hiding a second implementation, adding a final source dump, or linking to the maintained runtime instead of completing the learner's program.

Teach each concept once, in its final home, with lean code and source includes rather than duplicate listings. The mathematics slide belongs immediately after the first egui painting; its scene adapter belongs after camera, rows, strokes, markers and loading. Coupled modules need an explicit dependency design before their checkpoints are advertised as runnable.

The old numbered pages are in `archived/`, with URL redirects and a separate navigation group. `lessons/` and `typing/` are retained fixtures for historical reconstruction checks. Passing those checks does not establish current-viewer parity.

`journey/course.json` holds the cumulative order, teaching, estimates and exact edits. Snippets live in `journey/code`. The tool reconstructs the project from these same snippets; init supplies the dependency lock only. Its ability to replay a replacement is a storage mechanism, not permission to teach successive rewrites of a finished subject.

The modules under `session_tests/scripts/course` have distinct jobs: `model.ts` replays source; `checkpoints.ts` preserves learner files; `render.ts` writes the lesson pages; `verify.ts` invokes Cargo and records evidence. The native Rust harness checks pixel values. The small PNG writer only packages those measured pixels for documentation.

Every lesson needs a visible goal, named owners and values, a prediction experiment, a question and a complete runnable checkpoint. The full viewer is the required destination. Planned lessons must remain visibly unfinished until their cumulative implementation and checks exist.

## Writing contract

Follow `session/.claude/rules/viewer-docs.md`. A lesson presents the current implementation in this order: goal, prerequisites and owned files, build instructions, run and observe, prediction experiment, question and answer, validation. Each concept is explained once. Later subjects add modules and declared registrations; they do not ask the learner to replace completed implementations.

Correct the owning lesson and its checkpoints when an implementation changes. Keep the correction history in an audit or commit when needed, outside the learner's route. Agent instructions state the current policy once, without a trail of superseded instructions.

Apply this contract to the existing course too. Merge repeated implementations and repair steps into their subject owners; archive superseded material. Measure reduction by executable subject checkpoints, not sidebar headings. A separate checkpoint must introduce a distinct coherent subject and visible result.

Keep normal changelog entries to one to three bullets and at most 100 words: resulting behavior, relevant validation, and a material remaining limitation if present. Explain historical context only when it justifies a present design or compatibility constraint. Omit routine command logs and abandoned approaches.

## Verify a change

```sh
npm --prefix ../session_tests run course -- generate
npm --prefix ../session_tests run course -- structure
npm --prefix ../session_tests run course -- ownership
npm --prefix ../session_tests run course:test
npm --prefix ../session_tests run course -- verify CHECKPOINT
npm --prefix ../session_tests run course -- parity
```

For prose-only changes, `verify --stored` checks the recorded source fingerprints without repeating Rust builds. `verify-reference --stored` validates all 43 existing reference checkpoints against their earlier build and render evidence. A source comparison is not a runtime result.

`ownership` checks existing checkpoints against their subject owners and fails when a later subject changes an earlier subject's implementation. Use `ownership --details` to locate each rewrite. Additive module declarations are reported separately. Passing this textual check still requires review of subject boundaries and executable checkpoints; it does not establish production parity.

`subject-audit ID` collapses a book subject's intermediate edits into one final edit per changed file and verifies its exact cumulative endpoint. It writes no course files. `subject-audit --all` checks every book subject; use the resource guards for that full audit. These candidates still require authored explanations, ownership review, builds and browser acceptance before replacing active checkpoints. Preserving an endpoint does not prove that later subjects leave its files finished.

Select checkpoint IDs explicitly; use `verify --all` only for an intentional full sweep. The runtime verifier currently requests one Cargo worker and applies a memory cap and timeout. Run heavy commands through the superproject storage guard and build slot. Generated reference projects stay under `target`; learner files remain under `workspace/journey`. The Rust pixel checks reject an empty or wrongly coloured result and check the input-state round trip.

## Preserve the destination

The inventory selected by `journey/course.json` fingerprints source and key inputs. `structure` fails on missing or changed source. Review an intentional change before updating the baseline; a refreshed hash is not evidence of teaching or execution. The current `parity` command still compares historical lesson 37 with production after removing teaching comments, and fails on the newer viewer. It must not be reported as a comparison of the journey endpoint.

The earlier reconstruction in `typing/manifest.json` remains readable without Python. The TypeScript model replays every addition and its tests compare all final file hashes. Its large sections are reference material while the complete smaller sequence is developed. Do not connect incompatible checkpoints through Next links.

The old Python lesson generators and checkers have been retired. `structure` now compares every reference insertion and displayed snippet with all 43 stored source snapshots. Edit the new cumulative sequence for teaching changes; the large reference is retained as a checked destination during conversion. `mkdocs.yml` contains navigation data only. Both documentation launchers build or serve Vue.

## Browser evidence

The [November 12 coverage audit](journey/coverage-audit-2026-11-12.json) distinguishes source references, exact edit paths and verified teaching. It records the working tree used for the audit; do not treat its unresolved statuses as completed coverage or silently refresh its source hashes.

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
