# 17.2 · Finite ink visibility and original colour/pick ownership

**Steps:** 25 · **Source:** [engine/gpu/triangle_tiles.rs](../../../src/engine/gpu/triangle_tiles.rs) · [shaders/project_triangles.wgsl](../../../src/shaders/project_triangles.wgsl) · [shaders/ink_visibility.wgsl](../../../src/shaders/ink_visibility.wgsl) · [shaders/ribbon.wgsl](../../../src/shaders/ribbon.wgsl)

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">16×16 Screen Tile Grid + Compute Frustum Culling</text>
  <rect x="64" y="48" width="212" height="106" fill="#f8fafc" stroke="#111111" stroke-width="1.75"/>
  <line x1="117" y1="48" x2="117" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="170" y1="48" x2="170" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="223" y1="48" x2="223" y2="154" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="64" y1="83" x2="276" y2="83" stroke="#cbd5e1" stroke-width="1"/>
  <line x1="64" y1="118" x2="276" y2="118" stroke="#cbd5e1" stroke-width="1"/>
  <rect x="117" y="83" width="106" height="35" fill="#fef08a" fill-opacity="0.65" stroke="#d97706" stroke-width="1.75"/>
  <polygon points="128,112 170,62 212,112" fill="none" stroke="#111111" stroke-width="2"/>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Section Clip: dot(P, n) + d &lt; 0 -&gt; Discard + Closed Cap Loop</text>
  <polygon points="434,140 484,64 584,64 534,140" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <line x1="510" y1="46" x2="510" y2="158" stroke="#dc2626" stroke-width="2.5" stroke-dasharray="5 3"/>
  <polygon points="510,64 584,64 534,140 510,140" fill="#fee2e2" fill-opacity="0.5" stroke="#dc2626" stroke-width="1.5" stroke-dasharray="3 3"/>
  <line x1="510" y1="64" x2="510" y2="140" stroke="#dc2626" stroke-width="4"/>
  <polygon points="614,72 688,72 688,132 614,132" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Clipped Solid + Hatched Cross-Section Cap</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="842,138 886,68 948,68 948,138" fill="#93c5fd" stroke="#111111" stroke-width="2"/>
  <polygon points="948,68 986,82 986,146 948,138" fill="#fef08a" stroke="#dc2626" stroke-width="2.2"/>
  <line x1="952" y1="84" x2="982" y2="114" stroke="#dc2626" stroke-width="1.5"/>
  <line x1="952" y1="108" x2="982" y2="138" stroke="#dc2626" stroke-width="1.5"/>
</svg>
</div>

## <a id="checkpoint-86s-test-visibility-at-the-original-center-line"></a>1. Test visibility at the center line

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

### 1. `src/finite_visibility.wgsl`

Create the file and type:

```wgsl
// The tutorial still converts forward depth; allow two float rounding units until the reversed camera connects.
const FINITE_GRID:bool=false;
@group(1) @binding(0) var<storage,read> finite_projected:array<ProjectedTriangle>;
@group(1) @binding(1) var<storage,read> finite_tiles:array<vec4<u32>>;
@group(1) @binding(2) var finite_depth:texture_depth_2d;
fn finite_physical(pixel:vec2<f32>,depth:f32,tolerance:f32)->bool{let at=finite_depth_pixel(pixel);let size=vec2<i32>(textureDimensions(finite_depth));if(any(at<vec2<i32>(0))||any(at>=size)){return false;}let z=1.0-textureLoad(finite_depth,at,0);return select(z<=depth+tolerance,z<depth-tolerance,FINITE_GRID);}
fn finite_screen(p:vec4<f32>)->vec2<f32>{return(p.xy/p.w*vec2<f32>(0.5,-0.5)+0.5)*view.size;}
fn finite_axis(a:vec4<f32>,b:vec4<f32>)->vec4<f32>{return vec4<f32>(finite_screen(a),finite_screen(b));}
fn finite_visible(pixel:vec2<f32>,axis:vec4<f32>,depths:vec2<f32>)->bool{
 let delta=axis.zw-axis.xy;let length2=dot(delta,delta);let t=clamp(dot(pixel-axis.xy,delta)/max(length2,1e-20),0.0,1.0);let at=axis.xy+t*delta;let depth=mix(depths.x,depths.y,t);let relative=abs(depth)*1.9073486e-6+abs(1.0-depth)*2.3841858e-7;
 if(any(at<vec2<f32>(0.0))||any(at>=view.size)){return finite_physical(pixel,depth,relative);}
 if(finite_tiles[0].x==0u){return finite_physical(pixel,depth,relative);}
 let span=f32(visibility_tile_span_of(u32(view.size.x),u32(view.size.y)));let size=vec2<u32>(ceil(view.size/span));let cell=vec2<u32>(at/span);let head=finite_tiles[1u+cell.y*size.x+cell.x];
 if(head.w!=0u||head.z!=head.x){return finite_physical(pixel,depth,relative);}
 for(var i=0u;i<head.x;i++){
  let offset=head.y+i*2u;let nearest=bitcast<f32>(finite_tiles[(offset+1u)/4u][(offset+1u)%4u]);if(nearest<depth-relative||(!FINITE_GRID&&nearest<=depth+relative)){continue;}
  let primitive=finite_tiles[offset/4u][offset%4u];if(primitive==0u||primitive>arrayLength(&finite_projected)){continue;}
  let triangle=finite_projected[primitive-1u];let hit=projected_triangle_at(triangle,at);let tolerance=relative+0.00390625*(abs(triangle.gradient.x)+abs(triangle.gradient.y));
  if(hit.y>0.5&&select(hit.x>depth+tolerance,hit.x>=depth-tolerance,FINITE_GRID)){return false;}
 }
 return true;
}
```

## <a id="checkpoint-86t-reuse-original-stroke-coverage"></a>2. Reuse stroke coverage

### 1. `src/stroke_coverage.wgsl`

Create the file and type:

```wgsl
fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.half_width + 0.5 - abs(input.side), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

### 2. `src/stroke.wgsl`

**Locate this existing block:**

```wgsl
}

fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.half_width + 0.5 - abs(input.side), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}

@vertex
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}

#include "stroke_coverage.wgsl"

@vertex
```

### 3. `src/stroke.rs`

**Locate this existing block:**

```rust
pub const SHADER: &str = include_str!("stroke.wgsl");
use std::rc::Rc;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn shader() -> String {
    crate::finite_project_source::expand(include_str!("stroke.wgsl"))
}
use std::rc::Rc;
```

### 4. `src/finite_project_source.rs`

**Locate this existing block:**

```rust
        )
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
        .replace(
            "#include \"stroke_coverage.wgsl\"",
            include_str!("stroke_coverage.wgsl"),
        )
}
```

### 5. `src/stroke_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    fn layout_depth(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &'static str,
        stride: u64,
```

Replace it with:

```rust
    fn layout_depth(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &str,
        stride: u64,
```

**Locate this block:**

```rust
        Self::layout_depth(
            device,
            format,
            crate::stroke::SHADER,
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
```

Replace it with:

```rust
        Self::layout_depth(
            device,
            format,
            &crate::stroke::shader(),
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
```

**Locate this block:**

```rust
        Self::layout_depth(
            device,
            format,
            crate::stroke::SHADER,
            44,
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],
```

Replace it with:

```rust
        Self::layout_depth(
            device,
            format,
            &crate::stroke::shader(),
            44,
            6,
            &wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32],
```

**Locate this block:**

```rust
    fn layout(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &'static str,
        stride: u64,
        vertices_per_instance: u32,
        attributes: &[wgpu::VertexAttribute],
```

Replace it with:

```rust
    fn layout(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        source: &str,
        stride: u64,
        vertices_per_instance: u32,
        attributes: &[wgpu::VertexAttribute],
```

**Locate this block:**

```rust
            format,
            crate::stroke::SHADER,
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
        )
```

Replace it with:

```rust
            format,
            &crate::stroke::shader(),
            44,
            6,
            &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x4, 3 => Float32],
        )
```

### 6. `src/finite_stroke.wgsl`

Reuse the shader you have already written:

```sh
cp src/stroke.wgsl src/finite_stroke.wgsl
```

## <a id="checkpoint-86u-carry-stroke-axis-and-depth"></a>3. Carry stroke axis and depth

### 1. `src/finite_stroke.wgsl`

**Locate this existing block:**

```wgsl
    @location(2) @interpolate(flat) half_width: f32,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0);
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
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

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(2) @interpolate(flat) half_width: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}

fn hidden(colour: vec4<f32>) -> Output {
    return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, 0.0, 0.0,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
}

fn geometry(start: vec3<f32>, end: vec3<f32>, colour: vec4<f32>, width: f32, index: u32) -> Output {
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
    return Output(position, colour, corner.y * extent, half_width,finite_axis(a,b),vec2<f32>(1.0-a.z/a.w,1.0-b.z/b.w),finite_screen(position));
}

#include "stroke_coverage.wgsl"

@vertex
fn vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths)){discard;}return colour(input);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

