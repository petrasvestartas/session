# 5.7 · Normals, placement and a depth-correct construction grid

**Goal:** Enable the finite depth-correct grid through actual dock input, preserving grid view state, source history and unchanged geometry during camera navigation. · **Steps:** 8 · **Source:** [app/walk/curves.rs](../../../../src/app/walk/curves.rs) · [engine/gpu/segments.rs](../../../../src/engine/gpu/segments.rs) · [shaders/ribbon.wgsl](../../../../src/shaders/ribbon.wgsl) · [shaders/vector.wgsl](../../../../src/shaders/vector.wgsl)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Unit Normal Vector N = (nx, ny, nz)</text>
  <polygon points="84,138 186,98 288,138 186,162" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <circle cx="186" cy="128" r="4.5" fill="#111111"/>
  <line x1="186" y1="128" x2="186" y2="48" stroke="#2563eb" stroke-width="2.5" marker-end="url(#arrB)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="206" y="56" font-family="monospace" font-size="10" font-weight="700" fill="#2563eb">N (||N||=1)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">@fragment: rgb = N * 0.5 + 0.5</text>
  <rect x="448" y="54" width="66" height="88" rx="4" fill="#fca5a5" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="481" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">+X→R</text>
  <rect x="527" y="54" width="66" height="88" rx="4" fill="#86efac" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">+Y→G</text>
  <rect x="606" y="54" width="66" height="88" rx="4" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="639" y="102" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">+Z→B</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Normal Map &amp; Needle Overlay in Viewer</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="934,62 1002,90 934,116 866,90" fill="#93c5fd" stroke="#111111" stroke-width="1.5"/>
  <polygon points="866,90 934,116 934,154 866,126" fill="#86efac" stroke="#111111" stroke-width="1.5"/>
  <polygon points="934,116 1002,90 1002,126 934,154" fill="#fca5a5" stroke="#111111" stroke-width="1.5"/>
  <line x1="934" y1="88" x2="934" y2="48" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
</svg>
</div>

## <a id="checkpoint-41-normals"></a>1. Retain checked source normals beside display positions

Retain a checked normal stream from the original kernel adapter while preserving source attributes and the existing position/colour layout.

### 1. `src/normals.rs`

Normalize bounded display directions safely while preserving an explicit flat fallback.

Create the file and type:

```rust
pub fn unit(vector: [f32; 3]) -> Result<[f32; 3], &'static str> {
    if vector.iter().any(|v| !v.is_finite()) {
        return Err("Display normals must be finite");
    }
    let scale = vector
        .iter()
        .map(|v| v.abs())
        .fold(0.0, f32::max);
    if scale == 0.0 {
        return Ok([0.0; 3]);
    }
    let vector = vector.map(|v| v / scale);
    let length = vector
        .iter()
        .map(|v| v * v)
        .sum::<f32>()
        .sqrt();
    Ok(vector.map(|v| v / length))
}
```

### 2. `src/lib.rs`

Register normal preparation and its source checks.

**Locate this existing block:**

```rust
pub mod mesh;
pub mod offset;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod mesh;
#[cfg(test)]
mod normal_source_tests;
pub mod normals;
pub mod offset;
```

### 3. `src/mesh.rs`

Retain normal data independently of the established position/colour stream. Give manually created display meshes explicit flat normals by default. Carry kernel-extracted normal directions into checked display preparation. Require one finite checked normal per display vertex without touching original geometry. Count actual retained normal capacity in CPU display bytes.

**Locate this existing block:**

```rust
    vertices: Vec<[f32; 6]>,
    indices: Vec<u32>,
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
        Ok(Self {
            vertices,
            indices,
        })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let vertices = render
            .vertices
            .iter()
            .map(|vertex| {
                let [x, y, z] = vertex.position;
                let [r, g, b, _] = vertex.color;
                [x, y, z, r, g, b]
            })
            .collect();
        Self::new(vertices, render.indices)
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    vertices: Vec<[f32; 6]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
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
        Ok(Self {
            vertices,
            normals,
            indices,
        })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let vertices = render
            .vertices
            .iter()
            .map(|vertex| {
                let [x, y, z] = vertex.position;
                let [r, g, b, _] = vertex.color;
                [x, y, z, r, g, b]
            })
            .collect();
        let normals = render
            .vertices
            .iter()
            .map(|vertex| vertex.normal)
            .collect();
        Self::new(vertices, render.indices)?.with_normals(normals)
    }

    pub fn with_normals(
        mut self,
        normals: Vec<[f32; 3]>,
    ) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() {
            return Err("Each display vertex needs one normal");
        }
        self.normals = normals
            .into_iter()
            .map(crate::normals::unit)
            .collect::<Result<_, _>>()?;
        Ok(self)
    }

    pub fn normals(&self) -> &[[f32; 3]] {
        &self.normals
    }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
    }
```

### 4. `src/memory.rs`

Copy the normal-capacity oracle for a display mesh retaining three normals.

**Locate this existing block:**

```rust
        assert_eq!(retained[3], baseline[3] + 1);
        assert_eq!(retained[4], baseline[4] + 16 * 24 + 16 * 4);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(retained[3], baseline[3] + 1);
        assert_eq!(
            retained[4],
            baseline[4] + 16 * 24 + 16 * 4 + 3 * 12
        );
    }
```

### 5. `src/normal_source_tests.rs`

Copy exact original attributes, normalized directions, flat zeros, malformed data and overflow-safe checks.

Create the file and type:

```rust
use crate::{mesh::Mesh, normals::unit, prepared::PreparedMesh};

#[test]
fn original_vertex_normal_attributes_survive_normalized_display_preparation() {
    let mut source = session_rust::Mesh::create_box(1.0, 1.0, 1.0);
    for key in source.vertices() {
        source.set_vertex_attribute(key, "nx", 4.0);
        source.set_vertex_attribute(key, "ny", 0.0);
        source.set_vertex_attribute(key, "nz", 3.0);
    }
    let prepared = PreparedMesh::new(source).unwrap();
    assert!(
        prepared
            .display
            .normals()
            .iter()
            .all(|n| (n[0] - 0.8).abs() < 1e-6 && (n[2] - 0.6).abs() < 1e-6)
    );
    assert_eq!(
        prepared.geometry.vertex_attribute(0, "nx"),
        Some(4.0)
    );
    assert_eq!(
        prepared.geometry.vertex_attribute(0, "nz"),
        Some(3.0)
    );
    let flat =
        PreparedMesh::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0)).unwrap();
    assert!(
        flat.display
            .normals()
            .iter()
            .all(|n| *n == [0.0; 3])
    );
}

#[test]
fn normal_streams_refuse_mismatches_and_nonfinite_values_without_overflow() {
    assert!(
        Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2])
            .unwrap()
            .with_normals(vec![[0.0; 3]; 2])
            .is_err()
    );
    assert!(
        Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2])
            .unwrap()
            .with_normals(vec![[f32::NAN; 3]; 3])
            .is_err()
    );
    let huge = unit([f32::MAX, 0.0, f32::MAX]).unwrap();
    assert!((huge[0] - 0.5f32.sqrt()).abs() < 1e-6);
    assert_eq!(unit([0.0; 3]).unwrap(), [0.0; 3]);
}
```

