# 10.4 · Original world labels, plates and integer picks

**Steps:** 37 · **Source:** [engine/text.rs](../../../src/engine/text.rs) · [engine/gpu/text.rs](../../../src/engine/gpu/text.rs) · [engine/gpu/text_plane.rs](../../../src/engine/gpu/text_plane.rs) · [engine/gpu/text_plate.rs](../../../src/engine/gpu/text_plate.rs)

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

## <a id="checkpoint-62-source"></a>1. Own scene words and font metrics

### 1. `src/scene_label.rs`

Create the file and type:

```rust
use crate::text::{TextDocument, TextLabel, TextPlacement};
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Source {
    pub guid: String,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub label: TextLabel,
}
#[derive(Clone)]
pub struct Prepared {
    pub source: Rc<Source>,
    pub extent: [f64; 2],
}
impl Prepared {
    pub fn new(source: Rc<Source>) -> Result<Self, &'static str> {
        if matches!(
            source.label.placement,
            TextPlacement::Screen { .. }
        ) {
            return Err("Screen presentation text has no scene row");
        }
        if source.label.object.is_some() {
            return Err("Original text does not carry a viewer pick address");
        }
        let mut document = TextDocument::new();
        document
            .set_labels(vec![source.label.clone()])
            .map_err(|_| "Invalid scene text source")?;
        let extent =
            document.runs[0]
                .buffer
                .layout_runs()
                .fold([0f64; 2], |[w, h], line| {
                    [
                        w.max(line.line_w as f64),
                        h.max((line.line_top + line.line_height) as f64),
                    ]
                });
        Ok(Self { source, extent })
    }
}
pub fn example(placement: TextPlacement) -> Source {
    let mut label = crate::text_example::label();
    label.text = "Original CAD · Ø25 · Ω".into();
    label.color = [255; 4];
    label.placement = placement;
    Source {
        guid: uuid::Uuid::new_v4().to_string(),
        name: "Original CAD label".into(),
        visible: true,
        locked: false,
        label,
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
#[cfg(test)]
mod text_atlas_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod text_atlas_tests;

pub mod scene_label;
```

## <a id="checkpoint-62a-placements"></a>2. Transform the label frame without changing its source

### 1. `src/text.rs`

**Locate this existing block:**

```rust
}

fn validate_label(label: &TextLabel) -> anyhow::Result<()> {
    anyhow::ensure!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn validate_label(label: &TextLabel) -> anyhow::Result<()> {
    anyhow::ensure!(
```

### 2. `src/scene_label.rs`

**Locate this existing block:**

```rust
}
pub fn example(placement: TextPlacement) -> Source {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
#[derive(Clone)]
pub struct Row {
    pub id: crate::scene::ObjectId,
    pub guid: String,
    pub prepared: Prepared,
    pub model: session_rust::Xform,
}
impl Row {
    pub fn label(
        &self,
        selected: bool,
        pickable: bool,
    ) -> Result<TextLabel, &'static str> {
        let mut label = self.prepared.source.label.clone();
        label.id = self.id.value();
        label.object = Some(crate::text::TextObject {
            row: if pickable {
                self.id.value() - 1
            } else {
                u32::MAX
            },
            selected,
        });
        let point = |p: [f64; 3]| {
            let p = self
                .model
                .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
            [p[0], p[1], p[2]]
        };
        let vector = |v: [f64; 3]| {
            std::array::from_fn::<_, 3, _>(|i| {
                (0..3)
                    .map(|j| self.model.m[j * 4 + i] * v[j])
                    .sum::<f64>()
            })
        };
        match &mut label.placement {
            TextPlacement::WorldPlane {
                world,
                right,
                up,
                world_height,
            } => {
                *world = point(*world);
                let r = vector(*right);
                let u = vector(*up);
                let rn = r.iter().map(|n| n * n).sum::<f64>().sqrt();
                let un = u.iter().map(|n| n * n).sum::<f64>().sqrt();
                if !rn.is_finite()
                    || rn <= 0.
                    || !un.is_finite()
                    || un <= 0.
                    || (rn - un).abs() > rn.max(un) * 1e-6
                    || r.iter()
                        .zip(u)
                        .map(|(a, b)| a * b)
                        .sum::<f64>()
                        .abs()
                        > rn * un * 1e-6
                {
                    return Err(
                        "Text plane placement must preserve its orthogonal equal-scale axes",
                    );
                }
                *right = r.map(|v| v / rn);
                *up = u.map(|v| v / un);
                *world_height *= rn;
            }
            TextPlacement::WorldBillboard {
                world,
                world_height,
            } => {
                *world = point(*world);
                let x = vector([1., 0., 0.]);
                let y = vector([0., 1., 0.]);
                let z = vector([0., 0., 1.]);
                let lengths =
                    [x, y, z].map(|v| v.iter().map(|n| n * n).sum::<f64>().sqrt());
                let size = lengths[0];
                if !size.is_finite()
                    || size <= 0.
                    || lengths
                        .iter()
                        .any(|v| (v - size).abs() > size * 1e-6)
                    || [x, y, z].iter().enumerate().any(|(i, v)| {
                        [x, y, z].iter().skip(i + 1).any(|u| {
                            v.iter()
                                .zip(u)
                                .map(|(a, b)| a * b)
                                .sum::<f64>()
                                .abs()
                                > size * size * 1e-6
                        })
                    })
                {
                    return Err(
                        "Billboard placement must use an orthogonal uniform scale",
                    );
                }
                *world_height *= size;
            }
            TextPlacement::Anchor { world, .. }
            | TextPlacement::Nameplate { world, .. } => *world = point(*world),
            TextPlacement::Screen { .. } => {
                return Err("Screen presentation text has no scene row");
            }
        }
        crate::text::validate_label(&label).map_err(|_| "Text placement is invalid")?;
        Ok(label)
    }
}
pub fn example(placement: TextPlacement) -> Source {
```

## <a id="checkpoint-62b-bounds"></a>3. Measure the world-sized label and its padded plate

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Curve Quadrature ∑ w_i ||C'(t_i)|| &amp; Surface Patch dA = ||Su×Sv||</text>
  <path d="M 56 138 Q 112 52 176 126" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <circle cx="84" cy="98" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <circle cx="116" cy="84" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <circle cx="148" cy="98" r="4" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <polygon points="196,136 226,74 288,74 258,136" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="226" y1="105" x2="268" y2="105" stroke="#2563eb" stroke-width="2" marker-end="url(#arrB)"/>
  <line x1="226" y1="105" x2="242" y2="74" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Divergence Volume: V = (1/6) ∑ det(v0, v1, v2) &amp; Centroid C_g</text>
  <circle cx="448" cy="138" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="448" y="154" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Origin O</text>
  <polygon points="448,138 556,58 656,92" fill="#dbeafe" fill-opacity="0.55" stroke="#2563eb" stroke-width="1.5" stroke-dasharray="4 3"/>
  <polygon points="556,58 656,92 596,142" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <circle cx="576" cy="104" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Exact Length, Area, Volume &amp; Centroid Marker</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,136 894,64 988,64 940,136" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="917" cy="100" r="6" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="1016" y="96" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">C_g</text>
</svg>
</div>

### 1. `src/scene_label.rs`

**Locate this existing block:**

```rust
        Ok(label)
    }
}
pub fn example(placement: TextPlacement) -> Source {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Ok(label)
    }
    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let label = self.label(false, true).ok()?;
        let (world, right, up, height) = match label.placement {
            TextPlacement::WorldPlane {
                world,
                right,
                up,
                world_height,
            } => (world, right, up, world_height),
            TextPlacement::WorldBillboard {
                world,
                world_height,
            } => (
                world,
                [1., 0., 0.],
                [0., 1., 0.],
                world_height,
            ),
            TextPlacement::Anchor { world, .. }
            | TextPlacement::Nameplate { world, .. } => {
                return Some(crate::bounds::Bounds::point(world));
            }
            TextPlacement::Screen { .. } => return None,
        };
        let unit = height / label.font_size as f64;
        let [width, line_height] = self.prepared.extent.map(|v| v * unit);
        let vertical = height * 2. / 9.;
        let horizontal = line_height * 0.5 + vertical;
        let mut bounds = crate::bounds::Bounds::point(world);
        for x in [-horizontal, width + horizontal] {
            for y in [-vertical, line_height + vertical] {
                bounds.include(std::array::from_fn(|i| {
                    world[i] + right[i] * x - up[i] * y
                }));
            }
        }
        Some(bounds)
    }
}
pub fn example(placement: TextPlacement) -> Source {
```

## <a id="checkpoint-62c-rows"></a>4. Register text rows and release them with the scene

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.points = Vec::new();
```

Replace it with:

```rust
        self.ids = Default::default();
        self.view = Default::default();
        self.forest = Default::default();
        self.texts = Vec::new();
        self.points = Vec::new();
```

**Locate this block:**

```rust
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn points(&self) -> &[PointObject] {
        &self.points
    }
```

Replace it with:

```rust
        self.lines.capacity() * std::mem::size_of::<LineObject>()
    }

    pub fn texts(&self) -> &[crate::scene_label::Row] {
        &self.texts
    }
    pub(crate) fn texts_mut(&mut self) -> &mut [crate::scene_label::Row] {
        &mut self.texts
    }
    pub fn insert_text(
        &mut self,
        prepared: crate::scene_label::Prepared,
    ) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        let mut guid = prepared.source.guid.clone();
        while self.texts.iter().any(|row| row.guid == guid) {
            guid = uuid::Uuid::new_v4().to_string();
        }
        let label = prepared.source.name.clone();
        let row = crate::scene_label::Row {
            id,
            guid: guid.clone(),
            prepared,
            model: session_rust::Xform::identity(),
        };
        row.label(false, true)?;
        if row.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Text bounds exceed the display range");
        }
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(id);
        Rc::make_mut(&mut self.view).initial(
            id,
            row.prepared.source.visible,
            row.prepared.source.locked,
        );
        self.texts.push(row);
        crate::layer_edit::attach_generated(self, id, guid, label)?;
        Ok(id)
    }
    pub fn place_text(
        &mut self,
        id: ObjectId,
        model: session_rust::Xform,
    ) -> Result<(), &'static str> {
        if !crate::placement::valid(&model.m) {
            return Err("Invalid text placement");
        }
        let row = self
            .texts
            .iter_mut()
            .find(|row| row.id == id)
            .ok_or("Text object not found")?;
        let mut candidate = row.clone();
        candidate.model = model;
        candidate.label(false, true)?;
        if candidate.bounds().is_some_and(|b| {
            b.min
                .into_iter()
                .chain(b.max)
                .any(|v| !(v as f32).is_finite())
        }) {
            return Err("Text bounds exceed the display range");
        }
        *row = candidate;
        Ok(())
    }
    pub fn text_labels(
        &self,
        selected: &[ObjectId],
    ) -> Result<Vec<crate::text::TextLabel>, &'static str> {
        self.texts
            .iter()
            .filter(|row| self.visible(row.id))
            .map(|row| {
                row.label(
                    selected.contains(&row.id),
                    self.pick_id(row.id.value()).is_some(),
                )
            })
            .collect()
    }
    pub fn points(&self) -> &[PointObject] {
        &self.points
    }
```

**Locate this block:**

```rust
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
```

Replace it with:

```rust
            ids: Default::default(),
            view: Default::default(),
            forest: Default::default(),
            texts: Vec::new(),
            objects: Vec::new(),
            points: Vec::new(),
            paths: Vec::new(),
```

**Locate this block:**

```rust
    pub forest: crate::hierarchy::Forest,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
```

Replace it with:

```rust
    pub forest: crate::hierarchy::Forest,
    texts: Vec<crate::scene_label::Row>,
    points: Vec<PointObject>,
    paths: Vec<PathObject>,
    lines: Vec<LineObject>,
```

## <a id="checkpoint-62d-identity"></a>5. Include label owners in selection layers and history

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .chain(self.lines.iter().map(|r| r.id))
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
```

Replace it with:

```rust
            .chain(self.lines.iter().map(|r| r.id))
            .chain(self.paths.iter().map(|r| r.id))
            .chain(self.points.iter().map(|r| r.id))
            .chain(self.texts.iter().map(|r| r.id))
            .filter(|id| self.pick_id(id.value()).is_some())
```

**Locate this block:**

```rust
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
```

Replace it with:

```rust
        self.lines.retain(|row| row.id != id);
        self.paths.retain(|row| row.id != id);
        self.points.retain(|row| row.id != id);
        self.texts.retain(|row| row.id != id);
        if self.extra == Some(id) {
            self.extra = None;
        }
```

**Locate this block:**

```rust
                    .find(|row| row.id == id)
                    .map(|row| row.prepared.source.guid())
            })
    }

    pub fn clear(&mut self) {
```

Replace it with:

```rust
                    .find(|row| row.id == id)
                    .map(|row| row.prepared.source.guid())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|row| row.id == id)
                    .map(|row| row.guid.as_str())
            })
    }

    pub fn clear(&mut self) {
```

**Locate this block:**

```rust
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        }
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(copy);
```

Replace it with:

```rust
            row.id = copy;
            row.guid = guid;
            self.points.push(row);
        } else if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            let mut row = row.clone();
            row.id = copy;
            row.guid = guid;
            self.texts.push(row);
        }
        self.next_id = next;
        Rc::make_mut(&mut self.ids).insert(copy);
```

**Locate this block:**

```rust
        if !self.visible(id) {
            return None;
        }
        let object = self
            .objects
            .iter()
```

Replace it with:

```rust
        if !self.visible(id) {
            return None;
        }
        if let Some(row) = self.texts.iter().find(|row| row.id == id) {
            return row.bounds();
        }
        let object = self
            .objects
            .iter()
```

**Locate this block:**

```rust
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        Err("Object not found")
    }
```

Replace it with:

```rust
            let model = &shift * &row.model;
            return self.place_point(id, model);
        }
        if let Some(row) = self.texts.iter().find(|r| r.id == id) {
            let model = &shift * &row.model;
            return self.place_text(id, model);
        }
        Err("Object not found")
    }
```

**Locate this block:**

```rust
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else {
            self.place_point(id, model)
        }
```

Replace it with:

```rust
            self.place_line(id, model)
        } else if self.paths.iter().any(|r| r.id == id) {
            self.place_path(id, model)
        } else if self.texts.iter().any(|r| r.id == id) {
            self.place_text(id, model)
        } else {
            self.place_point(id, model)
        }
```

**Locate this block:**

```rust
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
    }

    pub fn with_descendants(&self, ids: &[ObjectId]) -> Vec<ObjectId> {
```

Replace it with:

```rust
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
            .or_else(|| {
                self.texts
                    .iter()
                    .find(|r| r.id == id)
                    .map(|r| r.model.clone())
            })
    }

    pub fn with_descendants(&self, ids: &[ObjectId]) -> Vec<ObjectId> {
```

**Locate this block:**

```rust
        }
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
```

Replace it with:

