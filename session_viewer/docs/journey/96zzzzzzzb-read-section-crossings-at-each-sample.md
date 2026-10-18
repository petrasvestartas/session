# 22.5.8 · Read section crossings at each sample

**Chapter 22 · Surface lighting, contact and display quality · Section 22.5 · Frame resources and multisampling** · [Contents](contents.md)

**Typing: 29–57 minutes.** [Estimate](typing-load.md).

Read a section cap from its matching physical sample. Give crossing images and pipeline descriptors an explicit count, keep one direct fragment calculation for both counts, and retain original cap geometry when the active image changes.

## Type

Continue from [Retarget existing overlay lanes](96zzzzzzza-retarget-existing-overlay-lanes.md). [Save or recover your work](recovery.md).

### 1. `src/cap_pipeline.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

<details>
<summary>Locate the existing block</summary>

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:Default::default(),multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub record_count:wgpu::RenderPipeline,pub geometry:wgpu::BindGroupLayout,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device,format:wgpu::TextureFormat)->Self{
  let view=crate::surface_view::layout(device);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},false)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED};let count=pipeline(device,crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0),&[&view,&model,&clip],"vs_count","fs_count",&[target.clone()],true);
  let geometry=crate::cap_geometry::layout(device);let source=crate::object_rows::source(include_str!("cap_count.wgsl"));let record_count=pipeline_buffers(device,crate::clipping_gpu::source(&source,2,0),&[&view,&crate::object_rows::layout(device),&clip,&geometry],"vs_record_count","fs_count",&[target],&[crate::draw_slots::layout()],true);
  let color=pipeline(device,crate::clipping_gpu::source(include_str!("cap_color.wgsl"),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);Self{count,record_count,geometry,color,counts,frame}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub(crate) fn pipeline(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],count:bool,samples:u32)->wgpu::RenderPipeline{
 let attributes=wgpu::vertex_attr_array![0=>Float32x3];let buffers=if count{vec![wgpu::VertexBufferLayout{array_stride:20,step_mode:wgpu::VertexStepMode::Vertex,attributes:&attributes}]}else{Vec::new()};
 pipeline_buffers(device,source,groups,vertex,fragment,targets,&buffers,count,samples)
}
pub(crate) fn pipeline_buffers(device:&wgpu::Device,source:String,groups:&[&wgpu::BindGroupLayout],vertex:&str,fragment:&str,targets:&[wgpu::ColorTargetState],buffers:&[wgpu::VertexBufferLayout<'_>],count:bool,samples:u32)->wgpu::RenderPipeline{
 let shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{label:Some("original closed-solid sections"),source:wgpu::ShaderSource::Wgsl(source.into())});let layout=device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{label:Some("original closed-solid sections"),bind_group_layouts:&groups.iter().map(|g|Some(*g)).collect::<Vec<_>>(),immediate_size:0});
 device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{label:Some("original closed-solid sections"),layout:Some(&layout),vertex:wgpu::VertexState{module:&shader,entry_point:Some(vertex),compilation_options:Default::default(),buffers},fragment:Some(wgpu::FragmentState{module:&shader,entry_point:Some(fragment),compilation_options:Default::default(),targets:&targets.iter().cloned().map(Some).collect::<Vec<_>>()}),primitive:Default::default(),depth_stencil:if count{None}else{Some(crate::depth::surface())},multisample:wgpu::MultisampleState{count:samples,..Default::default()},multiview_mask:None,cache:None})
}
pub struct Pipelines{pub count:wgpu::RenderPipeline,pub record_count:wgpu::RenderPipeline,pub geometry:wgpu::BindGroupLayout,pub color:wgpu::RenderPipeline,pub counts:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn with_samples(device:&wgpu::Device,format:wgpu::TextureFormat,samples:u32)->Self{assert!(matches!(samples,1|4));
  let view=crate::surface_view::layout(device);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);let clip=crate::clipping_gpu::layout(device);let frame=crate::gpu_layout::uniform(device,16);let counts=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("section crossing texture"),entries:&[crate::text_gpu_support::texture_entry(0,wgpu::ShaderStages::FRAGMENT,wgpu::TextureSampleType::Float{filterable:false},samples==4)]});
  let add=wgpu::BlendComponent{src_factor:wgpu::BlendFactor::One,dst_factor:wgpu::BlendFactor::One,operation:wgpu::BlendOperation::Add};let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R16Float,blend:Some(wgpu::BlendState{color:add,alpha:add}),write_mask:wgpu::ColorWrites::RED};let count=pipeline(device,count_source(include_str!("cap_count.wgsl"),samples),&[&view,&model,&clip],"vs_count","fs_count",&[target.clone()],true,samples);
  let geometry=crate::cap_geometry::layout(device);let source=crate::object_rows::source(include_str!("cap_count.wgsl"));let record_count=pipeline_buffers(device,count_source(&source,samples),&[&view,&crate::object_rows::layout(device),&clip,&geometry],"vs_record_count","fs_count",&[target],&[crate::draw_slots::layout()],true,samples);
  let color=pipeline(device,crate::clipping_gpu::source(&colour_source(samples),1,0),&[&frame,&clip,&counts],"vs_cap","fs_cap",&[wgpu::ColorTargetState{format,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false,samples);Self{count,record_count,geometry,color,counts,frame}
 }
}

