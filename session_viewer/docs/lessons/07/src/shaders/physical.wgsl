// Depth slope is stored times this.
const PLANE_SCALE: f32 = 65536.0;
// Slope value that means "no plane".
const PLANE_INVALID: f32 = 65504.0;

// Output of a face fragment: color and depth slope.
struct PhysicalColor {
    @location(0) color: vec4<f32>, // rgba
    @location(1) gradient: vec2<f32>,
};

// Output of a face id fragment: ids and depth slope.
struct PhysicalId {
 @location(0) id: vec2<u32>, // object row + 1, sub id + 1
 @location(1) gradient: vec2<f32>,
};

fn physical_gradient(depth: f32) -> vec2<f32> {
    let scaled = vec2<f32>(dpdx(depth), dpdy(depth)) * PLANE_SCALE;

    if (any(abs(scaled) >= vec2<f32>(PLANE_INVALID))) {
        return vec2<f32>(PLANE_INVALID);
    }

    return scaled;
}
