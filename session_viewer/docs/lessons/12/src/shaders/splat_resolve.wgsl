// Point cloud settings, 48 bytes; matches CloudUniform in Rust.
struct CloudUniform {
    size: f32, // point size scale; applied on the CPU
    vp_w: f32, // target width, px
    vp_h: f32, // target height, px
    edl: f32, // eye-dome lighting strength; 0 = off
};

@group(0) @binding(0) var<uniform> cloud: CloudUniform; // cloud settings
@group(1) @binding(0) var sdepth: texture_depth_2d; // nearest point depth per pixel
@group(1) @binding(1) var scolor: texture_2d<f32>; // its color

// One vertex of the fullscreen triangle.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
};

@vertex
// Place the three corners of a screen-covering triangle.
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    let x = f32(i32(vid & 1u) * 4 - 1);
    let y = f32(i32(vid >> 1u) * 4 - 1);
    o.pos = vec4<f32>(x, y, 0.0, 1.0);
    return o;
}

// Output: color, depth slope, and the depth written to the scene.
struct FsOut {
    @location(0) color: vec4<f32>, // rgba
    @location(1) gradient: vec2<f32>,
    @builtin(frag_depth) depth: f32, // point depth into the scene
};

// Depth on a log scale that grows with distance.
fn log_depth(d: f32) -> f32 {
    return -log2(max(d, 1.0e-7));
}

// Copy the point texture into the scene, darkening edges when EDL is on.
fn shade(in: VsOut) -> FsOut {
    let pix = vec2<i32>(in.pos.xy);
    let d = textureLoad(sdepth, pix, 0);

    // no point here
    if (d == 0.0) {
        discard;
    }

    var o: FsOut;
    var rgb = textureLoad(scolor, pix, 0).rgb;

    // eye-dome lighting: darken where neighbours are nearer
    if (cloud.edl > 0.0) {
        let w = i32(cloud.vp_w);
        let h = i32(cloud.vp_h);
        let me = log_depth(d);
        var sum = 0.0;
        var taps = array<vec2<i32>, 4>(vec2<i32>(-1, 0), vec2<i32>(1, 0), vec2<i32>(0, -1), vec2<i32>(0, 1));

        for (var k = 0; k < 4; k++) {
            let q = pix + taps[k];

            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) {
                continue;
            }

            let nd = textureLoad(sdepth, q, 0);

            if (nd == 0.0) {
                continue;
            }

            sum += max(0.0, me - log_depth(nd));
        }

        // darken by the depth steps, never below a quarter
        let shade = max(exp(-sum * 75.0 * cloud.edl), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
    o.depth = d;
    o.gradient = vec2<f32>(0.0);
    return o;
}

@fragment
// Resolve one pixel.
fn fs_main(in: VsOut) -> FsOut {
    return shade(in);
}
