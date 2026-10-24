# 10.2 · Validated shaping and revision-based reuse

**Goal:** Prove the current fonts shaping and reuse rules. · **Steps:** 7 · **Source:** [engine/text.rs](../../../src/engine/text.rs) · [engine/gpu/text.rs](../../../src/engine/gpu/text.rs) · [engine/gpu/text_plane.rs](../../../src/engine/gpu/text_plane.rs) · [engine/gpu/text_plate.rs](../../../src/engine/gpu/text_plate.rs)

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">R8Unorm Glyph Atlas Shelf Packing (u0,v0..u1,v1)</text>
  <rect x="58" y="48" width="224" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <line x1="58" y1="84" x2="282" y2="84" stroke="#111111" stroke-width="1.2"/>
  <line x1="58" y1="120" x2="282" y2="120" stroke="#111111" stroke-width="1.2"/>
  <rect x="66" y="52" width="28" height="28" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="80" y="72" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">A</text>
  <rect x="100" y="52" width="28" height="28" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="114" y="72" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">B</text>
  <rect x="134" y="52" width="28" height="28" fill="#fef08a" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="148" y="72" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">0</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">3D Plane Quad (O + x*u + y*v) vs 2D Billboard</text>
  <polygon points="424,138 494,88 574,104 504,154" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="424" y1="138" x2="494" y2="88" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <line x1="424" y1="138" x2="504" y2="154" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <rect x="602" y="64" width="96" height="36" rx="4" fill="#fef9c3" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="650" y="86" text-anchor="middle" font-family="monospace" font-size="11" font-weight="700" fill="#111111">Label #4</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">World Labels + Procedural 4×4 Matrix Grid</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="112" height="88" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="854" y1="58" x2="854" y2="146" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="882" y1="58" x2="882" y2="146" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="910" y1="58" x2="910" y2="146" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="826" y1="80" x2="938" y2="80" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="826" y1="102" x2="938" y2="102" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="826" y1="124" x2="938" y2="124" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="840" y="73" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="868" y="95" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="896" y="117" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">1</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="924" y="139" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">1</text>
  <rect x="956" y="78" width="88" height="32" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1000" y="98" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">(0, 0, 0)</text>
</svg>
</div>

## <a id="checkpoint-60ba-validation"></a>1. Validate a complete label replacement before changing it

### 1. `src/text.rs`

Validate metrics, positions, plane axes, padding and clip bounds.

**Locate this existing block:**

```rust
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
        TextPlacement::Nameplate {
            world, padding, ..
        } => world.iter().all(finite_f64) && padding.iter().all(valid_padding),
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
        } => {
            world.iter().all(finite_f64)
                && world_height.is_finite()
                && world_height > 0.0
        }
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
```

## <a id="checkpoint-60bb-shaping"></a>2. Shape words into glyph clusters and baselines

### 1. `src/text.rs`

Create advanced shaped buffers and distinguish layout-changing source fields.

**Locate this existing block:**

```rust
    value.is_finite() && (0.0..=256.0).contains(value)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    value.is_finite() && (0.0..=256.0).contains(value)
}

fn same_layout(a: &TextLabel, b: &TextLabel) -> bool {
    a.text == b.text && a.font_size == b.font_size && a.line_height == b.line_height
}

fn shape(fonts: &mut FontSystem, label: &TextLabel) -> Buffer {
    let mut buffer = Buffer::new(
        fonts,
        Metrics::new(label.font_size, label.line_height),
    );
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

## <a id="checkpoint-60c-document"></a>3. Own the shaped runs with their font revisions

### 1. `src/text.rs`

Own explicit fonts, shaped runs and independent source/font revisions.

**Locate this existing block:**

```rust
}

pub fn covers(text: &str) -> bool {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
}

impl Default for TextDocument {
    fn default() -> Self {
        Self::new()
    }
}

