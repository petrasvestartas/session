---
paths:
  - "session_viewer/docs/**"
  - "session_viewer/src/**"
---

# Viewer lesson docs (session_viewer/docs/NN-*.md)

- Every checkpoint of the course is a complete, runnable crate under
  `session_viewer/docs/lessons/<id>/`. The course is ONE chain, `00` to `37` (with `04a-d`):
  one starting point, one ending point, and `37` equals the maintained viewer. A new feature
  becomes the next lesson at the end; there are no branches or alternate tracks.
  `docs/lessons/SERIES.txt` is the parent map: `<id> <parent-id> <page>`. Each crate depends on
  the live kernel through `session_rust = { path = "../../../../session_rust" }` and carries an
  empty `[workspace]`; `cargo check` / `trunk serve` inside the directory must just work.
- Lesson code is NEVER written in the Markdown. A step is one reference line
  (`` `lessons/01/src/lib.rs` · type this, append at the end of the file ``) followed by a
  `pymdownx.snippets` include of a named section of the real file:
  ```` ```rust
  --8<-- "lessons/01/src/lib.rs:step-2"
  ``` ````
  The section is fenced in the crate by two comment lines, `// --8<-- [start:step-2]` and
  `// --8<-- [end:step-2]` (`#` in TOML/YAML/sh, `<!-- -->` in HTML outside script), stripped
  from the rendered page. Those markers are the only link between page and crate: keep them when
  editing, move them with the code, and a missing one fails the site build by name. Only the two
  kernel listings (`--8<-- "session_rust/src/color.rs:171:173"`) still use line numbers, because
  the kernel is never annotated. The kernel is included through the `..` base path, never copied
  into a lesson. There is no generator: a change to step N is applied to every later crate by
  hand (`diff -r docs/lessons/N docs/lessons/N+1` shows what each step owns).
- Page prose: ONE plain sentence per step, under 25 words, no parenthetical asides, no
  cross-references to other steps or lessons. It says what the block does, not how Rust or wgpu
  works. Example: "Append: create the GPU connection in four steps, instance → surface → adapter
  → device and queue." Label "type this" / "copy the file" deliberately. Tests, examples and
  parity ports are listed as files to copy from the lesson crate, never explained.
- Code comments, in the lesson crates AND in production `session_viewer/src/`: every line a
  first-time reader would ask "what is this?" about gets a comment: on the right for a field or argument, one line above for a statement. Under ten
  words, plain words, present tense, no jargon the code does not already show. Say what the
  thing IS or DOES, never why the API is designed that way. Good: `width: 1, // canvas size in
  pixels`, `// no depth buffer yet`. Bad: anything that needs a second sentence, a "because",
  or names a later lesson. Each step starts with one heading comment naming its part. A
  comment must survive two readings: first as its author, then by a fresh agent with no
  context (`Explore` or a plain fork) asked to explain the line from the comment alone; a
  comment it cannot explain is rewritten, not extended. No comment of any kind spans more than one line, `///` included; a paragraph of
  rationale is deleted, not shortened, and the code's own names carry the meaning.
  A comment is the same text wherever the line exists: in production `src/` and in every lesson
  crate that contains it, so a note written in one place is copied verbatim to the others.
- The only site check is `docs/serve.sh build` (MkDocs with `check_paths: true`: a snippet that
  names a missing file fails the build). The only code check is `cargo check -j4 --lib` inside
  the lesson directory, one lesson at a time.
- The viewer page carries a black folded-corner link to `docs/`; Trunk copies `target/docs/site`
  into `dist/docs` and `docs/build_site.sh` (pre-build hook) rebuilds the site when stale.
- Mermaid: `flowchart TB` for chains longer than five nodes (LR gets shrunk to unreadable size);
  several small diagrams beat one tangled one; edge labels stay short.
- Flowcharts are D2, not Mermaid: the source is `docs/diagrams/<lesson>-<n>.d2` and the rendered
  SVG is committed beside the illustrations. `python3 docs/diagrams.py` re-renders what changed
  (`--all` everything, `--check` fails when a committed SVG is stale). It fetches its own pinned
  d2 into `target/tools/` the first time, so a fresh checkout needs one command and a network
  connection once. Edit the `.d2`, never the `.svg`.
- `python3 docs/check_svg.py docs/illustrations/*.svg` is the playwright-free check: it renders each
  SVG in an installed Chrome and fails on a label that leaves the canvas, overlaps another, or is
  too close in colour to the shape it sits on (below 3:1). It exits non-zero. Use it when
  `check_illustrations.cjs` cannot run. NOTE: running `draw.py` REWRITES every SVG and strips the
  measured `textLength` pins; restore them with
  `git checkout -- $(grep -l textLength docs/illustrations/*.svg)` before committing, or re-pin the
  regenerated set with `python3 docs/check_svg.py --write docs/illustrations/*.svg`. `--write` skips
  the d2 diagrams: those are verified by byte comparison, so a pin would make them report stale
  for good. `illustrations/map.svg` is a committed static drawing with no generator.
- Illustrations come from `docs/illustrations/draw.py` (BRG Equilibrium palette; boxes sized
  from text). Never hand-place SVG text: regenerate, then `node docs/check_illustrations.cjs
  --write` must PASS (real Chrome metrics, no overflow, no collisions, pinned textLength).
