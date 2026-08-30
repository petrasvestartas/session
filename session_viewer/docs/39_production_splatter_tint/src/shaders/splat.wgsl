// Computer-shader point splatting for the cloud lane (Schutz-style https://github.com/m-schuetz/compute_rasterizer)
// One thread per point. Pass 1 (cs_depth): atomicMax the point's reverse-Z depth into a
// per-pixel u32 buffer for every pixel of its disc - bigger f32 bits = closer, and positive
// f32 compare correctly as u32s. Pass2 (cs_color): re-project, and the thread those depth
// won a pixel stores its colour there. No rasteriser, no per-point vertices, no discard.

// splat.wgsl is COMPUTE (cs_depth, cs_color). Compute shaders have no framebuffer
// they cannot draw a pixel to the screen or touch the depth attachment at all. 
// What they can do is hammer atomics into plain storage buffers, so they build a hand-made z-buffer: 
// sdepth (per-pixel winning reverse-Z bits via atomicMax) and scolor (the winner's colour). 
// That's the whole trick of the lane — the "rasterizer" is these two dispatches, and it runs in the compute prelude before the render pass.

struct CloudUniform{
    size: f32,
    vp_w: f32,
    vp_h: f32,
    _pad: f32,
};

// The record table is read as raw words - 4-word header {n, total, 0, 0}, then 24 words per record:
// 16 matrix (mvp x model, column-major) and {first, count, cum, rbits}.
// 4 tint
// Raw indexing sidesteps every struct-layout question between Rust pacaking and WGSL rules.
const REC_WORDS: u32 = 24u;

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(0) @binding(1) var<uniform> cloud: CloudUniform;
@group(0) @binding(2) var<storage, read> instances_unused: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read> table: array<u32>;

@group(1) @binding(0) var<storage, read> positions: array<f32>;
@group(1) @binding(1) var<storage, read> colors: array<u32>;
@group(1) @binding(2) var<storage, read_write> sdepth: array<atomic<u32>>;
@group(1) @binding(3) var<storage, read_write> scolor: array<u32>;

struct Splat {
    px: vec2<i32>,
    r: f32,
    dbits: u32,
    color: u32,
    ok: bool,
};

fn rec_f(base: u32, w: u32) -> f32 {
    return bitcast<f32>(table[base + w]);
}

fn project(gid: u32) -> Splat {
    var s: Splat;
    s.ok = false;
    if (gid >= table[1]) { return s; } // header: total threads
    let n = table[0];
    var i = 0u;
    var base = 4u;
    for (var j = 0u; j < n; j++) {
        let b = 4u + j * REC_WORDS;
        let cum = table[b + 22u];
        let count = table[b + 21u];
        if (gid >= cum && gid < cum + count) { i = table[b + 20u] + (gid - cum); base = b; break; }
    }
    let m = mat4x4<f32>(
        vec4<f32>(rec_f(base, 0u),  rec_f(base, 1u),  rec_f(base, 2u),  rec_f(base, 3u)),
        vec4<f32>(rec_f(base, 4u),  rec_f(base, 5u),  rec_f(base, 6u),  rec_f(base, 7u)),
        vec4<f32>(rec_f(base, 8u),  rec_f(base, 9u),  rec_f(base, 10u), rec_f(base, 11u)),
        vec4<f32>(rec_f(base, 12u), rec_f(base, 13u), rec_f(base, 14u), rec_f(base, 15u)),
    );
    let clip = m * vec4<f32>(positions[i * 3u], positions[i * 3u + 1u], positions[i * 3u + 2u], 1.0);
    if (clip.w <= 0.0) { return s; }
    let ndc = clip.xyz / clip.w;
    if (ndc.z < 0.0 || ndc.z > 1.0) { return s; } // outside [far, near] in reverse-Z
    s.r = bitcast<f32>(table[base + 23u]);
    let x = (ndc.x * 0.5 + 0.5) * cloud.vp_w;
    let y = (0.5 - ndc.y * 0.5) * cloud.vp_h;
    if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) { return s; }
    s.px = vec2<i32>(i32(x), i32(y));
    s.dbits = bitcast<u32>(ndc.z);
    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0);
    s.color = pack4x8unorm(unpack4x8unorm(colors[i]) * tint);
    s.ok = true;
    return s;
}


// DIspatched a a 2D grid: 4096 workgroups wide, as many rows as needed - a 1D dispatch
// caps at 65535 workgroups (4.2M threads), well under a 7M-point frame, and an oversized
// dispatch invalidates the whole command buffer: the frame silently never draws.
const STRIDE: u32 = 4096u * 64u; // threads per grid row

@compute @workgroup_size(64)
fn cs_depth(@builtin(global_invocation_id) g: vec3<u32>){
    let s = project(g.y * STRIDE + g.x);
    if (!s.ok) { return; }
    let ir = i32(ceil(s.r - 0.5));
    let w = i32(cloud.vp_w);
    let h = i32(cloud.vp_h);
    for (var dy = -ir; dy <= ir; dy++) {
        for (var dx = -ir; dx <= ir; dx++) {
            let q = s.px + vec2<i32>(dx, dy);
            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) { continue; }
            if (f32(dx * dx + dy * dy) > s.r * s.r) { continue; } // ROUND dot
            let idx = u32(q.y) * u32(w) + u32(q.x);
            // Contention killer: plain load first, the atomic RMW only when this point
            // would actually win - losing threads must not serialize on the atomic unit.
            if (s.dbits > atomicLoad(&sdepth[idx])) {
                atomicMax(&sdepth[idx], s.dbits);
            }
        }
    }
}

@compute @workgroup_size(64)
fn cs_color(@builtin(global_invocation_id) g: vec3<u32>) {
    let s = project(g.y * STRIDE + g.x);
    if (!s.ok) { return; }
    let ir = i32(ceil(s.r - 0.5));
    let w = i32(cloud.vp_w);
    let h = i32(cloud.vp_h);
    for (var dy = -ir; dy <= ir; dy++) {
        for (var dx = -ir; dx <= ir; dx++) {
            let q = s.px + vec2<i32>(dx, dy);
            if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) { continue; }
            if (f32(dx * dx + dy * dy) > s.r * s.r) { continue; }
            let idx = u32(q.y) * u32(w) + u32(q.x);
            // The winner of pass 1 owns the pixel; equal-depth ties race, any tied colour
            // is a correct answer.
            if (atomicLoad(&sdepth[idx]) == s.dbits) { scolor[idx] = s.color; }
        }
    }
}