```rust
        }
        for row in self
            .texts
            .iter()
            .filter(|row| self.visible(row.id))
        {
            if let Some(next) = row.bounds() {
                if let Some(bounds) = &mut bounds {
                    bounds.include(next.min);
                    bounds.include(next.max);
                } else {
                    bounds = Some(next);
                }
            }
        }
        bounds
    }

    pub fn bounds_of(&self, ids: &[ObjectId]) -> Option<crate::bounds::Bounds> {
```

### 2. `src/hierarchy.rs`

**Locate this existing block:**

```rust
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        );
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        )
        .chain(
            scene
                .texts()
                .iter()
                .map(|r| (r.id, r.prepared.source.name.as_str())),
        );
```

### 3. `src/document.rs`

**Locate this existing block:**

```rust
    }
    if !scene.points().is_empty() {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    if !scene.texts().is_empty() {
        return Err("This checkpoint cannot save scene text objects yet");
    }
    if !scene.points().is_empty() {
```

### 4. `src/memory.rs`

**Locate this existing block:**

```rust
    counts
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    counts
}

pub fn texts<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 4] {
    let mut sources = HashSet::new();
    let mut usage = [0; 4];
    for scene in scenes {
        usage[0] += scene.texts().len();
        usage[2] +=
            scene.texts().len() * std::mem::size_of::<crate::scene_label::Row>();
        for row in scene.texts() {
            if sources.insert(Rc::as_ptr(&row.prepared.source)) {
                let source = &row.prepared.source;
                usage[3] += std::mem::size_of::<crate::scene_label::Source>()
                    + source.guid.capacity()
                    + source.name.capacity()
                    + source.label.text.capacity();
            }
        }
    }
    usage[1] = sources.len();
    usage
}
```

### 5. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    .chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id))
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .collect();
```

Replace it with:

```rust
                    .chain(self.scene.lines().iter().map(|r| r.id))
                    .chain(self.scene.paths().iter().map(|r| r.id))
                    .chain(self.scene.points().iter().map(|r| r.id))
                    .chain(self.scene.texts().iter().map(|r| r.id))
                    .collect();
```

**Locate this block:**

```rust
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id)
            .ok_or("Object not found")?;
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.geometry().is_none() {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
```

Replace it with:

```rust
        }
        if !self.scene.contains(id) {
            return Err("Object not found");
        }
        let object = self
            .scene
            .objects()
            .iter()
            .find(|row| row.id == id);
        if self.scene.forest.readonly(id) {
            return Err("Object attributes follow their parent and are read only");
        }
        if object.is_some_and(|object| object.geometry().is_none()) {
            return Err("Reload editable sources before Move");
        }
        if offset.iter().all(|&value| value == 0.0) {
```

## <a id="checkpoint-62e-rectangles"></a>6. Select a label through its projected plate

### 1. `src/scene_label_screen.rs`

Create the file and type:

```rust
use crate::{
    rect_select::{Rectangle, screen},
    scene_label::Row,
    text::TextPlacement,
};

pub fn polygon(row: &Row, matrix: &[f64; 16], size: [f64; 2]) -> Option<Vec<[f64; 2]>> {
    let label = row.label(false, true).ok()?;
    let [width, height] = row.prepared.extent;
    if let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
    } = label.placement
    {
        let unit = world_height / label.font_size as f64;
        let vertical = label.font_size as f64 * 2. / 9.;
        let horizontal = height * 0.5 + vertical;
        return [
            [-horizontal, -vertical],
            [-horizontal, height + vertical],
            [width + horizontal, height + vertical],
            [width + horizontal, -vertical],
        ]
        .into_iter()
        .map(|[x, y]| {
            screen(
                matrix,
                std::array::from_fn(|i| world[i] + (right[i] * x - up[i] * y) * unit),
                size,
            )
        })
        .collect();
    }
    let (world, offset, scale, center, padding) = match label.placement {
        TextPlacement::Anchor { world, offset } => (
            world,
            offset,
            1.,
            false,
            [
                height * 0.5 + label.font_size as f64 * 2. / 9.,
                label.font_size as f64 * 2. / 9.,
            ],
        ),
        TextPlacement::Nameplate {
            world,
            padding,
            rounded,
        } => (
            world,
            [0.; 2],
            1.,
            true,
            [
                if rounded {
                    (padding[0] as f64).max(height * 0.5 + padding[1] as f64)
                } else {
                    padding[0] as f64
                },
                padding[1] as f64,
            ],
        ),
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } => {
            let w = (0..3)
                .map(|i| matrix[i * 4 + 3] * world[i])
                .sum::<f64>()
                + matrix[15];
            let projection = [matrix[1], matrix[5], matrix[9]]
                .iter()
                .map(|n| n * n)
                .sum::<f64>()
                .sqrt();
            let scale =
                world_height * projection * size[1] / (2. * w * label.font_size as f64);
            if !scale.is_finite() || scale <= 0. {
                return None;
            }
            (
                world,
                [0.; 2],
                scale,
                false,
                [
                    height * 0.5 + label.font_size as f64 * 2. / 9.,
                    label.font_size as f64 * 2. / 9.,
                ],
            )
        }
        _ => return None,
    };
    let [mut left, mut top] = screen(matrix, world, size)?;
    left += offset[0] as f64;
    top += offset[1] as f64;
    if center {
        left -= width * scale * 0.5;
        top -= height * scale * 0.5;
    }
    let bounds = [
        left - padding[0] * scale,
        top - padding[1] * scale,
        left + (width + padding[0]) * scale,
        top + (height + padding[1]) * scale,
    ];
    Some(vec![
        [bounds[0], bounds[1]],
        [bounds[0], bounds[3]],
        [bounds[2], bounds[3]],
        [bounds[2], bounds[1]],
    ])
}
fn inside(p: [f64; 2], points: &[[f64; 2]]) -> bool {
    let mut sign = 0.;
    for (a, b) in points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .take(points.len())
    {
        let cross = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
        if cross == 0. {
            continue;
        }
        if sign == 0. {
            sign = cross.signum();
        } else if sign != cross.signum() {
            return false;
        }
    }
    sign != 0.
}
pub fn matches(
    row: &Row,
    matrix: &[f64; 16],
    size: [f64; 2],
    rectangle: Rectangle,
) -> bool {
    let Some(points) = polygon(row, matrix, size) else {
        return false;
    };
    if !rectangle.crossing {
        return points.iter().all(|p| rectangle.contains(*p));
    }
    points.iter().any(|p| rectangle.contains(*p))
        || points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .take(points.len())
            .any(|(a, b)| rectangle.crosses(*a, *b))
        || inside(
            [rectangle.bounds[0], rectangle.bounds[1]],
            &points,
        )
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod text_atlas_tests;

pub mod scene_label;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod text_atlas_tests;

pub mod scene_label;

pub mod scene_label_screen;
```

### 3. `src/rect_select.rs`

**Locate this existing block:**

```rust
    }
    found.sort_unstable_by_key(|id| id.value());
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    for row in scene
        .texts()
        .iter()
        .filter(|row| eligible(row.id))
    {
        if crate::scene_label_screen::matches(row, matrix, size, rectangle) {
            found.push(row.id);
        }
    }
    found.sort_unstable_by_key(|id| id.value());
```

## <a id="checkpoint-62f-gpu-support"></a>7. Supply the text buffer and target adapters

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

### 1. `src/text_gpu_support.rs`

Create the file and type:

```rust
pub struct GpuCtx {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
impl GpuCtx {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        Self {
            device: device.clone(),
            queue: queue.clone(),
        }
    }
}
pub type Pipeline = wgpu::RenderPipeline;
#[derive(Clone, Copy)]
pub struct Target {
    pub format: wgpu::TextureFormat,
    pub samples: u32,
}
impl Target {
    pub const ID: Self = Self {
        format: wgpu::TextureFormat::Rg32Uint,
        samples: 1,
    };
}
pub const VERTS: wgpu::BufferUsages =
    wgpu::BufferUsages::VERTEX.union(wgpu::BufferUsages::COPY_DST);
pub struct GrowBuf {
    pub buf: wgpu::Buffer,
    stride: u64,
    len: u32,
    label: &'static str,
    usage: wgpu::BufferUsages,
}
impl GrowBuf {
    pub fn new(
        ctx: &GpuCtx,
        label: &'static str,
        stride: u64,
        usage: wgpu::BufferUsages,
    ) -> Self {
        Self {
            buf: ctx
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size: stride,
                    usage,
                    mapped_at_creation: false,
                }),
            stride,
            len: 0,
            label,
            usage,
        }
    }
    pub fn append<const N: usize>(&mut self, ctx: &GpuCtx, vertices: &[[f32; N]]) {
        assert_eq!(self.stride, N as u64 * 4);
        if vertices.is_empty() {
            return;
        }
        let count = u32::try_from(vertices.len()).expect("Bounded text vertices");
        let size = (self.len as u64 + count as u64) * self.stride;
        if size > self.buf.size() {
            assert_eq!(
                self.len, 0,
                "Text upload starts at an empty buffer"
            );
            self.buf = ctx
                .device
                .create_buffer(&wgpu::BufferDescriptor {
                    label: Some(self.label),
                    size: size.next_power_of_two(),
                    usage: self.usage,
                    mapped_at_creation: false,
                });
        }
        let bytes: Vec<u8> = vertices
            .iter()
            .flatten()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        ctx.queue.write_buffer(
            &self.buf,
            self.len as u64 * self.stride,
            &bytes,
        );
        self.len += count;
    }
    pub fn reset(&mut self) {
        self.len = 0;
    }
    pub fn release(&mut self, ctx: &GpuCtx) {
        *self = Self::new(ctx, self.label, self.stride, self.usage);
    }
    pub fn len(&self) -> u32 {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
pub fn texture_entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    sample_type: wgpu::TextureSampleType,
    multisampled: bool,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Texture {
            sample_type,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled,
        },
        count: None,
    }
}
pub fn resource_group<'a, const N: usize>(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    label: &str,
    entries: [(u32, wgpu::BindingResource<'a>); N],
) -> wgpu::BindGroup {
    ctx.device
        .create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries: &entries
                .into_iter()
                .map(|(binding, resource)| wgpu::BindGroupEntry {
                    binding,
                    resource,
                })
                .collect::<Vec<_>>(),
        })
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod scene_label;

pub mod scene_label_screen;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_label;

pub mod scene_label_screen;

pub mod text_gpu_support;
```

## <a id="checkpoint-62g-plane-shader"></a>8. Keep plane clipping correct in a small pick window

### 1. `src/text_plane.wgsl`

Create the file and type:

```wgsl
struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) @interpolate(flat) clip: vec4<f32>,
    @location(3) @interpolate(flat) object: u32,
    @location(4) @interpolate(flat) selected: f32,
    @location(5) original: vec4<f32>,
}

@group(0) @binding(0) var coverage_texture: texture_2d<f32>;
@group(0) @binding(1) var coverage_sampler: sampler;

@group(1) @binding(0) var<uniform> pick: mat4x4<f32>;

fn plane_vertex(position: vec4<f32>, uv: vec2<f32>, color: vec4<f32>, clip: vec4<f32>,
                object: u32, selected: f32) -> Vertex {
    var out: Vertex;
    out.original = position;
    out.position = position; out.uv = uv; out.color = color; out.clip = clip;
    out.object = object; out.selected = selected;
    return out;
}

@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
           @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
           @location(4) object: u32, @location(5) selected: f32) -> Vertex {
    return plane_vertex(position, uv, color, clip, object, selected);
}

@vertex
fn vs_id(@location(0) position: vec4<f32>, @location(1) uv: vec2<f32>,
         @location(2) color: vec4<f32>, @location(3) clip: vec4<f32>,
         @location(4) object: u32, @location(5) selected: f32) -> Vertex {
    var out = plane_vertex(position, uv, color, clip, object, selected);
    out.position = pick * position;
    return out;
}

fn plate_distance(uv: vec2<f32>) -> f32 {
    let half_size = vec2<f32>(textureDimensions(coverage_texture)) * 0.5;
    let radius = min(half_size.x, half_size.y);
    let q = abs((uv * 2.0 - 1.0) * half_size) - half_size + radius;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: Vertex) -> @location(0) vec4<f32> {
    let original = in.original.xy / in.original.w;
    if (original.x < in.clip.x || original.y <= in.clip.y || original.x >= in.clip.z || original.y > in.clip.w) {
        discard;
    }

    let coverage = textureSample(coverage_texture, coverage_sampler, in.uv).r;
    let distance = plate_distance(in.uv);
    let plate_coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    let background = vec3<f32>(in.selected, in.selected, 0.0);
    return vec4<f32>(mix(background, in.color.rgb, coverage), in.color.a * plate_coverage);
}

@fragment
fn fs_id(in: Vertex) -> @location(0) vec2<u32> {
    let original = in.original.xy / in.original.w;
    if (original.x < in.clip.x || original.y <= in.clip.y || original.x >= in.clip.z || original.y > in.clip.w) {
        discard;
    }

    if (plate_distance(in.uv) > 0.0) {
        discard;
    }

    return vec2<u32>(in.object, 0u);
}
```

## <a id="checkpoint-62h-plate-shader"></a>9. Draw the current rounded nameplate and its ID

### 1. `src/text_plate.wgsl`

Create the file and type:

```wgsl
struct PlateVertex {
    @builtin(position) position: vec4<f32>,
    @location(0) local: vec2<f32>,
    @location(1) half_size: vec2<f32>,
    @location(2) radius: f32,
    @location(3) @interpolate(flat) object: u32,
    @location(4) @interpolate(flat) selected: f32,
}

@group(0) @binding(0) var<uniform> pick: mat4x4<f32>;

fn plate_vertex(position: vec4<f32>, local: vec2<f32>, half_size: vec2<f32>, radius: f32,
                object: u32, selected: f32) -> PlateVertex {
    var out: PlateVertex;
    out.position = position;
    out.local = local;
    out.half_size = half_size;
    out.radius = radius;
    out.object = object;
    out.selected = selected;
    return out;
}

@vertex
fn vs_main(@location(0) position: vec3<f32>, @location(1) local: vec2<f32>,
           @location(2) half_size: vec2<f32>, @location(3) radius: f32, @location(4) object: u32,
           @location(5) selected: f32) -> PlateVertex {
    return plate_vertex(vec4<f32>(position, 1.0), local, half_size, radius, object, selected);
}

@vertex
fn vs_id(@location(0) position: vec3<f32>, @location(1) local: vec2<f32>,
         @location(2) half_size: vec2<f32>, @location(3) radius: f32, @location(4) object: u32,
         @location(5) selected: f32) -> PlateVertex {
    return plate_vertex(pick * vec4<f32>(position, 1.0), local, half_size, radius, object, selected);
}

fn plate_distance(in: PlateVertex) -> f32 {
    let radius = min(in.radius, min(in.half_size.x, in.half_size.y));
    let q = abs(in.local) - in.half_size + vec2<f32>(radius);
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: PlateVertex) -> @location(0) vec4<f32> {
    let distance = plate_distance(in);
    let coverage = clamp(0.5 - distance / max(fwidth(distance), 0.001), 0.0, 1.0);
    return vec4<f32>(in.selected, in.selected, 0.0, coverage);
}

