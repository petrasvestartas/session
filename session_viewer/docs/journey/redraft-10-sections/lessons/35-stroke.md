# 5.1 · Original lines and a screen-space stroke lane

**Steps:** 7 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl)

## <a id="checkpoint-35-stroke"></a>1. Prepare a stroke without changing its source

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-ff2d7a7b" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-ff2d7a7b" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-ff2d7a7b" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-ff2d7a7b" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-ff2d7a7b" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">35-STROKE · SCREEN RIBBON</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Screen-Space Line Extrusion in @vertex &amp; Instanced Quad Lane</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">p_clip = clip + vec4(n_px * (w/2) * (2/vp) * clip.w, 0, 0)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-ff2d7a7b)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Project Endpoints A, B -&gt; Screen Tangent t</text>
  <line x1="68" y1="158" x2="296" y2="92" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="68" cy="158" r="4.5" fill="#dc2626"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="64" y="176" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">A_ndc</text>
  <circle cx="296" cy="92" r="4.5" fill="#2563eb"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="292" y="82" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">B_ndc</text>
  <line x1="176" y1="127" x2="232" y2="111" stroke="#2563eb" stroke-width="2.2" marker-end="url(#arr-b-ff2d7a7b)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="218" y="134" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">t</text>
  <line x1="176" y1="127" x2="162" y2="79" stroke="#16a34a" stroke-width="2.2" marker-end="url(#arr-g-ff2d7a7b)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="142" y="88" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">n=(-t.y,t.x)</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-ff2d7a7b)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-ff2d7a7b)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · 6-Vertex Instanced Quad Expansion (±w/2 px)</text>
  <polygon points="434,132 664,74 678,118 448,176" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="434" y1="132" x2="678" y2="118" stroke="#93c5fd" stroke-width="1.2" stroke-dasharray="4 3"/>
  <line x1="441" y1="154" x2="671" y2="96" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3"/>
  <circle cx="434" cy="132" r="3.5" fill="#111111"/>
  <circle cx="448" cy="176" r="3.5" fill="#111111"/>
  <circle cx="664" cy="74" r="3.5" fill="#111111"/>
  <circle cx="678" cy="118" r="3.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="186" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Multiply by clip.w to cancel 1/w foreshortening</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-ff2d7a7b)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-ff2d7a7b)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Constant Pixel Width at Any 3D Zoom</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="836" y1="162" x2="1032" y2="88" stroke="#2563eb" stroke-width="6" stroke-linecap="round"/>
  <circle cx="836" cy="162" r="4" fill="#dc2626"/>
  <circle cx="1032" cy="88" r="4" fill="#dc2626"/>
</svg>
</div>

### 1. `src/stroke.rs`

Create the file and type:

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stroke {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub colour: [f32; 4],
    pub width: f32,
}

#[derive(Clone)]
pub struct PreparedLine {
    pub source: Rc<session_rust::Line>,
    pub display: Stroke,
}

impl PreparedLine {
    pub fn new(source: Rc<session_rust::Line>) -> Result<Self, &'static str> {
        let _ = source.guid();
        let start = std::array::from_fn(|i| source.start()[i] as f32);
        let end = std::array::from_fn(|i| source.end()[i] as f32);
        let colour = source.linecolor.to_f32();
        let width = if source.width.is_finite() && source.width > 0.0 {
            source.width as f32
        } else {
            1.0
        };
        let width = if width == 0.0 {
            1.0
        } else {
            width
        };
        let display = Stroke {
            start,
            end,
            colour,
            width,
        };
        if display
            .values()
            .iter()
            .any(|value| !value.is_finite())
        {
            return Err("Line exceeds the display range");
        }
        Ok(Self { source, display })
    }
}

