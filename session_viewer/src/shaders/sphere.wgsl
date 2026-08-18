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

// Matches the Rust GlyphPoint (48 B) - order and sizes are identical.
struct GlyphPoint{
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
    // Up to SIX incident face normals (oct16 pairs), widest incident edge's two first. A trihedral
    // corner needs THREE: hugging only the widest edge's pair leaves the third face's band able
    // to bite a sector out of the disc at grazing slants, and the marker is meant to go in front.
    facing: u32,
    facing_ext: vec2<u32>,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

// Matches FACING_UNKNOWN in scene.rs. All-ones, not 0: (0,0) is the honest oct16 code for +Z.
const FACING_UNKNOWN: u32 = 0xffffffffu;
// Instance flag bit: the eye is inside THIS object's bounds (set per frame on the CPU).
const FLAG_INSIDE: u32 = 4u;

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

// Must match ribbon.wgsl; the lift is one radius more than its 3, so the marker stays the
// topmost ink at the joint it punctuates.
const MM_TO_M = 0.001;
const LIFT_RADII = 4.0;

// Sub-pixel pens never fade below this - the glyph.wgsl hairline rule, so a marker and a point
// dot agree about weight at the same width.
const HAIRLINE_MIN_ALPHA = 0.5;

// The surface-hug epsilon - the SAME constants ribbon.wgsl uses, so the marker and a band meet
// on a shared face at the same computed depth, and SPHERE_TIE breaks that tie for the marker
// (the rule LIFT_RADII = 4 used to enforce on its own). 2e-6 is ~30 ULP at depth ~1 - above the
// float disagreement between two joins of the same plane, far below anything visible.
const HUG_ABS = 1e-6;
const HUG_PIX = 1.5;
const HUG_REL = 0.35;
const SPHERE_TIE = 2e-5;
// The marker's LIFT_RADII is one radius more than the bands' 3, which wins the joint - until the
// lift CLAMP (0.5 of eye depth, close zoom) saturates: then marker and band centreline compute
// the identical depth for the shared vertex, and where no face plane applies (a back-facing
// sector), the strict-Greater marker loses its own tie to the band's cap and shows a ring bite
// out of its rim. This breaks the exact tie. It is a tie-breaker, not an offset: 2e-6 of ndc at
// scene depths is a fraction of a millimetre, and only something at the SAME depth to float
// precision - the band it punctuates - can ever be decided by it.
const MARKER_TIE = 2e-6;

// A "no plane here" value for the plane varyings: pl.z == 0 skips the depth solve entirely and
// the fragment keeps the disc's own depth (no adjacency, back-facing planes).
const PLANE_NONE = vec4<f32>(0.0, 0.0, 0.0, 0.0);

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


// DENSITY LOD, the marker half of ribbon.wgsl's WIRE_MIN_PX.
//
// A marker is screen-constant: it never shrinks with the model, so at distance a dense mesh puts
// one on every pixel several times over - 35,947 of them on a bunny 100 px tall - and the surface
// reads as speckle you can see through. That is not a depth failure and no depth fix touches it.
// The edge lane can measure its own projected length; a marker cannot, so it uses the object's
// vertex SPACING projected to pixels, which is the same quantity one step removed.
const MARKER_MIN_PX = 2.5;

// The two adjacent face normals, mesh-local. Octahedral decode: undo the fold, then normalize.
// Identical to ribbon.wgsl's - both lanes must read the same `facing` word the same way.
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

// The homogeneous JOIN of three clip-space points: the plane they span, as four signed 3x3
// minors. Same function as ribbon.wgsl's; the marker's plane is built the same way, from the
// vertex and two points stepped one radius along the face.
fn join3(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        dot(a.yzw, cross(b.yzw, c.yzw)),
        -dot(a.xzw, cross(b.xzw, c.xzw)),
        dot(a.xyw, cross(b.xyw, c.xyw)),
        -dot(a.xyz, cross(b.xyz, c.xyz)),
    );
}