@fragment
fn fs_id(in: PlateVertex) -> @location(0) vec2<u32> {
    if (plate_distance(in) > 0.0) {
        discard;
    }

    return vec2<u32>(in.object, 0u);
}
```

## <a id="checkpoint-62i-coverage"></a>10. Keep CSS pixels separate from framebuffer pixels

### 1. `src/scene_text_gpu.rs`

Create the file and type:

```rust
use crate::text::{TextDocument, TextLabel, TextPlacement, TextRun};
use crate::text_gpu_support::GpuCtx;
use crate::text_gpu_support::Target;
use glyphon::{
    Cache, Color, ColorMode, Resolution, SwashCache, TextArea, TextAtlas, TextBounds,
    TextRenderer, Viewport,
};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
pub struct TextFrame {
    pub mvp: [f32; 16],
    pub origin: [f64; 3],
    pub framebuffer: [u32; 2],
    pub logical: [f64; 2],
    pub ortho_half_height: f32,
    pub clip: [[f64; 4]; 4],
}

impl TextFrame {
    fn canvas(&self) -> TextFrame {
        TextFrame {
            mvp: [0.0; 16],
            origin: [0.0; 3],
            ortho_half_height: 0.0,
            clip: [[0.0; 4]; 4],
            ..self.clone()
        }
    }

    pub fn cut(&self, p: [f64; 3]) -> bool {
        self.clip
            .iter()
            .any(|k| k[0] * p[0] + k[1] * p[1] + k[2] * p[2] + k[3] < 0.0)
    }

    pub fn scale(&self) -> anyhow::Result<f32> {
        anyhow::ensure!(
            self.framebuffer[0] > 0 && self.framebuffer[1] > 0,
            "empty text framebuffer"
        );
        anyhow::ensure!(
            self.logical[0].is_finite()
                && self.logical[1].is_finite()
                && self.logical[0] > 0.0
                && self.logical[1] > 0.0,
            "empty text CSS box"
        );
        let x = self.framebuffer[0] as f64 / self.logical[0];
        let y = self.framebuffer[1] as f64 / self.logical[1];
        let tolerance = 1.0 / self.logical[0] + 1.0 / self.logical[1];
        anyhow::ensure!(
            (x - y).abs() <= tolerance,
            "text canvas is stretched non-uniformly"
        );
        Ok(y as f32)
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod scene_label_screen;

pub mod text_gpu_support;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_label_screen;

pub mod text_gpu_support;

pub mod scene_text_gpu;
```

## <a id="checkpoint-62ia-project-an-original-label-into-framebuffer-coordinates"></a>11. Project an label into framebuffer coordinates

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

struct PlacedText {
    left: f32,
    top: f32,
    scale: f32,
    depth: Option<f32>,
}

fn place(label: &TextLabel, frame: &TextFrame, scale: f32) -> Option<PlacedText> {
    let (world, offset, world_height) = match label.placement {
        TextPlacement::WorldPlane { .. } => return None,
        TextPlacement::Screen { left, top } => {
            return Some(PlacedText {
                left: left * scale,
                top: top * scale,
                scale,
                depth: None,
            });
        }
        TextPlacement::Anchor { world, offset } => (world, offset, None),
        TextPlacement::Nameplate { world, .. } => (world, [0.0; 2], None),
        TextPlacement::WorldBillboard {
            world,
            world_height,
        } => (world, [0.0; 2], Some(world_height)),
    };

    if frame.cut(world) {
        return None;
    }

    let p = [
        (world[0] - frame.origin[0]) as f32,
        (world[1] - frame.origin[1]) as f32,
        (world[2] - frame.origin[2]) as f32,
    ];
    let m = &frame.mvp;
    let w = m[3] * p[0] + m[7] * p[1] + m[11] * p[2] + m[15];

    if !w.is_finite() || w <= 0.0 {
        return None;
    }

    let z = (m[2] * p[0] + m[6] * p[1] + m[10] * p[2] + m[14]) / w;

    if !(0.0..=1.0).contains(&z) {
        return None;
    }

    let x = (m[0] * p[0] + m[4] * p[1] + m[8] * p[2] + m[12]) / w;
    let y = (m[1] * p[0] + m[5] * p[1] + m[9] * p[2] + m[13]) / w;
    let raster_scale = match world_height {
        Some(height) => {
            let projection = (m[1] * m[1] + m[5] * m[5] + m[9] * m[9]).sqrt();
            height as f32 * projection * frame.framebuffer[1] as f32
                / (2.0 * w * label.font_size)
        }
        None => scale,
    };

    if !raster_scale.is_finite() || raster_scale <= 0.0 {
        return None;
    }

    Some(PlacedText {
        left: (x + 1.0) * 0.5 * frame.framebuffer[0] as f32 + offset[0] * scale,
        top: (1.0 - y) * 0.5 * frame.framebuffer[1] as f32 + offset[1] * scale,
        scale: raster_scale,
        depth: match label.placement {
            TextPlacement::Nameplate { .. } => None,
            _ => Some(z),
        },
    })
}

fn clip_bounds(label: &TextLabel, frame: &TextFrame, scale: f32) -> TextBounds {
    match label.clip {
        Some(c) => TextBounds {
            left: (c[0] * scale).floor() as i32,
            top: (c[1] * scale).floor() as i32,
            right: (c[2] * scale).ceil() as i32,
            bottom: (c[3] * scale).ceil() as i32,
        },
        None => TextBounds {
            left: 0,
            top: 0,
            right: frame.framebuffer[0] as i32,
            bottom: frame.framebuffer[1] as i32,
        },
    }
}
```

## <a id="checkpoint-62ib-give-nameplate-vertices-one-gpu-owner"></a>12. Give nameplate vertices one GPU owner

### 1. `src/text_plate.rs`

Create the file and type:

```rust
use crate::text_gpu_support::{GpuCtx, GrowBuf, VERTS};
use crate::text_gpu_support::{Pipeline, Target};

pub(super) struct Rectangle {
    pub(super) bounds: [f32; 4],
    pub(super) clip: [f32; 4],
    pub(super) rounded: bool,
    pub(super) depth: Option<f32>,
    pub(super) object: Option<crate::text::TextObject>,
}
pub(super) struct Plates {
    vertices: GrowBuf,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    physical_vertices: u32,
}
```

### 2. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
        },
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        },
    }
}

#[path = "text_plate.rs"]
mod plate;
```

## <a id="checkpoint-62ic-build-nameplate-color-and-object-id-pipelines"></a>13. Build nameplate color and object-ID pipelines

### 1. `src/text_plate.rs`

**Locate this existing block:**

```rust
    physical_vertices: u32,
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    physical_vertices: u32,
}

fn pipeline(ctx: &GpuCtx, target: Target, ids: bool) -> Pipeline {
    let device = ctx.device.clone();
    let pick = crate::gpu_layout::uniform(&ctx.device, 80);
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text plate shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_plate.wgsl").into()),
        });
        let id_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("text plate ids"),
                bind_group_layouts: &[Some(&pick)],
                immediate_size: 0,
            });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text plates"),
            layout: if ids { Some(&id_layout) } else { None },
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some(if ids { "vs_id" } else { "vs_main" }),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 40,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x2, 3 => Float32, 4 => Uint32, 5 => Float32],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some(if ids { "fs_id" } else { "fs_main" }),
                targets: &[Some(wgpu::ColorTargetState {
                    format: target.format,
                    blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) },
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: Some(false),
                depth_compare: Some(wgpu::CompareFunction::LessEqual),
                stencil: Default::default(),
                bias: Default::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: target.samples,
                ..Default::default()
            },
            multiview_mask: None,
            cache: None,
        })
    }
}
```

## <a id="checkpoint-62id-own-and-release-nameplate-draw-resources"></a>14. Own and release nameplate draw resources

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU CommandEncoder Records GPU Commands</text>
  <rect x="54" y="50" width="264" height="102" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="76" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">wgpu::CommandEncoder</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="100" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#2563eb">1. begin_render_pass (3D Scene)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="122" text-anchor="middle" font-family="monospace" font-size="9.5" fill="#16a34a">2. begin_render_pass (Overlay/UI)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="142" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">3. encoder.finish()</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Pass 1 (LoadOp::Clear) -&gt; Pass 2 (LoadOp::Load)</text>
  <rect x="416" y="52" width="128" height="96" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="480" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 1: Clear</text>
  <polygon points="452,132 480,88 508,132" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <line x1="544" y1="100" x2="572" y2="100" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="576" y="52" width="128" height="96" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="640" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Pass 2: Load</text>
  <polygon points="612,126 640,86 668,126" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <rect x="590" y="130" width="100" height="12" fill="#fef08a" stroke="#111111" stroke-width="1"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.submit([cmd_buf]) -&gt; GPU Execution</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="884,132 934,58 984,132" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <rect x="822" y="136" width="224" height="18" rx="3" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="149" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Composited Frame Ready</text>
</svg>
</div>

### 1. `src/text_plate.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
}

impl Plates {
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        Self {
            vertices: GrowBuf::new(ctx, "text.plates", 40, VERTS),
            pipeline: pipeline(ctx, target, false),
            id_pipeline: pipeline(ctx, Target::ID, true),
            physical_vertices: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.pipeline = pipeline(ctx, target, false);
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>, overlay: bool) -> u32 {
        let range = if overlay {
            self.physical_vertices..self.vertices.len()
        } else {
            0..self.physical_vertices
        };

        if range.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(range, 0..1);
        1
    }

    pub(super) fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        if self.vertices.is_empty() {
            return 0;
        }

        pass.set_pipeline(&self.id_pipeline);
        pass.set_bind_group(0, pick_transform, &[]);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));
        pass.draw(0..self.vertices.len(), 0..1);
        1
    }

    pub(super) fn reset(&mut self) {
        self.vertices.reset();
        self.physical_vertices = 0;
    }

    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.vertices.release(ctx);
    }

    pub(super) fn allocated_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }
}
```

## <a id="checkpoint-62ie-upload-nameplate-rectangles-in-one-physical-coordinate-system"></a>15. Upload nameplate rectangles in one physical coordinate system

### 1. `src/text_plate.rs`

**Locate this existing block:**

```rust
        self.vertices.buf.size()
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.vertices.buf.size()
    }
}

impl Plates {
    pub(super) fn prepare(
        &mut self,
        ctx: &GpuCtx,
        rectangles: &[Rectangle],
        size: [u32; 2],
    ) {
        self.vertices.reset();
        let mut vertices = Vec::with_capacity(rectangles.len() * 6);
        self.physical_vertices = 0;

        for overlay in [false, true] {
            for rectangle in rectangles {
                if rectangle.depth.is_none() != overlay {
                    continue;
                }

                let [left, top, right, bottom] = rectangle.bounds;
                let half = [(right - left) * 0.5, (bottom - top) * 0.5];
                let center = [(left + right) * 0.5, (top + bottom) * 0.5];
                let radius = if rectangle.rounded {
                    half[0].min(half[1]).max(0.0)
                } else {
                    0.0
                };
                let left = left.max(rectangle.clip[0]);
                let top = top.max(rectangle.clip[1]);
                let right = right.min(rectangle.clip[2]);
                let bottom = bottom.min(rectangle.clip[3]);

                if right <= left || bottom <= top {
                    continue;
                }

                for [x, y] in [
                    [left, top],
                    [left, bottom],
                    [right, bottom],
                    [left, top],
                    [right, bottom],
                    [right, top],
                ] {
                    vertices.push([
                        2.0 * x / size[0] as f32 - 1.0,
                        1.0 - 2.0 * y / size[1] as f32,
                        rectangle.depth.unwrap_or(0.0),
                        x - center[0],
                        y - center[1],
                        half[0],
                        half[1],
                        radius,
                        f32::from_bits(
                            rectangle
                                .object
                                .map_or(0, |object| object.row.wrapping_add(1)),
                        ),
                        if rectangle
                            .object
                            .is_some_and(|object| object.selected)
                        {
                            1.0
                        } else {
                            0.0
                        },
                    ]);
                }
            }

            if !overlay {
                self.physical_vertices = vertices.len() as u32;
            }
        }

        self.vertices.append(ctx, &vertices);
    }
}
```

## <a id="checkpoint-62if-place-the-plate-around-the-shaped-label"></a>16. Place the plate around the shaped label

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
#[path = "text_plate.rs"]
mod plate;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "text_plate.rs"]
mod plate;

#[derive(Clone, Debug, Default, Serialize)]
pub struct TextStats {
    pub preparations: u64,
    pub skipped_preparations: u64,
    pub shape_count: u64,
    pub shaping_ms: f64,
    pub requested_glyphs: usize,
    pub distinct_raster_keys: usize,
    pub new_raster_keys: usize,
    pub raster_images: usize,
    pub new_raster_images: usize,
    pub raster_image_capacity_bytes: usize,
    pub atlas_resets: u64,
    pub preparation_ms: f64,
    pub active_instance_bytes: usize,
    pub missing_glyphs: usize,
    pub nameplate_capacity_bytes: u64,
    pub world_plane_buffer_bytes: u64,
    pub world_plane_texture_bytes: u64,
    pub world_plane_rasterizations: u64,
}

fn text_rectangle(
    run: &TextRun,
    placed: &mut PlacedText,
    frame: &TextFrame,
    scale: f32,
) -> Option<plate::Rectangle> {
    let (mut padding, rounded, centered) = match run.label.placement {
        TextPlacement::Nameplate {
            padding, rounded, ..
        } => (padding, rounded, true),
        _ if run.label.object.is_some() => (
            [0.0, run.label.font_size * 2.0 / 9.0],
            true,
            false,
        ),
        _ => return None,
    };

    if run.label.text.is_empty() {
        return None;
    }

    let mut width = 0.0f32;
    let mut top = f32::INFINITY;
    let mut bottom = f32::NEG_INFINITY;

    for line in run.buffer.layout_runs() {
        width = width.max(line.line_w);
        top = top.min(line.line_top);
        bottom = bottom.max(line.line_top + line.line_height);
    }

    if !top.is_finite() || !bottom.is_finite() {
        return None;
    }

    if rounded {
        padding[0] = padding[0].max((bottom - top) * 0.5 + padding[1]);
    }

    let raster = placed.scale;

    if centered {
        placed.left -= width * raster * 0.5;
        placed.top -= (top + bottom) * raster * 0.5;
    }

    let bounds = clip_bounds(&run.label, frame, scale);
    let bounds = [
        bounds.left as f32,
        bounds.top as f32,
        bounds.right as f32,
        bounds.bottom as f32,
    ];
    Some(plate::Rectangle {
        bounds: [
            placed.left - padding[0] * raster,
            placed.top + (top - padding[1]) * raster,
            placed.left + (width + padding[0]) * raster,
            placed.top + (bottom + padding[1]) * raster,
        ],
        clip: bounds,
        rounded,
        depth: placed.depth,
        object: run.label.object,
    })
}
```