## <a id="checkpoint-41a-placement"></a>2. Transform normals with nonuniform surface placement

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Triangle Vertices (v0, v1, v2) -&gt; Centroid C = (v0+v1+v2)/3</text>
  <polygon points="68,144 170,54 272,144" fill="#dbeafe" stroke="#111111" stroke-width="2"/>
  <line x1="68" y1="144" x2="221" y2="99" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <line x1="272" y1="144" x2="119" y2="99" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <line x1="170" y1="54" x2="170" y2="144" stroke="#94a3b8" stroke-width="1.2" stroke-dasharray="3 3"/>
  <circle cx="170" cy="114" r="5" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="188" y="120" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">C</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Outward Stem: P_tip = C + L_arrow * normalize((v1-v0)×(v2-v0))</text>
  <polygon points="444,142 560,82 676,142" fill="#f1f5f9" stroke="#111111" stroke-width="1.75"/>
  <circle cx="560" cy="122" r="4.5" fill="#111111"/>
  <line x1="560" y1="122" x2="560" y2="48" stroke="#dc2626" stroke-width="3" marker-end="url(#arrR)"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="598" y="64" font-family="monospace" font-size="10" font-weight="700" fill="#dc2626">C + L*N_f</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Face-Normal Orientation Hedgehog Overlay</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="846,136 934,68 934,146" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <polygon points="934,68 1022,136 934,146" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <line x1="904" y1="116" x2="862" y2="82" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
  <line x1="964" y1="116" x2="1006" y2="82" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arrR)"/>
</svg>
</div>

Prepare a finite oriented normal matrix that stays perpendicular under nonuniform scale, shear, reflection and flattening.

### 1. `src/normals.rs`

Prepare bounded cofactor columns for oriented affine normal transformation.

**Locate this existing block:**

```rust
    Ok(vector.map(|v| v / length))
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    Ok(vector.map(|v| v / length))
}

pub fn matrix(model: &session_rust::Xform) -> Result<[f32; 16], &'static str> {
    if !crate::placement::valid(&model.m) {
        return Err("Normal placement needs a finite affine matrix");
    }
    let column = |index: usize| {
        [
            model.m[index * 4],
            model.m[index * 4 + 1],
            model.m[index * 4 + 2],
        ]
    };
    let cross = |a: [f64; 3], b: [f64; 3]| {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    };
    let [a, b, c] = [column(0), column(1), column(2)];
    let columns = [cross(b, c), cross(c, a), cross(a, b)];
    let scale = columns
        .iter()
        .flatten()
        .map(|v| v.abs())
        .fold(0.0, f64::max);
    let mut matrix = [0.0; 16];
    if scale > 0.0 {
        for col in 0..3 {
            for row in 0..3 {
                matrix[col * 4 + row] = (columns[col][row] / scale) as f32;
            }
        }
    }
    matrix[15] = 1.0;
    Ok(matrix)
}
```

### 2. `src/lib.rs`

Register nonuniform, reflected and singular placement checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod normal_source_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod normal_matrix_tests;
#[cfg(test)]
mod normal_source_tests;
```

### 3. `src/normal_matrix_tests.rs`

Copy perpendicularity, translation independence, winding, collapse and finite-range checks.

Create the file and type:

```rust
use crate::normals::matrix;
use session_rust::Xform;

fn vector(matrix: &[f32; 16], v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|row| {
        (0..3)
            .map(|col| matrix[col * 4 + row] as f64 * v[col])
            .sum()
    })
}

#[test]
fn normal_cofactors_keep_transformed_tangents_perpendicular_and_ignore_translation() {
    let mut model = Xform::identity();
    model.m[0] = 2.0;
    model.m[4] = 1.0;
    model.m[5] = 3.0;
    model.m[10] = 4.0;
    let normal = vector(&matrix(&model).unwrap(), [1.0, 1.0, 1.0]);
    for tangent in [[1.0, -3.0, 0.0], [2.0, 0.0, -4.0]] {
        let dot: f64 = (0..3)
            .map(|axis| normal[axis] * tangent[axis])
            .sum();
        assert!(dot.abs() < 1e-6);
    }
    let before = matrix(&model).unwrap();
    model.m[12] = 123.456;
    model.m[13] = -987.654;
    assert_eq!(matrix(&model).unwrap(), before);
}

#[test]
fn reflections_flattening_and_extreme_scale_keep_finite_oriented_normals() {
    let mut reflected = Xform::identity();
    reflected.m[0] = -2.0;
    assert!(vector(&matrix(&reflected).unwrap(), [0.0, 0.0, 1.0])[2] < 0.0);
    let mut flat = Xform::identity();
    flat.m[10] = 0.0;
    assert_eq!(
        vector(&matrix(&flat).unwrap(), [0.0, 0.0, 1.0]),
        [0.0, 0.0, 1.0]
    );
    flat.m[5] = 0.0;
    assert_eq!(
        vector(&matrix(&flat).unwrap(), [1.0; 3]),
        [0.0; 3]
    );
    let mut huge = Xform::identity();
    for index in [0, 5, 10] {
        huge.m[index] = f32::MAX as f64;
    }
    let result = matrix(&huge).unwrap();
    assert!(result.iter().all(|v| v.is_finite()));
    assert_eq!([result[0], result[5], result[10]], [1.0; 3]);
    huge.m[0] = f64::NAN;
    assert!(matrix(&huge).is_err());
    let mut perspective = Xform::identity();
    perspective.m[3] = 0.1;
    assert!(matrix(&perspective).is_err());
}
```

## <a id="checkpoint-41b-drawing"></a>3. Draw smooth normals and sharp surface faces

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

Upload the retained source normal stream and its oriented placement matrix, with real smooth interpolation and explicit flat-face fallback.

### 1. `src/gpu_geometry.rs`

Share the checked normal stream with the geometry’s existing GPU lifetime. Upload three finite floats for each display vertex normal. Retain the normal allocation beside position and index buffers. Report actual normal allocation bytes as shared geometry. Bind the normal stream independently from position and colour attributes.

**Locate this existing block:**

```rust
    vertices: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            indices,
            index_count,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.indices.size()
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    vertices: wgpu::Buffer,
    normals: wgpu::Buffer,
    indices: wgpu::Buffer,
    index_count: u32,
}

impl GpuGeometry {
    pub fn upload(device: &wgpu::Device, source: Rc<Mesh>) -> Self {
        let bytes: Vec<u8> = source
            .vertices()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh vertices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .normals()
            .iter()
            .flatten()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let normals = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh normals"),
            contents: &bytes,
            usage: wgpu::BufferUsages::VERTEX,
        });
        let bytes: Vec<u8> = source
            .indices()
            .iter()
            .flat_map(|index| index.to_ne_bytes())
            .collect();
        let indices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shared mesh indices"),
            contents: &bytes,
            usage: wgpu::BufferUsages::INDEX,
        });
        let index_count = source.indices().len() as u32;
        Self {
            source,
            vertices,
            normals,
            indices,
            index_count,
        }
    }

    pub fn allocated_bytes(&self) -> u64 {
        self.vertices.size() + self.normals.size() + self.indices.size()
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_vertex_buffer(1, self.normals.slice(..));
        pass.set_index_buffer(
```

### 2. `src/renderer.rs`

Describe three normal floats in shader attribute two.

**Locate this existing block:**

```rust
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                    attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3],
                }, wgpu::VertexBufferLayout {
                    array_stride: 12, step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![2 => Float32x3],
                }],
