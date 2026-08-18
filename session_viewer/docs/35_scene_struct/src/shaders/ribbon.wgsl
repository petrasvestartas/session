@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    extent: f32,   // world AABB diagonal; 0 = unknown. Caps the ink lift - see lift_capped().
    spacing: f32,  // typical vertex spacing, world units; 0 = unknown. Density LOD - see below.
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust CylinderSegment, 40 B. The ends are SCALARS, not vec3<f32>: WGSL aligns
// vec3<f32> to 16, which padded this struct to 48 and made the 40 impossible.
// Matches FACING_UNKNOWN in scene.rs.
const FACING_UNKNOWN: u32 = 0xffffffffu;
// Instance flag bit: the eye is inside THIS object's bounds (set per frame on the CPU).
const FLAG_INSIDE: u32 = 4u;

struct CylinderSegment{
    p0x: f32, p0y: f32, p0z: f32,
    radius: f32,
    p1x: f32, p1y: f32, p1z: f32,
    instance_id: u32,
    color: u32,   // RGBA8, low byte red
    facing: u32,  // two oct16 adjacent face normals; FACING_UNKNOWN = no adjacency, always draw.
                  // 0 is NOT the sentinel: it is the honest oct16 code for a +Z/+Z face pair.
}

// The two adjacent face normals, mesh-local. Octahedral decode: undo the fold, then normalize.
fn oct16_decode(p: u32) -> vec3<f32> {
    let e = vec2<f32>(
        f32(i32(p << 24u) >> 24u) / 127.0,
        f32(i32(p << 16u) >> 24u) / 127.0,
    );
    var n = vec3<f32>(e, 1.0 - abs(e.x) - abs(e.y));
    if (n.z < 0.0){
        // signNotZero, matching the encoder: WGSL `sign(0.0)` is 0.0, and using it here folds the
        // -Z pole onto the +Z code instead of back onto -Z.
        let s = vec2<f32>(select(1.0, -1.0, n.x < 0.0), select(1.0, -1.0, n.y < 0.0));
        n = vec3<f32>((1.0 - abs(n.y)) * s.x, (1.0 - abs(n.x)) * s.y, n.z);
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
// FACING_UNKNOWN means the geometry never had adjacency - free-standing linework, drawing pens,
// BRep edges - and those always draw. It is all-ones rather than 0 because (0,0) is the honest
// octahedral code for +Z, and a box's top face is exactly that.
//
// Takes the two normals ALREADY decoded and rotated to world: vs_main needs them a second time
// to build the face planes, and decoding once keeps the two uses consistent.
fn edge_faces_camera(facing: u32, n0: vec3<f32>, n1: vec3<f32>, to_eye: vec3<f32>) -> bool {
    if (facing == FACING_UNKNOWN){
        return true;
    }
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

// The surface-hug epsilon (see `ink_depth`). ABS covers the float disagreement between the
// plane solve here and the face rasterizer's own depth for the same surface (a few ULP at
// depth ~1). PIX scales with the plane's own screen slope: under 4x MSAA the ink's depth is
// solved once at the pixel centre while the face holds a depth PER SAMPLE, so the eps must
// span the slope over the sample spread (sub-pixel) - exactly glPolygonOffset's slope term,
// but computed in closed form from the plane instead of from a state's magic constant.
// REL is a fraction of the plane's local RISE across the band and covers the oct16
// normals' ~1.4 degree quantization: tilting the plane by d changes its rise by
// d*secant^2(theta), i.e. 2d/sin(2theta) AS A FRACTION of the rise - under ~30% for any face
// less than ~85 degrees to the view ray, which is why the value below is what it is. Neither
// constant is the old failure mode: they move only the INK, never a face, so two faces meeting
// at an edge cannot be made to fight over a pen-width band by them.
const HUG_ABS = 1e-6;
const HUG_PIX = 1.5;
const HUG_REL = 0.35;

// A "no plane here" value for the plane varyings: pl.z == 0 skips the depth solve entirely and
// the fragment keeps the centreline depth (dead vertices, no adjacency, back-facing planes).
const PLANE_NONE = vec4<f32>(0.0, 0.0, 0.0, 0.0);

// DENSITY LOD: a wire shorter than this many pixels is dropped.
//
// At distance a dense mesh has more wires than the screen has pixels - the bunny is 104,288 edges
// and 35,947 markers, and at 100 px tall that is ink on every pixel several times over. The
// surface then reads as speckle you can see through, which is not a depth failure and no depth
// fix touches it: it is that a 2px screen-constant pen does not shrink with the model. Rhino and
// every CAD viewer stop drawing wires below a density like this.
//
// Measured against the projected length of the edge itself, so it needs no per-object data and
// costs one compare: an edge whose two ends land within a few pixels cannot communicate a
// direction, only noise. Free-standing linework is exempt - a short polyline segment is still a
// real line the user drew, and drawings are full of them - so this applies to geometry that HAS
// adjacency, which is exactly mesh and BRep wireframe.
const WIRE_MIN_PX = 2.5;

// The ink lift, CAPPED so it can never lift ink in front of the object it belongs to.
//
// `lift` here is a fraction of EYE DEPTH, which is what makes it unit-free and correct at any
// zoom - but it is also why it cannot be left alone. World lift = lift * eye_depth, so it grows
// with camera distance while an object's front-to-back size does not. Past some distance the
// object's BACK wireframe is lifted in front of its own front faces and the model goes
// see-through: measured on a 1000 mm box with a 2px pen, 242 m for a band and 91 m for a marker,
// which is ordinary zoom-out in a scene spanning tens of metres. Zoom in and it cannot happen;
// zoom out and it must.
//
// The cap is a fraction of the object's own world AABB diagonal, which the CPU already computes
// for FLAG_INSIDE and now ships per instance row. A tenth of the diagonal is far more than the
// lift ever needs when the pen is small against the object (the case the lift was tuned for) and
// bites only in the regime where the lift had stopped meaning "just in front of the surface".
// `extent == 0` is unknown (linework, clouds): no cap, since there is no object to punch through.
const LIFT_MAX_EXTENT = 0.1;

fn lift_capped(lift: f32, w: f32, extent: f32) -> f32 {
    if (extent <= 0.0) {
        return clamp(lift, 0.0, 0.5);
    }
    // extent is world units (mm); w is eye depth in metres, so world lift = lift * w / MM_TO_M.
    let max_lift = LIFT_MAX_EXTENT * extent * MM_TO_M / max(w, 1e-9);
    return clamp(min(lift, max_lift), 0.0, 0.5);
}

// One end's lifted w: the LIFT_RADII pull toward the camera, as a fraction of eye depth, so it
// is unit-free. `clip.w` IS the eye depth in this projection; scaling it shrinks the depth
// while `ndc * w` keeps the pixel where it was. Done PER END because the two ends project to
// different widths under perspective, and the fragment needs both ends' depths to reproduce
// the centreline depth exactly (see `zend`).
fn lifted_w(raw_px: f32, e: vec4<f32>, extent: f32) -> f32 {
    let lift = floor_hairline(raw_px) * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h);
    return e.w * (1.0 - lift_capped(lift, e.w, extent));
}

// The homogeneous JOIN of three clip-space points: the plane they span, as four signed 3x3
// minors (each a dot with a cross). No matrix inverse and no normalize - the fragment's depth
// solve divides the overall scale back out. Verified: join of (0,0,0,1),(1,0,0,1),(0,1,0,1)
// is (0,0,1,0), the z=0 plane.
fn join3(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        dot(a.yzw, cross(b.yzw, c.yzw)),
        -dot(a.xzw, cross(b.xzw, c.xzw)),
        dot(a.xyw, cross(b.xyw, c.xyw)),
        -dot(a.xyz, cross(b.xyz, c.xyz)),
    );
}

// HALF-width in px at one end: half the global pen thickness, or a world radius projected.
//
// The two 0.5s are the bug this lane shipped with, and they are the same bug twice. NDC spans
// [-1, 1] across vp_h pixels, so ONE NDC unit is vp_h/2 px, not vp_h:
//
//     y_ndc = (y_eye / d) * cot(fovy/2)        px = y_ndc * vp_h/2 = y*cot*vp_h / (2*d)
//
// and `proj_y` is cot(fovy/2) with the mm->m scale folded in, so a world radius projects to
// `r * proj_y * vp_h / (2*w)` px. Without the 2 every ribbon drew at twice its pen. `thickness`
// is documented as an on-screen WIDTH in px (cylinder.wgsl's screen_radius reads it that way and
// is correct), so as a HALF-width it needs halving too.
//
// The reference is the tube lane, which is real geometry of radius r and cannot be argued with:
// before this, a mesh edge measured 8 px flat against 4 px as a tube.
fn half_width_px(radius: f32, w: f32) -> f32 {
    if (radius > 0.0){
        if (line.ortho_h > 0.0){
            return radius * line.vp_h * 0.5 / line.ortho_h;
        }
        return radius * line.proj_y * line.vp_h * 0.5 / w;
    }
    return line.thickness * 0.5 * select(1.0, -radius, radius < 0.0);
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
    // The two adjacent FACE PLANES in clip space (join3 of three transformed points), flat, or
    // PLANE_NONE where a face is back-facing / unknown. The fragment solves the band's depth
    // from them - see `ink_depth`.
    @location(6) @interpolate(flat) pl0: vec4<f32>,
    @location(7) @interpolate(flat) pl1: vec4<f32>,
    // Centreline depth (z/w, lift included) at each clipped end, flat. Screen-linear in between,
    // which is exactly how the rasterizer would have interpolated it - see `ink_depth`.
    @location(8) @interpolate(flat) zend: vec2<f32>,
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
    dead.pl0 = PLANE_NONE;
    dead.pl1 = PLANE_NONE;
    dead.zend = vec2<f32>(0.0);
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
    let mid = (w0 + w1) * 0.5;
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - mid;

    // The two adjacent face normals, decoded once and rotated into world with the model's linear
    // part (non-uniform scale would strictly want the inverse transpose, but placements here are
    // rigid or uniform). The facing cull below needs their sign against the eye; the face planes
    // further down are built from the same two vectors.
    let n0 = (model * vec4<f32>(oct16_decode(seg.facing & 0xffffu), 0.0)).xyz;
    let n1 = (model * vec4<f32>(oct16_decode(seg.facing >> 16u), 0.0)).xyz;

    // HIDDEN EDGES NEVER REACH THE RASTERIZER. Both adjacent faces turned away means this edge is
    // inside the solid; drop it here and the ink that remains never has to win a depth argument
    // with the surface it decorates. Collapsed outside NDC so it is clipped, like the near-plane
    // case below.
    //
    // ...unless the eye is INSIDE this object's bounds (FLAG_INSIDE, set per frame on the CPU):
    // from inside a solid EVERY face points away by definition, so the cull would drop the whole
    // object the moment the camera crosses a face. The per-edge test cannot tell "far side of the
    // solid" from "eye inside it" - that difference is global, which is why the flag rides the
    // instance row.
    let inside = (instances[seg.instance_id].flags & FLAG_INSIDE) != 0u;
    if (!inside && !edge_faces_camera(seg.facing, n0, n1, to_eye)){
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

    // Below the density threshold this wire is noise, not information - see WIRE_MIN_PX.
    if (seg.facing != FACING_UNKNOWN && len < WIRE_MIN_PX){
        return dead_vertex();
    }

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
    //
    // The lift alone cannot win against a GRAZING face (the face's rise across the band is
    // d*tan(theta), unbounded), which is what the planes below are for; it stays because the
    // centreline depth is still the answer everywhere a plane does not apply, and the verified
    // close-zoom behavior rides on it.
    let wn0 = lifted_w(raw0, e0, instances[seg.instance_id].extent);
    let wn1 = lifted_w(raw1, e1, instances[seg.instance_id].extent);
    let wn = select(wn0, wn1, at_end1);

    // THE ADJACENT FACES ARE PLANES, and a plane's depth at any pixel is closed form. Instead of
    // floating the band a guessed constant in front of the surface it decorates (any constant
    // loses where d*tan(theta) exceeds it, and making the FACES move by that much just makes two
    // faces fight over a band of the same width), the band can RIDE the surface: write, per
    // fragment, the depth of whichever front-facing adjacent plane is nearer there, one epsilon
    // in front. The planes are built in CLIP space as the join of three transformed points - the
    // two endpoints (both lie on both faces) plus a third stepped one edge-length sideways off
    // the midpoint along the face - so no matrix inverse is needed, and near-plane clipping is
    // irrelevant to them: a clip-space point with w < 0 is still algebraically on the plane.
    //
    // A face turned AWAY gets PLANE_NONE: past the silhouette its plane continues through space
    // that is not the object, and there the honest depth is the edge's own. The max() in
    // ink_depth never lets a plane pull the ink BEHIND the centreline, so silhouette edges keep
    // their full pen width - the outer half stays at centreline depth. EXCEPT when the eye is
    // inside the object: from inside, the back-facing faces ARE the visible surface (they rise
    // toward the eye across the band exactly like a grazing face does from outside), so both
    // planes hug.
    var pl0 = PLANE_NONE;
    var pl1 = PLANE_NONE;
    if (seg.facing != FACING_UNKNOWN) {
        let edge = w1 - w0;
        let elen = length(edge);
        if (elen > 1e-12) {
            let edir = edge / elen;
            // Face normals are perpendicular to the edge (the face CONTAINS it), so
            // cross(n, edir) is an in-plane direction of about unit length.
            if (inside || dot(n0, to_eye) > 0.0) {
                let t0 = mvp * vec4<f32>(mid + cross(n0, edir) * elen, 1.0);
                pl0 = join3(c0, c1, t0);
            }
            if (inside || dot(n1, to_eye) > 0.0) {
                let t1 = mvp * vec4<f32>(mid + cross(n1, edir) * elen, 1.0);
                pl1 = join3(c0, c1, t1);
            }
        }
    }

    var o: VsOut;
    let ndc = (p / vp - 0.5) * 2.0;
    o.pos = vec4<f32>(ndc * wn, clip.z, wn);
    o.color = unpack4x8unorm(seg.color) * instances[seg.instance_id].color;
    o.p = p;
    o.a = s0;
    o.b = s1;
    o.hw0 = raw0;
    o.hw1 = raw1;
    o.pl0 = pl0;
    o.pl1 = pl1;
    o.zend = vec2<f32>(e0.z / wn0, e1.z / wn1);
    return o;
 }

 // The depth the fragment WRITES. The band's geometrically correct depth is its centreline's,
 // but the test is per fragment, and at a screen distance d from the centreline an adjacent
 // face has already risen toward the eye by d*tan(theta) - unbounded as the face turns
 // edge-on, which is exactly where a wide world-mm pen got eaten. The face is a PLANE, though,
 // and a clip-space plane's depth at the fragment's own ndc is one division:
 //
 //     pl.x*nx + pl.y*ny + pl.z*nz + pl.w = 0   =>   nz = -(pl.x*nx + pl.y*ny + pl.w) / pl.z
 //
 // So: keep the centreline depth, unless a front-facing adjacent plane rises ABOVE it here -
 // then take the plane, one epsilon in front. The ink hugs the surface it decorates instead of
 // floating a guessed distance in front of it, no offset constant is chosen anywhere (HUG_* is
 // a numeric guard, not a geometric offset), and occlusion by other geometry still works
 // because this is an ordinary depth value: anything genuinely nearer than the surface still
 // wins. A plane can never pull the ink behind the centreline (max), so silhouette outer halves
 // keep the full pen width.
 fn ink_depth(in: VsOut, h: f32) -> f32 {
    var z = mix(in.zend.x, in.zend.y, h);
    // The fragment's ndc, inverted from the same mapping the vertex stage used to build `p`.
    let ndc = (in.p / vec2<f32>(line.vp_w, line.vp_h) - 0.5) * 2.0;
    for (var k = 0u; k < 2u; k = k + 1u) {
        let pl = select(in.pl0, in.pl1, k == 1u);
        // pl.z == 0 is PLANE_NONE, and also a genuinely edge-on plane, whose depth at the
        // pixel is meaningless: fall back to the centreline for both.
        if (pl.z != 0.0) {
            let zp = clamp(-(pl.x * ndc.x + pl.y * ndc.y + pl.w) / pl.z, -1e9, 1.0);
            // One epsilon in front of the surface - see HUG_ABS / HUG_PIX / HUG_REL at the top.
            // The slope term is the plane's ndc-z change over one screen pixel.
            let slope_px = (abs(pl.x) / line.vp_w + abs(pl.y) / line.vp_h) * 2.0 / abs(pl.z);
            let eps = HUG_ABS + HUG_PIX * slope_px + HUG_REL * abs(zp - z);
            z = max(z, min(zp + eps, 1.0));
        }
    }
    return clamp(z, 0.0, 1.0);
 }

 struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
 };

 // Depth-only prepass: the SAME capsule, but binary at half coverage and writing NOTHING to
 // colour. It lays the ink's depth down so the colour pass below (which does not write depth,
 // so its blended AA feather cannot leave halos) can be occluded by ink drawn later in the
 // same frame - a dot behind a polyline now loses to it instead of winning on draw order.
 @fragment
 fn fs_depth(in: VsOut) -> FsOut {
    let pa = in.p - in.a;
    let ba = in.b - in.a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    let hf = resolve_width(in, h);
    if (clamp(hf.x + 0.5 - length(pa - ba * h), 0.0, 1.0) * hf.y < 0.5){
        discard;
    }
    var o: FsOut;
    o.color = vec4<f32>(0.0); // masked out by write_mask - only depth matters
    o.depth = ink_depth(in, h);
    return o;
 }

 @fragment
 fn fs_main(in: VsOut) -> FsOut{
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
    var o: FsOut;
    o.color = vec4<f32>(in.color.rgb, in.color.a * alpha);
    o.depth = ink_depth(in, h);
    return o;
 }
