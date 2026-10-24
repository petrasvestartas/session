# 14.4 · Solid questions, previews and one original commit

**Goal:** Reuse the cylinder’s existing center, radius and height questions for a cone. · **Steps:** 19 · **Source:** [app/walk/sheet.rs](../../../src/app/walk/sheet.rs) · [app/sheet_query.rs](../../../src/app/sheet_query.rs) · [app/stream.rs](../../../src/app/stream.rs) · [app/cplane.rs](../../../src/app/cplane.rs) · [app/coords.rs](../../../src/app/coords.rs) · [app/command/tool/shape.rs](../../../src/app/command/tool/shape.rs) · [app/command/tool/gather.rs](../../../src/app/command/tool/gather.rs) · [app/deform.rs](../../../src/app/deform.rs) (`src/app/cplane.rs,src/app/primitives.rs,src/app/curves.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Plane (O, u, v, w) + Polar (r, ∠θ) Input</text>
  <circle cx="112" cy="128" r="4" fill="#111111"/>
  <line x1="112" y1="128" x2="246" y2="128" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
  <line x1="112" y1="128" x2="112" y2="50" stroke="#16a34a" stroke-width="2" marker-end="url(#arrG)"/>
  <line x1="112" y1="128" x2="212" y2="68" stroke="#2563eb" stroke-width="2.2" marker-end="url(#arrB)"/>
  <path d="M 164 128 A 52 52 0 0 0 156 101" fill="none" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="176" y="114" font-family="monospace" font-size="10" font-weight="700" fill="#111111">r∠θ</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Rational NURBS Conic Arc + B-Rep Solid Primitives</text>
  <polyline points="424,140 494,52 564,140" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="4 3"/>
  <path d="M 424 140 Q 494 66 564 140" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <circle cx="494" cy="52" r="4.5" fill="#fef08a" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="494" y="44" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">w = cos(θ/2)</text>
  <ellipse cx="648" cy="72" rx="38" ry="12" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <path d="M 610 72 L 610 132 A 38 12 0 0 0 686 132 L 686 72" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Exact Curves + Closed B-Rep Box / Cylinder / Torus</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <path d="M 828 138 Q 868 62 908 138" fill="none" stroke="#dc2626" stroke-width="2.5"/>
  <polygon points="928,136 928,82 978,66 978,120" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="978,66 1028,82 1028,136 978,120" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-74-copy-current-solid-geometry-and-question-definitions"></a>1. Add the remaining regular kernel solids

**What this code does:** Build icosahedron and dodecahedron through the shared kernel solid helpers.

### 1. `src/primitive_icosahedron.rs`

Type the regular-solid wrapper using the existing common helpers.

Create the file and type:

```rust
use crate::primitive_shape as shape;
use crate::primitive_shape::{BREP_MESH, Part, Shape};
use session_rust::{Geometry, Mesh, Point};
use std::sync::OnceLock;

pub static SHAPE: Shape = Shape {
    name: "Icosahedron",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline,
    build,
};

static UNIT: OnceLock<Vec<Vec<[f64; 3]>>> = OnceLock::new(); // faces at radius 1

/// The kernel mesh of edge length `edge`.
fn make(edge: f64) -> Mesh {
    session_rust::Primitives::icosahedron(edge)
}

/// The faces at the radius.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    shape::polyhedron_outline(part, shape::unit_faces(&UNIT, make))
}

/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "icosahedron")
}
```

### 2. `src/lib.rs`

Register this completed kernel solid.

**Locate this existing block:**

```rust
pub mod primitive_octahedron;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod primitive_octahedron;

pub mod primitive_icosahedron;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/primitive_icosahedron.rs`

Supply the original kernel-solid validation fixture.

**Locate this existing block:**

```rust
    shape::polyhedron_of(part, option, make, "icosahedron")
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    shape::polyhedron_of(part, option, make, "icosahedron")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::Answer;
    use crate::primitive_shape::tests::{XZ, build, corners, p, plane};

    /// Every corner at the radius; the Brep option is a closed solid of 20 faces.
    #[test]
    fn the_corners_lie_at_the_radius() {
        let center = p(1.0, 2.0, 3.0);
        let answers = [
            Answer::Point(center.clone()),
            Answer::Number(300.0),
        ];
        let mesh = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Mesh").unwrap();
        assert!(matches!(mesh, Geometry::Mesh(_)));

        for q in corners(&mesh) {
            assert!((q.distance(&center, None) - 300.0).abs() < 1e-9 * 300.0);
        }

        let Geometry::BRep(brep) =
            build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 20);
        assert!(brep.is_solid());
    }
}
```

### 4. `src/primitive_dodecahedron.rs`

Type the regular-solid wrapper using the existing common helpers.

Create the file and type:

```rust
use crate::primitive_shape as shape;
use crate::primitive_shape::{BREP_MESH, Part, Shape};
use session_rust::{Geometry, Mesh, Point};
use std::sync::OnceLock;

pub static SHAPE: Shape = Shape {
    name: "Dodecahedron",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline,
    build,
};

static UNIT: OnceLock<Vec<Vec<[f64; 3]>>> = OnceLock::new(); // faces at radius 1

/// The kernel mesh of edge length `edge`.
fn make(edge: f64) -> Mesh {
    Mesh::create_dodecahedron(edge)
}

/// The faces at the radius.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    shape::polyhedron_outline(part, shape::unit_faces(&UNIT, make))
}

/// The kernel mesh scaled so its corners lie at the radius.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    shape::polyhedron_of(part, option, make, "dodecahedron")
}
```

### 5. `src/lib.rs`

Register this completed kernel solid.

**Locate this existing block:**

```rust
pub mod primitive_icosahedron;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod primitive_icosahedron;

pub mod primitive_dodecahedron;

#[cfg(target_arch = "wasm32")]
```

### 6. `src/primitive_dodecahedron.rs`

Supply the original kernel-solid validation fixture.

**Locate this existing block:**

```rust
    shape::polyhedron_of(part, option, make, "dodecahedron")
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    shape::polyhedron_of(part, option, make, "dodecahedron")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::Answer;
    use crate::primitive_shape::tests::{XZ, build, corners, p, plane};

    /// Every corner at the radius; the Brep option is a closed solid of 12 faces.
    #[test]
    fn the_corners_lie_at_the_radius() {
        let center = p(1.0, 2.0, 3.0);
        let answers = [
            Answer::Point(center.clone()),
            Answer::Number(300.0),
        ];
        let mesh = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Mesh").unwrap();
        assert!(matches!(mesh, Geometry::Mesh(_)));

        for q in corners(&mesh) {
            assert!((q.distance(&center, None) - 300.0).abs() < 1e-9 * 300.0);
        }

        let Geometry::BRep(brep) =
            build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 12);
        assert!(brep.is_solid());
    }
}
```

## <a id="checkpoint-74a-own-current-shape-questions-and-answer-state"></a>2. Own current shape questions and answer state

**What this code does:** Own current shape questions and answer state.

### 1. `src/primitive_draft.rs`

Own checked current primitive questions, representation choice and original answer state.

Create the file and type:

```rust
use crate::{
    cplane::CPlane,
    primitive_shape::{Answer, Ask, Kind, Part, Shape, frame_of},
};
use session_rust::{Geometry, Plane, Point, Vector};

#[derive(Clone)]
pub struct Draft {
    pub shape: &'static Shape,
    pub option: usize,
    pub answers: Vec<Answer>,
    pub part: Part,
    pub plane: CPlane,
}

impl Draft {
    pub fn new(
        shape: &'static Shape,
        plane: CPlane,
        option: usize,
    ) -> Result<Self, String> {
        if option >= shape.choices().count().max(1) {
            return Err("Unknown shape option".into());
        }
        let (x, y) = plane.axes();
        let part = Part::new(frame_of(
            &Plane::new(plane.origin(), x, y),
            &[],
        ));
        Ok(Self {
            shape,
            option,
            answers: Vec::new(),
            part,
            plane,
        })
    }
    pub fn choice(&self) -> &'static str {
        self.shape
            .choices()
            .nth(self.option)
            .unwrap_or("")
    }
    pub fn next(&self) -> Option<Ask> {
        (self.shape.ask)(&self.answers, &self.part, self.choice())
    }
    pub fn prompt(&self) -> String {
        let Some(ask) = self.next() else {
            return String::new();
        };
        let default = ask.default.map_or(String::new(), |value| {
            format!(" · Enter uses {value}")
        });
        format!(
            "{} · {}{default} · Esc cancels",
            self.shape.name, ask.prompt
        )
    }
}
```

### 2. `src/lib.rs`

Register the primitive component.

**Locate this existing block:**

```rust
pub mod primitive_dodecahedron;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod primitive_dodecahedron;