```

### 3. `src/gpu_mesh.rs`

Retain previous model, selection and normal-matrix settings for change-only writes. Allocate 64 model bytes, 16 selection bytes and 64 normal-matrix bytes. Serialize bounded normal cofactors after model and selection settings. Rewrite only the settings region whose bytes changed.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    ) -> [usize; 2] {
        let next = settings(object, selected);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80)] {
            if next[start..end] != self.previous[start..end] {
```

Replace it with:

```rust
    ) -> [usize; 2] {
        let next = settings(object, selected);
        let mut writes = [0, 0];
        for (start, end) in [(0, 64), (64, 80), (80, 144)] {
            if next[start..end] != self.previous[start..end] {
```

**Locate this block:**

```rust
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    bytes
}
```

Replace it with:

```rust
        0.0
    };
    bytes[64..68].copy_from_slice(&flag.to_ne_bytes());
    let normals = crate::normals::matrix(&object.model)
        .expect("Scene placements are checked before insertion or editing");
    for (index, value) in normals.iter().enumerate() {
        bytes[80 + index * 4..84 + index * 4].copy_from_slice(&value.to_ne_bytes());
    }
    bytes
}
```

**Locate this block:**

```rust
    model: wgpu::Buffer,
    previous: [u8; 80],
}

fn settings(object: &Object, selected: bool) -> [u8; 80] {
    let mut bytes = [0; 80];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
```

Replace it with:

```rust
    model: wgpu::Buffer,
    previous: [u8; 144],
}

fn settings(object: &Object, selected: bool) -> [u8; 144] {
    let mut bytes = [0; 144];
    for (index, value) in object.model.m.iter().enumerate() {
        bytes[index * 4..index * 4 + 4].copy_from_slice(&(*value as f32).to_ne_bytes());
    }
```

### 4. `src/scene.rs`

Refuse invalid initial placement before consuming an ID or creating a row.

**Locate this existing block:**

```rust
    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        let next = self
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn insert(&mut self, prepared: PreparedMesh) -> Result<ObjectId, &'static str> {
        if !crate::placement::valid(&prepared.model.m) {
            return Err("Placement must be finite, affine and fit the display range");
        }
        let next = self
```

### 5. `src/geometry_cache.rs`

Count initialized normal payload separately from retained GPU capacity.

**Locate this existing block:**

```rust
        self.uploads += 1;
        self.uploaded_bytes +=
            (source.vertices().len() * 24 + source.indices().len() * 4) as u64;
        geometry
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.uploads += 1;
        self.uploaded_bytes += (source.vertices().len() * 24
            + source.normals().len() * 12
            + source.indices().len() * 4) as u64;
        geometry
```

### 6. `src/renderer.rs`

Count complete initial object settings without confusing the separate ink view uniform.

**Locate this existing block:**

```rust
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 80
            + self.settings_bytes as u64
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        self.geometry.uploaded_bytes
            + self.settings_allocations as u64 * 144
            + self.settings_bytes as u64
```

### 7. `src/triangle.wgsl`

Keep the shader uniform layout identical to the 144-byte host settings. Interpolate the placed source normal across the rasterized face. Read the normal attribute from the separate shared stream. Transform a direction without adding the object’s translation. Choose source smoothing or sharp derivatives, then normalize without zero division or squaring huge vectors.

**Locate this existing block:**

```wgsl
    selection: vec4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x > 0.5);
    output.world = world.xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Neighbouring fragments give two directions along this triangle's surface.
    let normal = normalize(cross(dpdx(input.world), dpdy(input.world)));
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    selection: vec4<f32>,
    normal_matrix: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x > 0.5);
    output.world = world.xyz;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let face = cross(dpdx(input.world), dpdy(input.world));
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
```

### 8. `src/lib.rs`

Register atomic placement and normal-stream GPU boundary checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod normal_matrix_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod normal_gpu_tests;
#[cfg(test)]
mod normal_matrix_tests;
```

### 9. `src/normal_gpu_tests.rs`

Copy invalid-placement refusal and unchanged identity-allocation checks.

Create the file and type:

```rust
use crate::{prepared::PreparedMesh, scene::Scene};

#[test]
fn invalid_initial_normal_placement_does_not_consume_an_object_identity() {
    let mut scene = Scene::demo();
    scene.clear();
    let baseline = scene.clone();
    let mut prepared =
        PreparedMesh::new(session_rust::Mesh::create_box(1.0, 1.0, 1.0)).unwrap();
    prepared.model.m[0] = f64::NAN;
    assert!(scene.insert(prepared).is_err());
    assert!(scene.objects().is_empty());
    let source = session_rust::Mesh::create_box(1.0, 1.0, 1.0);
    let actual = scene
        .insert(PreparedMesh::new(source.clone()).unwrap())
        .unwrap();
    let mut baseline = baseline;
    let expected = baseline
        .insert(PreparedMesh::new(source).unwrap())
        .unwrap();
    assert_eq!(actual, expected);
}
```

## <a id="checkpoint-41c-view"></a>4. Reveal oriented world normals in the renderer

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

Draw an oriented world-normal diagnostic with sharp-face winding and finite source smoothing, independently of source history.

### 1. `src/editor.rs`

Expose source insertion and normal diagnostics as separate editor actions. Retain the diagnostic view outside source document history. Start with ordinary surface lighting. Change normal visualization without adding document history.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::Zoom(factor) => self.camera.zoom(factor),
```

Replace it with:

```rust
            view => {
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Zoom(factor) => self.camera.zoom(factor),
```

**Locate this block:**

```rust
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            history: History::default(),
            last_release: 0,
        }
```

Replace it with:

```rust
            selected: None,
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
            history: History::default(),
            last_release: 0,
        }
```

**Locate this block:**

```rust
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    history: History,
    last_release: u64,
}
```

Replace it with:

```rust
    pub selected: Option<ObjectId>,
    pub camera: Camera,
    pub background: Background,
    pub normal_view: bool,
    history: History,
    last_release: u64,
}
```

**Locate this block:**

```rust
    AddColours(crate::colour::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
```

Replace it with:

```rust
    AddColours(crate::colour::Mode),
    NormalView(bool),
    Controls(bool),
    AddPolyline,
    AddArrow,
```

### 2. `src/triangle.wgsl`

Describe camera transform and diagnostic mode in the view uniform. Use the transform field of the extended view settings. Receive actual face orientation rather than inferring winding from lighting. Recover oriented sharp-face normals from raster derivatives and actual front-facing state. Map oriented world-normal components into visible diagnostic RGB.

**Locate this existing block:**

```wgsl
@group(0) @binding(0) var<uniform> transform: mat4x4<f32>;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<f32>,
    normal_matrix: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x > 0.5);
    output.world = world.xyz;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let face = cross(dpdx(input.world), dpdy(input.world));
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<f32>,
    normal_matrix: mat4x4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    output.colour = select(colour, vec3<f32>(1.0, 0.75, 0.05), object.selection.x > 0.5);
    output.world = world.xyz;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), 1.0); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
```

### 3. `src/renderer.rs`

