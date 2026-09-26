// `@location(n)` on an output = the n-th colour target in the pipeline's list.
// Output of a face fragment: color and triangle id.
struct PhysicalColor {
    @location(0) color: vec4<f32>, // rgba
    @location(1) primitive: vec2<u32>, // triangle index + 1, 0 for none, in two 16-bit halves; register:physical
}

// Output of a face id fragment: ids and triangle id.
struct PhysicalId { // register:pick
    @location(0) id: vec2<u32>, // object row + 1, sub id + 1
    @location(1) primitive: vec2<u32>, // triangle index + 1, 0 for none, in two 16-bit halves
}

// A triangle id as the two 16-bit halves the id target stores; 32-bit ids have no 4x format.
fn physical_primitive(primitive: u32) -> vec2<u32> { // register:physical
    return vec2<u32>(primitive & 0xffffu, primitive >> 16u);
}
