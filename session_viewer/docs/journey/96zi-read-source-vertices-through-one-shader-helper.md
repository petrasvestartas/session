# 20.1.2 · Read source vertices through one shader helper

**Chapter 20 · Shared GPU rendering · Section 20.1 · Shared geometry and primitive addresses** · [Contents](contents.md)

**Typing: 21–42 minutes.** [Estimate](typing-load.md).

A storage array of f32 has a four-byte stride.

## Type

Continue from [Bind the shared geometry as storage](96zh-bind-the-shared-geometry-as-storage.md). [Save or recover your work](recovery.md).

### 1. `src/triangle.wgsl`

Give ordinary and instanced vertex functions one object access boundary. Pass one object record explicitly through the existing geometry transform. Type the shared index-to-vertex storage reader; its resources remain inactive until the vertex entry calls it. Use one object lookup for ordinary and instanced vertex invocations.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>) -> VertexOutput { return geometry(position, colour, normal); }
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<u32>,
    normal_matrix: mat4x4<f32>,
    face_colour:vec4<f32>,
}
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index: u32) -> ObjectSettings { return object_settings; }

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
    @location(1) world: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) @interpolate(flat) clip_flags:u32,
    @location(4) @interpolate(flat) mirrored:u32,
}

fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings) -> VertexOutput {
    var output: VertexOutput;
    let world = object.model * vec4<f32>(position, 1.0);
    output.position = view.transform * world;
    let displayed=select(colour,object.face_colour.xyz,object.face_colour.w>0.5);
    output.colour = select(displayed, vec3<f32>(1.0, 0.75, 0.05), object.selection.x != 0u);
    output.mirrored=select(0u,1u,determinant(mat3x3<f32>(object.model[0].xyz,object.model[1].xyz,object.model[2].xyz))<0.);
    output.world = world.xyz;output.clip_flags=object.selection.z;
    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    return output;
}

@fragment
fn fragment(input: VertexOutput, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    // Derivatives require neighbouring fragments; calculate the sharp face direction before choosing a potentially smooth interpolated source direction.
    let derivative = cross(dpdx(input.world), dpdy(input.world));
    // Screen derivatives ignore vertex order; front-facing restores the original face orientation for reversed winding and world-normal inspection.
    let face = select(derivative, -derivative, front);
    if(clip_active()&&clip_cut(input.clip_flags,input.world)){discard;}
    let closed=(input.clip_flags&8192u)!=0u;
    let source_front=(front!=(input.mirrored!=0u))!=((input.clip_flags&16384u)!=0u);
    if((input.clip_flags&32768u)!=0u || (closed&&!source_front&&view.display.y>0.&&view.display.y<1.)){discard;}
    let direction = select(face, input.normal, dot(input.normal, input.normal) > 0.0);
    let scale = max(max(abs(direction.x), abs(direction.y)), abs(direction.z));
    let bounded = direction / max(scale, 1e-30);
    let normal = bounded / max(length(bounded), 1e-30);
    if view.display.x > 0.5 { return vec4<f32>(normal * 0.5 + vec3<f32>(0.5), select(1.,view.display.y,closed)); }
    let light = normalize(vec3<f32>(0.4, -0.6, 1.0));
    // abs lights both sides of an open face; 0.3 keeps unlit faces readable.
    let brightness = 0.3 + 0.7 * abs(dot(normal, light));
    return vec4<f32>(input.colour * brightness, select(1.,view.display.y,closed));
}

@group(3) @binding(0) var<storage, read> mesh_values: array<f32>;
@group(3) @binding(1) var<storage, read> mesh_normals: array<f32>;
@group(3) @binding(2) var<storage, read> mesh_indices: array<u32>;
struct MeshInput { position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, }
fn mesh_vertex(occurrence: u32) -> MeshInput {
    let index = mesh_indices[occurrence];
    let v = index * 6u;
    let n = index * 3u;
    return MeshInput(
        vec3<f32>(mesh_values[v], mesh_values[v+1u], mesh_values[v+2u]),
        vec3<f32>(mesh_values[v+3u], mesh_values[v+4u], mesh_values[v+5u]),
        vec3<f32>(mesh_normals[n], mesh_normals[n+1u], mesh_normals[n+2u]),
    );
}

