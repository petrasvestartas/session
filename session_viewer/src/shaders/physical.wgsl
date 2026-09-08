// Preserve the rasterizer's primitive gradient, including subpixel and grazing triangles.
const PLANE_SCALE: f32 = 65536.0;
const PLANE_INVALID: f32 = 65504.0;
struct PhysicalColor {
    @location(0) color: vec4<f32>,
    @location(1) gradient: vec2<f32>,
};
struct PhysicalId {
 @location(0) id: vec2<u32>,
 @location(1) gradient: vec2<f32>,
};
fn physical_gradient(depth: f32) -> vec2<f32> {
    let scaled = vec2<f32>(dpdx(depth), dpdy(depth)) * PLANE_SCALE;
    if (any(abs(scaled) >= vec2<f32>(PLANE_INVALID))) { return vec2<f32>(PLANE_INVALID); }
    return scaled;
}