## <a id="checkpoint-62ig-give-world-text-a-bounded-coverage-cache"></a>17. Give world text a bounded coverage cache

### 1. `src/text_plane.rs`

Create the file and type:

```rust
use super::TextFrame;
use crate::text::{TextDocument, TextLabel, TextPlacement, TextRun};
use crate::text_gpu_support::resource_group;
use crate::text_gpu_support::texture_entry;
use crate::text_gpu_support::{GpuCtx, GrowBuf, VERTS};
use crate::text_gpu_support::{Pipeline, Target};
use glyphon::{FontSystem, SwashCache, SwashContent};
use wgpu::{ShaderStages, TextureSampleType};

const TEXTURE_BUDGET: u64 = 32 * 1024 * 1024;
struct CachedPlane {
    label: TextLabel,
    font_revision: u64,
    em_pixels: u32,
    size: [u32; 2],
    extent: [f32; 4],
    texture: wgpu::Texture,
    bind: wgpu::BindGroup,
}
impl Drop for CachedPlane {
    fn drop(&mut self) {
        self.texture.destroy();
    }
}
pub(super) struct Planes {
    cached: Vec<CachedPlane>,
    draws: Vec<(usize, u32)>,
    vertices: GrowBuf,
    layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    pipeline: Pipeline,
    id_pipeline: Pipeline,
    target: Target,
    pub(super) rasterizations: u64,
}

fn same_raster(
    cached: &CachedPlane,
    label: &TextLabel,
    revision: u64,
    em_pixels: u32,
) -> bool {
    cached.font_revision == revision
        && cached.label.text == label.text
        && cached.label.font_size == label.font_size
        && cached.label.line_height == label.line_height
        && cached.em_pixels >= em_pixels
}

fn project(world: [f64; 3], frame: &TextFrame) -> [f32; 4] {
    let point = [
        (world[0] - frame.origin[0]) as f32,
        (world[1] - frame.origin[1]) as f32,
        (world[2] - frame.origin[2]) as f32,
        1.0,
    ];
    let mut clip = [0.0; 4];

    for (row, out) in clip.iter_mut().enumerate() {
        for (column, value) in point.iter().enumerate() {
            *out += frame.mvp[column * 4 + row] * value;
        }
    }

    clip
}
```

### 2. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
        object: run.label.object,
    })
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        object: run.label.object,
    })
}

#[path = "text_plane.rs"]
mod plane;
```

## <a id="checkpoint-62ih-choose-bounded-glyph-raster-resolution"></a>18. Choose bounded glyph raster resolution

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
    clip
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    clip
}

fn raster_em(label: &TextLabel, frame: &TextFrame) -> u32 {
    let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
        ..
    } = label.placement
    else {
        return 32;
    };
    let a = project(world, frame);

    if a[3] <= 0.0 {
        return 32;
    }

    let mut projected_em = 0.0f32;

    for direction in [right, up] {
        let mut end = world;

        for axis in 0..3 {
            end[axis] += direction[axis] * world_height;
        }

        let b = project(end, frame);

        if b[3] <= 0.0 {
            continue;
        }

        let x = (a[0] / a[3] - b[0] / b[3]) * frame.framebuffer[0] as f32 * 0.5;
        let y = (a[1] / a[3] - b[1] / b[3]) * frame.framebuffer[1] as f32 * 0.5;
        projected_em = projected_em.max(x.hypot(y));
    }

    let needed = (projected_em * 2.0).clamp(32.0, 256.0);
    let mut bucket = 32;

    while (bucket as f32) < needed {
        bucket *= 2;
    }

    bucket
}

fn fitting_em(run: &TextRun) -> u32 {
    let mut size = 0.0f32;

    for line in run.buffer.layout_runs() {
        size = size
            .max(line.line_w)
            .max(line.line_top + line.line_height);
    }

    let ems = size / run.label.font_size + 3.0;
    let mut em = 256;

    while em > 32 && ems * em as f32 > 4000.0 {
        em /= 2;
    }

    em
}

fn coverage(content: &SwashContent, data: &[u8], source: usize) -> u8 {
    match content {
        SwashContent::Mask => data[source],
        SwashContent::Color => data[source * 4 + 3],
        SwashContent::SubpixelMask => {
            ((u16::from(data[source * 4])
                + u16::from(data[source * 4 + 1])
                + u16::from(data[source * 4 + 2]))
                / 3) as u8
        }
    }
}
```

## <a id="checkpoint-62ii-rasterize-the-shaped-run-into-owned-coverage-bytes"></a>19. Rasterize the shaped run into owned coverage bytes

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
        }
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        }
    }
}

fn rasterize(
    run: &TextRun,
    fonts: &mut FontSystem,
    raster: &mut SwashCache,
    em_pixels: u32,
) -> anyhow::Result<(Vec<u8>, [u32; 2], [f32; 4])> {
    let scale = em_pixels as f32 / run.label.font_size;
    let mut glyphs = Vec::new();
    let mut bounds = [0i32; 4];

    for line in run.buffer.layout_runs() {
        bounds[2] = bounds[2].max((line.line_w * scale).ceil() as i32);
        bounds[3] =
            bounds[3].max(((line.line_top + line.line_height) * scale).ceil() as i32);
        anyhow::ensure!(
            bounds[2] <= 4092 && bounds[3] <= 4092,
            "world text layout exceeds 4096px extent"
        );

        for glyph in line.glyphs {
            let physical = glyph.physical((0.0, 0.0), scale);
            let Some(image) = raster.get_image(fonts, physical.cache_key) else {
                continue;
            };
            let x = physical.x + image.placement.left;
            let y =
                (line.line_y * scale).round() as i32 + physical.y - image.placement.top;
            bounds[0] = bounds[0].min(x);
            bounds[1] = bounds[1].min(y);
            bounds[2] = bounds[2].max(x + image.placement.width as i32);
            bounds[3] = bounds[3].max(y + image.placement.height as i32);
            glyphs.push((physical.cache_key, x, y));
        }
    }

    let vertical_padding = (run.label.font_size * (2.0 / 9.0) * scale).ceil() as i32;
    let horizontal_padding = (bounds[3] - bounds[1] + 2 * vertical_padding + 1) / 2;
    bounds[0] -= horizontal_padding;
    bounds[1] -= vertical_padding;
    bounds[2] += horizontal_padding;
    bounds[3] += vertical_padding;
    let size = [
        (bounds[2] - bounds[0]) as u32,
        (bounds[3] - bounds[1]) as u32,
    ];
    anyhow::ensure!(
        size[0] <= 4096 && size[1] <= 4096,
        "world text texture exceeds 4096px extent"
    );
    let mut pixels = vec![0u8; (size[0] * size[1]) as usize];

    for (key, x, y) in glyphs {
        let Some(image) = raster.get_image(fonts, key) else {
            continue;
        };

        for row in 0..image.placement.height {
            for column in 0..image.placement.width {
                let source = (row * image.placement.width + column) as usize;
                let coverage = coverage(&image.content, &image.data, source);
                let target = ((y - bounds[1] + row as i32) as u32 * size[0]
                    + (x - bounds[0] + column as i32) as u32)
                    as usize;
                let previous = u16::from(pixels[target]);
                pixels[target] =
                    (previous + u16::from(coverage) * (255 - previous) / 255) as u8;
            }
        }
    }

    Ok((
        pixels,
        size,
        [
            bounds[0] as f32 / scale,
            bounds[1] as f32 / scale,
            bounds[2] as f32 / scale,
            bounds[3] as f32 / scale,
        ],
    ))
}
```

## <a id="checkpoint-62ij-upload-and-bind-one-world-text-coverage-texture"></a>20. Upload and bind one world-text coverage texture

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Rasterized Bytes (R8Unorm / Rgba8Unorm)</text>
  <rect x="64" y="50" width="244" height="102" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <rect x="80" y="64" width="42" height="42" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="101" y="91" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">A</text>
  <rect x="130" y="64" width="42" height="42" fill="#fef08a" stroke="#dc2626" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="151" y="91" text-anchor="middle" font-family="monospace" font-size="16" font-weight="700" fill="#111111">B</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">CPU Shelf Bitmap &amp;[u8]</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">queue.write_texture(TexelCopyTextureInfo, Extent3d)</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dcfce7" stroke="#16a34a" stroke-width="1.75"/>
  <line x1="448" y1="86" x2="672" y2="86" stroke="#16a34a" stroke-width="3" marker-end="url(#arrG)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="74" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">queue.write_texture(...)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="114" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">TEXTURE_BINDING | COPY_DST</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">origin: (x, y, 0), size: (w, h, 1)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Texture2D + Sampler in @fragment</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="76" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">textureSample(t_atlas, s, uv).r</text>
  <rect x="852" y="96" width="164" height="44" rx="4" fill="#fef08a" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="123" text-anchor="middle" font-family="monospace" font-size="13" font-weight="700" fill="#111111"> Crisp GPU Text </text>
</svg>
</div>

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
    ))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ))
}

fn upload_plane(
    ctx: &GpuCtx,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    label: &TextLabel,
    font_revision: u64,
    em_pixels: u32,
    data: (Vec<u8>, [u32; 2], [f32; 4]),
) -> CachedPlane {
    let (pixels, size, extent) = data;
    let texture = ctx
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("world text coverage"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
    ctx.queue.write_texture(
        texture.as_image_copy(),
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(size[0]),
            rows_per_image: Some(size[1]),
        },
        texture.size(),
    );
    let view = texture.create_view(&Default::default());
    let bind = resource_group(
        ctx,
        &layout,
        "world text coverage",
        [
            (0, wgpu::BindingResource::TextureView(&view)),
            (1, wgpu::BindingResource::Sampler(&sampler)),
        ],
    );
    CachedPlane {
        label: label.clone(),
        font_revision: font_revision,
        em_pixels,
        size,
        extent,
        texture,
        bind,
    }
}
```

## <a id="checkpoint-62ik-build-the-world-plane-text-quad"></a>21. Build the world-plane text quad

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
        bind,
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        bind,
    }
}

fn append_quad(
    vertices: &mut Vec<[f32; 16]>,
    label: &TextLabel,
    extent: [f32; 4],
    frame: &TextFrame,
    srgb: bool,
) {
    let TextPlacement::WorldPlane {
        world,
        right,
        up,
        world_height,
    } = label.placement
    else {
        return;
    };
    let unit = world_height / f64::from(label.font_size);
    let mut color = [0.0; 4];

    for (index, component) in color.iter_mut().enumerate() {
        let value = f32::from(label.ink_color()[index]) / 255.0;
        *component = if srgb && index < 3 {
            if value <= 0.04045 {
                value / 12.92
            } else {
                ((value + 0.055) / 1.055).powf(2.4)
            }
        } else {
            value
        };
    }

    let scale = frame.framebuffer[0] as f32 / frame.logical[0] as f32;
    let mut bounds = [
        0.0,
        0.0,
        frame.framebuffer[0] as f32,
        frame.framebuffer[1] as f32,
    ];

    if let Some(clip) = label.clip {
        for (index, value) in bounds.iter_mut().enumerate() {
            *value = clip[index] * scale;
        }
    }

    bounds = [
        2.0 * bounds[0] / frame.framebuffer[0] as f32 - 1.0,
        1.0 - 2.0 * bounds[3] / frame.framebuffer[1] as f32,
        2.0 * bounds[2] / frame.framebuffer[0] as f32 - 1.0,
        1.0 - 2.0 * bounds[1] / frame.framebuffer[1] as f32,
    ];
    for [u, v] in [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [1.0, 0.0],
    ] {
        let x = f64::from(extent[0] + (extent[2] - extent[0]) * u) * unit;
        let y = f64::from(extent[1] + (extent[3] - extent[1]) * v) * unit;
        let mut point = world;

        for axis in 0..3 {
            point[axis] += right[axis] * x - up[axis] * y;
        }

        let clip = project(point, frame);
        vertices.push([
            clip[0],
            clip[1],
            clip[2],
            clip[3],
            u,
            v,
            color[0],
            color[1],
            color[2],
            color[3],
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],
            f32::from_bits(
                label
                    .object
                    .map_or(0, |object| object.row.wrapping_add(1)),
            ),
            if label
                .object
                .is_some_and(|object| object.selected)
            {
                1.0
            } else {
                0.0
            },
        ]);
    }
}
```

## <a id="checkpoint-62il-build-world-text-color-and-id-pipelines"></a>22. Build world-text color and ID pipelines

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
        ]);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        ]);
    }
}

fn pipeline(
    ctx: &GpuCtx,
    target: Target,
    layout: &wgpu::BindGroupLayout,
    ids: bool,
) -> Pipeline {
    let device = ctx.device.clone();
    let layout = layout.clone();
    let pick = crate::gpu_layout::uniform(&ctx.device, 80);
    {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("world text shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("text_plane.wgsl").into()),
        });
        let colour_groups = [Some(&layout)];
        let id_groups = [Some(&layout), Some(&pick)];
        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("world text"),
                bind_group_layouts: if ids {
                    &id_groups
                } else {
                    &colour_groups
                },
                immediate_size: 0,
            });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("world text"), layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState { module: &shader, entry_point: Some(if ids { "vs_id" } else { "vs_main" }), buffers: &[wgpu::VertexBufferLayout { array_stride: 64, step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x2, 2 => Float32x4, 3 => Float32x4, 4 => Uint32, 5 => Float32] }], compilation_options: Default::default() },
            fragment: Some(wgpu::FragmentState { module: &shader, entry_point: Some(if ids { "fs_id" } else { "fs_main" }), targets: &[Some(wgpu::ColorTargetState { format: target.format, blend: if ids { None } else { Some(wgpu::BlendState::ALPHA_BLENDING) }, write_mask: wgpu::ColorWrites::ALL })], compilation_options: Default::default() }),
            primitive: Default::default(), depth_stencil: Some(wgpu::DepthStencilState { format: wgpu::TextureFormat::Depth32Float, depth_write_enabled: Some(false), depth_compare: Some(wgpu::CompareFunction::LessEqual), stencil: Default::default(), bias: Default::default() }),
            multisample: wgpu::MultisampleState { count: target.samples, ..Default::default() }, multiview_mask: None, cache: None,
        })
    }
}
```

## <a id="checkpoint-62im-own-the-world-text-buffer-and-pipeline-lifetime"></a>23. Own the world-text buffer and pipeline lifetime

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
        })
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        })
    }
}

