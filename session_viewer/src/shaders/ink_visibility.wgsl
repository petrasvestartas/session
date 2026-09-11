// Ink compares its axis with the physical primitive depth carried by that primitive's
// raster gradient. This retains subpixel and grazing facets without relaxing occlusion.
// A bounded neighboring-plane fit handles gradients outside the attachment's range.
// Reverse-Z: nearer is greater, 0 is cleared. Pixels are framebuffer coordinates, y down.
@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d;

override SCENE_MSAA: bool = false;
@group(2) @binding(4) var scene_gradient_single: texture_2d<f32>;
@group(2) @binding(5) var scene_gradient_msaa: texture_multisampled_2d<f32>;

// 2^-19: about 16 ULPs of a float, relative to the depth.
const DEPTH_REL_TOL: f32 = 1.9073486e-6;
// The rasterizer snaps vertices to 1/256 px, so a fitted plane's depth is off by its slope
// times that; 2^-8 is exactly that quantisation, with the headroom measured away: the close-up
// holds at 242720 non-background pixels and the probe matrix at 54 cases with its nine
// distance-1 counts unchanged, while the floor census residual falls from 29 to 13 samples.
const SLOPE_PX: f32 = 0.00390625;
// How much of the two slopes a KINK may differ by and still count as one surface. A tessellation
// is piecewise planar and turns at every facet boundary, so a curved BRep's edge curve runs along
// kinks and loses half its width when the guard reads one as a surface jump; a real jump changes
// the slope by many times itself. 2^-5 is the largest fraction that keeps the floor census at
// zero at every camera at 1x and 4x - 2^-4 leaks one sample at down_4_flat - and it leaves the
// close-up at 242720 non-background pixels, the probe matrix at 54 cases with its nine distance-1
// counts unchanged, and the census's 16x counts unchanged, while the BRep orbit mean rises from
// 709 to 724. It does NOT admit a whole facet kink: 5 degrees between samples at 45 degrees of
// incidence is about 0.09 of the slope. The rest of a sphere's meridian waits for edge and face
// discretisation to match, which is phase 2.
const KINK: f32 = 0.03125;

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
// pair may be fitted: on a plane the next texel out extends the slope, bent only by the
// rasterizer's vertex snapping and by whatever kink a tessellation has there, while a step from
// one surface to another is many times the slope. A cleared neighbour is no pair at all; cleared
// beyond it means the surface ends there and the pair is all there is to fit.

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
    return abs(g_far - g) <= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
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

fn ink_axis_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
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

fn ink_disc_fragment_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let d = pixel - centre;
    var along_x = vec2<f32>(select(1.0, -1.0, d.x < 0.0), 0.0);
    var along_y = vec2<f32>(0.0, select(1.0, -1.0, d.y < 0.0));
    if (!ink_pair_planar(pixel, along_x, z, sample)) {
        along_x = -along_x;
        if (!ink_pair_planar(pixel, along_x, z, sample)) {
            return z <= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }
    if (!ink_pair_planar(pixel, along_y, z, sample)) {
        along_y = -along_y;
        if (!ink_pair_planar(pixel, along_y, z, sample)) {
            return z <= depth + abs(depth) * DEPTH_REL_TOL;
        }
    }
    let gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
    let gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
    let predicted = z - d.x * gx - d.y * gy;
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}

// Orthographic visibility uses parallel rays, independent of lateral camera position.

fn toward_eye(point: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
    }
    return vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - point;
}

// Test the footprint and its source centre at the same subpixel sample position.
// Only a verified plane can veto the centre: a raw texel at a silhouette is ambiguous.

fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    if (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
        return false;
    }
    let source_pixel = floor(centre) + fract(pixel);
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}

// At a corner, independent x/y fits can belong to different faces. The diagonal must
// agree too; try each quadrant so a boundary does not discard an otherwise valid fit.
// The centre is hidden only when that complete plane lies strictly in front of it.

fn ink_disc_source_hidden(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return false;
    }
    let d = pixel - centre;
    for (var x = 0u; x < 2u; x++) {
        let dx = vec2<f32>(select(1.0, -1.0, x == 1u), 0.0);
        if (!ink_pair_planar(pixel, dx, z, sample)) {
            continue;
        }
        let gx = (ink_depth(pixel + dx, sample) - z) * dx.x;
        for (var y = 0u; y < 2u; y++) {
            let dy = vec2<f32>(0.0, select(1.0, -1.0, y == 1u));
            if (!ink_pair_planar(pixel, dy, z, sample)) {
                continue;
            }
            let gy = (ink_depth(pixel + dy, sample) - z) * dy.y;
            let diagonal = ink_depth(pixel + dx + dy, sample);
            let expected = z + gx * dx.x + gy * dy.y;
            if (diagonal == 0.0 || abs(diagonal - expected) > ink_tolerance(z, abs(gx) + abs(gy), 2.0)) {
                continue;
            }
            let predicted = z - d.x * gx - d.y * gy;
            if (predicted > depth + ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y))) {
                return true;
            }
        }
    }
    return false;
}


