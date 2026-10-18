# 20.3.3 · Retain RGBA in prepared displays

**Chapter 20 · Shared GPU rendering · Section 20.3 · Shared physical inputs and packed formats** · [Contents](contents.md)

**Typing: 14–28 minutes.** [Estimate](typing-load.md).

Derived display vertices can carry a contact-fill marker in their alpha byte.

## Type

Continue from [Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). [Save or recover your work](recovery.md).

### 1. `src/mesh.rs`

Retain alpha beside the existing derived vertex columns. Keep ordinary RGB-created display vertices opaque by default. Use one validated RenderMesh conversion for ordinary kernel displays and derived feature displays. Keep length and finiteness invariants with the display owner. Account for the retained CPU alpha column.

<details>
<summary>Locate the existing block</summary>

```rust
#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 6]>, indices: Vec<u32>) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err("Indices must describe complete triangles within the draw limit");
        }
        if indices.iter().any(|&index| index as usize >= vertices.len()) {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices.iter().flatten().any(|value| !value.is_finite()) {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        Ok(Self { vertices, normals, indices, parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let vertices = render.vertices.iter().map(|vertex| {
            let [x, y, z] = vertex.position;
            let [r, g, b, _] = vertex.color;
            [x, y, z, r, g, b]
        }).collect();
        let normals = render.vertices.iter().map(|vertex| vertex.normal).collect();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::new(vertices, render.indices)?.with_normals(normals)?;
        display.parts = parts; Ok(display)
    }

    pub fn with_normals(mut self, normals: Vec<[f32; 3]>) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() { return Err("Each display vertex needs one normal"); }
        self.normals = normals.into_iter().map(crate::normals::unit).collect::<Result<_, _>>()?; Ok(self)
    }

    pub fn normals(&self) -> &[[f32; 3]] { &self.normals }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>() + self.parts.bytes()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kernel_box_becomes_eight_vertices_and_twelve_triangles() {
        let source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        let mesh = Mesh::from_kernel(&source).unwrap();
        assert_eq!(mesh.vertices().len(), 8);
        assert_eq!(mesh.indices().len(), 36);
    }

    #[test]
    fn invalid_connections_and_nonfinite_vertices_are_rejected() {
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 3]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1]).is_err());
        assert!(Mesh::new(vec![[f32::NAN; 6]; 3], vec![0, 1, 2]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).is_ok());
    }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
#[derive(Clone)]
pub struct Mesh {
    vertices: Vec<[f32; 6]>,
    normals: Vec<[f32; 3]>,
    alphas: Vec<f32>,
    indices: Vec<u32>,
    pub parts: crate::mesh_parts::Parts,
}

impl Mesh {
    pub fn new(vertices: Vec<[f32; 6]>, indices: Vec<u32>) -> Result<Self, &'static str> {
        if indices.len() % 3 != 0 || indices.len() > u32::MAX as usize {
            return Err("Indices must describe complete triangles within the draw limit");
        }
        if indices.iter().any(|&index| index as usize >= vertices.len()) {
            return Err("A triangle refers to a missing vertex");
        }
        if vertices.iter().flatten().any(|value| !value.is_finite()) {
            return Err("Vertex values must be finite");
        }
        let normals = vec![[0.0; 3]; vertices.len()];
        let alphas = vec![1.; vertices.len()];
        Ok(Self { vertices, normals, alphas, indices, parts: Default::default() })
    }

    pub fn from_kernel(mesh: &session_rust::Mesh) -> Result<Self, &'static str> {
        let render = mesh.to_render();
        let parts = crate::mesh_parts::Parts::new(mesh, render.indices.len() / 3)?;
        let mut display = Self::from_render(render)?;
        display.parts = parts; Ok(display)
    }

    pub fn from_render(render:session_rust::render_mesh::RenderMesh)->Result<Self,&'static str>{
        let vertices = render.vertices.iter().map(|vertex| {
            let [x, y, z] = vertex.position;
            let [r, g, b, _] = vertex.color;
            [x, y, z, r, g, b]
        }).collect();
        let normals = render.vertices.iter().map(|vertex| vertex.normal).collect();
        let alphas = render.vertices.iter().map(|vertex| vertex.color[3]).collect();
        Self::new(vertices, render.indices)?.with_normals(normals)?.with_alphas(alphas)
    }

    pub fn with_normals(mut self, normals: Vec<[f32; 3]>) -> Result<Self, &'static str> {
        if normals.len() != self.vertices.len() { return Err("Each display vertex needs one normal"); }
        self.normals = normals.into_iter().map(crate::normals::unit).collect::<Result<_, _>>()?; Ok(self)
    }

    pub fn with_alphas(mut self, alphas:Vec<f32>)->Result<Self,&'static str>{
        if alphas.len()!=self.vertices.len()||alphas.iter().any(|v|!v.is_finite()){
            return Err("Each display vertex needs finite alpha");
        }
        self.alphas=alphas;Ok(self)
    }
    pub fn alphas(&self)->&[f32]{&self.alphas}
    pub fn normals(&self) -> &[[f32; 3]] { &self.normals }

    pub fn payload_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<[f32; 6]>()
            + self.indices.capacity() * std::mem::size_of::<u32>()
            + self.normals.capacity() * std::mem::size_of::<[f32; 3]>()
            + self.alphas.capacity() * std::mem::size_of::<f32>() + self.parts.bytes()
    }

    pub fn vertices(&self) -> &[[f32; 6]] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u32] {
        &self.indices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_kernel_box_becomes_eight_vertices_and_twelve_triangles() {
        let source = session_rust::Mesh::create_box(0.8, 0.8, 0.8);
        let mesh = Mesh::from_kernel(&source).unwrap();
        assert_eq!(mesh.vertices().len(), 8);
        assert_eq!(mesh.indices().len(), 36);
    }

    #[test]
    fn invalid_connections_and_nonfinite_vertices_are_rejected() {
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 3]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1]).is_err());
        assert!(Mesh::new(vec![[f32::NAN; 6]; 3], vec![0, 1, 2]).is_err());
        assert!(Mesh::new(vec![[0.0; 6]; 3], vec![0, 1, 2]).is_ok());
    }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-1.rs"
```

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-2.rs"
```

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-3.rs"
```

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-4.rs"
```

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-5.rs"
```