@vertex
fn vertex(@location(0) position: vec3<f32>, @location(1) colour: vec3<f32>, @location(2) normal: vec3<f32>, @builtin(instance_index) instance: u32) -> VertexOutput { return geometry(position, colour, normal, object_at(instance)); }
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-1.wgsl"
```

```text
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-2.wgsl"
```

```text
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-3.wgsl"
```

```text
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-4.wgsl"
```

</details>

### 2. `src/instance_gpu.rs`

Customize only the object storage declaration and lookup; retain one geometry function and one vertex entry.

<details>
<summary>Locate the existing block</summary>

```rust
        let source=include_str!("triangle.wgsl").replace("var<uniform> object: ObjectSettings;","var<storage, read> objects: array<ObjectSettings>;").replace("fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>)","fn geometry(position: vec3<f32>, colour: vec3<f32>, normal: vec3<f32>, object: ObjectSettings)").replace("@location(2) normal: vec3<f32>) -> VertexOutput { return geometry(position, colour, normal); }","@location(2) normal: vec3<f32>, @builtin(instance_index) instance: u32) -> VertexOutput { return geometry(position, colour, normal, objects[instance]); }");
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
        let source=include_str!("triangle.wgsl")
            .replace("var<uniform> object_settings: ObjectSettings;", "var<storage, read> objects: array<ObjectSettings>;")
            .replace("return object_settings;", "return objects[index];");
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-5.rs"
```

</details>

### 3. `src/triangle_pick.wgsl`

Keep indexed object and source-face picking on the shared explicit object record without activating their storage vertex path.

<details>
<summary>Locate the existing block</summary>

```wgsl
@group(2) @binding(0) var<uniform> crop:Crop;
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32}
@vertex fn pick_vertex(@location(0) position:vec3<f32>,@location(1) colour:vec3<f32>,@location(2) normal:vec3<f32>)->MeshPickOutput{let placed=geometry(position,colour,normal);return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags);}
@fragment fn pick_fragment(input:MeshPickOutput)->@location(0) vec2<u32>{if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}return vec2<u32>(select(0u,object.selection.y,crop.mode==0u),0u);}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-6.wgsl"
```

### 4. `src/face_pick.wgsl`

Keep indexed object and source-face picking on the shared explicit object record without activating their storage vertex path.

<details>
<summary>Locate the existing block</summary>

```wgsl
struct PackedVertex { x: f32, y: f32, z: f32, r: f32, g: f32, b: f32 }
struct FaceSettings { base: u32, _a: u32, _b: u32, _c: u32 }
@group(2) @binding(0) var<storage, read> positions: array<PackedVertex>;
@group(2) @binding(1) var<storage, read> indices: array<u32>;
@group(2) @binding(2) var<storage, read> faces: array<u32>;
@group(2) @binding(3) var<uniform> source: FaceSettings;
@group(3) @binding(0) var<uniform> crop: Crop;
struct FaceOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) face: u32,
    @location(1) world:vec3<f32>,@location(2) @interpolate(flat) flags:u32,
}
@vertex
fn face_vertex(@builtin(vertex_index) index: u32) -> FaceOutput {
    let vertex = positions[indices[index]];
    let placed = geometry(vec3<f32>(vertex.x, vertex.y, vertex.z), vec3<f32>(vertex.r, vertex.g, vertex.b), vec3<f32>(0.0));
    return FaceOutput(crop.transform * placed.position, faces[index / 3u] + source.base,placed.world,placed.clip_flags);
}
@fragment
fn face_fragment(input: FaceOutput) -> @location(0) vec2<u32> {
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return vec2<u32>(object.selection.y, (0x20000000u | input.face) + 1u);
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```wgsl
--8<-- "journey/code/96zi-read-source-vertices-through-one-shader-helper-7.wgsl"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 20.1.2
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

This supporting frame has no instances; use the supplied original Session specimen for the browser exercise.

![Native renderer output for read source vertices through one shader helper.](../screenshots/journey/96u-fit-original-box-corners-to-the-free-canvas.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [19.3.3 · Fit original box corners to the free canvas](96u-fit-original-box-corners-to-the-free-canvas.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

Read the source index first, then its six position/color scalars and three normal scalars. The source buffers remain unchanged. Pass ObjectSettings explicitly to the existing geometry function. A small object_at helper selects a uniform for ordinary drawing or a storage record for instancing, replacing the fragile geometry-signature rewrites. Update both indexed picking shaders to pass their uniform record explicitly too. Keep the current vertex attributes active for this preparatory boundary; the next step connects the storage reader.

Follow gpu_geometry.rs, triangle.wgsl, gpu_mesh.rs, instance_gpu.rs and renderer.rs in this cumulative checkpoint; compare the maintained engine/gpu/ and shaders/triangle.wgsl ownership..



Why does an index occurrence differ from a source vertex?

Several triangles may reuse one source vertex. A nonindexed invocation number identifies the occurrence; reading the index buffer then recovers the shared position and attributes.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Rebuild ordinary and instanced scenes. The same geometry and colors must remain visible before activation.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 20.1.2
npm --prefix ../session_tests run course -- save 20.1.2
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Prepare shared vertex pulling for current primitive addressing without a second geometry upload. Primitive MRT, packed vertices, MSAA and current finite visibility remain subsequent bridges.

Chrome loads this checkpoint and passes the startup, command focus and navigation checks.

[Full validation scope](release.md).

</details>
