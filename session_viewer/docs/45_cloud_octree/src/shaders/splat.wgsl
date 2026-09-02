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

// The record table is read as raw words - 4-word header {n, total, 0, 0}, then 36 words per
// record: 16 matrix (mvp x model, column-major), 4 tint (.a = minimum radius px),
// {first, count, cum, k-bits}, then 12 words of the model's rotation columns for normals.
// Raw indexing sidesteps every struct-layout question between Rust pacaking and WGSL rules.
const REC_WORDS: u32 = 36u;

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(0) @binding(1) var<uniform> cloud: CloudUniform;
@group(0) @binding(2) var<storage, read> instances_unused: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read> table: array<u32>;

@group(1) @binding(0) var<storage, read> positions: array<f32>;
@group(1) @binding(1) var<storage, read> colors: array<u32>;
@group(1) @binding(2) var<storage, read_write> sdepth: array<atomic<u32>>;
@group(1) @binding(3) var<storage, read_write> scolor: array<u32>;
@group(1) @binding(4) var<storage, read> normals: array<u32>; // oct16; MAX = point has none

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

    // Attenuated radius: the record's k folds the cloud's world-space point footprint and the
    // projection, so the screen radius is one divide - big near, dust far, gap-free in between
    // (Potree's attenuated model). The floor (tint.a) keeps the manifest px at range.
    let r_min = rec_f(base, 19u);
    s.r = clamp(bitcast<f32>(table[base + 23u]) * cloud.vp_h / clip.w, r_min, 8.0);

    let x = (ndc.x * 0.5 + 0.5) * cloud.vp_w;
    let y = (0.5 - ndc.y * 0.5) * cloud.vp_h;
    if (x < -s.r || y < -s.r || x >= cloud.vp_w + s.r || y >= cloud.vp_h + s.r) { return s; }
    s.px = vec2<i32>(i32(x), i32(y));
    s.dbits = bitcast<u32>(ndc.z);

    let tint = vec4<f32>(rec_f(base, 16u), rec_f(base, 17u), rec_f(base, 18u), 1.0); // .a is r_min
    var rgba = unpack4x8unorm(colors[i]) * tint;
    // LAMBERT, when the point has a normal. The record's trailing words carry the model's
    // rotation columns, so the oct16 normal reaches world space; abs() because a scanned
    // normal's orientation is a coin toss.
    let packed_n = normals[i];
    if (packed_n != 0xffffffffu) {
        let rot = mat3x3<f32>(
            vec3<f32>(rec_f(base, 24u), rec_f(base, 25u), rec_f(base, 26u)),
            vec3<f32>(rec_f(base, 28u), rec_f(base, 29u), rec_f(base, 30u)),
            vec3<f32>(rec_f(base, 32u), rec_f(base, 33u), rec_f(base, 34u)),
        );
        let nw = normalize(rot * oct16_decode(packed_n));
        let light = normalize(vec3<f32>(0.4, 0.4, 0.8)); // fixed key light
        let lambert = 0.25 + 0.75 * abs(dot(nw, light));
        rgba = vec4<f32>(rgba.rgb * lambert, rgba.a);
    }
    s.color = pack4x8unorm(rgba);

    s.ok = true;
    return s;
}

// DIspatched a a 2D grid: 4096 workgroups wide, as many rows as needed - a 1D dispatch
// caps at 65535 workgroups (4.2M threads), well under a 7M-point frame, and an oversized
// dispatch invalidates the whole command buffer: the frame silently never draws.
const STRIDE: u32 = 4096u * 64u; // threads per grid row