</details>

### 2. `src/packed_vertex.rs`

Encode the actual display alpha instead of an opaque constant. Use the same clamped RGBA quantization for all four channels. Pack the validated columns together without another encoder.

<details>
<summary>Locate the existing block</summary>

```rust
pub const NO_NORMAL:u32=0x8000_8000;
pub fn oct32(normal:[f32;3])->u32{
 let length=normal.iter().map(|v|v.abs()).sum::<f32>();
 if length==0.||length.is_nan(){return NO_NORMAL;}
 let (x,y)=(normal[0]/length,normal[1]/length);
 let sign=|v:f32|if v<0.{-1.}else{1.};
 let (x,y)=if normal[2]<0.{((1.-y.abs())*sign(x),(1.-x.abs())*sign(y))}else{(x,y)};
 let half=|v:f32|u32::from((v*32767.).round().clamp(-32766.,32767.)as i16 as u16);
 half(x)|(half(y)<<16)
}
pub fn pack(vertex:[f32;6],normal:[f32;3])->[u32;5]{
 let color=[vertex[3],vertex[4],vertex[5],1.].map(|v|(v.clamp(0.,1.)*255.).round()as u8);
 [vertex[0].to_bits(),vertex[1].to_bits(),vertex[2].to_bits(),oct32(normal),u32::from_le_bytes(color)]
}
pub fn bytes(mesh:&crate::mesh::Mesh)->Vec<u8>{
 mesh.vertices().iter().zip(mesh.normals()).flat_map(|(v,n)|pack(*v,*n)).flat_map(u32::to_ne_bytes).collect()
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub const NO_NORMAL:u32=0x8000_8000;
pub fn oct32(normal:[f32;3])->u32{
 let length=normal.iter().map(|v|v.abs()).sum::<f32>();
 if length==0.||length.is_nan(){return NO_NORMAL;}
 let (x,y)=(normal[0]/length,normal[1]/length);
 let sign=|v:f32|if v<0.{-1.}else{1.};
 let (x,y)=if normal[2]<0.{((1.-y.abs())*sign(x),(1.-x.abs())*sign(y))}else{(x,y)};
 let half=|v:f32|u32::from((v*32767.).round().clamp(-32766.,32767.)as i16 as u16);
 half(x)|(half(y)<<16)
}
pub fn pack(vertex:[f32;6],normal:[f32;3],alpha:f32)->[u32;5]{
 let color=[vertex[3],vertex[4],vertex[5],alpha].map(|v|(v.clamp(0.,1.)*255.).round()as u8);
 [vertex[0].to_bits(),vertex[1].to_bits(),vertex[2].to_bits(),oct32(normal),u32::from_le_bytes(color)]
}
pub fn bytes(mesh:&crate::mesh::Mesh)->Vec<u8>{
 mesh.vertices().iter().zip(mesh.normals()).zip(mesh.alphas()).flat_map(|((v,n),a)|pack(*v,*n,*a)).flat_map(u32::to_ne_bytes).collect()
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-6.rs"
```

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-7.rs"
```

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-8.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 3. `src/packed_vertex_tests.rs`

Keep the existing opaque quantization contract explicit.

<details>
<summary>Locate the existing block</summary>

```text
pack(v,[0.;3])
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-9.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 4. `src/display_rgba_tests.rs`

