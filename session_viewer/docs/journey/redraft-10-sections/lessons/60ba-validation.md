# 10.2 · Validated shaping and revision-based reuse

**Section 6: CAD B-Rep Topology, World Text Atlases, Outlines, and Point Clouds** · **Lesson 10.2** (Redraft `6.7`) · **7 checkpoints (`60ba-validation` → `60g-text-shaping`)** · **Study time: 3.5–7 h**

> **What You Will Learn & Build:** Validate a complete label replacement before changing it. By the final checkpoint (`60g-text-shaping`), you will prove the current fonts shaping and reuse rules.

## 1. Visual Pipeline & WebGPU Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 220" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="212" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Lesson 10.2 Execution Flow: Validated shaping and revision-based reuse</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">Concrete data pipeline you build in this lesson (7 Checkpoints: 60ba-validation → 60g-text-shaping)</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="89" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="92" x2="151" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="89" y="108" text-anchor="middle" font-size="11" fill="#111111">incoming labels</text>
    <line x1="161" y1="108" x2="181" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="183" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="254" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="193" y1="92" x2="316" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="254" y="108" text-anchor="middle" font-size="11" fill="#111111">bounded words and</text>
    <text x="254" y="124" text-anchor="middle" font-size="11" fill="#111111">IDs</text>
    <line x1="326" y1="108" x2="346" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="348" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="419" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="358" y1="92" x2="481" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="419" y="108" text-anchor="middle" font-size="11" fill="#111111">finite placement</text>
    <text x="419" y="124" text-anchor="middle" font-size="11" fill="#111111">and metrics</text>
    <line x1="491" y1="108" x2="511" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="513" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="584" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="523" y1="92" x2="646" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="584" y="108" text-anchor="middle" font-size="11" fill="#111111">atomic replacement</text>
    <line x1="656" y1="108" x2="676" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="678" y="68" width="143" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="749" y="86" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 5</text>
    <line x1="688" y1="92" x2="811" y2="92" stroke="#111111" stroke-width="0.75"/>
    <text x="749" y="108" text-anchor="middle" font-size="11" fill="#111111">real bundled fonts</text>
    
    <rect x="18" y="162" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="181" font-size="11" font-weight="700" fill="#111111">Files You Edit in Lesson 10.2:</text>
    <text x="215" y="181" font-size="11" fill="#111111">src/text.rs</text>
    <text x="32" y="198" font-size="10.5" fill="#111111">7 Checkpoints: 60ba-validation → 60g-text-shaping — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg viewBox="0 0 840 240" xmlns="http://www.w3.org/2000/svg" style="max-width:100%;height:auto;display:block;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <rect x="4" y="4" width="832" height="232" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="28" font-size="14" font-weight="700" fill="#111111">Chapter 10 WebGPU Text & Shared Arena Pipeline: Glyph Atlas Packing, World/Billboard Text & Matrix Shader</text>
  <text x="20" y="46" font-size="11.5" fill="#111111">How crisp typography (text_plane.wgsl / text.wgsl) and procedural cell digits (matrix.wgsl) render in 3D world space.</text>
  <line x1="16" y1="54" x2="824" y2="54" stroke="#111111" stroke-width="1"/>
  
    <rect x="18" y="68" width="235" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="135" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">1. Shelf-Packed Glyph Atlas</text>
    <text x="28" y="108" font-size="10.5" fill="#111111">• Rasterize needed Unicode codepoints</text>
    <text x="28" y="126" font-size="10.5" font-weight="600" fill="#111111">• Pack into R8Unorm GPU Atlas Texture</text>
    <text x="28" y="142" font-size="10" fill="#111111">  with 1px padding (no bilinear bleed!)</text>
    <text x="28" y="160" font-size="10.5" fill="#111111">• queue.write_texture dirty rects only</text>
    <text x="28" y="178" font-size="10.5" fill="#111111">• Layout baseline, kerning &amp; line</text>
    <text x="28" y="193" font-size="10.5" fill="#111111">  wraps into instanced glyph quads</text>

    <line x1="253" y1="138" x2="283" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="285" y="68" width="265" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="417" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">2. World-Plane vs Screen Billboard</text>
    <text x="297" y="108" font-size="10.5" font-weight="600" fill="#111111">• text_plane.wgsl (3D World Text):</text>
    <text x="297" y="123" font-size="10" fill="#111111">  q_world = origin + u*plane_x + v*plane_y</text>
    <text x="297" y="138" font-size="10" fill="#111111">  Foreshortens naturally with 3D perspective</text>
    <text x="297" y="158" font-size="10.5" font-weight="600" fill="#111111">• text.wgsl (Screen-Aligned Label):</text>
    <text x="297" y="173" font-size="10" fill="#111111">  Expands along camera Right/Up vectors;</text>
    <text x="297" y="188" font-size="10" fill="#111111">  optional background plate + 3D leader line</text>

    <line x1="550" y1="138" x2="580" y2="138" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>

    <rect x="582" y="68" width="238" height="145" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="701" y="88" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">3. Whole-Text Pick &amp; Matrix (Sec 10)</text>
    <text x="594" y="108" font-size="10.5" fill="#111111">• Clicking any glyph or plate writes</text>
    <text x="594" y="124" font-size="10.5" font-weight="600" fill="#111111">  the parent Text owner_id to Rg32Uint</text>
    <text x="594" y="144" font-size="10.5" fill="#111111">• Later in Sec 10 (matrix.wgsl):</text>
    <text x="594" y="160" font-size="10" fill="#111111">  16-word header + 3 words/cell renders</text>
    <text x="594" y="175" font-size="10" fill="#111111">  procedural 3-decimal digits (0 blank)</text>
    <text x="594" y="190" font-size="10" fill="#111111">  + plate-less world title &amp; yellow select</text>
    
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Rust / WebGPU** | Shelf-Packed `R8Unorm` Glyph Atlas + 1px Padding | Rasterizes Unicode glyphs on demand into a single-channel GPU texture with a 1-pixel transparent gutter so bilinear filtering never bleeds neighbor glyphs. |
| **WGSL (`text_plane.wgsl` / `text.wgsl`)** | World-Plane Quad vs. Screen-Aligned Billboard Quad | `text_plane.wgsl` expands glyph quads along 3D plane axes `(plane_x, plane_y)` that foreshorten in perspective; `text.wgsl` expands along camera `(Right, Up)` vectors. |
| **WGSL (`matrix.wgsl`)** | Procedural Matrix Cell Digits (16-word Header + 3 words/cell) | Draws heatmap cells and procedural 3-decimal digits directly in the fragment shader (leaving `0` cells blank), with a plate-less world title and yellow selection border. |

