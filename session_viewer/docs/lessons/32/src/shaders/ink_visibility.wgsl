// --8<-- [start:ink-inputs]
// Ink = everything drawn over the faces: edges, curves, markers, dots and arrows.
// Every ink fragment asks this file one question: does a face lie in front of me here?
@group(2) @binding(2) var scene_depth_single: texture_depth_2d; // scene depth at 1x
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d; // scene depth at 4x

override SCENE_MSAA: bool = false; // which depth texture is live
@group(2) @binding(4) var scene_primitive_single: texture_2d<u32>; // triangle index + 1 at 1x, in 16-bit halves
@group(2) @binding(5) var scene_primitive_msaa: texture_multisampled_2d<u32>; // the same at 4x

// 2^-19 of the depth: two depths this close count as the same surface.
const DEPTH_REL_TOL: f32 = 1.9073486e-6;

// Vertex snapping error, px: 1/256.
const SLOPE_PX: f32 = 0.00390625;

// Two neighbouring slopes within 1/32 of each other still count as one flat surface.
const KINK: f32 = 0.03125;

// Depth change per pixel from which a slope carries no plane: 65504 / 65536.
const SLOPE_INVALID: f32 = 0.99951171875;

// Output of an ink fragment.
struct InkColor {
    @location(0) color: vec4<f32>, // rgba
};

// The stroke's center line as one fragment sees it.
struct InkAxis {
    at: vec2<f32>, // nearest point on the center line, screen px
    depth: f32, // its depth
    along: vec2<f32>, // stroke direction on screen, unit
    slope: f32, // depth change per pixel along it
};
// --8<-- [end:ink-inputs]

// --8<-- [start:ink-fit]
// Scene depth at a pixel; 0 outside the screen or where nothing was drawn.
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

// Allowed error of a depth carried `lever` pixels along a slope.
fn ink_tolerance(depth: f32, slope: f32, lever: f32) -> f32 {
    return abs(depth) * DEPTH_REL_TOL + abs(slope) * SLOPE_PX * (1.0 + lever);
}

// Plane fit = read the depth one and two pixels along `dir`; three depths on one straight line are one flat surface.
fn ink_pair_planar(pixel: vec2<f32>, dir: vec2<f32>, z: f32, sample: u32) -> bool {
    let z_side = ink_depth(pixel + dir, sample);

    // no neighbour: no pair
    if (z_side == 0.0) {
        return false;
    }

    let z_far = ink_depth(pixel + dir * 2.0, sample);

    // surface ends: the pair is all there is
    if (z_far == 0.0) {
        return true;
    }

    // the two slopes must nearly agree
    let g = z_side - z;
    let g_far = z_far - z_side;
    return abs(g_far - g) <= abs(z) * DEPTH_REL_TOL + KINK * (abs(g) + abs(g_far));
}

// Carry = extend the face's depth from this pixel along its slope to another point; `predicted` is that depth.
fn ink_carry_visible(predicted: f32, z: f32, depth: f32, tolerance: f32) -> bool {
    // pixel nearer than the ink: only its own surface may show it
    if (z > depth + abs(depth) * DEPTH_REL_TOL) {
        return abs(predicted - depth) <= tolerance;
    }

    return predicted <= depth + tolerance;
}

// One-pixel step away from the stroke, along x or y.
fn ink_step(pixel: vec2<f32>, axis: InkAxis) -> vec2<f32> {
    let perp = vec2<f32>(-axis.along.y, axis.along.x);
    var step = vec2<f32>(sign(perp.x), 0.0);

    if (abs(perp.y) > abs(perp.x)) {
        step = vec2<f32>(0.0, sign(perp.y));
    }

    return select(step, -step, dot(pixel - axis.at, step) < 0.0);
}
// --8<-- [end:ink-fit]

// --8<-- [start:ink-axis]
// A 6 px edge on a curved face covers pixels where the face is nearer than the edge's center line,
// so comparing depth there would hide half the stroke: carry the face's depth to the center line first.
fn ink_axis_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);

    // nothing drawn here: visible
    if (z == 0.0) {
        return true;
    }

    let step = ink_step(pixel, axis);
    // fit away from the stroke, else toward it, else compare raw depth
    var side = step;

    if (!ink_pair_planar(pixel, side, z, sample)) {
        side = -step;

        if (!ink_pair_planar(pixel, side, z, sample)) {
            return z <= axis.depth + abs(axis.depth) * DEPTH_REL_TOL;
        }
    }

    let z_side = ink_depth(pixel + side, sample);
    // write the offset to the center line as a * along + b * side and solve the 2x2 system (Cramer's rule)
    let e = axis.at - pixel;
    let det = axis.along.x * side.y - axis.along.y * side.x;
    let a = (e.x * side.y - e.y * side.x) / det;
    let b = (axis.along.x * e.y - axis.along.y * e.x) / det;
    let g = z_side - z;
    // surface depth carried to the center line
    let predicted = z + a * axis.slope + b * g;
    return ink_carry_visible(predicted, z, axis.depth, ink_tolerance(axis.depth, abs(g) + abs(axis.slope), abs(b)));
}
// --8<-- [end:ink-axis]

