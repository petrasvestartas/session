# 22.3 · Original boundary pens, joins and vertex markers

**Goal:** Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership. · **Steps:** 4 · **Source:** [engine/gpu/render.rs](../../../src/engine/gpu/render.rs) · [engine/gpu/pass.rs](../../../src/engine/gpu/pass.rs) · [engine/gpu/targets.rs](../../../src/engine/gpu/targets.rs) · [engine/gpu/frame.rs](../../../src/engine/gpu/frame.rs) · [engine/gpu/view.rs](../../../src/engine/gpu/view.rs) · [engine/pipelines/mod.rs](../../../src/engine/pipelines/mod.rs) · [shaders/triangle.wgsl](../../../src/shaders/triangle.wgsl) · [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../src/shaders/vector.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [app/scene.rs](../../../src/app/scene.rs) (`src/shaders/scene.wgsl,src/engine/gpu/msaa.rs,src/engine/gpu/render.rs`)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Corner Tangents t0, t1 &amp; Miter Vector m = (n0+n1)/dot</text>
  <polyline points="58,136 164,66 270,136" fill="none" stroke="#111111" stroke-width="2" stroke-dasharray="4 3"/>
  <polygon points="48,122 164,44 280,122 260,150 164,88 68,150" fill="#dbeafe" fill-opacity="0.7" stroke="#2563eb" stroke-width="1.75"/>
  <line x1="164" y1="66" x2="164" y2="44" stroke="#dc2626" stroke-width="2.5" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="192" y="54" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">m</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Round Cap SDF (||uv|| &lt;= 1) + Arc-Length Dash Modulo</text>
  <circle cx="468" cy="102" r="28" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <line x1="468" y1="102" x2="496" y2="102" stroke="#dc2626" stroke-width="2"/>
  <rect x="526" y="88" width="46" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <rect x="592" y="88" width="24" height="28" fill="none" stroke="#94a3b8" stroke-width="1.5" stroke-dasharray="3 3"/>
  <rect x="636" y="88" width="46" height="28" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Clean Mitered Corners &amp; Dashed CAD Strokes</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polyline points="834,136 904,68 974,136" fill="none" stroke="#111111" stroke-width="6" stroke-linejoin="miter" stroke-linecap="round"/>
  <line x1="964" y1="74" x2="1044" y2="136" stroke="#2563eb" stroke-width="4" stroke-dasharray="10 6" stroke-linecap="round"/>
</svg>
</div>

## <a id="checkpoint-96zzzzzzn-respect-original-boundary-pens"></a>1. Respect boundary pens

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">4× MSAA Subpixel Samples (s0..s3) Inside Pixel Square</text>
  <rect x="108" y="50" width="108" height="104" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <line x1="116" y1="146" x2="208" y2="56" stroke="#2563eb" stroke-width="2.5"/>
  <polygon points="116,146 208,56 216,56 216,154 116,154" fill="#dbeafe" fill-opacity="0.65"/>
  <circle cx="142" cy="76" r="5" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <circle cx="188" cy="68" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="136" cy="124" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <circle cx="182" cy="116" r="5" fill="#2563eb" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="252" y="106" font-family="monospace" font-size="10" font-weight="700" fill="#111111">cov = 3/4</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Key + Fill + Rim Rig + Ground Plane Major/Minor Grid</text>
  <polygon points="432,140 492,98 688,98 628,140" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="462" y1="119" x2="658" y2="119" stroke="#dc2626" stroke-width="1.75"/>
  <line x1="560" y1="98" x2="500" y2="140" stroke="#16a34a" stroke-width="1.75"/>
  <polygon points="520,122 560,62 600,122" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <ellipse cx="560" cy="126" rx="34" ry="8" fill="#64748b" fill-opacity="0.45"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Anti-Aliased CAD Viewport + Ground Grid &amp; Contact Shadow</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="826,144 876,104 1042,104 992,144" fill="#f8fafc" stroke="#cbd5e1" stroke-width="1.2"/>
  <ellipse cx="934" cy="130" rx="36" ry="8" fill="#94a3b8" fill-opacity="0.5"/>
  <polygon points="894,126 934,58 974,126" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
</svg>
</div>

**What this code does:** Render original edge widths and colors, hide authored zero-width mesh edges, and omit coplanar seams without changing source face keys.

### 1. `src/mesh_pens.rs`

Prepare only numeric pens using the already-taught topology and original encounter order.

Create the file and type:

```rust
use crate::source_mesh_topology::{SlotMap, mesh_topology};
use session_rust::Mesh;
pub type Pen = (f32, u32);

pub fn width(value: f64) -> f32 {
    let value = value as f32;
    if value.is_finite() && value > 0. {
        value
    } else {
        1.
    }
}

pub fn prepare(source: &Mesh, count: usize) -> (Vec<(usize, usize)>, Vec<Pen>) {
    if count > 200_000 || source.widths() == [0.] {
        return (Vec::new(), Vec::new());
    }
    let keys = source.vertices();
    let points = keys
        .iter()
        .map(|key| {
            let p = &source.vertex[key];
            [p.x, p.y, p.z]
        })
        .collect::<Vec<_>>();
    let topology = mesh_topology(source, &keys, &points, &SlotMap::new(&keys));
    let mut pens = Vec::with_capacity(topology.edges.len());
    for (i, &(_, _, colour)) in topology.edges.iter().enumerate() {
        let widths = source.widths();
        let value = if widths.len() == 1 {
            widths[0]
        } else {
            widths.get(i).copied().unwrap_or(1.)
        };
        let pair = topology.edge_faces[i].map(|face| {
            topology
                .normals
                .get(face as usize)
                .copied()
                .flatten()
        });
        let flat = match pair {
            [Some(a), Some(b)] => {
                a.iter()
                    .zip(b)
                    .map(|(a, b)| a * b)
                    .sum::<f64>()
                    * if topology.opposed[i] {
                        1.
                    } else {
                        -1.
                    }
                    >= 1. - 1e-9
            }
            _ => false,
        };
        pens.push((
            if value == 0. || flat {
                0.
            } else {
                width(value)
            },
            if topology.edges.len() >= 10_000 {
                crate::source_topology_colour::BLACK
            } else {
                colour
            },
        ));
    }
    if pens
        .first()
        .is_some_and(|first| pens.iter().all(|pen| pen == first))
    {
        pens = vec![pens[0]];
    }
    (
        topology
            .edges
            .into_iter()
            .map(|(a, b, _)| (a, b))
            .collect(),
        pens,
    )
}
```

### 2. `src/lib.rs`

Connect the prepared style producer to the independent learner runtime.

**Locate this existing block:**

```rust
pub mod mesh_parts;

#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod mesh_parts;
pub mod mesh_pens;

#[cfg(test)]
```

### 3. `src/mesh_parts.rs`

Keep lightweight pens beside the existing shared source-key display. Reuse one topology preparation for styling and coplanar edge omission. Retain pens after the editable source is unloaded. Decode packed original colors only when preparing the existing stroke stream. Count actual retained numeric pen capacity.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        })
    }

    pub fn bytes(&self) -> usize {
        self.faces.capacity() * std::mem::size_of::<usize>()
            + self.triangles.capacity() * 4
```

Replace it with:

```rust
        })
    }

    pub fn pen(&self, index: usize) -> (f32, [f32; 4]) {
        let &(width, colour) = self
            .pens
            .get(if self.pens.len() == 1 {
                0
            } else {
                index
            })
            .unwrap_or(&(1., crate::source_topology_colour::BLACK));
        (
            width,
            colour.to_le_bytes().map(|v| v as f32 / 255.),
        )
    }

    pub fn bytes(&self) -> usize {
        self.pens.capacity() * std::mem::size_of::<crate::mesh_pens::Pen>()
            + self.faces.capacity() * std::mem::size_of::<usize>()
            + self.triangles.capacity() * 4
```

**Locate this block:**

```rust
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self {
            faces,
            triangles,
            edges,
```

Replace it with:

```rust
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self {
            pens,
            faces,
            triangles,
            edges,
```

**Locate this block:**

```rust
        if triangles.len() != count {
            return Err("Source faces do not match the rendered triangles");
        }
        let edges = source.edges();
        let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source
```

Replace it with:

```rust
        if triangles.len() != count {
            return Err("Source faces do not match the rendered triangles");
        }
        let (edges, pens) = crate::mesh_pens::prepare(source, count);
        let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source
```

**Locate this block:**

```rust
pub struct Parts {
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
```

Replace it with:

```rust
pub struct Parts {
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
    pub edges: Vec<(usize, usize)>,
```

### 4. `src/bounds_original.rs`

Existing analytic bounds edges retain their default black pen.

**Locate this existing block:**

```rust
        endpoints,
    };
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        endpoints,
        pens: Vec::new(),
    };
```

### 5. `src/pick_sources.rs`

Keep source endpoint keys, boundary addresses and pen slots aligned. Respect each original geometry’s width semantics before emitting visible ink. Preserve source edge colors unless the existing display override wins. Use the existing CSS-pixel stroke path rather than another render lane.

**Locate this existing block:**

```rust
                .map(|surface| surface.edge_ids());
            for (&(a, b), ends) in row
                .mesh
                .parts
                .edges
                .iter()
                .zip(&row.mesh.parts.endpoints)
            {
                let part = cad_edges
                    .as_mut()
                    .and_then(|edges| edges.next())
                    .map(|edge| Part::BRepEdge(*edge))
                    .unwrap_or_else(|| {
                        surface_edges
                            .as_mut()
                            .and_then(|edges| edges.next())
                            .map_or(Part::MeshEdge(a, b), Part::SurfaceEdge)
                    });
                let ids = append(
                    &mut sources.edges,
                    Kind::Edge,
                    Target {
                        owner: row.id,
                        part,
                    },
                )?;
                let ends = ends.map(|p| {
                    model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
                        .to_f32()
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
                    end: ends[1],
                    colour: scene.view.edges.get(&row.id).map_or(
                        [0., 0., 0., 1.],
                        |rgb| {
                            [
                                rgb[0] as f32 / 255.,
                                rgb[1] as f32 / 255.,
                                rgb[2] as f32 / 255.,
                                1.,
                            ]
                        },
                    ),
                    width: 1.,
                });
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                .map(|surface| surface.edge_ids());
            for (index, (&(a, b), ends)) in row
                .mesh
                .parts
                .edges
                .iter()
                .zip(&row.mesh.parts.endpoints)
                .enumerate()
            {
                let part = cad_edges
                    .as_mut()
                    .and_then(|edges| edges.next())
                    .map(|edge| Part::BRepEdge(*edge))
                    .unwrap_or_else(|| {
                        surface_edges
                            .as_mut()
                            .and_then(|edges| edges.next())
                            .map_or(Part::MeshEdge(a, b), Part::SurfaceEdge)
                    });
                let ids = append(
                    &mut sources.edges,
                    Kind::Edge,
                    Target {
                        owner: row.id,
                        part,
                    },
                )?;
                let (width, colour) = row.mesh.parts.pen(index);
                let width = boundary_width.unwrap_or(width);
                if width == 0. {
                    continue;
                }
                let ends = ends.map(|p| {
                    model
                        .transform_point(&session_rust::Point::new(p[0], p[1], p[2]))
                        .to_f32()
                });
                sources.mesh_ink.push(crate::stroke::Stroke {
                    start: ends[0],
                    end: ends[1],
                    colour: edge_colour.unwrap_or(colour),
                    width,
                });
```

### 6. `src/source_pen_tests.rs`

Supply real GPU/source/archive, sparse style-order and source-unload controls.

Create the file and type:

```rust
use crate::{
    camera::Projection,
    cap_gpu_tests::{pixels_at, renderer, wait},
    editor::{Action, Editor},
    pick_sources::{Mode, Part, Sources, Target},
    prepared::PreparedMesh,
    scene::Scene,
};
use prost::Message;
use session_rust::{BRep, Color, Mesh, NurbsSurface, Point, Session, Xform, proto};
use std::rc::Rc;

fn panel(width: f64) -> Mesh {
    let mut source = Mesh::from_vertices_and_faces(
        vec![
            Point::new(-900., -650., 0.),
            Point::new(900., -650., 0.),
            Point::new(900., 650., 0.),
            Point::new(-900., 650., 0.),
        ],
        vec![vec![0, 1, 2, 3]],
    );
    source.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    source.set_linecolors(vec![], vec![width]);
    source
}
fn sparse() -> Mesh {
    let mut source = Mesh::new();
    let keys = [11, 70, 803, 9004, 500000, 1000000];
    for (key, p) in keys.into_iter().zip([
        [-900., -650., 0.],
        [0., -650., 0.],
        [900., -650., 0.],
        [-900., 650., 0.],
        [0., 650., 0.],
        [900., 650., 0.],
    ]) {
        assert_eq!(
            source.add_vertex(Point::new(p[0], p[1], p[2]), Some(key)),
            key
        );
    }
    source.add_face(vec![11, 70, 500000, 9004], Some(17));
    source.add_face(vec![70, 803, 1000000, 500000], Some(93));
    source.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    source.set_linecolors(
        vec![
            Color::new(1., 0., 0., 1.),
            Color::new(1., 0., 1., 1.),
            Color::new(0., 1., 0., 1.),
            Color::new(0., 0., 1., 1.),
            Color::new(1., 0., 1., 1.),
            Color::new(0., 1., 1., 1.),
            Color::new(1., 0.5, 0., 1.),
        ],
        vec![7., 9., 3., 5., 0., 4., 6.],
    );
    source
}
fn camera() -> crate::camera::Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3000.;
    c.radius = 1500.;
    c.aspect = 4. / 3.;
    c
}
fn point(matrix: [f32; 16], p: [f64; 3]) -> (usize, usize) {
    let p = super::contact_feature_tests::project(matrix, p);
    (
        (((p[0] + 1.) * 160.).floor() as usize).min(319),
        (((1. - p[1]) * 120.).floor() as usize).min(239),
    )
}
fn at(image: &[u8], matrix: [f32; 16], p: [f64; 3]) -> [u8; 4] {
    let (x, y) = point(matrix, p);
    crate::cap_gpu_tests::at(image, x, y)
        .try_into()
        .unwrap()
}
fn span(
    image: &[u8],
    matrix: [f32; 16],
    p: [f64; 3],
    horizontal: bool,
    accept: impl Fn(&[u8]) -> bool,
) -> usize {
    let (x, y) = point(matrix, p);
    (-24..=24)
        .filter(|delta| {
            let (x, y) = if horizontal {
                (x as isize + delta, y as isize)
            } else {
                (x as isize, y as isize + delta)
            };
            x >= 0
                && y >= 0
                && x < 320
                && y < 240
                && accept(crate::cap_gpu_tests::at(
                    image, x as usize, y as usize,
                ))
        })
        .count()
}
fn pick(
    gpu: &mut crate::renderer::Renderer,
    scene: &Scene,
    matrix: [f32; 16],
    p: [f64; 3],
    mode: Mode,
) -> Option<Target> {
    gpu.set_pick_mode(mode);
    gpu.request_pick(
        super::contact_feature_tests::project(matrix, p),
        matrix,
    )
    .unwrap();
    wait(gpu);
    gpu.take_pick(matrix)
        .unwrap()
        .unwrap()
        .and_then(|raw| gpu.resolve_pick(scene, raw))
}
fn archive(scene: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let bundle =
        crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap())
            .unwrap();
    (
        bundle
            .documents
            .iter()
            .map(|bytes| proto::Session::decode(bytes.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(bundle.metadata).unwrap(),
    )
}
fn export(name: &str, scene: &Scene, pixels: Option<&[u8]>) {
    if let Some(root) = std::env::var_os("VIEWER_SOURCE_PEN_PROOF_DIR") {
        use std::io::Write;
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        let bytes = crate::scene_archive::save(scene).unwrap();
        let bundle = crate::archive::Bundle::decode(&bytes).unwrap();
        assert_eq!(bundle.documents.len(), 1);
        std::fs::write(root.join(format!("{name}.session")), bytes).unwrap();
        std::fs::write(
            root.join(format!("{name}.pb")),
            &bundle.documents[0],
        )
        .unwrap();
        if let Some(pixels) = pixels {
            let mut file =
                std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();
            write!(file, "P6\n320 240\n255\n").unwrap();
            for pixel in pixels.chunks_exact(4) {
                file.write_all(&pixel[..3]).unwrap();
            }
        }
    }
}
fn mesh_scene(source: Mesh) -> Scene {
    let mut scene = Scene::empty();
    scene
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    scene
}

#[test]
fn authored_mesh_pens_retain_sparse_keys_colors_and_flat_face_queries() {
    let mut scene = mesh_scene(sparse());
    let source = scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    let initial = archive(&scene);
    let owner = scene.objects()[0].id;
    let targets = Sources::new(&scene).unwrap();
    assert_eq!(
        targets
            .faces
            .iter()
            .map(|t| t.part)
            .collect::<Vec<_>>(),
        [Part::Face(17), Part::Face(93)]
    );
    assert_eq!(
        targets.mesh_ink.len(),
        5,
        "Seven true source edges, minus the coplanar seam and authored hidden edge"
    );
    assert!(
        targets
            .edges
            .iter()
            .any(|t| t.part == Part::MeshEdge(70, 500000)),
        "Omitted ink does not rewrite source identity"
    );
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let uploads = gpu.stats()[0];
    let mut c = camera();
    for projection in [
        Projection::Orthographic,
        Projection::Perspective,
    ] {
        c.projection = projection;
        let matrix = c.uniform();
        for density in [1., 2.] {
            gpu.set_density(density);
            let before = pixels_at(&gpu, matrix);
            assert_eq!(
                at(&before, matrix, [0., 0., 0.]),
                [51, 102, 153, 255],
                "A coplanar source seam has no decorative stroke"
            );
            assert!(
                span(
                    &before,
                    matrix,
                    [-450., -650., 0.],
                    false,
                    |p| p[0] > 220 && p[1] < 40 && p[2] < 40
                ) >= 4 * density as usize,
                "Authored red seven-pixel pen belongs to the first encountered edge"
            );
            assert!(
                span(&before, matrix, [-900., 0., 0.], true, |p| p
                    [2]
                    > 220
                    && p[0] < 40
                    && p[1] < 40)
                    >= 2 * density as usize,
                "Source encounter order preserves the blue left pen"
            );
            assert_eq!(
                pick(
                    &mut gpu,
                    &scene,
                    matrix,
                    [0., 0., 0.],
                    Mode::Edge
                ),
                None,
                "The coplanar seam is not visible selectable ink"
            );
            assert_eq!(
                pick(
                    &mut gpu,
                    &scene,
                    matrix,
                    [450., -650., 0.],
                    Mode::Edge
                ),
                None,
                "Authored zero-width edge remains hidden"
            );
            assert_eq!(
                pick(
                    &mut gpu,
                    &scene,
                    matrix,
                    [-450., -650., 0.],
                    Mode::Edge
                ),
                Some(Target {
                    owner,
                    part: Part::MeshEdge(11, 70)
                })
            );
            for (x, key) in [(-450., 17), (450., 93)] {
                assert_eq!(
                    pick(
                        &mut gpu,
                        &scene,
                        matrix,
                        [x, 0., 0.],
                        Mode::Face
                    ),
                    Some(Target {
                        owner,
                        part: Part::Face(key)
                    })
                );
            }
            assert_eq!(pixels_at(&gpu, matrix), before);
            assert_eq!(archive(&scene), initial);
            assert_eq!(
                scene.objects()[0]
                    .geometry()
                    .unwrap()
                    .to_proto(),
                source
            );
            if projection == Projection::Orthographic && density == 1. {
                export("sparse-pens", &scene, Some(&before));
            }
        }
    }
    gpu.set_density(1.);
    Rc::make_mut(&mut scene.view)
        .edges
        .insert(owner, [255, 0, 255]);
    gpu.set_scene(&scene, None);
    let override_pixels = pixels_at(&gpu, c.uniform());
    assert!(
        span(
            &override_pixels,
            c.uniform(),
            [-900., 0., 0.],
            true,
            |p| p[0] > 220 && p[2] > 220 && p[1] < 40
        ) >= 2,
        "Existing explicit display-edge override wins"
    );
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(
        scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        source
    );
}

fn surface(width: f64) -> NurbsSurface {
    let mut source = NurbsSurface::create(
        false,
        false,
        1,
        1,
        2,
        2,
        &[
            Point::new(-900., -650., 0.),
            Point::new(-900., 650., 0.),
            Point::new(900., -650., 0.),
            Point::new(900., 650., 0.),
        ],
    )
    .unwrap();
    source.width = width;
    source.facecolors = vec![Color::new(0.2, 0.4, 0.6, 1.)];
    source
}
#[test]
fn actual_surface_and_brep_boundary_widths_use_css_pens() {
    for kind in ["mesh", "surface", "brep"] {
        let mut scenes = Vec::new();
        for width in [1., 7., 0., -2.] {
            let mut scene = Scene::empty();
            match kind {
                "mesh" => {
                    scene
                        .insert(PreparedMesh::new(panel(width)).unwrap())
                        .unwrap();
                }
                "surface" => {
                    scene
                        .insert_surface(Rc::new(surface(width)))
                        .unwrap();
                }
                _ => {
                    let mut source = BRep::create_box(1800., 1300., 100.);
                    source.width = width;
                    source.surfacecolor = Color::new(0.2, 0.4, 0.6, 1.);
                    scene.insert_cad(Rc::new(source)).unwrap();
                }
            }
            scenes.push(scene);
        }
        let mut gpu = renderer(&scenes[0]);
        gpu.set_opacity(1.);
        let c = camera();
        let matrix = c.uniform();
        let sample = [
            900.,
            0.,
            if kind == "brep" {
                50.
            } else {
                0.
            },
        ];
        for density in [1., 2.] {
            gpu.set_density(density);
            let mut images = Vec::new();
            let mut widths = Vec::new();
            for (index, scene) in scenes.iter().enumerate() {
                let original = archive(scene);
                gpu.set_scene(scene, None);
                let pixels = pixels_at(&gpu, matrix);
                widths.push(span(&pixels, matrix, sample, true, |p| {
                    p[..3].iter().all(|v| *v < 64)
                }));
                assert_eq!(archive(scene), original);
                if density == 1. {
                    export(
                        &format!(
                            "{kind}-{}",
                            ["default", "wide", "zero", "negative"][index]
                        ),
                        scene,
                        Some(&pixels),
                    );
                }
                images.push(pixels);
            }
            assert!(
                widths[1] >= widths[0] + 4 * density as usize,
                "Actual {kind} seven-CSS-pixel border must be wider: {widths:?}"
            );
            if kind != "mesh" {
                assert_eq!(
                    images[2], images[0],
                    "Nonpositive {kind} width uses the default pen"
                );
            }
            assert_eq!(
                images[3], images[0],
                "Negative width is the default pen, not a world-unit stroke"
            );
        }
    }
}

#[test]
fn prepared_pens_survive_unload_and_shared_placement() {
    let mut document = Session::new("Original unload pens");
    document.add_mesh(panel(7.), None);
    let bytes = document.to_proto().encode_to_vec();
    let mut editor = Editor::default();
    editor
        .apply(Action::ReplaceAt(
            bytes.clone(),
            Rc::new(crate::reload_url::ReloadUrl::new(
                "test:source-pens".into(),
            )),
        ))
        .unwrap();
    editor.camera = camera();
    let source = archive(&editor.scene);
    let display = Rc::clone(&editor.scene.objects()[0].mesh);
    let original = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    let document = Rc::downgrade(
        &editor.scene.objects()[0]
            .source()
            .unwrap()
            .document,
    );
    assert_eq!(
        display.parts.pens.capacity() * std::mem::size_of::<crate::mesh_pens::Pen>(),
        8,
        "Identical pens retain one packed eight-byte entry"
    );
    let mut gpu = renderer(&editor.scene);
    gpu.set_opacity(1.);
    let matrix = editor.camera.uniform();
    let before = pixels_at(&gpu, matrix);
    let uploads = gpu.stats()[0];
    editor.apply(Action::UnloadSources).unwrap();
    assert!(
        original.upgrade().is_none() && document.upgrade().is_none(),
        "Prepared numeric pens must not pin original Mesh/Document ownership"
    );
    assert!(
        editor.scene.objects()[0]
            .geometry()
            .is_none()
    );
    assert!(Rc::ptr_eq(
        &display,
        &editor.scene.objects()[0].mesh
    ));
    gpu.set_scene(&editor.scene, None);
    assert_eq!(
        pixels_at(&gpu, matrix),
        before,
        "Source unload must retain exact original pen pixels"
    );
    assert_eq!(gpu.stats()[0], uploads);
    let keys = editor.reload_keys();
    assert!(
        editor
            .hydrate(
                keys.into_iter()
                    .map(|key| (key, bytes.clone()))
                    .collect()
            )
            .unwrap()
    );
    assert_eq!(archive(&editor.scene), source);
    gpu.set_scene(&editor.scene, None);
    assert_eq!(pixels_at(&gpu, matrix), before);
    assert_eq!(gpu.stats()[0], uploads);
    let mesh = panel(7.);
    let guid = mesh.guid().to_owned();
    let mut shared = crate::instance_specimen::message();
    shared.definitions.as_mut().unwrap().meshes = vec![mesh.to_proto()];
    for instance in &mut shared.objects.as_mut().unwrap().instances {
        instance.definition_guid = guid.clone();
        instance.xform = None;
        instance.flags = 0;
    }
    shared.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1200., 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1200., 0., 0.)
                    * &Xform::scale_xyz(-0.8, 1.2, 1.4))
                    .to_proto(),
            ),
        },
    ];
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &shared.encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert!(Rc::ptr_eq(
        &scene.objects()[0].mesh,
        &scene.objects()[1].mesh
    ));
    let initial = archive(&scene);
    let ids = scene
        .objects()
        .iter()
        .map(|r| r.id)
        .collect::<Vec<_>>();
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let mut c = camera();
    c.distance = 6000.;
    c.radius = 3000.;
    let matrix = c.uniform();
    let before = pixels_at(&gpu, matrix);
    let uploads = gpu.stats()[0];
    let placement = scene.placement(ids[0]).unwrap();
    scene
        .place(
            ids[0],
            &Xform::translation(100., 150., 0.) * &placement,
        )
        .unwrap();
    gpu.set_scene(&scene, None);
    assert_ne!(pixels_at(&gpu, matrix), before);
    scene.place(ids[0], placement).unwrap();
    gpu.set_scene(&scene, None);
    assert_eq!(pixels_at(&gpu, matrix), before);
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(archive(&scene), initial);
    export("shared-placed-pens", &scene, Some(&before));
}

