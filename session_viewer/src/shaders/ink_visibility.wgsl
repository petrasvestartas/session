// Shared by the ink lanes. Visibility comes from the physical depth alone, read as a
// piecewise-planar surface: a fragment fits the plane of the surface under it from its own
// texel and the next one AWAY from the stroke, and is on its surface when that plane passes
// through the stroke's axis. Otherwise only something nearer than the axis hides it.
// Reverse-Z: nearer is greater, 0 is cleared. Pixels are framebuffer coordinates, y down.
@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d;

override SCENE_MSAA: bool = false;

// 2^-19: about 16 ULPs of a float, relative to the depth.
const DEPTH_REL_TOL: f32 = 1.9073486e-6;
// The rasterizer snaps vertices to 1/256 px, so a fitted plane's depth is off by its slope
// times that; 2^-8 is exactly that quantisation, with the headroom measured away: the close-up
// holds at 242406 non-background pixels and the probe matrix at 54 cases with its nine
// distance-1 counts unchanged, while the floor census residual falls from 29 to 13 samples.
const SLOPE_PX: f32 = 0.00390625;

struct InkColor {
    @location(0) color: vec4<f32>,
};

// The stroke as one fragment sees it: the closest axis point, its depth, the unit screen
// direction of the stroke and the axis's depth change per pixel along it.
struct InkAxis {
    at: vec2<f32>,
    depth: f32,
    along: vec2<f32>,
    slope: f32,
};

// A texel's physical depth at one sample; outside the viewport counts as cleared.
fn ink_depth(pixel: vec2<f32>, sample: u32) -> f32 {
    if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
        return 0.0;
    }
    let at = vec2<i32>(pixel);
    if (SCENE_MSAA) {
        return textureLoad(scene_depth_msaa, at, i32(sample));
    }
    return textureLoad(scene_depth_single, at, 0);
}

// How far a fitted plane may miss: float precision plus the slope's snapping error over the
// lever arm it was extrapolated across.
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}

// Whether the fragment and its neighbour one texel along `dir` lie on one surface, so that
// pair may be fitted: on a plane the next texel out extends the slope exactly, moved only by
// the rasterizer's vertex snapping, while a step from one surface to another is many times the
// slope. A cleared neighbour is no pair at all; cleared beyond it means the surface ends there
// and the pair is all there is to fit.
fn ink_pair_planar(pixel: vec2<f32>, dir: vec2<f32>, z: f32, sample: u32) -> bool {
    let z_side = ink_depth(pixel + dir, sample);
    if (z_side == 0.0) {
        return false;
    }
    let z_far = ink_depth(pixel + dir * 2.0, sample);
    if (z_far == 0.0) {
        return true;
    }
    let g = z_side - z;
    let g_far = z_far - z_side;
    return abs(g_far - g) <= ink_tolerance(z, abs(g) + abs(g_far), 0.0);
}

// The carry's verdict, shared by strokes and discs. A texel already nearer than the axis is
// ink only when its surface passes THROUGH the axis, so a plane fitted in front of the axis
// that lands behind it cannot uncover a covered stroke; a farther texel keeps the one-sided
// compare, so a stroke still overhangs a silhouette at full width.
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
    if (z > depth + abs(depth) * DEPTH_REL_TOL) {
        return abs(predicted - depth) <= tolerance;
    }
    return predicted <= depth + tolerance;
}

// The unit texel step away from the stroke on this fragment's side, along the dominant
// component of the perpendicular, so both texels of the fit lie on the fragment's surface.
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
    let perp = vec2<f32>(-axis.along.y, axis.along.x);
    var step = vec2<f32>(sign(perp.x), 0.0);
    if (abs(perp.y) > abs(perp.x)) {
        step = vec2<f32>(0.0, sign(perp.y));
    }
    return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}

// A stroke fragment: the surface under it, fitted from its own texel and the next one away
// from the stroke, carried to the axis, must not be nearer than the axis. On its own face
// or a touching neighbour the carry lands on the axis; a nearer occluder carries nearer and
// hides the fragment even where the occluder recedes past the axis depth at this pixel; a
// farther surface beyond a silhouette carries farther and the stroke overhangs it.
fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let step = ink_step(pixel, axis);
    // Three outcomes, in this order. The pair away from the stroke is written and planar, so it
    // holds the fragment's own surface and carries it. Else the pair toward the stroke is, and
    // a face two texels wide, or one whose edge runs beside the stroke, still carries its own
    // plane; a rejected outward pair never carries, because the step across a surface boundary
    // it just failed on is exactly the phantom plane this guard exists to remove. Else the raw
    // compare, and that is the end of the line: a texel whose neighbours disagree with each
    // other has no surface to carry to the axis, so only its own depth can decide.
    var side = step;
    if (!ink_pair_planar(pixel, side, z, sample)) {
        side = -step;
        if (!ink_pair_planar(pixel, side, z, sample)) {
            return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
        }
    }
    let z_side = ink_depth(pixel + side, sample);
    // The displacement to the axis as a * along + b * side; the two are never parallel.
    let e = axis.at - pixel;
    let det = axis.along.x * side.y - axis.along.y * side.x;
    let a = (e.x * side.y - e.y * side.x) / det;
    let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    let g = z_side - z;
    let predicted = z + a * axis.slope + b * g;
    return ink_carry_visible(predicted, z, axis.depth, ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)));
}

// A disc fragment: the surface under it, fitted along both axes away from the centre and
// carried back to the centre by that plane, is not in front of the disc. A disc is a
// camera-facing billboard, so the whole of it stands or falls with its centre; comparing at
// the fragment instead lets a grazing surface, which crosses the disc's own depth within a
// few pixels of its radius, uncover the rim of a marker buried behind it.
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let d = pixel - centre;
    let along_x = vec2<f32>(select(1.0, -1.0, d.x < 0.0), 0.0);
    let along_y = vec2<f32>(0.0, select(1.0, -1.0, d.y < 0.0));
    // Both axes must sample one surface, by the same guard the stroke uses. A disc steps away
    // from its own centre on both axes and so has no second direction to fall back on: when
    // either tap straddles a discontinuity there is no plane, and the fragment's own depth
    // decides rather than a phantom one carried to the centre.
    if (!ink_pair_planar(pixel, along_x, z, sample) || !ink_pair_planar(pixel, along_y, z, sample)) {
        return z <= depth + abs(depth) * DEPTH_REL_TOL;
    }
    let gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
    let gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
    let predicted = z - d.x * gx - d.y * gy;
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}

// Normals transform by the inverse transpose, including nonuniform instance scales.
fn face_normal(model: mat4x4<f32>, normal: vec3<f32>) -> vec3<f32> {
    let x = model[0].xyz;
    let y = model[1].xyz;
    let z = model[2].xyz;
    let det = dot(x, cross(y, z));
    return mat3x3<f32>(cross(y, z), cross(z, x), cross(x, y)) * normal / det;
}

// Orthographic visibility uses parallel rays, independent of lateral camera position.
fn toward_eye(point: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
    }
    return vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - point;
}
