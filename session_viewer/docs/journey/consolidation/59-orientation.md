# 9.5 · Oriented CAD solids, original picks and empty bores

**Steps:** 7 · **Source:** [app/walk/brep.rs](../../../src/app/walk/brep.rs) · [app/walk/brep_edges.rs](../../../src/app/walk/brep_edges.rs) · [app/walk/brep_orient.rs](../../../src/app/walk/brep_orient.rs) · [app/walk/brep_shapes.rs](../../../src/app/walk/brep_shapes.rs)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">(u,v) Outer Loop + Inner Hole Trim Loop</text>
  <rect x="64" y="48" width="212" height="106" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <circle cx="170" cy="101" r="28" fill="#ffffff" stroke="#dc2626" stroke-width="2"/>
  <line x1="64" y1="48" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="48" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="64" y1="154" x2="142" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="276" y1="154" x2="198" y2="101" stroke="#64748b" stroke-width="1"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crease Angle &gt; θ: Split Normals N1, N2 vs Smooth N</text>
  <line x1="426" y1="138" x2="496" y2="88" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="566" y2="138" stroke="#111111" stroke-width="2.5"/>
  <line x1="496" y1="88" x2="464" y2="44" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <line x1="496" y1="88" x2="528" y2="44" stroke="#2563eb" stroke-width="2.2" marker-end="url(#arrB)"/>
  <path d="M 596 138 Q 646 72 696 138" fill="none" stroke="#111111" stroke-width="2.5"/>
  <line x1="646" y1="105" x2="646" y2="48" stroke="#16a34a" stroke-width="2.5" marker-end="url(#arrG)"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Crisp Crease Shading + Boundary Wire Edges</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,128 934,62 934,146" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <polygon points="934,62 1022,128 934,146" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <ellipse cx="904" cy="112" rx="12" ry="8" fill="#ffffff" stroke="#dc2626" stroke-width="1.75"/>
</svg>
</div>

## <a id="checkpoint-59-orientation"></a>1. Orient connected face groups

### 1. `src/cad_orient.rs`

**Locate this existing block:**

```rust
    Some(away_a != away_b)
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Some(away_a != away_b)
}

fn six_volume(fm: &Mesh) -> f64 {
    let mut keys: Vec<usize> = fm.face.keys().copied().collect();
    keys.sort_unstable();
    let mut v = 0.0;

    for k in keys {
        let verts = &fm.face[&k];

        if verts.len() < 3 {
            continue;
        }

        let (a, b, c) = (
            at(fm, verts[0]),
            at(fm, verts[1]),
            at(fm, verts[2]),
        );
        v += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0])
            + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }

    v
}

pub fn face_signs(b: &BRep, fms: &[Mesh], chains: &[Option<EdgeChain>]) -> Vec<f64> {
    let nf = fms.len();

    if !b.is_solid() {
        return vec![1.0; nf];
    }

    let mut adjacent: Vec<Vec<(usize, bool)>> = vec![Vec::new(); nf];

    for c in chains.iter().flatten() {
        if let (Some(other), Some(opp)) = (c.other, opposed(fms, c)) {
            adjacent[c.face].push((other, opp));
            adjacent[other].push((c.face, opp));
        }
    }

    let mut sign = vec![0.0f64; nf];

    for start in 0..nf {
        if sign[start] != 0.0 {
            continue;
        }

        sign[start] = 1.0;
        let mut group = vec![start];
        let mut head = 0;

        while head < group.len() {
            let f = group[head];
            head += 1;

            for &(g, opp) in &adjacent[f] {
                if sign[g] == 0.0 {
                    sign[g] = if opp {
                        sign[f]
                    } else {
                        -sign[f]
                    };
                    group.push(g);
                }
            }
        }

        let mut volume = 0.0;

        for &face in &group {
            volume += sign[face] * six_volume(&fms[face]);
        }

        if volume < 0.0 {
            for &f in &group {
                sign[f] = -sign[f];
            }
        }
    }

    sign
}
```

## <a id="checkpoint-59a-face-data"></a>2. Map each display triangle to its CAD face

### 1. `src/cad_face.rs`

Create the file and type:

```rust
use session_rust::Point;
pub struct Sample {
    pub vertex: u32,
    pub surface: usize,
    pub uv: [f64; 2],
    pub sign: f64,
}
pub struct Data {
    pub points: Vec<Point>,
    pub faces: Vec<Vec<usize>>,
    pub vertices: Vec<[f32; 6]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub face_ids: Vec<u32>,
    pub keys: Vec<std::collections::HashMap<usize, u32>>,
    pub samples: Vec<Sample>,
}

pub fn prepare(boundary: &crate::cad_boundary::Prepared) -> Result<Data, &'static str> {
    let source = &boundary.source;
    let signs =
        crate::cad_orient::face_signs(source, &boundary.faces, &boundary.chains);
    let colour = source.surfacecolor.to_f32();
    let (mut points, mut faces, mut vertices, mut normals, mut indices, mut face_ids) = (
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    let mut mesh_keys = Vec::new();
    let mut samples = Vec::new();
    for (fi, mesh) in boundary.faces.iter().enumerate() {
        let render = mesh.to_render();
        let mut keys = mesh
            .vertex
            .keys()
            .copied()
            .collect::<Vec<_>>();
        keys.sort_unstable();
        if keys.len() != render.vertices.len() {
            return Err("Split face vertices require an explicit source map");
        }
        let base = u32::try_from(vertices.len())
            .map_err(|_| "Too many CAD display vertices")?;
        let map = keys
            .iter()
            .enumerate()
            .map(|(i, key)| (*key, base + i as u32))
            .collect::<std::collections::HashMap<_, _>>();
        mesh_keys.push(map);
        for (&key, vertex) in keys.iter().zip(&render.vertices) {
            let original = &mesh.vertex[&key];
            points.push(Point::new(
                original.x, original.y, original.z,
            ));
            let [x, y, z] = vertex.position;
            vertices.push([x, y, z, colour[0], colour[1], colour[2]]);
            normals.push(
                vertex
                    .normal
                    .map(|v| (v as f64 * signs[fi]) as f32),
            );
        }
        for sample in crate::cad_samples::face(boundary, fi)? {
            samples.push(Sample {
                vertex: mesh_keys[fi][&sample.key],
                surface: sample.surface,
                uv: sample.uv,
                sign: signs[fi],
            });
        }
        for tri in render.indices.chunks_exact(3) {
            let mut tri = [base + tri[0], base + tri[1], base + tri[2]];
            if signs[fi] < 0. {
                tri.swap(1, 2);
            }
            indices.extend_from_slice(&tri);
            faces.push(tri.map(|i| i as usize).to_vec());
            face_ids.push(fi as u32);
        }
    }
    Ok(Data {
        points,
        faces,
        vertices,
        normals,
        indices,
        face_ids,
        keys: mesh_keys,
        samples,
    })
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
mod cad_seam_tests;

pub mod cad_orient;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod cad_seam_tests;

pub mod cad_orient;

pub mod cad_face;
```

## <a id="checkpoint-59b-display"></a>3. Retain source UVs and draw only CAD boundaries

### 1. `src/cad_display.rs`

Create the file and type:

```rust
use session_rust::Xform;
use std::rc::Rc;

pub struct Boundary {
    pub edge: usize,
    pub points: Vec<[f64; 3]>,
}
pub struct Prepared {
    pub source: Rc<session_rust::BRep>,
    pub edges: Vec<Boundary>,
    pub samples: Vec<crate::cad_face::Sample>,
    pub face_count: usize,
    pub edge_ids: Vec<usize>,
}

pub fn prepare(
    source: Rc<session_rust::BRep>,
) -> Result<(Prepared, crate::prepared::PreparedMesh), &'static str> {
    let boundary = crate::cad_boundary::Prepared::new(source.clone())?;
    let data = crate::cad_face::prepare(&boundary)?;
    let mut proxy =
        session_rust::Mesh::from_vertices_and_faces(data.points, data.faces);
    proxy.set_guid(source.guid().to_owned());
    proxy.name = source.name.clone();
    proxy.is_visible = source.is_visible;
    proxy.is_locked = source.is_locked;
    proxy.set_objectcolor(source.surfacecolor.clone());
    let mut display = crate::mesh::Mesh::new(data.vertices, data.indices)?
        .with_normals(data.normals)?;
    display.parts.faces = (0..boundary.faces.len()).collect();
    display.parts.triangles = data.face_ids;
    let (mut edges, mut edge_ids) = (Vec::new(), Vec::new());
    for chain in boundary.chains.iter().flatten() {
        let (points, spans) = boundary
            .display(chain.edge)
            .ok_or("Missing CAD boundary sample")?;
        for (ends, span) in points.windows(2).zip(spans) {
            let pair = &chain.keys[span..=span + 1];
            display.parts.edges.push((
                data.keys[chain.face][&pair[0]] as usize,
                data.keys[chain.face][&pair[1]] as usize,
            ));
            display
                .parts
                .endpoints
                .push([ends[0], ends[1]]);
            edge_ids.push(chain.edge);
        }
        edges.push(Boundary {
            edge: chain.edge,
            points,
        });
    }
    let prepared = Prepared {
        source,
        edges,
        samples: data.samples,
        face_count: boundary.faces.len(),
        edge_ids,
    };
    let mesh = crate::prepared::PreparedMesh {
        geometry: Rc::new(proxy),
        display,
        source: None,
        model: Xform::identity(),
    };
    Ok((prepared, mesh))
}

pub fn example() -> session_rust::BRep {
    let mut source = session_rust::BRep::create_block_with_hole(4., 3., 0.6, 0.65);
    source.name = "Original CAD bore".into();
    source.surfacecolor = session_rust::Color::new(0.35, 0.65, 0.85, 1.);
    source
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod cad_orient;

pub mod cad_face;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cad_orient;

pub mod cad_face;

pub mod cad_display;
```

