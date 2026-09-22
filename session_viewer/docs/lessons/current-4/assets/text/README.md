# Bundled text fonts

Noto Sans Regular, Noto Sans Symbols Regular and Noto Sans Symbols 2 Regular are distributed under the adjacent
SIL Open Font License 1.1 (`OFL.txt`). All are embedded in the WASM text document;
Trunk also copies these exact bytes for `text-quality.html`'s browser reference.
No machine-installed font discovery or asynchronous placeholder font is used.

Upstream sources:
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSansSymbols2/NotoSansSymbols2-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSansSymbols/NotoSansSymbols-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/LICENSE

`src/engine/text.rs` owns advanced shaping and explicit font fallback.
`src/engine/gpu/text.rs` integrates Glyphon **0.11.0**, compatible with the existing
wgpu **29.0.4**. Glyphon **0.12.0** requires wgpu 30 and is intentionally not selected.
The maintained dedicated WGSL is the pinned dependency's `src/shader.wgsl`:
https://docs.rs/crate/glyphon/0.11.0/source/src/shader.wgsl

The primary font covers the Latin, Lithuanian, German and CAD specimen characters;
the symbols fonts are explicit fallback. Other scripts require an additional licensed
font, loaded through `TextDocument::replace_fonts`, which invalidates every affected
shape and GPU resource. Missing glyph IDs remain visible .notdef boxes and are counted.

Existing imported PDF letters are vector meshes with per-glyph geometry already baked
by `session_rust::pdf`; source strings and font runs are absent from that mesh contract.
`text_outline.wgsl` preserves those exact positions and source font shapes through an
unlit, alpha-preserving coverage pipeline. Legacy PDFs can mix letters into page fills,
so both sheet index runs use it without guessing glyph boundaries or changing order.
The automatic sample policy requests four coverage samples for sheet vectors as well
as solids, within the existing adapter memory budget. Forced 1x and canvases above that
budget retain visibly lower quality on thin outlines. Source strings/font runs are not
reconstructed or substituted; the Glyphon lane is for actual source-text labels.

`TextStats` reports actual Swash raster image counts and byte-vector capacities on the CPU,
separately from candidate raster keys and active instance payload. The pinned Glyphon API
does not expose GPU atlas/instance capacity, upload bytes or isolated rasterization time.
Those are excluded from measured GPU totals; combined preparation time is not labeled as
rasterization or upload time.