#[test]
fn printed_and_dense_originals_keep_faces_without_decoration() {
    let scene = mesh_scene(panel(0.));
    let initial = archive(&scene);
    let owner = scene.objects()[0].id;
    assert!(
        scene.objects()[0]
            .mesh
            .parts
            .edges
            .is_empty()
    );
    assert!(
        Sources::new(&scene)
            .unwrap()
            .mesh_ink
            .is_empty()
    );
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let matrix = camera().uniform();
    let pixels = pixels_at(&gpu, matrix);
    assert_eq!(
        at(&pixels, matrix, [0., 0., 0.]),
        [51, 102, 153, 255]
    );
    assert_eq!(
        pick(
            &mut gpu,
            &scene,
            matrix,
            [0., 0., 0.],
            Mode::Face
        ),
        Some(Target {
            owner,
            part: Part::Face(0)
        })
    );
    assert_eq!(archive(&scene), initial);
    export("mesh-printed-fill", &scene, Some(&pixels));
    let n = 317;
    let mut points = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            points.push(Point::new(
                -900. + 1800. * x as f64 / n as f64,
                -650. + 1300. * y as f64 / n as f64,
                0.,
            ));
        }
    }
    let mut faces = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let a = y * (n + 1) + x;
            faces.extend([
                vec![a, a + 1, a + n + 2],
                vec![a, a + n + 2, a + n + 1],
            ]);
        }
    }
    let mut dense = Mesh::from_vertices_and_faces(points, faces);
    dense.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    let count = dense.number_of_faces();
    assert!(count > 200_000);
    let scene = mesh_scene(dense);
    assert_eq!(
        scene.objects()[0].mesh.indices().len() / 3,
        count
    );
    assert!(
        scene.objects()[0]
            .mesh
            .parts
            .edges
            .is_empty()
    );
    assert!(scene.objects()[0].mesh.parts.pens.is_empty());
    gpu.set_scene(&scene, None);
    let pixels = pixels_at(&gpu, matrix);
    assert_eq!(
        at(&pixels, matrix, [0., 0., 0.]),
        [51, 102, 153, 255]
    );
    assert_eq!(
        pick(
            &mut gpu,
            &scene,
            matrix,
            [100., 100., 0.],
            Mode::Object
        ),
        Some(Target {
            owner: scene.objects()[0].id,
            part: Part::Object
        })
    );
}
```

### 7. `src/renderer.rs`

Connect Native validation outside the browser runtime.

**Locate this existing block:**

```rust
#[path = "source_face_policy_tests.rs"]
mod source_face_policy_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "source_face_policy_tests.rs"]
mod source_face_policy_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "source_pen_tests.rs"]
mod source_pen_tests;
```

### 8. `src/plane_original.rs`

Existing analytic plane boundaries retain their default black pen.

**Locate this existing block:**

```rust
    display.parts = crate::mesh_parts::Parts {
        faces: Vec::new(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    display.parts = crate::mesh_parts::Parts {
        pens: Vec::new(),
        faces: Vec::new(),
```

### 9. `src/pick_sources.rs`

Resolve immutable mesh-row ownership and explicit display color once. Share the existing source-face eligibility classification. Resolve the original surface/BRep scalar pen once per row, outside its boundary segments. Retain identical original pick addresses without repeated per-segment hierarchy lookups.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    colour: edge_colour.unwrap_or(colour),
                    width,
                });
                sources.mesh_ids.push(id(row.id, ids[1]));
                sources.mesh_metadata.extend([flag, facing]);
```

Replace it with:

```rust
                    colour: edge_colour.unwrap_or(colour),
                    width,
                });
                sources.mesh_ids.push([
                    owner,
                    if derived {
                        0
                    } else {
                        ids[1]
                    },
                ]);
                sources.mesh_metadata.extend([flag, facing]);
```

**Locate this block:**

```rust
            } else {
                0
            };
            let mut cad_edges = row
                .cad
                .as_ref()
```

Replace it with:

```rust
            } else {
                0
            };
            let boundary_width = row
                .cad
                .as_ref()
                .map(|c| c.source.width)
                .or_else(|| row.surface.as_ref().map(|s| s.source.width))
                .map(crate::mesh_pens::width);
            let mut cad_edges = row
                .cad
                .as_ref()
```

**Locate this block:**

```rust
        {
            let model = crate::render_space::placed(&row.model, anchor);
            sources.mesh_bases.push((
                row.id,
                u32::try_from(sources.faces.len())
                    .map_err(|_| "Too many source faces")?,
            ));
            if !scene.forest.derived(row.id) {
                for &key in &row.mesh.parts.faces {
                    append(
                        &mut sources.faces,
```

Replace it with:

```rust
        {
            let derived = scene.forest.derived(row.id);
            let owner = crate::feature_owner::display_owner(scene, row.id)
                .map_or(0, ObjectId::value);
            let edge_colour = scene.view.edges.get(&row.id).map(|rgb| {
                [
                    rgb[0] as f32 / 255.,
                    rgb[1] as f32 / 255.,
                    rgb[2] as f32 / 255.,
                    1.,
                ]
            });
            let model = crate::render_space::placed(&row.model, anchor);
            sources.mesh_bases.push((
                row.id,
                u32::try_from(sources.faces.len())
                    .map_err(|_| "Too many source faces")?,
            ));
            if !derived {
                for &key in &row.mesh.parts.faces {
                    append(
                        &mut sources.faces,
```

### 10. `src/mesh_arena_tests.rs`

Keep the supplied stale-binding GPU probe inside its actual bound arrays; preserve all pixel, identity and write-count assertions.

**Locate this existing block:**

```rust
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  let v=vertices[indices[occurrence]];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
 @vertex fn vertex(@builtin(vertex_index) occurrence:u32)->Out{
  if (occurrence >= arrayLength(&indices)) { return Out(vec4<f32>(2.,2.,2.,1.),vec4<f32>(0.)); }
  let index=indices[occurrence];
  if (index >= arrayLength(&vertices)) { return Out(vec4<f32>(2.,2.,2.,1.),vec4<f32>(0.)); }
  let v=vertices[index];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));
 }
```

## <a id="checkpoint-96zzzzzzo-close-original-boundary-joins"></a>2. Close boundary joins

**What this code does:** Close wide original boundary corners and preserve actual source queries at round ends and flat near-plane cuts.

### 1. `src/stroke_quad.wgsl`

Share near-clipped round-stroke projection without new records or buffers.

Create the file and type:

```wgsl
struct StrokeQuad {
    position:vec4f, axis:vec4f, pixel:vec2f, depths:vec2f, radius:f32, flat_ends:u32,
}
fn stroke_screen(p:vec4f)->vec2f {return (p.xy/p.w*vec2f(0.5,-0.5)+0.5)*view.size;}
fn stroke_quad(start:vec4f,end:vec4f,width:f32,index:u32)->StrokeQuad {
    var a=start;var b=end;
    var flat=u32(near_distance(a)<0.)|(u32(near_distance(b)<0.)<<1u);
    let hidden=StrokeQuad(vec4f(2.,2.,2.,1.),vec4f(0.),vec2f(0.),vec2f(0.),0.,0u);
    if(flat==3u||all(a==b)){return hidden;}
    a=near_endpoint(a,b);b=near_endpoint(b,a);
    if(a.w<=0.||b.w<=0.){return hidden;}
    let delta=(b.xy/b.w-a.xy/a.w)*view.size;
    let length2=dot(delta,delta);var direction=vec2f(1.,0.);
    if(length2>0.){direction=delta*inverseSqrt(length2);}else{
        a=select(a,b,b.z/b.w>a.z/a.w);b=a;flat=0u;
    }
    let corners=array<vec2f,6>(vec2f(0.,-1.),vec2f(1.,-1.),vec2f(0.,1.),vec2f(0.,1.),vec2f(1.,-1.),vec2f(1.,1.));
    let q=corners[index];let radius=width*view.density*0.5;let extent=radius+0.70711;
    var p=select(a,b,q.x>0.5);
    let offset=(vec2f(-direction.y,direction.x)*q.y+direction*(2.*q.x-1.))*extent;
    p=vec4f(p.xy+offset*2./view.size*p.w,p.zw);
    return StrokeQuad(p,vec4f(stroke_screen(a),stroke_screen(b)),stroke_screen(p),vec2f(a.z/a.w,b.z/b.w),radius,flat);
}
```

### 2. `src/finite_project_source.rs`

Expand the one shared projection helper in both consumers.

**Locate this existing block:**

```rust
        .replace(
            "#include \"stroke_coverage.wgsl\"",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .replace(
            "#include \"stroke_quad.wgsl\"",
            include_str!("stroke_quad.wgsl"),
        )
        .replace(
            "#include \"stroke_coverage.wgsl\"",
```

### 3. `src/stroke.wgsl`

Use one shared stroke shape for ordinary and finite rendering. Distinguish genuine round ends from flat near-plane cuts. Carry only screen coordinates needed by existing fragment coverage. Replace duplicate projection with the shared clipped capsule.

**Locate this existing block:**

```wgsl
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

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
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
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4f,
    @location(11) @interpolate(linear) finite_pixel:vec2f,
}

fn geometry(start:vec3f,end:vec3f,colour:vec4f,width:f32,index:u32)->Output {
    let s=stroke_quad(view.transform*vec4f(start,1.),view.transform*vec4f(end,1.),width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,s.axis,s.pixel);
}
```

### 4. `src/finite_stroke.wgsl`

Use one shared stroke shape for ordinary and finite rendering. Distinguish genuine round ends from flat near-plane cuts. Let the shared projection own hidden, near-clipped and end-on cases. Retain contact rise and reuse the shared screen-space shape.

**Locate this existing block:**

```wgsl
@group(0) @binding(0) var<uniform> view: View;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn hidden(colour: vec4<f32>,flags:u32) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0),0.);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    if (near_distance(a) < 0.0 && near_distance(b) < 0.0) { return hidden(colour,flags); }
    a=near_endpoint(a,b);
    b=near_endpoint(b,a);
    if (a.w <= 0.0 || b.w <= 0.0) { return hidden(colour,flags); }
    let delta = (b.xy / b.w - a.xy / a.w) * view.size;
    let length2 = dot(delta, delta);
    if (length2 < 0.000001) { return hidden(colour,flags); }
    let normal = vec2<f32>(-delta.y, delta.x) * inverseSqrt(length2);
    let corners = array<vec2<f32>, 6>(vec2<f32>(0.0, -1.0), vec2<f32>(1.0, -1.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let corner = corners[index];
    let half_width = width * view.density * 0.5;
    let extent = half_width + 0.70711;
    var position = select(a, b, corner.x > 0.5);
    position = vec4<f32>(position.xy + normal * extent * 2.0 / view.size * corner.y * position.w, position.zw);
    return Output(position, colour, corner.y * extent, half_width,flags,finite_axis(a,b),vec2<f32>(a.z/a.w,b.z/b.w),finite_screen(position),rise);
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
@group(0) @binding(0) var<uniform> view: View;
#include "stroke_quad.wgsl"

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32,metadata:InkRecord) -> Output {
    let words=ink_words(metadata);let flags=words.x;
    var a = view.transform * vec4<f32>(start, 1.0);
    var b = view.transform * vec4<f32>(end, 1.0);
    let z=vec2<f32>(a.z,b.z);let n=oct32_decode(words.y);
    a=contact_layer(a,start,n,view.transform,view.size,view.eye);
    b=contact_layer(b,end,n,view.transform,view.size,view.eye);
    let rise=max(0.,max((a.z-z.x)/max(a.w,1e-20),(b.z-z.y)/max(b.w,1e-20)));
    let s=stroke_quad(a,b,width,index);
    return Output(s.position,colour,s.flat_ends,s.radius,flags,s.axis,s.depths,s.pixel,rise);
}
```

### 5. `src/stroke_coverage.wgsl`

Measure distance to the finite segment; keep near cuts flat.

Replace the entire file with:

```wgsl
fn colour(input:Output)->vec4f {
    let ends=input.finite_axis_value;let delta=ends.zw-ends.xy;
    let span=length(delta);let direction=delta/max(span,1e-6);
    let along=dot(input.finite_pixel-ends.xy,direction);
    let distance=length(input.finite_pixel-ends.xy-direction*clamp(along,0.,span));
    var coverage=clamp(input.half_width+0.5-distance,0.,1.);
    if((input.flat_ends&1u)!=0u){coverage*=clamp(along+0.5,0.,1.);}
    if((input.flat_ends&2u)!=0u){coverage*=clamp(span-along+0.5,0.,1.);}
    return vec4f(input.colour.rgb,input.colour.a*coverage);
}
```

### 6. `src/stroke_pick.wgsl`

Keep color and integer queries on the same end policy. Carry the actual clipping-end policy into the cropped request. Reuse the exact color coverage in integer picking. Use full-viewport pixel coordinates for cropped coverage. Cropping changes clip position, not the original screen-space footprint. Pass the shared footprint to the ordinary pick fragment.

**Locate this existing block:**

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width);
    if (!wanted(input.id.y, crop.mode)) { discard; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4f,
    @location(11) @interpolate(linear) finite_pixel:vec2f,
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.flat_ends = source.flat_ends;
    output.half_width = source.half_width;
    output.finite_axis_value=source.finite_axis_value;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.flat_ends, input.half_width,input.finite_axis_value,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
```

### 7. `src/finite_stroke_pick.wgsl`

Keep color and integer queries on the same end policy. Carry the actual clipping-end policy into the cropped request. Reuse the exact color coverage in integer picking.

**Locate this existing block:**

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) side: f32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) metadata:InkRecord) -> PickOutput {
    let source = geometry(start, end, colour, width, index,metadata);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.side = source.side;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) flat_ends:u32,
    @location(2) @interpolate(flat) half_width: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
    @location(12) @interpolate(flat) rise:f32,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>,@location(8) metadata:InkRecord) -> PickOutput {
    let source = geometry(start, end, colour, width, index,metadata);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.flat_ends = source.flat_ends;
    output.half_width = source.half_width;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.rise=source.rise;output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.flat_ends, input.half_width,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel,input.rise);
    if (!wanted(input.id.y, crop.mode)) { discard; }
```

### 8. `src/round_caps_tests.rs`

Copy actual GPU validation fixtures.

Create the file and type:

```rust
use crate::{
    cap_gpu_tests::{pixels_at, renderer, wait},
    pick_sources::{Mode, Part, Target},
    prepared::PreparedMesh,
    scene::Scene,
};
use prost::Message;
use session_rust::{
    BRep, Color, Line, Mesh, NurbsSurface, Point, Session, Xform, proto,
};
use std::rc::Rc;

fn panel() -> Mesh {
    let mut mesh = Mesh::new();
    for (key, p) in [11, 70, 803, 9004].into_iter().zip([
        [-900., -650., 0.],
        [900., -650., 0.],
        [900., 650., 0.],
        [-900., 650., 0.],
    ]) {
        mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
    }
    mesh.add_face(vec![11, 70, 803, 9004], Some(17));
    mesh.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    mesh.set_linecolors(vec![], vec![7.]);
    mesh
}
fn camera() -> crate::camera::Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3000.;
    c.radius = 1500.;
    c.aspect = 4. / 3.;
    c
}
fn projected(m: [f32; 16], p: [f64; 3]) -> [f64; 2] {
    let c: [f64; 4] = std::array::from_fn(|i| {
        m[12 + i] as f64
            + (0..3)
                .map(|j| m[j * 4 + i] as f64 * p[j])
                .sum::<f64>()
    });
    [
        (c[0] / c[3] + 1.) * 160.,
        (1. - c[1] / c[3]) * 120.,
    ]
}
fn rgba(image: &[u8], x: usize, y: usize) -> &[u8] {
    crate::cap_gpu_tests::at(image, x, y)
}
fn corner(
    image: &[u8],
    m: [f32; 16],
    p: [f64; 3],
    outside: [f64; 2],
    density: f32,
) -> [usize; 2] {
    let at = projected(m, p);
    let sample = std::array::from_fn(|i| {
        if outside[i] < 0. {
            at[i].floor() as usize - density as usize
        } else {
            at[i].ceil() as usize + density as usize - 1
        }
    });
    let d: [f64; 2] = std::array::from_fn(|i| sample[i] as f64 + 0.5 - at[i]);
    assert!((0..2).all(|i| d[i] * outside[i] > 0.49));
    assert!(d[0].hypot(d[1]) < 3.5 * density as f64 - 0.75);
    assert_eq!(
        rgba(image, sample[0], sample[1]),
        [0, 0, 0, 255],
        "Actual complete round corner, not the old exterior butt gap"
    );
    sample
}
fn pick(
    gpu: &mut super::Renderer,
    scene: &Scene,
    m: [f32; 16],
    pixel: [usize; 2],
    mode: Mode,
) -> Option<Target> {
    gpu.set_pick_mode(mode);
    gpu.request_pick(
        [
            (pixel[0] as f32 + 0.5) / 160. - 1.,
            1. - (pixel[1] as f32 + 0.5) / 120.,
        ],
        m,
    )
    .unwrap();
    wait(gpu);
    gpu.take_pick(m)
        .unwrap()
        .unwrap()
        .and_then(|raw| gpu.resolve_pick(scene, raw))
}
fn source(scene: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let bundle =
        crate::archive::Bundle::decode(&crate::scene_archive::save(scene).unwrap())
            .unwrap();
    (
        bundle
            .documents
            .iter()
            .map(|b| proto::Session::decode(b.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(bundle.metadata).unwrap(),
    )
}
fn export_archive(name: &str, scene: &Scene) {
    if let Some(root) = std::env::var_os("VIEWER_ROUND_CAP_PROOF_DIR") {
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        std::fs::write(
            root.join(format!("{name}.session")),
            crate::scene_archive::save(scene).unwrap(),
        )
        .unwrap();
    }
}
fn export(name: &str, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_ROUND_CAP_PROOF_DIR") {
        use std::io::Write;
        let root = std::path::Path::new(&root);
        std::fs::create_dir_all(root).unwrap();
        let mut f = std::fs::File::create(root.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}

#[test]
fn original_mesh_surface_and_brep_round_corners_keep_source_queries() {
    for kind in ["mesh", "surface", "brep"] {
        let mut scene = Scene::empty();
        match kind {
            "mesh" => {
                scene
                    .insert(PreparedMesh::new(panel()).unwrap())
                    .unwrap();
            }
            "surface" => {
                let mut s = NurbsSurface::create(
                    false,
                    false,
                    1,
                    1,
                    2,
                    2,
                    &[
                        Point::new(-900., -650., 0.),
                        Point::new(-900., 650., 0.),
                        Point::new(900., -650., 0.),
                        Point::new(900., 650., 0.),
                    ],
                )
                .unwrap();
                s.width = 7.;
                s.facecolors = vec![Color::new(0.2, 0.4, 0.6, 1.)];
                scene.insert_surface(Rc::new(s)).unwrap();
            }
            _ => {
                let mut s = BRep::create_box(1800., 1300., 100.);
                s.width = 7.;
                s.surfacecolor = Color::new(0.2, 0.4, 0.6, 1.);
                scene.insert_cad(Rc::new(s)).unwrap();
            }
        }
        let original = source(&scene);
        let owner = scene.objects()[0].id;
        let mut gpu = renderer(&scene);
        gpu.set_opacity(1.);
        let uploads = gpu.stats()[0];
        let mut c = camera();
        for projection in [
            crate::camera::Projection::Orthographic,
            crate::camera::Projection::Perspective,
        ] {
            c.projection = projection;
            let m = c.uniform();
            for density in [1., 2.] {
                gpu.set_density(density);
                let before = pixels_at(&gpu, m);
                let z = if kind == "brep" {
                    50.
                } else {
                    0.
                };
                for (p, sign) in [
                    ([-900., 650., z], [-1., -1.]),
                    ([900., 650., z], [1., -1.]),
                    ([-900., -650., z], [-1., 1.]),
                    ([900., -650., z], [1., 1.]),
                ] {
                    let at = corner(&before, m, p, sign, density);
                    let target = pick(&mut gpu, &scene, m, at, Mode::Edge).unwrap();
                    assert_eq!(target.owner, owner);
                    assert!(matches!(
                        target.part,
                        Part::MeshEdge(..) | Part::SurfaceEdge(..) | Part::BRepEdge(..)
                    ));
                }
                let center = projected(m, [0., 0., z]).map(|v| v.floor() as usize);
                let face = pick(&mut gpu, &scene, m, center, Mode::Face).unwrap();
                assert_eq!(face.owner, owner);
                if kind == "mesh" {
                    assert_eq!(face.part, Part::Face(17));
                }
                assert!(pixels_at(&gpu, m) == before);
                assert_eq!(gpu.stats()[0], uploads);
                assert_eq!(source(&scene), original);
                if projection == crate::camera::Projection::Orthographic
                    && density == 1.
                {
                    export(&format!("{kind}-round-corners"), &before);
                    export_archive(&format!("{kind}-round-corners"), &scene);
                }
            }
        }
    }
}

fn line(a: [f64; 3], b: [f64; 3]) -> Scene {
    let mut s = Line::new(a[0], a[1], a[2], b[0], b[1], b[2]);
    s.width = 7.;
    s.linecolor = Color::new(0., 0., 0., 1.);
    let mut scene = Scene::empty();
    scene
        .insert_line(crate::stroke::PreparedLine::new(Rc::new(s)).unwrap())
        .unwrap();
    scene
}
#[test]
fn original_end_on_and_near_cut_lines_share_color_and_pick_coverage() {
    for density in [1., 2.] {
        let mut images = Vec::new();
        for (a, b) in [
            ([0., 0., -100.], [0., 0., 100.]),
            ([0., 0., 100.], [0., 0., -100.]),
        ] {
            let scene = line(a, b);
            let original = source(&scene);
            let owner = scene.lines()[0].id;
            let mut gpu = renderer(&scene);
            gpu.set_opacity(1.);
            gpu.set_density(density);
            let m = camera().uniform();
            let image = pixels_at(&gpu, m);
            let at = projected(m, [0., 0., 0.]).map(|v| v.floor() as usize);
            assert_eq!(
                rgba(&image, at[0], at[1]),
                [0, 0, 0, 255],
                "A nonzero original line viewed end-on remains a visible disk"
            );
            assert_eq!(
                pick(&mut gpu, &scene, m, at, Mode::Edge),
                Some(Target {
                    owner,
                    part: Part::Curve
                })
            );
            assert_eq!(source(&scene), original);
            if density == 1. {
                export_archive("end-on-original", &scene);
            }
            images.push(image);
        }
        assert!(
            images[0] == images[1],
            "Nearest reverse-depth end is independent of source endpoint order"
        );
        if density == 1. {
            export("end-on-original", &images[0]);
        }
        let scene = line([-0.6, 0., 0.5], [0.6, 0., 1.5]);
        let original = source(&scene);
        let mut gpu = renderer(&scene);
        gpu.set_opacity(1.);
        gpu.set_density(density);
        let m = Xform::identity().m.map(|v| v as f32);
        let image = pixels_at(&gpu, m);
        assert_eq!(rgba(&image, 150, 120), [0, 0, 0, 255]);
        assert_eq!(
            rgba(&image, 162, 120),
            [255, 255, 255, 255],
            "A clipped near-plane cut does not invent a round source end"
        );
        assert_eq!(source(&scene), original);
        if density == 1. {
            export("flat-near-cut", &image);
            export_archive("flat-near-cut", &scene);
        }
    }
}

#[test]
fn round_caps_keep_legacy_fallback_source_release_and_shared_geometry() {
    let mut document = Session::new("Original round caps");
    document.add_mesh(panel(), None);
    let mut editor = crate::editor::Editor::default();
    editor
        .apply(crate::editor::Action::ReplaceAt(
            document.to_proto().encode_to_vec(),
            Rc::new(crate::reload_url::ReloadUrl::new(
                "test:round-caps".into(),
            )),
        ))
        .unwrap();
    let owner = editor.scene.objects()[0].id;
    let original = source(&editor.scene);
    let display = Rc::clone(&editor.scene.objects()[0].mesh);
    let weak = Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    let mut gpu = renderer(&editor.scene);
    gpu.set_opacity(1.);
    let m = camera().uniform();
    let before = pixels_at(&gpu, m);
    let uploads = gpu.stats()[0];
    let metadata = gpu.mesh_ink.clipping_bytes();
    assert_eq!(metadata[0], 4 * 8);
    let placement = editor.scene.placement(owner).unwrap();
    editor
        .scene
        .place(
            owner,
            &Xform::translation(100., 0., 0.) * &placement,
        )
        .unwrap();
    gpu.set_scene(&editor.scene, None);
    assert_ne!(pixels_at(&gpu, m), before);
    editor.scene.place(owner, placement).unwrap();
    gpu.set_scene(&editor.scene, None);
    assert!(pixels_at(&gpu, m) == before);
    assert_eq!(gpu.stats()[0], uploads);
    assert_eq!(source(&editor.scene), original);
    gpu.mesh_ink.use_legacy_fixture();
    assert!(
        pixels_at(&gpu, m) == before,
        "Actual existing legacy pipeline keeps the same complete unoccluded boundary footprint"
    );
    assert_eq!(gpu.mesh_ink.clipping_bytes(), metadata);
    editor
        .apply(crate::editor::Action::UnloadSources)
        .unwrap();
    assert!(weak.upgrade().is_none());
    assert!(Rc::ptr_eq(
        &display,
        &editor.scene.objects()[0].mesh
    ));
    gpu.set_scene(&editor.scene, None);
    assert!(pixels_at(&gpu, m) == before);
    assert_eq!(gpu.stats()[0], uploads);
    let mesh = panel();
    let guid = mesh.guid().to_owned();
    let mut message = crate::instance_specimen::message();
    message.definitions.as_mut().unwrap().meshes = vec![mesh.to_proto()];
    for instance in &mut message.objects.as_mut().unwrap().instances {
        instance.definition_guid = guid.clone();
        instance.xform = None;
        instance.flags = 0;
    }
    message.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1200., 0., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1200., 0., 0.)
                    * &Xform::scale_xyz(-0.8, 1.2, 1.4))
                    .to_proto(),
            ),
        },
    ];
    let mut scene = Scene::empty();
    crate::scene_document::import(
        &mut scene,
        &message.encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert!(Rc::ptr_eq(
        &scene.objects()[0].mesh,
        &scene.objects()[1].mesh
    ));
    let original = source(&scene);
    let mut gpu = renderer(&scene);
    gpu.set_opacity(1.);
    let mut c = camera();
    c.distance = 6000.;
    c.radius = 3000.;
    let m = c.uniform();
    for density in [1., 2.] {
        gpu.set_density(density);
        let image = pixels_at(&gpu, m);
        corner(
            &image,
            m,
            [-2100., 650., 0.],
            [-1., -1.],
            density,
        );
        corner(
            &image,
            m,
            [1920., 780., 0.],
            [1., -1.],
            density,
        );
        assert_eq!(source(&scene), original);
        if density == 1. {
            export("shared-mirrored-round-corners", &image);
            export_archive("shared-mirrored-round-corners", &scene);
        }
    }
}
```

### 9. `src/renderer.rs`

Register Native validation only.

**Locate this existing block:**

```rust
#[path = "source_pen_tests.rs"]
mod source_pen_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[path = "source_pen_tests.rs"]
mod source_pen_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "round_caps_tests.rs"]
mod round_caps_tests;
```

### 10. `src/stroke_gpu.rs`

Exercise the actual existing legacy pipeline in Native fixtures.

**Locate this existing block:**

```rust
}

impl Lane {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

#[cfg(test)]
impl Lane {
    pub(crate) fn use_legacy_fixture(&mut self) {
        self.finite = None;
    }
}

impl Lane {
```

### 11. `src/cap_gpu_tests.rs`

Sample the unoccluded original back face, away from the now-visible end-on construction-plane marker. Keep the zero section-fill assertion.

**Locate this existing block:**

```rust
    assert_eq!(r.cap_usage()[1], 0);
    let center = at(&image, 160, 120);
    assert!(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(r.cap_usage()[1], 0);
    let center = at(&image, 208, 96);
    assert!(
```

### 12. `src/finite_visibility_tests.rs`

Retain exact clipped headed-chain comparison; distinguish genuine round source caps with exact body, flat-cut and bounded-cap pixel assertions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
            let (cropped, _, _) = make(true);
            gpu.set_scene(&cropped, None);
            assert_eq!(
                red_pixels(&gpu, size),
                pixels,
                "homogeneous near clipping matches the explicit segment and invents no arrowhead at DPR{density}"
            );
        }
```

Replace it with:

```rust
            }
            let (cropped, _, _) = make(true);
            gpu.set_scene(&cropped, None);
            if kind == 1 {
                assert_eq!(
                    red_pixels(&gpu, size),
                    pixels,
                    "homogeneous near clipping matches the explicit headed segment and invents no arrowhead at DPR{density}"
                );
            } else {
                let explicit = frame_pixels(&gpu, size, transform);
                let mut changed = 0;
                for (i, (cut, round)) in original_pixels
                    .chunks_exact(4)
                    .zip(explicit.chunks_exact(4))
                    .enumerate()
                {
                    let x = i % size[0] as usize;
                    let y = i / size[0] as usize;
                    if x >= size[0] as usize / 2 {
                        assert_eq!(
                            cut, round,
                            "Every visible body and genuine far-end pixel matches the explicit segment"
                        );
                    } else {
                        assert_eq!(
                            cut,
                            [255, 255, 255, 255],
                            "A camera cut has no invented source cap"
                        );
                    }
                    if cut != round {
                        changed += 1;
                        assert!(x < size[0] as usize / 2);
                        assert!(
                            (x as f64 + 0.5 - size[0] as f64 / 2.)
                                .hypot(y as f64 + 0.5 - size[1] as f64 / 2.)
                                < 2. * density as f64 + 0.5,
                            "Only the genuine explicit source cap differs"
                        );
                    }
                }
                assert!(changed > 0);
                assert!(
                    red_pixels(&gpu, size) > pixels,
                    "A genuine round source end adds its own cap; the camera cut stays flat"
                );
            }
        }