Allocate the complete camera and diagnostic uniform. Draw either actual surface lighting or normal diagnostics with shared geometry and ink.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) {
        let bytes: Vec<u8> = transform
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        self.queue
```

Replace it with:

```rust
        background: &crate::background::Background,
        transform: &[f32; 16],
    ) {
        self.draw_mode(view, background, transform, false);
    }

    pub fn draw_mode(
        &self,
        view: &wgpu::TextureView,
        background: &crate::background::Background,
        transform: &[f32; 16],
        normals: bool,
    ) {
        let mut bytes: Vec<u8> = transform
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect();
        let display = [
            if normals {
                1.0f32
            } else {
                0.0
            },
            0.0,
            0.0,
            0.0,
        ];
        bytes.extend(
            display
                .iter()
                .flat_map(|value| value.to_ne_bytes()),
        );
        self.queue
```

**Locate this block:**

```rust
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
```

Replace it with:

```rust
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("view transform and normal diagnostic"),
            size: 80,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
```

### 4. `src/browser.rs`

Pass the editor’s diagnostic setting to actual initial or subsequent browser drawing. Carry diagnostic mode through browser surface presentation. Present actual diagnostic pixels through the maintained surface renderer. Expose current view mode for actual command verification. Offer normal visualization through completion and Help. Connect actual typed commands to the independent view setting.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw(&view, background, transform);
    panel.draw(renderer, &view);
```

Replace it with:

```rust
            format: Some(frame.texture.format().add_srgb_suffix()),
            ..Default::default()
        });
    renderer.draw_mode(&view, background, transform, normals);
    panel.draw(renderer, &view);
```

**Locate this block:**

```rust
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

Replace it with:

```rust
    renderer: &Renderer,
    background: &Background,
    transform: &[f32; 16],
    normals: bool,
    panel: &mut crate::panel::Panel,
) -> Result<(), JsValue> {
    let frame = match surface.get_current_texture() {
```

**Locate this block:**

```rust
            .unwrap_or_default(),
    )?;
    canvas.set_attribute(
        "data-camera-matrix",
        &serde_json::to_string(&editor.camera.view_projection().m)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
```

Replace it with:

```rust
            .unwrap_or_default(),
    )?;
    canvas.set_attribute(
        "data-normal-view",
        if editor.normal_view {
            "true"
        } else {
            "false"
        },
    )?;
    canvas.set_attribute(
        "data-camera-matrix",
        &serde_json::to_string(&editor.camera.view_projection().m)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
```

**Locate this block:**

```rust
                    &renderer,
                    &editor.background,
                    &editor.camera.uniform(),
                    &mut panel,
                ) {
                    report(&format!("Cannot redraw: {error:?}"));
```

Replace it with:

```rust
                    &renderer,
                    &editor.background,
                    &editor.camera.uniform(),
                    editor.normal_view,
                    &mut panel,
                ) {
                    report(&format!("Cannot redraw: {error:?}"));
```

**Locate this block:**

```rust
                        "example face colors" => {
                            Action::AddColours(crate::colour::Mode::Faces)
                        }
                        "controls on" => Action::Controls(true),
                        "controls off" => Action::Controls(false),
                        "example polyline" => Action::AddPolyline,
```

Replace it with:

```rust
                        "example face colors" => {
                            Action::AddColours(crate::colour::Mode::Faces)
                        }
                        "view normals on" => Action::NormalView(true),
                        "view normals off" => Action::NormalView(false),

                        "controls on" => Action::Controls(true),
                        "controls off" => Action::Controls(false),
                        "example polyline" => Action::AddPolyline,
```

**Locate this block:**

```rust
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            &mut panel,
        )?;
        let completing = fault.clone();
```

Replace it with:

```rust
            &renderer,
            &editor.background,
            &editor.camera.uniform(),
            editor.normal_view,
            &mut panel,
        )?;
        let completing = fault.clone();
```

**Locate this block:**

```rust
            "Example Face Colors",
            "Controls On",
            "Controls Off",
            "Example Polyline",
```

Replace it with:

```rust
            "Example Face Colors",
            "View Normals On",
            "View Normals Off",
            "Controls On",
            "Controls Off",
            "Example Polyline",
```

### 5. `src/lib.rs`

Register the source-history independence check.

**Locate this existing block:**

```rust
mod close_report_tests;
pub mod colour;
#[cfg(test)]
mod colour_input_tests;
#[cfg(test)]
mod colour_tests;
#[cfg(test)]
mod control_parent_tests;
#[cfg(test)]
mod control_tests;
pub mod controls;
pub mod curve;
#[cfg(test)]
mod curve_input_tests;
#[cfg(test)]
mod curve_layout_tests;
#[cfg(test)]
mod curve_sample_tests;
#[cfg(test)]
mod curve_source_tests;
pub mod diagnostic;
#[cfg(test)]
mod diagnostic_shape_tests;
#[cfg(test)]
mod diagnostic_tests;
pub mod drawing_document;
#[cfg(test)]
mod edit_delete_tests;
pub mod edit_intent;
#[cfg(test)]
mod edit_move_tests;
#[cfg(test)]
mod edit_owner_tests;
pub mod edit_replay;
#[cfg(test)]
mod edit_scope_tests;
mod edit_source;
pub mod editor;
pub mod error_message;
#[cfg(test)]
mod error_message_tests;
pub mod gesture;
pub mod gpu_fault;
#[cfg(test)]
mod gpu_fault_tests;
#[cfg(test)]
mod heartbeat_tests;
pub mod history;
#[cfg(test)]
mod index_width_tests;
#[cfg(test)]
mod line_scene_tests;
pub mod load_measure;
#[cfg(test)]
mod load_measure_tests;
pub mod load_phase;
#[cfg(test)]
mod load_phase_tests;
pub mod marker;
#[cfg(test)]
mod marker_tests;
pub mod memory;
pub mod mesh;
#[cfg(test)]
mod normal_gpu_tests;
#[cfg(test)]
mod normal_matrix_tests;
#[cfg(test)]
mod normal_source_tests;
pub mod normals;
pub mod offset;
pub mod origin;
#[cfg(test)]
mod origin_tests;
#[cfg(test)]
mod path_document_tests;
#[cfg(test)]
mod path_input_tests;
#[cfg(test)]
mod path_placement_tests;
pub mod picking;
pub mod placement;
#[cfg(test)]
mod point_document_tests;
#[cfg(test)]
mod point_input_tests;
pub mod prepared;
pub mod recovery;
#[cfg(test)]
mod recovery_tests;
pub mod rehydrate;
#[cfg(test)]
mod rehydrate_tests;
#[cfg(test)]
mod reload_complete_tests;
#[cfg(test)]
mod reload_failure_tests;
pub mod reload_job;
#[cfg(test)]
mod reload_precision_tests;
pub mod reload_reply;
#[cfg(test)]
mod reload_reply_tests;
pub mod reload_url;
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod row_metadata;
pub mod scene;
pub mod shortcuts;
#[cfg(test)]
mod shortcuts_tests;
#[cfg(test)]
mod source_visibility_tests;
pub mod startup;
#[cfg(test)]
mod startup_tests;
pub mod stroke;
pub mod stroke_gpu;
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
pub mod suspension;
#[cfg(test)]
mod suspension_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod close_report_tests;
#[cfg(test)]
mod control_parent_tests;
#[cfg(test)]
mod control_tests;
pub mod controls;
pub mod curve;
#[cfg(test)]
mod curve_input_tests;
#[cfg(test)]
mod curve_layout_tests;
#[cfg(test)]
mod curve_sample_tests;
#[cfg(test)]
mod curve_source_tests;
pub mod diagnostic;
#[cfg(test)]
mod diagnostic_shape_tests;
#[cfg(test)]
mod diagnostic_tests;
pub mod error_message;
#[cfg(test)]
mod error_message_tests;
pub mod gpu_fault;
#[cfg(test)]
mod gpu_fault_tests;
#[cfg(test)]
mod heartbeat_tests;
pub mod load_measure;
#[cfg(test)]
mod load_measure_tests;
pub mod load_phase;
#[cfg(test)]
mod load_phase_tests;
pub mod marker;
#[cfg(test)]
mod marker_tests;
pub mod memory;
pub mod mesh;
#[cfg(test)]
mod normal_view_tests;
pub mod normals;
#[cfg(test)]
mod path_document_tests;
#[cfg(test)]
mod path_input_tests;
#[cfg(test)]
mod path_placement_tests;
#[cfg(test)]
mod point_document_tests;
#[cfg(test)]
mod point_input_tests;
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod startup;
#[cfg(test)]
mod startup_tests;
pub mod stroke;
pub mod stroke_gpu;
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
pub mod suspension;
#[cfg(test)]
mod suspension_tests;