impl Stroke {
    pub fn values(&self) -> [f32; 11] {
        let [a, b, c] = self.start;
        let [d, e, f] = self.end;
        let [r, g, blue, alpha] = self.colour;
        [
            a, b, c, d, e, f, r, g, blue, alpha, self.width,
        ]
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.values()
            .into_iter()
            .flat_map(f32::to_ne_bytes)
            .collect()
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod startup_tests;
pub mod suspension;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod startup_tests;
pub mod stroke;
#[cfg(test)]
mod stroke_tests;
pub mod suspension;
```

### 3. `src/stroke_tests.rs`

Create the file and type:

```rust
use crate::stroke::PreparedLine;
use session_rust::{Color, Line};
use std::rc::Rc;

#[test]
fn display_packing_preserves_the_original_source_owner_and_precision() {
    let mut line = Line::new(0.123456789012345, 0.0, 0.0, 1.0, 2.0, 3.0);
    line.linecolor = Color::new(0.2, 0.4, 0.6, 1.0);
    line.width = 3.0;
    let source = Rc::new(line);
    let weak = Rc::downgrade(&source);
    let first = PreparedLine::new(Rc::clone(&source)).unwrap();
    let second = PreparedLine::new(Rc::clone(&source)).unwrap();
    assert!(Rc::ptr_eq(&first.source, &second.source));
    assert_eq!(first.source.start()[0], 0.123456789012345);
    assert_ne!(
        first.display.start[0] as f64,
        first.source.start()[0]
    );
    assert_eq!(first.display.colour, [0.2, 0.4, 0.6, 1.0]);
    assert_eq!(first.display.width, 3.0);
    assert_eq!(first.display.bytes().len(), 44);
    assert_eq!(
        f32::from_ne_bytes(
            first.display.bytes()[40..44]
                .try_into()
                .unwrap()
        ),
        3.0
    );
    let guid = first.source.guid().to_owned();
    drop(first);
    drop(source);
    assert_eq!(second.source.guid(), guid);
    assert!(weak.upgrade().is_some());
    drop(second);
    assert!(weak.upgrade().is_none());
}

#[test]
fn default_widths_and_invalid_display_coordinates_follow_their_separate_rules() {
    for width in [0.0, -2.0, f64::NAN, f64::INFINITY, 1e-100] {
        let mut line = Line::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        line.width = width;
        assert_eq!(
            PreparedLine::new(Rc::new(line))
                .unwrap()
                .display
                .width,
            1.0
        );
    }
    for value in [f64::NAN, f64::INFINITY, 1e100] {
        assert!(
            PreparedLine::new(Rc::new(Line::new(
                value, 0.0, 0.0, 1.0, 0.0, 0.0
            )))
            .is_err()
        );
    }
    let mut wide = Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    wide.width = 1e100;
    assert!(PreparedLine::new(Rc::new(wide)).is_err());
}
```

## <a id="checkpoint-35a-document"></a>2. Retain line owners in document history

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
```

Replace it with:

```rust
    }

    pub fn clear(&mut self) {
        self.lines = Vec::new();
        self.objects = Vec::new();
```

**Locate this block:**

```rust
        Some(bounds)
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
```

Replace it with:

```rust
        Some(bounds)
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        row.model = model;
        Ok(())
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }
```

**Locate this block:**

```rust
                }
            }
        }
        bounds
    }
```

Replace it with:

```rust
                }
            }
        }
        for line in &self.lines {
            for point in line.endpoints() {
                let point = [point[0], point[1], point[2]];
                match &mut bounds {
                    Some(bounds) => bounds.include(point),
                    None => bounds = Some(crate::bounds::Bounds::point(point)),
                }
            }
        }
        bounds
    }
```

**Locate this block:**

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            next_id: 1,
            extra: None,
        };
```

Replace it with:

```rust
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            lines: Vec::new(),
            next_id: 1,
            extra: None,
        };
```

**Locate this block:**

```rust
#[derive(Clone)]
pub struct Scene {
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
```

Replace it with:

```rust
#[derive(Clone)]
pub struct LineObject {
    pub id: ObjectId,
    pub prepared: crate::stroke::PreparedLine,
    pub model: session_rust::Xform,
}

impl LineObject {
    pub fn endpoints(&self) -> [session_rust::Point; 2] {
        let line = &self.prepared.source;
        [
            self.model.transform_point(&line.start()),
            self.model.transform_point(&line.end()),
        ]
    }