// Use the physical primitive's own gradient even when it covers only one sample.

fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) { return true; }
    var encoded = vec2<f32>(0.0);
    if (SCENE_MSAA) { encoded = textureLoad(scene_gradient_msaa, vec2<i32>(pixel), i32(sample)).xy; }
    else { encoded = textureLoad(scene_gradient_single, vec2<i32>(pixel), 0).xy; }
    if (any(abs(encoded) >= vec2<f32>(PLANE_INVALID))) { return ink_axis_visible(pixel, axis, sample); }
    let gradient = encoded / PLANE_SCALE;
    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    return ink_carry_visible(predicted, z, axis.depth, ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)));
}
@group(2) @binding(6) var<storage,read> projected: array<ProjectedTriangle>;

// Return the index+1 of the exact opaque triangle that won this depth sample.

fn ink_primitive(pixel: vec2<f32>, sample: u32) -> u32 {
    if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
        return 0u;
    }
    if (SCENE_MSAA) {
        return ink_decode_primitive(textureLoad(scene_gradient_msaa, vec2<i32>(pixel), i32(sample)).zw);
    }
    return ink_decode_primitive(textureLoad(scene_gradient_single, vec2<i32>(pixel), 0).zw);
}
@group(2) @binding(7) var<storage, read> triangle_tiles: array<vec4<u32>>;

fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    if (ink_visible_plane(pixel, axis, sample)) {
        return true;
    }
    if (triangle_tiles[0].x==0u) {
        return false;
    }
    // The projected triangles and their tiles are in canvas pixels; the pick pass renders a
    // window of the canvas into an attachment of its own, so its fragments are offset by
    // `origin` (zero in a frame).
    let at = axis.at + line.origin;
    let fringe = ink_primitive(pixel, sample);
    if (fringe==0u) {
        return false;
    }
    let fringe_hit = projected_triangle_at(projected[fringe-1u], at);
    if (fringe_hit.y>0.5 && fringe_hit.x>axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
        return false;
    }
    let source_base = floor(axis.at-fract(pixel))+fract(pixel);
    for (var i = 0u;i<4u;i++) {
        let primitive = ink_primitive(source_base+vec2<f32>(f32(i&1u), f32(i>>1u)), sample);
        if (primitive==0u || primitive==fringe) {
            continue;
        }
        let hit = projected_triangle_at(projected[primitive-1u], at);
        if (hit.y>0.5 && hit.x>axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            return false;
        }
    }
    if (any(at<vec2<f32>(0.0)) || any(at>=line.frame)) {
        return false;
    }
    let span = f32(visibility_tile_span_of(u32(line.frame.x), u32(line.frame.y)));
    let size = vec2<u32>(ceil(line.frame/span));
    let cell = vec2<u32>(at/span);
    let head = triangle_tiles[1u+cell.y*size.x+cell.x];
    // The tile's record as four words: x = triangles covering it, y = where its list starts,
    // z = how many were actually written, w = the overflow flag. An incomplete list (z != x)
    // or an overflowing one cannot prove that the axis is clear.
    if (head.w!=0u || head.z!=head.x) {
        return false;
    }
    for (var i = 0u;i<head.x;i++) {
        let offset = head.y+i*2u;
        let nearest = bitcast<f32>(triangle_tiles[(offset+1u)/4u][(offset+1u)%4u]);
        if (nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            continue;
        }
        let primitive = triangle_tiles[offset/4u][offset%4u];
        let bounds = projected[primitive-1u].bounds;
        if (any(at<bounds.xy-SLOPE_PX) || any(at>bounds.zw+SLOPE_PX)) {
            continue;
        }
        let hit = projected_triangle_at(projected[primitive-1u], at);
        if (hit.y>0.5 && hit.x>axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            return false;
        }
    }
    return true;
}

fn ink_decode_primitive(encoded: vec2<f32>) -> u32 {
    if (any(encoded==vec2<f32>(0.0))) {
        return 0u;
    }
    let packed = pack2x16float(encoded);
    return ((packed&0xffffu)-0x400u) | (((packed>>16u)-0x400u)<<14u);
}