// The workgroup's 64 projected points, so its 64 LANES can rasterize them TOGETHER instead of
// each lane owning a disc of its own.
//
// This lane is memory-pattern bound, and that was measured rather than guessed. With one thread
// per point the 64 lanes of one SIMD instruction touch 64 unrelated screen rows, so every lane's
// read is its own memory transaction. Three things that should have helped did nothing at all on
// the lion (342k points, 900x700, camera moving): bounding each disc row exactly instead of
// walking the square and rejecting corners (21.1 -> 21.1 ms), reading `sdepth` non-atomically
// (21.16 -> 21.14 ms), and workgroup sizes 32/128/256 (41.2 ms every time). Feeding that same
// instruction 64 CONSECUTIVE addresses - identical read count, identical arithmetic, only the
// pattern changed - took the depth pass from 21.1 ms to 6.2 ms. Splitting ONE point's disc across
// the lanes is how that pattern is reached honestly: a disc row is up to 17 consecutive pixels,
// so a couple of cache lines serve what used to need 64 separate fetches.
//
// The DEPTH buffer is provably untouched: pass 1 is `atomicMax` over a set of (pixel, depth)
// updates, which is commutative and idempotent, so the winner cannot depend on the order the
// discs arrive in. Measured, not just argued - with only this pass made cooperative, the lion
// rendered BYTE-IDENTICAL to the old shader over three runs.
//
// Pass 2 writes only where that settled depth already matches, so it covers the same pixels; what
// it cannot fix is a pixel where TWO points hold the winning depth exactly. Those already raced
// (hence `nondet(splat)` in docs/_GOLDENS.tsv) and reordering picks the other one: the lion has
// exactly ONE such pixel, (529,290), and ink/draws/objects - what the gate compares on a cloud
// scene - do not move. The byte-identical depth pass is the proof that this is the tie and
// nothing else, because pass 2 can only write where the depth already agrees.
var<workgroup> wg_splats: array<Splat, 64>;
/// The four sampled lanes' bounding-square cell counts - all the mode decision is taken on.
var<workgroup> wg_cells: array<u32, 4>;
/// What `prepare` hands `rasterize`: this lane's own point, and the workgroup's mode.
struct Prep { s: Splat, coop: bool };

// Which cells of a point's bounding square this lane takes: cell `lane`, then every 64th after
// it. `adv` is that +64 step decomposed into (rows, cols) ONCE per point, so the walk carries
// with an add and a compare and never divides inside the loop - `side` is a runtime value, and an
// integer division per cell would cost more than the memory traffic this whole change buys.
struct Walk { row: u32, col: u32, adv_row: u32, adv_col: u32 };

fn walk_start(lane: u32, side: u32) -> Walk {
    var k: Walk;
    k.row = lane / side;
    k.col = lane - k.row * side;
    k.adv_row = 64u / side;
    k.adv_col = 64u - k.adv_row * side; // < side, so the single fixup below is always enough
    return k;
}

fn walk_next(k: Walk, side: u32) -> Walk {
    var n = k;
    n.col = k.col + k.adv_col;
    n.row = k.row + k.adv_row;
    if (n.col >= side) { n.col = n.col - side; n.row = n.row + 1u; }
    return n;
}

/// Side of a splat's bounding square, in pixels. `ceil(r - 0.5)` is never smaller than
/// `floor(r)`, so the square never clips the disc the round test then carves out of it.
fn splat_side(s: Splat) -> u32 {
    return u32(2 * i32(ceil(s.r - 0.5)) + 1);
}

/// One cell of one disc, in whichever pass. `color_pass` is a literal at both call sites, so
/// there is one branch here in the source and none in either compiled shader.
fn emit(s: Splat, dx: i32, dy: i32, w: i32, h: i32, rr: f32, color_pass: bool) {
    let q = s.px + vec2<i32>(dx, dy);
    if (q.x < 0 || q.y < 0 || q.x >= w || q.y >= h) { return; }
    if (f32(dx * dx + dy * dy) > rr) { return; } // ROUND dot
    let idx = u32(q.y) * u32(w) + u32(q.x);
    if (color_pass) {
        // The winner of pass 1 owns the pixel; equal-depth ties race, any tied colour
        // is a correct answer.
        if (atomicLoad(&sdepth[idx]) == s.dbits) { scolor[idx] = s.color; }
    } else {
        // Contention killer: plain load first, the atomic RMW only when this point
        // would actually win - losing threads must not serialize on the atomic unit.
        if (s.dbits > atomicLoad(&sdepth[idx])) { atomicMax(&sdepth[idx], s.dbits); }
    }
}

