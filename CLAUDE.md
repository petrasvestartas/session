# CLAUDE.md

Multi-language geometry kernel (Python, C++, Rust) with shared protobuf schemas and Vue test viewer.

## Style
- Least verbose, no print messages, no excessive comments
- C++ is ground truth — port to Rust and Python with identical APIs, variable names, test logic, line counts

## Structure
```
session_cpp/     # C++ (submodule)
session_py/      # Python (submodule)
session_rust/    # Rust (submodule)
session_proto/   # Protobuf schemas
session_tests/   # Vue 3 test viewer
```

## Build & Test
```bash
./bash/minitest.sh --py --no-web      # Python only (fastest)
./bash/minitest.sh --rust --no-web    # Rust only
./bash/minitest.sh --cpp --no-web     # C++ only
./bash/minitest.sh                    # All + viewer at localhost:8769
./bash/quicktest.sh <class> --py      # Single class test
./bash/git_push.sh "message"          # Push all submodules
```
Dev order: Python → Rust → C++. Use `/build` command for full reference.

## Git
- NEVER add Claude/AI as git contributor, author, or co-author
- The daily kernel-audit action (05:00 Europe/Zurich) opens ONE review-ready PR per kernel
  submodule and merges nothing — you approve and merge, then bump the parent pointers.
  At session start and after merging an audit PR: `git pull` in the parent and `git submodule update --remote`
  (or pull inside session_cpp/py/rust) before editing kernel code.
- After EVERY push (parent or submodule): watch ALL workflows that push triggered to
  completion — parent `Session mini tests` included — `gh run list`/`gh run watch`; a push is
  not done until its runs are green. Never split a parent CI-config change and the submodule
  pointer bump it depends on into two commits (the intermediate commit is a guaranteed-red run).
- A submodule pointer bump now triggers `viewer-check` (it compiles the viewer on wasm + native
  and runs `cargo xtest`, and gates the Pages deploy) — watch it too.
- Check CI: `gh run list --limit 5`, failures: `gh run view <id> --log-failed`
- CI: macOS-15 (ARM64), manylinux_2_28 (Linux), chmod +x bash scripts in CI

## Minitest Rules (essentials)
- Tests identical across all 3 languages (names, logic, line count)
- One test per API method; constructor test groups: ctor, [], ==, !=, str, repr
- JSON fields alphabetically ordered across all languages
- Every class needs: file_json_dump/file_json_load + to_proto/from_proto tests
- Operators go inside constructor test, not separate tests
- Method order: constructors → accessors → mutators (*_self) → operators → utilities → serialization → str/repr
- Use `/test-rules` command for full import patterns and conventions

## Code Style
- Python: 3.9 is the floor — Rhino 8's embedded CPython (`~/.rhinocode/py39-rh8`, see bash/install_rhino.sh). Style stays modern — `X | None`, builtin generics (`list[str]`), never `typing.Union/Optional/List` — legal on 3.9 because `from __future__ import annotations` heads EVERY module (mandatory: PEP 604 unions are 3.10+ at runtime, and it also neutralizes the house `str` property shadowing builtins in class-body annotations). protobuf stays on the 6.x line + grpcio-tools 1.80.0 (newest with 3.9 wheels; gencode must match runtime). One import per line. TOLERANCE/PI from `.tolerance` at top of file. Geometry imports inside test functions. Use flat imports: `from session_py import Line, Plane` not `from session_py.line import Line`. Exception: `from session_py.intersection import line_line`.
- C++: never `#include "tolerance.h"` in production code. Use `std::cout << point` not manual coords.
- Rust: `use crate::tolerance::{TOLERANCE, PI};` at top. Geometry imports inside MINI_TEST blocks.