pub mod primitive_draft;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-74b-validate-a-new-answer-before-changing-its-original-state"></a>3. Validate a new answer before changing its state

**What this code does:** Validate a new answer before changing its original state.

### 1. `src/primitive_draft.rs`

Validate original answer candidates and changed frames without partially adopting them.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn set_plane(&mut self, plane: CPlane) -> Result<(), String> {
        let (x, y) = plane.axes();
        let frame = frame_of(
            &Plane::new(plane.origin(), x, y),
            &self.answers,
        );
        let part = (self.shape.read)(&frame, &self.answers, self.choice())?;
        self.plane = plane;
        self.part = part;
        Ok(())
    }
    fn with(&self, answer: Answer) -> Result<(Vec<Answer>, Part), String> {
        match &answer {
            Answer::Point(p) => {
                crate::tool_coords::checked([p[0], p[1], p[2]])
                    .map_err(str::to_owned)?;
            }
            Answer::Number(value) if !value.is_finite() || value.abs() > 1e12 => {
                return Err("Shape sizes must be finite and within ±1e12".into());
            }
            _ => {}
        }
        if self.next().is_none() {
            return Err("The shape already has all its answers".into());
        }
        let mut answers = self.answers.clone();
        answers.push(answer);
        if answers.len() > crate::tool::MAX_POINTS {
            return Err("Too many shape answers".into());
        }
        let (x, y) = self.plane.axes();
        let frame = frame_of(
            &Plane::new(self.plane.origin(), x, y),
            &answers,
        );
        let part = (self.shape.read)(&frame, &answers, self.choice())?;
        Ok((answers, part))
    }
}
```

## <a id="checkpoint-74c-build-only-after-the-last-shape-question"></a>4. Build only after the last shape question

**What this code does:** Build only after the last shape question.

### 1. `src/primitive_draft.rs`

Build a complete original primitive before adopting the completed answer set.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn accept(&mut self, answer: Answer) -> Result<Option<Geometry>, String> {
        let (answers, part) = self.with(answer)?;
        let complete = (self.shape.ask)(&answers, &part, self.choice()).is_none();
        let geometry = if complete {
            Some((self.shape.build)(&part, self.choice())?)
        } else {
            None
        };
        self.answers = answers;
        self.part = part;
        Ok(geometry)
    }
}
```

## <a id="checkpoint-74d-answer-shape-dimensions-with-atomic-typed-batches"></a>5. Answer shape dimensions with atomic typed batches

**What this code does:** Answer shape dimensions with atomic typed batches.

### 1. `src/primitive_draft.rs`

Resolve current shape choices, numbers, coordinates and Enter defaults atomically.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn text(
        &mut self,
        text: &str,
        cursor: Option<[f64; 3]>,
        fixed: Option<CPlane>,
    ) -> Result<Option<Geometry>, String> {
        // A cloned answer set makes a multiword input atomic, including its final build and any extra invalid words.
        let mut candidate = self.clone();
        let mut geometry = None;
        let words: Vec<_> = text.split_whitespace().collect();
        if words.is_empty() {
            let value = candidate
                .next()
                .and_then(|ask| ask.default)
                .ok_or_else(|| {
                    format!(
                        "{} needs the {}",
                        candidate.shape.name,
                        candidate
                            .next()
                            .map_or("next answer", |ask| ask.prompt)
                    )
                })?;
            geometry = candidate.accept(Answer::Number(value))?;
        } else {
            let mut index = 0;
            while index < words.len() {
                if geometry.is_some() {
                    return Err("The shape already has all its answers".into());
                }
                if let Some((option, count)) =
                    candidate.shape.option_in(&words[index..])
                {
                    if candidate.shape.upfront
                        && !candidate.answers.is_empty()
                        && candidate.option != option
                    {
                        return Err("Choose this option before the first point".into());
                    }
                    candidate.option = option;
                    candidate.set_plane(candidate.plane)?;
                    index += count;
                    continue;
                }
                let ask = candidate
                    .next()
                    .ok_or("The shape already has all its answers")?;
                let answer = match (ask.kind, words[index].parse::<f64>().ok()) {
                    (Kind::Size | Kind::Height, Some(value)) => Answer::Number(value),
                    _ => {
                        let previous = candidate
                            .answers
                            .iter()
                            .filter_map(|answer| match answer {
                                Answer::Point(p) => Some([p[0], p[1], p[2]]),
                                _ => None,
                            })
                            .collect::<Vec<_>>();
                        let points = crate::drawing_input::points(
                            words[index],
                            fixed,
                            candidate.plane,
                            &previous,
                            cursor,
                        )
                        .map_err(str::to_owned)?;
                        let p = points[0];
                        Answer::Point(Point::new(p[0], p[1], p[2]))
                    }
                };
                geometry = candidate.accept(answer)?;
                index += 1;
            }
        }
        *self = candidate;
        Ok(geometry)
    }
}
```

## <a id="checkpoint-74e-preview-solid-wires-and-measure-normal-axis-clicks"></a>6. Preview solid wires and measure normal axis clicks

**What this code does:** Preview solid wires and measure normal axis clicks.

### 1. `src/primitive_draft.rs`

Derive temporary current solid wires and finite height-axis click locations.

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn outline(&self, cursor: Option<[f64; 3]>) -> Vec<Vec<[f64; 3]>> {
        let pending = cursor
            .and_then(|p| {
                self.with(Answer::Point(Point::new(p[0], p[1], p[2])))
                    .ok()
            })
            .map(|(_, part)| part);
        (self.shape.outline)(pending.as_ref().unwrap_or(&self.part))
            .into_iter()
            .map(|wire| {
                wire.into_iter()
                    .map(|p| [p[0], p[1], p[2]])
                    .collect()
            })
            .collect()
    }
    pub fn on_axis(
        &self,
        camera: &crate::camera::Camera,
        screen: [f32; 2],
    ) -> Option<[f64; 3]> {
        let ray = camera.ray(screen)?;
        let axis = &self.part.frame;
        let direction = &ray.direction;
        let (a, b) = (
            direction.dot(direction),
            direction.dot(&axis.z),
        );
        let denominator = a - b * b;
        if denominator <= 0.01 * a {
            return None;
        }
        let offset = Vector::new(
            ray.origin[0] - axis.origin[0],
            ray.origin[1] - axis.origin[1],
            ray.origin[2] - axis.origin[2],
        );
        let height =
            (a * offset.dot(&axis.z) - b * offset.dot(direction)) / denominator;
        let p = axis.at(0., 0., height);
        crate::tool_coords::checked([p[0], p[1], p[2]]).ok()
    }
}
```

## <a id="checkpoint-74f-adopt-original-primitive-geometry-through-existing-rows"></a>7. Adopt primitive geometry through existing rows

**What this code does:** Adopt original primitive geometry through existing rows.

### 1. `src/primitive_geometry.rs`

Adopt original kernel geometry through existing original-source scene rows.

Create the file and type:

```rust
use crate::scene::{ObjectId, Scene};
use session_rust::Geometry;

pub fn insert(scene: &mut Scene, geometry: Geometry) -> Result<ObjectId, &'static str> {
    match geometry {
        Geometry::BRep(source) => scene.insert_cad(source),
        Geometry::Mesh(source) => scene.insert(crate::prepared::PreparedMesh::new(
            (*source).clone(),
        )?),
        Geometry::NurbsCurve(source) => {
            scene.insert_path(crate::chain::PreparedChain::curve(source)?)
        }
        _ => Err("This primitive geometry is not supported"),
    }
}
```

### 2. `src/lib.rs`