/// Project this lane's point, and answer whether this workgroup should go COOPERATIVE.
///
/// Sharing the lanes out over one disc only pays when a disc has more cells than there are lanes.
/// Below that the spare lanes idle through every point, and MEASURED, that is not a small tax: on
/// `cloud_mix` (7.5M points, two scans authored at 1 px, so most discs are a single cell) an
/// unconditionally cooperative shader ran 212 ms against the old 77 ms. So the mode is decided per
/// workgroup - and it has to be decided UNIFORMLY, because the cooperative loop hands each lane a
/// SHARE of the cells and a lane that took the other branch would leave its share unrasterized.
///
/// Deciding is not free, and a scene of pure dust pays for it and gets nothing back, so it is
/// made as cheap as it can be: FOUR u32s written by four lanes, and the 24-byte `Splat` reaches
/// `wg_splats` only when the answer is yes. On `lidar14` (13.8M points, all of them dust, which
/// then takes the unchanged path) that is the difference between 44.3 ms and 43.3 ms against a
/// 41.7 ms floor - and dropping one of the two barriers recovered NONE of the rest, so what is
/// left is the first barrier itself, which is the price of asking the question at all. That
/// residual 4% on dust is the trade for 33-38% on scenes whose discs are real.
///
/// Four samples out of 64 decide as well as a real maximum in practice, because a workgroup's 64
/// points are consecutive in one cloud and so are the same size on screen - and a wrong answer
/// here is only ever slower, never different: both branches rasterize the same discs.
fn prepare(gid: u32, lane: u32) -> Prep {
    var pr: Prep;
    pr.s = project(gid);
    if ((lane & 15u) == 0u) {
        let side = splat_side(pr.s);
        wg_cells[lane >> 4u] = select(0u, side * side, pr.s.ok);
    }
    workgroupBarrier();
    var widest = 0u;
    for (var q = 0u; q < 4u; q++) { widest = max(widest, wg_cells[q]); }
    pr.coop = widest > 64u;
    if (pr.coop) { wg_splats[lane] = pr.s; }
    workgroupBarrier();
    return pr;
}

fn rasterize(pr: Prep, lane: u32, color_pass: bool) {
    let w = i32(cloud.vp_w);
    let h = i32(cloud.vp_h);
    if (!pr.coop) { // one thread, one point - the original loop, for workgroups of dust
        let s = pr.s;
        if (!s.ok) { return; }
        let ir = i32(ceil(s.r - 0.5));
        let rr = s.r * s.r;
        for (var dy = -ir; dy <= ir; dy++) {
            for (var dx = -ir; dx <= ir; dx++) { emit(s, dx, dy, w, h, rr, color_pass); }
        }
        return;
    }
    for (var p = 0u; p < 64u; p++) {
        let s = wg_splats[p];
        if (!s.ok) { continue; }
        let ir = i32(ceil(s.r - 0.5));
        let side = u32(2 * ir + 1);
        let rr = s.r * s.r;
        var k = walk_start(lane, side);
        while (k.row < side) {
            emit(s, i32(k.col) - ir, i32(k.row) - ir, w, h, rr, color_pass);
            k = walk_next(k, side);
        }
    }
}

@compute @workgroup_size(64)
fn cs_depth(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_index) lane: u32){
    rasterize(prepare(g.y * STRIDE + g.x, lane), lane, false);
}

@compute @workgroup_size(64)
fn cs_color(@builtin(global_invocation_id) g: vec3<u32>, @builtin(local_invocation_index) lane: u32) {
    rasterize(prepare(g.y * STRIDE + g.x, lane), lane, true);
}

// Octahedral decode: undo the fold, then normalize - the mirror of scene.rs oct16()
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(
        f32(i32(p << 24u) >> 24u) / 127.0,
        f32(i32(p << 16u) >> 24u) / 127.0,
    );
    var n = vec3<f32>(e, 1.0 -abs(e.x) - abs(e.y));
    if (n.z < 0.0){
        let sgn = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * sgn.x, (1.0 - abs(n.x)) * sgn.y, n.z);
    }
    return normalize(n);
}
