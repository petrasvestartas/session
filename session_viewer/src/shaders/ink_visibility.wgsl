// Shared by the ink lanes. Face identity is exact; depth only orders unrelated geometry.
@group(2) @binding(2) var scene_depth_single: texture_depth_2d;
@group(2) @binding(3) var scene_depth_msaa: texture_depth_multisampled_2d;
@group(2) @binding(4) var scene_face_single: texture_2d<u32>;
@group(2) @binding(5) var scene_face_msaa: texture_multisampled_2d<u32>;

struct FacePlane {
    point: vec3<f32>,
    instance_id: u32,
    normal: vec3<f32>,
    _pad: u32,
};
@group(2) @binding(6) var<storage, read> face_planes: array<FacePlane>;

override SCENE_MSAA: bool = false;

struct InkSupport {
    face: u32,
    region: u32,
};
@group(3) @binding(1) var<storage, read> supports: array<InkSupport>;

struct InkFootprint {
    range: vec2<u32>,
    start: vec3<f32>,
    end: vec3<f32>,
};

// A pixel-frequency shader shares shading while independently enabling visible MSAA samples.
struct InkColor {
    @location(0) color: vec4<f32>,
    @builtin(sample_mask) mask: u32,
};

struct InkSample {
    depth: f32,
    world: vec3<f32>,
};

// A face's support regions are independent of the queried footprint position. Cache
// the three bits once; endpoint regions remain restricted to their own round corners.
fn support_regions(face: u32, footprint: InkFootprint) -> u32 {
    var regions = 0u;
    for (var i = 0u; i < footprint.range.y; i += 1u) {
        let support = supports[footprint.range.x + i];
        if (support.face == face) {
            if (support.region == 0u) { return 1u; }
            regions |= 1u << support.region;
        }
    }
    return regions;
}

struct InkOccluder {
    depth: f32,
    face: u32,
};

struct InkDecision {
    face: u32,
    visible: bool,
    regions: u32,
};

// Explicit sample reads expose the common all-clear pixel before support/plane work.
fn ink_depths(pixel: vec2<f32>) -> vec4<f32> {
    if (any(pixel < vec2<f32>(0.0)) || any(pixel >= vec2<f32>(line.vp_w, line.vp_h))) {
        return vec4<f32>(0.0);
    }
    let at = vec2<i32>(pixel);
    if (SCENE_MSAA) {
        return vec4<f32>(
            textureLoad(scene_depth_msaa, at, 0),
            textureLoad(scene_depth_msaa, at, 1),
            textureLoad(scene_depth_msaa, at, 2),
            textureLoad(scene_depth_msaa, at, 3));
    }
    return vec4<f32>(textureLoad(scene_depth_single, at, 0), 0.0, 0.0, 0.0);
}

// Read identity only for a nonempty physical sample; its depth was already fetched.
fn ink_occluder(pixel: vec2<f32>, sample_index: u32, physical: f32) -> InkOccluder {
    if (physical == 0.0) {
        return InkOccluder(0.0, 0u);
    }
    var packed: vec4<u32>;
    if (SCENE_MSAA) {
        packed = textureLoad(scene_face_msaa, vec2<i32>(pixel), i32(sample_index));
    } else {
        packed = textureLoad(scene_face_single, vec2<i32>(pixel), 0);
    }
    let face = packed.x | (packed.y << 16u);
    return InkOccluder(physical, face);
}

// A nonzero face's plane result and support bits depend on the physical axis, not
// on which footprint pixel or MSAA sample is querying that same face.
fn ink_face_decision(face: u32, axis: InkSample, footprint: InkFootprint) -> InkDecision {
    let regions = support_regions(face, footprint);
    if ((regions & 1u) != 0u) {
        return InkDecision(face, true, 0u);
    }
    let plane = face_planes[face - 1u];
    let point = plane.point + translations[plane.instance_id].xyz;
    let normal = plane.normal;
    let visible = dot(normal, axis.world - point) * dot(normal, toward_eye(point)) >= 0.0;
    return InkDecision(face, visible, regions);
}

// Restrict endpoint support separately at the centreline and at the stroke pixel.
fn ink_decision_visible(decision: InkDecision, pixel: vec2<f32>, footprint: InkFootprint) -> bool {
    return decision.visible
        || ((decision.regions & 2u) != 0u && distance(pixel, footprint.start.xy) <= footprint.start.z)
        || ((decision.regions & 4u) != 0u && distance(pixel, footprint.end.xy) <= footprint.end.z);
}

