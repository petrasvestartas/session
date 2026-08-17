@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment, 40 B. The ends are SCALARS, not vec3<f32>: WGSL aligns
// vec3<f32> to 16, which padded this struct to 48 and made the 40 impossible.
struct CylinderSegment{
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,   // RGBA8, low byte red
    facing: u32,  // two oct16 adjacent face normals; 0 = no adjacency, always draw
}

// The two adjacent face normals, mesh-local. Octahedral decode: undo the fold, then normalize.
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(
        f32(i32(p << 24u) >> 24u) / 127.0,
        f32(i32(p << 16u) >> 24u) / 127.0,
    );
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0){
        n = vec3<f32>((1.0 - abs(n.y)) * sign(n.x), (1.0 - abs(n.x)) * sign(n.y), n.z);
    }
    return normalize(n);
}

// Is this edge worth drawing at all?
//
// An edge belongs to two faces. If BOTH turn away from the camera it is inside the solid and must
// not be drawn; otherwise it is on the silhouette or on visible surface. That single test is what
// replaces asking the depth buffer, and the reason it has to: a pen has WIDTH, so ink depth-tested
// against the very surface it decorates is either cut by it or has to float in front of it, and the
// float needed scales with the pen while the offset that would supply it makes neighbouring faces
// fight each other. Classifying the edge sidesteps the whole trade.
//
// `facing == 0` means the geometry never had adjacency - free-standing linework, drawing pens,
// BRep edges - and those always draw.
fn edge_faces_camera(seg: CylinderSegment, model: mat4x4<f32>, mid: vec3<f32>) -> bool {
    if (seg.facing == 0u){
        return true;
    }
    // Rotate into world with the model's linear part. Non-uniform scale would strictly want the
    // inverse transpose, but this only decides a SIGN and placements here are rigid or uniform.
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;
    return dot(n0, to_eye) > 0.0 || dot(n1, to_eye) > 0.0;
}
@group(3) @binding(0) var<storage, read> segments: array<CylinderSegment>;

struct LineUniform{
    thickness: f32,
    proj_y: f32,
    ortho_h: f32,
    vp_h: f32,
    vp_w: f32,
    // The camera position, as three SCALARS. It occupies exactly the 12 bytes WGSL pads out
    // between `vp_w` and `anchor` - a vec3<f32> aligns to 16 and would be pushed to offset 32,
    // silently shifting `anchor` and growing the block to 64 B against Rust's 48.
    eye_x: f32,
    eye_y: f32,
    eye_z: f32,
    anchor: vec3<f32>,   // camera-relative anchor, world units (see gpu/mod.rs)
};

// Sub-pixel pens never fade below this: 0 = original continuous fade, 1 = always solid 1px.
const HAIRLINE_MIN_ALPHA = 0.5;

// The scene unit (mm) in metres, matching the factor baked into LineUniform.proj_y.
const MM_TO_M = 0.001;

// How far the ink floats in front of the edge it draws, in HALF-WIDTHS. One is the geometric
// minimum (the tube's own radius) and is not enough: a pen has width, and where the surface under
// it grazes the eye its depth climbs by r*tan(slant) across that width, so the face resurfaces
// INSIDE the band as a tapering wedge. Three covers slants to ~70 degrees. Measured, not guessed -
// at 1 the wedge is plainly visible on a box corner at close zoom, at 3 it is gone.
//
// The cost of a larger value is ink bleeding through geometry within that many half-widths of
// depth, which for a 2px pen is about 3px worth - nothing. It scales with the pen, which is what
// keeps it honest for the wide world-mm pens a drawing uses.
// How far the ink floats in front of the edge it draws, in HALF-WIDTHS.
//
// A secant version of this - r*tan(theta) per edge, from the adjacent normals now in the table -
// is the theoretically right law, and it was tried at a ceiling of 64 radii. It changed nothing
// measurable, because the residual it was meant to fix turned out not to be a lift problem at all
// (the edges that vanish at some rotations are genuinely occluded by the box in front). So the
// constant stays: it is one line, it has no per-vertex cost, and it is the value verified across
// five zooms and six orbits.
const LIFT_RADII = 3.0;

// Half-width in px at one end: the global pen thickness, or a world radius projected.
// The inverse of `screen_radius` in cylinder.wgsl, solved for pixels.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0){
        if (line.ortho_h > 0.0){
            return radius * line.vp_h / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h / w;
    }
    return line.thickness * select(1.0, -radius, radius < 0.0);
}

