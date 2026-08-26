// Composite the splate buffers into frame: one fullscreen triangle.
// Each fragment looks up its pixel
// no splat (depth bits = 0 = reverse-Z far) discards, a splat emits the colour and 
// exports the splat's depth via frag_depth - so points and solids depth-test each other
// exactly, and late passes (markers, flat ink) see real cloud depth.
// frag_depths costs early-Z only this one triangle, ~2M cheap fragments.
// splat_resolve.wgsl is a RENDER shader (vs + fs). 
// Only a render pipeline can write the swapchain texture and the real depth buffer. 
// So one fullscreen triangle, drawn inside the render pass with the solids, looks up each pixel in those two storage buffers, 
// discards empties, emits the colour, and exports the splat's depth via frag_depth
//which is what lets splats and meshes occlude each other exactly.

struct CloudUniform{
    size: f32,
    vp_w: f32,
    vp_h: f32,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> cloud: CloudUniform;

@group(1) @binding(0) var<storage, read> sdepth: array<u32>;
@group(1) @binding(1) var<storage, read> scolor: array<u32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    var o: VsOut;
    let x = f32(i32(vid & 1u) * 4 - 1);
    let y = f32(i32(vid >> 1u) * 4 - 1);
    o.pos = vec4<f32>(x, y, 0.0, 1.0); // (-1, 1) (3, -1) (-1, 3): one triangle covers the screen
    return o;
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

@fragment
fn fs_main(in: VsOut) -> FsOut{
    let idx = u32(in.pos.y) * u32(cloud.vp_w) + u32(in.pos.x);
    let d = sdepth[idx];
    if (d == 0u) {
        discard; // no splat landed here
    }
    var o: FsOut;
    o.color = vec4<f32>(unpack4x8unorm(scolor[idx]).rgb, 1.0);
    o.depth = bitcast<f32>(d);
    return o;
}