## <a id="checkpoint-86v-reuse-original-stroke-integer-coverage"></a>4. Reuse stroke integer coverage

### 1. `src/finite_stroke_pick.wgsl`

Reuse the shader you have already written:

```sh
cp src/stroke_pick.wgsl src/finite_stroke_pick.wgsl
```

## <a id="checkpoint-86w-keep-stroke-crop-ownership-aligned"></a>5. Keep stroke crop ownership aligned

### 1. `src/finite_stroke_pick.wgsl`

**Locate this existing block:**

```wgsl
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
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
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
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.side, input.half_width,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths)){discard;}
    return input.id;
```

## <a id="checkpoint-86x-reuse-original-chain-coverage"></a>6. Reuse chain coverage

### 1. `src/chain_coverage.wgsl`

Create the file and type:

```wgsl
fn colour(input: Output) -> vec4<f32> {
    let local = input.pixel - input.origin;
    let distance = dot(local, perpendicular(input.direction));
    var coverage = clamp(input.half_width + 0.5 - abs(distance), 0.0, 1.0);
    if (input.part == 1u) {
        let back = -dot(local, input.direction);
        if (back >= input.head_length) { discard; }
        let side = (0.4 * back - abs(distance)) / sqrt(1.16);
        let base = select(input.head_length - back, 1e6, abs(distance) <= input.half_width);
        coverage = clamp(min(min(side, back), base) + 0.5, 0.0, 1.0);
    }
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

### 2. `src/chain.wgsl`

**Locate this existing block:**

```wgsl
}
fn colour(input: Output) -> vec4<f32> {
    let local = input.pixel - input.origin;
    let distance = dot(local, perpendicular(input.direction));
    var coverage = clamp(input.half_width + 0.5 - abs(distance), 0.0, 1.0);
    if (input.part == 1u) {
        let back = -dot(local, input.direction);
        if (back >= input.head_length) { discard; }
        let side = (0.4 * back - abs(distance)) / sqrt(1.16);
        let base = select(input.head_length - back, 1e6, abs(distance) <= input.half_width);
        coverage = clamp(min(min(side, back), base) + 0.5, 0.0, 1.0);
    }
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}

@vertex
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
#include "chain_coverage.wgsl"

@vertex
```

### 3. `src/chain.rs`

**Locate this existing block:**

```rust
pub const SHADER: &str = include_str!("chain.wgsl");
use crate::stroke::PreparedLine;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn shader() -> String {
    crate::finite_project_source::expand(include_str!("chain.wgsl"))
}
use crate::stroke::PreparedLine;
```

### 4. `src/finite_project_source.rs`

**Locate this existing block:**

```rust
        )
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
        .replace(
            "#include \"chain_coverage.wgsl\"",
            include_str!("chain_coverage.wgsl"),
        )
}
```

### 5. `src/stroke_gpu.rs`

**Locate this existing block:**

```rust
            format,
            crate::chain::SHADER,
            72,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            format,
            &crate::chain::shader(),
            72,
```

### 6. `src/finite_chain.wgsl`

Reuse the shader you have already written:

```sh
cp src/chain.wgsl src/finite_chain.wgsl
```

## <a id="checkpoint-86y-carry-chain-axis-and-depth"></a>7. Carry chain axis and depth

### 1. `src/finite_chain.wgsl`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```wgsl
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> { return colour(input); }
```

Replace it with:

```wgsl
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32) -> Output { return geometry(start, end, colour, width, previous, next, heads, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths)){discard;}return colour(input);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

**Locate this block:**

```wgsl
        * 2.0 / view.size * position.w, position.zw);
    let offset = select(left, right, corner.x > 0.5) * (half_width + 0.70711) * corner.y;
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0);
}
#include "chain_coverage.wgsl"
```

Replace it with:

```wgsl
        * 2.0 / view.size * position.w, position.zw);
    let offset = select(left, right, corner.x > 0.5) * (half_width + 0.70711) * corner.y;
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, screen(position), screen(a), direction, half_width, 0u, 0.0,finite_axis(a,b),vec2<f32>(1.0-a.z/a.w,1.0-b.z/b.w),finite_screen(position));
}
#include "chain_coverage.wgsl"
```

**Locate this block:**

```wgsl
        let offset = -aim * mix(-0.70711, head_length + 0.70711, corner.x)
            + perpendicular(aim) * (head_length * 0.4 + 0.70711) * corner.y;
        let position = vec4<f32>(tip.xy + offset * 2.0 / view.size * tip.w, tip.zw);
        return Output(position, colour, screen(position), screen(tip), aim, half_width, 1u, head_length);
    }
    let start_offset = direction * select(0.0, head_length, at_start);
    let end_offset = -direction * select(0.0, head_length, at_end);
```

Replace it with:

```wgsl
        let offset = -aim * mix(-0.70711, head_length + 0.70711, corner.x)
            + perpendicular(aim) * (head_length * 0.4 + 0.70711) * corner.y;
        let position = vec4<f32>(tip.xy + offset * 2.0 / view.size * tip.w, tip.zw);
        return Output(position, colour, screen(position), screen(tip), aim, half_width, 1u, head_length,finite_axis(a,b),vec2<f32>(1.0-a.z/a.w,1.0-b.z/b.w),finite_screen(position));
    }
    let start_offset = direction * select(0.0, head_length, at_start);
    let end_offset = -direction * select(0.0, head_length, at_end);
```

**Locate this block:**

```wgsl
    @location(6) @interpolate(flat) head_length: f32,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
```

Replace it with:

```wgsl
    @location(6) @interpolate(flat) head_length: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
fn screen(p: vec4<f32>) -> vec2<f32> { return p.xy / p.w * view.size * 0.5; }
fn perpendicular(d: vec2<f32>) -> vec2<f32> { return vec2<f32>(-d.y, d.x); }
```

## <a id="checkpoint-86z-reuse-original-chain-integer-coverage"></a>8. Reuse chain integer coverage

### 1. `src/finite_chain_pick.wgsl`

Reuse the shader you have already written:

```sh
cp src/chain_pick.wgsl src/finite_chain_pick.wgsl
```

## <a id="checkpoint-86aa-keep-chain-crop-ownership-aligned"></a>9. Keep chain crop ownership aligned

### 1. `src/finite_chain_pick.wgsl`

**Locate this existing block:**

