# Viewer course handoff (for a cloud session)

Written 2026-09-26 by the local session. Work only from git: the local machine, its caches and its GPU are not available.

## What petras asked for (the requirements, in his words where possible)

- "I must be able from start to finish write the whole viewer." Typing every code block from the first page to the last gives exactly `session_viewer/src`.
- "One tutorial section must work on its own, and code typing must not exceed one hour of a newbie learner." The reader has never learnt WebGPU. Budget: at most about 250 typed lines per lesson (Rust + WGSL + toml/html, tests/examples/assets excluded), and every lesson builds and runs and shows something new.
- "Go step by step to the final viewer." Lesson 01 today adds 5,150 lines (whole final-version files); that is the failure to fix. Lesson 01 must be something like: open a window and clear it to a colour. Then a triangle. Then a camera. Small steps.
- "As little sideways tutorials as possible; never follow a tutorial and then have to drop the previous code." Every typed line survives unchanged into the final viewer (write-once).
- The voice: "the teacher who could teach the weakest student in the class - never verbose, extremely concise, every concept clear." Problem and idea first, concrete before general (numbers, "object 7 paints the value 7"), one idea per sentence, a term defined the moment it appears, step headings name the idea (not the file), a checkpoint "run it now, you should see ...", a 2-4 sentence recap.
- Push to GitHub after every finished lesson group, and watch EVERY CI run the push triggers (viewer-pages, viewer-check, Session mini tests) to green before starting the next group.

## Where things stand (measure, do not trust this)

- One chain of lesson crates `session_viewer/docs/lessons/<id>/`, ids in `docs/lessons/SERIES.txt` (00..37 with 04a-d, 18a, 18b, 23a-d). `docs/lessons/37` is the master (= src plus teaching comments and `// --8<-- [start:NN-slug]` markers); `docs/cut.py` cuts every earlier crate from it using `docs/lessons/FILES.txt` (file -> first lesson) and `REGISTER.txt` (`// register:` lines).
- Checks (all must stay 0): `python3 docs/check_write_once.py`, `python3 docs/check_lesson37.py`, `python3 docs/cut.py --check`, `python3 docs/check_complete.py viewer` (lines a lesson adds that its page does not show; 1,878 left in the local tree, 5,728 in git).
- Pages 00-03 were rewritten in the teacher voice (bacfb4fb) - use them as the voice model, not as the size model.
- Lesson sizes today (added viewer lines): 01 5,150; 04a 2,568; 04b 2,785; 06 3,830; 12 4,343; 14 3,437; 21 9,449; 23 6,076; 23a 6,296 - far over budget.
- The Vue docs site (`session_tests/`, `npm run build`) renders the pages at https://petrasvestartas.github.io/session/docs/ and fails on a missing snippet include.

## The work

1. Add a size check: `docs/check_budget.py` prints typed lines added per lesson (viewer code only) and fails above 250.
2. Re-plan the chain into small lessons that each build, run and show one new thing, in a straight line to the final viewer. Expect many more lessons than today (77,700 viewer lines / ~250 = about 300 lessons); group them into parts (e.g. Part 1 "First pixels": window, clear colour, triangle, buffer, camera, depth, mesh ...). Write the plan to `docs/plans/course-plan.md` first and push it.
3. Make that possible in `src`, behaviour-neutral only: split big files so an early lesson only needs a small piece (the core frame loop must not need picking, text, clipping ... - later features join through the existing registries: `// register:` lines, the verbs!/panels! macros, the PASSES list, `#include` in build.rs). Gates for any src change: `cargo test` (native, RUST_TEST_THREADS=4), `cargo xtest`, wasm `cargo check --target wasm32-unknown-unknown`, and the viewer-check CI run. No visual change allowed.
4. Re-cut with `cut.py` into the new ids, rewrite the pages lesson by lesson in the voice, and keep all checks at 0 plus `check_budget.py`. Tests, examples and assets are listed as download links, not typed.
5. After each part: commit (plain message, no AI trailers), push, watch all CI to green, then continue.
6. Afterwards (lower priority): remove MkDocs (`mkdocs.yml`, `docs/hooks`, `build_site.sh`, `serve.sh`), command reference page, ARCHITECTURE/README pages, illustrations in black and grey.

## Machine and git rules

- Build jobs: `cargo -j8` at most, one heavy build at a time. Initialise only what you need: `git submodule update --init session_rust session_proto` (the viewer depends on them).
- Never `git add -A`; stage named paths. Commit messages plain, no Co-Authored-By or other AI trailers.

## Done when

Every lesson passes `check_budget.py` (at most 250 typed lines) and builds and runs on its own; typing all lessons in order gives `src` (checks at 0); every page reads in the teacher voice; the site builds and CI is green.