```

**Locate this block:**

```rust
            );
            let pixels = red_pixels(&gpu, size);
            assert!(pixels > 100);
            if kind == 0 {
                assert!(Rc::ptr_eq(
                    &original.lines()[0].prepared.source,
```

Replace it with:

```rust
            );
            let pixels = red_pixels(&gpu, size);
            assert!(pixels > 100);
            let transform = crate::cap_gpu_tests::reversed(
                session_rust::Xform::identity()
                    .m
                    .map(|v| v as f32),
            );
            let original_pixels = frame_pixels(&gpu, size, transform);
            if kind == 0 {
                assert!(Rc::ptr_eq(
                    &original.lines()[0].prepared.source,
```

**Locate this block:**

```rust
    wait(renderer);
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range();
    let count = bytes
        .chunks_exact(pitch as usize)
        .flat_map(|row| row[..size[0] as usize * 4].chunks_exact(4))
        .filter(|p| p[0] > 200 && p[1] < 50 && p[2] < 50)
        .count();
    drop(bytes);
    read.unmap();
    count
}
fn picked(renderer: &mut Renderer, at: [f32; 2]) -> Option<[u32; 2]> {
    pick_frame(
```

Replace it with:

```rust
    wait(renderer);
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range();
    let pixels = bytes
        .chunks_exact(pitch as usize)
        .flat_map(|row| row[..size[0] as usize * 4].iter().copied())
        .collect();
    drop(bytes);
    read.unmap();
    pixels
}
fn picked(renderer: &mut Renderer, at: [f32; 2]) -> Option<[u32; 2]> {
    pick_frame(
```

**Locate this block:**

```rust
fn red_frame(renderer: &Renderer, size: [u32; 2], matrix: [f32; 16]) -> usize {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
```

Replace it with:

```rust
fn red_frame(renderer: &Renderer, size: [u32; 2], matrix: [f32; 16]) -> usize {
    frame_pixels(renderer, size, matrix)
        .chunks_exact(4)
        .filter(|p| p[0] > 200 && p[1] < 50 && p[2] < 50)
        .count()
}
fn frame_pixels(renderer: &Renderer, size: [u32; 2], matrix: [f32; 16]) -> Vec<u8> {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
```

## <a id="checkpoint-96zzzzzzp-draw-default-mesh-vertex-dots"></a>3. Draw default mesh vertex dots

**What this code does:** Render the maintained default and authored mesh vertex dots without duplicating source meshes or endpoint positions.

### 1. `src/mesh_pens.rs`

Prepare numeric endpoint references and only authored point colors on the existing display owner. Printed and dense fills have no default vertex decoration. Reuse the same source topology and sparse slot map once. A nonzero authored source edge makes its endpoints eligible, including a removed coplanar seam. Default dark color needs no per-vertex color allocation; common authored colors retain one entry.

**Locate this existing block:**

```rust
}

pub fn prepare(source: &Mesh, count: usize) -> (Vec<(usize, usize)>, Vec<Pen>) {
    if count > 200_000 || source.widths() == [0.] {
        return (Vec::new(), Vec::new());
    }
    let keys = source.vertices();
    let points = keys
        .iter()
        .map(|key| {
            let p = &source.vertex[key];
            [p.x, p.y, p.z]
        })
        .collect::<Vec<_>>();
    let topology = mesh_topology(source, &keys, &points, &SlotMap::new(&keys));
    let mut pens = Vec::with_capacity(topology.edges.len());
    for (i, &(_, _, colour)) in topology.edges.iter().enumerate() {
        let widths = source.widths();
        let value = if widths.len() == 1 {
            widths[0]
        } else {
            widths.get(i).copied().unwrap_or(1.)
        };
        let pair = topology.edge_faces[i].map(|face| {
            topology
                .normals
                .get(face as usize)
                .copied()
                .flatten()
        });
        let flat = match pair {
            [Some(a), Some(b)] => {
                a.iter()
                    .zip(b)
                    .map(|(a, b)| a * b)
                    .sum::<f64>()
                    * if topology.opposed[i] {
                        1.
                    } else {
                        -1.
                    }
                    >= 1. - 1e-9
            }
            _ => false,
        };
        pens.push((
            if value == 0. || flat {
                0.
            } else {
                width(value)
            },
            if topology.edges.len() >= 10_000 {
                crate::source_topology_colour::BLACK
            } else {
                colour
            },
        ));
    }
    if pens
        .first()
        .is_some_and(|first| pens.iter().all(|pen| pen == first))
    {
        pens = vec![pens[0]];
    }
    (
        topology
            .edges
            .into_iter()
            .map(|(a, b, _)| (a, b))
            .collect(),
        pens,
    )
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}

pub fn prepare(
    source: &Mesh,
    count: usize,
) -> (
    Vec<(usize, usize)>,
    Vec<Pen>,
    Vec<u32>,
    Vec<[f32; 4]>,
) {
    if count > 200_000 || source.widths() == [0.] {
        return (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
    }
    let keys = source.vertices();
    let points = keys
        .iter()
        .map(|key| {
            let p = &source.vertex[key];
            [p.x, p.y, p.z]
        })
        .collect::<Vec<_>>();
    let slots = SlotMap::new(&keys);
    let topology = mesh_topology(source, &keys, &points, &slots);
    let mut vertices = vec![u32::MAX; keys.len()];
    let mut pens = Vec::with_capacity(topology.edges.len());
    for (i, &(_, _, colour)) in topology.edges.iter().enumerate() {
        let widths = source.widths();
        let value = if widths.len() == 1 {
            widths[0]
        } else {
            widths.get(i).copied().unwrap_or(1.)
        };
        if value != 0. {
            for (side, key) in [topology.edges[i].0, topology.edges[i].1]
                .into_iter()
                .enumerate()
            {
                let dot = &mut vertices[slots.slot(key)];
                if *dot == u32::MAX {
                    *dot = (i * 2 + side) as u32;
                }
            }
        }
        let pair = topology.edge_faces[i].map(|face| {
            topology
                .normals
                .get(face as usize)
                .copied()
                .flatten()
        });
        let flat = match pair {
            [Some(a), Some(b)] => {
                a.iter()
                    .zip(b)
                    .map(|(a, b)| a * b)
                    .sum::<f64>()
                    * if topology.opposed[i] {
                        1.
                    } else {
                        -1.
                    }
                    >= 1. - 1e-9
            }
            _ => false,
        };
        pens.push((
            if value == 0. || flat {
                0.
            } else {
                width(value)
            },
            if topology.edges.len() >= 10_000 {
                crate::source_topology_colour::BLACK
            } else {
                colour
            },
        ));
    }
    if pens
        .first()
        .is_some_and(|first| pens.iter().all(|pen| pen == first))
    {
        pens = vec![pens[0]];
    }
    let mut dots = Vec::new();
    let mut colours = Vec::new();
    let pc = source.get_pointcolors();
    let coloured = source.color_mode == session_rust::mesh::ColorMode::POINTCOLORS
        && pc.len() == keys.len();
    for (slot, edge) in vertices.into_iter().enumerate() {
        if edge != u32::MAX {
            dots.push(edge);
            if coloured {
                colours.push(pc[slot].to_f32());
            }
        }
    }
    if colours
        .first()
        .is_some_and(|c| colours.iter().all(|v| v == c))
    {
        colours = vec![colours[0]];
    }
    (
        topology
            .edges
            .into_iter()
            .map(|(a, b, _)| (a, b))
            .collect(),
        pens,
        dots,
        colours,
    )
```

### 2. `src/mesh_parts.rs`

Retain four-byte references into existing f64 endpoints rather than duplicate positions or original meshes. Carry the prepared numerical decoration through source release. Keep the original shared display owner. Read authored point colors or the maintained default exactly, without byte quantization. Count actual retained numeric decoration capacity.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        )
    }

    pub fn bytes(&self) -> usize {
        self.pens.capacity() * std::mem::size_of::<crate::mesh_pens::Pen>()
            + self.faces.capacity() * std::mem::size_of::<usize>()
```

Replace it with:

```rust
        )
    }

    pub fn dot_colour(&self, index: usize) -> [f32; 4] {
        self.dot_colours
            .get(if self.dot_colours.len() == 1 {
                0
            } else {
                index
            })
            .copied()
            .unwrap_or([0.1, 0.1, 0.1, 1.])
    }

    pub fn bytes(&self) -> usize {
        self.dots.capacity() * 4
            + self.dot_colours.capacity() * 16
            + self.pens.capacity() * std::mem::size_of::<crate::mesh_pens::Pen>()
            + self.faces.capacity() * std::mem::size_of::<usize>()
```

**Locate this block:**

```rust
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self {
            pens,
            faces,
            triangles,
```

Replace it with:

```rust
            endpoints.push([[a[0], a[1], a[2]], [b[0], b[1], b[2]]]);
        }
        Ok(Self {
            dots,
            dot_colours,
            pens,
            faces,
            triangles,
```

**Locate this block:**

```rust
        if triangles.len() != count {
            return Err("Source faces do not match the rendered triangles");
        }
        let (edges, pens) = crate::mesh_pens::prepare(source, count);
        let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source
```

Replace it with:

```rust
        if triangles.len() != count {
            return Err("Source faces do not match the rendered triangles");
        }
        let (edges, pens, dots, dot_colours) = crate::mesh_pens::prepare(source, count);
        let mut endpoints = Vec::new();
        for &(a, b) in &edges {
            let a = source
```

**Locate this block:**

```rust
pub struct Parts {
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
```

Replace it with:

```rust
pub struct Parts {
    pub dots: Vec<u32>,
    pub dot_colours: Vec<[f32; 4]>,
    pub pens: Vec<crate::mesh_pens::Pen>,
    pub faces: Vec<usize>,
    pub triangles: Vec<u32>,
```

### 3. `src/bounds_original.rs`

Bounding frames do not invent mesh vertex markers.

**Locate this existing block:**

```rust
        endpoints,
        pens: Vec::new(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        endpoints,
        dots: Vec::new(),
        dot_colours: Vec::new(),
        pens: Vec::new(),
```

### 4. `src/plane_original.rs`

Original planes retain their own frame glyph policy.

**Locate this existing block:**

```rust
    display.parts = crate::mesh_parts::Parts {
        pens: Vec::new(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    display.parts = crate::mesh_parts::Parts {
        dots: Vec::new(),
        dot_colours: Vec::new(),
        pens: Vec::new(),
```

### 5. `src/pick_sources.rs`

Build ordinary point instances with object ownership and no new editable component domain. Transform numerical endpoints after origin anchoring; BRep, NURBS and derived contact shapes keep their own display policy. Account for temporary marker preparation; move it into the existing upload.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity())
            * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_metadata.capacity() * 4
```

Replace it with:

```rust
        (self.faces.capacity() + self.edges.capacity() + self.controls.capacity())
            * std::mem::size_of::<Target>()
            + self.mesh_bases.capacity() * std::mem::size_of::<(ObjectId, u32)>()
            + self.mesh_markers.capacity()
                * std::mem::size_of::<crate::marker::Marker>()
            + self.mesh_metadata.capacity() * 4
```

**Locate this block:**

```rust
                    (0..3).map(|j| m[j * 4 + i] * n[j]).sum()
                }));
            }
            let flag = if planes.contains(&row.id) {
                4096
            } else {
```

Replace it with:

```rust
                    (0..3).map(|j| m[j * 4 + i] * n[j]).sum()
                }));
            }
            if row.cad.is_none() && row.surface.is_none() && !derived {
                for (i, &address) in row.mesh.parts.dots.iter().enumerate() {
                    let p = row.mesh.parts.endpoints[address as usize / 2]
                        [address as usize % 2];
                    sources
                        .mesh_markers
                        .push(crate::marker::Marker {
                            center: model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32(),
                            diameter: 1.,
                            colour: edge_colour
                                .unwrap_or_else(|| row.mesh.parts.dot_colour(i)),
                        });
                    sources.marker_ids.push([owner, 0]);
                }
            }
            let flag = if planes.contains(&row.id) {
                4096
            } else {
```

**Locate this block:**

```rust
    pub path_ids: Vec<[u32; 2]>,
    pub marker_ids: Vec<[u32; 2]>,
}
```

Replace it with:

```rust
    pub path_ids: Vec<[u32; 2]>,
    pub mesh_markers: Vec<crate::marker::Marker>,
    pub marker_ids: Vec<[u32; 2]>,
}
```

### 6. `src/renderer.rs`

Upload through the existing point buffer once, releasing temporary CPU instances. Match default mesh, original point and active control ownership order.

**Locate this existing block:**

```rust
                &self.device,
                scene
                    .points()
                    .iter()
                    .filter(|row| scene.visible(row.id))
                    .map(|row| {
                        let mut marker = row.marker_at(anchor);
                        if let Some(colour) = scene.face_rgba(row.id) {
                            marker.colour = colour;
                        }
                        marker
                    })
                    .chain(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                &self.device,
                std::mem::take(&mut self.sources.mesh_markers)
                    .into_iter()
                    .chain(
                        scene
                            .points()
                            .iter()
                            .filter(|row| scene.visible(row.id))
                            .map(|row| {
                                let mut marker = row.marker_at(anchor);
                                if let Some(colour) = scene.face_rgba(row.id) {
                                    marker.colour = colour;
                                }
                                marker
                            }),
                    )
                    .chain(
```

### 7. `src/mesh_marker_tests.rs`

Exercise actual GPU dots, source keys, authored colors, incidence and shared source unloading.

Create the file and type:

```rust
use crate::{
    cap_gpu_tests::{at, pixels_at, renderer, wait},
    pick_sources::{Mode, Part, Sources, Target},
    prepared::PreparedMesh,
    scene::Scene,
};
use prost::Message;
use session_rust::mesh::ColorMode;
use session_rust::{Color, Mesh, Point, Session, Xform, proto};
use std::rc::Rc;
fn panel() -> Mesh {
    let mut m = Mesh::new();
    for (key, p) in [11, 70, 803, 9004].into_iter().zip([
        [0., 0., 0.],
        [1800., 0., 0.],
        [1800., 1300., 0.],
        [0., 1300., 0.],
    ]) {
        m.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
    }
    m.add_face(vec![11, 70, 803, 9004], Some(17));
    m.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    m.set_linecolors(vec![], vec![7.]);
    m
}
fn scene(m: Mesh) -> Scene {
    let mut s = Scene::empty();
    s.insert(PreparedMesh::new(m).unwrap())
        .unwrap();
    s
}
fn camera() -> crate::camera::Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3000.;
    c.radius = 1500.;
    c.aspect = 4. / 3.;
    c
}
fn matrix(c: &crate::camera::Camera) -> [f32; 16] {
    let mut m = c.uniform();
    m[12] += m[15] / 320.;
    m[13] -= m[15] / 240.;
    m
}
fn pick(
    g: &mut super::Renderer,
    s: &Scene,
    m: [f32; 16],
    p: [f64; 3],
    mode: Mode,
) -> Option<Target> {
    g.set_pick_mode(mode);
    g.request_pick(
        super::contact_feature_tests::project(m, p),
        m,
    )
    .unwrap();
    wait(g);
    g.take_pick(m)
        .unwrap()
        .unwrap()
        .and_then(|v| g.resolve_pick(s, v))
}
fn source(s: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap())
        .unwrap();
    (
        b.documents
            .iter()
            .map(|v| proto::Session::decode(v.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn export(name: &str, s: &Scene, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_MESH_MARKER_PROOF_DIR") {
        use std::io::Write;
        let p = std::path::Path::new(&root);
        std::fs::create_dir_all(p).unwrap();
        std::fs::write(
            p.join(format!("{name}.session")),
            crate::scene_archive::save(s).unwrap(),
        )
        .unwrap();
        let mut f = std::fs::File::create(p.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}

#[test]
fn actual_default_and_authored_mesh_vertices_keep_original_queries() {
    for kind in ["default", "authored", "ignored"] {
        let mut mesh = panel();
        if kind != "default" {
            mesh.set_pointcolors(vec![
                Color::new(0.8, 0.2, 0.4, 1.),
                Color::new(0.1, 0.9, 0.2, 1.),
                Color::new(0.2, 0.3, 0.9, 1.),
                Color::new(0.9, 0.8, 0.1, 1.),
            ]);
            if kind == "ignored" {
                mesh.color_mode = ColorMode::OBJECTCOLOR;
            }
        }
        let s = scene(mesh);
        let owner = s.objects()[0].id;
        let original = source(&s);
        let parts = &s.objects()[0].mesh.parts;
        assert_eq!(parts.dots.len(), 4);
        assert_eq!(
            parts.dot_colours.len(),
            if kind == "authored" {
                4
            } else {
                0
            }
        );
        assert_eq!(
            parts.dots.capacity() * 4,
            16,
            "Default markers retain endpoint addresses, not duplicate positions"
        );
        let mut g = renderer(&s);
        g.set_opacity(1.);
        g.set_lighting(false, false);
        assert_eq!(g.point_usage()[0], 4);
        assert_eq!(
            g.markers.clipping_bytes(),
            [4, 4],
            "Default mesh and ordinary markers retain one common four-byte metadata record"
        );
        assert!(
            g.sources.mesh_markers.is_empty() && g.sources.mesh_markers.capacity() == 0,
            "Temporary point instances move into the existing upload"
        );
        let uploads = g.stats()[0];
        let mut c = camera();
        for projection in [
            crate::camera::Projection::Orthographic,
            crate::camera::Projection::Perspective,
        ] {
            c.projection = projection;
            let m = matrix(&c);
            for density in [1., 2.] {
                g.set_density(density);
                let image = pixels_at(&g, m);
                assert_eq!(
                    at(&image, 160, 120),
                    if kind == "authored" {
                        &[204, 51, 102, 255]
                    } else {
                        &[26, 26, 26, 255]
                    },
                    "Actual original vertex11 has its maintained default/authored point color"
                );
                assert_eq!(
                    pick(&mut g, &s, m, [0., 0., 0.], Mode::Object),
                    Some(Target {
                        owner,
                        part: Part::Object
                    })
                );
                assert_eq!(
                    pick(&mut g, &s, m, [900., 650., 0.], Mode::Face),
                    Some(Target {
                        owner,
                        part: Part::Face(17)
                    })
                );
                assert_eq!(
                    pick(&mut g, &s, m, [900., 0., 0.], Mode::Edge),
                    Some(Target {
                        owner,
                        part: Part::MeshEdge(11, 70)
                    })
                );
                assert_eq!(
                    pick(&mut g, &s, m, [0., 0., 0.], Mode::Control),
                    None,
                    "Default dots are not active editing controls"
                );
                assert!(pixels_at(&g, m) == image);
                assert_eq!(g.stats()[0], uploads);
                assert_eq!(source(&s), original);
                if projection == crate::camera::Projection::Orthographic
                    && density == 1.
                {
                    export(kind, &s, &image);
                }
            }
        }
    }
}

#[test]
fn source_widths_coplanar_vertices_and_other_families_control_default_dots() {
    let mut m = panel();
    m.add_vertex(Point::new(-200., -200., 0.), Some(999999));
    m.set_linecolors(vec![], vec![0., 0., 7., 0.]);
    let s = scene(m);
    assert_eq!(
        s.objects()[0].mesh.parts.dots.len(),
        2,
        "Only the nonzero authored edge contributes endpoints; an isolated source vertex is not a dot"
    );
    let mut g = renderer(&s);
    g.set_opacity(1.);
    assert_eq!(g.point_usage()[0], 2);
    let original = source(&s);
    let m = matrix(&camera());
    let image = pixels_at(&g, m);
    assert_eq!(source(&s), original);
    export("eligible-edge", &s, &image);
    for widths in [vec![0.], vec![0.; 4]] {
        let mut mesh = panel();
        mesh.set_linecolors(vec![], widths);
        let s = scene(mesh);
        assert!(s.objects()[0].mesh.parts.dots.is_empty());
        assert!(
            s.objects()[0]
                .mesh
                .parts
                .dot_colours
                .is_empty()
        );
        g.set_scene(&s, None);
        assert_eq!(g.point_usage()[0], 0);
        let before = source(&s);
        let image = pixels_at(&g, m);
        assert_eq!(source(&s), before);
        export("no-dots", &s, &image);
    }
    let mut fan = Mesh::new();
    for (key, p) in [11, 70, 803, 9004, 500000].into_iter().zip([
        [-900., -650., 0.],
        [900., -650., 0.],
        [900., 650., 0.],
        [-900., 650., 0.],
        [0., 0., 0.],
    ]) {
        fan.add_vertex(Point::new(p[0], p[1], p[2]), Some(key));
    }
    for (face, ring) in [
        (17, vec![11, 70, 500000]),
        (93, vec![70, 803, 500000]),
        (125, vec![803, 9004, 500000]),
        (302, vec![9004, 11, 500000]),
    ] {
        fan.add_face(ring, Some(face));
    }
    fan.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    fan.set_linecolors(vec![], vec![7.]);
    let s = scene(fan);
    let original = source(&s);
    assert_eq!(
        s.objects()[0].mesh.parts.dots.len(),
        5,
        "A source vertex on nonzero coplanar edges remains a maintained dot even when its seam ink is omitted"
    );
    g.set_scene(&s, None);
    let image = pixels_at(&g, m);
    assert_eq!(at(&image, 160, 120), [26, 26, 26, 255]);
    assert_eq!(source(&s), original);
    export("coplanar-center", &s, &image);
    for other in ["surface", "brep", "plane"] {
        let mut s = Scene::empty();
        match other {
            "surface" => {
                s.insert_surface(Rc::new(crate::surface::example()))
                    .unwrap();
            }
            "brep" => {
                s.insert_cad(Rc::new(session_rust::BRep::create_box(
                    1800., 1300., 100.,
                )))
                .unwrap();
            }
            _ => {
                crate::plane_original::insert(
                    &mut s,
                    Rc::new(session_rust::Plane::xy_plane()),
                )
                .unwrap();
            }
        }
        g.set_scene(&s, None);
        pixels_at(&g, m);
        assert_eq!(
            g.point_usage()[0],
            0,
            "Other source families keep their own authored glyph policy"
        );
    }
}

#[test]
fn prepared_marker_addresses_survive_source_unload_and_shared_placement() {
    let mut doc = Session::new("Original marker source");
    doc.add_mesh(panel(), None);
    let bytes = doc.to_proto().encode_to_vec();
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::ReplaceAt(
        bytes.clone(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:mesh-markers".into(),
        )),
    ))
    .unwrap();
    let owner = e.scene.objects()[0].id;
    let display = Rc::clone(&e.scene.objects()[0].mesh);
    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    let original = source(&e.scene);
    let mut g = renderer(&e.scene);
    g.set_opacity(1.);
    let m = matrix(&camera());
    let before = pixels_at(&g, m);
    let uploads = g.stats()[0];
    let old = e.scene.placement(owner).unwrap();
    e.scene
        .place(
            owner,
            &Xform::translation(100., 0., 0.) * &old,
        )
        .unwrap();
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) != before);
    e.scene.place(owner, old).unwrap();
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == before);
    assert_eq!(g.stats()[0], uploads);
    assert_eq!(source(&e.scene), original);
    e.apply(crate::editor::Action::UnloadSources)
        .unwrap();
    assert!(weak.upgrade().is_none());
    assert!(Rc::ptr_eq(
        &display,
        &e.scene.objects()[0].mesh
    ));
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == before);
    assert_eq!(g.point_usage()[0], 4);
    assert_eq!(g.stats()[0], uploads);
    let keys = e.reload_keys();
    e.hydrate(
        keys.into_iter()
            .map(|k| (k, bytes.clone()))
            .collect(),
    )
    .unwrap();
    assert_eq!(source(&e.scene), original);
    for (index, projection) in [
        crate::camera::Projection::Orthographic,
        crate::camera::Projection::Perspective,
    ]
    .into_iter()
    .enumerate()
    {
        let mut c = camera();
        c.projection = projection;
        c.grid_plane = Some(crate::cplane::CPlane::Frame {
            origin: [0.; 3],
            x: [1., 0., 0.],
            y: [0., 1., 0.],
        });
        let now = 1000. + index as f64 * 1000.;
        g.set_scene(&e.scene, None);
        g.prepare_origin(&c, &e.scene, None, &[], now)
            .unwrap();
        let near = g.camera_matrix(&c);
        let before = pixels_at(&g, near);
        let uploads = g.stats()[0];
        let marker_count = g.point_usage()[0];
        let mut far = e.scene.clone();
        let anchor = [
            1_000_000_000.,
            -1_000_000_000.,
            1_000_000_000.,
        ];
        far.translate(owner, anchor).unwrap();
        g.set_scene(&far, None);
        c.target = anchor;
        c.grid_plane = Some(crate::cplane::CPlane::Frame {
            origin: anchor,
            x: [1., 0., 0.],
            y: [0., 1., 0.],
        });
        assert!(
            g.prepare_origin(&c, &far, None, &[], now + 300.)
                .unwrap()
                .moved
        );
        assert_eq!(g.camera_matrix(&c), near);
        let image = pixels_at(&g, near);
        assert!(
            image == before,
            "Actual production origin preparation preserves all mesh, ink and marker pixels at a billion units"
        );
        assert_eq!(g.point_usage()[0], marker_count);
        assert_eq!(marker_count, 4);
        assert_eq!(g.stats()[0], uploads);
        assert_eq!(
            g.sources.marker_ids,
            vec![[owner.value(), 0]; 4]
        );
        g.set_pick_mode(Mode::Object);
        g.request_pick(
            super::contact_feature_tests::project(near, [0., 0., 0.]),
            near,
        )
        .unwrap();
        wait(&g);
        assert_eq!(
            g.take_pick(near).unwrap().unwrap().unwrap()[0],
            owner.value()
        );
        if index == 0 {
            export("rebased-default", &far, &image);
        }
    }
    let mesh = panel();
    let guid = mesh.guid().to_owned();
    let mut message = crate::instance_specimen::message();
    message.definitions.as_mut().unwrap().meshes = vec![mesh.to_proto()];
    for row in &mut message.objects.as_mut().unwrap().instances {
        row.definition_guid = guid.clone();
        row.xform = None;
        row.flags = 0;
    }
    message.xforms = vec![
        proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1800., -650., 0.).to_proto()),
        },
        proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1800., -650., 0.)
                    * &Xform::scale_xyz(-0.8, 1.2, 1.4))
                    .to_proto(),
            ),
        },
    ];
    let mut s = Scene::empty();
    crate::scene_document::import(
        &mut s,
        &message.encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert!(Rc::ptr_eq(
        &s.objects()[0].mesh,
        &s.objects()[1].mesh
    ));
    let original = source(&s);
    g.set_scene(&s, None);
    assert_eq!(g.point_usage()[0], 8);
    let mut c = camera();
    c.distance = 6000.;
    c.radius = 3000.;
    assert!(
        g.prepare_origin(&c, &s, None, &[], 3000.)
            .unwrap()
            .moved
    );
    let m = matrix(&c);
    let image = pixels_at(&g, m);
    assert!(
        image
            .chunks_exact(4)
            .filter(|p| p[2] > p[0].saturating_add(50))
            .count()
            > 100,
        "Both actual placed panels must render after restoring the origin"
    );
    for (i, p) in [[-900., 0., 0.], [1080., 130., 0.]]
        .into_iter()
        .enumerate()
    {
        assert_eq!(
            pick(&mut g, &s, m, p, Mode::Face),
            Some(Target {
                owner: s.objects()[i].id,
                part: Part::Face(17)
            })
        );
    }
    assert!(pixels_at(&g, m) == image);
    export("shared-markers", &s, &image);
    assert_eq!(source(&s), original);
    let id = s.objects()[0].id;
    let old = s.placement(id).unwrap();
    s.place(
        id,
        Xform::translation(1_000_000_000.125, 0., 0.),
    )
    .unwrap();
    let prepared = Sources::anchored(&s, [1_000_000_000., 0., 0.]).unwrap();
    assert_eq!(
        prepared.mesh_markers[0].center[0], 0.125,
        "Transform after anchoring preserves the source endpoint fraction"
    );
    s.place(id, old).unwrap();
}
```

### 8. `src/renderer.rs`

Register the three actual mesh marker GPU checks.

**Locate this existing block:**

```rust
mod source_pen_tests;