pub mod colour;
#[cfg(test)]
mod colour_input_tests;
#[cfg(test)]
mod colour_tests;
pub mod drawing_document;
#[cfg(test)]
mod edit_delete_tests;
pub mod edit_intent;
#[cfg(test)]
mod edit_move_tests;
#[cfg(test)]
mod edit_owner_tests;
pub mod edit_replay;
#[cfg(test)]
mod edit_scope_tests;
mod edit_source;
pub mod editor;
pub mod gesture;
pub mod history;
#[cfg(test)]
mod index_width_tests;
#[cfg(test)]
mod line_scene_tests;
#[cfg(test)]
mod normal_gpu_tests;
#[cfg(test)]
mod normal_matrix_tests;
#[cfg(test)]
mod normal_source_tests;
pub mod offset;
pub mod origin;
#[cfg(test)]
mod origin_tests;
pub mod picking;
pub mod placement;
pub mod prepared;
pub mod recovery;
#[cfg(test)]
mod recovery_tests;
pub mod rehydrate;
#[cfg(test)]
mod rehydrate_tests;
#[cfg(test)]
mod reload_complete_tests;
#[cfg(test)]
mod reload_failure_tests;
pub mod reload_job;
#[cfg(test)]
mod reload_precision_tests;
pub mod reload_reply;
#[cfg(test)]
mod reload_reply_tests;
pub mod reload_url;
pub mod row_metadata;
pub mod scene;
pub mod shortcuts;
#[cfg(test)]
mod shortcuts_tests;
#[cfg(test)]
mod source_visibility_tests;
#[cfg(test)]
```

### 6. `src/normal_view_tests.rs`

Copy the check that viewing normals preserves geometry, identity and Undo.

Create the file and type:

```rust
use crate::{
    colour::Mode,
    editor::{Action, Change, Editor},
};

#[test]
fn normal_view_changes_without_consuming_source_history() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor
        .apply(Action::AddColours(Mode::Faces))
        .unwrap();
    let id = editor.scene.objects()[0].id;
    let original = editor.scene.objects()[0]
        .geometry()
        .unwrap()
        .to_proto();
    assert_eq!(
        editor
            .apply(Action::NormalView(true))
            .unwrap(),
        Change::View
    );
    assert_eq!(
        editor.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.objects().is_empty());
    assert!(editor.normal_view);
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].id, id);
    assert_eq!(
        editor.scene.objects()[0]
            .geometry()
            .unwrap()
            .to_proto(),
        original
    );
    assert_eq!(
        editor
            .apply(Action::NormalView(false))
            .unwrap(),
        Change::View
    );
    assert!(!editor.normal_view);
}
```

## <a id="checkpoint-41ca-input"></a>5. Inspect smooth normals and face winding through commands

Compare sharp/smooth source surfaces and actual face winding through real commands, retaining source history and editable normal attributes.

### 1. `src/normal_example.rs`

Create original source specimens with missing, smooth or oppositely wound normals.

Create the file and type:

```rust
use session_rust::{Color, Mesh, Point};

#[derive(Clone, Copy)]
pub enum Mode {
    Sharp,
    Smooth,
    Winding,
}

pub fn specimen(mode: Mode) -> Mesh {
    let winding = matches!(mode, Mode::Winding);
    let xs: &[f64] = if winding {
        &[-0.7, 0.0, 0.7]
    } else {
        &[-0.7, 0.7]
    };
    let mut points: Vec<Point> = xs
        .iter()
        .map(|&x| Point::new(x, -0.5, 0.0))
        .chain(xs.iter().map(|&x| Point::new(x, 0.5, 0.0)))
        .collect();
    if winding {
        points.extend([
            Point::new(0.0, -0.5, 0.0),
            Point::new(0.0, 0.5, 0.0),
        ]);
    }
    let faces = if winding {
        vec![vec![0, 1, 4, 3], vec![6, 7, 5, 2]]
    } else {
        vec![vec![0, 1, 3, 2]]
    };
    let mut mesh = Mesh::from_vertices_and_faces(points, faces);
    mesh.set_objectcolor(Color::new(0.5, 0.5, 0.5, 1.0));
    if matches!(mode, Mode::Smooth) {
        for key in mesh.vertices() {
            let right = key == 1 || key == 3;
            mesh.set_vertex_attribute(
                key,
                "nx",
                if right {
                    1.0
                } else {
                    0.0
                },
            );
            mesh.set_vertex_attribute(key, "ny", 0.0);
            mesh.set_vertex_attribute(
                key,
                "nz",
                if right {
                    0.0
                } else {
                    1.0
                },
            );
        }
    }
    mesh
}
```

### 2. `src/editor.rs`

Expose source insertion and normal diagnostics as separate editor actions. Commit one original normal specimen in a single undoable insertion.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddColours(mode) => {
```

Replace it with:

```rust
                    })?;
            }
            Action::Controls(visible) => self.scene.show_controls(visible),
            Action::AddNormals(mode) => {
                let prepared = crate::prepared::PreparedMesh::new(
                    crate::normal_example::specimen(mode),
                )?;
                self.history
                    .try_edit(&mut self.scene, |scene| {
                        scene.insert(prepared).map(|_| ())
                    })?;
            }
            Action::AddColours(mode) => {
```

**Locate this block:**

```rust
    NormalView(bool),
    Controls(bool),
    AddPolyline,
    AddArrow,
```

Replace it with:

```rust
    NormalView(bool),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
    AddArrow,
```

### 3. `src/browser.rs`

Expose normal specimens and diagnostics in actual dock completion and Help. Dispatch real command input through source and view actions.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        }
                        "view normals on" => Action::NormalView(true),
                        "view normals off" => Action::NormalView(false),

                        "controls on" => Action::Controls(true),
```

Replace it with:

```rust
                        }
                        "view normals on" => Action::NormalView(true),
                        "view normals off" => Action::NormalView(false),
                        "example sharp" => {
                            Action::AddNormals(crate::normal_example::Mode::Sharp)
                        }
                        "example smooth" => {
                            Action::AddNormals(crate::normal_example::Mode::Smooth)
                        }
                        "example winding" => {
                            Action::AddNormals(crate::normal_example::Mode::Winding)
                        }

                        "controls on" => Action::Controls(true),
```

**Locate this block:**

```rust
            "View Normals Off",
            "Controls On",
            "Controls Off",
            "Example Polyline",