pub fn covers(text: &str) -> bool {
```

## <a id="checkpoint-60d-reuse"></a>4. Reshape only when the source layout changes

### 1. `src/text.rs`

Reuse stable shaped buffers and measure only genuine shaping work.

**Locate this existing block:**

```rust
        }
    }
}

impl Default for TextDocument {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }

    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        let mut bytes = 0usize;
        let mut ids = std::collections::HashSet::new();

        for label in &labels {
            bytes = bytes.saturating_add(label.text.len());
            anyhow::ensure!(
                bytes <= MAX_TEXT_BYTES,
                "text document exceeds 256 KiB"
            );
            anyhow::ensure!(
                ids.insert(label.id),
                "duplicate text label ID {}",
                label.id
            );
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
}

impl Default for TextDocument {
```

## <a id="checkpoint-60e-font-reload"></a>5. Replace usable fonts without corrupting current text

### 1. `src/text.rs`

Adopt fonts only after every supplied source contains a usable face.

**Locate this existing block:**

```rust
        Ok(())
    }
}

impl Default for TextDocument {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Ok(())
    }

    pub fn replace_fonts(&mut self, sources: Vec<Vec<u8>>) -> anyhow::Result<()> {
        let sources = sources
            .into_iter()
            .map(|bytes| fontdb::Source::Binary(std::sync::Arc::new(bytes)))
            .collect();
        self.replace_sources(sources)
    }

    pub fn replace_sources(
        &mut self,
        sources: Vec<fontdb::Source>,
    ) -> anyhow::Result<()> {
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
}

impl Default for TextDocument {
```

## <a id="checkpoint-60f-diagnostics"></a>6. Inspect glyph origins clusters and multiline metrics

### 1. `src/text.rs`

Expose actual shaped glyph metrics and release current run capacity on Clear.

**Locate this existing block:**

```rust
    }
}