fn screen_radius(clip_w: f32, u: LineUniform) -> f32{
    if (u.ortho_h > 0.0){
        return u.thickness * u.ortho_h / u.vp_h;
    }
    return u.thickness * clip_w / (u.proj_y * u.vp_h);
}

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    // Quad coordinates, |corner| = 1 on the disc rim; the fragment trims to the circle with a
    // 1px AA ramp. LINEAR: the quad is built in screen space with one w, so this is affine.
    @location(1) corner: vec2<f32>,
    @location(2) @interpolate(flat) px: f32,   // disc radius, px
    @location(3) @interpolate(flat) fade: f32, // sub-pixel opacity (hairline rule)
    // The incident FACE PLANES in clip space (join3 of three transformed points), flat, or
    // PLANE_NONE where a face is back-facing / unknown. The fragment solves the marker's depth
    // from them - see `hug_depth` and the long note in ribbon.wgsl's vs_main. Six named fields:
    // naga rejects array members in shader IO.
    @location(4) @interpolate(flat) pl0: vec4<f32>,
    @location(5) @interpolate(flat) pl1: vec4<f32>,
    @location(6) @interpolate(flat) pl2: vec4<f32>,
    @location(7) @interpolate(flat) pl3: vec4<f32>,
    @location(8) @interpolate(flat) pl4: vec4<f32>,
    @location(9) @interpolate(flat) pl5: vec4<f32>,
    // What the flat disc stands for: a BALL. The fragment adds the sphere surface's pride -
    // r*sqrt(1-d^2) toward the eye - so the marker keeps the real margin over the band caps
    // that the old tessellated sphere won with geometry (a flat disc at centre depth lost its
    // rim to the caps wherever a face grazed; measured ring ~1e-3 ndc at close zoom).
    @location(10) @interpolate(flat) zraw: f32,  // centreline ndc depth, NO lift (clip.z/clip.w)
    @location(11) @interpolate(flat) wc: f32,    // clip.w - eye depth, GPU units (m)
    @location(12) @interpolate(flat) rm: f32,    // world radius, GPU units (m)
};

fn plane_of(in: VsOut, k: u32) -> vec4<f32> {
    switch (k) {
        case 0u: { return in.pl0; }
        case 1u: { return in.pl1; }
        case 2u: { return in.pl2; }
        case 3u: { return in.pl3; }
        case 4u: { return in.pl4; }
        default: { return in.pl5; }
    }
}

// A quad collapsed outside NDC: clipped away, and even if rasterized the SDF discards it.
fn dead_dot() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.corner = vec2<f32>(0.0);
    dead.px = 0.0;
    dead.fade = 0.0;
    dead.pl0 = PLANE_NONE;
    dead.pl1 = PLANE_NONE;
    dead.pl2 = PLANE_NONE;
    dead.pl3 = PLANE_NONE;
    dead.pl4 = PLANE_NONE;
    dead.pl5 = PLANE_NONE;
    dead.zraw = 0.0;
    dead.wc = 1.0;
    dead.rm = 0.0;
    return dead;
}