impl Planes {
    pub(super) fn new(ctx: &GpuCtx, target: Target) -> Self {
        let layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("world text texture"),
                    entries: &[
                        texture_entry(
                            0,
                            ShaderStages::FRAGMENT,
                            TextureSampleType::Float { filterable: true },
                            false,
                        ),
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(
                                wgpu::SamplerBindingType::Filtering,
                            ),
                            count: None,
                        },
                    ],
                });
        let sampler = ctx
            .device
            .create_sampler(&wgpu::SamplerDescriptor {
                label: Some("world text coverage"),
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Linear,
                ..Default::default()
            });
        let id_pipeline = pipeline(ctx, Target::ID, &layout, true);
        let pipeline = pipeline(ctx, target, &layout, false);
        Self {
            cached: Vec::new(),
            draws: Vec::new(),
            vertices: GrowBuf::new(ctx, "world text vertices", 64, VERTS),
            layout,
            sampler,
            pipeline,
            id_pipeline,
            target,
            rasterizations: 0,
        }
    }

    pub(super) fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.pipeline = pipeline(ctx, target, &self.layout, false);
    }

    pub(super) fn reset(&mut self) {
        self.cached.clear();
        self.draws.clear();
        self.vertices.reset();
    }

    pub(super) fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.vertices.release(ctx);
    }

    pub(super) fn buffer_bytes(&self) -> u64 {
        self.vertices.buf.size()
    }

    pub(super) fn texture_bytes(&self) -> u64 {
        let mut total = 0;

        for cached in &self.cached {
            total += u64::from(cached.size[0]) * u64::from(cached.size[1]);
        }

        total
    }
}
```

## <a id="checkpoint-62in-rebuild-only-invalid-world-text-cache-entries"></a>24. Rebuild only invalid world-text cache entries

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
        total
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        total
    }
}

impl Planes {
    fn cached_plane(
        &mut self,
        ctx: &GpuCtx,
        run: &TextRun,
        fonts: &mut FontSystem,
        raster: &mut SwashCache,
        font_revision: u64,
        em_pixels: u32,
    ) -> anyhow::Result<usize> {
        let mut index = None;

        for (at, cached) in self.cached.iter().enumerate() {
            if cached.label.id == run.label.id {
                index = Some(at);
                break;
            }
        }

        let rebuild = match index {
            Some(at) => !same_raster(
                &self.cached[at],
                &run.label,
                font_revision,
                em_pixels,
            ),
            None => true,
        };

        if rebuild {
            let (pixels, size, extent) = rasterize(run, fonts, raster, em_pixels)?;
            let mut allocated = u64::from(size[0]) * u64::from(size[1]);

            for (at, cached) in self.cached.iter().enumerate() {
                if Some(at) != index {
                    allocated += u64::from(cached.size[0]) * u64::from(cached.size[1]);
                }
            }

            anyhow::ensure!(
                allocated <= TEXTURE_BUDGET,
                "world text coverage exceeds 32 MiB budget"
            );
            let cached = upload_plane(
                ctx,
                &self.layout,
                &self.sampler,
                &run.label,
                font_revision,
                em_pixels,
                (pixels, size, extent),
            );

            match index {
                Some(at) => self.cached[at] = cached,
                None => {
                    index = Some(self.cached.len());
                    self.cached.push(cached);
                }
            }

            self.rasterizations += 1;

            if raster.image_cache.len() > 4096 {
                *raster = SwashCache::new();
            }
        }

        Ok(index.expect("a prepared world plane has a cache entry"))
    }
}
```

## <a id="checkpoint-62io-prepare-and-draw-the-retained-world-text-planes"></a>25. Prepare and draw the retained world-text planes

### 1. `src/text_plane.rs`

**Locate this existing block:**

```rust
        Ok(index.expect("a prepared world plane has a cache entry"))
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Ok(index.expect("a prepared world plane has a cache entry"))
    }
}

impl Planes {
    pub(super) fn prepare(
        &mut self,
        ctx: &GpuCtx,
        document: &mut TextDocument,
        raster: &mut SwashCache,
        frame: &TextFrame,
    ) -> anyhow::Result<()> {
        self.draws.clear();
        self.vertices.reset();
        self.cached.retain(|cached| {
            document.runs.iter().any(|run| {
                run.label.id == cached.label.id
                    && matches!(
                        run.label.placement,
                        TextPlacement::WorldPlane { .. }
                    )
            })
        });
        let mut vertices = Vec::new();

        for run in &document.runs {
            if !matches!(
                run.label.placement,
                TextPlacement::WorldPlane { .. }
            ) || run.label.text.is_empty()
            {
                continue;
            }

            if let TextPlacement::WorldPlane { world, .. } = run.label.placement
                && frame.cut(world)
            {
                continue;
            }
            let em_pixels = raster_em(&run.label, frame).min(fitting_em(run));
            let index = self.cached_plane(
                ctx,
                run,
                &mut document.fonts,
                raster,
                document.font_revision,
                em_pixels,
            )?;

            let start = vertices.len() as u32;
            append_quad(
                &mut vertices,
                &run.label,
                self.cached[index].extent,
                frame,
                self.target.format.is_srgb(),
            );
            self.draws.push((index, start));
        }

        self.vertices.append(ctx, &vertices);
        Ok(())
    }

    fn draw_run(&self, pass: &mut wgpu::RenderPass<'_>, pipeline: &Pipeline) -> u32 {
        if self.draws.is_empty() {
            return 0;
        }

        pass.set_pipeline(pipeline);
        pass.set_vertex_buffer(0, self.vertices.buf.slice(..));

        for &(index, start) in &self.draws {
            pass.set_bind_group(0, &self.cached[index].bind, &[]);
            pass.draw(start..start + 6, 0..1);
        }

        self.draws.len() as u32
    }

    pub(super) fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> u32 {
        if self.draws.is_empty() {
            return 0;
        }

        self.draw_run(pass, &self.pipeline)
    }

    pub(super) fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        pass.set_bind_group(1, pick_transform, &[]);
        self.draw_run(pass, &self.id_pipeline)
    }
}
```

## <a id="checkpoint-62ip-collect-anchored-and-overlay-glyph-work-separately"></a>26. Collect anchored and overlay glyph work separately

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
#[path = "text_plane.rs"]
mod plane;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "text_plane.rs"]
mod plane;

struct PreparedText<'a> {
    overlays: Vec<TextArea<'a>>,
    anchors: Vec<TextArea<'a>>,
    depths: HashMap<usize, f32>,
    plates: Vec<plate::Rectangle>,
    overlay: bool,
    anchored: bool,
}

fn areas<'a>(
    runs: &'a [TextRun],
    frame: &TextFrame,
    scale: f32,
    stats: &mut TextStats,
    raster_keys: &mut HashSet<glyphon::CacheKey>,
) -> PreparedText<'a> {
    let mut overlay = false;
    let mut anchored = false;
    let mut overlays = Vec::new();
    let mut anchors = Vec::new();
    let mut depths = HashMap::new();
    let mut plates = Vec::new();
    stats.new_raster_keys = 0;
    stats.requested_glyphs = 0;
    stats.missing_glyphs = 0;

    for run in runs {
        let Some(mut placed) = place(&run.label, frame, scale) else {
            continue;
        };

        if let Some(rectangle) = text_rectangle(run, &mut placed, frame, scale) {
            plates.push(rectangle);
        }

        let area = TextArea {
            buffer: &run.buffer,
            left: placed.left,
            top: placed.top,
            scale: placed.scale,
            bounds: clip_bounds(&run.label, frame, scale),
            default_color: Color::rgba(
                run.label.ink_color()[0],
                run.label.ink_color()[1],
                run.label.ink_color()[2],
                run.label.ink_color()[3],
            ),
            custom_glyphs: &[],
        };

        for line in run.buffer.layout_runs() {
            for glyph in line.glyphs {
                stats.requested_glyphs += 1;
                stats.missing_glyphs += usize::from(glyph.glyph_id == 0);
                let physical = glyph.physical((placed.left, placed.top), placed.scale);
                stats.new_raster_keys +=
                    usize::from(raster_keys.insert(physical.cache_key));
            }
        }

        if !run.label.text.is_empty() {
            match run.label.placement {
                TextPlacement::Screen { .. } | TextPlacement::Nameplate { .. } => {
                    overlay = true
                }
                _ => anchored = true,
            }
        }

        if let Some(depth) = placed.depth {
            depths.insert(run.label.id as usize, depth);
            anchors.push(area);
        } else {
            overlays.push(area);
        }
    }
    PreparedText {
        overlays,
        anchors,
        depths,
        plates,
        overlay,
        anchored,
    }
}
```

## <a id="checkpoint-62iq-give-the-text-lane-its-retained-resource-owners"></a>27. Give the text lane its retained resource owners

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
        anchored,
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        anchored,
    }
}

pub struct Lane {
    pub document: TextDocument,
    pub stats: TextStats,
    cache: Cache,
    atlas: TextAtlas,
    viewport: Viewport,
    raster: SwashCache,
    overlay: TextRenderer,
    anchored: TextRenderer,
    plates: plate::Plates,
    planes: plane::Planes,
    target: Target,
    atlas_font_revision: u64,
    prepared: Option<(u64, u64, TextFrame)>,
    raster_keys: HashSet<glyphon::CacheKey>,
    overlay_count: u32,
    anchored_count: u32,
}

fn renderer(
    ctx: &GpuCtx,
    atlas: &mut TextAtlas,
    target: Target,
    compare: wgpu::CompareFunction,
) -> TextRenderer {
    TextRenderer::new(
        atlas,
        &ctx.device,
        wgpu::MultisampleState {
            count: target.samples,
            ..Default::default()
        },
        Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(false),
            depth_compare: Some(compare),
            stencil: Default::default(),
            bias: Default::default(),
        }),
    )
}

fn make_atlas(ctx: &GpuCtx, cache: &Cache, format: wgpu::TextureFormat) -> TextAtlas {
    let mode = if format.is_srgb() {
        ColorMode::Accurate
    } else {
        ColorMode::Web
    };
    TextAtlas::with_color_mode(&ctx.device, &ctx.queue, cache, format, mode)
}

fn depth_for(depths: &HashMap<usize, f32>, id: usize) -> f32 {
    depths.get(&id).copied().unwrap_or(0.0)
}
```

## <a id="checkpoint-62ir-construct-and-rebuild-the-text-lane-once"></a>28. Construct and rebuild the text lane once

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
    depths.get(&id).copied().unwrap_or(0.0)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    depths.get(&id).copied().unwrap_or(0.0)
}

impl Lane {
    pub fn new(ctx: &GpuCtx, target: Target) -> Self {
        let cache = Cache::new(&ctx.device);
        let viewport = Viewport::new(&ctx.device, &cache);
        let mut atlas = make_atlas(ctx, &cache, target.format);
        let overlay = renderer(
            ctx,
            &mut atlas,
            target,
            wgpu::CompareFunction::Always,
        );
        let anchored = renderer(
            ctx,
            &mut atlas,
            target,
            wgpu::CompareFunction::LessEqual,
        );
        Self {
            document: TextDocument::new(),
            stats: TextStats::default(),
            cache,
            atlas,
            viewport,
            raster: SwashCache::new(),
            overlay,
            anchored,
            plates: plate::Plates::new(ctx, target),
            planes: plane::Planes::new(ctx, target),
            target,
            atlas_font_revision: 1,
            prepared: None,
            raster_keys: HashSet::new(),
            overlay_count: 0,
            anchored_count: 0,
        }
    }

    fn rebuild_resources(&mut self, ctx: &GpuCtx) {
        self.atlas = make_atlas(ctx, &self.cache, self.target.format);
        self.overlay = renderer(
            ctx,
            &mut self.atlas,
            self.target,
            wgpu::CompareFunction::Always,
        );
        self.anchored = renderer(
            ctx,
            &mut self.atlas,
            self.target,
            wgpu::CompareFunction::LessEqual,
        );
        self.raster = SwashCache::new();
        self.raster_keys.clear();
        self.stats.raster_images = 0;
        self.stats.new_raster_images = 0;
        self.stats.raster_image_capacity_bytes = 0;
        self.stats.atlas_resets += 1;
        self.atlas_font_revision = self.document.font_revision;
        self.prepared = None;
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.plates.allocated_bytes() + self.planes.buffer_bytes()
    }

    pub fn texture_bytes(&self) -> u64 {
        self.planes.texture_bytes()
    }

    pub fn set_labels(&mut self, labels: Vec<TextLabel>) -> anyhow::Result<()> {
        self.document.set_labels(labels)
    }
}
```

## <a id="checkpoint-62is-reset-the-text-lane-and-record-scoped-preparation-costs"></a>29. Reset the text lane and record scoped preparation costs

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
        self.document.set_labels(labels)
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.document.set_labels(labels)
    }
}

impl Lane {
    fn measure_preparation(&mut self, raster_images_before: usize, start: f64) {
        self.stats.raster_images = 0;
        self.stats.raster_image_capacity_bytes = 0;

        for image in self.raster.image_cache.values().flatten() {
            self.stats.raster_images += 1;
            self.stats.raster_image_capacity_bytes += image.data.capacity();
        }

        self.stats.new_raster_images = self
            .stats
            .raster_images
            .saturating_sub(raster_images_before);
        self.stats.preparations += 1;
        self.stats.shape_count = self.document.shape_count;
        self.stats.shaping_ms = self.document.shaping_ms;
        self.stats.distinct_raster_keys = self.raster_keys.len();
        self.stats.active_instance_bytes = self.stats.requested_glyphs * 28;
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = self.planes.texture_bytes();
        self.stats.world_plane_rasterizations = self.planes.rasterizations;
        self.stats.preparation_ms = crate::load_measure::now() - start;
    }

    pub fn retarget(&mut self, ctx: &GpuCtx, target: Target) {
        self.target = target;
        self.plates.retarget(ctx, target);
        self.planes.retarget(ctx, target);
        self.overlay = renderer(
            ctx,
            &mut self.atlas,
            target,
            wgpu::CompareFunction::Always,
        );
        self.anchored = renderer(
            ctx,
            &mut self.atlas,
            target,
            wgpu::CompareFunction::LessEqual,
        );
        self.prepared = None;
    }

    pub fn reset(&mut self) {
        self.document.clear();
        self.prepared = None;
        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        self.planes.reset();
    }

    pub fn release(&mut self, ctx: &GpuCtx) {
        self.reset();
        self.plates.release(ctx);
        self.planes.release(ctx);
        self.stats.nameplate_capacity_bytes = self.plates.allocated_bytes();
        self.stats.world_plane_buffer_bytes = self.planes.buffer_bytes();
        self.stats.world_plane_texture_bytes = 0;
        self.rebuild_resources(ctx);
    }
}
```

## <a id="checkpoint-62it-prepare-glyphs-only-when-the-witness-changes"></a>30. Prepare glyphs only when the witness changes

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
        self.rebuild_resources(ctx);
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.rebuild_resources(ctx);
    }
}

