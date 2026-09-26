// `@location(n)` on an output = the n-th colour target in the pipeline's list.
// Output of a face fragment: color and triangle id.
struct PhysicalColor {
    @location(0) color: vec4<f32>, // rgba
    @location(1) primitive: vec2<u32>, // triangle index + 1, 0 for none, in two 16-bit halves; register:physical
}

// A triangle id as the two 16-bit halves the id target stores; 32-bit ids have no 4x format.
fn physical_primitive(primitive: u32) -> vec2<u32> { // register:physical
    return vec2<u32>(primitive & 0xffffu, primitive >> 16u);
}
