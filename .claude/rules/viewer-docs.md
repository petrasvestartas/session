---
paths:
  - "session_viewer/docs/**"
  - "session_viewer/src/**"
---

# Viewer lesson docs (session_viewer/docs/NN-*.md)

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
- Flowcharts are D2, not Mermaid: the source is `docs/diagrams/<lesson>-<n>.d2` and the rendered
  SVG is committed beside the illustrations. `python3 docs/diagrams.py` re-renders what changed
  (`--all` everything, `--check` fails when a committed SVG is stale). It fetches its own pinned
  d2 into `target/tools/` the first time, so a fresh checkout needs one command and a network
  connection once. Edit the `.d2`, never the `.svg`.
- Every step opens with the viewer map, and a compact copy of it stays pinned while the reader
  scrolls: `python3 docs/locator.py` regenerates both plus the per-code-block zone marks the
  pinned bar follows (`--check` fails when a lesson is stale). It refuses to run when a taught
  file matches no zone, so a new top-level path means adding it to `ZONES` there.
- `python3 docs/check_svg.py docs/illustrations/*.svg` is the playwright-free check: it renders each
  SVG in an installed Chrome and fails on a label that leaves the canvas, overlaps another, or is
  too close in colour to the shape it sits on (below 3:1, found by paint order and exact fill
  containment - `Canvas.raw()` rewrites white to near-black for the dark page, which silently turns
  ink on a light box into ink on a black one). It exits non-zero. Use it when
  `check_illustrations.cjs` cannot run. NOTE: running `draw.py` REWRITES every SVG and strips the
  measured `textLength` pins; restore them with
  `git checkout -- $(grep -l textLength docs/illustrations/*.svg)` before committing, or re-pin the
  regenerated set with `python3 docs/check_svg.py --write docs/illustrations/*.svg`. `--write` skips
  a drawing its generator owns (the d2 diagrams, `map.svg`, `locator-*`, `strip-*`): those are
  verified by byte comparison, so a pin would make them report stale for good.
- Illustrations come from `docs/illustrations/draw.py` (BRG Equilibrium palette; boxes sized
  from text). Never hand-place SVG text: regenerate, then `node docs/check_illustrations.cjs
  --write` must PASS (real Chrome metrics, no overflow, no collisions, pinned textLength).

