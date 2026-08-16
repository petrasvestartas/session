// The SOLID lane, as one primitive.
//
// A mesh edge is a tube and a mesh vertex is a ball, and until now each had its own tessellated
// template: unit_cylinder(6) = 12 triangles an edge, unit_sphere() 12x6 = 144 triangles a vertex.
// On a CAD solid that is free. On the Stanford ladder it was 23.2M + 92.9M triangles to decorate
// 1.29M triangles of actual mesh - 90x - and it is why dense meshes had to be stripped bare.
//
// Both are the same shape: a CAPSULE. A ball is a capsule whose two ends coincide. So one
// primitive draws both, from ONE instance table, in ONE draw call:
//
//     6 vertices per instance, a screen-space quad along the projected segment,
//     the round ends carved out by a 2D capsule SDF in the fragment.
//
//     edge   12 tris -> 2   (6x)
//     vertex 144 tris -> 2  (72x)
//     total  116M    -> 5.2M triangles
//
// The silhouette gets BETTER, not worse: it is exact at any zoom instead of a 6-sided prism and
// a 12x6 lat-long ball. This is the same trick ribbon.wgsl already uses for flat linework - the
// solid lane simply never got it - with two differences: this one writes depth, and it extends
// the quad ALONG the axis by the radius so there is room for the caps.
//
// Both lanes' fragment shaders return flat colour, so no normal is needed - only the silhouette
// and a depth that varies correctly along the tube. Depth comes from interpolating the two
// endpoints' clip values, so early-Z survives: nothing here writes frag_depth.

@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment (48 B). A vertex dot is p0 == p1.
struct CylinderSegment{
    p0: vec3<f32>,
    radius: f32,
    p1: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    anchor: vec3<f32>,
};

const FLAG_HIDDEN: u32 = 2u;

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    // Capsule space, in PIXELS: x runs along the axis from end0, y is perpendicular. Doing the
    // SDF here rather than from @builtin(position) keeps it independent of the framebuffer's
    // y-down convention, which is the classic way to get caps mirrored.
    @location(1) cap: vec2<f32>,
    @location(2) @interpolate(flat) len_px: f32,
    @location(3) @interpolate(flat) rad_px: f32,
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let seg = segments[vid / 6u];
    let corner = vid % 6u;
    let inst = instances[seg.instance_id];

    let w0 = (inst.model * vec4<f32>(seg.p0, 1.0)).xyz;
    let w1 = (inst.model * vec4<f32>(seg.p1, 1.0)).xyz;
    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);

    // 0,3 = end0 -side | 1,4 = end1 -side/+side | 2,5 = +side  (two triangles)
    let at_end1 = corner == 1u || corner == 2u || corner == 4u;
    let side = select(-1.0, 1.0, corner == 2u || corner == 4u || corner == 5u);
    let clip = select(c0, c1, at_end1);

    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (c0.xy / max(abs(c0.w), 1e-6) * 0.5 + 0.5) * vp;
    let s1 = (c1.xy / max(abs(c1.w), 1e-6) * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let l = length(d);
    // A ball is a capsule of zero length: any direction will do, so pick one rather than NaN.
    let dir = select(vec2<f32>(1.0, 0.0), d / l, l > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // Same radius rule as cylinder.wgsl / ribbon.wgsl: a negative radius is a multiplier on the
    // global pen, a positive one is a world-space radius projected to pixels.
    let mult = select(1.0, -seg.radius, seg.radius < 0.0);
    var px = line.thickness * mult;
    if (seg.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = seg.radius * line.vp_h / line.ortho_h;
        } else {
            px = seg.radius * line.proj_y * line.vp_h / max(clip.w, 1e-6);
        }
    }
    px = max(px, 0.5);

    // Expand perpendicular AND along the axis, so the round caps have room to be carved out.
    let base = select(s0, s1, at_end1);
    let along = select(-px, px, at_end1);
    let sp = base + dir * along + n * (side * px);

    var o: VsOut;
    let ndc = (sp / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * abs(clip.w), clip.z, clip.w);
    if ((inst.flags & FLAG_HIDDEN) != 0u) {
        o.pos = vec4<f32>(0.0, 0.0, -1.0, 1.0);
    }
    o.color = seg.color * inst.color;
    o.cap = vec2<f32>(select(-px, l + px, at_end1), side * px);
    o.len_px = l;
    o.rad_px = px;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    // 2D capsule SDF: distance from this pixel to the axis segment (0,0)-(len,0).
    let x = in.cap.x - clamp(in.cap.x, 0.0, in.len_px);
    if (length(vec2<f32>(x, in.cap.y)) > in.rad_px) {
        discard;
    }
    return in.color;
}