fn count_source(source:&str,samples:u32)->String {
 let source=crate::clipping_gpu::source(source,2,0);
 if samples==1 {source}else{source.replace("@location(0) world_pos:","@location(0) @interpolate(perspective,sample) world_pos:")}
}
fn colour_source(samples:u32)->String {
 let source=include_str!("cap_color.wgsl");if samples==1 {return source.into();}
 source.replace("var counts:texture_2d<f32>;","var counts:texture_multisampled_2d<f32>;")
 .replace("const SAMPLES:u32=1u;","const SAMPLES:u32=4u;")
 .replace("textureLoad(counts,at,0)","textureLoad(counts,at,i32(sample))")
 .replace("@location(0) @interpolate(flat) plane: u32,","@location(0) @interpolate(flat) plane: u32,\n    @location(1) @interpolate(linear,sample) pixel:vec2f,")
 .replace("out.plane = plane;","out.plane = plane;out.pixel=(out.pos.xy/out.pos.w*vec2f(0.5,-0.5)+0.5)*cap_frame.size;")
 .replace("in.pos.xy","in.pixel")
 .replace("let samples = clip_samples(t, gradient);","let samples = select(0u,0xffffffffu,t>=0.);")

}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-01.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-02.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-03.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-04.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-05.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-06.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-07.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-08.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-09.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-10.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-11.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-12.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-13.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-14.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-15.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-16.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-17.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-18.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-19.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-20.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-21.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-22.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-23.rs"
```

</details>

### 2. `src/cap_color.wgsl`

Use the actual physical sample in the same cap fragment calculation at both counts. Read one matching crossing and selection word directly, preserving the single-sample calculation without duplicated shader source.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```wgsl
    }

    let at = vec2<i32>(in.pos.xy);
    var inside = 0u;
    var selected = false;

    // inside a solid where more crossings leave than enter behind the plane
    for (var k = 0u; k < SAMPLES; k++) {
        let count = count_at(at, k);
        inside |= select(0u, 1u << k, count > 0.5);
        selected = selected || count > SELECTED_CROSSING - 0.5;
    }

    let mask = inside & kept;

    if (mask == 0u || !(z >= 0.0 && z <= 1.0)) {
```

</details>

Replace it with:

```wgsl
    }

    let at = vec2<i32>(in.pos.xy);
    let count=count_at(at,sample);
    let inside=select(0u,1u<<sample,count>0.5);
    let selected=count>SELECTED_CROSSING-0.5;

    let mask = inside & kept;

    if (mask == 0u || !(z >= 0.0 && z <= 1.0)) {
```

<details>
<summary>Locate this block</summary>

```wgsl

@fragment
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in: CapVertex) -> CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
```

</details>

Replace it with:

```wgsl

@fragment
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in:CapVertex,@builtin(sample_index) sample:u32)->CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
```

<details>
<summary>Complete updated block reference</summary>

```wgsl
struct CapFrame{size:vec2<f32>,origin:vec2<f32>}
@group(0) @binding(0) var<uniform> cap_frame:CapFrame;
@group(2) @binding(0) var counts:texture_2d<f32>;
const SAMPLES:u32=1u;const SELECT_COLOR:vec3<f32>=vec3<f32>(1.0,1.0,0.0);
fn count_at(at:vec2<i32>,sample:u32)->f32{return textureLoad(counts,at,0).x;}
// Prefixed in Rust: `counts` and `primitives` at group 3, SAMPLES, `count_at` and `primitive_at`.

// One corner of a fullscreen triangle; the instance picks the plane or the caps.
struct CapVertex {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) @interpolate(flat) plane: u32, // clipping plane; for the masks 1 = selected caps only
}

@vertex
// A triangle over the whole target.
fn vs_cap(@builtin(vertex_index) vertex: u32, @builtin(instance_index) plane: u32) -> CapVertex {
    let xy = vec2<f32>(f32((vertex << 1u) & 2u), f32(vertex & 2u));
    var out: CapVertex;
    out.pos = vec4<f32>(xy * 2.0 - 1.0, 0.0, 1.0);
    out.plane = plane;
    return out;
}

// A section cap fragment: color, triangle id, the plane's depth and the samples it covers.
struct CapOut {
    @location(0) color: vec4<f32>, // hatch or light grey, with a black boundary
    @location(1) primitive: vec2<u32>,
    @builtin(frag_depth) depth: f32, // depth of the plane here
    @builtin(sample_mask) mask: u32, // samples inside a solid and kept by the other planes
}

// Fraction of a pixel square lying below signed distance `t` from a line; the ribbon filter.
fn box_cdf(t: f32, hi: f32, lo: f32, m: f32, q: f32) -> f32 {
    let s = clamp(t, -hi, hi);
    let e = hi - abs(s);
    let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
    return select(tail, 0.5 + s / m, abs(s) <= lo);
}

// Exact area of this pixel within `hw` of a line `d` away, across direction `g`.
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
    let a = abs(g.x);
    let b = abs(g.y);
    let hi = 0.5 * (a + b);
    let lo = 0.5 * abs(a - b);
    let m = max(max(a, b), 1e-6);
    let q = max(2.0 * a * b, 1e-6);
    return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}


// Coverage of the nearest hatch line; `slope` is the change of `u` per pixel.
fn hatch_ink(u: f32, slope: vec2<f32>) -> f32 {
    let rate = max(length(slope), 1e-30); // scene units per pixel across the lines
    let level = log2(clipping.spacing * rate);
    // lines every power of two in scene units; every other one fades out over the last half octave
    let step = exp2(floor(level));
    let fade = smoothstep(0.5, 1.0, fract(level));
    let n = u / step;
    let nearest = round(n);
    let distance = abs(n - nearest) * step / rate;
    let odd = abs(nearest - 2.0 * round(nearest * 0.5)) > 0.5;
    let ink = clamp(band_area(distance, clipping.width * 0.5, slope / rate), 0.0, 1.0);
    return ink * select(1.0, 1.0 - fade, odd);
}

