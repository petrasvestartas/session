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
- A lesson is a SUBJECT, not a commit. Its job is to build one part of the viewer completely, in
  final form, in the place that part belongs. The course is a decomposition of the finished
  viewer by subject, never a replay of the order the work happened in.
- WRITE ONCE. A lesson never rewrites, re-opens or patches code an earlier lesson taught; it only
  adds new files and appends to files it already owns. A step anchor that says "Replaces …"
  against a region an earlier lesson wrote is the defect this rule exists to prevent - the reader
  is being marched back through a file they already finished. The check is mechanical and must
  read zero: `grep -c Replaces docs/*.md` counting only anchors that name an earlier lesson.
- A new feature goes where its subject already lives, and EXTENDS that lesson: `Attributes On|Off`
  is a command, so it belongs in the command-line lesson, not in a lesson of its own at the end;
  a per-object SSAO radius belongs in the shading lesson; a phone keyboard belongs in the input
  lesson. Only a genuinely new subject - one no existing lesson owns - earns a new lesson, and it
  is inserted at its place in the dependency order, not appended because it happened last.
- This only holds if the code allows it: a feature must be one file plus one registration line, so
  a later lesson can add it without touching what an earlier lesson wrote. When a feature cannot
  be added that way, fix the architecture (a registry, a pass list, a command list), do not fall
  back to teaching an edit of an earlier lesson's file.
- Page prose: one plain sentence per step saying what the step builds, under 25 words. Example:
  "Append: create the GPU connection in four steps, instance → surface → adapter → device and
  queue." The teaching happens in the code comments, not here, so the page never repeats them.
  Label "type this" / "copy the file" deliberately. Tests, examples and parity ports are listed
  as files to copy from the lesson crate, never explained.
- Code comments in the lesson crates: comment the CONCEPT, once, where it first appears; say
  nothing anywhere else. A concept's first appearance earns a real sentence, 15-25 words, that
  answers what the thing IS and why it exists - the part a reader cannot recover from the code.
  The model is the reader's own lesson 01 (`session_view/src/lib.rs`): "The GPU does not execute
  calls one by one, you record a list of commands and hand over this encoder.", "Create an
  instance of the struct; Ok is needed to return also the error message Err(...).", "Convert the
  logical size to real pixel size, e.g. width = 800, scale = 2.0." Rust and wgpu mechanics
  (`Ok(Self)`, `?`, a second `impl` block, a borrow ending at a brace) ARE the lesson: explain
  them. A concrete example with numbers beats an abstract phrase. A step may open with a short
  glossary of the nouns it introduces, one `Noun = plain meaning` per line.
  NEVER restate the identifier: `create_view` does not earn "a view of that texture"; if the only
  thing a comment can say is the name again, delete it. Struct fields, obvious calls and
  self-evident literals stay bare - a reader who met the concept two steps ago does not need a
  label on every use. Still one line per comment, still no rationale paragraphs.
  Production `session_viewer/src/` keeps the terse form: one short line only where a line is not
  self-evident, no teaching, since it is read by someone who already knows the viewer. Where a
  line exists in both, the lesson's teaching comment is the longer one and they may differ.
- The only site check is `docs/serve.sh build` (MkDocs with `check_paths: true`: a snippet that
  names a missing file fails the build). The only code check is `cargo check -j4 --lib` inside
  the lesson directory, one lesson at a time.
- The site is the Vue app in `session/session_tests` (course, Kernel API, install), not MkDocs.
  Pages are plain Markdown plus `--8<--` includes, which its build plugin resolves; no MkDocs-only
  syntax (no `!!!` admonitions, `===` tabs, `{: }` attribute lists or Material icons). The
  viewer's black folded-corner link opens `docs/`: Trunk's pre-build hook
  `session_tests/scripts/build-viewer-docs.sh` builds the app into `target/docs/vue` when a
  source changed and Trunk copies it to `dist/docs`; Pages builds it for `/session/docs/`. Once
  MkDocs is removed the site check becomes `npm run build` in `session_tests`, which fails on a
  missing include or a broken link.
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
