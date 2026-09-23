// One vector: a shaft with an arrowhead at `end`, 48 bytes; matches VectorRow in Rust.
struct VectorRow {
    start: vec3<f32>, // tail, object space
    radius: f32, // 0 = pen; > 0 world mm; < 0 pen multiplier
    end: vec3<f32>, // tip, object space
    instance_id: u32, // object row
    color: u32, // packed rgba
    head: f32, // arrowhead length in pens; 0 = HEAD_PENS
    pad: vec2<u32>, // to 48 bytes
};

@group(3) @binding(0) var<storage, read> vectors: array<VectorRow>; // one row per vector

// Default arrowhead length, in pen widths.
const HEAD_PENS: f32 = 10.0;
// Half the arrowhead's base width over its length.
const HEAD_ASPECT: f32 = 0.4;
// Half a pixel's diagonal: how far a pixel reaches from its center.
const FILTER_REACH: f32 = 0.70711;
// Quad corner of vertex `k` of 6: 0 back-, 1 back+, 2 front-, 3 front+.
const CORNERS = array<u32, 6>(0u, 1u, 2u, 2u, 1u, 3u);

// What the vertex shader hands the fragment shader.
struct VsOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) color: vec4<f32>, // rgba
    @location(1) @interpolate(linear) p: vec2<f32>, // this corner, screen px
    @location(2) @interpolate(flat) a: vec2<f32>, // tail, screen px
    @location(3) @interpolate(flat) b: vec2<f32>, // tip, screen px
    @location(4) @interpolate(flat) base: vec2<f32>, // arrowhead base center, screen px
    @location(5) @interpolate(flat) wing: vec2<f32>, // base center to one base corner, px
    @location(6) @interpolate(flat) hw: vec2<f32>, // shaft half width at tail and tip, px
    @location(7) @interpolate(flat) end_depth: vec2<f32>, // depth at tail and tip
    @location(8) @interpolate(flat) inst_id: u32, // object row
};

// Half width in px at depth `w`, from the radius field.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius < 0.0) { return -radius * line.thickness; }
    if (radius > 0.0) {
        if (line.ortho_h > 0.0) {
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }

        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }

    return line.thickness * 0.5;
}

// A vertex placed off screen, so nothing is drawn.
fn dead_vertex() -> VsOut {
    var dead: VsOut; // all zero; only the position matters
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    return dead;
}

// Corner `vid` of 12 of vector `row`: 0-5 the shaft quad, 6-11 the head quad.
@vertex
fn vs_main(@builtin(vertex_index) vid: u32, @builtin(instance_index) row: u32) -> VsOut {
    let v = vectors[row];
    let inst = instances[v.instance_id];

    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        return dead_vertex();
    }

    // one clipping plane cuts the whole vector away
    if (clip_active() && clip_cut_both(inst.flags, place(v.instance_id, v.start), place(v.instance_id, v.end))) {
        return dead_vertex();
    }

    let c0 = mvp * vec4<f32>(place(v.instance_id, v.start), 1.0);
    let c1 = mvp * vec4<f32>(place(v.instance_id, v.end), 1.0);

    // clip to the near plane before dividing by w
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;

    if (f0 > 0.0 && f1 > 0.0) {
        return dead_vertex();
    }

    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);

    // both ends in screen pixels
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
    let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // a selected shaft is at least a full pen wide
    let selected = (inst.flags & FLAG_SELECTED) != 0u;
    let hw0 = max(half_width_px(v.radius, e0.w), select(0.0, line.thickness, selected));
    let hw1 = max(half_width_px(v.radius, e1.w), select(0.0, line.thickness, selected));

    // head: constant on screen, wider than the shaft, never longer than 0.6 of the vector
    var head = max(select(HEAD_PENS, v.head, v.head > 0.0) * line.thickness, hw1 * 6.0);
    head = select(min(head, len * 0.6), 0.0, f1 > 0.0);
    let half = head * HEAD_ASPECT;
    let base = s1 - dir * head;

    // the head quad starts where the shaft quad ends, so no pixel is blended twice
    let k = vid % 6u;
    let corner = CORNERS[k];
    let front = corner >= 2u;
    let side = select(-1.0, 1.0, (corner & 1u) == 1u);
    let seam = base - dir * FILTER_REACH;
    var along: vec2<f32>;
    var across: f32;

    if (vid < 6u) {
        along = select(s0 - dir * (max(hw0, 0.5) + FILTER_REACH), seam, front);
        across = max(max(hw0, hw1), 0.5) + FILTER_REACH;
    } else {
        // a sharp tip antialiases past itself by the reach over the tip half-angle's sine
        let tip = FILTER_REACH * sqrt(head * head + half * half) / max(half, 1e-3);
        along = select(seam, s1 + dir * tip, front);
        across = max(half, max(hw1, 0.5)) + FILTER_REACH;
    }

    let p = along + n * side * across;
    let depth = clamp(select(e0.z / e0.w, e1.z / e1.w, front), 0.0, 1.0);

    var o: VsOut;
    o.pos = vec4<f32>((p / vp - 0.5) * 2.0, depth, 1.0);
    var color = edge_color(unpack4x8unorm(v.color), inst);

    if (selected) {
        color = vec4<f32>(SELECT_COLOR, color.a);
    }

    o.color = color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.base = base;
    o.wing = n * half;
    o.hw = vec2<f32>(hw0, hw1);
    o.end_depth = vec2<f32>(e0.z / e0.w, e1.z / e1.w);
    o.inst_id = v.instance_id;
    return o;
}