// --8<-- [start:ink-discs]
// A disc (marker or dot) has no direction, so the fit runs along x and y and carries to its centre.
fn ink_disc_fragment_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);

    if (z == 0.0) {
        return true;
    }

    let d = pixel - centre;
    // fit along x and y, away from the center first
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

    // slopes per pixel
    let gx = (ink_depth(pixel + along_x, sample) - z) * along_x.x;
    let gy = (ink_depth(pixel + along_y, sample) - z) * along_y.y;
    let predicted = z - d.x * gx - d.y * gy;
    return ink_carry_visible(predicted, z, depth, ink_tolerance(depth, abs(gx) + abs(gy), abs(d.x) + abs(d.y)));
}

// Direction from a point to the camera; constant in ortho.
fn toward_eye(point: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
    }

    return vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - point;
}

// Is a disc visible: its fragment, and its center pixel too.
fn ink_disc_visible(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    if (!ink_disc_fragment_visible(pixel, centre, depth, sample)) {
        return false;
    }

    let source_pixel = floor(centre) + fract(pixel);
    return !ink_disc_source_hidden(source_pixel, centre, depth, sample);
}

// Is a plane fitted at the center pixel in front of the disc?
fn ink_disc_source_hidden(pixel: vec2<f32>, centre: vec2<f32>, depth: f32, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);

    if (z == 0.0) {
        return false;
    }

    let d = pixel - centre;

    // try every quadrant
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
            // the diagonal pixel must agree with the plane too
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
// --8<-- [end:ink-discs]

// --8<-- [start:ink-plane]
// The face pass stores which triangle it drew at each pixel; that triangle's exact slope beats a fit from neighbours.
fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);

    if (z == 0.0) {
        return true;
    }

    // the grid and points are flat
    let primitive = ink_primitive(pixel, sample);
    var gradient = vec2<f32>(0.0);

    if (primitive != 0u) {
        let triangle = min(primitive, arrayLength(&projected)) - 1u;

        // no projected triangle: fit a plane
        if (projected[triangle].edge3.w < 3.0) {
            return ink_axis_visible(pixel, axis, sample);
        }

        gradient = projected[triangle].gradient.xy;
    }

    // too steep for a plane: fit one
    if (any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
        return ink_axis_visible(pixel, axis, sample);
    }

    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    return ink_carry_visible(predicted, z, axis.depth, ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)));
}

@group(2) @binding(6) var<storage, read> projected: array<ProjectedTriangle>; // every triangle in screen space

// Triangle index + 1 at a pixel, or 0.
fn ink_primitive(pixel: vec2<f32>, sample: u32) -> u32 {
    if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
        return 0u;
    }

    var halves: vec2<u32>;

    if (SCENE_MSAA) {
        halves = textureLoad(scene_primitive_msaa, vec2<i32>(pixel), i32(sample)).xy;
    } else {
        halves = textureLoad(scene_primitive_single, vec2<i32>(pixel), 0).xy;
    }

    return halves.x | (halves.y << 16u);
}

@group(2) @binding(7) var<storage, read> triangle_tiles: array<vec4<u32>>; // which triangles cover each screen tile
// --8<-- [end:ink-plane]

// --8<-- [start:ink-cuts]
// Section cap = the flat face a clipping plane draws where it cuts a closed solid (lesson 18b).
// True when a clipping plane removed the triangle point hit at canvas point `at`, depth `depth`.
fn ink_hit_cut(at: vec2<f32>, depth: f32) -> bool {
    return clip_active() && clip_cut_ndc(0u, clip_ndc(at, line.frame, depth));
}

// Is the ink visible where the section cap of clipping plane `plane` lies under its center line?
fn ink_cap_visible(plane: u32, axis: InkAxis) -> bool {
    let slope = clip_plane_slope(plane, line.frame);
    let depth = clip_plane_depth(plane, clip_ndc(axis.at + line.origin, line.frame, 0.0).xy);
    return depth <= axis.depth + ink_tolerance(axis.depth, abs(slope.x) + abs(slope.y), 1.0);
}
// --8<-- [end:ink-cuts]