## <a id="checkpoint-59c-source-row"></a>4. Attach BRep ownership to existing rows

### 1. `src/scene.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            if row.surface.is_some() {
                return Err(
```

Replace it with:

```rust
            .iter_mut()
            .find(|r| r.id == target.owner)
        {
            if row.surface.is_some() || row.cad.is_some() {
                return Err(
```

**Locate this block:**

```rust
        Ok(id)
    }

    pub fn insert_surface(
        &mut self,
        source: Rc<session_rust::NurbsSurface>,
```

Replace it with:

```rust
        Ok(id)
    }

    pub fn insert_cad(
        &mut self,
        source: Rc<session_rust::BRep>,
    ) -> Result<ObjectId, &'static str> {
        let (prepared, mesh) = crate::cad_display::prepare(source)?;
        let prepared = Rc::new(prepared);
        let id = self.insert(mesh)?;
        self.objects
            .iter_mut()
            .find(|row| row.id == id)
            .unwrap()
            .cad = Some(prepared);
        Ok(id)
    }

    pub fn insert_surface(
        &mut self,
        source: Rc<session_rust::NurbsSurface>,
```

**Locate this block:**

```rust
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            surface: None,
            id,
            guid,
```

Replace it with:

```rust
        let label = metadata.name.clone();
        let node_guid = guid.clone();
        self.objects.push(Object {
            cad: None,
            surface: None,
            id,
            guid,
```

**Locate this block:**

```rust
pub struct Object {
    pub surface: Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
```

Replace it with:

```rust
pub struct Object {
    pub cad: Option<Rc<crate::cad_display::Prepared>>,
    pub surface: Option<Rc<crate::surface::Prepared>>,
    pub id: ObjectId,
    pub guid: String,
```

### 2. `src/document.rs`

**Locate this existing block:**

```rust
        .iter()
        .any(|row| row.surface.is_some())
    {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        .iter()
        .any(|row| row.surface.is_some() || row.cad.is_some())
    {
```

### 3. `src/part_preview.rs`

**Locate this existing block:**

```rust
    {
        if row.surface.is_some() {
            return Err(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    {
        if row.surface.is_some() || row.cad.is_some() {
            return Err(
```

## <a id="checkpoint-59d-identities"></a>5. Pick and highlight an CAD border

### 1. `src/pick_sources.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                .iter()
                .zip(&row.mesh.parts.endpoints)
            {
                let part = surface_edges
                    .as_mut()
                    .and_then(|edges| edges.next())
                    .map_or(Part::MeshEdge(a, b), Part::SurfaceEdge);
                let ids = append(
```

Replace it with:

```rust
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
```

**Locate this block:**

```rust
                    },
                )?;
            }
            let mut surface_edges = row
                .surface
                .as_ref()
```

Replace it with:

```rust
                    },
                )?;
            }
            let mut cad_edges = row
                .cad
                .as_ref()
                .map(|cad| cad.edge_ids.iter());
            let mut surface_edges = row
                .surface
                .as_ref()
```

**Locate this block:**

```rust
    SurfaceEdge(usize),
    MeshEdge(usize, usize),
    PathEdge(usize),
    Curve,
```

Replace it with:

```rust
    SurfaceEdge(usize),
    BRepEdge(usize),
    MeshEdge(usize, usize),
    PathEdge(usize),
    Curve,
```

### 2. `src/selection.rs`

**Locate this existing block:**

```rust
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::SurfaceEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.surface.as_ref().is_some_and(|s|s.grid.boundaries.iter().any(|b|b.edge==edge))),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::Object => format!("Object {}", self.owner.value()),
            Part::Face(key) => format!(
                "Object {} · source face {key}",
                self.owner.value()
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            Part::Face(key) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.faces.contains(&key)),
            Part::BRepEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.cad.as_ref().is_some_and(|cad|cad.edges.iter().any(|border|border.edge==edge))),
            Part::SurfaceEdge(edge)=>scene.objects().iter().any(|r|r.id==self.owner&&r.surface.as_ref().is_some_and(|s|s.grid.boundaries.iter().any(|b|b.edge==edge))),
            Part::MeshEdge(a, b) => scene.objects().iter().any(|r| r.id == self.owner && r.mesh.parts.edges.contains(&(a, b))),
            Part::PathEdge(index) => scene.paths().iter().any(|r| r.id == self.owner && matches!(&r.prepared.source,
                crate::chain::Source::Polyline(line) if line.get_points().windows(2).nth(index).is_some_and(|pair| pair[0].to_f32() != pair[1].to_f32()))),
            Part::Curve => scene.lines().iter().any(|r| r.id == self.owner) || scene.paths().iter().any(|r| r.id == self.owner && !matches!(r.prepared.source, crate::chain::Source::Polyline(_))),
            Part::Control(index) => scene.paths().iter().any(|r| r.id == self.owner && r.controls_visible && matches!(&r.prepared.source,
                crate::chain::Source::Curve(curve, _) if crate::controls::Control::new(std::rc::Rc::clone(curve), index).is_ok())),
        }
    }

    pub fn describe(self) -> String {
        match self.part {
            Part::Object => format!("Object {}", self.owner.value()),
            Part::Face(key) => format!(
                "Object {} · source face {key}",
                self.owner.value()
            ),
            Part::BRepEdge(edge) => format!(
                "Object {} · original BRep edge {edge}",
                self.owner.value()
```

### 3. `src/highlight.rs`

**Locate this existing block:**

```rust
                }
                Part::SurfaceEdge(edge) => {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                }
                Part::BRepEdge(edge) => {
                    let row = scene
                        .objects()
                        .iter()
                        .find(|r| r.id == target.owner)
                        .unwrap();
                    let points = &row
                        .cad
                        .as_ref()
                        .unwrap()
                        .edges
                        .iter()
                        .find(|border| border.edge == edge)
                        .unwrap()
                        .points;
                    for pair in points.windows(2) {
                        let ends = [pair[0], pair[1]].map(|p| {
                            row.model
                                .transform_point(&session_rust::Point::new(
                                    p[0], p[1], p[2],
                                ))
                                .to_f32()
                        });
                        strokes.push(crate::stroke::Stroke {
                            start: ends[0],
                            end: ends[1],
                            colour: yellow,
                            width: 3.,
                        });
                    }
                }
                Part::SurfaceEdge(edge) => {
```

## <a id="checkpoint-59e-cache"></a>6. Inspect shared CAD owners and compact cache counts

### 1. `src/memory.rs`

**Locate this existing block:**

```rust
    usage[2] = prepared.len();
    usage
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    usage[2] = prepared.len();
    usage
}

pub fn cads<'a>(scenes: impl IntoIterator<Item = &'a Scene>) -> [usize; 6] {
    let (mut sources, mut prepared) = (HashSet::new(), HashSet::new());
    let mut counts = [0; 6];
    for scene in scenes {
        for row in scene.objects() {
            if let Some(cad) = &row.cad {
                counts[0] += 1;
                sources.insert(Rc::as_ptr(&cad.source));
                if prepared.insert(Rc::as_ptr(cad)) {
                    counts[3] += cad.samples.len();
                    counts[4] += cad.face_count;
                    counts[5] += cad.edge_ids.len();
                }
            }
        }
    }
    counts[1] = sources.len();
    counts[2] = prepared.len();
    counts
}
```

### 2. `src/browser.rs`

**Locate this existing block:**

```rust
    canvas.set_attribute(
        "data-surface-cpu",
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    canvas.set_attribute(
        "data-cad-cache",
        &serde_json::to_string(&crate::memory::cads(
            editor.presentation_scenes(),
        ))
        .unwrap(),
    )?;
    let cads=editor.scene.objects().iter().filter_map(|row|row.cad.as_ref().map(|cad|serde_json::json!({"id":row.id.value(),"source":cad.source.guid(),"faces":row.mesh.parts.faces,"edges":cad.edges.iter().map(|border|border.edge).collect::<Vec<_>>(),"vertices":cad.samples.len()}))).collect::<Vec<_>>();
    canvas.set_attribute(
        "data-cads",
        &serde_json::to_string(&cads).unwrap(),
    )?;
    canvas.set_attribute(
        "data-surface-cpu",
```

## <a id="checkpoint-59f-cad"></a>7. Draw the bore and prove its empty hole

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
            Action::AddCad => {
                let source = std::rc::Rc::new(crate::cad_display::example());
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert_cad(source).map(|_| ())
                    })?;
            }
```

**Locate this block:**

```rust
    AddNested,
    AddSurface,
    AddLine,
    AddPoint,
```

Replace it with:

```rust
    AddNested,
    AddCad,
    AddSurface,
    AddLine,
    AddPoint,
```

### 2. `src/commands.rs`

**Locate this existing block:**

```rust
    ("Example Box", || Action::AddBox),
    ("Example Surface", || Action::AddSurface),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ("Example Box", || Action::AddBox),
    ("Example CAD", || Action::AddCad),
    ("Example Surface", || Action::AddSurface),
```

### 3. `src/cad_display_tests.rs`

Create the file and type:

```rust
use crate::editor::{Action, Editor};
use crate::pick_sources::{Part, Sources};
use std::rc::Rc;

#[test]
fn original_cad_faces_and_edges_survive_display_and_whole_placement_history() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddCad).unwrap();
    let row = &e.scene.objects()[0];
    let id = row.id;
    let cad = row.cad.as_ref().unwrap().clone();
    let before = cad.source.to_proto();
    assert_eq!(
        row.mesh.parts.faces,
        (0..7).collect::<Vec<_>>()
    );
    assert_eq!(row.metadata.source_guid, cad.source.guid());
    assert!(
        row.mesh
            .parts
            .triangles
            .iter()
            .all(|face| *face < 7)
    );
    assert_eq!(
        row.mesh.parts.edges.len(),
        cad.edge_ids.len()
    );
    let sources = Sources::new(&e.scene).unwrap();
    assert_eq!(sources.faces.len(), 7);
    assert!(
        sources
            .edges
            .iter()
            .all(|target| matches!(target.part, Part::BRepEdge(0..=14)))
    );
    assert!(crate::document::snapshot(&e.scene).is_err());
    e.select_objects([id], false);
    e.apply(Action::Translate([
        0.123456789012345,
        0.,
        0.,
    ]))
    .unwrap();
    assert!(Rc::ptr_eq(
        &cad,
        e.scene.objects()[0].cad.as_ref().unwrap()
    ));
    assert_eq!(cad.source.to_proto(), before);
    assert_eq!(
        crate::memory::cads(e.presentation_scenes())[1..3],
        [1, 1]
    );
    e.apply(Action::Undo).unwrap();
    assert_eq!(
        e.scene.placement(id).unwrap().m,
        session_rust::Xform::identity().m
    );
    e.apply(Action::Redo).unwrap();
    assert_eq!(
        e.scene.placement(id).unwrap().m,
        session_rust::Xform::translation(0.123456789012345, 0., 0.).m
    );
}

#[test]
fn close_releases_original_cad_without_releasing_retained_display() {
    let mut e = Editor::default();
    e.apply(Action::Close).unwrap();
    e.apply(Action::AddCad).unwrap();
    let row = &e.scene.objects()[0];
    let weak = Rc::downgrade(&row.cad.as_ref().unwrap().source);
    let display = row.mesh.clone();
    let metadata = row.metadata.clone();
    assert_eq!(
        crate::memory::cads(e.presentation_scenes())[1..3],
        [1, 1]
    );
    e.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert!(!display.indices().is_empty());
    assert_eq!(metadata.name, "Original CAD bore");
    assert_eq!(
        crate::memory::cads(e.presentation_scenes()),
        [0; 6]
    );
}

#[test]
fn solid_face_winding_and_normals_keep_the_original_outward_orientation() {
    for source in [
        session_rust::BRep::create_box(4., 3., 2.),
        session_rust::BRep::create_cylinder(2., 3.),
        crate::cad_display::example(),
    ] {
        let boundary = crate::cad_boundary::Prepared::new(Rc::new(source)).unwrap();
        let signs = crate::cad_orient::face_signs(
            &boundary.source,
            &boundary.faces,
            &boundary.chains,
        );
        assert!(signs.iter().all(|sign| *sign == 1.));
        let (prepared, mesh) =
            crate::cad_display::prepare(boundary.source.clone()).unwrap();
        assert!(
            mesh.display
                .normals()
                .iter()
                .all(|normal| normal.iter().all(|v| v.is_finite()))
        );
        assert!(!prepared.samples.is_empty());
    }
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod cad_face;

pub mod cad_display;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod cad_face;

pub mod cad_display;

#[cfg(test)]
mod cad_display_tests;
```

### 5. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
    assert_eq!(source.to_proto(), before);
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    assert_eq!(source.to_proto(), before);
}

#[test]
fn actual_cad_hole_has_no_face_pick_and_integer_faces_retain_original_identity() {
    use crate::pick_sources::{Mode, Part};
    let mut scene = Scene::demo();
    scene.clear();
    let mut original = session_rust::BRep::create_block_with_hole(1.6, 1.2, 0.24, 0.26);
    original.surfacecolor = session_rust::Color::new(0.35, 0.65, 0.85, 1.);
    let source = Rc::new(original);
    let _ = source.guid();
    let before = source.to_proto();
    let id = scene.insert_cad(source.clone()).unwrap();
    let mut renderer = renderer(&scene);
    let camera = crate::camera::Camera::default();
    let transform = camera.uniform();
    let gpu = renderer.stats();
    let finish_at = |renderer: &mut Renderer| {
        renderer
            .device
            .poll(wgpu::PollType::Wait {
                submission_index: None,
                timeout: Some(std::time::Duration::from_secs(5)),
            })
            .unwrap();
        renderer
            .take_pick(transform)
            .unwrap()
            .unwrap()
            .and_then(|hit| renderer.resolve_pick(&scene, hit))
    };
    renderer.set_pick_mode(Mode::Face);
    renderer
        .request_pick([0., 0.], transform)
        .unwrap();
    assert!(
        finish_at(&mut renderer).is_none(),
        "The actual shaded hole stays empty"
    );
    let p = camera
        .view_projection()
        .transform_point(&session_rust::Point::new(0.5, 0., 0.12));
    renderer
        .request_pick([p[0] as f32, p[1] as f32], transform)
        .unwrap();
    let target = finish_at(&mut renderer).unwrap();
    assert_eq!(target.owner, id);
    assert_eq!(
        target.part,
        Part::Face(6),
        "The visible top cap retains its original CAD identity"
    );
    renderer.set_pick_mode(Mode::Edge);
    let mut seen = 0;
    for border in &scene.objects()[0]
        .cad
        .as_ref()
        .unwrap()
        .edges
    {
        let points = &border.points;
        if !points
            .iter()
            .all(|p| (p[2] - 0.12).abs() < 1e-10)
        {
            continue;
        }
        let middle = points
            .windows(2)
            .nth((points.len() - 1) / 2)
            .unwrap();
        let p = std::array::from_fn::<_, 3, _>(|i| (middle[0][i] + middle[1][i]) * 0.5);
        let p = camera
            .view_projection()
            .transform_point(&session_rust::Point::new(p[0], p[1], p[2]));
        renderer
            .request_pick([p[0] as f32, p[1] as f32], transform)
            .unwrap();
        assert_eq!(
            finish_at(&mut renderer).unwrap().part,
            Part::BRepEdge(border.edge)
        );
        seen += 1;
    }
    assert_eq!(
        seen, 5,
        "All four outer edges and the original hole rim resolve"
    );
    assert_eq!(renderer.stats(), gpu);
    assert_eq!(source.to_proto(), before);
}
```

## Build & Run

![Recorded visual output for Lesson 9.5 (59f-cad)](../../screenshots/journey/59f-cad.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 9.5
npm --prefix ../session_tests run course -- save 9.5
```

Continue with [10.1 · Explicit fonts and source label placement](60-fonts.md).