impl Lane {
    pub fn prepare(&mut self, ctx: &GpuCtx, frame: &TextFrame) -> anyhow::Result<()> {
        let fixed = self.document.runs.iter().all(|run| {
            matches!(
                run.label.placement,
                TextPlacement::Screen { .. }
            )
        });
        let key = (
            self.document.revision,
            self.document.font_revision,
            if fixed {
                frame.canvas()
            } else {
                frame.clone()
            },
        );

        if self.prepared.as_ref() == Some(&key) {
            self.stats.skipped_preparations += 1;
            return Ok(());
        }

        self.overlay_count = 0;
        self.anchored_count = 0;
        self.plates.reset();
        let scale = frame.scale()?;
        let start = crate::load_measure::now();
        let fonts_changed = self.atlas_font_revision != self.document.font_revision;

        if fonts_changed || self.raster_keys.len() > 4096 {
            self.rebuild_resources(ctx);
        }

        let raster_images_before = self
            .raster
            .image_cache
            .values()
            .flatten()
            .count();
        self.planes.prepare(
            ctx,
            &mut self.document,
            &mut self.raster,
            frame,
        )?;
        self.atlas.trim();
        self.viewport.update(
            &ctx.queue,
            Resolution {
                width: frame.framebuffer[0],
                height: frame.framebuffer[1],
            },
        );
        let prepared = areas(
            &self.document.runs,
            frame,
            scale,
            &mut self.stats,
            &mut self.raster_keys,
        );
        self.overlay_count = u32::from(prepared.overlay);
        self.anchored_count = u32::from(prepared.anchored);

        self.overlay.prepare(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            prepared.overlays,
            &mut self.raster,
        )?;
        self.anchored.prepare_with_depth(
            &ctx.device,
            &ctx.queue,
            &mut self.document.fonts,
            &mut self.atlas,
            &self.viewport,
            prepared.anchors,
            &mut self.raster,
            |id| depth_for(&prepared.depths, id),
        )?;
        self.plates
            .prepare(ctx, &prepared.plates, frame.framebuffer);
        self.measure_preparation(raster_images_before, start);
        self.prepared = Some(key);
        Ok(())
    }
}
```

## <a id="checkpoint-62iu-draw-the-prepared-text-passes-in-order"></a>31. Draw the prepared text passes in order

### 1. `src/scene_text_gpu.rs`

**Locate this existing block:**

```rust
        Ok(())
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        Ok(())
    }
}

impl Lane {
    pub fn draw_ids(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        pick_transform: &wgpu::BindGroup,
    ) -> u32 {
        self.planes.draw_ids(pass, pick_transform)
            + self.plates.draw_ids(pass, pick_transform)
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) -> anyhow::Result<u32> {
        let mut draws = self.planes.draw(pass);
        draws += self.plates.draw(pass, false);

        if self.anchored_count != 0 {
            self.anchored
                .render(&self.atlas, &self.viewport, pass)?;
            draws += 1;
        }

        draws += self.plates.draw(pass, true);

        if self.overlay_count != 0 {
            self.overlay
                .render(&self.atlas, &self.viewport, pass)?;
            draws += 1;
        }

        Ok(draws)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn frame(scale: f64) -> TextFrame {
        TextFrame {
            mvp: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ],
            origin: [0.0; 3],
            framebuffer: [
                (800.0 * scale) as u32,
                (600.0 * scale) as u32,
            ],
            logical: [800.0, 600.0],
            ortho_half_height: 1.0,
            clip: [[0.0; 4]; 4],
        }
    }

    #[test]
    fn logical_to_physical_scale_is_applied_once() {
        for scale in [1.0, 1.25, 1.5, 2.0] {
            let frame = frame(scale);
            assert_eq!(frame.scale().unwrap(), scale as f32);
            let label = TextLabel {
                object: None,
                id: 1,
                text: "A".into(),
                font_size: 16.0,
                line_height: 24.0,
                color: [255; 4],
                placement: TextPlacement::Screen {
                    left: 12.25,
                    top: 8.5,
                },
                clip: None,
            };
            let placed = place(&label, &frame, frame.scale().unwrap()).unwrap();
            assert_eq!(placed.left, 12.25 * scale as f32);
            assert_eq!(
                placed.scale * label.font_size,
                16.0 * scale as f32
            );
        }
    }

    #[test]
    fn scene_anchor_preserves_rebased_depth_and_culls_near_plane() {
        let mut frame = frame(2.0);
        frame.origin = [1_000_000.0, 0.0, 0.0];
        let mut label = TextLabel {
            object: None,
            id: 1,
            text: "A".into(),
            font_size: 16.0,
            line_height: 24.0,
            color: [255; 4],
            placement: TextPlacement::Anchor {
                world: [1_000_000.0, 0.0, 0.25],
                offset: [0.0; 2],
            },
            clip: None,
        };
        let placed = place(&label, &frame, 2.0).unwrap();
        assert_eq!((placed.left, placed.top), (800.0, 600.0));
        assert_eq!(placed.depth, Some(0.25));
        label.placement = TextPlacement::Anchor {
            world: [1_000_000.0, 0.0, 1.01],
            offset: [0.0; 2],
        };
        assert!(place(&label, &frame, 2.0).is_none());
    }

    #[test]
    fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system() {
        let mut document = TextDocument::new();
        let mut label = TextLabel {
            object: None,
            id: 1,
            text: "Sphere Ø25".into(),
            font_size: 18.0,
            line_height: 26.0,
            color: [255; 4],
            placement: TextPlacement::Nameplate {
                world: [0.0, 0.0, 0.25],
                padding: [6.0, 4.0],
                rounded: false,
            },
            clip: None,
        };
        document
            .set_labels(vec![label.clone()])
            .unwrap();

        for scale in [1.0, 1.25, 2.0] {
            let frame = frame(scale);
            let run = &document.runs[0];
            let mut placed = place(&run.label, &frame, scale as f32).unwrap();
            let rectangle = text_rectangle(run, &mut placed, &frame, scale as f32)
                .unwrap()
                .bounds;
            assert_eq!(
                placed.depth, None,
                "a source-center annotation overlays its own solid"
            );
            assert!(
                ((rectangle[0] + rectangle[2]) * 0.5 - 400.0 * scale as f32).abs()
                    < 0.001
            );
            assert!(
                ((rectangle[1] + rectangle[3]) * 0.5 - 300.0 * scale as f32).abs()
                    < 0.001
            );
            assert!((rectangle[3] - rectangle[1] - 34.0 * scale as f32).abs() < 0.001);
        }

        label.clip = Some([390.0, 290.0, 410.0, 310.0]);
        label.color = [255, 255, 0, 255];
        document
            .set_labels(vec![label.clone()])
            .unwrap();
        assert_eq!(
            document.shape_count, 1,
            "annotation style and clip reuse shaping"
        );
        let frame = frame(2.0);
        let run = &document.runs[0];
        let mut placed = place(&run.label, &frame, 2.0).unwrap();
        assert_eq!(
            text_rectangle(run, &mut placed, &frame, 2.0)
                .unwrap()
                .clip,
            [780.0, 580.0, 820.0, 620.0]
        );
        label.placement = TextPlacement::Nameplate {
            world: [0.0, 0.0, 0.25],
            padding: [f32::NAN, 4.0],
            rounded: false,
        };
        assert!(document.set_labels(vec![label]).is_err());
        assert_eq!(document.shape_count, 1);
        assert_eq!(
            document.runs.len(),
            1,
            "invalid padding preserves the previous document"
        );
    }
}
```

## <a id="checkpoint-62j-frame-pass"></a>32. Prepare the full text frame and load the scene depth

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.text_error.borrow_mut().take();
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            match lane.prepare(
                &self.device,
                &self.queue,
                self.size,
                self.density,
            ) {
                Ok(()) => {
                    let mut pass =
                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("screen glyphs"),
                            color_attachments: &[Some(
```

Replace it with:

```rust
        self.text_error.borrow_mut().take();
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            match lane.prepare(
                &self.text_context(),
                &self.text_frame(*transform),
            ) {
                Ok(()) => {
                    let mut pass =
                        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("scene glyphs"),
                            depth_stencil_attachment: Some(
                                wgpu::RenderPassDepthStencilAttachment {
                                    view: &self.depth,
                                    depth_ops: Some(wgpu::Operations {
                                        load: wgpu::LoadOp::Load,
                                        store: wgpu::StoreOp::Store,
                                    }),
                                    stencil_ops: None,
                                },
                            ),
                            color_attachments: &[Some(
```

**Locate this block:**

```rust
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                let [rasters, bytes] = lane.raster_usage();
                [
                    lane.document.shape_count,
                    lane.preparations,
                    lane.skips,
                    rasters as u64,
                    bytes as u64,
                    lane.resets,
                ]
            })
    }
```

Replace it with:

```rust
            .borrow()
            .as_ref()
            .map_or([0; 6], |lane| {
                [
                    lane.document.shape_count,
                    lane.stats.preparations,
                    lane.stats.skipped_preparations,
                    lane.stats.raster_images as u64,
                    lane.stats.raster_image_capacity_bytes as u64,
                    lane.stats.atlas_resets,
                ]
            })
    }
```

**Locate this block:**

```rust
        }
        let mut lane = self.text.borrow_mut();
        let lane = lane.get_or_insert_with(|| {
            crate::text_atlas::Lane::new(&self.device, &self.queue, self.format)
        });
        lane.document.set_labels(labels)
    }
```

Replace it with:

```rust
        }
        let mut lane = self.text.borrow_mut();
        let lane = lane.get_or_insert_with(|| {
            crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            )
        });
        lane.document.set_labels(labels)
    }
```

**Locate this block:**

```rust
        self.widget.extent()
    }

    pub fn set_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
```

Replace it with:

```rust
        self.widget.extent()
    }

    fn text_context(&self) -> crate::text_gpu_support::GpuCtx {
        crate::text_gpu_support::GpuCtx::new(&self.device, &self.queue)
    }
    fn text_frame(&self, transform: [f32; 16]) -> crate::scene_text_gpu::TextFrame {
        crate::scene_text_gpu::TextFrame {
            mvp: transform,
            origin: [0.; 3],
            framebuffer: self.size,
            logical: [
                self.size[0] as f64 / self.density as f64,
                self.size[1] as f64 / self.density as f64,
            ],
            ortho_half_height: 0.,
            clip: [[0.; 4]; 4],
        }
    }
    pub fn set_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
```

**Locate this block:**

```rust
pub struct Renderer {
    text: std::cell::RefCell<Option<crate::text_atlas::Lane>>,
    text_error: std::cell::RefCell<Option<String>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
```

Replace it with:

```rust
pub struct Renderer {
    text: std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error: std::cell::RefCell<Option<String>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
```

## <a id="checkpoint-62k-lane-labels"></a>33. Merge scene labels with the screen specimen

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            clip: [[0.; 4]; 4],
        }
    }
    pub fn set_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        let lane = lane.get_or_insert_with(|| {
            crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            )
        });
        lane.document.set_labels(labels)
    }
```

Replace it with:

```rust
            clip: [[0.; 4]; 4],
        }
    }
    fn adopt_text(&self, labels: Vec<crate::text::TextLabel>) -> anyhow::Result<()> {
        if labels.is_empty() {
            self.text.borrow_mut().take();
            self.text_error.borrow_mut().take();
            return Ok(());
        }
        let mut lane = self.text.borrow_mut();
        if lane.is_none() {
            let mut new = crate::scene_text_gpu::Lane::new(
                &self.text_context(),
                crate::text_gpu_support::Target {
                    format: self.format,
                    samples: 1,
                },
            );
            if let Some(fonts) = self.full_text_fonts.borrow().as_ref() {
                new.document.replace_fonts(fonts.clone())?;
            }
            *lane = Some(new);
        }
        lane.as_mut().unwrap().set_labels(labels)
    }
    pub fn set_text(
        &self,
        mut labels: Vec<crate::text::TextLabel>,
    ) -> anyhow::Result<()> {
        for (i, label) in labels.iter_mut().enumerate() {
            label.id = u32::MAX - i as u32;
            label.object = None;
        }
        let mut merged = self.scene_text.borrow().clone();
        merged.extend(labels.clone());
        self.adopt_text(merged)?;
        *self.presentation_text.borrow_mut() = labels;
        Ok(())
    }
    fn set_scene_text(
        &self,
        scene: &Scene,
        selected: &[ObjectId],
    ) -> anyhow::Result<()> {
        let labels = scene
            .text_labels(selected)
            .map_err(anyhow::Error::msg)?;
        let mut merged = labels.clone();
        merged.extend(self.presentation_text.borrow().clone());
        self.adopt_text(merged)?;
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
```

**Locate this block:**

```rust
            self.highlight
                .objects(&self.device, scene, ids)?;
        }
        Ok(())
    }
```

Replace it with:

```rust
            self.highlight
                .objects(&self.device, scene, ids)?;
        }
        self.set_scene_text(scene, ids)
            .map_err(|_| "Cannot prepare scene labels")?;
        Ok(())
    }
```

**Locate this block:**

```rust
        }
        drop(old);
        self.geometry.prune();
    }

    pub fn request_pick(
```

Replace it with:

```rust
        }
        drop(old);
        self.geometry.prune();
        if let Err(error) = self.set_scene_text(scene, selected.as_slice()) {
            *self.text_error.borrow_mut() = Some(error.to_string());
        }
    }

    pub fn request_pick(
```

**Locate this block:**

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
```

Replace it with:

```rust
        let faces = crate::face_pick::Faces::new(&device);
        let mesh_ink = crate::stroke_gpu::Lane::new(&device, format);
        let mut renderer = Self {
            full_text_fonts: Default::default(),
            presentation_text: Default::default(),
            scene_text: Default::default(),
            text: std::cell::RefCell::new(None),
            text_error: std::cell::RefCell::new(None),
            format,
```

**Locate this block:**

```rust
    text_error: std::cell::RefCell<Option<String>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
```

Replace it with:

```rust
    text_error: std::cell::RefCell<Option<String>>,
    scene_text: std::cell::RefCell<Vec<crate::text::TextLabel>>,
    presentation_text: std::cell::RefCell<Vec<crate::text::TextLabel>>,
    full_text_fonts: std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
```

## <a id="checkpoint-62l-picking"></a>34. Draw text IDs through bounded readback

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
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn text_error(&self) -> Option<String> {
```

Replace it with:

```rust
        *self.scene_text.borrow_mut() = labels;
        Ok(())
    }
    pub fn refresh_text_bounds(&self, scene: &mut Scene) {
        if let Some(lane) = self.text.borrow().as_ref() {
            for row in scene.texts_mut() {
                if let Some(run) = lane
                    .document
                    .runs
                    .iter()
                    .find(|run| run.label.id == row.id.value())
                {
                    row.prepared.extent =
                        run.buffer
                            .layout_runs()
                            .fold([0f64; 2], |[w, h], line| {
                                [
                                    w.max(line.line_w as f64),
                                    h.max((line.line_top + line.line_height) as f64),
                                ]
                            });
                }
            }
        }
    }
    pub fn text_stats(&self) -> Option<crate::scene_text_gpu::TextStats> {
        self.text
            .borrow()
            .as_ref()
            .map(|lane| lane.stats.clone())
    }
    pub fn text_scene_usage(&self) -> [u64; 4] {
        self.text
            .borrow()
            .as_ref()
            .map_or([0; 4], |lane| {
                [
                    lane.allocated_bytes(),
                    lane.texture_bytes(),
                    lane.stats.world_plane_rasterizations,
                    lane.document.runs.len() as u64,
                ]
            })
    }
    pub fn replace_text_fonts(&self, fonts: Vec<Vec<u8>>) -> anyhow::Result<()> {
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            lane.document.replace_fonts(fonts.clone())?;
        } else {
            let mut document = crate::text::TextDocument::new();
            document.replace_fonts(fonts.clone())?;
        }
        *self.full_text_fonts.borrow_mut() = Some(fonts);
        Ok(())
    }
    pub fn clear_text_fonts(&self) {
        self.full_text_fonts.borrow_mut().take();
    }

    pub fn text_error(&self) -> Option<String> {
```

**Locate this block:**

```rust
                    &self.markers,
                ],
                &self.faces,
            );
        }
    }
