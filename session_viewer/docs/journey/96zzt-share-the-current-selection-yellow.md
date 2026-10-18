# 20.6.5 · Share the current selection yellow

**Chapter 20 · Shared GPU rendering · Section 20.6 · Current file and command behavior** · [Contents](contents.md)

**Typing: 8–16 minutes.** [Estimate](typing-load.md).

Read selection yellow from the Session color palette. Reuse it in temporary CPU highlights and generate the shared WGSL constant from those same components. Surface selection changes presentation; it must preserve authored colors, original owners and GPU storage.

## Type

Continue from [Reuse full source preparation for editor and reload](96zzs-reuse-full-source-preparation-for-editor-and-reload.md). [Save or recover your work](recovery.md).

### 1. `src/colour.rs`

Define selection presentation once from the existing Session palette.

<details>
<summary>Locate the existing block</summary>

```rust
        Mode::Points => mesh.set_pointcolors(vec![red.clone(), green.clone(), blue.clone(), red, green, blue]),
        Mode::Faces => mesh.set_facecolors(vec![red, blue]),
    }
    mesh
}
```

</details>

After that block, type:

```rust

pub fn selection()->[f32;4]{Color::yellow().to_unified_array()}
pub fn selection_shader()->String{
    let [r,g,b,_]=selection();
    format!("const SELECTION_COLOR: vec3<f32> = vec3<f32>({r:?}, {g:?}, {b:?});")
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-1.rs"
```

</details>

### 2. `src/highlight.rs`

Use the shared palette in the existing temporary overlay owner.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
        self.mesh = None; let yellow = [1., 0.75, 0.05, 1.];
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
```

</details>

Replace it with:

```rust
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
        self.mesh = None; let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = [1., 0.75, 0.05, 1.];
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
```

</details>

Replace it with:

```rust
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::{gpu_mesh::GpuMesh, pick_sources::{Part, Target}, scene::Scene, stroke_gpu::Lane};
use std::rc::Rc;

pub struct Highlight { mesh: Option<GpuMesh>, strokes: Lane, paths: Lane, markers: Lane }