    pub fn stroke(&self) -> crate::stroke::Stroke {
        let [start, end] = self.endpoints();
        crate::stroke::Stroke {
            start: std::array::from_fn(|i| start[i] as f32),
            end: std::array::from_fn(|i| end[i] as f32),
            ..self.prepared.display
        }
    }
}

#[derive(Clone)]
pub struct Scene {
    lines: Vec<LineObject>,
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
```

### 2. `src/document.rs`

**Locate this existing block:**

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    let mut objects = proto::Objects::default();
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn snapshot(scene: &crate::scene::Scene) -> Result<Vec<u8>, &'static str> {
    if !scene.lines().is_empty() {
        return Err("This checkpoint cannot save line objects yet");
    }
    let mut objects = proto::Objects::default();
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod history;
pub mod load_measure;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod history;
#[cfg(test)]
mod line_scene_tests;
pub mod load_measure;
```

### 4. `src/line_scene_tests.rs`

Create the file and type:

```rust
use crate::{document, history::History, scene::Scene, stroke::PreparedLine};
use session_rust::{Line, Xform};
use std::rc::Rc;

#[test]
fn line_rows_keep_distinct_ids_original_sources_and_placed_bounds() {
    let mut scene = Scene::demo();
    let source = Rc::new(Line::new(
        2.123456789,
        0.0,
        0.0,
        4.0,
        1.0,
        2.0,
    ));
    let id = scene
        .insert_line(PreparedLine::new(Rc::clone(&source)).unwrap())
        .unwrap();
    assert!(
        scene
            .objects()
            .iter()
            .all(|row| row.id != id)
    );
    scene
        .place_line(id, Xform::translation(10.0, 0.0, 0.0))
        .unwrap();
    assert!((scene.lines()[0].endpoints()[0][0] - 12.123456789).abs() < 1e-12);
    assert_eq!(scene.bounds().unwrap().max[0], 14.0);
    assert_eq!(source.start()[0], 2.123456789);
    assert!(document::snapshot(&scene).is_err());
}

#[test]
fn history_shares_line_sources_and_close_releases_every_owner() {
    let mut scene = Scene::demo();
    let mut history = History::default();
    let source = Rc::new(Line::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0));
    let weak = Rc::downgrade(&source);
    let id = scene
        .insert_line(PreparedLine::new(source).unwrap())
        .unwrap();
    history
        .try_edit(&mut scene, |scene| {
            scene.place_line(id, Xform::translation(3.0, 0.0, 0.0))
        })
        .unwrap();
    assert!(Rc::ptr_eq(
        &scene.lines()[0].prepared.source,
        &history.scenes().next().unwrap().lines()[0]
            .prepared
            .source
    ));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 3.0);
    assert!(history.undo(&mut scene));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 0.0);
    assert!(history.redo(&mut scene));
    assert_eq!(scene.lines()[0].endpoints()[0][0], 3.0);
    scene.clear();
    assert!(weak.upgrade().is_some());
    history.clear();
    assert!(weak.upgrade().is_none());
    assert!(scene.lines().is_empty());
    assert!(scene.objects().is_empty());
    assert!(scene.bounds().is_none());
}
```

## <a id="checkpoint-35b-extrude"></a>3. Extrude a stroke in screen pixels

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Profile Curve C(u) + Extrude Vector d / Revolve Axis A</text>
  <path d="M 64 138 Q 114 82 164 138" fill="none" stroke="#dc2626" stroke-width="3"/>
  <line x1="114" y1="110" x2="114" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="234" y1="46" x2="234" y2="156" stroke="#2563eb" stroke-width="2" stroke-dasharray="5 3"/>
  <path d="M 254 136 Q 282 100 254 64" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">S_ext(u,v) = C(u) + v*d  |  S_rev(u,v) = Rot_A(v) * C(u)</text>
  <polygon points="426,138 476,92 546,92 496,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="426,96 476,50 546,50 496,96" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="426" y1="138" x2="426" y2="96" stroke="#16a34a" stroke-width="2"/>
  <ellipse cx="638" cy="72" rx="42" ry="14" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <ellipse cx="638" cy="132" rx="42" ry="14" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="596" y1="72" x2="596" y2="132" stroke="#111111" stroke-width="1.75"/>
  <line x1="680" y1="72" x2="680" y2="132" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Extruded Prism &amp; Revolved Surface of Revolution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="832,138 866,76 924,76 890,138" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <ellipse cx="992" cy="74" rx="36" ry="12" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <path d="M 956 74 L 956 134 A 36 12 0 0 0 1028 134 L 1028 74" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

### 1. `src/stroke.wgsl`

Create the file and type:

```wgsl
struct View { transform: mat4x4<f32>, size: vec2<f32>, density: f32 }
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0);
}

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (a.z < 0.0 && b.z < 0.0) { return hidden(colour); }
    if (a.z < 0.0) { a = mix(a, b, -a.z / (b.z - a.z)); }
    if (b.z < 0.0) { b = mix(b, a, -b.z / (a.z - b.z)); }
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width);
}

@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {
    let coverage = clamp(input.half_width + 0.5 - abs(input.side), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

### 2. `src/stroke.rs`

**Locate this existing block:**

```rust
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const SHADER: &str = include_str!("stroke.wgsl");
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq)]
```

## <a id="checkpoint-35c-lane"></a>4. Create the shared stroke pipeline

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-8987bfd9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-8987bfd9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-8987bfd9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-8987bfd9" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-8987bfd9" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">35C..35D · INSTANCED LANE</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Instanced Segment Buffer (draw(0..6, 0..N)) &amp; Fragment SDF Edge Anti-Aliasing</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">v_id ∈ 0..6 (Quad Corner) · i_id ∈ 0..N (Segment)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-8987bfd9)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Per-Instance Segment Record (VertexStepMode::Instance)</text>
  <rect x="44" y="76" width="284" height="104" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">struct GpuSegment {</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="118" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">  p0: [f32; 3], width_px: f32,</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="138" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">  p1: [f32; 3], color: [f32; 4] }</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="162" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">1 GPU draw call renders N 3D lines</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-8987bfd9)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-8987bfd9)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Lateral Edge Distance v_edge ∈ [-1, +1]</text>
  <rect x="434" y="88" width="252" height="72" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="434" y1="124" x2="686" y2="124" stroke="#dc2626" stroke-width="2" stroke-dasharray="4 3"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="82" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">v_edge = +1.0 (Top Border)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="118" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#dc2626">v_edge = 0.0 (Centerline)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="176" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">alpha = 1.0 - smoothstep(1.0 - aa, 1.0, |v_edge|)</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-8987bfd9)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-8987bfd9)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Crisp Anti-Aliased 3D Wireframes</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="836" y1="164" x2="946" y2="82" stroke="#2563eb" stroke-width="5" stroke-linecap="round"/>
  <line x1="946" y1="82" x2="1034" y2="152" stroke="#16a34a" stroke-width="5" stroke-linecap="round"/>
</svg>
</div>

### 1. `src/stroke_gpu.rs`

Create the file and type:

```rust
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    uniform: wgpu::Buffer,
    group: wgpu::BindGroup,
    vertices: Option<wgpu::Buffer>,
    data: Vec<u8>,
    pub uploads: u64,
    pub uploaded_bytes: u64,
}

impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(crate::stroke::SHADER.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"), layout: None,
            vertex: wgpu::VertexState { module: &shader, entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout { array_stride: 44, step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32] }],
            },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState { format, blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL })],
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(), bias: Default::default() }),
            multisample: Default::default(), multiview_mask: None, cache: None,
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("stroke view"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("stroke view"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
        });
        Self {
            pipeline,
            uniform,
            group,
            vertices: None,
            data: Vec::new(),
            uploads: 0,
            uploaded_bytes: 0,
        }
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod stroke;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod stroke;
pub mod stroke_gpu;
#[cfg(test)]
```

## <a id="checkpoint-35ca-sync"></a>5. Upload changed stroke instances only

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Camera / View Uniform (16-Byte Aligned)</text>
  <rect x="52" y="48" width="268" height="106" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="66" y="58" width="240" height="38" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="81" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">view_proj: [[f32; 4]; 4] (64B)</text>
  <rect x="66" y="104" width="240" height="36" fill="#fef08a" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">eye_pos / viewport_px (16B)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_buffer -&gt; wgpu::BindGroup (@group(0))</text>
  <rect x="424" y="50" width="272" height="102" rx="6" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="442" y="64" width="236" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">BufferUsages::UNIFORM | COPY_DST</text>
  <rect x="442" y="106" width="236" height="32" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="126" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">pass.set_bind_group(0, &amp;bg, &amp;[])</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Shader Uniform Read: var&lt;uniform&gt;</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">@group(0) @binding(0)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">var&lt;uniform&gt; u: Uniforms;</text>
  <rect x="828" y="114" width="212" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">u.view_proj * vec4(p, 1.0)</text>
</svg>
</div>

### 1. `src/stroke_gpu.rs`

**Locate this existing block:**

```rust
impl Lane {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Lane {
    pub fn set(
        &mut self,
        device: &wgpu::Device,
        strokes: impl IntoIterator<Item = crate::stroke::Stroke>,
    ) -> Result<(), &'static str> {
        use wgpu::util::DeviceExt;
        let mut data = Vec::new();
        for stroke in strokes {
            if stroke.width <= 0.0
                || stroke
                    .values()
                    .iter()
                    .any(|value| !value.is_finite())
            {
                return Err("Invalid stroke display data");
            }
            data.extend(stroke.bytes());
        }
        u32::try_from(data.len() / 44).map_err(|_| "Too many stroke instances")?;
        if data == self.data {
            return Ok(());
        }
        self.vertices = if data.is_empty() {
            None
        } else {
            self.uploads += 1;
            self.uploaded_bytes += data.len() as u64;
            Some(
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("stroke instances"),
                    contents: &data,
                    usage: wgpu::BufferUsages::VERTEX,
                }),
            )
        };
        self.data = data;
        Ok(())
    }

    pub fn view(
        &self,
        queue: &wgpu::Queue,
        transform: &[f32; 16],
        size: [u32; 2],
        density: f32,
    ) {
        if self.vertices.is_none() {
            return;
        }
        let bytes: Vec<u8> = transform
            .iter()
            .copied()
            .chain([size[0] as f32, size[1] as f32, density, 0.0])
            .flat_map(f32::to_ne_bytes)
            .collect();
        queue.write_buffer(&self.uniform, 0, &bytes);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vertices) = &self.vertices else {
            return;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..6, 0..(self.data.len() / 44) as u32);
    }

    pub fn usage(&self) -> [u64; 2] {
        [
            self.data.len() as u64 / 44,
            self.vertices
                .as_ref()
                .map_or(0, wgpu::Buffer::size),
        ]
    }

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
```

## <a id="checkpoint-35cb-render"></a>6. Connect the stroke lane to the document renderer

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU RAM: #[repr(C)] Structs -&gt; bytemuck::cast_slice</text>
  <rect x="46" y="54" width="280" height="44" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="52" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="84" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v0: 24B</text>
  <rect x="122" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="154" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v1: 24B</text>
  <rect x="192" y="60" width="64" height="32" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="224" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">v2: 24B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="290" y="80" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">...</text>
  <rect x="66" y="112" width="240" height="34" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="133" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">&amp;[u8] Contiguous Byte Slice</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU -&gt; GPU DMA: queue.write_buffer(buf, 0, bytes)</text>
  <rect x="424" y="54" width="272" height="92" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <line x1="448" y1="84" x2="672" y2="84" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">create_buffer_init / write_buffer</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="112" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">VERTEX | INDEX | COPY_DST</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="132" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">array_stride = size_of::&lt;Vertex&gt;()</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU VRAM Buffer -&gt; @vertex Input Locations</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="56" width="224" height="40" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="80" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(0) pos: vec3&lt;f32&gt;</text>
  <rect x="822" y="106" width="224" height="40" rx="4" fill="#fce7f3" stroke="#db2777" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="130" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">@location(1) color: vec3&lt;f32&gt;</text>
</svg>
</div>

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
        }
```