@vertex
fn vs_main(@location(0) tmpl: vec3<f32>, @builtin(instance_index) gi: u32) -> VsOut{
    let g = glyphs[gi];
    let model = instances[g.instance_id].model;

    // centre only - radius is scale-invariant
    let centre = (model * vec4<f32>(g.center, 1.0)).xyz;
    let clip = mvp * vec4<f32>(centre, 1.0);

    // NEAR-PLANE CLIP, the ribbon.wgsl rule: this lane projects by hand and a hand divide is only
    // valid in FRONT of the eye. A marker centre nearer than the near plane (z - w > 0 in
    // reverse-Z) would divide by a near-zero w and splay the quad across the screen.
    if (clip.z - clip.w > 0.0) {
        return dead_dot();
    }

    // Handles read a touch larger than lines (x30 a); a world-mm radius (>0) overrides.
    // `r` stays in WORLD units for the plane steps below; `px` is the same radius on screen.
    let mult = select(1.0, -g.radius, g.radius < 0.0);
    let r = select(screen_radius(clip.w, line) * mult, g.radius, g.radius > 0.0);
    var px: f32;
    if (line.ortho_h > 0.0) {
        px = r * line.vp_h * 0.5 / line.ortho_h;
    } else {
        px = r * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6);
    }

    // Below the density threshold this marker is noise, not information - see MARKER_MIN_PX.
    // Projected the same way a world radius is (half_width_px in ribbon.wgsl): spacing is a world
    // length, so it lands in px as `s * proj_y * vp_h / (2 * w)`.
    let sp = instances[g.instance_id].spacing;
    if (sp > 0.0 && line.ortho_h <= 0.0
        && sp * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6) < MARKER_MIN_PX) {
        return dead_dot();
    }

    // NEAR-EYE CAP, the glyph.wgsl rule: a marker whose radius alone exceeds the viewport is a
    // ball millimetres from the eye, not a marker - its correctly projected rim would land on
    // screen as a soft screen-wide arc. Drop it; the near-plane clip above takes it from there.
    if (px > max(line.vp_w, line.vp_h)) {
        return dead_dot();
    }

    // Hairline rule, the glyph.wgsl floor: never thinner than 1px, carry the deficit into alpha.
    var fade = 1.0;
    if (px < 0.5) {
        fade = max(px / 0.5, HAIRLINE_MIN_ALPHA);
        px = 0.5;
    }

    // Camera-facing quad: screen-space offset per corner, +0.5px so the AA feather fits inside.
    // `wn` scales w by the lift while ndc*w holds the pixel still - one radius MORE than the
    // bands (4 vs 3), so the marker floats at least a radius in front of the centreline ink and
    // stays the topmost mark where no plane applies (the silhouette side).
    let corner = tmpl.xy;
    let lift = px * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h);
    let wn = clip.w * (1.0 - lift_capped(lift, clip.w, instances[g.instance_id].extent));
    let off = corner * (px + 0.5) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;

    // THE ADJACENT FACES ARE PLANES - the ribbon.wgsl argument, applied to the marker. The bands
    // meeting at this vertex hug the faces at face+eps; a marker floating on the constant lift
    // alone loses the depth argument to its own bands over most of its disc wherever a face
    // grazes the eye, and shows up as a lopsided chunk smaller than the band width. Riding the
    // same planes puts the marker ON the surface the bands are on.
    //
    // The marker builds its planes DIRECTLY from the normal - no point triple. A join of three
    // points r apart (millimetres at close zoom) cancels f32 mantissa bits against clip-space
    // magnitudes and the solved depth drifts ~1e-3 ndc from the band's own join of the SAME
    // plane; measured, the marker then lost its rim to the band caps. Instead: world plane
    // (n, -n.centre) transformed to clip space by the inverse-transpose, whose entries are the
    // cofactors of mvp - join3 of mvp's ROWS (the 4D cross product), signs alternating, the
    // determinant dividing out in the fragment's solve. Only O(1) matrix entries multiply, so
    // nothing cancels, and the result agrees with the bands' planes to float noise.
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - centre;
    // From inside the solid every face points away; the back-facing cull's premise is void and
    // every plane hugs, exactly like the ribbon lane's FLAG_INSIDE rule.
    let inside = (instances[g.instance_id].flags & FLAG_INSIDE) != 0u;
    // mvp rows (WGSL indexes columns); j_k = cofactor row k = +/- join of the other three rows.
    let r0 = vec4<f32>(mvp[0].x, mvp[1].x, mvp[2].x, mvp[3].x);
    let r1 = vec4<f32>(mvp[0].y, mvp[1].y, mvp[2].y, mvp[3].y);
    let r2 = vec4<f32>(mvp[0].z, mvp[1].z, mvp[2].z, mvp[3].z);
    let r3 = vec4<f32>(mvp[0].w, mvp[1].w, mvp[2].w, mvp[3].w);
    let j0 = join3(r1, r2, r3);
    let j1 = join3(r0, r2, r3);
    let j2 = join3(r0, r1, r3);
    let j3 = join3(r0, r1, r2);
    var pl: array<vec4<f32>, 6>;
    for (var k = 0u; k < 6u; k = k + 1u) { pl[k] = PLANE_NONE; }
    let fwords = array<u32, 3>(g.facing, g.facing_ext.x, g.facing_ext.y);
    var known = false;   // this vertex carries adjacency at all
    var front = false;   // ...and at least one incident face turns toward the eye
    for (var w = 0u; w < 3u; w = w + 1u) {
        let fw = fwords[w];
        if (fw == FACING_UNKNOWN) { continue; }
        known = true;
        for (var h = 0u; h < 2u; h = h + 1u) {
            let n = (model * vec4<f32>(oct16_decode((fw >> (16u * h)) & 0xffffu), 0.0)).xyz;
            if (dot(n, to_eye) > 0.0) { front = true; }
            if (inside || dot(n, to_eye) > 0.0) {
                let pw = vec4<f32>(n, -dot(n, centre));
                pl[2u * w + h] = vec4<f32>(dot(j0, pw), -dot(j1, pw), dot(j2, pw), -dot(j3, pw));
            }
        }
    }

    // HIDDEN VERTICES NEVER REACH THE RASTERIZER - the ribbon lane's cull, which this lane was
    // missing. Every incident face turned away means the vertex is on the far side of the solid,
    // and a marker there is not merely redundant: it floats 4 radii toward the camera and then
    // HUGS a plane, so on a dense curved mesh it pokes through the near surface and the model
    // reads as though its back vertices were showing through. The bunny emits one marker per
    // vertex - 35,947 of them, about half on the far side - which is where this became obvious.
    //
    // `known == false` is geometry with no adjacency at all (free-standing points, a drawing's
    // dots): those always draw. `inside` keeps the whole object when the eye is within its
    // bounds, for the same reason the bands do - from inside, every face points away.
    if (known && !front && !inside) {
        return dead_dot();
    }

    var o: VsOut;
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z, wn);
    o.color = g.color * instances[g.instance_id].color;
    o.corner = corner;
    o.px = px;
    o.fade = fade;
    o.pl0 = pl[0];
    o.pl1 = pl[1];
    o.pl2 = pl[2];
    o.pl3 = pl[3];
    o.pl4 = pl[4];
    o.pl5 = pl[5];
    o.zraw = clip.z / clip.w;
    o.wc = clip.w;
    o.rm = r * MM_TO_M;
    return o;
}

