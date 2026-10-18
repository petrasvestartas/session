c3<f32>, // depth slope and nearest depth
    @location(7) @interpolate(flat) reference: vec3<f32>, // reference point and its depth
};
#include "projected_triangle.wgsl"
#include "projected_triangle.wgsl:viewport-span"

@vertex
fn vs_main(@builtin(vertex_index) vertex: u32, @builtin(instance_index) instance: u32) -> TileVertex {
    let triangle = projected[instance];
    var out: TileVertex;
    out.clip = vec4<f32>(2.0, 2.0, 0.0, 1.0);

    // empty triangle: off screen
    if (triangle.edge3.w<3.0) {
        return out;
    }

    // tile grid size
    let size = ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span()));
    let lo = clamp(floor((triangle.bounds.xy-0.00390625)/f32(visibility_tile_span())), vec2<f32>(0.0), size);
    let hi = clamp(floor((triangle.bounds.zw+0.00390625)/f32(visibility_tile_span()))+1.0, vec2<f32>(0.0), size);
    // triangle strip: two triangles from four corners
    let corners = array<vec2<f32>, 4>(vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0));
    let position = mix(lo, hi, corners[vertex])/size;
    out.clip = vec4<f32>(position.x*2.0-1.0, 1.0-position.y*2.0, 0.0, 1.0);
    out.edge0 = triangle.edge0.xyz;
    out.edge1 = triangle.edge1.xyz;
    out.edge2 = triangle.edge2.xyz;
    out.edge3 = triangle.edge3.xyz;
    out.bounds = triangle.bounds;
    out.primitive = instance+1u;
    out.gradient = triangle.gradient.xyz;
    out.reference = vec3<f32>(triangle.edge0.w, triangle.edge1.w, triangle.edge2.w);
    return out;
}
fn tile_outside(edge: vec3<f32>, centre: vec2<f32>) -> bool {
    return dot(edge.xy, centre)+edge.z+0.5*f32(visibility_tile_span())*(abs(edge.x)+abs(edge.y)) < -0.00390625;
}
fn covered_tile(v: TileVertex) -> u32 {
    let centre = (floor(v.clip.xy)+0.5)*f32(visibility_tile_span());

    if (tile_outside(v.edge0, centre) || tile_outside(v.edge1, centre) || tile_outside(v.edge2, centre) || tile_outside(v.edge3, centre)) {
        discard;
    }

    let size = vec2<u32>(ceil(vec2<f32>(line.vp_w, line.vp_h)/f32(visibility_tile_span())));
    let tile = 1u+u32(v.clip.y)*size.x+u32(v.clip.x);
    return tile;
}
@fragment
fn fs_count(v: TileVertex) -> @location(0) f32 {
    let tile = covered_tile(v);
    atomicAdd(&tile_records[tile].values[0], 1u);
    return 0.0;
}
