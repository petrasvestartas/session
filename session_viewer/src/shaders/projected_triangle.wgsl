// One triangle in screen space, 96 bytes.
struct ProjectedTriangle {
    edge0: vec4<f32>, // xyz: edge line in canvas px, unit normal inward; w: depth per px of distance from it
    edge1: vec4<f32>, // xyz: edge line; w: its depth weight
    edge2: vec4<f32>, // xyz: edge line; w: its depth weight
    edge3: vec4<f32>, // x: the plane's depth at the canvas origin; w: 3 for a drawn triangle, 0 for none
    gradient: vec4<f32>, // xy: depth slope per px; z: nearest depth; w: contact radius
    bounds: vec4<f32>, // screen box: left, top, right, bottom
};

// (depth, 1) where the triangle covers `at`, else (0, 0).
fn projected_triangle_at(triangle: ProjectedTriangle, at: vec2<f32>) -> vec2<f32> {
    // empty triangle
    if (triangle.edge3.w<3.0) {
        return vec2<f32>(0.0);
    }

    // 1/256 px slack: the rasterizer's vertex snap
    if (any(at<triangle.bounds.xy-0.00390625) || any(at>triangle.bounds.zw+0.00390625)) {
        return vec2<f32>(0.0);
    }

    // signed distance to each edge; negative = outside
    let distances = vec3<f32>(dot(triangle.edge0.xy, at)+triangle.edge0.z,
    dot(triangle.edge1.xy, at)+triangle.edge1.z,
    dot(triangle.edge2.xy, at)+triangle.edge2.z);

    if (any(distances<vec3<f32>(-0.00390625))) {
        return vec2<f32>(0.0);
    }

    // depth at `at`: each edge's distance times its weight; inside the triangle every term is
    // positive, so the sum rounds within a few ulps of the depth itself
    let depth = dot(vec3<f32>(triangle.edge0.w, triangle.edge1.w, triangle.edge2.w), distances);
    return vec2<f32>(depth, 1.0);
}
// Pixels per tile side for a canvas; must match TileLayout in Rust.

fn visibility_tile_span_of(width: u32, height: u32) -> u32 {
    var span = 4u;

    while (((width+span-1u)/span)*((height+span-1u)/span)>262144u) {
        span*=2u;
    }

    return span;
}

// Pixels per tile side for this canvas.
fn visibility_tile_span() -> u32 {
    return visibility_tile_span_of(u32(line.vp_w), u32(line.vp_h));
}