```

Replace it with:

```rust
                    &self.markers,
                ],
                &self.faces,
                self.text.borrow().as_ref(),
            );
        }
    }
```

**Locate this block:**

```rust
                &self.sources.mesh_bases,
            );
        }
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
```

Replace it with:

```rust
                &self.sources.mesh_bases,
            );
        }
        if let Some(lane) = self.text.borrow_mut().as_mut() {
            if let Err(error) = lane.prepare(
                &self.text_context(),
                &self.text_frame(transform),
            ) {
                *self.text_error.borrow_mut() = Some(error.to_string());
                return;
            }
        }
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
```

**Locate this block:**

```rust
    text_error: std::cell::RefCell<Option<String>>,
    scene_text: std::cell::RefCell<Vec<crate::text::TextLabel>>,
    presentation_text: std::cell::RefCell<Vec<crate::text::TextLabel>>,
    full_text_fonts: std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
```

Replace it with:

```rust
    text_error: std::cell::RefCell<Option<String>>,
    full_text_fonts: std::cell::RefCell<Option<Vec<Vec<u8>>>>,
    presentation_text: std::cell::RefCell<Vec<crate::text::TextLabel>>,
    scene_text: std::cell::RefCell<Vec<crate::text::TextLabel>>,
    format: wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
```

### 2. `src/pick_gpu.rs`

**Locate this existing block:**

```rust
        faces: &crate::face_pick::Faces,
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
        );
        let request = queued.request;
        let window = request.window;
        if self.image.as_ref().is_none_or(|image| {
            image.window.draw[2..] != window.draw[2..]
                || image.readback.size() != window.bytes()
        }) {
            self.image = Some(crate::pick_image::Image::new(device, window));
        }
        let image = self.image.as_mut().unwrap();
        image.window = window;
        let mut bytes: Vec<u8> = request
            .transform
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let mut bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.extend((request.mode as u32).to_ne_bytes());
        bytes.resize(80, 0);
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
                queue,
                &request.transform,
                window.full,
                request.density,
            );
        }
        queue.submit([crate::pick_encode::draw(
            device,
            image,
            window,
            &self.pipeline,
            &self.view_group,
            &self.crop_group,
            meshes,
            ink,
            faces,
            request.mode,
        )]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        faces: &crate::face_pick::Faces,
        text: Option<&crate::scene_text_gpu::Lane>,
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
        );
        let request = queued.request;
        let window = request.window;
        if self.image.as_ref().is_none_or(|image| {
            image.window.draw[2..] != window.draw[2..]
                || image.readback.size() != window.bytes()
        }) {
            self.image = Some(crate::pick_image::Image::new(device, window));
        }
        let image = self.image.as_mut().unwrap();
        image.window = window;
        let mut bytes: Vec<u8> = request
            .transform
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.resize(80, 0);
        queue.write_buffer(&self.view, 0, &bytes);
        let mut bytes: Vec<u8> = window
            .crop()
            .iter()
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.extend((request.mode as u32).to_ne_bytes());
        bytes.resize(80, 0);
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
            lane.view(
                queue,
                &request.transform,
                window.full,
                request.density,
            );
        }
        queue.submit([crate::pick_encode::draw(
            device,
            image,
            window,
            &self.pipeline,
            &self.view_group,
            &self.crop_group,
            meshes,
            ink,
            faces,
            request.mode,
            text,
        )]);
```

### 3. `src/pick_encode.rs`

**Locate this existing block:**

```rust
    mode: crate::pick_sources::Mode,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, crop_group, &[]);
        for mesh in meshes {
            mesh.draw(&mut pass);
        }
        if mode == crate::pick_sources::Mode::Face {
            faces.draw(&mut pass, meshes, view_group, crop_group);
        }
        for lane in ink {
            lane.pick(&mut pass, crop_group);
        }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    mode: crate::pick_sources::Mode,
    text: Option<&crate::scene_text_gpu::Lane>,
) -> wgpu::CommandBuffer {
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("IDs"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, view_group, &[]);
        pass.set_bind_group(2, crop_group, &[]);
        for mesh in meshes {
            mesh.draw(&mut pass);
        }
        if mode == crate::pick_sources::Mode::Face {
            faces.draw(&mut pass, meshes, view_group, crop_group);
        }
        for lane in ink {
            lane.pick(&mut pass, crop_group);
        }
        if let Some(text) = text {
            text.draw_ids(&mut pass, crop_group);
        }
```

## <a id="checkpoint-62m-whole-fonts"></a>35. Fetch missing whole fonts as one checked set

### 1. `src/text_fonts.rs`

Create the file and type:

```rust
use std::cell::{Cell, RefCell};
use std::rc::Rc;
#[derive(Default)]
pub struct Inbox {
    pub asked: Cell<bool>,
    epoch: Cell<u64>,
    retry_after: Cell<f64>,
    pub reply: RefCell<Option<Result<Vec<Vec<u8>>, String>>>,
}
impl Inbox {
    pub fn clear(&self) {
        self.epoch
            .set(self.epoch.get().wrapping_add(1));
        self.asked.set(false);
        self.retry_after.set(0.);
        self.reply.borrow_mut().take();
    }
    pub fn failed(&self) {
        self.asked.set(false);
        self.retry_after
            .set(crate::load_measure::now() + 1000.);
    }
    pub fn need(self: &Rc<Self>, words: impl IntoIterator<Item = impl AsRef<str>>) {
        if self.asked.get()
            || crate::load_measure::now() < self.retry_after.get()
            || words
                .into_iter()
                .all(|s| crate::text::covers(s.as_ref()))
        {
            return;
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.asked.set(true);
            let inbox = Rc::clone(self);
            let epoch = self.epoch.get();
            wasm_bindgen_futures::spawn_local(async move {
                let reply = fetch().await;
                if inbox.epoch.get() != epoch {
                    return;
                }
                if reply.is_err() {
                    inbox.failed();
                }
                *inbox.reply.borrow_mut() = Some(reply);
                crate::browser_pick::wake();
            });
        }
    }
}
#[cfg(target_arch = "wasm32")]
async fn fetch() -> Result<Vec<Vec<u8>>, String> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().ok_or("Missing browser window")?;
    let mut faces = Vec::new();
    for name in crate::text::FULL_FONTS {
        let response = wasm_bindgen_futures::JsFuture::from(
            window.fetch_with_str(&format!("text/{name}")),
        )
        .await
        .map_err(|e| format!("Cannot fetch {name}: {e:?}"))?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| "Invalid font response")?;
        if !response.ok() {
            return Err(format!(
                "Font {name}: HTTP {}",
                response.status()
            ));
        }
        let promise = response
            .array_buffer()
            .map_err(|e| format!("Cannot read {name}: {e:?}"))?;
        let data = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .map_err(|e| format!("Cannot read {name}: {e:?}"))?;
        let bytes = js_sys::Uint8Array::new(&data);
        if bytes.length() == 0 || bytes.length() > 4 * 1024 * 1024 {
            return Err(format!(
                "Font {name} exceeds the 4 MiB limit"
            ));
        }
        faces.push(bytes.to_vec());
    }
    Ok(faces)
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod text_gpu_support;

pub mod scene_text_gpu;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod text_gpu_support;

pub mod scene_text_gpu;

pub mod text_fonts;
```

## <a id="checkpoint-62n-diagnostics"></a>36. Show current text sources metrics and preparation errors

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        ..Default::default()
    });
    renderer.draw_mode(&view, background, transform, normals);
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
```

Replace it with:

```rust
        ..Default::default()
    });
    renderer.draw_mode(&view, background, transform, normals);
    if let Some(error)=renderer.text_error() {panel.result(&format!("Text: {error}"));}
    panel.draw(renderer, &view);
    frame.present();
    Ok(())
```

**Locate this block:**

```rust
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
```

Replace it with:

```rust
    canvas.set_attribute("data-row-placements", &serde_json::to_string(&placements).map_err(|error| JsValue::from_str(&error.to_string()))?)?;
    canvas.set_attribute("data-pick-gpu", &serde_json::to_string(&renderer.pick_usage()).unwrap())?;
    canvas.set_attribute("data-point-gpu", &serde_json::to_string(&renderer.point_usage()).unwrap())?;
    canvas.set_attribute("data-scene-text-stats",&serde_json::to_string(&renderer.text_stats()).unwrap())?;
    canvas.set_attribute("data-scene-text-cpu",&serde_json::to_string(&crate::memory::texts(editor.presentation_scenes())).unwrap())?;
    canvas.set_attribute("data-scene-text-gpu",&serde_json::to_string(&renderer.text_scene_usage()).unwrap())?;
    canvas.set_attribute("data-scene-texts",&serde_json::json!(editor.scene.texts().iter().map(|row|serde_json::json!({"id":row.id.value(),"guid":row.guid,"source":row.prepared.source,"model":row.model.m})).collect::<Vec<_>>()).to_string())?;
    canvas.set_attribute("data-text-error",&renderer.text_error().unwrap_or_default())?;
    canvas.set_attribute("data-text-atlas",&serde_json::to_string(&renderer.text_usage()).unwrap())?;
    canvas.set_attribute("data-cad-cache",&serde_json::to_string(&crate::memory::cads(editor.presentation_scenes())).unwrap())?;
```

**Locate this block:**

```rust
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    let view_ids=editor.scene.objects().iter().map(|row|row.id).chain(editor.scene.lines().iter().map(|row|row.id)).chain(editor.scene.paths().iter().map(|row|row.id)).chain(editor.scene.points().iter().map(|row|row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
```

Replace it with:

```rust
        .get_element_by_id("canvas")
        .ok_or("Missing canvas")?;
    let placements: Vec<_> = editor.scene.objects().iter().map(|row| (format!("{:?}", row.id), row.model.m)).collect();
    let view_ids=editor.scene.objects().iter().map(|row|row.id).chain(editor.scene.lines().iter().map(|row|row.id)).chain(editor.scene.paths().iter().map(|row|row.id)).chain(editor.scene.points().iter().map(|row|row.id)).chain(editor.scene.texts().iter().map(|row|row.id));
    let view_rows=view_ids.map(|id|serde_json::json!({"id":id.value(),"visible":editor.scene.visible(id),"locked":editor.scene.view.locked.contains(&id),"faces":editor.scene.view.faces.get(&id),"edges":editor.scene.view.edges.get(&id)})).collect::<Vec<_>>();
    canvas.set_attribute("data-canvas-drag",&editor.canvas_drag.as_ref().map_or_else(||"null".into(),|d|serde_json::json!({"from":d.from,"to":d.to,"released":d.released,"missed":d.missed,"preview":d.preview.as_ref().map(|p|p.matrix())}).to_string()))?;
    canvas.set_attribute("data-camera-frame",&serde_json::to_string(&editor.camera.frame).unwrap())?;
```

**Locate this block:**

```rust
        }
        if let Some(draft) = editor.draft.as_ref() { panel.tool(draft.prompt(), draft.tool.buttons()); }
        else { panel.tool(String::new(), Vec::new()); }
        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
```

Replace it with:

```rust
        }
        if let Some(draft) = editor.draft.as_ref() { panel.tool(draft.prompt(), draft.tool.buttons()); }
        else { panel.tool(String::new(), Vec::new()); }
        if let Some(reply)=text_fonts.reply.borrow_mut().take() {match reply {Ok(fonts)=>{if let Err(error)=renderer.replace_text_fonts(fonts) {text_fonts.failed();panel.result(&format!("Text: {error}"));}},Err(error)=>panel.result(&format!("Text: {error}"))}}
        renderer.refresh_text_bounds(&mut editor.scene);
        text_fonts.need(editor.scene.texts().iter().filter(|row|editor.scene.visible(row.id)).map(|row|row.prepared.source.label.text.as_str()));
        let rect=pointer_canvas.get_bounding_client_rect();
        let number=editor.number_prompt([rect.width(),rect.height()]);
        if editor.gizmo_edit.as_ref().is_some_and(|e|e.gizmo.typing.is_some()) && number.is_none() {
```

**Locate this block:**

```rust
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { request.borrow_mut().cancel();let _=renderer.set_text(Vec::new()); }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
```

Replace it with:

```rust
            match command {
            Err(error) => { panel.result(&error); None }
            Ok(crate::commands::Command::Editor(mut action)) => {
                if matches!(action, Action::Close) { request.borrow_mut().cancel();let _={text_fonts.clear();renderer.clear_text_fonts();renderer.set_text(Vec::new())}; }
                if shortcut.is_some() && editor.selected.is_some() && matches!(action, Action::Fit) { action = Action::FitSelected; }
                Some(action)
            }
```

**Locate this block:**

```rust
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let mut math_view=crate::math_view::Presentation::default();
    let mut renderer = Renderer::new(
        device,
```

Replace it with:

```rust
    surface.configure(&device, &config);
    let mut editor = shared.borrow_mut();
    let pipeline_started = crate::browser_phase::now();
    let text_fonts=std::rc::Rc::new(crate::text_fonts::Inbox::default());
    let mut math_view=crate::math_view::Presentation::default();
    let mut renderer = Renderer::new(
        device,
```

## <a id="checkpoint-62o-labels"></a>37. Create pick move and delete the scene labels

### 1. `src/editor.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                self.objects.clear();
                self.selected = None;
            }
```

Replace it with:

```rust
                self.objects.clear();
                self.selected = None;
            }
            Action::AddText(placement) => {
                let source = std::rc::Rc::new(crate::scene_label::example(placement));
                let prepared = crate::scene_label::Prepared::new(source)?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_text(prepared).map(|_| ())
                    })?;
            }
```

**Locate this block:**

```rust
    AddCad,
    AddSurface,
    AddLine,
    AddPoint,
```

Replace it with:

```rust
    AddCad,
    AddText(crate::text::TextPlacement),
    AddSurface,
    AddLine,
    AddPoint,
```

### 2. `src/commands.rs`

**Locate this existing block:**

```rust
    ("Example CAD", || Action::AddCad),
    ("Example Surface", || Action::AddSurface),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ("Example CAD", || Action::AddCad),
    ("Example Scene Text", || {
        Action::AddText(crate::text::TextPlacement::WorldPlane {
            world: [-1.5, 0., 1.],
            right: [1., 0., 0.],
            up: [0., 1., 0.],
            world_height: 0.22,
        })
    }),
    ("Example Billboard", || {
        Action::AddText(crate::text::TextPlacement::WorldBillboard {
            world: [-1.5, 0., 1.],
            world_height: 0.22,
        })
    }),
    ("Example Anchor", || {
        Action::AddText(crate::text::TextPlacement::Anchor {
            world: [0., 0., 1.],
            offset: [8., -12.],
        })
    }),
    ("Example Nameplate", || {
        Action::AddText(crate::text::TextPlacement::Nameplate {
            world: [0., 0., 1.],
            padding: [17., 4.],
            rounded: true,
        })
    }),
    ("Example Surface", || Action::AddSurface),
