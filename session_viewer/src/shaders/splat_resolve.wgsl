// Composite the point pass into the scene: one fullscreen triangle looks up its pixel in the
// lane's depth + colour targets, discards empties, applies Eye-Dome Lighting from the four
// neighbouring depths, and exports the point's depth through frag_depth so points and solids
// occlude each other exactly.

struct CloudUniform {
    size: f32,
    vp_w: f32,
    vp_h: f32,
    edl: f32,
};
@group(0) @binding(0) var<uniform> cloud: CloudUniform;
@group(1) @binding(0) var sdepth: texture_depth_2d;
@group(1) @binding(1) var scolor: texture_2d<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut {
    var o: VsOut;
    let x = f32(i32(vid & 1u) * 4 - 1);
    let y = f32(i32(vid >> 1u) * 4 - 1);
    o.pos = vec4<f32>(x, y, 0.0, 1.0);
    return o;
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

// -log2 of a reverse-Z depth grows with distance, like Potree's log depth.
fn log_depth(d: f32) -> f32 {
    return -log2(max(d, 1.0e-7));
}

fn shade(in: VsOut) -> FsOut {
    let pix = vec2<i32>(in.pos.xy);
    let d = textureLoad(sdepth, pix, 0);
    if (d == 0.0) {
        discard;
    }
    var o: FsOut;
    var rgb = textureLoad(scolor, pix, 0).rgb;

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
        // Floored at 0.25: an edge darkens, it never goes black.
        let shade = max(exp(-sum * 75.0 * cloud.edl), 0.25);
        rgb *= shade;
    }

    o.color = vec4<f32>(rgb, 1.0);
    o.depth = d;
    return o;
}

struct FaceOut {
    @location(0) color: vec4<f32>,
    @location(1) face: vec2<u32>,
    @builtin(frag_depth) depth: f32,
};

@fragment
fn fs_main(in: VsOut) -> FsOut {
    return shade(in);
}

@fragment
fn fs_face(in: VsOut) -> FaceOut {
    let shaded = shade(in);
    return FaceOut(shaded.color, vec2<u32>(0u), shaded.depth);
}
