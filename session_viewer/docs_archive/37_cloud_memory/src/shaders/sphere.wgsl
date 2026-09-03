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
// Instance flag bit: the mesh is OPEN (boundary edges) - not a solid, so the facing cull's
// premise is void and it is skipped exactly like FLAG_INSIDE (see Instance::FLAG_OPEN).
const FLAG_OPEN: u32 = 16u;

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

// Must match ribbon.wgsl's MM_TO_M; LIFT_RADII is TWO radii more than the bands' 3, and the
// margin is load-bearing, not cosmetic. The disc rides fixed-function depth at its centre's
// lifted value, flat across the whole footprint - but a band running TOWARD the eye from the
// vertex gets nearer along its length, by at most one radius of eye depth within one disc
// radius of the vertex. At 4 (one radius of margin) that worst case TIES and steeper spans
// clipped the disc - the "dot half-covered by its own edge" artifact on the cube corners.
// 5 gives band-max (3 + 1) + 1 radius of true clearance across the disc. A band from a
// DIFFERENT, genuinely nearer object still wins by its real depth advantage.
const MM_TO_M = 0.001;
const LIFT_RADII = 5.0;

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
// The marker half of ribbon.wgsl's WIRE_MIN_PENS, in the same units: a marker is dropped when the
// object's vertex spacing is under this many times the marker's OWN DIAMETER. Same reason - what
// matters is room between marks, not whether one mark is visible.
const MARKER_MIN_DIAMS = 3.0;
const TAPER_MIN = 0.15;   // a marker never thins past this fraction of its radius

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
};

// A quad collapsed outside NDC: clipped away, and even if rasterized the SDF discards it.
fn dead_dot() -> VsOut {
    var dead: VsOut;
    dead.pos = vec4<f32>(3.0, 3.0, 0.5, 1.0);
    dead.color = vec4<f32>(0.0);
    dead.corner = vec2<f32>(0.0);
    dead.px = 0.0;
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
    // Density taper, not a cull - the markers thin as their object's vertices crowd together, and
    // the hairline rule below carries the remainder into alpha. A vertex always keeps a mark.
    let sp = instances[g.instance_id].spacing;
    if (sp > 0.0) {
        // Same projection as the radius above: perspective by eye depth, ortho by half-height.
        var sp_px: f32;
        if (line.ortho_h > 0.0) {
            sp_px = sp * line.vp_h * 0.5 / line.ortho_h;
        } else {
            sp_px = sp * line.proj_y * line.vp_h * 0.5 / max(clip.w, 1e-6);
        }
        px = px * clamp(sp_px / max(MARKER_MIN_DIAMS * 2.0 * px, 1e-6), TAPER_MIN, 1.0);
    }

    // NEAR-EYE CAP, the glyph.wgsl rule: a marker whose radius alone exceeds the viewport is a
    // ball millimetres from the eye, not a marker - its correctly projected rim would land on
    // screen as a soft screen-wide arc. Drop it; the near-plane clip above takes it from there.
    if (px > max(line.vp_w, line.vp_h)) {
        return dead_dot();
    }

    // Hairline floor, but NO alpha fade: every row in this lane is a mesh/BRep vertex, and this
    // pipeline alpha-blends UNDER a depth write - semi-transparent marks composed through a depth
    // buffer resolve by draw-order luck and read as per-pixel noise on a dense mesh at distance
    // (the ribbon lane's resolve_width has the full argument). A sub-pixel marker draws as a 1px
    // OPAQUE dot, exactly what the tube lane's real sphere geometry resolves to. The free-standing
    // point dots are glyph.wgsl rows and keep their fade.
    px = max(px, 0.5);

    // Camera-facing quad: screen-space offset per corner, +0.5px so the AA feather fits inside.
    // `wn` scales w by the lift while ndc*w holds the pixel still - one radius MORE than the
    // bands (4 vs 3), so the marker floats at least a radius in front of the centreline ink and
    // stays the topmost mark where no plane applies (the silhouette side).
    let corner = tmpl.xy;
    // Ortho carries no eye depth in w, so the w-scale would collapse into a constant ndc offset
    // that outgrows the scene's whole depth span on zoom-out (the full argument is at
    // ribbon.wgsl's ortho_lift_ndc): lift in ndc instead - LIFT_RADII world radii, the same
    // LIFT_MAX_EXTENT cap, through the mvp's z row (ozn, also handed to hug_depth, 0 = persp).
    let ozn = select(0.0, length(vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z)), line.ortho_h > 0.0);
    let lift = px * LIFT_RADII * MM_TO_M / (line.proj_y * line.vp_h);
    var wn = clip.w * (1.0 - lift_capped(lift, clip.w, instances[g.instance_id].extent));
    var zlift = 0.0;
    if (line.ortho_h > 0.0) {
        wn = clip.w;
        let lw = px * LIFT_RADII * 2.0 * line.ortho_h / line.vp_h; // world units
        let ext = instances[g.instance_id].extent;
        zlift = min(lw, select(1e30, LIFT_MAX_EXTENT * ext, ext > 0.0)) * ozn;
    }
    let off = corner * (px + 0.5) * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * wn;

    // FACING, but no planes: the marker's depth is fully decided in this stage (the lifted
    // centre the rasterizer interpolates), so the adjacency words are only read for the
    // hidden-vertex cull below. No frag_depth downstream - early-Z stays alive, which is what
    // makes this lane cheap (see the ribbon.wgsl note at its vs_main tail).
    let to_eye = vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - centre;
    // From inside the solid every face points away; the cull's premise is void, like the bands.
    let inside = (instances[g.instance_id].flags & (FLAG_INSIDE | FLAG_OPEN)) != 0u;
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
    o.pos = vec4<f32>(clip.xy / clip.w * wn + off, clip.z + zlift * wn, wn);
    o.color = g.color * instances[g.instance_id].color;
    o.corner = corner;
    o.px = px;
    return o;
}

// Depth-only prepass: the SAME disc, binary at half coverage, colour masked out by the
// pipeline. It lays the marker's depth down so the blended colour pass below (which does NOT
// write depth - a semi-transparent rim pixel that wrote depth would reject the next stroke's
// opaque core and leave a pale fleck) still occludes and is occluded correctly.
@fragment
fn fs_depth(in: VsOut) -> @location(0) vec4<f32> {
    let d = length(in.corner) * (in.px + 0.5);
    if (clamp(in.px + 0.5 - d, 0.0, 1.0) < 0.5) { discard; }
    return vec4<f32>(0.0);
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    // Circle SDF in px (|corner| = 1 is the rim), 1px AA ramp alpha-blended - the exact look of
    // the point dots in glyph.wgsl. A binary discard cannot be smoothed by MSAA (all 4 samples
    // of a pixel live or die together), so the rim is blended. Depth is the rasterizer's own:
    // the vertex stage lifted the disc one radius MORE than the bands (4 vs 3), which is what
    // wins the joint against the band caps now that both lanes ride fixed-function depth.
    let d = length(in.corner) * (in.px + 0.5);
    let alpha = clamp(in.px + 0.5 - d, 0.0, 1.0);
    if (alpha <= 0.0) { discard; }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
