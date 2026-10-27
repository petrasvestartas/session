# Session Viewer Course Redraft (10 Main Sections)

This folder contains the 10-section Gemini Notebook redraft of the **Session Viewer** WebGPU tutorial course (`session_viewer/docs/journey`), preserving all **22 existing chapters**, **111 consolidated reader-facing lessons**, and **1,361 checkpoints** (`01-canvas` through `96zzzzzzzq-draw-original-cad-wire-curves` plus the reference comparison `97-finish-with-the-maintained-browser-runtime`), including custom inline `<svg>` WebGPU pipeline diagrams and the latest production `src/shaders/matrix.wgsl` shader.

## Deliverables in This Folder

1. **[`10-section-course-outline.md`](10-section-course-outline.md)** — Complete 10-section course outline in first-year student Gemini Notebook style (learning outcomes, custom SVG + ASCII/Mermaid WebGPU schematics, Rust/WGSL language bridges, absorbed chapters/lessons, rendering/geometry/interaction/runtime features, practical exercises, and verified vs. remaining status).
2. **[`old-to-new-coverage-map.md`](old-to-new-coverage-map.md)** & **[`old-to-new-coverage-map.json`](old-to-new-coverage-map.json)** — Complete 1:1 coverage mapping across all 10 sections, 22 chapters, 111 lessons, and all 1,361 checkpoint IDs, URLs, source snippets, and recovery commands.
3. **[`book-10-sections.json`](book-10-sections.json)** — Machine-readable 10-section `BookPlan` compatible with `session_tests/src/courseBook.ts`.
4. **[`sections/`](sections/)** — 10 standalone student section notebooks (`section-01-webgpu-and-command-dock.md` through `section-10-shared-gpu-precision-and-lighting.md`).
5. **[`lessons/`](lessons/)** — All 111 consolidated lesson pages redrafted with custom WebGPU `<svg>` diagrams, minimal prose, language bridge tables, and 100% preserved `<a id="checkpoint-...">` implementation blocks and `--8<--` source snippets.