// The depth the fragment WRITES: the marker's lifted centreline depth, unless a front-facing
// incident plane rises ABOVE it here - then the plane, one epsilon in front, exactly as
// ribbon.wgsl's ink_depth does for the bands. The marker then meets the bands on their shared
// faces at the same computed depth (+ SPHERE_TIE, so the strict-Greater test tips the joint to
// the marker). Anything genuinely nearer still wins: a back-corner marker stays hidden.
fn hug_depth(in: VsOut) -> f32 {
    // The ball this disc stands for: at d_frac of the radius out from the centre, the sphere
    // surface is proud of the vertex by rm*sqrt(1-d^2), which is the margin that wins the rim
    // against the band caps. The LIFT_RADII margin rides on top, all as a fraction of eye
    // depth - the same closed form the vertex-stage lift uses.
    let d_px = length(in.corner) * (in.px + 0.5);
    let d_frac = min(d_px / max(in.px, 1e-6), 1.0);
    let pride = in.rm * sqrt(max(1.0 - d_frac * d_frac, 0.0));
    let lift = in.px * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h);
    let pull = lift + pride / in.wc;
    var z = in.zraw / (1.0 - min(pull, 0.5)) + MARKER_TIE;
    // The eps REL term must reference the BAND's base depth at this vertex, not the marker's:
    // the bands lift 3 radii and the marker 4, so against a rising plane the band's |zp - z|
    // is larger by 35% of the margin (HUG_REL) - enough to tip the rim race to the band caps
    // (measured ring at close zoom). With the reference equalized the eps is identical on both
    // sides and SPHERE_TIE cleanly wins the tie for the marker. Ribbon lane untouched.
    let lift3 = in.px * 3.0 * MM_TO_M / (line.proj_y * line.vp_h);
    let z_band = in.zraw / (1.0 - min(lift3, 0.5));
    // This fragment's ndc from the framebuffer position: px are y-down, ndc is y-up.
    let ndc = vec2<f32>((in.pos.x / line.vp_w - 0.5) * 2.0, (0.5 - in.pos.y / line.vp_h) * 2.0);
    for (var k = 0u; k < 6u; k = k + 1u) {
        let pl = plane_of(in, k);
        // pl.z == 0 is PLANE_NONE, and also a genuinely edge-on plane, whose depth at the pixel
        // is meaningless: fall back to the marker's own depth for both.
        if (pl.z != 0.0) {
            let zp = clamp(-(pl.x * ndc.x + pl.y * ndc.y + pl.w) / pl.z, -1e9, 1.0);
            let slope_px = (abs(pl.x) / line.vp_w + abs(pl.y) / line.vp_h) * 2.0 / abs(pl.z);
            // The band's REL term references ITS centreline depth AT THIS PIXEL, and this pixel
            // is up to one marker radius away from the vertex along the band, where the centreline
            // has moved by the plane's own screen slope times that distance. Bounding it with
            // `slope_px * (px + 0.5)` makes the marker's eps at least the band's everywhere on the
            // disc - a derived bound, not a tuned margin, so it scales with zoom and slant instead
            // of needing a bigger constant every time a case is found.
            let band_span = slope_px * (in.px + 0.5);
            let eps = HUG_ABS + HUG_PIX * slope_px
                + HUG_REL * (abs(zp - z_band) + band_span) + SPHERE_TIE;
            z = max(z, min(zp + eps, 1.0));
        }
    }
    return clamp(z, 0.0, 1.0);
}

struct FsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

@fragment
fn fs_main(in: VsOut) -> FsOut{
    // Circle SDF in px (|corner| = 1 is the rim), 1px AA ramp alpha-blended, times the hairline
    // fade - the exact look of the point dots in glyph.wgsl. A binary discard cannot be smoothed
    // by MSAA (all 4 samples of a pixel live or die together), so the rim is blended.
    let d = length(in.corner) * (in.px + 0.5);
    let alpha = clamp(in.px + 0.5 - d, 0.0, 1.0) * in.fade;
    if (alpha <= 0.0) { discard; }
    var o: FsOut;
    o.color = vec4<f32>(in.color.rgb, in.color.a * alpha);
    o.depth = hug_depth(in);
    return o;
}