Register the primitive component.

**Locate this existing block:**

```rust
pub mod primitive_draft;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod primitive_draft;

pub mod primitive_geometry;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/tool.rs`

Carry an original completed primitive result beside existing point and placement results.

**Locate this existing block:**

```rust
    Polyline(crate::chain::PreparedChain),
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Polyline(crate::chain::PreparedChain),
    Geometry(session_rust::Geometry),
}
```

### 4. `src/editor.rs`

Commit completed original primitive geometry inside the existing history transaction.

**Locate this existing block:**

```rust
                }
                crate::tool::Built::Move(_) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                crate::tool::Built::Geometry(geometry) => {
                    crate::primitive_geometry::insert(scene, geometry).map(|_| ())
                }
                crate::tool::Built::Move(_) => {
```

## <a id="checkpoint-74g-keep-primitive-answers-beside-the-existing-drawing-draft"></a>8. Keep primitive answers beside the existing drawing draft

**What this code does:** Keep primitive answers beside the existing drawing draft.

### 1. `src/tool.rs`

Own mixed original primitive answers and multiple temporary wire sets with the current draft.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        )?;
        self.tool.build_on_plane(&world, self.frame)
    }
```

Replace it with:

```rust
        )?;
        self.tool.build_on_plane(&world, self.frame)
    }
    pub fn outlines(&self) -> Vec<Vec<[f64; 3]>> {
        if let Some(primitive) = &self.primitive {
            primitive.outline(self.cursor)
        } else {
            vec![self.outline()]
        }
    }
```

**Locate this block:**

```rust
            snapped: None,
            frame: crate::cplane::CPlane::Xy,
            plane_points: None,
        }
    }
    pub fn prompt(&self) -> String {
        let mut prompt = if let Some(points) = &self.plane_points {
            format!(
                "CPlane 3 Point · point {} of 3 · Esc cancels",
```

Replace it with:

```rust
            snapped: None,
            frame: crate::cplane::CPlane::Xy,
            plane_points: None,
            primitive: if let Shape::Primitive(shape) = shape {
                Some(
                    crate::primitive_draft::Draft::new(
                        shape,
                        crate::cplane::CPlane::Xy,
                        0,
                    )
                    .expect("Registered primitive option"),
                )
            } else {
                None
            },
        }
    }
    pub fn prompt(&self) -> String {
        if self.plane_points.is_none() {
            if let Some(primitive) = &self.primitive {
                return primitive.prompt();
            }
        }
        let mut prompt = if let Some(points) = &self.plane_points {
            format!(
                "CPlane 3 Point · point {} of 3 · Esc cancels",
```

**Locate this block:**

```rust
    pub snapped: Option<crate::snap::Kind>,
    pub frame: crate::cplane::CPlane,
    pub plane_points: Option<Vec<[f64; 3]>>,
}

impl Draft {
```

Replace it with:

```rust
    pub snapped: Option<crate::snap::Kind>,
    pub frame: crate::cplane::CPlane,
    pub plane_points: Option<Vec<[f64; 3]>>,
    pub primitive: Option<crate::primitive_draft::Draft>,
}

impl Draft {
```

**Locate this block:**

```rust
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        let mut buttons = vec![("Cancel".into(), "Escape".into())];
        if matches!(
            self,
```

Replace it with:

```rust
        }
    }
    fn buttons(&self) -> Vec<(String, String)> {
        if let Self::Primitive(shape) = self {
            return shape
                .options
                .iter()
                .map(|(label, line)| ((*label).into(), (*line).into()))
                .collect();
        }
        let mut buttons = vec![("Cancel".into(), "Escape".into())];
        if matches!(
            self,
```

**Locate this block:**

```rust
            Self::Line | Self::Move => 2,
            Self::Polyline => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
        }
    }
    fn sides(&self) -> Option<usize> {
```

Replace it with:

```rust
            Self::Line | Self::Move => 2,
            Self::Polyline => 0,
            Self::Rectangle | Self::Polygon { .. } => 2,
            Self::Primitive(_) => 0,
        }
    }
    fn sides(&self) -> Option<usize> {
```

**Locate this block:**

```rust
            Self::Polyline => "Polyline",
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
        }
    }
    fn count(&self) -> usize {
```

Replace it with:

```rust
            Self::Polyline => "Polyline",
            Self::Rectangle => "Polyline Rectangle",
            Self::Polygon { .. } => "Polyline Polygon",
            Self::Primitive(shape) => shape.name,
        }
    }
    fn count(&self) -> usize {
```

**Locate this block:**

```rust
    Polygon { sides: usize },
}

impl Tool for Shape {
```

Replace it with:

```rust
    Polygon { sides: usize },
    Primitive(&'static crate::primitive_shape::Shape),
}

impl Tool for Shape {
```

## <a id="checkpoint-74h-route-clicked-points-to-the-current-shape-question"></a>9. Route clicked points to the current shape question

**What this code does:** Route clicked points to the current shape question.

### 1. `src/editor.rs`

Connect current shape answers to the existing Editor transaction boundary. Connect the current solid input and original geometry contract.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        Ok(Change::Scene)
    }

    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

Replace it with:

```rust
        Ok(Change::Scene)
    }

    fn primitive_failure(&mut self, message: String) -> &'static str {
        self.tool_status = message;
        "Shape input invalid"
    }
    fn primitive_answer(
        &mut self,
        point: session_rust::Point,
    ) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_mut()
            .ok_or("No shape tool is running")?;
        let result = draft
            .primitive
            .as_mut()
            .ok_or("No shape tool is running")?
            .accept(crate::primitive_shape::Answer::Point(
                point.clone(),
            ));
        let geometry = result.map_err(|message| self.primitive_failure(message))?;
        let draft = self.draft.as_mut().unwrap();
        draft
            .points
            .push([point[0], point[1], point[2]]);
        draft.cursor = None;
        draft.snapped = None;
        if let Some(geometry) = geometry {
            return self.commit_tool(crate::tool::Built::Geometry(geometry));
        }
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
```

**Locate this block:**

```rust
        }
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built) = draft.submit(point)? else {
            self.tool_status = draft.prompt();
```

Replace it with:

```rust
        }
        if draft.primitive.is_some() {
            return self.primitive_answer(session_rust::Point::new(
                point[0], point[1], point[2],
            ));
        }
        crate::drawing_coords::set_plane(self.construction_plane);
        let Some(built) = draft.submit(point)? else {
            self.tool_status = draft.prompt();
```

## <a id="checkpoint-74i-route-typed-shape-input-and-preserve-detailed-errors"></a>10. Route typed shape input and preserve detailed errors

**What this code does:** Route typed shape input and preserve detailed errors.

### 1. `src/editor.rs`

Connect current shape answers to the existing Editor transaction boundary. Connect the current solid input and original geometry contract.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
        let draft = self
```

Replace it with:

```rust
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    fn primitive_text(&mut self, text: &str) -> Result<Change, &'static str> {
        let draft = self
            .draft
            .as_mut()
            .ok_or("No shape tool is running")?;
        let result = draft
            .primitive
            .as_mut()
            .ok_or("No shape tool is running")?
            .text(text, draft.cursor, self.construction_plane);
        let geometry = result.map_err(|message| self.primitive_failure(message))?;
        let draft = self.draft.as_mut().unwrap();
        draft.points = draft
            .primitive
            .as_ref()
            .unwrap()
            .answers
            .iter()
            .filter_map(|answer| match answer {
                crate::primitive_shape::Answer::Point(p) => Some([p[0], p[1], p[2]]),
                _ => None,
            })
            .collect();
        draft.cursor = None;
        draft.snapped = None;
        if let Some(geometry) = geometry {
            return self.commit_tool(crate::tool::Built::Geometry(geometry));
        }
        self.tool_status = draft.prompt();
        Ok(Change::View)
    }
    fn tool_text(&mut self, text: &str) -> Result<Change, &'static str> {
        if self
            .draft
            .as_ref()
            .is_some_and(|d| d.primitive.is_some() && d.plane_points.is_none())
        {
            return self.primitive_text(text);
        }
        let draft = self
```

**Locate this block:**

```rust
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count() == 0 {
            return Some(Ok(Action::CloseTool));
        }
```

Replace it with:

```rust
        }
        if draft.plane_points.is_none() && draft.primitive.is_some() {
            let words: Vec<_> = text.split_whitespace().collect();
            let primitive = draft.primitive.as_ref().unwrap();
            if text.trim().is_empty()
                || primitive.shape.option_in(&words).is_some()
                || words.first().is_some_and(|word| {
                    word.parse::<f64>().is_ok()
                        || crate::drawing_coords::parse_world(word).is_some()
                        || word.contains(',')
                        || word.contains('<')
                        || word.starts_with('@')
                })
            {
                return Some(Ok(Action::ToolText(text.to_owned())));
            }
            return None;
        }
        if text.trim().eq_ignore_ascii_case("Close") && draft.tool.count() == 0 {
            return Some(Ok(Action::CloseTool));
        }
```

## <a id="checkpoint-74j-synchronize-shape-frames-before-accepting-plane-changes"></a>11. Synchronize shape frames before accepting plane changes

**What this code does:** Synchronize shape frames before accepting plane changes.

### 1. `src/editor.rs`

Synchronize validated current primitive frames with fixed construction-plane changes.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                            .rotate_vector(session_rust::Vector::y_axis()),
                    )
                });
                self.draft = Some(draft);