// Hairline rule: never rasterize thinner than 1px - carry the deficit into OPACITY instead, so a
// 0.3px pen renders as a 1px line at 30% alpha and apparent weight stays continuous across zoom
// instead of snapping per pixel row.
//
// ...but floored. A drawing's plot pens (0.09-0.5 mm) are sub-pixel at EVERY sane zoom, so an
// unfloored fade washes the whole sheet out to near-background and its ink reads white. CAD draws
// a sub-pixel pen as a solid 1px hairline; the floor keeps the colour legible while the range
// above it still separates thin pens from fat ones.
fn floor_hairline(px: f32) -> f32 { return max(px, 0.5); }
fn hairline_fade(px: f32) -> f32 {
    if (px < 0.5){ return max(px / 0.5, HAIRLINE_MIN_ALPHA); }
    return 1.0;
}

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(linear) p: vec2<f32>, // this fragment's screen positions in px
    @location(2) @interpolate(flat) a: vec2<f32>, // segment endpoints on screen in px
    @location(3) @interpolate(flat) b: vec2<f32>,
    // Half-width in px at each END, both FLAT. Never interpolated - see `resolve_width`.
    @location(4) @interpolate(flat) hw0: f32,
    @location(5) @interpolate(flat) hw1: f32,
 };

 // The fragment's own half-width and hairline fade, resolved at `h` - its position along the
 // segment, 0 at `a` and 1 at `b`.
 //
 // WHY THIS IS NOT A VARYING. Under perspective the two ends of an edge project to different
 // widths, so the quad is a trapezoid and the width is a function of the ALONG coordinate - which
 // over a trapezoid is projective, not affine. Handing the rasterizer a per-vertex `hw` therefore
 // asks it to interpolate a non-affine quantity: the two triangles of the quad each produce their
 // own affine approximation, the two agree only on the diagonal they share, and the seam shows as
 // a TRIANGULAR BITE out of the band along that diagonal. Harmless when the ends are similar,
 // glaring when you zoom in close enough that one end is several times wider than the other.
 //
 // Resolving it here from two flat endpoint values is exact, and independent of how the quad
 // happens to be triangulated.
 fn resolve_width(in: VsOut, h: f32) -> vec2<f32> {
    let raw = mix(in.hw0, in.hw1, h);
    return vec2<f32>(floor_hairline(raw), hairline_fade(raw));
 }

 // A vertex placed outside NDC, so the whole quad is clipped and nothing rasterizes. Every one of
 // the six vertices takes it, so the triangles are degenerate as well.
 fn dead_vertex() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.p = vec2<f32>(0.0);
    dead.a = vec2<f32>(0.0);
    dead.b = vec2<f32>(0.0);
    dead.hw0 = 0.0;
    dead.hw1 = 0.0;
    return dead;
 }


 // corner 0: e0 - 1: e1 - 2: e1 + (tri 1)
 // corner 3: e0 - 4: e1 + 5: e0 + (tri 2)
 @vertex
 fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let seg = segments[vid / 6u];
    let corner = vid % 6u;
    let model = instances[seg.instance_id].model;

    let l0 = vec3<f32>(seg.p0x, seg.p0y, seg.p0z);
    let l1 = vec3<f32>(seg.p1x, seg.p1y, seg.p1z);
    let w0 = (model * vec4<f32>(l0, 1.0)).xyz;
    let w1 = (model * vec4<f32>(l1, 1.0)).xyz;

    // HIDDEN EDGES NEVER REACH THE RASTERIZER. Both adjacent faces turned away means this edge is
    // inside the solid; drop it here and the ink that remains never has to win a depth argument
    // with the surface it decorates. Collapsed outside NDC so it is clipped, like the near-plane
    // case below.
    if (!edge_faces_camera(seg, model, (w0 + w1) * 0.5)){
        return dead_vertex();
    }

    let c0 = mvp * vec4<f32>(w0, 1.0);
    let c1 = mvp * vec4<f32>(w1, 1.0);

    let at_end1 = corner == 1u || corner == 2u || corner == 4u;
    let side = select(-1.0, 1.0, corner == 2u || corner == 4u || corner == 5u);

    // CLIP THE SEGMENT AGAINST THE NEAR PLANE, BEFORE ANY DIVIDE.
    //
    // This lane projects by hand to get screen-space endpoints, and a hand divide is only valid
    // in FRONT of the eye. The old code wrote `c.xy / max(abs(c.w), 1e-6)`, and that `abs` is a
    // silent catastrophe: for a vertex behind the eye plane (w < 0) it does not clip, it MIRRORS
    // the point through the screen centre. `dir` then aims at a phantom, and the quad splays off
    // across the model - a fat band running where no edge is. Nothing to do with depth, or
    // tolerance: the transform itself was invalid for those vertices.
    //
    // A fit view never shows it, because everything stays comfortably in front. Zoom in until a
    // far corner of a box passes the eye plane and every edge touching that corner breaks.
    //
    // The fix is the clip the hardware already does for real geometry (which is why the tube
    // lane never had this bug). In CLIP space `z - w` is linear along the segment and the near
    // plane is exactly `z - w = 0` - reverse-Z depth z/w = 1 - with the visible side <= 0. So
    // the crossing solves in closed form, needs no uniform, and needs to know neither the near
    // distance nor the scene scale.
    let f0 = c0.z - c0.w;
    let f1 = c1.z - c1.w;

    // Wholly nearer than the near plane: collapse to a point outside NDC so it is clipped away.
    if (f0 > 0.0 && f1 > 0.0){
        return dead_vertex();
    }

    // At most one end is outside now, so both t's are safe to compute from the ORIGINALS.
    let e0 = select(c0, mix(c0, c1, f0 / (f0 - f1)), f0 > 0.0);
    let e1 = select(c1, mix(c1, c0, f1 / (f1 - f0)), f1 > 0.0);
    let clip = select(e0, e1, at_end1);

    // Endpoints in screen pixels (one consistent mapping, used both ways). No `abs` now: after
    // the clip above, w is the near plane's own z at worst, which is positive.
    let vp = vec2<f32>(line.vp_w, line.vp_h);
    let s0 = (e0.xy / e0.w * 0.5 + 0.5) * vp;
    let s1 = (e1.xy / e1.w * 0.5 + 0.5) * vp;
    let d = s1 - s0;
    let len = length(d);
    let dir = select(vec2<f32>(1.0, 0.0), d / len, len > 1e-6);
    let n = vec2<f32>(-dir.y, dir.x);

    // Half-width in px at BOTH ends. Under perspective these differ - the near end of an edge
    // is wider - so the quad is a TRAPEZOID, and that is the whole problem this pair solves:
    // see the note on the fragment side. Both go down flat and the width is resolved per pixel.
    let raw0 = half_width_px(seg.radius, e0.w);
    let raw1 = half_width_px(seg.radius, e1.w);
    let px = floor_hairline(select(raw0, raw1, at_end1));

    // Corner in px: sideways +/- half-width, past the end by half-width (cap room),
    // +0.5px on both so the AA feather ramp fits inside the quad
    let along = select(-1.0, 1.0, at_end1);
    let p = select(s0, s1, at_end1) + (n*side+dir*along) * (px + 0.5);

    // LIFT THE INK OFF THE SURFACE IT DECORATES.
    //
    // A camera-facing rectangle through a mesh edge is correctly oriented (Easy3D builds the
    // same one: `offset = radius * normalize(cross(view_dir, axes))` in
    // lines_plain_color_width_control.geom) and still renders wrong on a solid, because at a
    // convex edge the two adjacent faces form a wedge and a PLANE through the edge cuts into
    // it on both sides. Half the quad's width is then geometrically behind a face, so the
    // interior edges of a box get eaten down to slivers while the silhouette edges - which
    // have nothing behind their outer half - survive and read as offset outward.
    //
    // No depth-compare fixes that; it is interpenetration, not a tie. What makes the
    // tessellated tube immune is that its surface is a radius PROUD of the centreline, and
    // Easy3D's answer is the same: its cylinder impostor solves the ray/cylinder analytically
    // and writes the tube surface's depth (lines_cylinders_color.frag). Both put the ink in
    // front of the wedge instead of inside it.
    //
    // So do exactly that, for free: pull the quad toward the camera by its own radius. In
    // reverse-Z, `clip.w` IS the eye depth, so scaling it shrinks the depth while `ndc * w`
    // keeps the pixel where it was - one multiply, no eye position, no frag_depth, early-Z
    // intact. `z` would strictly want +A*delta with it, but A*delta / z is ~1e-4 here
    // (near = 10x the view distance), and erring low only lifts LESS, never buries it.
    //
    // `lift` is that radius as a FRACTION of eye depth, which is what makes it unit-free:
    // px = r * proj_y * vp_h / w (see above, and screen_radius in cylinder.wgsl), so
    // r/w = px / (proj_y * vp_h) - times the mm->m scale already baked into proj_y.
    let lift = px * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h);
    let wn = clip.w * (1.0 - clamp(lift, 0.0, 0.5));

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * wn, clip.z, wn);
    o.color = unpack4x8unorm(seg.color) * instances[seg.instance_id].color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0;
    o.hw1 = raw1;
    return o;
 }

 // Depth-only prepass: the SAME capsule, but binary at half coverage and writing NOTHING to
 // colour. It lays the ink's depth down so the colour pass below (which does not write depth,
 // so its blended AA feather cannot leave halos) can be occluded by ink drawn later in the
 // same frame - a dot behind a polyline now loses to it instead of winning on draw order.
 @fragment
 fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let hf = resolve_width(in, h);
    if (clamp(hf.x + 0.5 - length(pa - ba * h), 0.0, 1.0) * hf.y < 0.5){
        discard;
    }
    return vec4<f32>(0.0); // masked out by write_mask - only depth matters
 }

 @fragment
 fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    // Capsule SDF in screen px - rounds both caps. Analytic AA: a 1px alpha ramp centered
    // on the edge, alpha-blended (a binary discard cannot be smoothed by MSAA - all 4
    // samples of a pixel live or die together), times the hairline fade.
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let d = length(pa - ba * h);
    let hf = resolve_width(in, h);
    let alpha = clamp(hf.x + 0.5 - d, 0.0, 1.0) * hf.y;
    if (alpha <= 0.0){
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
 }