```wgsl
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, previous, next, heads, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.pixel = source.pixel;
    output.origin = source.origin;
    output.direction = source.direction;
    output.half_width = source.half_width;
    output.part = source.part;
    output.head_length = source.head_length;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.pixel, input.origin, input.direction, input.half_width, input.part, input.head_length);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    return input.id;
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) start: vec3<f32>, @location(1) end: vec3<f32>,
    @location(2) colour: vec4<f32>, @location(3) width: f32,
    @location(4) previous: vec3<f32>, @location(5) next: vec3<f32>,
    @location(6) heads: u32, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(start, end, colour, width, previous, next, heads, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.pixel = source.pixel;
    output.origin = source.origin;
    output.direction = source.direction;
    output.half_width = source.half_width;
    output.part = source.part;
    output.head_length = source.head_length;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.pixel, input.origin, input.direction, input.half_width, input.part, input.head_length,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths)){discard;}
    return input.id;
```

## <a id="checkpoint-86ab-reuse-original-marker-coverage"></a>10. Reuse marker coverage

### 1. `src/marker_coverage.wgsl`

Create the file and type:

```wgsl
fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.radius + 0.5 - length(input.offset), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}
```

### 2. `src/marker.wgsl`

**Locate this existing block:**

```wgsl
}
fn colour(input: Output) -> vec4<f32> {
    let coverage = clamp(input.radius + 0.5 - length(input.offset), 0.0, 1.0);
    return vec4<f32>(input.colour.rgb, input.colour.a * coverage);
}

@vertex
```

Edit the existing block to match this code; keep unchanged lines:

```wgsl
}
#include "marker_coverage.wgsl"

@vertex
```

### 3. `src/marker.rs`

**Locate this existing block:**

```rust
pub const SHADER: &str = include_str!("marker.wgsl");
use std::rc::Rc;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub fn shader() -> String {
    crate::finite_project_source::expand(include_str!("marker.wgsl"))
}
use std::rc::Rc;
```

### 4. `src/finite_project_source.rs`

**Locate this existing block:**

```rust
        )
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        )
        .replace(
            "#include \"marker_coverage.wgsl\"",
            include_str!("marker_coverage.wgsl"),
        )
}
```

### 5. `src/stroke_gpu.rs`

**Locate this existing block:**

```rust
            format,
            crate::marker::SHADER,
            32,
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            format,
            &crate::marker::shader(),
            32,
```

### 6. `src/finite_marker.wgsl`

Reuse the shader you have already written:

```sh
cp src/marker.wgsl src/finite_marker.wgsl
```

## <a id="checkpoint-86ac-carry-marker-axis-and-depth"></a>11. Carry marker axis and depth

### 1. `src/finite_marker.wgsl`

**Locate this existing block:**

```wgsl
    @location(2) @interpolate(flat) radius: f32,
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
    @location(2) @interpolate(flat) radius: f32,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
fn geometry(center: vec3<f32>, diameter: f32, colour: vec4<f32>, index: u32) -> Output {
    var position = view.transform * vec4<f32>(center, 1.0);
    if (position.w <= 0.0 || position.z < 0.0 || position.z > position.w) {
        return Output(vec4<f32>(2.0, 2.0, 2.0, 1.0), colour, vec2<f32>(0.0), 0.0,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0));
    }
    let corners = array<vec2<f32>, 6>(vec2<f32>(-1.0, -1.0), vec2<f32>(1.0, -1.0), vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0), vec2<f32>(1.0, -1.0), vec2<f32>(1.0, 1.0));
    let radius = diameter * view.density * 0.5;
    let offset = corners[index] * (radius + 0.70711);
    let axis=finite_axis(position,position);let depths=vec2<f32>(1.0-position.z/position.w);
    position = vec4<f32>(position.xy + offset * 2.0 / view.size * position.w, position.zw);
    return Output(position, colour, offset, radius,axis,depths,finite_screen(position));
}
#include "marker_coverage.wgsl"

@vertex
fn vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32) -> Output { return geometry(center, diameter, colour, index); }
@fragment
fn fragment(input: Output) -> @location(0) vec4<f32> {if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths)){discard;}return colour(input);}


#include "projected_triangle.wgsl"
#include "finite_visibility.wgsl"
fn finite_depth_pixel(pixel:vec2<f32>)->vec2<i32>{return vec2<i32>(pixel);}
```

## <a id="checkpoint-86ad-reuse-original-marker-integer-coverage"></a>12. Reuse marker integer coverage

### 1. `src/finite_marker_pick.wgsl`

Reuse the shader you have already written:

```sh
cp src/marker_pick.wgsl src/finite_marker_pick.wgsl
```

## <a id="checkpoint-86ae-keep-marker-crop-ownership-aligned"></a>13. Keep marker crop ownership aligned

### 1. `src/finite_marker_pick.wgsl`

**Locate this existing block:**

```wgsl
    @location(7) @interpolate(flat) id: vec2<u32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
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
    @location(7) @interpolate(flat) id: vec2<u32>,
    @location(9) @interpolate(flat) finite_axis_value:vec4<f32>,
    @location(10) @interpolate(flat) finite_depths:vec2<f32>,
    @location(11) @interpolate(linear) finite_pixel:vec2<f32>,
}
@vertex
fn pick_vertex(@location(0) center: vec3<f32>, @location(1) diameter: f32,
    @location(2) colour: vec4<f32>, @builtin(vertex_index) index: u32, @location(7) id: vec2<u32>) -> PickOutput {
    let source = geometry(center, diameter, colour, index);
    var output: PickOutput;
    output.position = crop.transform * source.position;
    output.colour = source.colour;
    output.offset = source.offset;
    output.radius = source.radius;
    output.finite_axis_value=source.finite_axis_value;output.finite_depths=source.finite_depths;output.finite_pixel=source.finite_pixel;
    output.id = id; return output;
}
@fragment
fn pick_fragment(input: PickOutput) -> @location(0) vec2<u32> {
    let source = Output(input.position, input.colour, input.offset, input.radius,input.finite_axis_value,input.finite_depths,input.finite_pixel);
    if (!wanted(input.id.y, crop.mode)) { discard; }
    if (colour(source).a <= 0.0) { discard; }
    if(!finite_visible(input.finite_pixel,input.finite_axis_value,input.finite_depths)){discard;}
    return input.id;
```

## <a id="checkpoint-86af-assemble-one-shared-finite-shader-predicate"></a>14. Assemble one shared finite shader predicate

### 1. `src/finite_sources.rs`

Create the file and type:

```rust
pub fn source(stride: u64, pick: bool) -> String {
    source_style(stride, pick, false)
}
pub fn source_style(stride: u64, pick: bool, grid: bool) -> String {
    let (base, ids) = match stride {
        44 => (
            include_str!("finite_stroke.wgsl"),
            include_str!("finite_stroke_pick.wgsl"),
        ),
        72 => (
            include_str!("finite_chain.wgsl"),
            include_str!("finite_chain_pick.wgsl"),
        ),
        32 => (
            include_str!("finite_marker.wgsl"),
            include_str!("finite_marker_pick.wgsl"),
        ),
        _ => panic!("Unsupported finite ink layout"),
    };
    let base = crate::finite_project_source::expand(base).replace(
        "#include \"finite_visibility.wgsl\"",
        include_str!("finite_visibility.wgsl"),
    );
    let base = if grid {
        base.replace(
            "const FINITE_GRID:bool=false;",
            "const FINITE_GRID:bool=true;",
        )
    } else {
        base
    };
    if pick {
        format!(
            "{}\n{}\n{ids}",
            base.replace("@group(1)", "@group(2)")
                .replace(
                    "return vec2<i32>(pixel);",
                    "return vec2<i32>(pixel)-vec2<i32>(i32(crop._a),i32(crop._b));"
                ),
            include_str!("pick_crop.wgsl")
        )
    } else {
        base
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod tile_pipelines;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
pub mod tile_gpu;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod tile_pipelines;

pub mod tile_gpu;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
pub mod finite_sources;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

## <a id="checkpoint-86ag-own-finite-color-and-integer-pipelines"></a>15. Own finite color and integer pipelines

### 1. `src/finite_ink.rs`

Create the file and type:

```rust
use std::cell::RefCell;
pub struct Ink {
    color: wgpu::RenderPipeline,
    pick: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
    bindings: RefCell<
        Option<(
            wgpu::Buffer,
            wgpu::Buffer,
            wgpu::TextureView,
            wgpu::BindGroup,
        )>,
    >,
}
#[derive(Clone)]
pub struct Tables {
    pub projected: wgpu::Buffer,
    pub tiles: wgpu::Buffer,
}
impl Ink {}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod tile_gpu;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
pub mod finite_sources;
#[cfg(test)]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod tile_gpu;

