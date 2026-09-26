# Bundled text fonts

Noto Sans Regular, Noto Sans Symbols Regular and Noto Sans Symbols 2 Regular, under the SIL Open Font License 1.1
(`OFL.txt`, next to this file). All three are compiled into the wasm, and Trunk copies the same bytes
so `text-quality.html` can compare against the browser. Fonts installed on the machine are never used.

Upstream sources:
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSansSymbols2/NotoSansSymbols2-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/hinted/ttf/NotoSansSymbols/NotoSansSymbols-Regular.ttf
- https://github.com/notofonts/noto-fonts/blob/main/LICENSE

`src/engine/text.rs` shapes the text and picks the fallback font.
`src/engine/gpu/text.rs` draws it with Glyphon **0.11.0**, the release that matches wgpu **29.0.4**;
Glyphon **0.12.0** needs wgpu 30. Its shader is Glyphon's own `src/shader.wgsl`:
https://docs.rs/crate/glyphon/0.11.0/source/src/shader.wgsl

Noto Sans covers the Latin, Lithuanian, German and CAD characters in the samples; the two symbol fonts
fill the gaps. Another script needs another licensed font, loaded with `TextDocument::replace_fonts`,
which reshapes every label and resets the GPU text. A glyph no font has shows as a .notdef box and is
counted in `missing_glyphs`.

Letters in imported PDFs are not text: `session_rust::pdf` already turned them into meshes, with no
string or font left. `text_outline.wgsl` draws those meshes unlit, in their exact positions, with
coverage as alpha. Old PDFs can mix letters into page fills, so both sheet index runs use that pipeline
instead of guessing where a letter ends. Sheet vectors get 4x MSAA like solids, within the adapter
memory budget; at a forced 1x, or on a canvas past that budget, thin outlines look rougher. The Glyphon
lane is only for real text labels.

`TextStats` counts Swash's glyph images on the CPU and their byte capacity, apart from raster keys and
the bytes of drawn glyph instances. Glyphon 0.11 does not report GPU atlas size, upload bytes or raster
time, so those are left out; `preparation_ms` is the whole rebuild, not raster or upload alone.