#[cfg(all(test, not(target_arch = "wasm32")))]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod source_pen_tests;

#[cfg(test)]
#[path = "mesh_marker_tests.rs"]
mod mesh_marker_tests;
#[cfg(all(test, not(target_arch = "wasm32")))]
```

### 9. `src/source_pen_tests.rs`

Extend the existing real 200000-triangle budget case without duplicating its dense fixture.

**Locate this existing block:**

```rust
    assert!(scene.objects()[0].mesh.parts.pens.is_empty());
    gpu.set_scene(&scene, None);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert!(scene.objects()[0].mesh.parts.pens.is_empty());
    assert!(scene.objects()[0].mesh.parts.dots.is_empty());
    assert!(
        scene.objects()[0]
            .mesh
            .parts
            .dot_colours
            .is_empty()
    );
    gpu.set_scene(&scene, None);
```

### 10. `src/global_outline_tests.rs`

This existing fixture manually isolates surface-buffer rebasing; suppress both original ink and new default dots while preserving every surface/grid/cap/pick/source assertion. Production complete-lane marker rebasing is separately exercised at a billion units.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    let mut far = scene.clone();
    far.translate(owner, anchor).unwrap();
    gpu.set_scene(&far, None);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
```

Replace it with:

```rust
    let mut far = scene.clone();
    far.translate(owner, anchor).unwrap();
    gpu.set_scene(&far, None);
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    gpu.markers
        .set_points(&gpu.device, std::iter::empty())
        .unwrap();
```