pub mod finite_sources;

pub mod finite_ink;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
```

## <a id="checkpoint-86ah-build-finite-shader-pipelines-with-original-layouts"></a>16. Build finite shader pipelines with layouts

### 1. `src/finite_ink.rs`

**Locate this existing block:**

```rust
use std::cell::RefCell;
pub struct Ink {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
fn pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
    stride: u64,
    grid: bool,
    picking: bool,
    view: &wgpu::BindGroupLayout,
    crop: &wgpu::BindGroupLayout,
    layout: &wgpu::BindGroupLayout,
    attributes: &[wgpu::VertexAttribute],
) -> wgpu::RenderPipeline {
    let source = crate::finite_sources::source_style(stride, picking, grid);
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("finite original ink"),
        source: wgpu::ShaderSource::Wgsl(source.into()),
    });
    let groups = if picking {
        vec![Some(view), Some(crop), Some(layout)]
    } else {
        vec![Some(view), Some(layout)]
    };
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("finite original ink"),
            bind_group_layouts: &groups,
            immediate_size: 0,
        });
    let mut buffers = vec![wgpu::VertexBufferLayout {
        array_stride: stride,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes,
    }];
    let ids = wgpu::vertex_attr_array![7=>Uint32x2];
    if picking {
        buffers.push(wgpu::VertexBufferLayout {
            array_stride: 8,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &ids,
        });
    }
    let mut depth = crate::depth::ink();
    depth.depth_compare = Some(wgpu::CompareFunction::Always);
    depth.bias = Default::default();
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("finite original ink"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some(if picking {
                "pick_vertex"
            } else {
                "vertex"
            }),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(if picking {
                "pick_fragment"
            } else {
                "fragment"
            }),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: if picking {
                    wgpu::TextureFormat::Rg32Uint
                } else {
                    format
                },
                blend: if picking {
                    None
                } else {
                    Some(wgpu::BlendState::ALPHA_BLENDING)
                },
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: Default::default(),
        depth_stencil: Some(depth),
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}
use std::cell::RefCell;
pub struct Ink {
```

## <a id="checkpoint-86ai-bind-finite-coverage-for-original-families"></a>17. Bind finite coverage for families

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

### 1. `src/finite_ink.rs`

**Locate this existing block:**

```rust
}
impl Ink {}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
}
impl Ink {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        stride: u64,
        grid: bool,
    ) -> Self {
        let attributes=match stride{44=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32].to_vec(),72=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Float32x3,5=>Float32x3,6=>Uint32].to_vec(),32=>wgpu::vertex_attr_array![0=>Float32x3,1=>Float32,2=>Float32x4].to_vec(),_=>panic!("Unsupported finite ink layout")};
        let view = crate::gpu_layout::uniform(device, 80);
        let crop = crate::gpu_layout::uniform(device, 80);
        let layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("finite ink coverage"),
                entries: &[
                    crate::tile_pipelines::entry(
                        0,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::BufferBindingType::Storage { read_only: true },
                    ),
                    crate::tile_pipelines::entry(
                        1,
                        wgpu::ShaderStages::FRAGMENT,
                        wgpu::BufferBindingType::Storage { read_only: true },
                    ),
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Depth,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                ],
            });
        let color = pipeline(
            device,
            format,
            stride,
            grid,
            false,
            &view,
            &crop,
            &layout,
            &attributes,
        );
        let pick = pipeline(
            device,
            format,
            stride,
            grid,
            true,
            &view,
            &crop,
            &layout,
            &attributes,
        );
        Self {
            color,
            pick,
            layout,
            bindings: RefCell::new(None),
        }
    }
    fn group(
        &self,
        device: &wgpu::Device,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("finite original ink coverage"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: tables.projected.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: tables.tiles.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(depth),
                },
            ],
        })
    }
}
```

## <a id="checkpoint-86aj-reuse-bindings-and-draw-original-owners"></a>18. Reuse bindings and draw owners

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

### 1. `src/finite_ink.rs`

**Locate this existing block:**

```rust
    }
}
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }
    pub fn set(
        &self,
        device: &wgpu::Device,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) {
        let mut bindings = self.bindings.borrow_mut();
        if bindings
            .as_ref()
            .is_some_and(|(p, t, d, _)| {
                p == &tables.projected && t == &tables.tiles && d == depth
            })
        {
            return;
        }
        let group = self.group(device, tables, depth);
        *bindings = Some((
            tables.projected.clone(),
            tables.tiles.clone(),
            depth.clone(),
            group,
        ));
    }
    pub fn draw(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        vertices: &wgpu::Buffer,
        count: u32,
        verts: u32,
    ) -> bool {
        let bindings = self.bindings.borrow();
        let Some((_, _, _, group)) = bindings.as_ref() else {
            return false;
        };
        pass.set_pipeline(&self.color);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.draw(0..verts, 0..count);
        true
    }
    pub fn pick(
        &self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass<'_>,
        view: &wgpu::BindGroup,
        crop: &wgpu::BindGroup,
        vertices: &wgpu::Buffer,
        ids: &wgpu::Buffer,
        count: u32,
        verts: u32,
        tables: &Tables,
        depth: &wgpu::TextureView,
    ) {
        let group = self.group(device, tables, depth);
        pass.set_pipeline(&self.pick);
        pass.set_bind_group(0, view, &[]);
        pass.set_bind_group(1, crop, &[]);
        pass.set_bind_group(2, &group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
        pass.set_vertex_buffer(1, ids.slice(..));
        pass.draw(0..verts, 0..count);
    }
}
```

## <a id="checkpoint-86ak-expose-original-gpu-models-to-projection"></a>19. Expose GPU models to projection

### 1. `src/gpu_layout.rs`

**Locate this existing block:**

```rust
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT
                | wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