```

Replace it with:

```rust
            "View Normals Off",
            "Example Sharp",
            "Example Smooth",
            "Example Winding",
            "Controls On",
            "Controls Off",
            "Example Polyline",
```

### 4. `src/lib.rs`

Register source normal attributes, view independence, history and editable roundtrip checks.

**Locate this existing block:**

```rust
pub mod mesh;
#[cfg(test)]
mod normal_view_tests;
pub mod normals;
#[cfg(test)]
mod path_document_tests;
#[cfg(test)]
mod path_input_tests;
#[cfg(test)]
mod path_placement_tests;
#[cfg(test)]
mod point_document_tests;
#[cfg(test)]
mod point_input_tests;
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod startup;
#[cfg(test)]
mod startup_tests;
pub mod stroke;
pub mod stroke_gpu;
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
pub mod suspension;
#[cfg(test)]
mod suspension_tests;

pub mod colour;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod mesh;
pub mod normal_example;
#[cfg(test)]
mod normal_input_tests;
pub mod normals;
#[cfg(test)]
mod path_document_tests;
#[cfg(test)]
mod path_input_tests;
#[cfg(test)]
mod path_placement_tests;
#[cfg(test)]
mod point_document_tests;
#[cfg(test)]
mod point_input_tests;
pub mod report_recency;
#[cfg(test)]
mod report_recency_tests;
pub mod report_store;
#[cfg(test)]
mod report_store_tests;
pub mod startup;
#[cfg(test)]
mod startup_tests;
pub mod stroke;
pub mod stroke_gpu;
#[cfg(test)]
mod stroke_input_tests;
#[cfg(test)]
mod stroke_tests;
pub mod suspension;
#[cfg(test)]
mod suspension_tests;

#[cfg(test)]
mod normal_view_tests;

pub mod colour;
```

### 5. `src/normal_input_tests.rs`

Copy source normal attributes, exact protobuf roundtrip, independent view mode, one-step history and Close checks.

Create the file and type:

```rust
use crate::{
    document,
    editor::{Action, Change, Editor},
    normal_example::Mode,
};

#[test]
fn normal_commands_preserve_original_attributes_history_and_roundtrips() {
    for mode in [Mode::Sharp, Mode::Smooth, Mode::Winding] {
        let mut editor = Editor::default();
        editor.apply(Action::Close).unwrap();
        editor
            .apply(Action::AddNormals(mode))
            .unwrap();
        let id = editor.scene.objects()[0].id;
        let source = editor.scene.objects()[0].geometry().unwrap();
        let weak = std::rc::Rc::downgrade(source);
        let original = source.to_proto();
        if matches!(mode, Mode::Smooth) {
            assert_eq!(source.vertex_attribute(1, "nx"), Some(1.0));
            assert_eq!(source.vertex_attribute(0, "nz"), Some(1.0));
        }
        let bytes = document::snapshot(&editor.scene).unwrap();
        let loaded = document::load(&bytes).unwrap();
        assert_eq!(
            loaded.meshes[0].geometry.to_proto(),
            original
        );
        drop(loaded);
        assert_eq!(
            editor
                .apply(Action::NormalView(true))
                .unwrap(),
            Change::View
        );
        assert!(editor.normal_view);
        editor.apply(Action::Undo).unwrap();
        assert!(editor.scene.objects().is_empty());
        assert!(weak.upgrade().is_some());
        editor.apply(Action::Redo).unwrap();
        assert_eq!(editor.scene.objects()[0].id, id);
        assert!(editor.normal_view);
        editor.apply(Action::Close).unwrap();
        assert!(weak.upgrade().is_none());
    }
}
```

## <a id="checkpoint-41d-grid"></a>6. Bound the ground grid and its camera depth range

Prepare a finite bounded ground grid with current BRG axis colours and include its distant corners in the camera depth range.

### 1. `src/grid.rs`

Validate and prepare finite grid lines, current axis colours and depth-reach reference points.

Create the file and type:

```rust
use crate::stroke::Stroke;

pub const AXIS_COLOURS: [[f32; 4]; 3] = [
    [0.910, 0.278, 0.545, 1.0],
    [0.604, 0.804, 0.196, 1.0],
    [0.129, 0.588, 0.918, 1.0],
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub spacing: f64,
    pub half_cells: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            spacing: 1.0,
            half_cells: 5,
        }
    }
}

impl Settings {
    pub fn half(self) -> Result<f64, &'static str> {
        let half = self.spacing * self.half_cells as f64;
        if !self.spacing.is_finite()
            || self.spacing <= 0.0
            || self.half_cells == 0
            || self.half_cells > 500
            || !(half as f32).is_finite()
            || self.spacing as f32 == 0.0
        {
            return Err("Grid spacing or extent is invalid");
        }
        Ok(half)
    }

    pub fn points(self) -> Result<Vec<[f64; 3]>, &'static str> {
        let h = self.half()?;
        Ok(vec![
            [-h, -h, 0.0],
            [-h, h, 0.0],
            [h, -h, 0.0],
            [h, h, 0.0],
            [h, 0.0, 0.0],
            [0.0, h, 0.0],
            [0.0, 0.0, self.spacing],
        ])
    }

    pub fn reach(self, target: [f64; 3]) -> Result<f64, &'static str> {
        if target.iter().any(|v| !v.is_finite()) {
            return Err("Grid target must be finite");
        }
        let reach = self
            .points()?
            .iter()
            .map(|p| {
                (p[0] - target[0])
                    .hypot(p[1] - target[1])
                    .hypot(p[2] - target[2])
            })
            .fold(0.0, f64::max);
        if !reach.is_finite() {
            return Err("Grid target exceeds the depth range");
        }
        Ok(reach)
    }

    pub fn strokes(self) -> Result<Vec<Stroke>, &'static str> {
        let half = self.half()? as f32;
        let n = self.half_cells as i32;
        let mut strokes = Vec::new();
        for k in -n..=n {
            let t = (k as f64 * self.spacing) as f32;
            for (start, end) in [
                ([-half, t, 0.0], [half, t, 0.0]),
                ([t, -half, 0.0], [t, half, 0.0]),
            ] {
                strokes.push(Stroke {
                    start,
                    end,
                    colour: [0.55, 0.55, 0.55, 1.0],
                    width: 1.0,
                });
            }
        }
        for (end, colour) in [
            [half, 0.0, 0.0],
            [0.0, half, 0.0],
            [0.0, 0.0, self.spacing as f32],
        ]
        .into_iter()
        .zip(AXIS_COLOURS)
        {
            strokes.push(Stroke {
                start: [0.0; 3],
                end,
                colour,
                width: 2.0,
            });
        }
        Ok(strokes)
    }
}
```

### 2. `src/lib.rs`

Register bounded ground-grid preparation and camera-range checks.

**Locate this existing block:**

```rust
mod gpu_fault_tests;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod gpu_fault_tests;
pub mod grid;
#[cfg(test)]
mod grid_source_tests;
#[cfg(test)]
```

### 3. `src/camera.rs`

Retain the enabled grid extent independently of source scene bounds. Keep prior camera drawing unchanged until actual grid input enables it. Include grid corners in enabled far depth without altering source Fit bounds.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = self.distance * NEAR_FRACTION;
        let far = (self.distance * 10.0).max(self.distance + self.radius * 2.0);
        let projection = match self.projection {
```

Replace it with:

```rust
        );
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let near = self.distance * NEAR_FRACTION;
        let reach = self
            .grid
            .and_then(|grid| grid.reach(self.target).ok())
            .map_or(self.radius, |reach| self.radius.max(reach));
        let far = (self.distance * 10.0).max(self.distance + 2.0 * reach);
        let projection = match self.projection {
```

**Locate this block:**

```rust
            ),
            aspect: 640.0 / 480.0,
            projection: Projection::Perspective,
        }
    }
}
```

Replace it with:

```rust
            ),
            aspect: 640.0 / 480.0,
            projection: Projection::Perspective,
            grid: None,
        }
    }
}
```

**Locate this block:**

```rust
    pub projection: Projection,
}

impl Default for Camera {
```

Replace it with:

```rust
    pub projection: Projection,
    pub grid: Option<crate::grid::Settings>,
}

impl Default for Camera {
```

### 4. `src/grid_source_tests.rs`

Copy checked grid capacity, source colour values, invalid input and actual camera corner-depth checks.

Create the file and type:

```rust
use crate::{
    camera::{Camera, Projection},
    grid::{AXIS_COLOURS, Settings},
};

#[test]
fn ground_grid_has_bounded_finite_lines_and_current_axis_colours() {
    let grid = Settings::default();
    let strokes = grid.strokes().unwrap();
    assert_eq!(strokes.len(), 25);
    assert!(
        strokes
            .iter()
            .all(|s| s.values().iter().all(|v| v.is_finite()))
    );
    assert_eq!(strokes[22].colour, AXIS_COLOURS[0]);
    assert_eq!(strokes[22].end, [5.0, 0.0, 0.0]);
    assert_eq!(strokes[23].colour, AXIS_COLOURS[1]);
    assert_eq!(strokes[24].colour, AXIS_COLOURS[2]);
    assert_eq!(strokes[24].end, [0.0, 0.0, 1.0]);
    assert_eq!(
        Settings {
            half_cells: 500,
            ..grid
        }
        .strokes()
        .unwrap()
        .len(),
        2005
    );
    for bad in [
        Settings {
            spacing: 0.0,
            ..grid
        },
        Settings {
            spacing: f64::NAN,
            ..grid
        },
        Settings {
            spacing: f64::MAX,
            ..grid
        },
        Settings {
            half_cells: 501,
            ..grid
        },
        Settings {
            half_cells: 0,
            ..grid
        },
    ] {
        assert!(bad.strokes().is_err());
    }
    assert!(grid.reach([f64::NAN, 0.0, 0.0]).is_err());
    assert_eq!(grid.reach([0.0; 3]).unwrap(), 50.0f64.sqrt());
}

#[test]
fn small_scene_camera_depth_reaches_visible_distant_grid_corners_without_changing_fit()
{
    for projection in [
        Projection::Perspective,
        Projection::Orthographic,
    ] {
        let mut camera = Camera::default();
        camera.projection = projection;
        camera.isometric();
        let bounds = crate::bounds::Bounds {
            min: [-0.1; 3],
            max: [0.1; 3],
        };
        camera.fit(&bounds);
        let original = (camera.target, camera.radius, camera.distance);
        let without = camera.view_projection();
        camera.grid = Some(Settings::default());
        let with = camera.view_projection();
        assert_eq!(
            (camera.target, camera.radius, camera.distance),
            original
        );
        let mut previously_clipped = 0;
        for point in Settings::default().points().unwrap() {
            let p = session_rust::Point::new(point[0], point[1], point[2]);
            let w = with.m[3] * point[0]
                + with.m[7] * point[1]
                + with.m[11] * point[2]
                + with.m[15];
            if w > 0.0 {
                let old = without.transform_point(&p);
                let new = with.transform_point(&p);
                if old[2] > 1.0 {
                    previously_clipped += 1;
                }
                assert!(
                    new[2] <= 1.0,
                    "An in-front grid reference cannot exceed the far depth"
                );
            }
        }
        assert!(
            previously_clipped > 0,
            "This fixture must exercise grid clipping under a small-scene range"
        );
    }
}
```

### 5. `src/camera.rs`

Cover enabled grid corners in the same orthographic depth interval.

**Locate this existing block:**

```rust
                let w = h * self.aspect;
                let depth_range = (self.distance + 2.0 * self.radius).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                let w = h * self.aspect;
                let depth_range = (self.distance + 2.0 * reach).max(1.0e-6);
                Xform::orthographic(-w, w, -h, h, -depth_range, depth_range)
```

## <a id="checkpoint-41e-drawing"></a>7. Draw a depth-correct grid with shared stroke buffers

Draw the bounded grid and BRG axes using the shared CSS stroke layout, with actual surface depth and exact independent grid-resource accounting.

### 1. `src/depth.rs`

Hide coplanar grid ink without changing source strokes’ equal-depth drawing policy.

**Locate this existing block:**

```rust
}

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
pub fn grid() -> wgpu::DepthStencilState {
    state(false, wgpu::CompareFunction::Less)
}

fn state(write: bool, compare: wgpu::CompareFunction) -> wgpu::DepthStencilState {
```

### 2. `src/stroke_gpu.rs`

Share stroke preparation while choosing a separate grid depth policy. Use the caller’s explicit ink or grid depth policy.

**Locate this existing block:**

```rust
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: stride,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: Some(crate::depth::ink()),
            multisample: Default::default(),
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ) -> Self {
        Self::layout_depth(
            device,
            format,
            source,
            stride,
            vertices_per_instance,
            attributes,
            crate::depth::ink(),
        )
    }

    pub fn grid(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self::layout_depth(
            device,
            format,
            crate::stroke::SHADER,
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
            crate::depth::grid(),
        )
    }

    fn layout_depth(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &'static str,
        stride: u64,
        vertices_per_instance: u32,
        attributes: &[wgpu::VertexAttribute],
        depth: wgpu::DepthStencilState,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vertex"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: stride,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(),
            depth_stencil: Some(depth),
            multisample: Default::default(),
```

### 3. `src/renderer.rs`

Own grid display data independently from document geometry. Create an empty depth-correct grid pipeline. Retain the grid lane for the renderer lifetime. Validate grid settings before mutation and expose actual grid bytes and uploads. Include initialized grid payload in cumulative uploads without mixing current allocations. Update only grid view settings when the camera or display density changes. Draw the grid against actual surface depth before original source ink.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.strokes.draw(&mut pass);
```

Replace it with:

```rust
            for mesh in &self.meshes {
                mesh.draw(&mut pass);
            }
            self.grid.draw(&mut pass);
            self.strokes.draw(&mut pass);
```

**Locate this block:**

```rust
            self.size,
            self.density,
        );
        let [r, g, b] = background.rgb();
        let mut encoder = self
            .device
```

Replace it with:

```rust
            self.size,
            self.density,
        );
        self.grid.view(
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
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
```

Replace it with:

```rust
            + self.strokes.uploaded_bytes
            + self.paths.uploaded_bytes
            + self.markers.uploaded_bytes
            + self.grid.uploaded_bytes
    }

    pub fn set_grid(
        &mut self,
        settings: Option<crate::grid::Settings>,
    ) -> Result<(), &'static str> {
        if settings == self.grid_settings {
            return Ok(());
        }
        let strokes = settings
            .map(crate::grid::Settings::strokes)
            .transpose()?
            .unwrap_or_default();
        self.grid.set(&self.device, strokes)?;
        self.grid_settings = settings;
        Ok(())
    }

    pub fn grid_usage(&self) -> [u64; 4] {
        let [count, bytes] = self.grid.usage();
        [
            count,
            bytes,
            self.grid.uploads,
            self.grid.uploaded_bytes,
        ]
    }

    pub fn stats(&self) -> [usize; 4] {
        [
            self.geometry.uploads,
```

**Locate this block:**

```rust
            strokes,
            paths,
            markers,
            size: [640, 480],
            density: 1.0,
        };
```

Replace it with:

```rust
            strokes,
            paths,
            markers,
            grid,
            grid_settings: None,
            size: [640, 480],
            density: 1.0,
        };