impl Default for TextDocument {
    fn default() -> Self {
        Self::new()
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
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
```

## <a id="checkpoint-60g-text-shaping"></a>7. Prove the current fonts shaping and reuse rules

### 1. `src/text.rs`

Copy current production ligature, accent, kerning, fallback, failed replacement, reuse and full-font coverage regressions.

**Locate this existing block:**

```rust
    buffer
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn label(text: &str) -> TextLabel {
        TextLabel {
            object: None,
            id: 1,
            text: text.into(),
            font_size: 16.0,
            line_height: 24.0,
            color: [255; 4],
            placement: TextPlacement::Screen {
                left: 0.0,
                top: 0.0,
            },
            clip: None,
        }
    }

    #[test]
    fn ligatures_accents_spaces_and_symbols_are_shaped() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label(
            "office ffi fl ÄÖÜ ĄČĘĖĮŠŲŪŽ Ø ± 90° m² e\u{301}",
        )])
        .unwrap();
        let glyphs = doc.diagnostics();
        assert!(glyphs.iter().all(has_glyph));
        assert!(glyphs.iter().any(multi_byte_cluster));
        doc.set_labels(vec![label("AV")]).unwrap();
        let pair_width = doc.diagnostics()[0].line_width;
        doc.set_labels(vec![label("A V")]).unwrap();
        assert!(doc.diagnostics()[0].line_width > pair_width);
        doc.set_labels(vec![label("ffi")]).unwrap();
        assert!(
            doc.diagnostics().len() < 3,
            "font's ffi ligature must preserve a multi-character cluster"
        );
    }

    fn has_glyph(glyph: &GlyphDiagnostic) -> bool {
        glyph.glyph != 0
    }

    fn multi_byte_cluster(glyph: &GlyphDiagnostic) -> bool {
        glyph.cluster[1] - glyph.cluster[0] > 1
    }

    #[test]
    fn placement_and_color_do_not_reshape_but_font_reload_does() {
        let mut doc = TextDocument::new();
        let mut item = label("AVATAR");
        doc.set_labels(vec![item.clone()]).unwrap();
        let before = doc.diagnostics()[0].line_width;
        item.color = [255, 255, 0, 255];
        item.placement = TextPlacement::Screen {
            left: 0.375,
            top: 1.25,
        };
        doc.set_labels(vec![item]).unwrap();
        assert_eq!(doc.shape_count, 1);
        assert_eq!(doc.diagnostics()[0].line_width, before);
        assert!(doc.replace_fonts(vec![vec![0; 8]]).is_err());
        assert_eq!(doc.shape_count, 1);
        doc.replace_fonts(vec![
            FONT_BYTES.to_vec(),
            FALLBACK_BYTES.to_vec(),
            SYMBOL_BYTES.to_vec(),
        ])
        .unwrap();
        assert_eq!(doc.shape_count, 2);
        assert_eq!(doc.diagnostics()[0].line_width, before);
    }

    #[test]
    fn composed_decomposed_accents_and_multiline_baselines_match() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label("é\ne\u{301}")])
            .unwrap();
        let glyphs = doc.diagnostics();
        assert_eq!(glyphs.len(), 2);
        assert_eq!(glyphs[0].glyph, glyphs[1].glyph);
        assert_eq!(glyphs[0].advance, glyphs[1].advance);
        assert!((glyphs[1].baseline - glyphs[0].baseline - 24.0).abs() < 0.001);
    }

    #[test]
    fn bundled_fallback_covers_symbols_without_system_fonts() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label("CAD ⚙ ⏳ ⌘")])
            .unwrap();
        let glyphs = doc.diagnostics();
        assert!(glyphs.iter().all(has_glyph));
        let mut faces = std::collections::HashSet::new();

        for glyph in glyphs {
            faces.insert(glyph.font);
        }

        assert!(
            faces.len() >= 2,
            "sample must exercise explicit font fallback"
        );
    }

    #[test]
    fn invalid_replacement_preserves_current_document() {
        let mut doc = TextDocument::new();
        doc.set_labels(vec![label("Keep")]).unwrap();
        let mut invalid = label("Reject");
        invalid.font_size = f32::NAN;
        assert!(doc.set_labels(vec![invalid]).is_err());
        assert_eq!(doc.runs[0].label.text, "Keep");
    }
}

#[cfg(test)]
mod commands_tests {
    use super::tests::label;
    use super::*;

    #[test]
    fn bundled_subsets_cover_the_viewer_and_whole_fonts_the_rest() {
        for name in crate::commands::names() {
            assert!(covers(name), "{name}");
        }

        for text in [
            "Ąą Čč Ęę Ėė Įį Šš Ųų Ūū Žž",
            "Äußere Wände, Größe, Straße ẞ, „Zitat“ »Guillemets«",
            "Ø 12 ± 0,5 mm, 45°, m² m³, µm",
            "→ ■ ⏵ ◻ ⚙ ⏳ ⌘",
            "+ –",
        ] {
            assert!(covers(text), "{text}");
        }

        assert!(!covers("Fixed Ω cube"));
        let whole: Vec<Vec<u8>> = FULL_FONTS
            .iter()
            .map(|name| std::fs::read(format!("assets/text/{name}")).unwrap())
            .collect();
        let mut doc = TextDocument::new();
        doc.replace_fonts(whole).unwrap();
        doc.set_labels(vec![label("Fixed Ω cube")])
            .unwrap();
        assert!(
            doc.diagnostics()
                .iter()
                .all(|glyph| glyph.glyph != 0)
        );
    }
}
```

## Build & Run

![Recorded visual output for Lesson 10.2 (60g-text-shaping)](../../screenshots/journey/60g-text-shaping.png)

```sh
npm --prefix ../session_tests run course -- dependencies 10.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 10.2
npm --prefix ../session_tests run course -- save 10.2
```

Continue with [10.3 · A shared glyph atlas and lazy text drawing](61-atlas.md).
