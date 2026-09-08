# A text lane: screen-readable labels, and the road to PDF text

2026-09-08. Decisions taken with the user: glyphs come from a pre-baked signed-distance atlas
asset (no font crate in the viewer); the first use is object labels on selection; the kernel
change that lets PDF lettering reach the lane is planned now as its own phase.

## 1. Goal

- Text that stays the same size on screen at every zoom and every distance, anchored to a
  world point, antialiased, never occluded by the ink rules (a label is a HUD element).
- Selecting an object shows its name beside its box; `T` toggles labels for every object.
- No new Rust dependency: the atlas and its metrics are assets baked once by a script.
- Later: PDF lettering arrives as text (string, font, size, matrix) and the lane renders it
  with an atlas baked from that document's own font, falling back to today's outline meshes.

Non-goals now: rich text, right-to-left, kerning beyond the metrics file, text along curves.

## 2. Facts the design rests on

- The PDF importer (`session_rust/src/pdf.rs`) keeps neither strings nor font names; a
  lettering mesh is named `"text"` and drawn last through the sheet pipeline
  (`ArenaLane::draw_text`). It dumps each embedded font program next to the file; those are
  the `.cff`/`.ttf` files under `session_viewer/assets/fonts/`, unused by any code.
- No lane binds a sampler today; every texture read is `textureLoad`. The atlas is the first
  sampled texture and the first `SamplerBindingType` in `layouts.rs`.
- `sphere.wgsl` already sizes a camera-facing quad in pixels (`to_px`) and puts it back into
  clip space with `* 2.0 / vec2(vp_w, vp_h) * clip.w`; a glyph quad is the same construction.
- The perf line is DOM text over the canvas (`performance.rs`); labels stay in the canvas so
  they pick, occlude and export like everything else.

## 3. The atlas asset

- `docs/_bake_atlas.py` (standard library plus PIL, which the harness already uses) renders
  the printable ASCII range of `assets/fonts/JetBrainsMono-Regular.ttf` at 64 px into cells,
  computes a signed distance field per cell (8-connected sweep, clamped to 8 px), and writes
  `assets/text/mono.sdf` (raw R8, width and height in the JSON) and `assets/text/mono.json`
  (cell size, per-glyph atlas rect, advance, bearing, and the font's ascent). Raw R8 needs no
  decoder in wasm; the file is about a quarter megabyte at 512 x 512.
- The same script bakes any TTF or CFF the user names (`--font`), which is how PDF fonts get
  their atlas in phase C. CFF files are converted with the system's fontTools when present;
  otherwise the script reports that the font needs conversion and stops.
- The viewer fetches both files through `app/fetch.rs` at boot (native harness: from disk),
  uploads the R8 texture once, and keeps the metrics in `TextFont { cell, glyphs: [Glyph; 95],
  ascent }`.

## 4. The lane

- Row: `TextGlyph` (48 B): anchor world position (vec3), offset in px (vec2, from the label's
  origin to this glyph's cell origin), size in px, atlas rect (u0 v0 u1 v1 as f32), colour
  RGBA8, instance id, flags. One row per glyph; a label is a run of rows.
- `engine/gpu/text.rs`: a `GrowBuf` of rows, the atlas texture and sampler in group 3
  (binding 0 rows, 1 texture, 2 sampler), `draw_labels` after the dots in `scene_list` and in
  `id_pass` (a label picks its object). Rows are rebuilt when the selection or the `T` state
  changes: the table is `reset` and re-appended, which is the one lane allowed to do so, since
  its rows are derived, not authored.
- `shaders/text.wgsl`: vertex pulls the row by `vertex_index / 6`, projects the anchor, adds
  `offset * dpr` in pixels plus the quad corner scaled by `size`, converts back to clip at the
  anchor's `w`, and passes the atlas uv. Fragment samples the SDF with linear filtering and
  shades `smoothstep(0.5 - aa, 0.5 + aa, d)` with `aa = fwidth(d)`; no depth test (labels sit
  on top), alpha blended, colour from the row. `fs_id` writes the instance id.
- Anchor and layout: a label's anchor is the top corner of the object's world box nearest the
  camera; the layout is a single line, left to right, baseline at the anchor, 12 px at DPR 1
  (`?label=` px knob), with a 2 px dark halo from a second, offset pass or a wider SDF band so
  text reads on any fill.
- `app/walk/labels.rs`: `label_rows(name, anchor, font, pen) -> Vec<TextGlyph>` from the
  metrics; `State::relabel()` builds rows for the selection, or for every object under `T`
  (capped at the 2000 nearest objects to the camera, measured before shipping).

## 5. Phases

A. Atlas script and asset, the lane, selection labels and `T`, picking, the knob, tests: a
   metrics test (advance sums), a naga mirror test for the row, and a harness render that
   counts label pixels at two zooms (equal within the AA fringe).
B. Kernel: a `Text` geometry (guid, name, string, font, size, plane or matrix, colour) in
   `session_proto`, `session_cpp` (ground truth), `session_rust`, `session_py`, with the
   minitest set every class carries (ctor, accessors, transform, json and proto round trips,
   file dump/load). The PDF importer emits one `Text` per text run alongside the outline mesh
   it emits today (the mesh keeps exact print fidelity; the text keeps meaning).
C. Viewer: `walk_text` renders `Text` through the lane when an atlas for its font exists
   (baked from the dumped font by the same script, keyed by font name), hides the paired
   outline mesh, and falls back to the mesh otherwise; `?pdftext=outline|atlas` chooses.

## 6. Verification

- Phase A: the harness renders a labelled scene at fit and at 8x zoom; label pixel counts
  equal within 5% (measured, then written); the id pass picks the object through its label;
  `cargo xtest` mirror test for `TextGlyph`; the ink suite unchanged.
- Phase B: minitests in three languages, identical names and logic; a PDF fixture whose text
  run count matches the importer's `st.fonts.glyphs` tally.
- Phase C: the same sheet rendered both ways; the atlas path's text lands within one glyph
  cell of the outline path's bounds, measured on one sheet.
