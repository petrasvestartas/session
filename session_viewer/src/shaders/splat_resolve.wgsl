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
    _pad: f32, // EDL strength; 0 = off
};

@group(0) @binding(0) var<uniform> cloud: CloudUniform;

@group(1) @binding(0) var sdepth: texture_depth_2d; // the cloud's own depth target, 0 = empty
@group(1) @binding(1) var scolor: texture_2d<f32>;

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
    let pix = vec2<i32>(in.pos.xy);
    let d = textureLoad(sdepth, pix, 0);
    if (d == 0.0) {
        discard; // no point landed here
    }
    var o: FsOut;
    var rgb = textureLoad(scolor, pix, 0).rgb;

    // EYE-DOM LIGHTING - Cloudcompare - potree formula.
    // Darket a pixel by how much closer its neihgbor are
    // depth discontinuities become dark rims, and a normal-less LIDAR
    // cloud suddenly reads as a 3D surface.
    // All from the splat depth buffer, four taps.
    // Our depth is reverse-z ndc bits; -log2(z) grows with distance like Potree's log depth
    let strength = cloud._pad;
    if (strength > 0.0) {
        let w = i32(cloud.vp_w);
        let h = i32(cloud.vp_h);
        let me = -log2(max(d, 1.0e-7));
        var sum = 0.0;
        for (var k = 0; k < 4; k++){
            var q = vec2<i32>(in.pos.xy);
            if (k==0) {
                q.x -= 1;
            } else if (k==1) {
                q.x += 1;
            } else if ( k==2 ) {
                q.y -= 1;
            } else {
                q.y += 1;
            }

            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h ) {
                continue;
            }

            let nd = textureLoad(sdepth, q, 0);

            if (nd == 0.0) {
                continue; // empty neighbour: no opinion;
            }
            sum += max(0.0, me - (-log2(max(nd, 1.0e-7))));
        }

        // floor at 0.25: an edge darkens, it never goes pure black - sparse dots
        // otherwise grow cartoon outliens instead of shading
        let shade = max(exp(-sum * 75.0 * strength), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
    o.depth = d;
    return o;
}
