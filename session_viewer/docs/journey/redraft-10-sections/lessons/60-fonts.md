# 10.1 · Explicit fonts and source label placement

**Steps:** 3 · **Source:** [engine/text.rs](../../../../src/engine/text.rs) · [engine/gpu/text.rs](../../../../src/engine/gpu/text.rs) · [engine/gpu/text_plane.rs](../../../../src/engine/gpu/text_plane.rs) · [engine/gpu/text_plate.rs](../../../../src/engine/gpu/text_plate.rs)

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

## <a id="checkpoint-60-fonts"></a>1. Pin scene-text shaping and supply the viewer fonts

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
serde_json = "1.0"

[dev-dependencies]
```

Edit the existing block to match this code; keep unchanged lines:

```toml
serde_json = "1.0"

glyphon = "=0.11.0"
anyhow = "=1.0.104"
ttf-parser = {version="=0.25.1",default-features=false}

[dev-dependencies]
```

### 2. `index.html`

**Locate this existing block:**

```html
  <link data-trunk rel="rust">
  <style>
```

Edit the existing block to match this code; keep unchanged lines:

```html
  <link data-trunk rel="rust">
  <link data-trunk rel="copy-dir" href="assets/text" data-target-path="text">
  <style>
```

## <a id="checkpoint-60a-labels"></a>2. Keep source words metrics and placement together

### 1. `src/text.rs`

Create the file and type:

```rust
use glyphon::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Wrap, fontdb};
use serde::Serialize;

pub const FONT_FAMILY: &str = "Noto Sans";

pub static FONT_BYTES: &[u8] =
    include_bytes!("../assets/text/NotoSans-Regular.subset.ttf");

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
        if self
            .object
            .is_some_and(|object| object.selected)
        {
            [0, 0, 0, 255]
        } else {
            self.color
        }
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod cad_display_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod cad_display_tests;

pub mod text;
```

## <a id="checkpoint-60b-coverage"></a>3. Load explicit fonts and inspect missing coverage

### 1. `src/text.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

pub fn covers(text: &str) -> bool {
    text.is_ascii()
        || text
            .chars()
            .all(|c| c.is_control() || bundled_glyph(c))
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
            map.subtables.into_iter().any(|table| {
                table.is_unicode() && table.glyph_index(u32::from(c)).is_some()
            })
        })
    })
}

fn bundled_fonts() -> FontSystem {
    let mut db = fontdb::Database::new();

    for bytes in [FONT_BYTES, FALLBACK_BYTES, SYMBOL_BYTES] {
        db.load_font_source(fontdb::Source::Binary(std::sync::Arc::new(
            bytes,
        )));
    }

    db.set_sans_serif_family(FONT_FAMILY);
    FontSystem::new_with_locale_and_db("en-US".into(), db)
}
```

## Build & Run

![Recorded visual output for Lesson 10.1 (60b-coverage)](../../../screenshots/journey/60b-coverage.png)

```sh
npm --prefix ../session_tests run course -- dependencies 10.1
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 10.1
npm --prefix ../session_tests run course -- save 10.1
```

Continue with [10.2 · Validated shaping and revision-based reuse](../../consolidation/60ba-validation.md).