**Locate this block:**

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let settings = Some(crate::grid::Settings {
        spacing: 0.1,
        half_cells: 5,
```

Replace it with:

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    gpu.markers
        .set_points(&gpu.device, std::iter::empty())
        .unwrap();
    let settings = Some(crate::grid::Settings {
        spacing: 0.1,
        half_cells: 5,
```

**Locate this block:**

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    assert!(
        gpu.space
            .update(anchor, 1., 0.)
```

Replace it with:

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    gpu.markers
        .set_points(&gpu.device, std::iter::empty())
        .unwrap();
    assert!(
        gpu.space
            .update(anchor, 1., 0.)
```

**Locate this block:**

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    let baseline = pixels(&gpu);
    assert!(
        crate::cap_gpu_tests::at(&baseline, 160, 120)[..3]
```

Replace it with:

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    gpu.markers
        .set_points(&gpu.device, std::iter::empty())
        .unwrap();
    let baseline = pixels(&gpu);
    assert!(
        crate::cap_gpu_tests::at(&baseline, 160, 120)[..3]
```

**Locate this block:**

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    assert_eq!(
        gpu.meshes[0].rebase(&gpu.queue, anchor),
        [1, 12]
```

Replace it with:

```rust
    gpu.mesh_ink
        .set(&gpu.device, std::iter::empty())
        .unwrap();
    gpu.markers
        .set_points(&gpu.device, std::iter::empty())
        .unwrap();
    assert_eq!(
        gpu.meshes[0].rebase(&gpu.queue, anchor),
        [1, 12]
```

**Locate this block:**

```rust
        .unwrap();
    let baseline = pixels(&gpu);
    assert!(
        baseline
```

Replace it with:

```rust
        .unwrap();
    gpu.markers
        .set_points(&gpu.device, std::iter::empty())
        .unwrap();
    let baseline = pixels(&gpu);
    assert!(
        baseline
```

### 11. `src/scene_origin_tests.rs`

Keep the original point/control count exact after the new eight default Box vertex markers; verify all marker values, owners, ordering and large-origin equality independently.

**Locate this existing block:**

```rust
    );
    assert_eq!(
        current.marker_ids.len(),
        far.points().len() + far.control_markers_at(anchor).len()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    );
    assert_eq!(current.mesh_markers.len(), 8);
    assert_eq!(
        current
            .mesh_markers
            .iter()
            .map(|m| m.values())
            .collect::<Vec<_>>(),
        baseline
            .mesh_markers
            .iter()
            .map(|m| m.values())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        &current.marker_ids[..8],
        &vec![[far.objects()[0].id.value(), 0]; 8]
    );
    assert_eq!(
        current.marker_ids[8..].len(),
        far.points().len() + far.control_markers_at(anchor).len()
```

## <a id="checkpoint-96zzzzzzq-taper-crowded-original-mesh-vertices"></a>4. Taper crowded mesh vertices

**What this code does:** Keep dense source vertices distinguishable while retaining explicit point sizes, editing controls and original ownership.

### 1. `src/mesh.rs`

Retain one prepared mesh spacing scalar after original source release. Only actual original default-dot meshes need a spacing estimate. Use actual render bounds and original vertex count once, matching the maintained per-mesh estimate; skip dense/printed fills without dots.

**Locate this existing block:**

```rust
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 6]>,
        indices: Vec<u32>,
    ) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err(
                "Indices must describe complete triangles within the draw limit",
            );
        }
        if indices
            .iter()
            .any(|&index| index as usize >= vertices.len())
        {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self {
            vertices,
            normals,
            alphas,
            indices,
            parts: Default::default(),
        })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        display.parts = parts;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    indices: Vec<u32>,
    pub spacing: f32,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(
        vertices: Vec<[f32; 6]>,
        indices: Vec<u32>,
    ) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err(
                "Indices must describe complete triangles within the draw limit",
            );
        }
        if indices
            .iter()
            .any(|&index| index as usize >= vertices.len())
        {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices
            .iter()
            .flatten()
            .any(|value| !value.is_finite())
        {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self {
            vertices,
            normals,
            alphas,
            indices,
            spacing: 0.,
            parts: Default::default(),
        })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        if !parts.dots.is_empty() {
            let mut b = session_rust::AABB::empty();
            for p in display.vertices() {
                b.union_with_point(p[0] as f64, p[1] as f64, p[2] as f64);
            }
            display.spacing = (b.diagonal().min(f32::MAX as f64) as f32)
                / (mesh.number_of_vertices() as f32).sqrt();
        }
        display.parts = parts;
```

### 2. `src/marker.rs`

Name the render-only size mode explicitly; original point widths and editing-control diameters remain unchanged.

**Locate this existing block:**

```rust
impl Marker {
    pub fn values(&self) -> [f32; 8] {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
impl Marker {
    // The signed GPU size is a CSS diameter, or negative mesh spacing in mm.
    pub fn mesh_dot(center: [f32; 3], colour: [f32; 4], spacing: f32) -> Self {
        Self {
            center,
            colour,
            diameter: if spacing > 0. {
                -spacing
            } else {
                1.
            },
        }
    }

    pub fn values(&self) -> [f32; 8] {
```

### 3. `src/pick_sources.rs`

Reuse the existing32byte point instance, endpoint addresses and upload; no distance array, extra metadata or buffer.

**Locate this existing block:**

```rust
                        .mesh_markers
                        .push(crate::marker::Marker {
                            center: model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32(),
                            diameter: 1.,
                            colour: edge_colour
                                .unwrap_or_else(|| row.mesh.parts.dot_colour(i)),
                        });
                    sources.marker_ids.push([owner, 0]);
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                        .mesh_markers
                        .push(crate::marker::Marker::mesh_dot(
                            model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32(),
                            edge_colour.unwrap_or_else(|| row.mesh.parts.dot_colour(i)),
                            row.mesh.spacing,
                        ));
                    sources.marker_ids.push([owner, 0]);
```

### 4. `src/stroke_gpu.rs`

Accept the explicit finite mesh-spacing size mode while rejecting zero and nonfinite display data.

**Locate this existing block:**

```rust
        for marker in markers {
            if marker.diameter <= 0.0
                || marker
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        for marker in markers {
            if marker.diameter == 0.0
                || marker
```

### 5. `src/marker_radius.wgsl`

Project one instance spacing at each vertex depth; retain the actual three-diameter taper and half-physical-pixel minimum.

Create the file and type:

```wgsl
fn marker_radius(position:vec4f,size:f32)->f32 {
    if(size>=0.){return size*view.density*0.5;}
    let scale=length(vec3f(view.transform[0].y,view.transform[1].y,view.transform[2].y));
    let spacing=-size*scale*view.size.y*0.5/position.w;
    let radius=view.density*0.5;
    return max(0.5,radius*clamp(spacing/max(6.*radius,1e-6),0.15,1.));
}
```

### 6. `src/finite_project_source.rs`

Expand one shared radius helper for both color/pick and existing finite/legacy paths.

**Locate this existing block:**

```rust
        )
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
        .replace(
            "#include \"marker_radius.wgsl\"",
            include_str!("marker_radius.wgsl"),
        )
}
```

### 7. `src/marker.wgsl`

Use the same radius rule in the existing marker paths. Color and integer picking retain the same projected marker footprint.

**Locate this existing block:**

```wgsl
}
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0);
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
```

### 8. `src/finite_marker.wgsl`

Use the same radius rule in the existing marker paths. Color and integer picking retain the same projected marker footprint.

**Locate this existing block:**

```wgsl
}
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
#include "marker_radius.wgsl"
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32,flags:u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
```

### 9. `src/mesh_density_tests.rs`

Exercise actual dense physical footprints, explicit point/control independence, correct rear-edge visibility, source release and shared geometry.

Create the file and type:

```rust
use crate::{
    cap_gpu_tests::{at, pixels_at, renderer, wait},
    pick_sources::{Mode, Part, Sources, Target},
    prepared::PreparedMesh,
    scene::Scene,
};
use prost::Message;
use session_rust::{Color, Mesh, Point, Session, Xform, proto};
use std::rc::Rc;
fn grid(n: usize) -> Mesh {
    let mut p = Vec::new();
    for y in 0..=n {
        for x in 0..=n {
            p.push(Point::new(
                -900. + 1800. * x as f64 / n as f64,
                -650. + 1300. * y as f64 / n as f64,
                0.,
            ));
        }
    }
    let mut f = Vec::new();
    for y in 0..n {
        for x in 0..n {
            let a = y * (n + 1) + x;
            f.extend([
                vec![a, a + 1, a + n + 2],
                vec![a, a + n + 2, a + n + 1],
            ]);
        }
    }
    let mut m = Mesh::from_vertices_and_faces(p, f);
    m.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    m
}
fn scene(m: Mesh) -> Scene {
    let mut s = Scene::empty();
    s.insert(PreparedMesh::new(m).unwrap())
        .unwrap();
    s
}
fn camera() -> crate::camera::Camera {
    let mut c = crate::camera_defaults_tests::top_fixture();
    c.distance = 3000.;
    c.radius = 1500.;
    c.aspect = 4. / 3.;
    c
}
fn matrix(c: &crate::camera::Camera) -> [f32; 16] {
    let mut m = c.uniform();
    m[12] += m[15] * 0.9 / 160.;
    m[13] -= m[15] * 0.9 / 120.;
    m
}
fn projected(m: [f32; 16], p: [f64; 3]) -> [f64; 2] {
    let c: [f64; 4] = std::array::from_fn(|i| {
        m[12 + i] as f64
            + (0..3)
                .map(|j| m[j * 4 + i] as f64 * p[j])
                .sum::<f64>()
    });
    [
        (c[0] / c[3] + 1.) * 160.,
        (1. - c[1] / c[3]) * 120.,
    ]
}
fn source(s: &Scene) -> (Vec<proto::Session>, serde_json::Value) {
    let b = crate::archive::Bundle::decode(&crate::scene_archive::save(s).unwrap())
        .unwrap();
    (
        b.documents
            .iter()
            .map(|v| proto::Session::decode(v.as_slice()).unwrap())
            .collect(),
        serde_json::to_value(b.metadata).unwrap(),
    )
}
fn export(name: &str, s: &Scene, image: &[u8]) {
    if let Some(root) = std::env::var_os("VIEWER_MESH_DENSITY_PROOF_DIR") {
        use std::io::Write;
        let p = std::path::Path::new(&root);
        std::fs::create_dir_all(p).unwrap();
        std::fs::write(
            p.join(format!("{name}.session")),
            crate::scene_archive::save(s).unwrap(),
        )
        .unwrap();
        let mut f = std::fs::File::create(p.join(format!("{name}.ppm"))).unwrap();
        write!(f, "P6\n320 240\n255\n").unwrap();
        for p in image.chunks_exact(4) {
            f.write_all(&p[..3]).unwrap();
        }
    }
}
fn pick(
    g: &mut super::Renderer,
    s: &Scene,
    m: [f32; 16],
    p: [f64; 3],
    mode: Mode,
) -> Option<Target> {
    g.set_pick_mode(mode);
    g.request_pick(
        super::contact_feature_tests::project(m, p),
        m,
    )
    .unwrap();
    wait(g);
    g.take_pick(m)
        .unwrap()
        .unwrap()
        .and_then(|v| g.resolve_pick(s, v))
}
fn expected(s: &Scene, m: [f32; 16]) -> [u8; 3] {
    let mut rgb = [51f64, 102., 153.];
    let row = &s.objects()[0];
    for &address in &row.mesh.parts.dots {
        let p = row.mesh.parts.endpoints[address as usize / 2][address as usize % 2];
        let p = row
            .model
            .transform_point(&Point::new(p[0], p[1], p[2]));
        let q = projected(m, [p[0], p[1], p[2]]);
        let coverage = (1. - (q[0] - 160.5).hypot(q[1] - 120.5)).clamp(0., 1.);
        for v in &mut rgb {
            *v = (25.5 * coverage + *v * (1. - coverage)).round();
        }
    }
    rgb.map(|v| v as u8)
}
#[test]
fn actual_original_dense_dots_share_color_and_pick_coverage_at_both_densities() {
    let s = scene(grid(128));
    let original = source(&s);
    let owner = s.objects()[0].id;
    assert_eq!(s.objects()[0].mesh.parts.dots.len(), 16641);
    let wanted = (1800f32.hypot(1300.)) / (16641f32).sqrt();
    assert!((s.objects()[0].mesh.spacing - wanted).abs() < 1e-5);
    let mut g = renderer(&s);
    g.set_opacity(1.);
    g.set_lighting(false, false);
    let uploads = g.stats()[0];
    let mut c = camera();
    for distance in [3000., 1500.] {
        c.distance = distance;
        for projection in [
            crate::camera::Projection::Orthographic,
            crate::camera::Projection::Perspective,
        ] {
            c.projection = projection;
            let m = matrix(&c);
            for density in [1., 2.] {
                g.set_density(density);
                let image = pixels_at(&g, m);
                let rgb = expected(&s, m);
                assert!(
                    at(&image, 160, 120)[..3]
                        .iter()
                        .zip(rgb)
                        .all(|(a, b)| a.abs_diff(b) <= 2),
                    "Actual full-source dots use the independently composited half-physical-pixel footprint"
                );
                assert_eq!(g.point_usage()[0], 16641);
                assert_eq!(g.markers.clipping_bytes(), [4, 4]);
                assert_eq!(
                    pick(&mut g, &s, m, [0., 0., 0.], Mode::Object),
                    Some(Target {
                        owner,
                        part: Part::Object
                    })
                );
                assert_eq!(
                    pick(&mut g, &s, m, [0., 0., 0.], Mode::Control),
                    None
                );
                assert!(pixels_at(&g, m) == image);
                assert_eq!(g.stats()[0], uploads);
                assert_eq!(source(&s), original);
                let name = if projection == crate::camera::Projection::Orthographic {
                    if density == 1. {
                        "dense-dpr1"
                    } else {
                        "dense-dpr2"
                    }
                } else {
                    "dense-perspective"
                };
                if projection == crate::camera::Projection::Orthographic
                    || density == 2.
                {
                    export(
                        &format!(
                            "{name}{}",
                            if distance == 1500. {
                                "-readable"
                            } else {
                                ""
                            }
                        ),
                        &s,
                        &image,
                    );
                }
            }
        }
    }
    g.set_density(2.);
    let before = pixels_at(&g, matrix(&c));
    g.markers.use_legacy_fixture();
    assert!(
        pixels_at(&g, matrix(&c)) == before,
        "The real retained legacy marker path shares the exact same spacing/coverage rule"
    );
}
#[test]
fn original_points_and_active_controls_keep_their_positive_css_sizes() {
    let mut s = scene(grid(1));
    let owner = s.objects()[0].id;
    let mut p = Point::new(0., 0., 20.);
    p.width = 9.;
    p.pointcolor = Color::new(0.8, 0.1, 0.2, 1.);
    let point = s
        .insert_point(crate::marker::PreparedPoint::new(Rc::new(p)).unwrap())
        .unwrap();
    s.owner_controls(owner).unwrap();
    let original = source(&s);
    let prepared = Sources::new(&s).unwrap();
    assert!(
        prepared
            .mesh_markers
            .iter()
            .all(|m| m.diameter < 0.)
    );
    assert_eq!(s.points()[0].marker().diameter, 9.);
    assert!(
        s.control_markers()
            .iter()
            .all(|m| m.display.diameter == 7.)
    );
    let mut g = renderer(&s);
    g.set_opacity(1.);
    g.set_lighting(false, false);
    let mut c = camera();
    for projection in [
        crate::camera::Projection::Orthographic,
        crate::camera::Projection::Perspective,
    ] {
        c.projection = projection;
        let mut m = c.uniform();
        m[12] += m[15] / 320.;
        m[13] -= m[15] / 240.;
        for density in [1., 2.] {
            g.set_density(density);
            let image = pixels_at(&g, m);
            assert_eq!(at(&image, 160, 120), [204, 26, 51, 255]);
            assert_eq!(g.point_usage()[0], 9);
            assert_eq!(g.markers.clipping_bytes(), [4, 4]);
            assert_eq!(
                pick(&mut g, &s, m, [0., 0., 20.], Mode::Object),
                Some(Target {
                    owner: point,
                    part: Part::Object
                })
            );
            let control = pick(
                &mut g,
                &s,
                m,
                [-900., -650., 0.],
                Mode::Control,
            )
            .unwrap();
            assert_eq!(control.owner, owner);
            assert!(matches!(
                control.part,
                Part::SourceControl(_)
            ));
            assert!(pixels_at(&g, m) == image);
            assert_eq!(source(&s), original);
            if projection == crate::camera::Projection::Orthographic && density == 2. {
                export("explicit-point-controls", &s, &image);
            }
        }
    }
    let mut box_mesh = Mesh::new();
    for (i, p) in [
        [-900., -650., -100.],
        [900., -650., -100.],
        [900., 650., -100.],
        [-900., 650., -100.],
        [-900., -650., 100.],
        [900., -650., 100.],
        [900., 650., 100.],
        [-900., 650., 100.],
    ]
    .into_iter()
    .enumerate()
    {
        box_mesh.add_vertex(Point::new(p[0], p[1], p[2]), Some(i));
    }
    for (face, ring) in [
        (17, vec![0, 3, 2, 1]),
        (93, vec![4, 5, 6, 7]),
        (125, vec![0, 4, 7, 3]),
        (302, vec![1, 2, 6, 5]),
        (456, vec![0, 1, 5, 4]),
        (9004, vec![3, 7, 6, 2]),
    ] {
        box_mesh.add_face(ring, Some(face));
    }
    box_mesh.set_objectcolor(Color::new(0.2, 0.4, 0.6, 1.));
    box_mesh.set_linecolors(vec![], [vec![41.; 4], vec![1.; 8]].concat());
    let s = scene(box_mesh);
    g.set_scene(&s, None);
    c.projection = crate::camera::Projection::Perspective;
    let m = c.uniform();
    let edge = projected(m, [-900., 0., 100.]);
    let image = pixels_at(&g, m);
    assert_eq!(
        at(
            &image,
            edge[0].floor() as usize - 8,
            edge[1].floor() as usize
        ),
        [255, 255, 255, 255],
        "Existing shared geometric visibility already hides rear edges; no redundant neighbor-normal buffer is needed for this opaque case"
    );
    export("hidden-rear-border", &s, &image);
}
#[test]
fn spacing_survives_source_release_shared_placement_and_camera_updates() {
    let mesh = grid(16);
    let mut document = Session::new("Shared density source");
    document.add_mesh(mesh, None);
    let bytes = document.to_proto().encode_to_vec();
    let mut e = crate::editor::Editor::default();
    e.apply(crate::editor::Action::ReplaceAt(
        bytes.clone(),
        Rc::new(crate::reload_url::ReloadUrl::new(
            "test:mesh-density".into(),
        )),
    ))
    .unwrap();
    let owner = e.scene.objects()[0].id;
    let display = Rc::clone(&e.scene.objects()[0].mesh);
    let weak = Rc::downgrade(e.scene.objects()[0].geometry().unwrap());
    let spacing = display.spacing;
    assert!(spacing > 0.);
    let original = source(&e.scene);
    let mut g = renderer(&e.scene);
    g.set_opacity(1.);
    g.set_density(2.);
    let mut c = camera();
    let m = matrix(&c);
    let before = pixels_at(&g, m);
    let uploads = g.stats()[0];
    for _ in 0..2 {
        c.distance *= 1.1;
        pixels_at(&g, matrix(&c));
        c.distance /= 1.1;
        assert!(pixels_at(&g, matrix(&c)) == before);
        assert_eq!(g.stats()[0], uploads);
        assert_eq!(e.scene.objects()[0].mesh.spacing, spacing);
    }
    e.apply(crate::editor::Action::UnloadSources)
        .unwrap();
    assert!(weak.upgrade().is_none());
    assert!(Rc::ptr_eq(
        &display,
        &e.scene.objects()[0].mesh
    ));
    g.set_scene(&e.scene, None);
    assert!(pixels_at(&g, m) == before);
    assert_eq!(g.stats()[0], uploads);
    let keys = e.reload_keys();
    e.hydrate(
        keys.into_iter()
            .map(|k| (k, bytes.clone()))
            .collect(),
    )
    .unwrap();
    assert_eq!(source(&e.scene), original);
    let mesh = grid(16);
    let guid = mesh.guid().to_owned();
    let mut proto = crate::instance_specimen::message();
    proto.definitions.as_mut().unwrap().meshes = vec![mesh.to_proto()];
    for row in &mut proto.objects.as_mut().unwrap().instances {
        row.definition_guid = guid.clone();
        row.xform = None;
        row.flags = 0;
    }
    proto.xforms = vec![
        session_rust::proto::XformEntry {
            guid: "first".into(),
            xform: Some(Xform::translation(-1800., 0., 0.).to_proto()),
        },
        session_rust::proto::XformEntry {
            guid: "second".into(),
            xform: Some(
                (&Xform::translation(1800., 0., 0.)
                    * &Xform::scale_xyz(-0.8, 1.2, 1.4))
                    .to_proto(),
            ),
        },
    ];
    let mut s = Scene::empty();
    crate::scene_document::import(
        &mut s,
        &proto.encode_to_vec(),
        "",
        &Xform::identity(),
        0.,
    )
    .unwrap();
    assert!(Rc::ptr_eq(
        &s.objects()[0].mesh,
        &s.objects()[1].mesh
    ));
    assert_eq!(
        s.objects()[0].mesh.spacing,
        s.objects()[1].mesh.spacing
    );
    let original = source(&s);
    g.set_scene(&s, None);
    c.distance = 6000.;
    c.radius = 3000.;
    let m = matrix(&c);
    let before = pixels_at(&g, m);
    let uploads = g.stats()[0];
    let place = s.placement(s.objects()[0].id).unwrap();
    s.place(
        s.objects()[0].id,
        &Xform::translation(100., 0., 0.) * &place,
    )
    .unwrap();
    g.set_scene(&s, None);
    assert!(pixels_at(&g, m) != before);
    s.place(s.objects()[0].id, place).unwrap();
    g.set_scene(&s, None);
    assert!(pixels_at(&g, m) == before);
    assert_eq!(g.stats()[0], uploads);
    assert_eq!(source(&s), original);
    export("shared-spacing", &s, &before);
}
```

### 10. `src/renderer.rs`

Register the three actual source-bound density GPU checks.

**Locate this existing block:**

```rust
#[cfg(test)]
#[path = "mesh_marker_tests.rs"]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
#[path = "mesh_density_tests.rs"]
mod mesh_density_tests;
#[cfg(test)]
#[path = "mesh_marker_tests.rs"]
```

### 11. `src/marker_coverage.wgsl`

Measure the circle at the actual physical fragment pixel relative to its fixed centre; keep one shared colour/query coverage function.

Replace the entire file with:

```wgsl
fn colour(rgba:vec4<f32>,radius:f32,delta:vec2<f32>)->vec4<f32> {
    let coverage=clamp(radius+0.5-length(delta),0.0,1.0);
    return vec4<f32>(rgba.rgb,rgba.a*coverage);
}
```

### 12. `src/marker.wgsl`

Carry a fixed screen centre instead of interpolating the expanded quad offset.

**Locate this existing block:**

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
```

### 13. `src/marker_pick.wgsl`

Carry a fixed screen centre instead of interpolating the expanded quad offset.

**Locate this existing block:**

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(flat) centre: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
```

### 14. `src/marker.wgsl`

Compute the centre before expanding the existing quad; no new vertex buffer or radius policy. Pass the fixed screen centre through the existing location. Use the fragment built-in pixel rather than interpolated quad coordinates.

**Locate this existing block:**

```wgsl
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    let radius = marker_radius(position,diameter);
    let centre=(position.xy/position.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;
    let offset = corners[index] * (radius + 0.70711);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, centre, radius);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input.colour,input.radius,input.position.xy-input.centre); }
```

### 15. `src/marker_pick.wgsl`

Give the cropped query the same centre in its local physical-pixel coordinates. Remove the unnecessary reconstructed colour-output record. Apply the same analytic disc coverage to integer queries.

**Locate this existing block:**

```wgsl
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.colour = source.colour;
    output.centre = source.centre-vec2<f32>(vec2<u32>(crop._a,crop._b));
    output.radius = source.radius;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(input.colour,input.radius,input.position.xy-input.centre).a <= 0.0) { discard; }
    return input.id;
```

### 16. `src/finite_marker.wgsl`

Reuse the existing fixed axis centre; drop the redundant interpolated offset. Use the actual fragment pixel and remove the second redundant varying.

**Locate this existing block:**

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
```

### 17. `src/finite_marker_pick.wgsl`

Reuse the existing fixed axis centre; drop the redundant interpolated offset. Use the actual fragment pixel and remove the second redundant varying.

**Locate this existing block:**

```wgsl
    @location(0) colour: vec4<f32>,
    @location(1) @interpolate(linear) offset: vec2<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(0) colour: vec4<f32>,
    @location(2) @interpolate(flat) radius: f32,
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(8) @interpolate(flat) clip_flags:u32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
}
```

### 18. `src/finite_marker.wgsl`

Keep the rejected-vertex result aligned with the smaller output. Reuse existing fixed source centre/depth fields and omit redundant pixel interpolation. Read visibility at the actual full-frame physical pixel. Evaluate colour coverage from the existing flat centre.

**Locate this existing block:**

```wgsl
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,flags,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = marker_radius(position,diameter);
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, radius,flags,axis,depths);
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32,@location(8) flags:u32) -> Output { return geometry(center, diameter, colour, index,flags); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {let alpha=finite_disc_alpha(input.position.xy,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,false);if(alpha<=0.){discard;}let c=colour(input.colour,input.radius,input.position.xy-input.finite_axis_value.xy);return vec4<f32>(c.rgb,c.a*alpha);}


#include "projected_triangle.wgsl"
```

### 19. `src/finite_marker_pick.wgsl`

Remove the deleted output varying from the query vertex. Keep the existing fixed axis/depth values without another varying. Restore the query fragment to the full physical frame once, then share it with coverage and visibility. Keep the integer-query disc identical to colour coverage. Use the same restored full-frame pixel for depth/section eligibility.

**Locate this existing block:**

```wgsl
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius,input.clip_flags,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(finite_disc_alpha(input.finite_pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.colour = source.colour;
    output.radius = source.radius;
    output.clip_flags=source.clip_flags;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let pixel=input.position.xy+vec2<f32>(vec2<u32>(crop._a,crop._b));
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(input.colour,input.radius,pixel-input.finite_axis_value.xy).a <= 0.0) { discard; }
    if(finite_disc_alpha(pixel,input.finite_axis_value.xy,input.finite_depths.x,input.clip_flags,true)<=0.){discard;}
    return input.id;
```

## Build & Run

```sh
npm --prefix ../session_tests run course -- dependencies 22.3
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 22.3
npm --prefix ../session_tests run course -- save 22.3
```

Continue with [22.4 · Original ambient inputs and real pass ordering](96zzzzzzr-share-geometric-ambient-inputs.md).
