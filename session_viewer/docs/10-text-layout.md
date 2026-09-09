# 10 · Shape text before drawing it

**Start:** checkpoint 09. **Finish:** source text is shaped with explicit fonts and measured against the same-font browser reference. GPU placement follows in lesson 11.

## Characters are not positioned glyphs

A string contains characters. A font provides glyph shapes. Shaping chooses glyphs and their positions, including kerning, ligatures, combining marks and fallback fonts. Counting characters or adding bitmap widths is not enough to lay out a line.

```text
UTF-8 string + font + size
             ↓ Cosmic Text shaping
 glyph IDs / advances / offsets / source clusters
             ↓ TextDocument retains the shaped runs
       line box, baseline and layout metrics
```

An **advance** tells where the next glyph's pen position goes. A glyph's bitmap bounds describe the pixels it needs. A space may advance without producing visible pixels; a letter may extend outside its advance. Preserve fractional positions instead of rounding every character to integer pixels.

A **cluster** associates shaped output with source text. One character need not produce one glyph, and several characters can form one ligature. Keeping source strings separate from glyph instances makes future editing and selection possible.

## Explicit fonts make the result reproducible

The browser cannot rely on a native system-font lookup path. Load the bundled licensed Noto font bytes into the font system. Use the same font, size and shaping options for the HTML reference, or a width comparison is meaningless.

`engine/text.rs` owns the text document and shaping state. A label contains content, style and placement intent. Camera movement changes placement, not the string or its glyph advances, so it should not reshape every line.

## Learn the cache boundary

Store the shaped result until text, font or layout constraints change. Rasterization has another cache because a glyph may need different pixel coverage at a different physical size. A camera-facing world label can keep the same shape while its screen size changes.

The pinned Glyphon integration uses the shaping stack compatible with this viewer's wgpu version. Keep the supplied lockfile. Swapping a text dependency independently can change wgpu types and break pipeline/atlas compatibility.

## Write the files

Follow [Complete file changes for 10](../lessons/10/index.md). Read the source label and shaped-run records, the font initialization and the layout function in that order. Copy the licensed fonts using the listed binary-input command; do not recreate font files from a code block.

The reference page is part of the lesson. Its same-font measurements make a spacing defect observable before camera placement and compositing complicate the result.

## Checkpoint

```sh
cd "$COURSE_WORK/session_viewer"
cargo check --locked --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>, then <http://localhost:8780/text-layout.html>. The layout reference must report passing measurements for its sample strings. Check spaces, punctuation, accented text and CAD symbols, not just a row of identical letters.

If metrics disagree, compare the actual font bytes, size, kerning/ligature settings and baseline convention. Adding arbitrary spacing to every character can hide one sample's error while breaking another.

**Before continuing:** explain why the bitmap width of a space cannot drive the pen. Continue to [text rendering](11-text-rendering.md).