impl Highlight {
    pub fn set_opacity(&self,value:f32){for lane in [&self.strokes,&self.paths,&self.markers]{lane.set_opacity(value);}}

    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self { mesh: None, strokes: Lane::new(device, format), paths: Lane::joined(device, format), markers: Lane::markers(device, format) }
    }

    pub fn enable_finite(&mut self,device:&wgpu::Device,format:wgpu::TextureFormat){for lane in [&mut self.strokes,&mut self.paths,&mut self.markers]{lane.enable_finite(device,format)}}
    pub fn finite_frame(&self,device:&wgpu::Device,tables:&crate::finite_ink::Tables,depth:&wgpu::TextureView){for lane in [&self.strokes,&self.paths,&self.markers]{lane.finite_frame(device,tables,depth)}}
    pub fn set(&mut self, device: &wgpu::Device, layout: &wgpu::BindGroupLayout, scene: &Scene, target: Option<Target>) -> Result<(), &'static str> {
        self.mesh = None; let mut strokes = Vec::new(); let mut paths = Vec::new(); let mut markers = Vec::new();
        let yellow = crate::colour::selection();
        if let Some(target) = target.filter(|t| t.valid(scene)) { match target.part {
            Part::Face(key) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let face = row.mesh.parts.faces.iter().position(|&f| f == key).unwrap() as u32;
                // Highlights own derived display data; changing selection changes presentation without mutating source geometry or adding a document history transaction.
                let mut vertices = Vec::new(); let mut normals = Vec::new(); let mut indices = Vec::new();
                for (tri, &source) in row.mesh.indices().chunks_exact(3).zip(&row.mesh.parts.triangles) { if source == face {
                    for &index in tri { indices.push(vertices.len() as u32); vertices.push(row.mesh.vertices()[index as usize]); normals.push(row.mesh.normals()[index as usize]); }
                } }
                let display = crate::mesh::Mesh::new(vertices, indices)?.with_normals(normals)?;
                let mut object = row.clone(); object.mesh = Rc::new(display);
                self.mesh = Some(GpuMesh::upload(device, layout, &object, true));
            }
            Part::BRepEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let points=&row.cad.as_ref().unwrap().edges.iter().find(|border|border.edge==edge).unwrap().points;
                for pair in points.windows(2) {let ends=[pair[0],pair[1]].map(|p|row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::SurfaceEdge(edge)=>{
                let row=scene.objects().iter().find(|r|r.id==target.owner).unwrap();let surface=row.surface.as_ref().unwrap();let boundary=surface.grid.boundaries.iter().find(|b|b.edge==edge).unwrap();
                for ends in &boundary.segments {let ends=ends.map(|index|{let p=surface.grid.samples[index as usize].point;row.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32()});strokes.push(crate::stroke::Stroke {start:ends[0],end:ends[1],colour:yellow,width:3.});}
            }
            Part::MeshEdge(a, b) => {
                let row = scene.objects().iter().find(|r| r.id == target.owner).unwrap();
                let index = row.mesh.parts.edges.iter().position(|&edge| edge == (a, b)).unwrap();
                let ends = row.mesh.parts.endpoints[index].map(|p| row.model.transform_point(&session_rust::Point::new(p[0], p[1], p[2])).to_f32());
                strokes.push(crate::stroke::Stroke { start: ends[0], end: ends[1], colour: yellow, width: 3. });
            }
            Part::PathEdge(index) => {
                let row = scene.paths().iter().find(|r| r.id == target.owner).unwrap(); let points = row.points();
                strokes.push(crate::stroke::Stroke { start: points[index].to_f32(), end: points[index + 1].to_f32(), colour: yellow, width: row.prepared.width + 2. });
            }
            Part::Curve => {
                if let Some(row) = scene.lines().iter().find(|r| r.id == target.owner) {
                    strokes.push(crate::stroke::Stroke { colour: yellow, width: row.prepared.display.width + 2., ..row.stroke() });
                } else if let Some(row) = scene.paths().iter().find(|r| r.id == target.owner) {
                    paths.extend(row.segments().into_iter().map(|mut segment| { segment.stroke.colour = yellow; segment.stroke.width += 2.; segment }));
                }
            }
            Part::Control(_) | Part::SourceControl(_) => {
                let marker = scene.control_markers().into_iter().find(|m| m.parent == target.owner && m.part == target.part).unwrap();
                markers.push(crate::marker::Marker { colour: [1., 1., 0., 1.], ..marker.display });
            }
            Part::CloudPoint(id)=>{
                let p=crate::cloud_pick::position(scene,target.owner,id).ok_or("Cloud point is missing")?;
                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::SheetEntity(_)|Part::Object => {}
        } }
        let flag=target.filter(|t|t.valid(scene)).map_or(0,|t|if crate::clipping_rows::owners(scene).contains(&t.owner){4096}else{0});let count=strokes.len();self.strokes.set(device, strokes)?;self.strokes.set_flags(device,vec![flag;count])?; self.paths.set_chains(device, paths)?; self.markers.set_points(device, markers)?; Ok(())
    }

    pub fn objects(&mut self, device: &wgpu::Device, scene: &Scene, ids: &[crate::scene::ObjectId]) -> Result<(), &'static str> {
        self.mesh = None; let yellow = crate::colour::selection();
        self.strokes.set(device, scene.lines().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::stroke::Stroke { colour: yellow, width: r.prepared.display.width + 2., ..r.stroke() }).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|r.mesh.parts.endpoints.iter().map(move|pair|{let ends=pair.map(|p|r.model.transform_point(&session_rust::Point::new(p[0],p[1],p[2])).to_f32());crate::stroke::Stroke{start:ends[0],end:ends[1],colour:yellow,width:3.}}))))?;
        let planes=crate::clipping_rows::owners(scene);let flags=scene.lines().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)).map(|_|0).chain(scene.objects().iter().filter(|r|ids.contains(&r.id)&&scene.visible(r.id)&&r.mesh.indices().is_empty()).flat_map(|r|std::iter::repeat_n(if planes.contains(&r.id){4096}else{0},r.mesh.parts.endpoints.len()))).collect();self.strokes.set_flags(device,flags)?;
        self.paths.set_chains(device, scene.paths().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).flat_map(|r|
            r.segments().into_iter().map(|mut s| { s.stroke.colour = yellow; s.stroke.width += 2.; s })))?;
        self.markers.set_points(device, scene.points().iter().filter(|r| ids.contains(&r.id) && scene.visible(r.id)).map(|r|
            crate::marker::Marker { colour: yellow, diameter: r.prepared.display.diameter + 4., ..r.marker() }))?;
        Ok(())
    }

    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        for lane in [&self.strokes, &self.paths, &self.markers] { lane.view(queue, transform, size, density); }
    }

    pub fn mesh(&self, pass: &mut wgpu::RenderPass<'_>) { if let Some(mesh) = &self.mesh { mesh.draw_pulled(pass); } }
    pub fn ink(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.paths.draw(pass); }
    pub fn points(&self, pass: &mut wgpu::RenderPass<'_>) { self.markers.draw(pass); }
    pub fn clipping_bytes(&self)->[u64;2]{[&self.strokes,&self.paths,&self.markers].into_iter().map(Lane::clipping_bytes).fold([0;2],|a,b|[a[0]+b[0],a[1]+b[1]])}
    pub fn bytes(&self) -> u64 { self.mesh.as_ref().map_or(0, |m| m.geometry.allocated_bytes() + m.settings_bytes())
        + self.strokes.usage()[1] + self.paths.usage()[1] + self.markers.usage()[1] }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-2.rs"
```

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-3.rs"
```

</details>

### 3. `src/tool_preview.rs`

Use the shared palette in the existing temporary overlay owner.

<details>
<summary>Locate the existing block</summary>

```rust
        let yellow = [1., 0.75, 0.05, 1.]; let mut strokes = Vec::new(); let mut markers = Vec::new();
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-4.rs"
```

### 4. `src/clipping_gpu.rs`

Include the same generated selection constant in the shared shader assembly.

<details>
<summary>Locate the existing block</summary>

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn source(base:&str,group:u32,binding:u32)->String{format!("{}\n@group({group}) @binding({binding}) var<uniform> clipping:ClipUniform;\n{}\n{base}",crate::colour::selection_shader(),concat!(include_str!("packed_vertex.wgsl"),"\n",include_str!("physical.wgsl"),"\n",include_str!("clip.wgsl")))}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-5.rs"
```

</details>

### 5. `src/triangle.wgsl`

Read selection presentation from the shared shader constant.

<details>
<summary>Locate the existing block</summary>

```wgsl
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), (object.selection.x & 1u) != 0u);
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    output.colour = select(displayed, SELECTION_COLOR, (object.selection.x & 1u) != 0u);
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-6.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/global_outline_tests.rs`

Update the existing exact contact-selection pixel assertion to the current palette.

<details>
<summary>Locate the existing block</summary>

```text
&[255,191,13]
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-7.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 7. `src/global_outline_tests.rs`