Replace it with:

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.strokes.draw(&mut pass);
        }
```

**Locate this block:**

```rust
            .collect();
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

Replace it with:

```rust
            .collect();
        self.queue
            .write_buffer(&self.uniform, 0, &bytes);
        self.strokes.view(
            &self.queue,
            transform,
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

**Locate this block:**

```rust
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 80
            + self.settings_bytes as u64
    }

    pub fn stats(&self) -> [usize; 4] {
```

Replace it with:

```rust
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 80
            + self.settings_bytes as u64
            + self.strokes.uploaded_bytes
    }

    pub fn stats(&self) -> [usize; 4] {
```

**Locate this block:**

```rust
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects() {
```

Replace it with:

```rust
    }

    pub fn resize(&mut self, size: crate::viewport::Viewport) {
        self.size = [size.width, size.height];
        self.depth = Self::depth(&self.device, size.width, size.height);
    }

    pub fn set_density(&mut self, density: f32) {
        if density.is_finite() && density > 0.0 {
            self.density = density;
        }
    }

    pub fn stroke_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.strokes.usage();
        [
            count,
            bytes,
            self.strokes.uploads,
            self.strokes.uploaded_bytes,
        ]
    }

    pub fn set_scene(&mut self, scene: &Scene, selected: Option<ObjectId>) {
        self.strokes
            .set(
                &self.device,
                scene
                    .lines()
                    .iter()
                    .map(crate::scene::LineObject::stroke),
            )
            .expect("Prepared line placement fits the display range");
        let layout = self.pipeline.get_bind_group_layout(1);
        let mut old = std::mem::take(&mut self.meshes);
        for object in scene.objects() {
```

**Locate this block:**

```rust
            uniform,
            view_group,
            depth,
        };
        renderer.set_scene(scene, None);
        renderer
```

Replace it with:

```rust
            uniform,
            view_group,
            depth,
            strokes,
            size: [640, 480],
            density: 1.0,
        };
        renderer.set_scene(scene, None);
        renderer