```

### 3. `src/scene_label_tests.rs`

Create the file and type:

```rust
use crate::{
    editor::{Action, Editor},
    scene_label::{Prepared, example},
    text::TextPlacement,
};
use std::rc::Rc;
fn plane(world: [f64; 3]) -> TextPlacement {
    TextPlacement::WorldPlane {
        world,
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.22,
    }
}

#[test]
fn original_scene_words_precision_placement_and_history_keep_one_source_owner() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    let source = Rc::new(example(plane([
        1_000_000_000.125,
        2.25,
        0.5,
    ])));
    let saved = serde_json::to_string(&*source).unwrap();
    let prepared = Prepared::new(source.clone()).unwrap();
    let id = e.scene.insert_text(prepared).unwrap();
    e.apply(Action::Picked(Some(id))).unwrap();
    let intent = crate::edit_intent::Intent::Move {
        id,
        offset: [0.25, -0.5, 1.],
    };
    assert!(
        intent.keys(&e).unwrap().is_empty(),
        "An already loaded text needs no mesh source fetch"
    );
    assert!(matches!(
        intent.replay(&mut e).unwrap(),
        crate::edit_replay::Reply::Changed(crate::editor::Change::Scene)
    ));
    assert_eq!(
        e.scene.texts()[0]
            .label(false, true)
            .unwrap()
            .placement,
        plane([1_000_000_000.375, 1.75, 1.5])
    );
    assert!(Rc::ptr_eq(
        &e.scene.texts()[0].prepared.source,
        &source
    ));
    assert_eq!(
        serde_json::to_string(&*source).unwrap(),
        saved
    );
    assert_eq!(
        crate::memory::texts(e.presentation_scenes())[1],
        1
    );
    e.apply(Action::Undo).unwrap();
    assert_eq!(
        e.scene.texts()[0]
            .label(false, true)
            .unwrap()
            .placement,
        plane([1_000_000_000.125, 2.25, 0.5])
    );
    e.apply(Action::Redo).unwrap();
    let original = e.scene.texts()[0].model.m;
    let mut scale = session_rust::Xform::identity();
    scale.m[0] = 2.;
    assert!(
        crate::history::History::default()
            .try_edit(&mut e.scene, |s| s.place_text(id, scale))
            .is_err()
    );
    assert_eq!(e.scene.texts()[0].model.m, original);
    assert_eq!(
        serde_json::to_string(&*source).unwrap(),
        saved
    );
    assert!(
        crate::document::snapshot(&e.scene).is_err(),
        "Mixed text persistence connects later; never silently omit it"
    );
}

#[test]
fn text_rows_follow_layers_selection_delete_undo_copy_and_close_lifetime() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddText(plane([0.; 3])))
        .unwrap();
    let id = e.scene.texts()[0].id;
    let weak = Rc::downgrade(&e.scene.texts()[0].prepared.source);
    let rows = crate::hierarchy::rows(&e.scene);
    let key = rows
        .iter()
        .find(|row| row.ids == [id] && row.depth == 1)
        .unwrap()
        .key
        .clone();
    e.apply(Action::LayerProperty {
        key: key.clone(),
        property: crate::layer_properties::Property::Locked(true),
    })
    .unwrap();
    e.apply(Action::SelectAll).unwrap();
    assert!(e.selected_objects().is_empty());
    assert_eq!(
        e.scene.text_labels(&[]).unwrap()[0]
            .object
            .unwrap()
            .row,
        u32::MAX
    );
    e.apply(Action::LayerProperty {
        key: key.clone(),
        property: crate::layer_properties::Property::Locked(false),
    })
    .unwrap();
    e.apply(Action::SelectAll).unwrap();
    assert_eq!(e.selected_objects(), vec![id]);
    let source = e.scene.texts()[0].prepared.source.clone();
    let copied = e.scene.copy_row(id, false).unwrap();
    assert_ne!(copied, id);
    assert_ne!(
        e.scene.texts()[0].guid,
        e.scene.texts()[1].guid
    );
    assert!(Rc::ptr_eq(
        &source,
        &e.scene.texts()[1].prepared.source
    ));
    e.scene.remove(copied);
    drop(source);
    e.apply(Action::LayerProperty {
        key,
        property: crate::layer_properties::Property::Hidden(true),
    })
    .unwrap();
    assert!(e.scene.text_labels(&[]).unwrap().is_empty());
    e.apply(Action::Show).unwrap();
    e.apply(Action::Picked(Some(id))).unwrap();
    e.apply(Action::Delete).unwrap();
    assert!(e.scene.texts().is_empty());
    assert!(weak.upgrade().is_some());
    e.apply(Action::Undo).unwrap();
    assert_eq!(e.scene.texts()[0].id, id);
    e.apply(Action::Redo).unwrap();
    e.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert_eq!(
        crate::memory::texts(e.presentation_scenes()),
        [0; 4]
    );
}

#[test]
fn world_text_bounds_and_rectangles_use_the_original_words_and_plate_extent() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddText(plane([0., 0., 0.5])))
        .unwrap();
    let row = &e.scene.texts()[0];
    let id = row.id;
    let bounds = e.scene.selected_bounds(id).unwrap();
    assert!(bounds.max[0] > 1. && bounds.min[1] < 0. && bounds.max[2] == 0.5);
    assert_eq!(e.scene.bounds().unwrap().min, bounds.min);
    let matrix = session_rust::Xform::identity().m;
    let points =
        crate::scene_label_screen::polygon(row, &matrix, [800., 600.]).unwrap();
    let small = crate::rect_select::Rectangle::new([405., 305.], [401., 301.]).unwrap();
    assert!(
        crate::rect_select::found(&e.scene, &matrix, [800., 600.], small).contains(&id),
        "Crossing rectangle wholly inside the plate selects its source row"
    );
    let tiny = crate::rect_select::Rectangle::new([401., 301.], [405., 305.]).unwrap();
    assert!(
        !crate::rect_select::found(&e.scene, &matrix, [800., 600.], tiny).contains(&id)
    );
    let min = [
        points
            .iter()
            .map(|p| p[0])
            .fold(f64::INFINITY, f64::min)
            - 1.,
        points
            .iter()
            .map(|p| p[1])
            .fold(f64::INFINITY, f64::min)
            - 1.,
    ];
    let max = [
        points
            .iter()
            .map(|p| p[0])
            .fold(f64::NEG_INFINITY, f64::max)
            + 1.,
        points
            .iter()
            .map(|p| p[1])
            .fold(f64::NEG_INFINITY, f64::max)
            + 1.,
    ];
    assert_eq!(
        crate::rect_select::found(
            &e.scene,
            &matrix,
            [800., 600.],
            crate::rect_select::Rectangle::new(min, max).unwrap()
        ),
        [id]
    );
}

#[test]
fn invalid_scene_labels_do_not_consume_history_or_turn_screen_overlays_into_sources() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddText(plane([0.; 3])))
        .unwrap();
    e.apply(Action::Undo).unwrap();
    let source = Rc::new(example(TextPlacement::Screen {
        left: 0.,
        top: 0.,
    }));
    assert!(Prepared::new(source).is_err());
    let source = Rc::new(example(plane([f64::NAN, 0., 0.])));
    assert!(Prepared::new(source).is_err());
    assert!(e.apply(Action::Redo).is_ok());
    assert_eq!(e.scene.texts().len(), 1);
    assert!(e.scene.bounds().is_some());
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod scene_text_gpu;

pub mod text_fonts;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod scene_text_gpu;

pub mod text_fonts;

#[cfg(test)]
mod scene_label_tests;
```

### 5. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(colour_pixels(&mut renderer), before);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(colour_pixels(&mut renderer), before);
}

#[test]
fn actual_owned_text_plane_respects_depth_integer_picks_selection_cache_and_close() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::demo();
    scene.clear();
    let mesh = scene.insert(triangle(0.2)).unwrap();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    let gpu = renderer.stats();
    let mut source = example(TextPlacement::WorldPlane {
        world: [-0.1, 0.05, 0.6],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.08,
    });
    source.label.text = "Hi".into();
    let source = Rc::new(source);
    let original = serde_json::to_string(&*source).unwrap();
    let weak = Rc::downgrade(&source);
    let id = scene
        .insert_text(Prepared::new(source.clone()).unwrap())
        .unwrap();
    drop(source);
    renderer.set_scene(&scene, None);
    assert_eq!(
        colour_pixels(&mut renderer),
        before,
        "Actual fixed text behind the opaque solid is hidden"
    );
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        mesh.value()
    );
    scene
        .place_text(
            id,
            session_rust::Xform::translation(0., 0., -0.45),
        )
        .unwrap();
    renderer.set_scene(&scene, None);
    let front = colour_pixels(&mut renderer);
    assert_ne!(front, before);
    assert_eq!(renderer.text_error(), None);
    assert!(
        front
            .chunks_exact(4)
            .filter(|p| p[0] < 10 && p[1] < 10 && p[2] < 10)
            .count()
            > 20
    );
    let raster = renderer.text_scene_usage()[2];
    assert_eq!(
        raster, 1,
        "Placement reuses already rasterized original words"
    );
    assert_eq!(renderer.text_usage()[0], 1);
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap(),
        [id.value(), 0]
    );
    renderer
        .select_objects(&scene, &[id])
        .unwrap();
    let selected = colour_pixels(&mut renderer);
    assert!(
        selected
            .chunks_exact(4)
            .filter(|p| p[0] > 200 && p[1] > 200 && p[2] < 40)
            .count()
            > 20
    );
    assert_eq!(renderer.text_scene_usage()[2], raster);
    assert_eq!(renderer.text_usage()[0], 1);
    assert_eq!(&renderer.stats()[..2], &gpu[..2]);
    assert_eq!(
        serde_json::to_string(&*scene.texts()[0].prepared.source).unwrap(),
        original
    );
    Rc::make_mut(&mut scene.view).lock(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.06, -0.008], identity())
        .unwrap();
    assert!(
        finish(&mut renderer).is_none(),
        "The locked plate remains an occluder without a selectable id"
    );
    Rc::make_mut(&mut scene.view).hide(&[id], true);
    renderer.set_scene(&scene, None);
    renderer
        .request_pick([-0.06, 0.02], identity())
        .unwrap();
    assert_eq!(
        finish(&mut renderer).unwrap()[0],
        mesh.value()
    );
    scene.clear();
    renderer.set_scene(&scene, None);
    assert!(weak.upgrade().is_none());
    assert_eq!(renderer.text_scene_usage(), [0; 4]);
    assert_eq!(renderer.text_usage(), [0; 6]);
}

#[test]
fn actual_anchor_nameplate_and_billboard_follow_the_current_depth_and_plate_rules() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    for (placement, overlay) in [
        (
            TextPlacement::Anchor {
                world: [0., 0., 0.6],
                offset: [0.; 2],
            },
            false,
        ),
        (
            TextPlacement::Nameplate {
                world: [0., 0., 0.6],
                padding: [17., 4.],
                rounded: true,
            },
            true,
        ),
        (
            TextPlacement::WorldBillboard {
                world: [0., 0., 0.6],
                world_height: 0.08,
            },
            false,
        ),
    ] {
        let mut scene = Scene::demo();
        scene.clear();
        let mesh = scene.insert(triangle(0.2)).unwrap();
        let mut renderer = renderer(&scene);
        let before = colour_pixels(&mut renderer);
        let mut source = example(placement);
        source.label.text = "Hi".into();
        let id = scene
            .insert_text(Prepared::new(Rc::new(source)).unwrap())
            .unwrap();
        renderer.set_scene(&scene, None);
        let after = colour_pixels(&mut renderer);
        assert_eq!(
            after != before,
            overlay,
            "Only the nameplate overlays its own solid"
        );
        renderer
            .request_pick([0., 0.], identity())
            .unwrap();
        assert_eq!(
            finish(&mut renderer).unwrap()[0],
            if overlay {
                id.value()
            } else {
                mesh.value()
            }
        );
        assert_eq!(renderer.text_error(), None);
        let fonts = vec![
            crate::text::FONT_BYTES.to_vec(),
            crate::text::FALLBACK_BYTES.to_vec(),
            crate::text::SYMBOL_BYTES.to_vec(),
        ];
        assert!(
            renderer
                .replace_text_fonts(vec![vec![0; 8]])
                .is_err()
        );
        assert_eq!(colour_pixels(&mut renderer), after);
        renderer.replace_text_fonts(fonts).unwrap();
        assert_eq!(colour_pixels(&mut renderer), after);
        assert_eq!(renderer.text_usage()[5], 1);
    }
}

#[test]
fn actual_world_text_clip_uses_canvas_coordinates_in_the_small_integer_pick_window() {
    use crate::{
        scene_label::{Prepared, example},
        text::TextPlacement,
    };
    let mut scene = Scene::demo();
    scene.clear();
    let mut source = example(TextPlacement::WorldPlane {
        world: [-0.2, 0.1, 0.3],
        right: [1., 0., 0.],
        up: [0., 1., 0.],
        world_height: 0.2,
    });
    source.label.text = "Hi".into();
    source.label.clip = Some([140., 100., 160., 140.]);
    let id = scene
        .insert_text(Prepared::new(Rc::new(source)).unwrap())
        .unwrap();
    let mut renderer = renderer(&scene);
    let before = colour_pixels(&mut renderer);
    assert_eq!(renderer.text_error(), None);
    let mut clean = Scene::demo();
    clean.clear();
    let mut no_text = super::pick_gpu_tests::renderer(&clean);
    let baseline = colour_pixels(&mut no_text);
    let mut changed = 0;
    for (i, (p, b)) in before
        .chunks_exact(4)
        .zip(baseline.chunks_exact(4))
        .enumerate()
    {
        let (x, y) = (i % 320, i / 320);
        if x < 140 || x >= 160 || y < 100 || y >= 140 {
            assert_eq!(
                p, b,
                "Clipped text does not affect pixels outside the CSS clip"
            );
        } else {
            changed += usize::from(p != b);
        }
    }
    assert!(changed > 50);
    renderer
        .request_pick([-0.0625, 0.], identity())
        .unwrap();
    assert_eq!(finish(&mut renderer).unwrap()[0], id.value());
    renderer
        .request_pick([0.125, 0.], identity())
        .unwrap();
    assert!(finish(&mut renderer).is_none());
}
```

## Build & Run

![Recorded visual output for Lesson 10.4 (62o-labels)](../../screenshots/journey/62o-labels-browser.png)

```sh
npm --prefix ../session_tests run course -- dependencies 10.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 10.4
npm --prefix ../session_tests run course -- save 10.4
```

Continue with [11.1 · Coverage-based screen-space outlines](63-radius.md).