// Coverage of the section's union, including holes, without a border at the viewport edge.
fn section_coverage(at: vec2<i32>) -> f32 {
    if (any(at < vec2<i32>(0)) || any(at >= vec2<i32>(textureDimensions(counts)))) {
        return 1.0;
    }
    var covered = 0.0;
    for (var k = 0u; k < SAMPLES; k++) {
        covered += select(0.0, 1.0, count_at(at, k) > 0.5);
    }
    return covered / f32(SAMPLES);
}

fn section_outline(at: vec2<f32>) -> f32 {
    let directions = array<vec2<f32>, 8>(
        vec2<f32>(1.0, 0.0), vec2<f32>(-1.0, 0.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(0.0, -1.0),
        vec2<f32>(0.7071, 0.7071), vec2<f32>(-0.7071, 0.7071),
        vec2<f32>(0.7071, -0.7071), vec2<f32>(-0.7071, -0.7071)
    );
    var edge = 0.0;
    for (var k = 0u; k < 8u; k++) {
        edge = max(edge, 1.0 - section_coverage(vec2<i32>(floor(at + directions[k] * clipping.outline))));
    }
    return edge;
}


@fragment
// Where the view ray meets a plane inside a closed solid: outlined hatch or solid grey.
fn fs_cap(in:CapVertex,@builtin(sample_index) sample:u32)->CapOut {
    let i = in.plane;
    let ndc = clip_ndc(in.pos.xy + cap_frame.origin, cap_frame.size, 0.0).xy;
    let z = clip_plane_depth(i, ndc);
    let h = vec3<f32>(ndc, 1.0);
    // scene units across the hatch lines, and their change per pixel
    let u = dot(clipping.hatch[i].xyz, h) / dot(clipping.hatch_w[i].xyz, h);
    let slope = vec2<f32>(dpdx(u), dpdy(u));
    var kept = 0xffffffffu;
    var edge = 0.0;

    // the other planes cut the cap sample by sample
    for (var j = 0u; j < clipping.count; j++) {
        let t = dot(clipping.screen[j], vec4<f32>(ndc, z, 1.0));
        let gradient = vec2<f32>(dpdx(t), dpdy(t));
        let samples = clip_samples(t, gradient);
        kept &= select(samples, 0xffffffffu, j == i);
        if (j != i && dot(gradient, gradient) > 1e-30) {
            let distance = t / max(length(gradient), 1e-20);
            edge = max(edge, 1.0 - smoothstep(clipping.outline - 0.5, clipping.outline + 0.5, distance));
        }
    }

    let at = vec2<i32>(in.pos.xy);
    let count=count_at(at,sample);
    let inside=select(0u,1u<<sample,count>0.5);
    let selected=count>SELECTED_CROSSING-0.5;

    let mask = inside & kept;

    if (mask == 0u || !(z >= 0.0 && z <= 1.0)) {
        discard;
    }

    let paper = select(vec3<f32>(1.0), SELECT_COLOR, selected);
    let hatched = mix(paper, vec3<f32>(0.0), hatch_ink(u, slope));
    let solid = select(vec3<f32>(0.6), SELECT_COLOR, selected);
    edge = max(edge, section_outline(in.pos.xy));
    let color = mix(select(hatched, solid, clipping.fill == 1u), vec3<f32>(0.0), edge);
    let marker = CAP_PRIMITIVE + i * 2u + select(0u, 1u, selected);
    return CapOut(vec4<f32>(color, 1.0), physical_primitive(marker), z, mask);
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-24.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-25.rs"
```

</details>

### 3. `src/cap_pick_pipeline.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Pipelines{pub record_count:wgpu::RenderPipeline,pub record_nearest:wgpu::RenderPipeline,pub record_geometry:wgpu::BindGroupLayout,pub count:wgpu::RenderPipeline,pub nearest:wgpu::RenderPipeline,pub ids:wgpu::RenderPipeline,pub records:wgpu::BindGroupLayout,pub owners:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as S,BufferBindingType as B};
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::surface_view::layout(device);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
  let records=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("bounded cap crossings and original crop"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform)]});
  let record_geometry=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("shared cap pick crossings"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform),crate::tile_pipelines::entry(3,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(4,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(5,S::VERTEX,B::Storage{read_only:true})]});
  let owners=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("cap crossings and original owner table"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::FRAGMENT,B::Storage{read_only:true})]});
  let source=crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0);let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()};let count=crate::cap_pipeline::pipeline(device,source.clone(),&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_count",&[target.clone()],true);let nearest=crate::cap_pipeline::pipeline(device,source,&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_owner",&[target.clone()],true);
  let shared=crate::object_rows::source(include_str!("cap_count.wgsl"));let source=crate::clipping_gpu::source(&shared,2,0);let layouts=[&view,&crate::object_rows::layout(device),&clip,&record_geometry];let record_count=crate::cap_pipeline::pipeline_buffers(device,source.clone(),&layouts,"vs_record_pick_count","fs_pick_count",&[target.clone()],&[crate::draw_slots::layout()],true);let record_nearest=crate::cap_pipeline::pipeline_buffers(device,source,&layouts,"vs_record_pick_count","fs_pick_owner",&[target],&[crate::draw_slots::layout()],true);
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false);
  Self{record_count,record_nearest,record_geometry,count,nearest,ids,records,owners,frame}
 }
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Pipelines{pub record_count:wgpu::RenderPipeline,pub record_nearest:wgpu::RenderPipeline,pub record_geometry:wgpu::BindGroupLayout,pub count:wgpu::RenderPipeline,pub nearest:wgpu::RenderPipeline,pub ids:wgpu::RenderPipeline,pub records:wgpu::BindGroupLayout,pub owners:wgpu::BindGroupLayout,pub frame:wgpu::BindGroupLayout}
impl Pipelines{
 pub fn new(device:&wgpu::Device)->Self{
  use wgpu::{ShaderStages as S,BufferBindingType as B};
  let frame=crate::gpu_layout::uniform(device,32);let clip=crate::clipping_gpu::layout(device);let view=crate::surface_view::layout(device);let model=crate::gpu_layout::uniform(device,crate::object_record::BYTES as u64);
  let records=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("bounded cap crossings and original crop"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform)]});
  let record_geometry=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("shared cap pick crossings"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(2,S::FRAGMENT,B::Uniform),crate::tile_pipelines::entry(3,S::VERTEX,B::Storage{read_only:true}),crate::tile_pipelines::entry(4,S::VERTEX,B::Uniform),crate::tile_pipelines::entry(5,S::VERTEX,B::Storage{read_only:true})]});
  let owners=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{label:Some("cap crossings and original owner table"),entries:&[crate::tile_pipelines::entry(0,S::FRAGMENT,B::Storage{read_only:false}),crate::tile_pipelines::entry(1,S::FRAGMENT,B::Storage{read_only:true})]});
  let source=crate::clipping_gpu::source(include_str!("cap_count.wgsl"),2,0);let target=wgpu::ColorTargetState{format:wgpu::TextureFormat::R8Unorm,blend:None,write_mask:wgpu::ColorWrites::empty()};let count=crate::cap_pipeline::pipeline(device,source.clone(),&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_count",&[target.clone()],true,1);let nearest=crate::cap_pipeline::pipeline(device,source,&[&view,&model,&clip,&records],"vs_pick_count","fs_pick_owner",&[target.clone()],true,1);
  let shared=crate::object_rows::source(include_str!("cap_count.wgsl"));let source=crate::clipping_gpu::source(&shared,2,0);let layouts=[&view,&crate::object_rows::layout(device),&clip,&record_geometry];let record_count=crate::cap_pipeline::pipeline_buffers(device,source.clone(),&layouts,"vs_record_pick_count","fs_pick_count",&[target.clone()],&[crate::draw_slots::layout()],true,1);let record_nearest=crate::cap_pipeline::pipeline_buffers(device,source,&layouts,"vs_record_pick_count","fs_pick_owner",&[target],&[crate::draw_slots::layout()],true,1);
  let ids=crate::cap_pipeline::pipeline(device,crate::clipping_gpu::source(include_str!("cap_pick.wgsl"),1,0),&[&frame,&clip,&owners],"vs_cap","fs_cap_id",&[wgpu::ColorTargetState{format:wgpu::TextureFormat::Rg32Uint,blend:None,write_mask:wgpu::ColorWrites::ALL},crate::primitive_image::target(true)],false,1);
  Self{record_count,record_nearest,record_geometry,count,nearest,ids,records,owners,frame}
 }
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-26.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-27.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-28.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-29.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-30.rs"
```

</details>

### 4. `src/caps_gpu.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