```

**Locate this block:**

```rust
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let mut renderer = Self {
            device,
            queue,
```

Replace it with:

```rust
            }],
        });
        let depth = Self::depth(&device, 640, 480);
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

**Locate this block:**

```rust
    depth: wgpu::TextureView,
}

impl Renderer {
```

Replace it with:

```rust
    depth: wgpu::TextureView,
    strokes: crate::stroke_gpu::Lane,
    size: [u32; 2],
    density: f32,
}

impl Renderer {
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
        .max_texture_dimension_2d;
    let Some(size) = Viewport::from_css(
        rect.width(),
        rect.height(),
        crate::browser_recovery::density(window.device_pixel_ratio()),
        limit,
    ) else {
        return false;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .max_texture_dimension_2d;
    let density = crate::browser_recovery::density(window.device_pixel_ratio());
    renderer.set_density(density as f32);
    let Some(size) = Viewport::from_css(rect.width(), rect.height(), density, limit)
    else {
        return false;
```

### 3. `src/memory.rs`

**Locate this existing block:**

```rust
}

pub fn gpu<'a>(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn lines<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 3] {
    let mut sources = HashSet::new();
    let mut count = [0; 3];
    for scene in scenes {
        count[0] += scene.lines().len();
        for row in scene.lines() {
            sources.insert(Rc::as_ptr(&row.prepared.source));
        }
        count[2] += scene.line_capacity_bytes();
    }
    count[1] = sources.len();
    count
}

pub fn gpu<'a>(
```

