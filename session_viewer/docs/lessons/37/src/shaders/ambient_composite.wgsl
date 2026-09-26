// --8<-- [start:ambient-composite]
// The same uniform as ssao.wgsl; only params.zw, the canvas size, is read here.
struct Ambient {
    inverse_mvp: mat4x4<f32>,
    mvp: mat4x4<f32>,
    params: vec4<f32>,
    extent: vec4<f32>,
    near: vec4<f32>,
    near_x: vec4<f32>,
    near_y: vec4<f32>,
    ray: vec4<f32>,
    ray_x: vec4<f32>,
    ray_y: vec4<f32>,
    previous_mvp: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> ambient: Ambient;
@group(0) @binding(1) var<storage,read> corrections: array<u32>;
@group(0) @binding(2) var<storage,read> tiles: array<u32>;
@group(1) @binding(0) var occlusion: texture_2d<f32>;
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = array<vec2<f32>,3>(vec2<f32>(-1.0,-1.0),vec2<f32>(3.0,-1.0),vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
// Black at alpha = occlusion: the blend state darkens whatever is already drawn.
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0,0.0,0.0,textureLoad(occlusion,vec2<i32>(pixel.xy),0).x);
}
struct Vertex {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(flat) sample: u32,
};
// One 16 x 16 square per flagged tile and sample: instance / 4 picks the tile, instance % 4 the sample.
@vertex fn vs_edges(@builtin(vertex_index) i: u32, @builtin(instance_index) instance: u32) -> Vertex {
    let tile = tiles[instance/4u];
    let width = (u32(ambient.params.z)+15u)/16u;
    let origin = vec2<f32>(f32(tile%width),f32(tile/width))*16.0;
    let corners = array<vec2<f32>,6>(vec2<f32>(0.0,0.0),vec2<f32>(1.0,0.0),vec2<f32>(0.0,1.0),vec2<f32>(0.0,1.0),vec2<f32>(1.0,0.0),vec2<f32>(1.0,1.0));
    let xy = (origin+corners[i]*16.0)/ambient.params.zw;
    return Vertex(vec4<f32>(xy.x*2.0-1.0,1.0-xy.y*2.0,0.0,1.0),instance%4u);
}
struct Shade {
    @location(0) color: vec4<f32>,
    @builtin(sample_mask) mask: u32, // write this MSAA sample only
};
// Darken one sample by its stored difference; a sample without one is discarded.
@fragment fn fs_edges(pixel: Vertex) -> Shade {
    let xy = vec2<u32>(pixel.position.xy);
    let packed = corrections[xy.y*u32(ambient.params.z)+xy.x];
    let alpha = f32((packed>>(pixel.sample*8u))&255u)/255.0;
    if alpha==0.0 { discard; }
    return Shade(vec4<f32>(0.0,0.0,0.0,alpha),1u<<pixel.sample);
}
// --8<-- [end:ambient-composite]