## Viewer Lesson Docs (session_viewer/docs/NN-*.md)
- Lesson code is NEVER written in the Markdown. A lesson is prose plus directives that
  `docs/course_pages.py` expands from the verified checkpoint patches under
  `docs/reconstruction/`: `<!-- file: NN path [type|copy] [hunks=..] [lines=A-B] [whole] -->`
  renders CURRENT / REPLACE WITH, CURRENT / ADD BELOW, NEW FILE or DELETE blocks;
  `<!-- supplied: NN -->`, `<!-- tree: NN prefix -->`, `<!-- listing: NN path -->`,
  `<!-- check: NN -->`, `<!-- checkpoint: NN -->` are the other directives.
- Prose is bullets, one idea per step, code first; a block says WHAT, prose says WHY. Never
  narrate an edit. Label TYPE THIS / COPY deliberately. Numbers only when visible in shown code.
- Tests are not documented in docs/: test files, examples and parity ports are "supplied"
  (installed by `replay.py --copy-supplied`), never explained.
- `python3 docs/course_pages.py --audit` must pass: every checkpoint change taught or supplied
  exactly once, and typing the lesson literally reproduces the checkpoint hashes. Every
  `<!-- check: NN -->` marker must be recorded by `docs/reconstruction/compile_points.py`
  (it runs cargo check on the typed state; one cargo process at a time, -j4).
- Each lesson also states, before its first step, which of its steps compile. That sentence is
  generated: `docs/reconstruction/step_checks.py` cargo-checks the typed state at the end of
  EVERY step (CARGO_INCREMENTAL=0 - 195 distinct states otherwise fill the disk) into
  `step-checks.json`, and `step_status.py` renders it between the `step-status` markers.
  `step_status.py --check` fails when a lesson is stale; re-run both after editing steps.
- Changing viewer source that the final checkpoint covers: reconstruct it
  (`replay.py --output <new> --through 18`), then `docs/reconstruction/refreeze.py --workspace <new>`
  folds the production diff into patch 18, series/baseline hashes and the tree hash; add a
  directive in lesson 18 for taught files; then audit + compile points + `docs/serve.sh build`
  + `docs/check_site.py` + `converge.py --workspace <fresh replay> --production .`.
- The viewer page carries a black folded-corner link to `docs/`; Trunk copies `target/docs/site`
  into `dist/docs` and `docs/build_site.sh` (pre-build hook) rebuilds the site when stale.
- Mermaid: `flowchart TB` for chains longer than five nodes (LR gets shrunk to unreadable size);
  several small diagrams beat one tangled one; edge labels stay short.
- `python3 docs/check_svg.py docs/illustrations/*.svg` is the playwright-free check: it renders each
  SVG in an installed Chrome and fails on a label that leaves the canvas or overlaps another. Use it
  when `check_illustrations.cjs` cannot run. NOTE: running `draw.py` REWRITES every SVG and strips the
  measured `textLength` pins; restore them with
  `git checkout -- $(grep -l textLength docs/illustrations/*.svg)` before committing.
- Illustrations come from `docs/illustrations/draw.py` (BRG Equilibrium palette; boxes sized
  from text). Never hand-place SVG text: regenerate, then `node docs/check_illustrations.cjs
  --write` must PASS (real Chrome metrics, no overflow, no collisions, pinned textLength).

## Custom Commands
- `/new-class <name>` — full checklist for adding a new geometry class
- `/build` — all build/test/git commands
- `/test-rules` — detailed minitest conventions and import patterns
- `/decompile` — Rhino reverse engineering reference

## Reference Files
- `.claude/skills/` — language-specific templates for common patterns
- Invocable skills (session-viewer-wgpu, session-comments, session-format,
  session-polyline-rectangle) live in github.com/petrasvestartas/skills - clone it into the
  `.claude/skills` of the repo you work in, or into `~/.claude/skills` for every repo at once
- `SKILLS_RHINO_GEOMETRY.md` — 711 C exports + ~7100 C++ methods from Rhino
- `SKILLS_RHINO_DECOMPILE.md` — Ghidra decompilation guide