### 4. `src/scene.rs`

**Locate this existing block:**

```rust
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        row.model = model;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn line_capacity_bytes(&self) -> usize {
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn lines(&self) -> &[LineObject] {
        &self.lines
    }

    pub fn insert_line(
        &mut self,
        prepared: crate::stroke::PreparedLine,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.lines.push(LineObject {
            id,
            prepared,
            model: session_rust::Xform::identity(),
        });
        Ok(id)
    }

    pub fn place_line(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid line placement");
        }
        let row = self
            .lines
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Line not found")?;
        for point in [
            row.prepared.source.start(),
            row.prepared.source.end(),
        ] {
            let placed = model.transform_point(&point);
            if (0..3).any(|i| !(placed[i] as f32).is_finite()) {
                return Err("Line placement exceeds the display range");
            }
        }
        row.model = model;
```

### 5. `src/browser.rs`

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-selected-id",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-stroke-usage",
        &serde_json::to_string(&renderer.stroke_usage()).unwrap(),
    )?;
    canvas.set_attribute(
        "data-line-usage",
        &serde_json::to_string(&crate::memory::lines(editor.scenes())).unwrap(),
    )?;
    canvas.set_attribute(
        "data-selected-id",
```

## <a id="checkpoint-35d-input"></a>7. Insert a line through the real command dock

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
```

Replace it with:

```rust
                let loaded = crate::document::load_at(&bytes, Some(location))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.replace(loaded)
                    })?;
            }
            Action::AddLine => {
                let mut line = session_rust::Line::new(-0.6, 0.0, 0.0, 0.6, 0.0, 0.0);
                line.width = 5.0;
                line.linecolor = session_rust::Color::new(0.0, 0.0, 0.0, 1.0);
                let prepared =
                    crate::stroke::PreparedLine::new(std::rc::Rc::new(line))?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_line(prepared).map(|_| ())
                    })?;
```

**Locate this block:**

```rust
    AddBox,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
```

Replace it with:

```rust
    AddBox,
    AddLine,
    Translate([f64; 3]),
    ToggleExtra,
    SelectNext,
```

### 2. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                } else {
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example triangle" => Action::ToggleExtra,
```

Replace it with:

```rust
                } else {
                    let action = match line.as_str() {
                        "example box" => Action::AddBox,
                        "example line" => Action::AddLine,
                        "example triangle" => Action::ToggleExtra,
```

**Locate this block:**

```rust
            "Example Box",
            "Example Triangle",
            "Select Next",
            "Delete",
```

Replace it with:

```rust
            "Example Box",
            "Example Line",
            "Example Triangle",
            "Select Next",
            "Delete",
```

### 3. `src/stroke_input_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    memory,
};

#[test]
fn line_command_is_one_transaction_and_close_releases_every_source() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor.apply(Action::AddLine).unwrap();
    let row = &editor.scene.lines()[0];
    let id = row.id;
    let weak = std::rc::Rc::downgrade(&row.prepared.source);
    assert_eq!(row.prepared.source.start()[0], -0.6);
    assert_eq!(row.prepared.display.width, 5.0);
    editor.apply(Action::Zoom(2.0)).unwrap();
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.lines().is_empty());
    assert!(weak.upgrade().is_some());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.lines()[0].id, id);
    editor.apply(Action::Undo).unwrap();
    editor.apply(Action::AddLine).unwrap();
    assert_ne!(editor.scene.lines()[0].id, id);
    assert!(weak.upgrade().is_none());
    editor.apply(Action::Close).unwrap();
    assert_eq!(memory::lines(editor.scenes()), [0; 3]);
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.lines().is_empty());
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod stroke_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
```

## Build & Run

![Recorded visual output for Lesson 5.1 (35d-input)](../../../screenshots/journey/35d-input-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.1
npm --prefix ../session_tests run course -- save 5.1
```

Continue with [5.2 · Joined paths and headed ends](../../consolidation/36-chain.md).
