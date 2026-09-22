@group(2) @binding(2) var scene_depth_single: texture_depth_2d; // scene depth at 1x
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d; // scene depth at 4x
override SCENE_MSAA: bool = false; // which depth texture is live

// Output of an ink fragment.
struct InkColor {
    @location(0) color: vec4<f32>, // rgba
}

// The stroke's center line as one fragment sees it.
struct InkAxis {
    at:vec2<f32>,
    depth:f32,
    along:vec2<f32>,
    slope:f32,
}

fn ink_depth(pixel:vec2<f32>, sample:u32) -> f32 {
    if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
        return 0.0;
    }

    if (SCENE_MSAA) {
        return textureLoad(scene_depth_msaa, vec2<i32>(pixel), i32(sample));
    }

    return textureLoad(scene_depth_single, vec2<i32>(pixel), 0);
}

fn ink_visible(pixel:vec2<f32>, axis:InkAxis, sample:u32) -> bool {
    return ink_depth(pixel, sample) <= axis.depth;
}

fn ink_disc_fragment_visible(pixel:vec2<f32>, centre:vec2<f32>, depth:f32, sample:u32) -> bool {
    return ink_depth(pixel, sample) <= depth;
}

fn ink_disc_visible(pixel:vec2<f32>, centre:vec2<f32>, depth:f32, sample:u32) -> bool {
    return ink_disc_fragment_visible(pixel, centre, depth, sample);
}

fn ink_disc_source_hidden(pixel:vec2<f32>, centre:vec2<f32>, depth:f32, sample:u32) -> bool {
    return ink_depth(pixel, sample) > depth;
}

// Direction from a point to the camera; constant in ortho.
fn toward_eye(point: vec3<f32>) -> vec3<f32> {
    if (line.ortho_h > 0.0) {
        return vec3<f32>(mvp[0].z, mvp[1].z, mvp[2].z);
    }

    return vec3<f32>(line.eye_x, line.eye_y, line.eye_z) - point;
}
