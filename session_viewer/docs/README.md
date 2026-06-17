# Session Viewer — Build Log (docs)

A standalone, self-contained site for the step-by-step lessons that build
`session_viewer` up to the full CAD viewer (`session_viewer_archive`). It has **no
build step** and **no dependency on `session_tests`** — just a static page plus a tiny
Python server that lists the lessons.

## Run

```bash
cd session_viewer/docs
python serve.py          # → http://localhost:8771
```

Open it in a real browser. The left sidebar lists every lesson; click to read.

## How it works

- `index.html` + `app.js` — the page. Renders Markdown with `marked` and highlights
  code with `highlight.js` (both from CDN).
- `serve.py` — serves this folder and generates `/sections.json` on the fly by scanning
  `*.md`. Files starting with `_` (e.g. `_TEMPLATE.md`, `_ROADMAP.md`) are ignored.
- Lessons are plain `NN-title.md` files. The first `# Heading` is the sidebar label;
  the `NN` prefix sets the order.

## Add a lesson

1. Copy `_TEMPLATE.md` to `NN-title.md` (next number).
2. Write Markdown; fenced code blocks get highlighted (` ```rust `, ` ```bash `, …).
3. Refresh — no manifest or rebuild needed.

## Snapshots

`NN_title/` folders (e.g. `03_window/`, `04_pipeline/`) are full standalone crate
snapshots of the viewer at the end of that lesson — copy one out and `trunk serve` it
to see exactly that chapter's result. `_ROADMAP.md` is the full curriculum plan.

## Live viewer

To see the viewer you're building running live, in another terminal:

```bash
cd session_viewer && trunk serve     # → http://localhost:8770
```
