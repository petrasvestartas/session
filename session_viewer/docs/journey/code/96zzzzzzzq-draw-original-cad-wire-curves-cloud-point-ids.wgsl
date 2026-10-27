// Page ownership belongs to the cached ID output, outside the production splat record.
@group(2) @binding(0) var<uniform> page_address: vec4<u32>;

struct CachedPoint {
    @location(0) color: vec4<f32>,
    @location(1) owner: vec2<u32>,
};

@fragment
fn fs_cached_point(in: PointOut) -> CachedPoint {
    if (outside(in)) {
        discard;
    }
    return CachedPoint(in.color, vec2<u32>(in.instance + 1u, in.row + page_address.x + 1u));
}