```

Replace it with:

```rust
                            .rotate_vector(session_rust::Vector::y_axis()),
                    )
                });
                if let Some(primitive) = draft.primitive.as_mut() {
                    primitive
                        .set_plane(draft.frame)
                        .map_err(|message| {
                            self.tool_status = message;
                            "Shape input invalid"
                        })?;
                }
                self.draft = Some(draft);
```

**Locate this block:**

```rust
            Action::ConstructionPlane(plane) => {
                self.construction_plane = plane;
                crate::drawing_coords::set_plane(plane);
                self.plane_menu = false;
```

Replace it with:

```rust
            Action::ConstructionPlane(plane) => {
                let frame = plane.unwrap_or_else(|| {
                    crate::cplane::CPlane::view(
                        &self
                            .camera
                            .orientation
                            .rotate_vector(session_rust::Vector::y_axis()),
                    )
                });
                if let Some(primitive) = self
                    .draft
                    .as_mut()
                    .and_then(|draft| draft.primitive.as_mut())
                {
                    primitive
                        .set_plane(frame)
                        .map_err(|message| {
                            self.tool_status = message;
                            "Shape input invalid"
                        })?;
                }
                self.construction_plane = plane;
                crate::drawing_coords::set_plane(plane);
                self.plane_menu = false;
```

## <a id="checkpoint-74k-resolve-height-clicks-without-closing-an-open-shape-collector"></a>12. Resolve height clicks without closing an open shape collector

**What this code does:** Resolve height clicks without closing an open shape collector.

### 1. `src/editor.rs`

Connect the current solid input and original geometry contract. Keep current primitive question clicks separate from open Polyline closure.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                if commit {
                    if hit.is_some_and(|h| {
                        self.draft.as_ref().is_some_and(|d| {
                            d.tool.count() == 0
                                && d.points.len() >= 2
```

Replace it with:

```rust
                if commit {
                    if hit.is_some_and(|h| {
                        self.draft.as_ref().is_some_and(|d| {
                            d.primitive.is_none()
                                && d.tool.count() == 0
                                && d.points.len() >= 2
```

**Locate this block:**

```rust
            .draft
            .as_ref()
            .filter(|d| {
                d.tool.count() == 0
                    && d.points.len() >= 2
                    && self.snapping.objects
                    && self.snapping.modes & crate::snap::END != 0
```

Replace it with:

```rust
            .draft
            .as_ref()
            .filter(|d| {
                d.primitive.is_none()
                    && d.tool.count() == 0
                    && d.points.len() >= 2
                    && self.snapping.objects
                    && self.snapping.modes & crate::snap::END != 0
```

**Locate this block:**

```rust
    ) -> Option<crate::snap::Hit> {
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
```

Replace it with:

```rust
    ) -> Option<crate::snap::Hit> {
        if let Some(primitive) = self.draft.as_ref()?.primitive.as_ref() {
            if primitive
                .next()
                .is_some_and(|ask| ask.kind == crate::primitive_shape::Kind::Height)
            {
                return primitive
                    .on_axis(&self.camera, screen)
                    .map(|point| crate::snap::Hit {
                        point,
                        kind: crate::snap::Kind::Near,
                        owner: None,
                    });
            }
        }
        let ray = self.camera.ray(screen)?;
        let from = self.draft.as_ref()?.points.last().copied();
        let hit = if self.snapping.objects {
```

## <a id="checkpoint-74l-prepare-complete-solid-commands-before-replacing-a-draft"></a>13. Prepare complete solid commands before replacing a draft

**What this code does:** Prepare complete solid commands before replacing a draft.

### 1. `src/editor.rs`

Validate direct solid tool candidates before replacing current original input.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            self.plane_only = false;
        }
        match action {
            Action::ToolSides(sides) => {
```

Replace it with:

```rust
            self.plane_only = false;
        }
        match action {
            Action::PrimitiveBegin {
                shape,
                option,
                text,
            } => {
                let frame = self.construction_plane.unwrap_or_else(|| {
                    crate::cplane::CPlane::view(
                        &self
                            .camera
                            .orientation
                            .rotate_vector(session_rust::Vector::y_axis()),
                    )
                });
                let mut primitive =
                    crate::primitive_draft::Draft::new(shape, frame, option)
                        .map_err(|message| self.primitive_failure(message))?;
                let geometry = if text.trim().is_empty() {
                    None
                } else {
                    primitive
                        .text(&text, None, self.construction_plane)
                        .map_err(|message| self.primitive_failure(message))?
                };
                self.apply(Action::BeginTool(
                    crate::tool::Shape::Primitive(shape),
                ))?;
                let draft = self.draft.as_mut().unwrap();
                draft.primitive = Some(primitive);
                draft.points = draft
                    .primitive
                    .as_ref()
                    .unwrap()
                    .answers
                    .iter()
                    .filter_map(|answer| match answer {
                        crate::primitive_shape::Answer::Point(p) => {
                            Some([p[0], p[1], p[2]])
                        }
                        _ => None,
                    })
                    .collect();
                if let Some(geometry) = geometry {
                    return self.commit_tool(crate::tool::Built::Geometry(geometry));
                }
                self.tool_status = draft.prompt();
                return Ok(Change::View);
            }
            Action::ToolSides(sides) => {
```

**Locate this block:**

```rust
                | Action::PlaneMenu
                | Action::Plane3
                | Action::ToolSides(_)
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
```

Replace it with:

```rust
                | Action::PlaneMenu
                | Action::Plane3
                | Action::ToolSides(_)
                | Action::PrimitiveBegin { .. }
                | Action::ToolPoint(_)
                | Action::ToolScreen { .. }
                | Action::ToolHover(_)
```

**Locate this block:**

```rust
    StreamRefresh,
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>),
    PlaneMenu,
```

Replace it with:

```rust
    StreamRefresh,
    PrimitiveBegin {
        shape: &'static crate::primitive_shape::Shape,
        option: usize,
        text: String,
    },
    ToolSides(usize),
    ConstructionPlane(Option<crate::cplane::CPlane>),
    PlaneMenu,
```

## <a id="checkpoint-74m-register-every-current-solid-name-and-representation"></a>14. Register every current solid name and representation

**What this code does:** Register every current solid name and representation.

### 1. `src/primitive_registry.rs`

Register current solid names, representation options and the existing Box alias.

Create the file and type:

```rust
use crate::{editor::Action, primitive_shape::Shape};

pub const ENTRIES: &[(&str, &Shape, usize)] = &[
    ("Box", &crate::primitive_box::SHAPE, 0),
    ("Box Brep", &crate::primitive_box::SHAPE, 0),
    ("Box Mesh", &crate::primitive_box::SHAPE, 1),
    (
        "Pyramid",
        &crate::primitive_pyramid::SHAPE,
        0,
    ),
    (
        "Pyramid Brep",
        &crate::primitive_pyramid::SHAPE,
        0,
    ),
    (
        "Pyramid Mesh",
        &crate::primitive_pyramid::SHAPE,
        1,
    ),
    (
        "Block With Hole",
        &crate::primitive_block_with_hole::SHAPE,
        0,
    ),
    (
        "Block With Hole Brep",
        &crate::primitive_block_with_hole::SHAPE,
        0,
    ),
    (
        "Block With Hole Mesh",
        &crate::primitive_block_with_hole::SHAPE,
        1,
    ),
    (
        "Tetrahedron",
        &crate::primitive_tetrahedron::SHAPE,
        0,
    ),
    (
        "Tetrahedron Brep",
        &crate::primitive_tetrahedron::SHAPE,
        0,
    ),
    (
        "Tetrahedron Mesh",
        &crate::primitive_tetrahedron::SHAPE,
        1,
    ),
    (
        "Octahedron",
        &crate::primitive_octahedron::SHAPE,
        0,
    ),
    (
        "Octahedron Brep",
        &crate::primitive_octahedron::SHAPE,
        0,
    ),
    (
        "Octahedron Mesh",
        &crate::primitive_octahedron::SHAPE,
        1,
    ),
    (
        "Icosahedron",
        &crate::primitive_icosahedron::SHAPE,
        0,
    ),
    (
        "Icosahedron Brep",
        &crate::primitive_icosahedron::SHAPE,
        0,
    ),
    (
        "Icosahedron Mesh",
        &crate::primitive_icosahedron::SHAPE,
        1,
    ),
    (
        "Dodecahedron",
        &crate::primitive_dodecahedron::SHAPE,
        0,
    ),
    (
        "Dodecahedron Brep",
        &crate::primitive_dodecahedron::SHAPE,
        0,
    ),
    (
        "Dodecahedron Mesh",
        &crate::primitive_dodecahedron::SHAPE,
        1,
    ),
    ("bo", &crate::primitive_box::SHAPE, 0),
];

pub fn command(name: &str, text: &str) -> Option<Action> {
    ENTRIES
        .iter()
        .find(|(key, _, _)| *key == name)
        .map(|(_, shape, option)| Action::PrimitiveBegin {
            shape,
            option: *option,
            text: text.to_owned(),
        })
}
```

### 2. `src/lib.rs`

Register the primitive component.

**Locate this existing block:**

```rust
pub mod primitive_geometry;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod primitive_geometry;

pub mod primitive_registry;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/commands.rs`

Route every current solid name and representation to its maintained original builder.

**Locate this existing block:**

```rust
        .chain(ACTIONS.iter().map(|(name, _)| *name))
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if name == "CPlane" && words.len() > count {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .chain(ACTIONS.iter().map(|(name, _)| *name))
        .chain(
            crate::primitive_registry::ENTRIES
                .iter()
                .map(|(name, _, _)| *name),
        )
        .collect()
}

// Choose the longest known name so Open Replace cannot become Open with an unwanted argument.
fn matched(words: &[&str]) -> Option<(&'static str, usize)> {
    names()
        .into_iter()
        .filter_map(|name| {
            let compact = name
                .split_whitespace()
                .collect::<String>()
                .to_ascii_lowercase();
            let mut rest = compact.as_str();
            for (index, word) in words.iter().enumerate() {
                let word = word.to_ascii_lowercase();
                rest = rest.strip_prefix(&word)?;
                if rest.is_empty() {
                    return Some((name, index + 1, compact.len()));
                }
            }
            None
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(name, count, _)| (name, count))
}

pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    matched(&words).map_or_else(
        || line.trim().to_owned(),
        |(name, count)| {
            std::iter::once(name)
                .chain(words[count..].iter().copied())
                .collect::<Vec<_>>()
                .join(" ")
        },
    )
}

pub fn parse(line: &str) -> Result<Command, String> {
    let words: Vec<_> = line.split_whitespace().collect();
    let (name, count) = matched(&words).ok_or("Unknown command. Type Help.")?;
    if let Some(action) =
        crate::primitive_registry::command(name, &words[count..].join(" "))
    {
        return Ok(Command::Editor(action));
    }
    if name == "CPlane" && words.len() > count {
```

## <a id="checkpoint-74n-draw-temporary-solid-wire-sets-and-report-current-input-failures"></a>15. Draw temporary solid wire sets and report current input failures

**What this code does:** Draw temporary solid wire sets and report current input failures.

### 1. `src/tool_preview.rs`

Preview independent temporary solid wire sets without connecting unrelated boundaries.

**Locate this existing block:**

```rust
        if let Some(draft) = draft {
            for pair in draft.outline().windows(2) {
                if pair[0] != pair[1] {
                    strokes.push(crate::stroke::Stroke {
                        start: pair[0].map(|v| v as f32),
                        end: pair[1].map(|v| v as f32),
                        width: 2.,
                        colour: yellow,
                    });
                }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        if let Some(draft) = draft {
            for wire in draft.outlines() {
                for pair in wire.windows(2) {
                    if pair[0] != pair[1] {
                        strokes.push(crate::stroke::Stroke {
                            start: pair[0].map(|v| v as f32),
                            end: pair[1].map(|v| v as f32),
                            width: 2.,
                            colour: yellow,
                        });
                    }
                }
```

### 2. `src/browser.rs`

Show current detailed shape input failures without changing the existing browser command lifecycle.

**Locate this existing block:**

```rust
                    Err(error) => {
                        report(&error);
                        if event.type_() == "viewer-file" {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    Err(error) => {
                        let error = if error == "Shape input invalid" {
                            editor.tool_status.as_str()
                        } else {
                            error.as_str()
                        };
                        report(error);
                        if event.type_() == "viewer-file" {
```

## <a id="checkpoint-74o-keep-quick-mouse-drift-as-a-click-using-current-event-time"></a>16. Keep quick mouse drift as a click using current event time

**What this code does:** Keep quick mouse drift as a click using current event time.

### 1. `src/gesture.rs`

Use current430accda quick-click drift thresholds and explicit gesture timestamps. Copy current quick-wobble and owned event-clock fixtures.

**Locate this existing block:**

```rust
struct Drag {
    id: i32,
    button: i16,
    start: [f64; 2],
    last: [f64; 2],
    moved: bool,
    pan: bool,
}
#[derive(Default)]
pub struct Gesture {
    active: Option<Drag>,
    repeat: bool,
}
impl Gesture {
    pub fn press(&mut self, id: i32, button: i16, position: [f64; 2]) -> bool {
        self.press_with(id, button, position, false)
    }
    pub fn press_with(
        &mut self,
        id: i32,
        button: i16,
        position: [f64; 2],
        pan: bool,
    ) -> bool {
        if self.active.is_some()
            || !matches!(button, 0 | 1 | 2)
            || position.iter().any(|v| !v.is_finite())
        {
            return false;
        }
        self.repeat = false;
        self.active = Some(Drag {
            id,
            button,
            start: position,
            last: position,
            moved: false,
            pan: pan || button == 1,
        });
        true
    }
    pub fn move_to(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        let drag = self.active.as_mut().filter(|d| d.id == id)?;
        if position.iter().any(|v| !v.is_finite()) {
            return None;
        }
        let was = drag.moved;
        drag.moved |= (position[0] - drag.start[0])
            .abs()
            .max((position[1] - drag.start[1]).abs())
            > 4.;
        let from = if was {
            drag.last
        } else {
            drag.start
        };
        let delta = std::array::from_fn(|i| position[i] - from[i]);
        drag.last = position;
        if !drag.moved {
            return None;
        }
        if drag.button == 0 {
            Some(if was {
                Motion::Drag(position)
            } else {
                Motion::Probe {
                    from: drag.start,
                    to: position,
                }
            })
        } else if delta == [0.; 2] {
            None
        } else {
            Some(if drag.pan {
                Motion::Pan(delta)
            } else {
                Motion::Orbit(delta)
            })
        }
    }
    pub fn release(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        if self.active.as_ref()?.id != id {
            return None;
        }
        let motion = self.move_to(id, position);
        let drag = self.active.take()?;
        if position.iter().any(|v| !v.is_finite()) {
            return None;
        }
        if drag.button != 0 {
            self.repeat = drag.button == 2 && !drag.moved;
            return motion;
        }
        Some(if drag.moved {
            if matches!(motion, Some(Motion::Probe { .. })) {
                motion.unwrap()
            } else {
                Motion::Finish(position)
            }
        } else {
            Motion::Pick(position)
        })
    }
    pub fn take_repeat(&mut self) -> bool {
        std::mem::take(&mut self.repeat)
    }
    pub fn cancel(&mut self) {
        self.active = None;
        self.repeat = false;
    }
    pub fn cancel_pointer(&mut self, id: i32) {
        if self
            .active
            .as_ref()
            .is_some_and(|d| d.id == id)
        {
            self.cancel();
        }
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
struct Drag {
    started: f64,
    id: i32,
    button: i16,
    start: [f64; 2],
    last: [f64; 2],
    moved: bool,
    pan: bool,
}
#[derive(Default)]
pub struct Gesture {
    active: Option<Drag>,
    repeat: bool,
}
impl Gesture {
    pub fn press(&mut self, id: i32, button: i16, position: [f64; 2]) -> bool {
        self.press_with(id, button, position, false)
    }
    pub fn press_with(
        &mut self,
        id: i32,
        button: i16,
        position: [f64; 2],
        pan: bool,
    ) -> bool {
        self.press_at(id, button, position, pan, 0.)
    }
    pub fn press_at(
        &mut self,
        id: i32,
        button: i16,
        position: [f64; 2],
        pan: bool,
        now: f64,
    ) -> bool {
        if !now.is_finite()
            || now < 0.
            || self.active.is_some()
            || !matches!(button, 0 | 1 | 2)
            || position.iter().any(|v| !v.is_finite())
        {
            return false;
        }
        self.repeat = false;
        self.active = Some(Drag {
            started: now,
            id,
            button,
            start: position,
            last: position,
            moved: false,
            pan: pan || button == 1,
        });
        true
    }
    pub fn move_to(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        self.move_at(id, position, 200.)
    }
    pub fn move_at(&mut self, id: i32, position: [f64; 2], now: f64) -> Option<Motion> {
        let drag = self.active.as_mut().filter(|d| d.id == id)?;
        if !now.is_finite()
            || now < drag.started
            || position.iter().any(|v| !v.is_finite())
        {
            return None;
        }
        let moved = (position[0] - drag.start[0])
            .abs()
            .max((position[1] - drag.start[1]).abs());
        let was = drag.moved;
        drag.moved |= if drag.button == 0 {
            drags(moved, now - drag.started)
        } else {
            moved > 4.
        };
        let from = if was {
            drag.last
        } else {
            drag.start
        };
        let delta = std::array::from_fn(|i| position[i] - from[i]);
        drag.last = position;
        if !drag.moved {
            return None;
        }
        if drag.button == 0 {
            Some(if was {
                Motion::Drag(position)
            } else {
                Motion::Probe {
                    from: drag.start,
                    to: position,
                }
            })
        } else if delta == [0.; 2] {
            None
        } else {
            Some(if drag.pan {
                Motion::Pan(delta)
            } else {
                Motion::Orbit(delta)
            })
        }
    }
    pub fn release(&mut self, id: i32, position: [f64; 2]) -> Option<Motion> {
        self.release_at(id, position, 200.)
    }
    pub fn release_at(
        &mut self,
        id: i32,
        position: [f64; 2],
        now: f64,
    ) -> Option<Motion> {
        if self.active.as_ref()?.id != id {
            return None;
        }
        let motion = self.move_at(id, position, now);
        let drag = self.active.take()?;
        if position.iter().any(|v| !v.is_finite()) {
            return None;
        }
        if drag.button != 0 {
            self.repeat = drag.button == 2 && !drag.moved;
            return motion;
        }
        Some(if drag.moved {
            if matches!(motion, Some(Motion::Probe { .. })) {
                motion.unwrap()
            } else {
                Motion::Finish(position)
            }
        } else {
            Motion::Pick(position)
        })
    }
    pub fn take_repeat(&mut self) -> bool {
        std::mem::take(&mut self.repeat)
    }
    pub fn cancel(&mut self) {
        self.active = None;
        self.repeat = false;
    }
    pub fn cancel_pointer(&mut self, id: i32) {
        if self
            .active
            .as_ref()
            .is_some_and(|d| d.id == id)
        {
            self.cancel();
        }
    }
}

// A quick press may wobble without moving geometry; a longer held press uses the narrower threshold for intentional dragging.
pub fn drags(moved: f64, held: f64) -> bool {
    moved > 10. || (moved > 4. && held >= 200.)
}

#[cfg(test)]
mod drift_tests {
    use super::*;
    #[test]
    fn a_quick_wobble_is_a_click() {
        assert!(!drags(5., 60.));
        assert!(!drags(9., 150.));
        assert!(drags(12., 40.));
        assert!(drags(5., 250.));
        assert!(!drags(3., 900.));
    }
    #[test]
    fn quick_and_held_mouse_events_keep_the_original_click_or_drag_owner() {
        let mut input = Gesture::default();
        assert!(input.press_at(1, 0, [10., 20.], false, 1000.));
        assert_eq!(input.move_at(1, [15., 20.], 1060.), None);
        assert_eq!(
            input.release_at(1, [19., 20.], 1150.),
            Some(Motion::Pick([19., 20.]))
        );
        assert!(input.press_at(1, 0, [10., 20.], false, 2000.));
        assert_eq!(
            input.move_at(1, [15., 20.], 2250.),
            Some(Motion::Probe {
                from: [10., 20.],
                to: [15., 20.]
            })
        );
        assert_eq!(
            input.release_at(1, [15., 20.], 2260.),
            Some(Motion::Finish([15., 20.]))
        );
        assert!(input.press_at(1, 0, [0., 0.], false, 3000.));
        assert_eq!(
            input.release_at(1, [12., 0.], 3040.),
            Some(Motion::Probe {
                from: [0., 0.],
                to: [12., 0.]
            })
        );
    }
    #[test]
    fn invalid_timestamps_and_other_pointers_cannot_begin_a_drag() {
        let mut input = Gesture::default();
        assert!(!input.press_at(1, 0, [0., 0.], false, f64::NAN));
        assert!(input.press_at(1, 0, [0., 0.], false, 1000.));
        assert_eq!(input.move_at(2, [20., 0.], 1200.), None);
        assert_eq!(input.move_at(1, [20., 0.], 900.), None);
        assert_eq!(input.move_at(1, [20., 0.], f64::NAN), None);
        assert_eq!(
            input.release_at(1, [3., 0.], 1900.),
            Some(Motion::Pick([3., 0.]))
        );
    }
```

### 2. `src/browser.rs`

Supply actual browser pointer event times to current click and drag ownership.

**Locate this existing block:**

```rust
            if pointer.is_primary()
                && gesture.press_with(
                    id,
                    pointer.button(),
                    position,
                    pointer.ctrl_key() || pointer.meta_key(),
                )
            {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_to(id, position),
        "pointerup" => {
            let motion = gesture.release(id, position);
            let _ = canvas.release_pointer_capture(id);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            if pointer.is_primary()
                && gesture.press_at(
                    id,
                    pointer.button(),
                    position,
                    pointer.ctrl_key() || pointer.meta_key(),
                    pointer.time_stamp(),
                )
            {
                event.prevent_default();
                let options = web_sys::FocusOptions::new();
                options.set_prevent_scroll(true);
                if canvas.focus_with_options(&options).is_err()
                    || canvas.set_pointer_capture(id).is_err()
                {
                    gesture.cancel();
                }
            }
            None
        }
        "pointermove" => gesture.move_at(id, position, pointer.time_stamp()),
        "pointerup" => {
            let motion = gesture.release_at(id, position, pointer.time_stamp());
            let _ = canvas.release_pointer_capture(id);
```

## <a id="checkpoint-74p-exercise-current-solid-input-and-original-persistence"></a>17. Exercise current solid input and persistence

**What this code does:** Exercise current solid input and original persistence.

### 1. `src/primitive_tests.rs`

Copy current integrated solid input, original persistence, axis clicks, temporary preview and atomic error fixtures.

Create the file and type:

```rust
use crate::{
    cplane::CPlane,
    editor::{Action, Editor},
    primitive_draft::Draft,
    primitive_shape::{Answer, Kind},
};
use session_rust::Point;
fn editor() -> Editor {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e
}
fn command(e: &mut Editor, line: &str) -> Result<(), String> {
    let action = if let Some(action) = e.tool_line(line) {
        action.map_err(str::to_owned)?
    } else {
        let crate::commands::Command::Editor(action) = crate::commands::parse(line)?
        else {
            return Err("Editor command".into());
        };
        action
    };
    e.apply(action).map_err(|s| {
        if s == "Shape input invalid" {
            e.tool_status.clone()
        } else {
            s.into()
        }
    })?;
    Ok(())
}
#[test]
fn direct_maintained_solid_builders_create_original_geometry_as_one_transaction() {
    let mut e = editor();
    for (line, faces) in [
        ("Box 0,0,0 2 3 4", 6),
        ("Pyramid 4,0,0 2 3", 5),
        ("Block With Hole 8,0,0 4 3 2 0.4", 7),
        ("Tetrahedron 12,0,0 2", 4),
        ("Octahedron 16,0,0 2", 8),
        ("Icosahedron 20,0,0 2", 20),
        ("Dodecahedron 24,0,0 2", 12),
    ] {
        let count = e.scene.objects().len();
        command(&mut e, line).unwrap();
        assert!(e.draft.is_none());
        let cad = e
            .scene
            .objects()
            .last()
            .unwrap()
            .cad
            .as_ref()
            .unwrap();
        assert_eq!(cad.source.face_count(), faces);
        command(&mut e, "Undo").unwrap();
        assert_eq!(e.scene.objects().len(), count);
        command(&mut e, "Redo").unwrap();
        assert_eq!(e.scene.objects().len(), count + 1);
    }
    let sources = e
        .scene
        .objects()
        .iter()
        .map(|row| row.cad.as_ref().unwrap().source.to_proto())
        .collect::<Vec<_>>();
    let bytes = crate::scene_archive::save(&e.scene).unwrap();
    e.apply(Action::Close).unwrap();
    crate::scene_document::open(&mut e.scene, &bytes).unwrap();
    assert_eq!(
        e.scene
            .objects()
            .iter()
            .map(|row| row.cad.as_ref().unwrap().source.to_proto())
            .collect::<Vec<_>>(),
        sources
    );
}
#[test]
fn typed_defaults_and_brep_mesh_choices_preserve_the_current_answers() {
    let mut e = editor();
    command(&mut e, "Box").unwrap();
    command(&mut e, "0,0,0").unwrap();
    assert!(command(&mut e, "").is_err());
    command(&mut e, "2").unwrap();
    command(&mut e, "Mesh").unwrap();
    assert_eq!(
        e.draft
            .as_ref()
            .unwrap()
            .primitive
            .as_ref()
            .unwrap()
            .answers
            .len(),
        2
    );
    command(&mut e, "").unwrap();
    assert!(e.scene.objects().is_empty());
    command(&mut e, "").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
    assert!(e.scene.objects()[0].cad.is_none());
    let bounds = e.scene.bounds().unwrap();
    assert_eq!(bounds.min, [-1., -1., 0.]);
    assert_eq!(bounds.max, [1., 1., 2.]);
    command(&mut e, "Undo").unwrap();
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
}
#[test]
fn clicked_corners_and_signed_heights_follow_the_current_fixed_plane() {
    let mut e = editor();
    command(&mut e, "CPlane XZ").unwrap();
    command(&mut e, "Box").unwrap();
    e.apply(Action::ToolPoint([1., 2., 3.]))
        .unwrap();
    e.apply(Action::ToolPoint([2., 2., 5.]))
        .unwrap();
    assert!(e.scene.objects().is_empty());
    command(&mut e, "-3").unwrap();
    let source = &e.scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .source;
    let bounds = crate::primitive_shape::tests::bounds(&source.vertex_points());
    assert_eq!(bounds, [[0., 2., 1.], [2., 5., 5.]]);
}
#[test]
fn invalid_direct_and_batched_shape_input_preserves_draft_points_and_redo() {
    let mut e = editor();
    command(&mut e, "Box 0,0,0 2 3 4").unwrap();
    command(&mut e, "Undo").unwrap();
    command(&mut e, "Line").unwrap();
    command(&mut e, "w0,0,0").unwrap();
    assert!(command(&mut e, "Box 0,0,0 0 3 4").is_err());
    assert_eq!(e.draft.as_ref().unwrap().tool.name(), "Line");
    assert_eq!(
        e.draft.as_ref().unwrap().points,
        vec![[0.; 3]]
    );
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Box").unwrap();
    for text in [
        "0,0,0 2 3 4 extra",
        "0,0,0 2 3 4 Mesh",
        "0,0,0 NaN",
        "0,0,0 2 0",
    ] {
        assert!(command(&mut e, text).is_err(), "{text}");
        assert!(
            e.draft
                .as_ref()
                .unwrap()
                .primitive
                .as_ref()
                .unwrap()
                .answers
                .is_empty()
        );
        assert!(e.scene.objects().is_empty());
    }
    command(&mut e, "Escape").unwrap();
    command(&mut e, "Redo").unwrap();
    assert_eq!(e.scene.objects().len(), 1);
}
#[test]
fn height_clicks_use_the_normal_axis_and_refuse_an_axis_aligned_view() {
    let mut draft = Draft::new(&crate::primitive_box::SHAPE, CPlane::Xy, 0).unwrap();
    draft
        .accept(Answer::Point(Point::new(0., 0., 0.)))
        .unwrap();
    draft.accept(Answer::Number(2.)).unwrap();
    draft.accept(Answer::Number(3.)).unwrap();
    assert_eq!(draft.next().unwrap().kind, Kind::Height);
    let mut camera = crate::camera::Camera::default();
    assert!(draft.on_axis(&camera, [0., 0.]).is_none());
    camera.orbit(0., std::f64::consts::FRAC_PI_4);
    let point = draft.on_axis(&camera, [0., 0.2]).unwrap();
    assert!(point[0].abs() < 1e-12 && point[1].abs() < 1e-12);
    assert!(point[2].is_finite());
}
#[test]
fn pending_solid_wires_and_cancel_never_create_saved_geometry() {
    let mut e = editor();
    command(&mut e, "Box 0,0,0 2 3").unwrap();
    let draft = e.draft.as_ref().unwrap();
    assert_eq!(
        draft
            .primitive
            .as_ref()
            .unwrap()
            .answers
            .len(),
        3
    );
    assert!(e.scene.objects().is_empty());
    assert_eq!(draft.outlines().len(), 1);
    e.apply(Action::ToolHover(Some([0., 0., 4.])))
        .unwrap();
    assert_eq!(e.draft.as_ref().unwrap().outlines().len(), 6);
    assert!(e.scene.objects().is_empty());
    command(&mut e, "Escape").unwrap();
    assert!(e.draft.is_none());
    assert!(e.scene.objects().is_empty());
}
```

### 2. `src/lib.rs`

Register the primitive component.

**Locate this existing block:**

```rust
pub mod primitive_registry;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod primitive_registry;

#[cfg(test)]
mod primitive_tests;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-74za-build-original-cylinders-with-signed-height"></a>18. Build cylinders with signed height

**What this code does:** Build cylinders with positive or negative height using the solid frame and radius checks already taught.

### 1. `src/round_cylinder.rs`

Type the whole original solid builder using the existing frame and question helpers. Supply only the unchanged original representation and topology fixture.

Create the file and type:

```rust
use crate::primitive_shape as shape;
use crate::primitive_shape::{
    Answer, Ask, BREP_MESH, Frame, Part, Shape, nonzero, positive,
};
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Cylinder",
    options: BREP_MESH,
    upfront: false,
    ask,
    read,
    outline,
    build,
};

/// Base center, radius, height: the questions of a cylinder or cone.
pub fn ask(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Base center")),
        1 => Some(Ask::size("Radius")),
        2 => Some(Ask::height("Height")),
        _ => None,
    }
}

/// Radius in the plane and signed height.
pub fn read(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes
            .push(positive(frame.size(radius), "The radius")?);
    }

    if let Some(height) = answers.get(2) {
        part.sizes
            .push(nonzero(frame.height(height), "The height")?);
    }

    Ok(part)
}

/// The base ring, then the top ring and four sides.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [radius] => vec![frame.ring(radius, 0.0)],
        [radius, height] => {
            let mut wires = vec![
                frame.ring(radius, 0.0),
                frame.ring(radius, height),
            ];

            for (u, v) in [
                (1.0, 0.0),
                (0.0, 1.0),
                (-1.0, 0.0),
                (0.0, -1.0),
            ] {
                wires.push(vec![
                    frame.at(u * radius, v * radius, 0.0),
                    frame.at(u * radius, v * radius, height),
                ]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel cylinder on the base, upside down for a negative height.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Cylinder needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    let brep = BRep::create_cylinder(radius, height);
    Ok(shape::solid(
        brep,
        option,
        &frame.to_xform(),
        "cylinder",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::tests::{
        XY, XZ, bounds, build, corners, near, p, plane,
    };

    /// Three faces, 2r × 2r × h; a negative height goes below; the Front plane stands it along −Y.
    #[test]
    fn the_cylinder_stands_on_its_base() {
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(600.0),
        ];
        let Geometry::BRep(brep) =
            build(&SHAPE, &plane(XY.0, XY.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 3);
        assert!(brep.is_solid());
        let mesh = build(&SHAPE, &plane(XY.0, XY.1), &answers, "Mesh").unwrap();
        let [low, high] = bounds(&corners(&mesh));
        assert!((high[2] - 600.0).abs() < 1e-9 && low[2].abs() < 1e-9);
        assert!((high[0] - 200.0).abs() < 1e-6 && (low[0] + 200.0).abs() < 1e-6);
        let down = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(-600.0),
        ];
        let low = build(&SHAPE, &plane(XY.0, XY.1), &down, "Mesh").unwrap();
        let [bottom, top] = bounds(&corners(&low));
        assert!((bottom[2] + 600.0).abs() < 1e-9 && top[2].abs() < 1e-9);
        let front = build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Mesh").unwrap();
        let [a, b] = bounds(&corners(&front));
        assert!(near(
            [[0.0, a[1], 0.0], [0.0, b[1], 0.0]],
            [[0.0, -600.0, 0.0], [0.0, 0.0, 0.0]]
        ));
        let zero = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(-1.0),
        ];
        assert!(build(&SHAPE, &plane(XY.0, XY.1), &zero, "Brep").is_err());
    }
}
```

### 2. `src/lib.rs`

Register this shared original solid builder.

**Locate this existing block:**

```rust
mod primitive_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod primitive_tests;

pub mod round_cylinder;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-74zb-build-original-cones-and-spheres"></a>19. Build cones and spheres

**What this code does:** Reuse the cylinder’s existing center, radius and height questions for a cone.

### 1. `src/round_cone.rs`

Type the whole original solid builder using the existing frame and question helpers. Supply only the unchanged original representation and topology fixture.

Create the file and type:

```rust
use crate::primitive_shape as shape;
use crate::primitive_shape::{BREP_MESH, Part, Shape};
use crate::round_cylinder as cylinder;
use session_rust::{BRep, Geometry, Point};

pub static SHAPE: Shape = Shape {
    name: "Cone",
    options: BREP_MESH,
    upfront: false,
    ask: cylinder::ask,
    read: cylinder::read,
    outline,
    build,
};

/// The base ring, then four lines to the apex.
fn outline(part: &Part) -> Vec<Vec<Point>> {
    let frame = &part.frame;

    match part.sizes[..] {
        [radius] => vec![frame.ring(radius, 0.0)],
        [radius, height] => {
            let apex = frame.at(0.0, 0.0, height);
            let mut wires = vec![frame.ring(radius, 0.0)];

            for (u, v) in [
                (1.0, 0.0),
                (0.0, 1.0),
                (-1.0, 0.0),
                (0.0, -1.0),
            ] {
                wires.push(vec![
                    frame.at(u * radius, v * radius, 0.0),
                    apex.clone(),
                ]);
            }

            wires
        }
        _ => Vec::new(),
    }
}

/// A kernel cone on the base, apex along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius, height] = part.sizes[..] else {
        return Err("Cone needs a radius and a height".into());
    };
    let (frame, height) = part.frame.upright(height);
    Ok(shape::solid(
        BRep::create_cone(radius, height),
        option,
        &frame.to_xform(),
        "cone",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::Answer;
    use crate::primitive_shape::tests::{XZ, build, p, plane};

    /// Two faces, the apex on the normal at the height.
    #[test]
    fn the_apex_is_on_the_normal() {
        let answers = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(200.0),
            Answer::Number(500.0),
        ];
        let Geometry::BRep(brep) =
            build(&SHAPE, &plane(XZ.0, XZ.1), &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 2);
        assert!(brep.is_solid());
        assert!(
            brep.vertex_points()
                .iter()
                .any(|q| q.distance(&p(0.0, -500.0, 0.0), None) < 1e-9)
        );
    }
}
```

### 2. `src/lib.rs`

Register this shared original solid builder.

**Locate this existing block:**

```rust
pub mod round_cylinder;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod round_cylinder;

pub mod round_cone;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/round_sphere.rs`

Type the whole original solid builder using the existing frame and question helpers. Supply only the unchanged original representation and topology fixture.

Create the file and type:

```rust
use crate::primitive_shape as shape;
use crate::primitive_shape::{BREP_MESH, Part, Shape};
use session_rust::{BRep, Geometry};

pub static SHAPE: Shape = Shape {
    name: "Sphere",
    options: BREP_MESH,
    upfront: false,
    ask: shape::center_radius,
    read: shape::read_radius,
    outline: shape::sphere_outline,
    build,
};

/// A kernel sphere at the center, poles along the normal.
fn build(part: &Part, option: &str) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err("Sphere needs a radius".into());
    };
    Ok(shape::solid(
        BRep::create_sphere(radius),
        option,
        &part.frame.to_xform(),
        "sphere",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive_shape::Answer;
    use crate::primitive_shape::tests::{XZ, build, corners, p, plane};

    /// One face, poles on the normal; the Mesh option lies on the sphere; zero is refused.
    #[test]
    fn the_sphere_has_its_poles_on_the_normal() {
        let front = plane(XZ.0, XZ.1);
        let answers = [
            Answer::Point(p(1.0, 2.0, 3.0)),
            Answer::Number(10.0),
        ];
        let Geometry::BRep(brep) = build(&SHAPE, &front, &answers, "Brep").unwrap()
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 1);
        assert!(brep.is_solid());
        let points = brep.vertex_points();
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(1.0, -8.0, 3.0), None) < 1e-9)
        );
        assert!(
            points
                .iter()
                .any(|q| q.distance(&p(1.0, 12.0, 3.0), None) < 1e-9)
        );
        let mesh = build(&SHAPE, &front, &answers, "Mesh").unwrap();
        assert!(matches!(mesh, Geometry::Mesh(_)));

        for q in corners(&mesh) {
            assert!((q.distance(&p(1.0, 2.0, 3.0), None) - 10.0).abs() < 0.1);
        }

        let zero = [
            Answer::Point(p(0.0, 0.0, 0.0)),
            Answer::Number(0.0),
        ];
        assert!(build(&SHAPE, &front, &zero, "Brep").is_err());
    }
}
```

### 4. `src/lib.rs`

Register this shared original solid builder.

**Locate this existing block:**

```rust
pub mod round_cone;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod round_cone;

pub mod round_sphere;

#[cfg(target_arch = "wasm32")]
```

## Build & Run

![Recorded visual output for Lesson 14.4 (74zb-build-original-cones-and-spheres)](../../screenshots/journey/74zb-build-original-cones-and-spheres.png)

```sh
npm --prefix ../session_tests run course -- dependencies 14.4
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 14.4
npm --prefix ../session_tests run course -- save 14.4
```

Continue with [14.5 · Round solids and exact planar conics](75-reuse-current-round-solid-builders.md).