// Bounds include physical mesh rasterization and cloud radii; uncertain projections cover
// the full viewport. Both queries must be outside before bypassing the silhouette gate.
fn outside_occluders(pixel: vec2<f32>) -> bool {
    return any(pixel < line.occluder_rect.xy) || any(pixel > line.occluder_rect.zw);
}

// A covered centreline cannot become visible merely because its wide footprint reaches
// past a covering silhouette. Compute the physical axis and coverage once per pixel while
// retaining one independent visibility bit for each physical raster sample.
fn ink_visible_mask(pixel: vec2<f32>, axis: InkSample, footprint: InkFootprint) -> u32 {
    if (line.occluder_rect.x > line.occluder_rect.z) {
        return select(1u, 15u, SCENE_MSAA);
    }
    let clip = mvp * vec4<f32>(axis.world, 1.0);
    let ndc = clip.xy / clip.w;
    let centre = (ndc * vec2<f32>(0.5, -0.5) + 0.5) * vec2<f32>(line.vp_w, line.vp_h);
    let centre_outside = outside_occluders(centre);
    let pixel_outside = outside_occluders(pixel);
    if (centre_outside && pixel_outside) {
        return select(1u, 15u, SCENE_MSAA);
    }
    // Floor keeps a centre just outside the viewport distinct from the first valid texel.
    let same_pixel = all(floor(centre) == floor(pixel));
    var centre_depths = vec4<f32>(0.0);
    if (!centre_outside) {
        centre_depths = ink_depths(centre);
    }
    var pixel_depths = centre_depths;
    if (!same_pixel) {
        pixel_depths = vec4<f32>(0.0);
        if (!pixel_outside) {
            pixel_depths = ink_depths(pixel);
        }
    }
    if (all(centre_depths == vec4<f32>(0.0)) && all(pixel_depths == vec4<f32>(0.0))) {
        return select(1u, 15u, SCENE_MSAA);
    }
    let count = select(1u, 4u, SCENE_MSAA);
    var mask = 0u;
    var centre_cache = InkDecision(0u, false, 0u);
    var pixel_cache = InkDecision(0u, false, 0u);
    for (var sample = 0u; sample < count; sample += 1u) {
        let centre_surface = ink_occluder(centre, sample, centre_depths[sample]);
        var centre_visible = centre_surface.depth == 0.0;
        if (!centre_visible) {
            if (centre_surface.face == 0u) {
                centre_visible = axis.depth >= centre_surface.depth;
            } else {
                if (centre_surface.face != centre_cache.face) {
                    if (centre_surface.face == pixel_cache.face) {
                        centre_cache = pixel_cache;
                    } else {
                        centre_cache = ink_face_decision(centre_surface.face, axis, footprint);
                    }
                }
                centre_visible = ink_decision_visible(centre_cache, centre, footprint);
            }
        }
        if (!centre_visible) {
            continue;
        }
        // A free stroke has no position-dependent support regions; the same texel is
        // therefore the same complete query. Other strokes still test both regions.
        if (same_pixel && footprint.range.y == 0u) {
            mask |= 1u << sample;
            continue;
        }
        var pixel_surface = centre_surface;
        if (!same_pixel) {
            pixel_surface = ink_occluder(pixel, sample, pixel_depths[sample]);
        }
        var pixel_visible = pixel_surface.depth == 0.0;
        if (!pixel_visible) {
            if (pixel_surface.face == 0u) {
                pixel_visible = axis.depth >= pixel_surface.depth;
            } else {
                if (pixel_surface.face != pixel_cache.face) {
                    if (pixel_surface.face == centre_cache.face) {
                        pixel_cache = centre_cache;
                    } else {
                        pixel_cache = ink_face_decision(pixel_surface.face, axis, footprint);
                    }
                }
                pixel_visible = ink_decision_visible(pixel_cache, pixel, footprint);
            }
        }
        if (pixel_visible) {
            mask |= 1u << sample;
        }
    }
    return mask;
}

// Picking is single-sampled; select ink if any corresponding colour sample is visible.
fn ink_pick_visible(pixel: vec2<f32>, axis: InkSample, footprint: InkFootprint) -> bool {
    return ink_visible_mask(pixel, axis, footprint) != 0u;
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