**Authoritative production source (`session_viewer/src`):** [engine/text.rs](../../../../src/engine/text.rs), [engine/gpu/text.rs](../../../../src/engine/gpu/text.rs), [engine/gpu/text_plane.rs](../../../../src/engine/gpu/text_plane.rs), [engine/gpu/text_plate.rs](../../../../src/engine/gpu/text_plate.rs), [app/scene_text.rs](../../../../src/app/scene_text.rs), [app/fonts.rs](../../../../src/app/fonts.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`60ba-validation`](#checkpoint-60ba-validation) | Validate a complete label replacement before changing it. | `src/text.rs` | `incoming labels → bounded words and IDs → finite placement and metrics → atomic replacement` |
| **2** | [`60bb-shaping`](#checkpoint-60bb-shaping) | Shape words into glyph clusters and baselines. | `src/text.rs` | `UTF-8 words → advanced shaping → glyph clusters → positioned baselines` |
| **3** | [`60c-document`](#checkpoint-60c-document) | Own the shaped runs with their font revisions. | `src/text.rs` | `explicit font set → shaped-run owner → document revision → font revision` |
| **4** | [`60d-reuse`](#checkpoint-60d-reuse) | Reshape only when the source layout changes. | `src/text.rs` | `valid label replacement → stable identity lookup → same layout reuse → changed words reshape` |
| **5** | [`60e-font-reload`](#checkpoint-60e-font-reload) | Replace usable fonts without corrupting current text. | `src/text.rs` | `new font bytes → usable database → atomic font adoption → run reshaping` |
| **6** | [`60f-diagnostics`](#checkpoint-60f-diagnostics) | Inspect glyph origins clusters and multiline metrics. | `src/text.rs` | `shaped buffer → glyph and font ID → UTF-8 cluster → baseline and advance` |
| **7** | [`60g-text-shaping`](#checkpoint-60g-text-shaping) | Prove the current fonts shaping and reuse rules. | `src/text.rs` | `real bundled fonts → glyph shaping → presentation reuse → native coverage proofs` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Read the **Concept Walkthrough** and **Self-Check Q&A** first, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-60ba-validation"></a>

### Step 1 of 7 · Validate a complete label replacement before changing it (`60ba-validation`)

- **Goal:** Validate a complete label replacement before changing it.
- **Data Flow Trace:** `incoming labels → bounded words and IDs → finite placement and metrics → atomic replacement`
- **Files You Touch:** `src/text.rs` (*Validate metrics, positions, plane axes, padding and clip bounds.*)

Check byte budget, unique identities, finite CSS metrics, world positions, orthonormal plane axes and ordered clips before replacing any current shaped runs.

> **Self-Check Question:** *Should a bad label discard the currently visible text?*  
> **Answer:** No. Every replacement label is checked before any current run is removed. Invalid metrics, positions or clips leave the existing label set and shaping intact.

- **Hands-On Lab Experiment:** Try a NaN font size, reversed clip box and nonunit world-plane axes. Verify each refusal preserves the current source text and revisions.

<details>
<summary>Implementation 60ba-validation: Validate a complete label replacement before changing it</summary>

Validate a complete label replacement before changing it.

Data flow: incoming labels → bounded words and IDs → finite placement and metrics → atomic replacement.

### 1. `src/text.rs`

Validate metrics, positions, plane axes, padding and clip bounds.

<details>
<summary>Locate the existing block</summary>

```rust
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60ba-validation-01.rs" skip=62
```

</details>

<a id="checkpoint-60bb-shaping"></a>

### Step 2 of 7 · Shape words into glyph clusters and baselines (`60bb-shaping`)

- **Goal:** Shape words into glyph clusters and baselines.
- **Data Flow Trace:** `UTF-8 words → advanced shaping → glyph clusters → positioned baselines`
- **Files You Touch:** `src/text.rs` (*Create advanced shaped buffers and distinguish layout-changing source fields.*)

Use advanced shaping with the explicit Noto family, unconstrained line width and original label identity. Preserve glyph cluster byte ranges and line metrics; they are not one-character indices.

> **Self-Check Question:** *Must every input character produce exactly one glyph?*  
> **Answer:** No. Ligatures can combine characters and combining marks can attach to another glyph. Cluster byte ranges retain the connection to original UTF-8 text while the shaped glyph count can differ.

- **Hands-On Lab Experiment:** Compare ffi with three characters, AV with A V, and composed é with e plus its combining accent. Inspect cluster byte ranges and line baselines.

<details>
<summary>Implementation 60bb-shaping: Shape words into glyph clusters and baselines</summary>

Shape words into glyph clusters and baselines.

Data flow: UTF-8 words → advanced shaping → glyph clusters → positioned baselines.

### 1. `src/text.rs`

Create advanced shaped buffers and distinguish layout-changing source fields.

<details>
<summary>Locate the existing block</summary>

```rust
    value.is_finite() && (0.0..=256.0).contains(value)
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60bb-shaping-01.rs" skip=58
```

</details>

<a id="checkpoint-60c-document"></a>

### Step 3 of 7 · Own the shaped runs with their font revisions (`60c-document`)

- **Goal:** Own the shaped runs with their font revisions.
- **Data Flow Trace:** `explicit font set → shaped-run owner → document revision → font revision`
- **Files You Touch:** `src/text.rs` (*Own explicit fonts, shaped runs and independent source/font revisions.*)

Keep current label runs, explicit fonts, document/font revisions and shaping counters in one document. The constructor loads fonts once without shaping any empty label list.

> **Self-Check Question:** *Does creating an empty text document need to shape any label?*  
> **Answer:** No. Its runs and counters start empty. Fonts are ready, and the first accepted source label creates a shaped run. Later presentation changes can keep it.

- **Hands-On Lab Experiment:** Create an empty document and inspect zero runs and zero shape count. Add one label at the next checkpoint and compare document and font revisions.

<details>
<summary>Implementation 60c-document: Own the shaped runs with their font revisions</summary>

Own the shaped runs with their font revisions.

Data flow: explicit font set → shaped-run owner → document revision → font revision.

### 1. `src/text.rs`

Own explicit fonts, shaped runs and independent source/font revisions.

<details>
<summary>Locate the existing block</summary>

```rust
            self.color
        }
    }
}
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60c-document-01.rs" skip=41
```

</details>

<a id="checkpoint-60d-reuse"></a>

### Step 4 of 7 · Reshape only when the source layout changes (`60d-reuse`)

- **Goal:** Reshape only when the source layout changes.
- **Data Flow Trace:** `valid label replacement → stable identity lookup → same layout reuse → changed words reshape`
- **Files You Touch:** `src/text.rs` (*Reuse stable shaped buffers and measure only genuine shaping work.*)

Validate the whole incoming set, reuse runs by stable label identity and preserve a shaped buffer when only position, colour, clipping or selection changes.

> **Self-Check Question:** *Should camera movement or label recolouring reshape the words?*  
> **Answer:** No. Words, font size and line height determine shaping here. Placement, colour and selection change presentation while reusing the shaped buffer.

- **Hands-On Lab Experiment:** Move and recolour one label, then change its words or font size. Compare shape_count and revision, and confirm only the layout change increases shaping.

<details>
<summary>Implementation 60d-reuse: Reshape only when the source layout changes</summary>

Reshape only when the source layout changes.

Data flow: valid label replacement → stable identity lookup → same layout reuse → changed words reshape.

### 1. `src/text.rs`

Reuse stable shaped buffers and measure only genuine shaping work.

<details>
<summary>Locate the existing block</summary>

```rust
            shaping_ms: 0.0,
        }
    }
```

</details>

After that block, type:

```rust
--8<-- "journey/code/60d-reuse-01.rs" skip=46
```

</details>

<a id="checkpoint-60e-font-reload"></a>

### Step 5 of 7 · Replace usable fonts without corrupting current text (`60e-font-reload`)

- **Goal:** Replace usable fonts without corrupting current text.
- **Data Flow Trace:** `new font bytes → usable database → atomic font adoption → run reshaping`
- **Files You Touch:** `src/text.rs` (*Adopt fonts only after every supplied source contains a usable face.*)

Build a replacement font database before adopting it. Refuse unusable data without touching current fonts; valid replacement reshapes every run and advances both font and document revisions.

> **Self-Check Question:** *Should a failed font download erase the existing labels?*  
> **Answer:** No. An unusable font set never replaces the current database. The current runs remain. Only an accepted usable replacement advances font revisions and reshapes the text.

- **Hands-On Lab Experiment:** Replace fonts with eight zero bytes and verify no change. Then adopt the supplied whole fonts and compare shape count, revisions and original label words.

<details>
<summary>Implementation 60e-font-reload: Replace usable fonts without corrupting current text</summary>

Replace usable fonts without corrupting current text.

Data flow: new font bytes → usable database → atomic font adoption → run reshaping.

### 1. `src/text.rs`

Adopt fonts only after every supplied source contains a usable face.

<details>
<summary>Locate the existing block</summary>

```rust
    }

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/60e-font-reload-01.rs"
```

</details>

<a id="checkpoint-60f-diagnostics"></a>

### Step 6 of 7 · Inspect glyph origins clusters and multiline metrics (`60f-diagnostics`)

- **Goal:** Inspect glyph origins clusters and multiline metrics.
- **Data Flow Trace:** `shaped buffer → glyph and font ID → UTF-8 cluster → baseline and advance`
- **Files You Touch:** `src/text.rs` (*Expose actual shaped glyph metrics and release current run capacity on Clear.*)

Expose derived glyph identity, font identity, UTF-8 cluster range, origin, advance, offset and baseline. Clear releases shaped runs while retaining the explicit font system for future labels.

> **Self-Check Question:** *Can a glyph cluster range be used as a character count?*  
> **Answer:** No. It addresses bytes in the original UTF-8 string. A multibyte character or ligature can span several bytes while producing one glyph.

- **Hands-On Lab Experiment:** Inspect office ffi and accented multiline labels. Compare line baselines one line_height apart and check that every cluster range lies in the original string.

<details>
<summary>Implementation 60f-diagnostics: Inspect glyph origins clusters and multiline metrics</summary>

Inspect glyph origins clusters and multiline metrics.

Data flow: shaped buffer → glyph and font ID → UTF-8 cluster → baseline and advance.

### 1. `src/text.rs`

Expose actual shaped glyph metrics and release current run capacity on Clear.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }
}

pub fn covers(text: &str) -> bool {
    text.is_ascii() || text.chars().all(|c| c.is_control() || bundled_glyph(c))
}
```

</details>

Replace it with:

```rust
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GlyphDiagnostic {
    pub label: u32,
    pub line: usize,
    pub cluster: [usize; 2],
    pub glyph: u16,
    pub font: String,
    pub origin: [f32; 2],
    pub advance: f32,
    pub offset: [f32; 2],
    pub baseline: f32,
    pub line_width: f32,
}

pub fn covers(text: &str) -> bool {
    text.is_ascii() || text.chars().all(|c| c.is_control() || bundled_glyph(c))
}
```

<details>
<summary>Locate this block</summary>

```rust
        Ok(())
    }

}

impl Default for TextDocument {
```

</details>

Replace it with:

```rust
        Ok(())
    }

    pub fn clear(&mut self) {
        self.runs.clear();
        self.runs.shrink_to_fit();
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn diagnostics(&self) -> Vec<GlyphDiagnostic> {
        let mut out = Vec::new();

        for run in &self.runs {
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    out.push(GlyphDiagnostic {
                        label: run.label.id,
                        line: line.line_i,
                        cluster: [glyph.start, glyph.end],
                        glyph: glyph.glyph_id,
                        font: format!("{:?}", glyph.font_id),
                        origin: [glyph.x, glyph.y],
                        advance: glyph.w,
                        offset: [
                            glyph.x_offset * glyph.font_size,
                            glyph.y_offset * glyph.font_size,
                        ],
                        baseline: line.line_y,
                        line_width: line.line_w,
                    });
                }
            }
        }

        out
    }
}

impl Default for TextDocument {
```

<details>
<summary>Complete updated block reference</summary>

```rust
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap, fontdb};
use serde::Serialize;

pub const FONT_FAMILY: &str = "Noto Sans";

pub static FONT_BYTES: &[u8] = include_bytes!("../assets/text/NotoSans-Regular.subset.ttf");

pub static SYMBOL_BYTES: &[u8] =
    include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf");

pub static FALLBACK_BYTES: &[u8] =
    include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf");

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
pub const FULL_FONTS: [&str; 3] = [
    "NotoSans-Regular.ttf",
    "NotoSansSymbols2-Regular.ttf",
    "NotoSansSymbols-Regular.ttf",
];

const MAX_TEXT_BYTES: usize = 256 * 1024;

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TextPlacement {
    Screen {
        left: f32,
        top: f32,
    },
    Anchor {
        world: [f64; 3],
        offset: [f32; 2],
    },
    Nameplate {
        world: [f64; 3],
        padding: [f32; 2],
        rounded: bool,
    },
    WorldPlane {
        world: [f64; 3],
        right: [f64; 3],
        up: [f64; 3],
        world_height: f64,
    },
    WorldBillboard {
        world: [f64; 3],
        world_height: f64,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextObject {
    pub row: u32,
    pub selected: bool,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TextLabel {
    pub id: u32,
    pub object: Option<TextObject>,
    pub text: String,
    pub font_size: f32,
    pub line_height: f32,
    pub color: [u8; 4],
    pub placement: TextPlacement,
    pub clip: Option<[f32; 4]>,
}

impl TextLabel {
    pub fn ink_color(&self) -> [u8; 4] {
        if self.object.is_some_and(|object| object.selected) {
            [0, 0, 0, 255]
        } else {
            self.color
        }
    }
}

pub struct TextRun {
    pub label: TextLabel,
    pub buffer: Buffer,
}

pub struct TextDocument {
    pub fonts: FontSystem,
    pub runs: Vec<TextRun>,
    pub revision: u64,
    pub font_revision: u64,
    pub shape_count: u64,
    pub shaping_ms: f64,
}

impl TextDocument {
    pub fn new() -> Self {
        Self {
            fonts: bundled_fonts(),
            runs: Vec::new(),
            revision: 0,
            font_revision: 1,
            shape_count: 0,
            shaping_ms: 0.0,
        }
    }

    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        let mut bytes = 0usize;
        let mut ids = std::collections::HashSet::new();

        for label in &labels {
            bytes = bytes.saturating_add(label.text.len());
            anyhow::ensure!(bytes <= MAX_TEXT_BYTES, "text document exceeds 256 KiB");
            anyhow::ensure!(ids.insert(label.id), "duplicate text label ID {}", label.id);
            validate_label(label)?;
        }

        if self.runs.len() == labels.len() {
            let mut unchanged = true;

            for (run, label) in self.runs.iter().zip(&labels) {
                unchanged &= run.label == *label;
            }

            if unchanged {
                return Ok(());
            }
        }

        let previous = std::mem::take(&mut self.runs);
        let mut previous_by_id = std::collections::HashMap::new();

        for run in previous {
            previous_by_id.insert(run.label.id, run);
        }

        for label in labels {
            let old = previous_by_id.remove(&label.id);
            let buffer = match old {
                Some(run) if same_layout(&run.label, &label) => run.buffer,
                _ => {
                    self.shape_count += 1;
                    let started = crate::load_measure::now();
                    let buffer = shape(&mut self.fonts, &label);
                    self.shaping_ms += crate::load_measure::now() - started;
                    buffer
                }
            };
            self.runs.push(TextRun { label, buffer });
        }

        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    pub fn replace_fonts(&mut self, sources: Vec<Vec<u8>>) -> anyhow::Result<()> {
        let sources = sources
            .into_iter()
            .map(|bytes| fontdb::Source::Binary(std::sync::Arc::new(bytes)))
            .collect();
        self.replace_sources(sources)
    }

    pub fn replace_sources(&mut self, sources: Vec<fontdb::Source>) -> anyhow::Result<()> {
        let mut db = fontdb::Database::new();

        for source in sources {
            let before = db.faces().count();
            db.load_font_source(source);
            anyhow::ensure!(
                db.faces().count() > before,
                "font data contains no usable face"
            );
        }

        anyhow::ensure!(db.faces().count() > 0, "font set is empty");
        db.set_sans_serif_family(FONT_FAMILY);
        self.fonts = FontSystem::new_with_locale_and_db("en-US".into(), db);

        for run in &mut self.runs {
            let started = crate::load_measure::now();
            run.buffer = shape(&mut self.fonts, &run.label);
            self.shaping_ms += crate::load_measure::now() - started;
            self.shape_count += 1;
        }

        self.font_revision = self.font_revision.wrapping_add(1);
        self.revision = self.revision.wrapping_add(1);
        Ok(())
    }

    pub fn clear(&mut self) {
        self.runs.clear();
        self.runs.shrink_to_fit();
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn diagnostics(&self) -> Vec<GlyphDiagnostic> {
        let mut out = Vec::new();

        for run in &self.runs {
            for line in run.buffer.layout_runs() {
                for glyph in line.glyphs {
                    out.push(GlyphDiagnostic {
                        label: run.label.id,
                        line: line.line_i,
                        cluster: [glyph.start, glyph.end],
                        glyph: glyph.glyph_id,
                        font: format!("{:?}", glyph.font_id),
                        origin: [glyph.x, glyph.y],
                        advance: glyph.w,
                        offset: [
                            glyph.x_offset * glyph.font_size,
                            glyph.y_offset * glyph.font_size,
                        ],
                        baseline: line.line_y,
                        line_width: line.line_w,
                    });
                }
            }
        }

        out
    }
}

impl Default for TextDocument {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GlyphDiagnostic {
    pub label: u32,
    pub line: usize,
    pub cluster: [usize; 2],
    pub glyph: u16,
    pub font: String,
    pub origin: [f32; 2],
    pub advance: f32,
    pub offset: [f32; 2],
    pub baseline: f32,
    pub line_width: f32,
}

pub fn covers(text: &str) -> bool {
    text.is_ascii() || text.chars().all(|c| c.is_control() || bundled_glyph(c))
}

fn bundled_glyph(c: char) -> bool {
    thread_local! {
        static MAPS: Vec<ttf_parser::cmap::Table<'static>> = [FONT_BYTES, FALLBACK_BYTES, SYMBOL_BYTES]
            .into_iter()
            .filter_map(|bytes| {
                let face = ttf_parser::RawFace::parse(bytes, 0).ok()?;
                ttf_parser::cmap::Table::parse(face.table(ttf_parser::Tag::from_bytes(b"cmap"))?)
            })
            .collect();
    }

    MAPS.with(|maps| {
        maps.iter().any(|map| {
            map.subtables
                .into_iter()
                .any(|table| table.is_unicode() && table.glyph_index(u32::from(c)).is_some())
        })
    })
}

fn bundled_fonts() -> FontSystem {
    let mut db = fontdb::Database::new();

    for bytes in [FONT_BYTES, FALLBACK_BYTES, SYMBOL_BYTES] {
        db.load_font_source(fontdb::Source::Binary(std::sync::Arc::new(bytes)));
    }

    db.set_sans_serif_family(FONT_FAMILY);
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}

fn validate_label(label: &TextLabel) -> anyhow::Result<()> {
    anyhow::ensure!(
        label.font_size.is_finite() && (1.0..=256.0).contains(&label.font_size),
        "font size must be 1..256 CSS px"
    );
    anyhow::ensure!(
        label.line_height.is_finite()
            && label.line_height >= label.font_size
            && label.line_height <= 1024.0,
        "invalid text line height"
    );
    let valid_placement = match label.placement {
        TextPlacement::Screen { left, top } => left.is_finite() && top.is_finite(),
        TextPlacement::Anchor { world, offset } => {
            world.iter().all(finite_f64) && offset.iter().all(finite_f32)
        }
        TextPlacement::Nameplate { world, padding, .. } => {
            world.iter().all(finite_f64) && padding.iter().all(valid_padding)
        }
        TextPlacement::WorldPlane {
            world,
            right,
            up,
            world_height,
        } => {
            world.iter().all(finite_f64)
                && valid_plane_axes(right, up)
                && world_height.is_finite()
                && world_height > 0.0
        }
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } => world.iter().all(finite_f64) && world_height.is_finite() && world_height > 0.0,
    };
    anyhow::ensure!(valid_placement, "invalid text placement");

    if let Some(c) = label.clip {
        anyhow::ensure!(
            c.iter().all(finite_f32) && c[2] >= c[0] && c[3] >= c[1],
            "invalid text clip rectangle"
        );
    }

    Ok(())
}

fn valid_plane_axes(right: [f64; 3], up: [f64; 3]) -> bool {
    let mut right_length = 0.0;
    let mut up_length = 0.0;
    let mut dot = 0.0;

    for axis in 0..3 {
        right_length += right[axis] * right[axis];
        up_length += up[axis] * up[axis];
        dot += right[axis] * up[axis];
    }

    (right_length.sqrt() - 1.0).abs() <= 1e-6
        && (up_length.sqrt() - 1.0).abs() <= 1e-6
        && dot.abs() <= 1e-6
}

fn finite_f32(value: &f32) -> bool {
    value.is_finite()
}

fn finite_f64(value: &f64) -> bool {
    value.is_finite()
}

fn valid_padding(value: &f32) -> bool {
    value.is_finite() && (0.0..=256.0).contains(value)
}

fn same_layout(a: &TextLabel, b: &TextLabel) -> bool {
    a.text == b.text && a.font_size == b.font_size && a.line_height == b.line_height
}

fn shape(fonts: &mut FontSystem, label: &TextLabel) -> Buffer {
    let mut buffer = Buffer::new(fonts, Metrics::new(label.font_size, label.line_height));
    buffer.set_size(fonts, None, None);
    buffer.set_wrap(fonts, Wrap::None);
    buffer.set_text(
        fonts,
        &label.text,
        &Attrs::new()
            .family(Family::Name(FONT_FAMILY))
            .metadata(label.id as usize),
        Shaping::Advanced,
        None,
    );
    buffer.shape_until_scroll(fonts, false);
    buffer
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/60f-diagnostics-01.rs"
```

```text
--8<-- "journey/code/60f-diagnostics-02.rs"
```

</details>

</details>

<a id="checkpoint-60g-text-shaping"></a>

### Step 7 of 7 · Prove the current fonts shaping and reuse rules (`60g-text-shaping`)

- **Goal:** Prove the current fonts shaping and reuse rules.
- **Data Flow Trace:** `real bundled fonts → glyph shaping → presentation reuse → native coverage proofs`
- **Files You Touch:** `src/text.rs` (*Copy current production ligature, accent, kerning, fallback, failed replacement, reuse and full-font coverage regressions.*)

Copy current production shaping regressions, adapted to the cumulative command vocabulary. Verify ligatures, kerning, combining accents, fallback fonts, atomic refusal and whole-font coverage.

> **Self-Check Question:** *What distinguishes this check from a character-to-quad mock?*  
> **Answer:** It uses real font shaping, ligatures, glyph IDs and UTF-8 clusters. The checks compare actual kerning, composed/decomposed accents, fallback font identities and valid whole-font replacement.

- **Hands-On Lab Experiment:** Inspect the shaped office ffi and accented multiline examples from the previous lesson in the browser. Compare glyph clusters with the original words and baselines. Read the optional ligature, fallback and invalid-font cases to explain why a rejected replacement leaves the displayed label intact.

<details>
<summary>Implementation 60g-text-shaping: Prove the current fonts shaping and reuse rules</summary>

Prove the current fonts shaping and reuse rules.

Data flow: real bundled fonts → glyph shaping → presentation reuse → native coverage proofs.

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 1. `src/text.rs`

Copy current production ligature, accent, kerning, fallback, failed replacement, reuse and full-font coverage regressions.

<details>
<summary>Locate the existing block</summary>

```text
    buffer
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/60g-text-shaping-01.rs" skip=14
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 60g-text-shaping
npm --prefix ../session_tests run course -- check 60g-text-shaping
npm --prefix ../session_tests run course -- save 60g-text-shaping
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

- **Final Lesson Experiment:** Inspect the shaped office ffi and accented multiline examples from the previous lesson in the browser. Compare glyph clusters with the original words and baselines. Read the optional ligature, fallback and invalid-font cases to explain why a rejected replacement leaves the displayed label intact.

Continue with [A shared glyph atlas and lazy text drawing](../../consolidation/61-atlas.md).
