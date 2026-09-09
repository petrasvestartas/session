struct ProjectedTriangle {
    edge0: vec4<f32>, // xyz = inward unit normal + constant; w = reference x
    edge1: vec4<f32>, // w = reference y
    edge2: vec4<f32>, // w = reference depth
    edge3: vec4<f32>, // w = corner count; xyz zero for an unclipped triangle
    gradient: vec4<f32>,
    bounds: vec4<f32>,
};

fn projected_triangle_at(triangle: ProjectedTriangle, at: vec2<f32>) -> vec2<f32> {
    if (triangle.edge3.w<3.0) {
        return vec2<f32>(0.0);
    }
    if (any(at<triangle.bounds.xy-0.00390625) || any(at>triangle.bounds.zw+0.00390625)) {
        return vec2<f32>(0.0);
    }
    let distances = vec4<f32>(dot(triangle.edge0.xy, at)+triangle.edge0.z,
    dot(triangle.edge1.xy, at)+triangle.edge1.z,
    dot(triangle.edge2.xy, at)+triangle.edge2.z,
    dot(triangle.edge3.xy, at)+triangle.edge3.z);
    if (any(distances<vec4<f32>(-0.00390625))) {
        return vec2<f32>(0.0);
    }
    let depth = triangle.edge2.w+dot(triangle.gradient.xy, at-vec2<f32>(triangle.edge0.w, triangle.edge1.w));
    return vec2<f32>(depth, 1.0);
}
// Bound the header and pooled-reference allocations at large framebuffer sizes.
// The CPU TileLayout uses the same integer rule; the common case is four pixels.

fn visibility_tile_span() -> u32 {
    let width = u32(line.vp_w);
    let height = u32(line.vp_h);
    var span = 4u;
    while (((width+span-1u)/span)*((height+span-1u)/span)>262144u) {
        span*=2u;
    }
    return span;
}