```

## <a id="checkpoint-86al-connect-finite-visibility-to-all-original-ink-lanes"></a>20. Connect finite visibility to all ink lanes

### 1. `src/stroke_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }],
        });
        Self {
            pipeline,
```

Replace it with:

```rust
            }],
        });
        Self {
            finite: None,
            strict_depth,
            pipeline,
```

**Locate this block:**

```rust
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: Some(&layout),
```

Replace it with:

```rust
            label: Some("stroke"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
        let strict_depth = depth.depth_compare == Some(wgpu::CompareFunction::Less);
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("stroke"),
            layout: Some(&layout),
```

**Locate this block:**

```rust
        );
    }

    pub fn pick_bytes(&self) -> u64 {
        self.ids
            .as_ref()
```

Replace it with:

```rust
        );
    }

    pub fn pick_finite(
        &self,
        device: &wgpu::Device,
        pass: &mut wgpu::RenderPass<'_>,
        crop: &wgpu::BindGroup,
        tables: Option<&crate::finite_ink::Tables>,
        depth: &wgpu::TextureView,
    ) {
        if let (Some(ink), Some(tables), Some(vertices), Some(ids)) = (
            &self.finite,
            tables,
            &self.vertices,
            &self.ids,
        ) {
            ink.pick(
                device,
                pass,
                &self.group,
                crop,
                vertices,
                ids,
                (self.data.len() as u64 / self.stride) as u32,
                self.vertices_per_instance,
                tables,
                depth,
            )
        } else {
            self.pick(pass, crop)
        }
    }
    pub fn pick_bytes(&self) -> u64 {
        self.ids
            .as_ref()
```

**Locate this block:**

```rust
        queue.write_buffer(&self.uniform, 0, &bytes);
    }

    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vertices) = &self.vertices else {
            return;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
```

Replace it with:

```rust
        queue.write_buffer(&self.uniform, 0, &bytes);
    }

    pub fn enable_finite(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) {
        self.finite = Some(crate::finite_ink::Ink::new(
            device,
            format,
            self.stride,
            self.strict_depth,
        ));
    }
    pub fn finite_frame(
        &self,
        device: &wgpu::Device,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        if let Some(ink) = &self.finite {
            ink.set(device, tables, depth)
        }
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        let Some(vertices) = &self.vertices else {
            return;
        };
        if self.finite.as_ref().is_some_and(|ink| {
            ink.draw(
                pass,
                &self.group,
                vertices,
                (self.data.len() as u64 / self.stride) as u32,
                self.vertices_per_instance,
            )
        }) {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.group, &[]);
        pass.set_vertex_buffer(0, vertices.slice(..));
```

**Locate this block:**

```rust
pub struct Lane {
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: wgpu::RenderPipeline,
    ids: Option<wgpu::Buffer>,
```

Replace it with:

```rust
pub struct Lane {
    finite: Option<crate::finite_ink::Ink>,
    strict_depth: bool,
    pipeline: wgpu::RenderPipeline,
    pick_pipeline: wgpu::RenderPipeline,
    ids: Option<wgpu::Buffer>,
```

### 2. `src/highlight.rs`

**Locate this existing block:**

```rust
    }

    pub fn set(
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    }

    pub fn enable_finite(
        &mut self,
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
    ) {
        for lane in [
            &mut self.strokes,
            &mut self.paths,
            &mut self.markers,
        ] {
            lane.enable_finite(device, format)
        }
    }
    pub fn finite_frame(
        &self,
        device: &wgpu::Device,
        tables: &crate::finite_ink::Tables,
        depth: &wgpu::TextureView,
    ) {
        for lane in [&self.strokes, &self.paths, &self.markers] {
            lane.finite_frame(device, tables, depth)
        }
    }
    pub fn set(
```

## <a id="checkpoint-86am-separate-integer-depth-writers-and-sampled-ink"></a>21. Separate integer depth writers and sampled ink

### 1. `src/pick_image.rs`

**Locate this existing block:**

```rust
                format: crate::depth::FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
```

Edit the existing block to match this code; keep unchanged lines:

```rust
                format: crate::depth::FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
```

### 2. `src/pick_gpu.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                device: device.clone(),
                queue: queue.clone(),
            },
        )]);
```

Replace it with:

```rust
                device: device.clone(),
                queue: queue.clone(),
            },
            tables,
        )]);
```

**Locate this block:**

```rust
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.extend((request.mode as u32).to_ne_bytes());
        bytes.resize(80, 0);
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
```

Replace it with:

```rust
            .flat_map(|n| n.to_ne_bytes())
            .collect();
        bytes.extend((request.mode as u32).to_ne_bytes());
        bytes.extend(window.draw[0].to_ne_bytes());
        bytes.extend(window.draw[1].to_ne_bytes());
        bytes.resize(80, 0);
        queue.write_buffer(&self.crop, 0, &bytes);
        for lane in ink {
```

**Locate this block:**

```rust
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
```

Replace it with:

```rust
    ) {
        self.submit_finite(
            device, queue, queued, meshes, ink, faces, text, clouds, sheets, None,
        )
    }
    pub(crate) fn submit_finite(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        queued: Queued,
        meshes: &[GpuMesh],
        ink: &[&Lane],
        faces: &crate::face_pick::Faces,
        text: Option<&crate::scene_text_gpu::Lane>,
        clouds: Option<&crate::cloud_gpu::Lane>,
        sheets: Option<&crate::sheet_gpu::Lane>,
        tables: Option<&crate::finite_ink::Tables>,
    ) {
        assert!(
            self.active.is_none(),
            "Only one pick may be in flight"
```

### 3. `src/pick_encode.rs`

**Locate this existing block:**

```rust
    ctx: &crate::text_gpu_support::GpuCtx,
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
        if let Some(clouds) = clouds {
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    ctx: &crate::text_gpu_support::GpuCtx,
    tables: Option<&crate::finite_ink::Tables>,
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
    }
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("finite integer ink"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: None,
                stencil_ops: None,
            }),
            ..Default::default()
        });
        for lane in ink {
            lane.pick_finite(
                device,
                &mut pass,
                crop_group,
                tables,
                &image.depth,
            );
        }
    }
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("other original integer owners"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &image.colour,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &image.depth,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });
        if let Some(clouds) = clouds {
```

## <a id="checkpoint-86an-own-tile-state-before-color-and-first-picks"></a>22. Own tile state before color and first picks

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    pub fn sheet_camera(&self, camera: &crate::camera::Camera) {
        self.sheets.camera(camera);
    }
```

Replace it with:

```rust
    pub fn sheet_camera(&self, camera: &crate::camera::Camera) {
        self.sheets.camera(camera);
    }
    #[cfg(test)]
    pub(crate) fn tile_buffers(&self) -> [wgpu::Buffer; 2] {
        let tiles = self.tiles.borrow();
        [tiles.projected.clone(), tiles.buffer.clone()]
    }
    pub fn tile_usage(&self) -> [u64; 7] {
        self.tiles.borrow().usage()
    }
```

**Locate this block:**

```rust
                self.text.borrow().as_ref(),
                self.clouds.borrow().as_ref(),
                Some(&self.sheets),
            );
        }
    }
```

Replace it with:

```rust
                self.text.borrow().as_ref(),
                self.clouds.borrow().as_ref(),
                Some(&self.sheets),
                Some(&tables),
            );
        }
    }
```

**Locate this block:**

```rust
        }
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            self.picker.submit(
                &self.device,
                &self.queue,
                request,
```

Replace it with:

```rust
        }
        self.prepare_clouds(transform);
        if let Some(request) = self.picker.next(view) {
            let mut bytes = transform
                .iter()
                .flat_map(|v| v.to_ne_bytes())
                .collect::<Vec<_>>();
            bytes.resize(80, 0);
            self.queue
                .write_buffer(&self.uniform, 0, &bytes);
            let tables = {
                let mut tiles = self.tiles.borrow_mut();
                tiles.prepare(&self.device, &self.queue, self.size);
                let mut encoder = self
                    .device
                    .create_command_encoder(&Default::default());
                tiles.encode(
                    &self.device,
                    &mut encoder,
                    &self.view_group,
                    transform,
                );
                self.queue.submit([encoder.finish()]);
                tiles.map_report();
                crate::finite_ink::Tables {
                    projected: tiles.projected.clone(),
                    tiles: tiles.buffer.clone(),
                }
            };
            self.picker.submit_finite(
                &self.device,
                &self.queue,
                request,
```

**Locate this block:**

```rust
            self.meshes.push(row);
        }
        drop(old);
        self.instance_rows = scene.instances.keys().copied().collect();
        self.instances
            .set(
```

Replace it with:

```rust
            self.meshes.push(row);
        }
        drop(old);
        self.tiles
            .borrow_mut()
            .set(&self.device, &self.meshes);
        self.instance_rows = scene.instances.keys().copied().collect();
        self.instances
            .set(
```

**Locate this block:**

```rust
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
```

Replace it with:

```rust
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: crate::depth::FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        depth_texture.create_view(&Default::default())
```

**Locate this block:**

```rust
            sources: Default::default(),
            pick_mode: Default::default(),
        };
        renderer.set_scene(scene, None);
        renderer
    }
```

Replace it with:

```rust
            sources: Default::default(),
            pick_mode: Default::default(),
        };
        for lane in [
            &mut renderer.mesh_ink,
            &mut renderer.strokes,
            &mut renderer.paths,
            &mut renderer.markers,
            &mut renderer.grid,
        ] {
            lane.enable_finite(&renderer.device, format);
        }
        renderer
            .highlight
            .enable_finite(&renderer.device, format);
        renderer.set_scene(scene, None);
        renderer
    }
```

**Locate this block:**

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            guides,
            measurement,
            sheets,
```

Replace it with:

```rust
        let measurement = crate::measure_gpu::Measurement::new(&device, format);
        let guides = crate::guide_gpu::Guide::new(&device, format);
        let mut renderer = Self {
            tiles,
            guides,
            measurement,
            sheets,
```

**Locate this block:**

```rust
        })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