```

**Locate this block:**

```rust
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

Replace it with:

```rust
        let strokes = crate::stroke_gpu::Lane::new(&device, format);
        let paths = crate::stroke_gpu::Lane::joined(&device, format);
        let markers = crate::stroke_gpu::Lane::markers(&device, format);
        let grid = crate::stroke_gpu::Lane::grid(&device, format);
        let mut renderer = Self {
            device,
            queue,
```

**Locate this block:**

```rust
    markers: crate::stroke_gpu::Lane,
    size: [u32; 2],
    density: f32,
}
```

Replace it with:

```rust
    markers: crate::stroke_gpu::Lane,
    grid: crate::stroke_gpu::Lane,
    grid_settings: Option<crate::grid::Settings>,
    size: [u32; 2],
    density: f32,
}
```

## <a id="checkpoint-41f-input"></a>8. Inspect the ground grid through actual commands

### 1. `src/editor.rs`

Expose checked viewport grid state through editor input. Validate finite grid settings before changing viewport state. Reset camera pose while preserving the existing grid view setting.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                        let aspect = self.camera.aspect;
                        self.camera = Camera {
                            aspect,
                            ..Camera::default()
```

Replace it with:

```rust
                        let aspect = self.camera.aspect;
                        self.camera = Camera {
                            aspect,
                            grid: self.camera.grid,
                            ..Camera::default()
```

**Locate this block:**

```rust
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
```

Replace it with:

```rust
                match view {
                    Action::Background => self.background.toggle(),
                    Action::NormalView(visible) => self.normal_view = visible,
                    Action::Grid(settings) => {
                        if let Some(grid) = settings {
                            grid.half()?;
                            grid.reach(self.camera.target)?;
                        }
                        self.camera.grid = settings;
                    }
                    Action::Zoom(factor) => self.camera.zoom(factor),
                    Action::Pan(dx, dy) => self.camera.pan(dx, dy),
                    Action::Orbit(yaw, pitch) => self.camera.orbit(yaw, pitch),
```

**Locate this block:**

```rust
    NormalView(bool),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
```

Replace it with:

```rust
    NormalView(bool),
    Grid(Option<crate::grid::Settings>),
    AddNormals(crate::normal_example::Mode),
    Controls(bool),
    AddPolyline,
```

### 2. `src/browser.rs`

Expose actual grid input in dock completion and Help. Dispatch real viewport grid commands through checked editor actions. Synchronize cached grid state before reporting or presenting the actual frame. Expose actual enabled state and grid allocation/upload counters.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            .map(|id| format!("{id:?}"))
            .unwrap_or_default(),
    )?;
```

Replace it with:

```rust
            .map(|id| format!("{id:?}"))
            .unwrap_or_default(),
    )?;
    canvas.set_attribute(
        "data-grid-enabled",
        if editor.camera.grid.is_some() {
            "true"
        } else {
            "false"
        },
    )?;
    canvas.set_attribute(
        "data-grid-gpu",
        &serde_json::to_string(&renderer.grid_usage()).unwrap(),
    )?;
```

**Locate this block:**

```rust
                    }
                }
            }
            if let Err(error) = inspect(&editor, &renderer) {
                report(&format!(
                    "Cannot report projection: {error:?}"
```

Replace it with:

```rust
                    }
                }
            }
            if let Err(error) = renderer.set_grid(editor.camera.grid) {
                report(error);
                return;
            }
            if let Err(error) = inspect(&editor, &renderer) {
                report(&format!(
                    "Cannot report projection: {error:?}"
```

**Locate this block:**

```rust
                        }
                        "view normals on" => Action::NormalView(true),
                        "view normals off" => Action::NormalView(false),
                        "example sharp" => {
                            Action::AddNormals(crate::normal_example::Mode::Sharp)
                        }
```

Replace it with:

```rust
                        }
                        "view normals on" => Action::NormalView(true),
                        "view normals off" => Action::NormalView(false),
                        "view grid on" => {
                            Action::Grid(Some(crate::grid::Settings::default()))
                        }
                        "view grid off" => Action::Grid(None),
                        "example sharp" => {
                            Action::AddNormals(crate::normal_example::Mode::Sharp)
                        }
```

**Locate this block:**

```rust
            "View Normals Off",
            "Example Sharp",
            "Example Smooth",
            "Example Winding",
```

Replace it with:

```rust
            "View Normals Off",
            "View Grid On",
            "View Grid Off",
            "Example Sharp",
            "Example Smooth",
            "Example Winding",
```

### 3. `src/lib.rs`

Register atomic grid state, camera reset and source-history checks.

**Locate this existing block:**

```rust
#[cfg(test)]
mod grid_source_tests;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
#[cfg(test)]
mod grid_input_tests;
#[cfg(test)]
mod grid_source_tests;
```

### 4. `src/grid_input_tests.rs`

Copy validation atomicity, reset preservation, view-only history and independent source/grid release checks.

Create the file and type:

```rust
use crate::{
    editor::{Action, Change, Editor},
    grid::Settings,
    normal_example::Mode,
};

#[test]
fn grid_commands_preserve_source_history_camera_settings_and_viewport_lifetime() {
    let mut editor = Editor::default();
    editor.apply(Action::Close).unwrap();
    editor
        .apply(Action::AddNormals(Mode::Sharp))
        .unwrap();
    let id = editor.scene.objects()[0].id;
    let weak = std::rc::Rc::downgrade(editor.scene.objects()[0].geometry().unwrap());
    assert_eq!(
        editor
            .apply(Action::Grid(Some(Settings::default())))
            .unwrap(),
        Change::View
    );
    editor
        .apply(Action::Orbit(0.2, 0.1))
        .unwrap();
    editor.apply(Action::ResetView).unwrap();
    assert_eq!(editor.camera.grid, Some(Settings::default()));
    editor.apply(Action::Undo).unwrap();
    assert!(editor.scene.objects().is_empty());
    editor.apply(Action::Redo).unwrap();
    assert_eq!(editor.scene.objects()[0].id, id);
    let before = editor.camera.grid;
    assert!(
        editor
            .apply(Action::Grid(Some(Settings {
                spacing: f64::NAN,
                ..Settings::default()
            })))
            .is_err()
    );
    assert_eq!(editor.camera.grid, before);
    editor.apply(Action::Close).unwrap();
    assert!(weak.upgrade().is_none());
    assert_eq!(editor.camera.grid, before);
    editor.apply(Action::Grid(None)).unwrap();
    assert!(editor.camera.grid.is_none());
}
```

## Build & Run

![Recorded visual output for Lesson 5.7 (41f-input)](../../../screenshots/journey/41f-input-browser.png)

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 5.7
npm --prefix ../session_tests run course -- save 5.7
```

Continue with [6.1 · A bounded integer picking pass](../../consolidation/42-window.md).