// Fraction of a pixel square lying below signed distance `t` from a line.
fn box_cdf(t: f32, hi: f32, lo: f32, m: f32, q: f32) -> f32 {
    let s = clamp(t, -hi, hi);
    let e = hi - abs(s);
    let tail = select(e * e / q, 1.0 - e * e / q, s > 0.0);
    return select(tail, 0.5 + s / m, abs(s) <= lo);
}

// Exact area of this pixel within `hw` of the line, `d` away, direction `g`; the ribbon filter.
fn band_area(d: f32, hw: f32, g: vec2<f32>) -> f32 {
    let a = abs(g.x);
    let b = abs(g.y);
    let hi = 0.5 * (a + b);
    let lo = 0.5 * abs(a - b);
    let m = max(max(a, b), 1e-6);
    let q = max(2.0 * a * b, 1e-6);
    return box_cdf(hw - d, hi, lo, m, q) + box_cdf(hw + d, hi, lo, m, q) - 1.0;
}

// Signed distance from `p` to triangle (p0, p1, p2), negative inside.
fn triangle_distance(p: vec2<f32>, p0: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>) -> f32 {
    let e0 = p1 - p0;
    let e1 = p2 - p1;
    let e2 = p0 - p2;
    let v0 = p - p0;
    let v1 = p - p1;
    let v2 = p - p2;
    let q0 = v0 - e0 * clamp(dot(v0, e0) / max(dot(e0, e0), 1e-6), 0.0, 1.0);
    let q1 = v1 - e1 * clamp(dot(v1, e1) / max(dot(e1, e1), 1e-6), 0.0, 1.0);
    let q2 = v2 - e2 * clamp(dot(v2, e2) / max(dot(e2, e2), 1e-6), 0.0, 1.0);
    // winding, so inside is negative either way round
    let s = select(-1.0, 1.0, e0.x * e2.y - e0.y * e2.x >= 0.0);
    let d = min(min(
        vec2<f32>(dot(q0, q0), s * (v0.x * e0.y - v0.y * e0.x)),
        vec2<f32>(dot(q1, q1), s * (v1.x * e1.y - v1.y * e1.x))),
        vec2<f32>(dot(q2, q2), s * (v2.x * e2.y - v2.y * e2.x)));
    return -sqrt(d.x) * select(-1.0, 1.0, d.y >= 0.0);
}

// How much of this pixel the shaft or head covers, 0..1.
fn coverage(in: VsOut) -> f32 {
    // shaft: distance to the tail-to-base segment
    let ba = in.base - in.a;
    let h = clamp(dot(in.p - in.a, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let raw = mix(in.hw.x, in.hw.y, h);
    let off = in.p - in.a - ba * h;
    let d = length(off);
    let g = select(vec2<f32>(1.0, 0.0), off / d, d > 1e-6);
    // thinner than a pixel: keep half a pixel, fade instead
    let fade = select(1.0, max(raw / 0.5, HAIRLINE_MIN_ALPHA), raw < 0.5);
    let shaft_alpha = clamp(band_area(d, max(raw, 0.5), g), 0.0, 1.0) * fade;

    if (dot(in.wing, in.wing) < 1e-6) {
        return shaft_alpha;
    }

    let head = triangle_distance(in.p, in.b, in.base + in.wing, in.base - in.wing);
    return max(shaft_alpha, clamp(0.5 - head, 0.0, 1.0));
}

// The vector's center line as this fragment sees it.
fn ink_axis(in: VsOut) -> InkAxis {
    let ba = in.b - in.a;
    let len2 = max(dot(ba, ba), 1e-6);
    let h = clamp(dot(in.p - in.a, ba) / len2, 0.0, 1.0);
    let at = in.a + ba * h;
    let len = sqrt(len2);
    let along = select(vec2<f32>(1.0, 0.0), vec2<f32>(ba.x, -ba.y) / len, len > 1e-3);
    let slope = (in.end_depth.y - in.end_depth.x) / max(len, 1e-3);
    return InkAxis(vec2<f32>(at.x, line.vp_h - at.y), mix(in.end_depth.x, in.end_depth.y, h), along, slope);
}

// True when a clipping plane cuts the center line away beside this fragment.
fn axis_cut(in: VsOut) -> bool {
    if (!clip_active()) {
        return false;
    }

    let axis = ink_axis(in);
    return clip_cut_ndc(instances[in.inst_id].flags, clip_ndc(axis.at + line.origin, line.frame, axis.depth));
}

@fragment
// Color: the arrow, faded where geometry hides it.
fn fs_main(in: VsOut, @builtin(sample_index) sample: u32) -> InkColor {
    let covered = coverage(in);

    // no coverage: no depth reads
    if (covered <= 0.0 || axis_cut(in)) {
        discard;
    }

    let hidden = !ink_visible(in.pos.xy, ink_axis(in), sample, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u);
    let alpha = covered * through_glass(hidden);

    if (alpha <= 0.0) {
        discard;
    }

    return InkColor(vec4<f32>(in.color.rgb, in.color.a * alpha));
}

@fragment
// Pick id: object row + 1 and a marker tag.
fn fs_id(in: VsOut) -> @location(0) vec2<u32> {
    if (coverage(in) <= 0.0 || axis_cut(in) || !ink_visible(in.pos.xy, ink_axis(in), 0u, (instances[in.inst_id].flags & FLAG_SMOOTH) != 0u)) {
        discard;
    }

    return vec2<u32>(in.inst_id + 1u, DISC_ID_TAG);
}