```

Replace it with:

```rust
        })
        };
        let pipeline = mesh_pipeline(crate::depth::surface());
        let tiles = std::cell::RefCell::new(crate::tile_gpu::Tiles::new(
            &device,
            &view_layout,
        ));
        let instances = crate::instance_gpu::Lane::new(&device, format, &view_layout);
        let highlight_pipeline = mesh_pipeline(crate::depth::ink());
        let highlight = crate::highlight::Highlight::new(&device, format);
```

**Locate this block:**

```rust
pub struct Renderer {
    guides: crate::guide_gpu::Guide,
    measurement: crate::measure_gpu::Measurement,
    sheets: crate::sheet_gpu::Lane,
```

Replace it with:

```rust
pub struct Renderer {
    tiles: std::cell::RefCell<crate::tile_gpu::Tiles>,
    guides: crate::guide_gpu::Guide,
    measurement: crate::measure_gpu::Measurement,
    sheets: crate::sheet_gpu::Lane,
```

## <a id="checkpoint-86ao-split-opaque-finite-ink-and-cloud-composition"></a>23. Split opaque finite ink and cloud composition

### 1. `src/renderer.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
            }
        }
        self.queue.submit([encoder.finish()]);
    }
```

Replace it with:

```rust
            }
        }
        self.queue.submit([encoder.finish()]);
        tiles.map_report();
    }
```

**Locate this block:**

```rust
                mesh.draw(&mut pass);
            }
            self.instances.draw(&mut pass);
            if let Some(clouds) = self.clouds.borrow().as_ref() {
                clouds.draw(&mut pass);
            }
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
```

Replace it with:

```rust
                mesh.draw(&mut pass);
            }
            self.instances.draw(&mut pass);
            pass.set_pipeline(&self.highlight_pipeline);
            pass.set_bind_group(0, &self.view_group, &[]);
            self.highlight.mesh(&mut pass);
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cloud depth composite"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
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
                ..Default::default()
            });
            if let Some(clouds) = self.clouds.borrow().as_ref() {
                clouds.draw(&mut pass);
            }
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("finite scene ink"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth,
                        depth_ops: None,
                        stencil_ops: None,
                    },
                ),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            self.highlight.ink(&mut pass);
            self.mesh_ink.draw(&mut pass);
            self.grid.draw(&mut pass);
```

**Locate this block:**

```rust
            .create_command_encoder(&Default::default());
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

Replace it with:

```rust
            .create_command_encoder(&Default::default());
        let mut tiles = self.tiles.borrow_mut();
        tiles.prepare(&self.device, &self.queue, self.size);
        tiles.encode(
            &self.device,
            &mut encoder,
            &self.view_group,
            *transform,
        );
        let tables = crate::finite_ink::Tables {
            projected: tiles.projected.clone(),
            tiles: tiles.buffer.clone(),
        };
        for lane in [
            &self.mesh_ink,
            &self.strokes,
            &self.paths,
            &self.markers,
            &self.grid,
        ] {
            lane.finite_frame(&self.device, &tables, &self.depth);
        }
        self.highlight
            .finite_frame(&self.device, &tables, &self.depth);
        {
            // The pass borrows the encoder. This scope ends that borrow before finish takes it.
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
```

## <a id="checkpoint-86ap-observe-actual-derived-tile-resources"></a>24. Observe actual derived tile resources

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
```

Replace it with:

```rust
        &serde_json::to_string(&placements)
            .map_err(|error| JsValue::from_str(&error.to_string()))?,
    )?;
    canvas.set_attribute(
        "data-triangle-tiles",
        &serde_json::to_string(&renderer.tile_usage()).unwrap(),
    )?;
```

**Locate this block:**

```rust
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
```

Replace it with:

```rust
    listeners.listen(window.as_ref(), "viewer-pick")?;
    listeners.listen(window.as_ref(), "viewer-tiles")?;
    listeners.listen(window.as_ref(), "viewer-publication")?;
    listeners.listen(window.as_ref(), "viewer-sheet-entity")?;
    listeners.listen(window.as_ref(), "viewer-stream-page")?;
```

## <a id="checkpoint-86aq-exercise-finite-color-and-original-picking"></a>25. Exercise finite color and picking

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
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">GPU Offscreen Rg32Uint Texture (owner_id, sub_id)</text>
  <rect x="74" y="48" width="224" height="106" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <rect x="146" y="76" width="80" height="50" fill="#fef08a" stroke="#dc2626" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="105" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">5×5 px Window</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="144" text-anchor="middle" font-family="monospace" font-size="9" fill="#2563eb">COPY_SRC Attachment</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">copy_texture_to_buffer (256B Row Pitch) -&gt; Staging</text>
  <rect x="424" y="52" width="272" height="96" rx="6" fill="#dbeafe" stroke="#2563eb" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Staging Buffer (MAP_READ | COPY_DST)</text>
  <rect x="446" y="90" width="228" height="24" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="106" text-anchor="middle" font-family="monospace" font-size="9" fill="#111111">bytes_per_row = 256 (WebGPU alignment)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">slice.map_async(MapMode::Read, ...)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">CPU Callback Reads Mapped [u32; 2] Winner</text>
  <rect x="806" y="44" width="256" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="58" width="216" height="44" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="78" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">get_mapped_range() -&gt; (id, sub)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="94" text-anchor="middle" font-family="monospace" font-size="9" fill="#16a34a">staging.unmap()</text>
  <rect x="846" y="114" width="176" height="32" rx="4" fill="#fef08a" stroke="#dc2626" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="134" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Selected Owner Updated</text>
</svg>
</div>

### 1. `src/tile_shader_tests.rs`

Create the file and type:

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn current_finite_triangle_shader_recipes_validate_with_explicit_course_bindings() {
    use wgpu::naga;
    let mut sources = vec![
        (
            "project",
            crate::finite_project_source::source(),
        ),
        (
            "tiles",
            crate::finite_project_source::tiles(),
        ),
        ("scan", crate::finite_project_source::scan()),
    ];
    for stride in [44, 72, 32] {
        sources.push((
            "finite color",
            crate::finite_sources::source(stride, false),
        ));
        sources.push((
            "finite pick",
            crate::finite_sources::source(stride, true),
        ));
    }
    for (name, source) in sources {
        let module = naga::front::wgsl::parse_str(&source).unwrap_or_else(|error| {
            panic!("{name}: {}", error.emit_to_string(&source))
        });
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
    }
}
```

### 2. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod finite_ink;
pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod finite_ink;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
#[cfg(test)]
mod tile_shader_tests;

