fn ink_visible_plane(pixel: vec2<f32>, axis: InkAxis, sample: u32) -> bool {
    let z = ink_depth(pixel, sample);
    if (z == 0.0) {
        return true;
    }
    let primitive = ink_primitive(pixel, sample);
    var gradient = vec2<f32>(0.0);
    var fallback = false;
    if (clip_is_cap(primitive)) {
        let plane = clip_cap_plane(primitive);
        if (plane >= clipping.count) {
            fallback = true;
        } else {
            gradient = -clip_plane_slope(plane, view.size);
        }
    } else if (primitive != 0u) {
        if (primitive > arrayLength(&finite_projected)) {
            fallback = true;
        } else {
            let triangle = primitive - 1u;
            if (finite_projected[triangle].edge3.w < 3.0) {
                fallback = true;
            } else {
                gradient = finite_projected[triangle].gradient.xy;
            }
        }
    }
    if (fallback || any(abs(gradient) >= vec2<f32>(SLOPE_INVALID))) {
        return ink_axis_visible(pixel, axis, sample);
    }
    let delta = axis.at - pixel;
    let predicted = z + dot(gradient, delta);
    let tolerance = ink_tolerance(axis.depth, abs(gradient.x)+abs(gradient.y), abs(delta.x)+abs(delta.y)) + ink_sag_slack(axis.sag, gradient);
    return ink_carry_visible(predicted, z, axis.depth, tolerance);
}
