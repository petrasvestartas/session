@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{ model: mat4x4<f32>, color: vec4<f32>, flags: u32, };
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment (48 B) — same table the cylinder pipeline reads.
struct CylinderSegment{
    p0: vec3<f32>, radius: f32, p1: vec3<f32>, instance_id: u32, color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32,
    vp_w: f32, _pad0: f32, _pad1: f32, _pad2: f32,
};

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>, // this fragment's screen position, px
    @location(2) @interpolate(flat) a: vec2<f32>,   // segment endpoints on screen, px
    @location(3) @interpolate(flat) b: vec2<f32>,
    @location(4) @interpolate(linear) hw: f32,      // half-width, px
};

//   corner 0: e0−   1: e1−   2: e1+     (tri 1)
//   corner 3: e0−   4: e1+   5: e0+     (tri 2)
@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let seg = segments[vid / 6u];
    let corner = vid % 6u;
    let model = instances[seg.instance_id].model;

    let w0 = (model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(seg.p1, 1.0)).xyz;
    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);

    let at_end1 = corner == 1u || corner == 2u || corner == 4u;
    let side = select(-1.0, 1.0, corner == 2u || corner == 4u || corner == 5u);
    let clip = select(c0, c1, at_end1);

    // Endpoints in screen pixels (one consistent mapping, used both ways)
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (c0.xy / max(abs(c0.w), 1e-6) * 0.5 + 0.5) * vp;
    let s1 = (c1.xy / max(abs(c1.w), 1e-6) * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // px half-width at this end: global thickness, or a world radius projected (>0) —
    // the inverse of cylinder.wgsl's screen_radius, solved for pixels.
    let mult = select(1.0, -seg.radius, seg.radius < 0.0);
    var px = line.thickness * mult;
    if (seg.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = seg.radius * line.vp_h / line.ortho_h;
        } else {
            px = seg.radius * line.proj_y * line.vp_h / clip.w;
        }
        px = max(px, 0.5); // paper-space ink never vanishes: floor at ~1px on screen
    }

    // Corner in px: sideways ± half-width, and PAST the end by half-width (cap room)
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n * side + dir * along) * px;

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * clip.w, clip.zw);
    o.color = seg.color * instances[seg.instance_id].color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw = px;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    // Capsule SDF in screen px — rounds both caps; MSAA smooths the rim.
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    if (length(pa - ba * h) > in.hw) { discard; }
    return in.color;
}
