# Bundled text fonts

Noto Sans Regular, Noto Sans Symbols Regular and Noto Sans Symbols 2 Regular are distributed under the adjacent
SIL Open Font License 1.1 (`OFL.txt`). The WASM embeds only their `*.subset.ttf` files (hinted, `.notdef`
outline and license names kept): Noto Sans cut to Latin, Lithuanian, German, the CAD specimen and every
string the viewer draws itself (51 KB), the symbol fonts cut to the symbols those strings use (→ ⚙ and
■ ⏵ ◻ ⏳ ⌘). Labels and panels (egui draws Noto Sans too; Ubuntu Light is gone) use the same faces.
A label, document, layer or object name with a character outside the subsets fetches the three whole
fonts once from `text/` and reshapes; until they arrive that character is a `.notdef` box (petras
approved this async fallback, 2026-09-24). Trunk copies the whole fonts for that fetch and for
`text-quality.html`'s browser reference. No machine-installed font discovery is used.
The subsets are fontTools subsets (`pyftsubset <font> --unicodes=... --notdef-outline
--name-IDs=0,1,2,3,4,5,6,13,14`, default layout features, hinting kept); glyph outlines, hinting
programs and shaping of every covered string match the whole font. `engine::text` tests that the
command strings and the Lithuanian, German and CAD samples stay covered.

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
