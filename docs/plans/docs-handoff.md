# Documentation handoff (for a cloud session)

Written 2026-09-26 by the local viewer session (wood-research-75) so a cloud session can finish the documentation if the local usage budget runs out. Work only from git: the local machine, its caches and its GPU are not available to you.

## Setup

- `git submodule update --init session_cpp session_py session_rust session_proto session_data`
- Rust stable, Node 20+, Python 3.12. Build jobs: `cargo -j8`, never `-j$(nproc)`.
- Commit with plain messages, no Co-Authored-By or other AI trailers. Push to `main` after every finished piece (petras wants to see the tutorials update live). Watch CI with `gh run list` until green.

## Goal

ONE documentation site, built on the Vue app in `session_tests/` (not MkDocs), served at `https://petrasvestartas.github.io/session/docs/` and opened by the viewer's black bottom-right corner triangle (`#viewer-docs` in `session_viewer/index.html`, `href="docs/"`). It has:
- Course: the viewer tutorial, lessons 00 to 37 plus 04a-d and lettered lessons (e.g. 18a instancing, 18b clipping, 23a-d commands), pages `session_viewer/docs/NN-*.md`, code in lesson crates `session_viewer/docs/lessons/<id>/`.
- Kernel API: the existing Tests view (each kernel class, every minitest in C++, Python and Rust side by side).
- Install.

## Rules

- Course rules: `.claude/rules/viewer-docs.md`. One write-once chain: lesson N+1 = lesson N plus only added files and appended or `// register:` lines; lesson 37 equals `session_viewer/src`; a feature goes in the lesson that owns its subject.
- Pages: plain markdown plus `--8<--` snippet includes of named sections (`// --8<-- [start:x]` / `[end:x]`). No MkDocs-only syntax: no `!!!` callouts, no `===` tabs, no `{: attrs}`.
- Voice: plain teacher, short. Only non-obvious lines get a comment; each technical term is defined once where it first appears, in few natural words; never restate an identifier. Page prose: one sentence per step, under 25 words.
- Look: system fonts, black text, grey rules, white background. No coloured accents, no decorative tables.

## Find out what is left (do not trust any status list; measure)

- `python3 session_viewer/docs/check_write_once.py`: must report 0 violations.
- `python3 session_viewer/docs/check_lesson37.py`: must pass.
- `grep -c "Replaces\|Added after the line" session_viewer/docs/*.md`: must be 0.
- Every lesson crate: `cd session_viewer/docs/lessons/<id> && cargo clean -p session_viewer && cargo check -j8 --lib` (all crates share one package name, so clean first or use a separate target dir per crate).
- Vue site: `cd session_tests && npm ci && npm run build` must pass with no unresolved snippet includes; if the Vue course plugin does not exist yet, build it (plan in "Vue site" below).
- `git log --oneline -30` shows what was already pushed ("Viewer course, wave N", "Vue docs", ...).

## Remaining work, in order

1. Finish the write-once re-cut of the course (the checks above at zero), pushing after each group of lessons.
2. Vue site: course pages rendered from the markdown (Vite plugin resolving `--8<--` includes and kernel line ranges, failing on a missing one), lazy route per lesson, nav from `docs/lessons/SERIES.txt`, previous/next, table of contents, copy buttons, images; one search over course and kernel API; home page with Course and Kernel API; corner link back to the viewer; plain look.
3. One Pages deploy: the viewer dist, the kernel test-result JSONs (artifact from the "Session mini tests" workflow) and the Vue build under `/docs/`. Only one workflow deploys Pages.
4. Remove MkDocs (`session_viewer/mkdocs.yml`, `docs/hooks`, `build_site.sh`, `serve.sh`, the Trunk copy of `target/docs/site`) once the Vue site renders every page; update `.claude/rules/viewer-docs.md` so the check is `npm run build`.
5. Reference pages in the Vue site: every viewer command in Title Case with options and phone use; ARCHITECTURE and README; link the kernels' `docs/history.md` undo pages.
6. Illustrations and diagrams redrawn in black and grey (`session_viewer/docs/illustrations/draw.py`, D2 sources under `docs/diagrams/`).

## Done when

All checks above pass, `https://petrasvestartas.github.io/session/docs/` shows the Course and the Kernel API, the viewer's corner triangle opens it, and CI is green.