#[cfg(target_arch = "wasm32")]
```

### 3. `src/tile_gpu_tests.rs`

Create the file and type:

```rust
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn actual_finite_gpu_lists_cache_projection_and_drop_deleted_geometry() {
    use crate::{renderer::Renderer, scene::Scene, viewport::Viewport};
    let scene = Scene::demo();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let (device, queue) =
        pollster::block_on(adapter.request_device(&Default::default())).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        panic!("actual finite GPU: {error}")
    }));
    let mut renderer = Renderer::new(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        &scene,
    );
    renderer.resize(Viewport {
        width: 320,
        height: 240,
    });
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("finite actual frame"),
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
    let view = texture.create_view(&Default::default());
    let camera = crate::camera::Camera::default();
    let matrix = camera.uniform();
    renderer.draw(&view, &Default::default(), &matrix);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let buffers = renderer.tile_buffers();
    let read = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: Some("actual projected record and tile header"),
            size: 96 + buffers[1].size(),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_buffer_to_buffer(&buffers[0], 0, &read, 0, 96);
    encoder.copy_buffer_to_buffer(&buffers[1], 0, &read, 96, buffers[1].size());
    renderer.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap()
        });
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    rx.recv().unwrap().unwrap();
    let bytes = read.slice(..).get_mapped_range();
    let corners = f32::from_ne_bytes(bytes[60..64].try_into().unwrap());
    assert!(
        (3.0..=4.0).contains(&corners),
        "actual near-clipped polygon: {corners}"
    );
    assert_eq!(
        u32::from_ne_bytes(bytes[96..100].try_into().unwrap()),
        1
    );
    assert!(
        u64::from(u32::from_ne_bytes(
            bytes[100..104].try_into().unwrap()
        )) <= buffers[1].size() / 4
    );
    let words = |offset: usize| {
        u32::from_ne_bytes(bytes[offset..offset + 4].try_into().unwrap())
    };
    let mut occupied = 0;
    for tile in 1..=4800 {
        let at = 96 + tile * 16;
        let count = words(at);
        if count == 0 {
            continue;
        }
        occupied += 1;
        let start = words(at + 4) as usize;
        assert_eq!(words(at + 8), count);
        assert_eq!(words(at + 12), 0);
        assert!((start as u64 + u64::from(count) * 2) * 4 <= buffers[1].size());
        for i in 0..count as usize {
            let primitive = words(96 + (start + i * 2) * 4);
            assert!(primitive > 0 && u64::from(primitive) <= renderer.tile_usage()[3]);
            let depth = f32::from_ne_bytes(
                bytes[96 + (start + i * 2 + 1) * 4..96 + (start + i * 2 + 2) * 4]
                    .try_into()
                    .unwrap(),
            );
            assert!(depth.is_finite());
        }
    }
    assert!(
        occupied > 0,
        "actual rasterized tile lists contain triangles"
    );
    drop(bytes);
    read.unmap();
    let first = renderer.tile_usage();
    assert!(first[3] > 0 && first[0] >= first[3] * 96);
    assert_eq!(first[5..], [1, 1]);
    renderer.draw(&view, &Default::default(), &matrix);
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert_eq!(renderer.tile_usage()[5..], [1, 1]);
    let mut camera = camera;
    camera.orbit(0.1, 0.05);
    renderer.draw(&view, &Default::default(), &camera.uniform());
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    assert_eq!(renderer.tile_usage()[5..], [2, 2]);
    drop(buffers);
    renderer.set_scene(&Scene::empty(), None);
    renderer.draw(&view, &Default::default(), &camera.uniform());
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
    let empty = renderer.tile_usage();
    assert_eq!(empty[..5], [96, 16, 48, 0, 0]);
}
```

### 4. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod finite_ink;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
#[cfg(test)]
mod tile_shader_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod finite_ink;

#[cfg(test)]
mod tile_shader_tests;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
#[cfg(test)]
mod tile_gpu_tests;

#[cfg(target_arch = "wasm32")]
```

### 5. `src/finite_visibility_tests.rs`

Create the file and type:

```rust
use crate::{
    renderer::Renderer,
    scene::{ObjectId, Scene},
    viewport::Viewport,
};
use std::rc::Rc;
fn scene(depth: f64, kind: u8, sloped: bool) -> (Scene, ObjectId, ObjectId) {
    let mut scene = Scene::empty();
    let points = if sloped {
        [
            [-0.8, -0.8, 0.1],
            [0.8, -0.8, 0.1],
            [0., 0.8, 0.9],
        ]
    } else {
        [
            [-0.8, -0.8, 0.5],
            [0.8, -0.8, 0.5],
            [0., 0.8, 0.5],
        ]
    };
    let solid = scene
        .insert(
            crate::prepared::PreparedMesh::triangle(points, [0.1, 0.5, 0.8]).unwrap(),
        )
        .unwrap();
    let color = session_rust::Color::new(1., 0., 0., 1.);
    let ink = if kind == 2 {
        let mut p = session_rust::Point::new(0., 0., depth);
        p.width = 10.;
        p.pointcolor = color;
        scene
            .insert_point(crate::marker::PreparedPoint::new(Rc::new(p)).unwrap())
            .unwrap()
    } else {
        let mut line = session_rust::Line::new(-0.25, 0., depth, 0.25, 0., depth);
        line.width = 4.;
        line.linecolor = color;
        if kind == 1 {
            line.arrowhead = session_rust::Arrowhead::END;
            scene
                .insert_path(crate::chain::PreparedChain::line(Rc::new(line)).unwrap())
                .unwrap()
        } else {
            scene
                .insert_line(crate::stroke::PreparedLine::new(Rc::new(line)).unwrap())
                .unwrap()
        }
    };
    (scene, solid, ink)
}
fn renderer(scene: &Scene, size: [u32; 2], density: f32) -> Renderer {
    renderer_limit(scene, size, density, None)
}
fn renderer_limit(
    scene: &Scene,
    size: [u32; 2],
    density: f32,
    storage: Option<u64>,
) -> Renderer {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let adapter =
        pollster::block_on(instance.request_adapter(&Default::default())).unwrap();
    let mut descriptor = wgpu::DeviceDescriptor::default();
    if let Some(bytes) = storage {
        descriptor
            .required_limits
            .max_storage_buffer_binding_size = bytes;
    }
    let (device, queue) =
        pollster::block_on(adapter.request_device(&descriptor)).unwrap();
    device.on_uncaptured_error(std::sync::Arc::new(|error| {
        panic!("actual finite visibility: {error}")
    }));
    let mut renderer = Renderer::new(
        device,
        queue,
        wgpu::TextureFormat::Rgba8Unorm,
        scene,
    );
    renderer.resize(Viewport {
        width: size[0],
        height: size[1],
    });
    renderer.set_density(density);
    renderer
}
fn wait(renderer: &Renderer) {
    renderer
        .device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        })
        .unwrap();
}
fn red_pixels(renderer: &Renderer, size: [u32; 2]) -> usize {
    red_frame(
        renderer,
        size,
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    )
}
fn red_frame(renderer: &Renderer, size: [u32; 2], matrix: [f32; 16]) -> usize {
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: Some("finite visibility pixels"),
            size: wgpu::Extent3d {
                width: size[0],
                height: size[1],
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
    renderer.draw(
        &texture.create_view(&Default::default()),
        &Default::default(),
        &matrix,
    );
    let pitch = (size[0] * 4).div_ceil(256) * 256;
    let read = renderer
        .device
        .create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: u64::from(pitch) * u64::from(size[1]),
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    let mut encoder = renderer
        .device
        .create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        texture.as_image_copy(),
        wgpu::TexelCopyBufferInfo {
            buffer: &read,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(pitch),
                rows_per_image: Some(size[1]),
            },
        },
        texture.size(),
    );
    renderer.queue.submit([encoder.finish()]);
    let (tx, rx) = std::sync::mpsc::channel();
    read.slice(..)
        .map_async(wgpu::MapMode::Read, move |r| {
            tx.send(r).unwrap()
        });
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
        renderer,
        at,
        session_rust::Xform::identity()
            .m
            .map(|v| v as f32),
    )
}
fn pick_frame(
    renderer: &mut Renderer,
    at: [f32; 2],
    matrix: [f32; 16],
) -> Option<[u32; 2]> {
    renderer.request_pick(at, matrix).unwrap();
    wait(renderer);
    renderer.take_pick(matrix).unwrap().unwrap()
}
#[test]
fn original_lines_headed_chains_and_points_share_actual_color_and_integer_occlusion() {
    for density in [1u32, 2] {
        let size = [320 * density, 240 * density];
        for kind in 0..3 {
            let (hidden, solid, _) = scene(0.7, kind, false);
            let mut renderer = renderer(&hidden, size, density as f32);
            assert_eq!(
                picked(&mut renderer, [0., 0.]).unwrap()[0],
                solid.value(),
                "a hidden original cannot win a pick before the first color frame"
            );
            assert_eq!(
                red_pixels(&renderer, size),
                0,
                "hidden family{kind}, density{density}"
            );
            let (front, _, ink) = scene(0.3, kind, false);
            renderer.set_scene(&front, None);
            assert!(
                red_pixels(&renderer, size) > 30,
                "actual visible family{kind}, density{density}"
            );
            assert_eq!(
                picked(&mut renderer, [0., 0.]).unwrap()[0],
                ink.value()
            );
        }
    }
}
#[test]
fn a_line_on_a_sloping_original_face_keeps_its_full_css_width() {
    for density in [1u32, 2] {
        let size = [320 * density, 240 * density];
        let (front, _, _) = scene(0.3, 0, false);
        let mut renderer = renderer(&front, size, density as f32);
        let flat = red_pixels(&renderer, size);
        let (sloped, _, ink) = scene(0.5, 0, true);
        renderer.set_scene(&sloped, None);
        let grazing = red_pixels(&renderer, size);
        assert_eq!(
            grazing, flat,
            "finite center-line visibility preserves both sides of the four-CSS-pixel ribbon"
        );
        assert_eq!(
            picked(&mut renderer, [0., 0.]).unwrap()[0],
            ink.value()
        );
    }
}

#[test]
fn shifted_view_frames_keep_original_color_and_crop_ownership() {
    let size = [320, 240];
    let mut matrix = session_rust::Xform::identity()
        .m
        .map(|v| v as f32);
    matrix[12] = 0.35;
    matrix[13] = -0.25;
    for kind in 0..3 {
        let (hidden, solid, _) = scene(0.7, kind, false);
        let mut renderer = renderer(&hidden, size, 1.);
        assert_eq!(red_frame(&renderer, size, matrix), 0);
        assert_eq!(
            pick_frame(&mut renderer, [0.35, -0.25], matrix).unwrap()[0],
            solid.value()
        );
        let (front, _, ink) = scene(0.3, kind, false);
        renderer.set_scene(&front, None);
        assert!(red_frame(&renderer, size, matrix) > 30);
        assert_eq!(
            pick_frame(&mut renderer, [0.35, -0.25], matrix).unwrap()[0],
            ink.value()
        );
    }
}
#[test]
fn a_device_limit_retains_physical_color_and_pick_occlusion() {
    let size = [320, 240];
    let (hidden, solid, _) = scene(0.7, 0, false);
    let mut renderer = renderer_limit(&hidden, size, 1., Some(64 * 1024));
    assert_eq!(red_pixels(&renderer, size), 0);
    assert_eq!(renderer.tile_usage()[..2], [96, 16]);
    assert_eq!(
        picked(&mut renderer, [0., 0.]).unwrap()[0],
        solid.value()
    );
    let (front, _, ink) = scene(0.3, 0, false);
    renderer.set_scene(&front, None);
    assert!(red_pixels(&renderer, size) > 30);
    assert_eq!(
        picked(&mut renderer, [0., 0.]).unwrap()[0],
        ink.value()
    );
}
#[test]
fn an_unchanged_dense_view_consumes_actual_overflow_and_stops_rebuilding() {
    let mut scene = Scene::empty();
    for _ in 0..8 {
        scene
            .insert(
                crate::prepared::PreparedMesh::triangle(
                    [
                        [-10., -10., 0.5],
                        [10., -10., 0.5],
                        [0., 10., 0.5],
                    ],
                    [0.1, 0.5, 0.8],
                )
                .unwrap(),
            )
            .unwrap();
    }
    let size = [320, 240];
    let renderer = renderer(&scene, size, 1.);
    let mut matrix = session_rust::Xform::identity()
        .m
        .map(|v| v as f32);
    let texture = renderer
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: 320,
                height: 240,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
    let view = texture.create_view(&Default::default());
    renderer.draw(&view, &Default::default(), &matrix);
    let initial = renderer.tile_usage()[4];
    matrix[12] = 0.01;
    renderer.draw(&view, &Default::default(), &matrix);
    wait(&renderer);
    for _ in 0..6 {
        renderer.draw(&view, &Default::default(), &matrix);
        wait(&renderer);
    }
    let settled = renderer.tile_usage();
    assert!(
        settled[4] > initial,
        "actual overlapping triangles require a larger shared pool"
    );
    assert!(settled[6] <= 6, "actual growth converges");
    for _ in 0..3 {
        renderer.draw(&view, &Default::default(), &matrix);
        wait(&renderer);
    }
    assert_eq!(
        renderer.tile_usage(),
        settled,
        "a resting view has no redraw or report-copy loop"
    );
}

#[test]
fn coplanar_construction_grid_is_hidden_without_hiding_original_ink() {
    let size = [320, 240];
    let (scene, _, _) = scene(0.3, 0, false);
    let mut renderer = renderer(&scene, size, 1.);
    let baseline = red_pixels(&renderer, size);
    let plane = crate::cplane::CPlane::Frame {
        origin: [0., 0., 0.5],
        x: [1., 0., 0.],
        y: [0., 1., 0.],
    };
    renderer
        .set_grid_on(
            Some(crate::grid::Settings {
                spacing: 0.1,
                half_cells: 1,
                ..Default::default()
            }),
            plane,
        )
        .unwrap();
    assert_eq!(
        red_pixels(&renderer, size),
        baseline,
        "coplanar red construction grid cannot paint over the solid"
    );
    let first = renderer.tile_usage();
    renderer.resize(crate::viewport::Viewport {
        width: 319,
        height: 239,
    });
    let _ = red_pixels(&renderer, [319, 239]);
    assert_eq!(
        renderer.tile_usage()[5],
        first[5] + 1,
        "physical viewport change invalidates projection even with the same tile dimensions"
    );
}
```

### 6. `src/lib.rs`

**Locate this existing block:**

```rust
mod tile_shader_tests;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
pub mod source_mesh_topology;
#[cfg(test)]
mod tile_gpu_tests;

#[cfg(target_arch = "wasm32")]
```

Edit the existing block to match this code; keep unchanged lines:

```rust
mod tile_shader_tests;

#[cfg(test)]
mod tile_gpu_tests;

pub mod finite_project_source;
#[cfg(test)]
mod finite_scan_contract;
#[cfg(test)]
mod finite_tiles_contract;
#[cfg(test)]
mod finite_visibility_tests;
pub mod source_mesh_topology;

#[cfg(target_arch = "wasm32")]
```

### 7. `src/pick_gpu_tests.rs`

**Locate this existing block:**

```rust
        assert_eq!(
            finish_at(&mut renderer).unwrap().part,
            Part::BRepEdge(border.edge)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
        assert_eq!(
            finish_at(&mut renderer)
                .unwrap_or_else(|| panic!(
                    "Missing original CAD edge {} at {:?}",
                    border.edge, p
                ))
                .part,
            Part::BRepEdge(border.edge)
```

## Build & Run

![Recorded visual output for Lesson 17.2 (86aq-exercise-finite-color-and-original-picking)](../../screenshots/journey/86aq-exercise-finite-color-and-original-picking.png)

```sh
npm --prefix ../session_tests run course -- dependencies 17.2
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 17.2
npm --prefix ../session_tests run course -- save 17.2
```

Continue with [17.3 · Original clipping half-spaces across drawing families](87-copy-current-original-plane-modes.md).