Supply ordinary kernel and derived contact conversion, exact RGBA, unchanged source and column-validation fixtures.

Copy this supplied file:

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-10.rs"
```

</details>

### 5. `src/lib.rs`

Connect the supplied alpha-retention fixtures to the authoring harness.

<details>
<summary>Locate the existing block</summary>

```rust
pub mod packed_vertex;
```

</details>

After that block, type:

```rust
#[cfg(test)]
mod display_rgba_tests;
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-11.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/memory.rs`

Include the three retained alpha values in the exact shared-snapshot memory fixture.

<details>
<summary>Locate the existing block</summary>

```text
baseline[4] + 16 * 24 + 16 * 4 + 3 * 12
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zze-retain-rgba-in-prepared-displays-memory-alpha.rs" skip=39
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.3.3
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

One shared display conversion retains ordinary and derived RGBA, including contact markers.

### Recorded visual result

**Previous native render — current verification pending.**

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for retain rgba in prepared displays.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The kernel emits ordinary mesh vertices with opaque alpha; the viewer display walk assigns the contact marker afterward. Add one shared RenderMesh conversion so both ordinary and derived displays retain RGBA. Keep validation, original ownership and CPU memory accounting together. Ordinary shading and picks remain unchanged here; this data bridge prepares contact-fill rendering.

Follow ordinary or derived RenderVertex alpha through one Mesh conversion to the packed RGBA byte..



Why retain an alpha value that ordinary surface shading does not use?

The current contact-fill shader recognizes a special alpha byte as a marker. Dropping it during display conversion would erase that distinction before rendering.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Inspect an original CAD surface, toggle normal view and pick its faces. Confirm that the display remains unchanged; the supplied authoring fixture separately verifies contact-marker bytes and original payload retention.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.3.3
npm --prefix ../session_tests run course -- save 20.3.3
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

The current display walk adds CONTACT_COLOR to derived RenderVertex values after kernel rendering. The teaching display now preserves that RGBA through one shared conversion and compact upload. Contact classification, depth layering and feature contact geometry still require connection; contact fills do not render yet.

The supplied fixture checks opaque kernel display vertices, the derived contact marker, exact packed RGBA bytes, unchanged source payload and validated alpha columns.

[Full validation scope](release.md).

</details>