<details>
<summary>Locate the existing block</summary>

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::cap_pipeline::Pipelines>,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::new(device,format);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(pipes);}
  let pipes=self.pipes.as_ref().unwrap();if self.counts.as_ref().is_none_or(|c|c.image.size!=size){let image=Image::new(device,size,wgpu::TextureFormat::R16Float,2);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry:Default::default()});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode,records:Option<&crate::instance_gpu::Lane>){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(device,encoder,image,window,view,clipping,meshes,&self.targets,self.planes,records);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
use wgpu::util::DeviceExt;
use crate::{gpu_mesh::GpuMesh,solid_orientation::CLOSED};
use crate::cap_image::Image;
struct Counts{image:Image,group:wgpu::BindGroup,geometry:crate::cap_geometry::Geometry}
pub struct Caps{pub enabled:bool,planes:u32,pick:Option<crate::cap_pick_gpu::Pick>,pub pick_error:Option<&'static str>,bounds:crate::cap_source_bounds::Bounds,targets:std::collections::HashMap<u32,u32>,pub solids:crate::solid_cache::Solids,pipes:Option<crate::sample_pipelines::Variants<crate::cap_pipeline::Pipelines>>,samples:u32,counts:Option<Counts>,frame:wgpu::Buffer,frame_group:Option<wgpu::BindGroup>,pub rows:Vec<[u32;2]>,pub draws:u64,pub passes:u64}
impl Caps{
 pub fn new(device:&wgpu::Device)->Self{let frame=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("section canvas frame"),contents:&[0;16],usage:wgpu::BufferUsages::UNIFORM|wgpu::BufferUsages::COPY_DST});Self{enabled:true,planes:0,pick:None,pick_error:None,bounds:Default::default(),targets:Default::default(),solids:Default::default(),pipes:None,samples:1,counts:None,frame,frame_group:None,rows:Vec::new(),draws:0,passes:0}}
 pub fn set_samples(&mut self,samples:u32){assert!(matches!(samples,1|4));self.samples=samples;}
 pub fn set_scene(&mut self,queue:&wgpu::Queue,scene:&crate::scene::Scene,meshes:&mut[GpuMesh],planes:&[crate::clipping_rows::Row]){
  if let Some(counts)=self.counts.as_mut(){counts.geometry.invalidate();}
  self.planes=planes.len()as u32;self.rows.clear();self.targets.clear();for(mesh,object)in meshes.iter_mut().zip(scene.objects().iter().filter(|r|scene.visible(r.id))){assert_eq!(mesh.id,object.id);let flags=self.solids.flags(object);let token=if !planes.is_empty()&&flags&CLOSED!=0{self.rows.push([mesh.id.value(),u32::from(mesh.pickable())]);self.rows.len()as u32}else{0};mesh.set_caps(queue,flags,token);if token>0{if let Some(bounds)=self.bounds.placed(&object.mesh,&object.model){let mask=planes.iter().enumerate().fold(0,|mask,(i,row)|mask|if row.plane.crosses(&bounds,[0.;3]){1<<i}else{0});self.targets.insert(mesh.id.value(),mask);}}}
  self.solids.prune();self.bounds.prune();if self.rows.is_empty(){self.rows.shrink_to_fit();self.targets.shrink_to_fit();}
 }
 pub fn prepare(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat,size:[u32;2],active:bool){
  let size=if active&&!self.rows.is_empty(){size}else{[1,1]};
  if size==[1,1]{self.counts=None;return;}
  if self.pipes.is_none(){let pipes=crate::cap_pipeline::Pipelines::with_samples(device,format,self.samples);self.frame_group=Some(crate::gpu_layout::group(device,&pipes.frame,&self.frame));self.pipes=Some(crate::sample_pipelines::Variants::new(pipes,self.samples));}
  self.pipes.as_mut().unwrap().select(self.samples,||crate::cap_pipeline::Pipelines::with_samples(device,format,self.samples));
  let pipes=&self.pipes.as_ref().unwrap().active;if self.counts.as_ref().is_none_or(|c|c.image.size!=size||c.image.texture.sample_count()!=self.samples){let geometry=self.counts.take().map_or_else(Default::default,|c|c.geometry);let image=Image::with_samples(device,size,wgpu::TextureFormat::R16Float,2,self.samples);let group=device.create_bind_group(&wgpu::BindGroupDescriptor{label:Some("original section crossings"),layout:&pipes.counts,entries:&[wgpu::BindGroupEntry{binding:0,resource:wgpu::BindingResource::TextureView(&image.view)}]});self.counts=Some(Counts{image,group,geometry});}
  self.counts.as_mut().unwrap().geometry.set_planes(device,self.planes).expect("Validated clipping plane count");
  let bytes=[size[0]as f32,size[1]as f32,0.,0.].into_iter().flat_map(f32::to_ne_bytes).collect::<Vec<_>>();queue.write_buffer(&self.frame,0,&bytes);
 }
 pub fn draw(&mut self,device:&wgpu::Device,encoder:&mut wgpu::CommandEncoder,color:&wgpu::TextureView,primitive:&wgpu::TextureView,depth:&wgpu::TextureView,view:&wgpu::BindGroup,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],planes:u32,records:Option<&crate::instance_gpu::Lane>){
  let(Some(pipes),Some(counts))=(&self.pipes,&mut self.counts)else{return};let pipes=&pipes.active;
  for plane in 0..planes{
   let rows=meshes.iter().filter(|m|m.closed()&&self.targets.get(&m.id.value()).is_some_and(|mask|mask&(1<<plane)!=0)).map(|mesh|{let group=records.and_then(|records|records.current_row(mesh)).map(|_|counts.geometry.group(device,&pipes.geometry,&mesh.geometry,plane).expect("Prepared cap plane"));(mesh,group)}).collect::<Vec<_>>();
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original closed-solid crossings"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:&counts.image.view,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],..Default::default()});pass.set_pipeline(&pipes.count);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,clipping,&[]);for(mesh,group)in &rows{if let Some(group)=group.as_ref().filter(|_|records.is_some_and(|records|records.bind_record(&mut pass,mesh))){pass.set_pipeline(&pipes.record_count);pass.set_bind_group(3,group,&[]);pass.draw(0..mesh.geometry.index_count,0..1);}else{pass.set_pipeline(&pipes.count);mesh.bind_model(&mut pass);mesh.geometry.draw_range(&mut pass,plane..plane+1);}self.draws+=1;}}
   {let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor{label:Some("original section color and depth"),color_attachments:&[Some(wgpu::RenderPassColorAttachment{view:color,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment{view:primitive,depth_slice:None,resolve_target:None,ops:wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment{view:depth,depth_ops:Some(wgpu::Operations{load:wgpu::LoadOp::Load,store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});pass.set_pipeline(&pipes.color);pass.set_bind_group(0,self.frame_group.as_ref().unwrap(),&[]);pass.set_bind_group(1,clipping,&[]);pass.set_bind_group(2,&counts.group,&[]);pass.draw(0..3,plane..plane+1);self.draws+=1;}
   self.passes+=2;
  }
 }
 pub fn encode_pick(&mut self,device:&wgpu::Device,queue:&wgpu::Queue,encoder:&mut wgpu::CommandEncoder,image:&crate::pick_image::Image,window:crate::pick_window::Window,view:&wgpu::BindGroup,crop:&wgpu::Buffer,clipping:&wgpu::BindGroup,meshes:&[GpuMesh],mode:crate::pick_sources::Mode,records:Option<&crate::instance_gpu::Lane>){
  if !self.enabled||self.planes==0||self.rows.is_empty(){self.pick=None;self.pick_error=None;return;}
  let pick=self.pick.get_or_insert_with(||crate::cap_pick_gpu::Pick::new(device));if let Err(error)=pick.prepare(device,queue,window,crop,&self.rows,self.planes,mode){self.pick_error=Some(error);self.pick=None;return;}self.pick_error=None;pick.encode(device,encoder,image,window,view,clipping,meshes,&self.targets,self.planes,records);
 }
 pub fn pick_bytes(&self)->u64{self.pick.as_ref().map_or(0,crate::cap_pick_gpu::Pick::bytes)}
 pub fn usage(&self)->[u64;5]{[self.frame.size()+self.pick_bytes()+self.counts.as_ref().map_or(0,|c|c.image.bytes+c.geometry.bytes()),self.rows.len()as u64,self.solids.checks,self.draws,self.passes]}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-31.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-32.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-33.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-34.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-35.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-36.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-37.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-38.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-39.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-40.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-41.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-42.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-43.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-44.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-45.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-46.rs"
```

</details>

### 5. `src/cap_image.rs`

Match active cap image, pipeline and physical sample count while retaining original geometry and independent one-sample queries.

<details>
<summary>Locate the existing block</summary>

```rust
pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub size:[u32;2],pub bytes:u64}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,bytes:u64)->Self{let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("original section data"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});let view=texture.create_view(&Default::default());Self{texture,view,size,bytes:bytes*u64::from(size[0])*u64::from(size[1])}}
}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
pub struct Image{pub texture:wgpu::Texture,pub view:wgpu::TextureView,pub size:[u32;2],pub bytes:u64}
impl Image{
 pub fn new(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,bytes:u64)->Self{Self::with_samples(device,size,format,bytes,1)}
 pub fn with_samples(device:&wgpu::Device,size:[u32;2],format:wgpu::TextureFormat,bytes:u64,samples:u32)->Self{assert!(matches!(samples,1|4));let texture=device.create_texture(&wgpu::TextureDescriptor{label:Some("original section data"),size:wgpu::Extent3d{width:size[0],height:size[1],depth_or_array_layers:1},mip_level_count:1,sample_count:samples,dimension:wgpu::TextureDimension::D2,format,usage:wgpu::TextureUsages::TEXTURE_BINDING|wgpu::TextureUsages::RENDER_ATTACHMENT,view_formats:&[]});let view=texture.create_view(&Default::default());Self{texture,view,size,bytes:bytes*u64::from(size[0])*u64::from(size[1])*u64::from(samples)}}
}
```

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-47.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-48.rs"
```

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-49.rs"
```

</details>

<details>
<summary>Supplied validation fixture</summary>

Copy this supplied authoring fixture once. Continue with the browser build below; the course does not require running native tests.

### 6. `src/sample_surface_tests.rs`

Retain the complete existing ordinary/shared surface fixture and add actual solid, hatch, selected cap, original-owner query, geometry/source preservation and exact return checks at both counts.

<details>
<summary>Locate the existing block</summary>

```text
use super::*;
use std::rc::Rc;
fn surface_frame(r:&Renderer,target:&crate::frame_targets::Targets,shared:bool)->Vec<u8> {
 let output=r.device.create_texture(&wgpu::TextureDescriptor {label:Some("actual multisampled original surfaces"),size:wgpu::Extent3d {width:320,height:240,depth_or_array_layers:1},mip_level_count:1,sample_count:1,dimension:wgpu::TextureDimension::D2,format:r.format,usage:wgpu::TextureUsages::RENDER_ATTACHMENT|wgpu::TextureUsages::COPY_SRC,view_formats:&[]});let view=output.create_view(&Default::default());
 let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));r.queue.write_buffer(&r.uniform,0,&crate::surface_view::bytes(matrix,[0.,1.,0.,0.],[320,240],1.));
 r.clipping.borrow_mut().frame_anchored(&r.queue,matrix,r.anchor(),240,1.);
 let mut encoder=r.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_render_pass(&wgpu::RenderPassDescriptor {label:Some("original and shared surface count witness"),color_attachments:&[Some(wgpu::RenderPassColorAttachment {view:target.color(&view),resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::WHITE),store:wgpu::StoreOp::Store}}),Some(wgpu::RenderPassColorAttachment {view:&target.primitive.view,resolve_target:None,depth_slice:None,ops:wgpu::Operations {load:wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),store:wgpu::StoreOp::Store}})],depth_stencil_attachment:Some(wgpu::RenderPassDepthStencilAttachment {view:&target.depth,depth_ops:Some(wgpu::Operations {load:wgpu::LoadOp::Clear(0.),store:wgpu::StoreOp::Store}),stencil_ops:None}),..Default::default()});
  pass.set_bind_group(0,&r.view_group,&[]);pass.set_bind_group(2,&r.clipping.borrow().group,&[]);
  for mesh in r.meshes.iter().filter(|mesh|!shared||!r.instance_rows.contains(&mesh.id)) {pass.set_pipeline(&r.surfaces.active.0);r.instances.bind_own(&mut pass);mesh.draw_pulled(&mut pass);}
  if shared {r.instances.draw(&mut pass);}
 }
 target.resolve(&mut encoder,&view);
 let buffer=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size:320*240*4,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});encoder.copy_texture_to_buffer(output.as_image_copy(),wgpu::TexelCopyBufferInfo {buffer:&buffer,layout:wgpu::TexelCopyBufferLayout {offset:0,bytes_per_row:Some(1280),rows_per_image:Some(240)}},output.size());r.queue.submit([encoder.finish()]);
 let(tx,rx)=std::sync::mpsc::channel();buffer.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());crate::cap_gpu_tests::wait(r);rx.recv().unwrap().unwrap();let pixels=buffer.slice(..).get_mapped_range().to_vec();buffer.unmap();pixels
}
fn actual_physical_samples(r:&Renderer,target:&crate::frame_targets::Targets)->usize {
 let module=r.device.create_shader_module(wgpu::ShaderModuleDescriptor {label:Some("read every physical original sample"),source:wgpu::ShaderSource::Wgsl(r#"
@group(0) @binding(0) var primitive:texture_multisampled_2d<u32>;
@group(0) @binding(1) var depth:texture_depth_multisampled_2d;
@group(0) @binding(2) var<storage,read_write> output:array<vec4u>;
@compute @workgroup_size(64) fn read_samples(@builtin(global_invocation_id) id:vec3u) {
 if(id.x>=320u*240u*4u){return;}let p=id.x/4u;let at=vec2i(i32(p%320u),i32(p/320u));let sample=id.x%4u;
 output[id.x]=vec4u(textureLoad(primitive,at,i32(sample)).xy,bitcast<u32>(textureLoad(depth,at,i32(sample))),sample);
}"#.into())});
 let pipeline=r.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {label:None,layout:None,module:&module,entry_point:Some("read_samples"),compilation_options:Default::default(),cache:None});
 let size=320*240*4*16;let output=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage:wgpu::BufferUsages::STORAGE|wgpu::BufferUsages::COPY_SRC,mapped_at_creation:false});let read=r.device.create_buffer(&wgpu::BufferDescriptor {label:None,size,usage:wgpu::BufferUsages::MAP_READ|wgpu::BufferUsages::COPY_DST,mapped_at_creation:false});
 let group=r.device.create_bind_group(&wgpu::BindGroupDescriptor {label:None,layout:&pipeline.get_bind_group_layout(0),entries:&[wgpu::BindGroupEntry {binding:0,resource:wgpu::BindingResource::TextureView(&target.primitive.view)},wgpu::BindGroupEntry {binding:1,resource:wgpu::BindingResource::TextureView(&target.depth)},wgpu::BindGroupEntry {binding:2,resource:output.as_entire_binding()}]});let mut encoder=r.device.create_command_encoder(&Default::default());{
  let mut pass=encoder.begin_compute_pass(&Default::default());pass.set_pipeline(&pipeline);pass.set_bind_group(0,&group,&[]);pass.dispatch_workgroups((320*240*4u32).div_ceil(64),1,1);
 }encoder.copy_buffer_to_buffer(&output,0,&read,0,size);r.queue.submit([encoder.finish()]);let(tx,rx)=std::sync::mpsc::channel();read.slice(..).map_async(wgpu::MapMode::Read,move|v|tx.send(v).unwrap());crate::cap_gpu_tests::wait(r);rx.recv().unwrap().unwrap();let bytes=read.slice(..).get_mapped_range();let mut mixed=0;
 for pixel in bytes.chunks_exact(64) {let mut addresses=[0;4];for (i,sample) in pixel.chunks_exact(16).enumerate(){let words=std::array::from_fn::<_,4,_>(|j|u32::from_ne_bytes(sample[j*4..j*4+4].try_into().unwrap()));assert_eq!(words[3],i as u32);let address=words[0]|(words[1]<<16);let depth=f32::from_bits(words[2]);assert_eq!(depth,if address==0 {0.}else{1.});addresses[i]=address;}if addresses.iter().any(|a|*a==0)&&addresses.iter().any(|a|*a!=0){mixed+=1;}}
 drop(bytes);read.unmap();assert!(mixed>100,"Actual original silhouettes have mixed covered and uncovered samples");mixed
}
#[test]
fn original_and_shared_meshes_switch_samples_without_source_or_geometry_rebuilds() {
 let mut scene=Scene::empty();crate::scene_document::import(&mut scene,&crate::instance_specimen::display_bytes(),"",&session_rust::Xform::identity(),0.).unwrap();scene.insert(crate::prepared::PreparedMesh::triangle([[-0.3,-0.85,0.],[0.3,-0.85,0.],[0.,-0.3,0.]],[0.1,0.8,0.3]).unwrap()).unwrap();
 let mut r=crate::cap_gpu_tests::renderer(&scene);let geometry=r.meshes.iter().map(|m|Rc::clone(&m.geometry)).collect::<Vec<_>>();let sources=scene.objects().iter().map(|o|o.geometry().unwrap().to_proto()).collect::<Vec<_>>();let stats=r.stats();let usage=r.instance_usage();
 let one=crate::frame_targets::Targets::new(&r.device,[320,240],r.format,1);let four=crate::frame_targets::Targets::new(&r.device,[320,240],r.format,4);let first=surface_frame(&r,&one,true);assert!(first.chunks_exact(4).filter(|p|p[..3]!=[255;3]).count()>5000);let surface1=r.surfaces.active.0.clone();
 r.surfaces.select(4,||crate::surface_pipelines::pipelines(&r.device,r.format,&crate::surface_view::layout(&r.device),&crate::gpu_layout::uniform(&r.device,crate::object_record::BYTES as u64),4));r.instances.set_samples(&r.device,r.format,4);let surface4=r.surfaces.active.0.clone();assert_ne!(surface1,surface4);let shared=surface_frame(&r,&four,true);assert!(surface_frame(&r,&four,false)==shared,"Original uniforms and shared records have identical4x colour");let mixed=actual_physical_samples(&r,&four);assert!(first!=shared,"Actual colour changes on silhouette coverage");
 for count in [4,1,4] {r.surfaces.select(count,||panic!("A retained surface variant must not compile again"));r.instances.set_samples(&r.device,r.format,count);let pixels=surface_frame(&r,if count==1 {&one}else{&four},true);assert!(pixels==if count==1 {first.clone()}else{shared.clone()});assert_eq!(r.surfaces.active.0,if count==1 {surface1.clone()}else{surface4.clone()});}
 let matrix=crate::cap_gpu_tests::reversed(session_rust::Xform::identity().m.map(|v|v as f32));for (i,x) in [-0.6,0.6].into_iter().enumerate(){r.request_pick([x,0.],matrix).unwrap();crate::cap_gpu_tests::wait(&r);let pick=r.take_pick(matrix).unwrap().unwrap().unwrap();assert_eq!(pick[0],scene.objects()[i].id.value(),"Fresh owner queries remain single-sample after surface4x switching");}
 assert_eq!(r.stats(),stats);assert_eq!(r.instance_usage(),usage);for ((gpu,before),source) in r.meshes.iter().zip(&geometry).zip(&sources){assert!(Rc::ptr_eq(&gpu.geometry,before));let row=scene.objects().iter().find(|o|o.id==gpu.id).unwrap();assert_eq!(&row.geometry().unwrap().to_proto(),source);}
 let root=std::path::Path::new("surface-pixels");crate::sample_test_proof::write(root.join("one.rgba"),first).unwrap();crate::sample_test_proof::write(root.join("four.rgba"),shared).unwrap();crate::sample_test_proof::write(root.join("physical.json"),format!("{{\"actualMixedCoveragePixels\":{mixed}}}\n")).unwrap();
}
```

</details>

Update the existing block with this supplied code; keep unchanged lines:

```text
--8<-- "journey/code/96zzzzzzzb-read-section-crossings-at-each-sample-50.rs"
```

</details>

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 22.5.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.

Section crossing images and cap pipeline recipes support matching one- and four-sample resources. One cap fragment calculation reads its own physical crossing, selection and plane depth; original-owner queries remain independently single-sample. Cap geometry stays retained when images change. Complete browser rendering remains one-sample at this checkpoint.

### Native section check

These actual isolated Native GPU pictures show four-sample section caps from the original closed solid. The grey, hatch and selected states use matching physical crossing images. Complete browser rendering remains at one sample.

**solid grey section — actual four-sample section pass.**

![Actual Native section: solid grey section](../screenshots/journey/review/96zzzzzzzb-read-section-crossings-at-each-sample-cap-0.png)

**hatched section — actual four-sample section pass.**

![Actual Native section: hatched section](../screenshots/journey/review/96zzzzzzzb-read-section-crossings-at-each-sample-cap-1.png)

**selected yellow section — actual four-sample section pass.**

![Actual Native section: selected yellow section](../screenshots/journey/review/96zzzzzzzb-read-section-crossings-at-each-sample-cap-2.png)


**Verified checkpoint in Chrome.**

**Critical browser-scene review accepted.**

Actual single-sample browser contact scene after matching physical-sample section crossings and explicit independent queries. Both densities preserve original owners, complete sources and archives, shared geometry, contact shading and exact resize restoration. Three labelled Native pictures show actual four-sample grey, hatched and selected section caps. Complete browser multisampling and whole-course visual acceptance remain unfinished.

![Actual single-sample browser contact scene after matching physical-sample section crossings and explicit independent queries. Both densities preserve original owners, complete sources and archives, shared geometry, contact shading and exact resize restoration. Three labelled Native pictures show actual four-sample grey, hatched and selected section caps. Complete browser multisampling and whole-course visual acceptance remain unfinished.](../screenshots/journey/96zzzzzzt-keep-contact-ambient-inputs-on-originals-browser.png)

[Verification scope](release.md).

This reference image is shared with [22.4.3 · Keep contact ambient inputs on originals](96zzzzzzt-keep-contact-ambient-inputs-on-originals.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>

The shared image allocator keeps existing one-sample callers and accepts an explicit count for crossing images. Their R16Float values count entering and leaving original closed-solid triangles; they remain separate at each physical sample. Colour resolves later, while crossing values, depth and integer primitive addresses keep their own roles.

Existing pipeline factories receive the count directly. Cap colour and crossing shaders use matching physical sample coordinates; the cap fragment reads that sample once and writes only its corresponding coverage bit. The same calculation naturally uses sample zero in a one-sample pass. All integer-query callers explicitly request one sample.

The cap owner caches pipeline recipes and keeps only the active crossing image. Replacing that image retains the original cap geometry and plane records; repeating a count reuses the current recipe. The supplied actual GPU witness checks solid grey caps, hatching, selected yellow caps, fresh original cap-owner queries, complete source records, retained geometry and exact return pixels.

Original closed solid → physical-sample crossings → matching cap colour and plane depth → original cap owner query.



Why must crossing counts stay separate even though the final colour can resolve?

A physical sample can lie inside a solid while a neighbouring sample lies outside or belongs to another selected solid. Averaging crossings, plane depth or original primitive words would destroy those separate geometric decisions.

Study estimate, including typing and experiments: 1–2 hours.

</details>

<details>
<summary>Optional experiment</summary>

Run the actual solid/hatch/selection witness at both counts and switch4→1→4. Require exact return pixels, unchanged original sources and GPU geometry, then freshly query the original cap owner through its one-sample lane. Compare every current Native image against the source solid and actual clipping plane.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 22.5.8
npm --prefix ../session_tests run course -- save 22.5.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>

Section crossing images, colour/depth descriptors and shaders use an explicit matching physical sample count. One cap fragment calculation reads its own crossing and selection state and writes its own coverage bit; single-sample rendering uses sample zero. R16Float crossing values are never resolved into depth or integer addresses. Existing factories accept the count directly, and all five integer-query calls explicitly request one sample. The cap owner retains original geometry and plane records, caches recipes and replaces only the active crossing image when count or extent changes. The fresh actual Native witness covers grey solid caps, hatching, yellow selection, full source records, retained GPU geometry, fresh original cap-owner queries and exact4→1→4 pixels. Existing ordinary/shared surface and closed/open-solid regressions remain intact. All 997 Native tests, WASM/optimized builds, native frame, canonical Chrome original-contact/resize checks and stored evidence pass. Three actual Native cap pictures and ten current canonical Native/browser pictures were individually inspected. Complete browser rendering remains one-sample until every consumer is connected. Complete four-sample consumer integration, remaining view controls, runtime efficiency, Cargo independence and final whole-course visual validation remain unfinished. Broader facing and NURBS quality review is still required.

Retain the existing ordinary/shared surface GPU assertions and extend its helper to draw actual section caps. Add solid, hatch, selected-cap, original-owner query, full-source, retained-geometry and exact-return checks at both counts. Preserve existing closed/open-solid and clipping regressions.

Actual single-sample browser contact scene after matching physical-sample section crossings and explicit independent queries. Both densities preserve original owners, complete sources and archives, shared geometry, contact shading and exact resize restoration. Three labelled Native pictures show actual four-sample grey, hatched and selected section caps. Complete browser multisampling and whole-course visual acceptance remain unfinished.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 22.5.8
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