// --8<-- [start:ink-exact]
// Exact test = ask a triangle itself, by its edge lines and depth plane, whether it covers the center point in front of the ink.
// `boundary` = the stroke is a surface's own border, where a plane fit is least sure, so the exact tests always run.
fn ink_visible(pixel: vec2<f32>, axis: InkAxis, sample: u32, boundary: bool) -> bool {
    // a section cap is flat on its plane: exact where it lies under the center line
    var fit_at = pixel;

    if (clip_active()) {
        let under = floor(axis.at) + fract(pixel);
        let cap = ink_primitive(under, sample);

        if (clip_is_cap(cap)) {
            return ink_cap_visible(clip_cap_plane(cap), axis);
        }

        // a cap beside the center line only: the surface under the line decides
        if (clip_is_cap(ink_primitive(pixel, sample))) {
            fit_at = under;
        }
    }

    let plane_visible = ink_visible_plane(fit_at, axis, sample);
    if (plane_visible && !boundary) {
        return true;
    }

    // Tile = a square of screen pixels listing every triangle that touches it; lesson 18 builds the lists.
    // no tile lists: the plane fit decides
    if (triangle_tiles[0].x==0u) {
        return plane_visible;
    }

    // canvas pixel of the center line
    let at = axis.at + line.origin;
    // the triangle under this pixel, tested exactly
    let fringe = ink_primitive(fit_at, sample);

    if (fringe!=0u && !(clip_active() && clip_is_cap(fringe))) {
        let fringe_hit = projected_triangle_at(projected[fringe-1u], at);
        if (fringe_hit.y>0.5 && fringe_hit.x>axis.depth+abs(axis.depth)*DEPTH_REL_TOL && !ink_hit_cut(at, fringe_hit.x)) {
            return false;
        }
    }

    // the four triangles around the center line, tested exactly
    let source_base = floor(axis.at-fract(fit_at))+fract(fit_at);

    for (var i = 0u;i<4u;i++) {
        let primitive = ink_primitive(source_base+vec2<f32>(f32(i&1u), f32(i>>1u)), sample);

        if (primitive==0u || primitive==fringe || (clip_active() && clip_is_cap(primitive))) {
            continue;
        }

        let hit = projected_triangle_at(projected[primitive-1u], at);

        if (hit.y>0.5 && hit.x>axis.depth+abs(axis.depth)*DEPTH_REL_TOL && !ink_hit_cut(at, hit.x)) {
            return false;
        }
    }

    // a boundary stroke also checks every triangle in its tile
    if (plane_visible) { return true; }

    if (any(at<vec2<f32>(0.0)) || any(at>=line.frame)) {
        return false;
    }

    let span = f32(visibility_tile_span_of(u32(line.frame.x), u32(line.frame.y)));
    let size = vec2<u32>(ceil(line.frame/span));
    let cell = vec2<u32>(at/span);
    let head = triangle_tiles[1u+cell.y*size.x+cell.x];

    // tile record: x count, y list start, z written, w overflow
    if (head.w!=0u || head.z!=head.x) {
        return false;
    }

    for (var i = 0u;i<head.x;i++) {
        let offset = head.y+i*2u;
        let nearest = bitcast<f32>(triangle_tiles[(offset+1u)/4u][(offset+1u)%4u]);

        // triangle no nearer than the ink: skip
        if (nearest<=axis.depth+abs(axis.depth)*DEPTH_REL_TOL) {
            continue;
        }

        let primitive = triangle_tiles[offset/4u][offset%4u];
        let bounds = projected[primitive-1u].bounds;

        if (any(at<bounds.xy-SLOPE_PX) || any(at>bounds.zw+SLOPE_PX)) {
            continue;
        }

        let hit = projected_triangle_at(projected[primitive-1u], at);

        if (hit.y>0.5 && hit.x>axis.depth+abs(axis.depth)*DEPTH_REL_TOL && !ink_hit_cut(at, hit.x)) {
            return false;
        }
    }

    return true;
}
// --8<-- [end:ink-exact]

// --8<-- [start:ink-glass]
// Alpha of hidden ink: 0, or 1 - opacity through translucent faces.
fn through_glass(hidden: bool) -> f32 {
    let glass = line.opacity > 0.0 && line.opacity < 1.0;
    return select(1.0, select(0.0, 1.0 - line.opacity, glass), hidden);
}

// Not WGSL: build.rs pastes the named file in place of this line before the shader compiles.
#include "projected_triangle.wgsl"
// --8<-- [end:ink-glass]
