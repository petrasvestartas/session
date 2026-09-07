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
// times that; 2^-6 carries a factor four of headroom.
const SLOPE_PX: f32 = 0.015625;

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
    // At the surface's rim the outward texel is cleared: fit toward the stroke instead, so a
    // face two texels wide still carries its plane to the axis.
    var side = step;
    var z_side = ink_depth(pixel + side, sample);
    if (z_side == 0.0) {
        side = -step;
        z_side = ink_depth(pixel + side, sample);
    }
    if (z_side == 0.0) {
        return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
    }
    // The displacement to the axis as a * along + b * side; the two are never parallel.
    let e = axis.at - pixel;
    let det = axis.along.x * side.y - axis.along.y * side.x;
    let a = (e.x * side.y - e.y * side.x) / det;
    let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    let g = z_side - z;
    let predicted = z + a * axis.slope + b * g;
    return predicted <= axis.depth + ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b));
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
    let sx = select(1.0, -1.0, d.x < 0.0);
    let sy = select(1.0, -1.0, d.y < 0.0);
    let zx = ink_depth(pixel + vec2<f32>(sx, 0.0), sample);
    let zy = ink_depth(pixel + vec2<f32>(0.0, sy), sample);
    if (zx != 0.0 && zy != 0.0) {
        let gx = (zx - z) * sx;
        let gy = (zy - z) * sy;
        let predicted = z - d.x * gx - d.y * gy;
        return predicted <= depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y));
    }
    return z <= depth + abs(depth) * DEPTH_REL_TOL;
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