Supply actual selected pixels, original picks, exact restoration, stable uploads and release assertions.

<details>
<summary>Locate the existing block</summary>

```text
sert_eq!(gpu.stats()[0],uploads);assert_eq!(source.alphas(),vec![1./255.;vertices]);
 scene.clear();gpu.set_scene(&scene,None);assert_eq!(gpu.usage(),[0;4]);
}
```

</details>

After that block, append this supplied code:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-8.rs" skip=160
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 8. `src/pick_gpu_tests.rs`

Require current yellow in the original surface and temporary-preview GPU fixtures; preserve ownership and upload checks.

<details>
<summary>Locate the existing block</summary>

```text
assert!(p[0] > p[1] && p[1] > p[2]);
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-9.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 9. `src/pick_gpu_tests.rs`

Require current yellow in the original surface and temporary-preview GPU fixtures; preserve ownership and upload checks.

<details>
<summary>Locate the existing block</summary>

```text
assert!(p[0]>p[1] && p[1]>p[2], "The preview is yellow: {p:?}");
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzt-share-the-current-selection-yellow-10.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.6.5
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

![Native renderer output for share the current selection yellow.](../screenshots/journey/96zzd-pack-surface-position-normal-and-color.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [20.3.2 · Use compact vertices in every surface pass](96zzd-pack-surface-position-normal-and-color.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



Session palette → CPU highlight data and shared WGSL → selected display.



Why generate the shader constant from the CPU palette?

Both sides must use the same components. A shared definition prevents surface selection and temporary overlays from drifting apart, without adding a uniform or changing the object record.

Study estimate, including typing and experiments: 0.8–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Select a surface, then select one of its original edges or controls. Compare their yellow. Clear selection and confirm the original colors return; Undo should still describe geometry edits.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.6.5
npm --prefix ../session_tests run course -- save 20.6.5
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

This matches current selection yellow. Lighting, Arctic background, depth, contact boundaries and the remaining renderer bridges still need implementation and acceptance.

Actual Chrome selection pixels use current yellow at DPR 1; clearing selection restores the scene exactly without geometry uploads. The retained Close and Opacity contract also passes at actual DPR 1 and 2.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 20.6.5
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
