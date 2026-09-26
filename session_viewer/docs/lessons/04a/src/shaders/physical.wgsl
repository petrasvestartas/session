// Output of a face fragment: color and triangle id.
struct PhysicalColor {
    @location(0) color: vec4<f32>, // rgba
    @location(1) primitive: vec2<u32>, // triangle index + 1, 0 for none, in two 16-bit halves
}

// Output of a face id fragment: ids and triangle id.
struct PhysicalId {
    @location(0) id: vec2<u32>, // object row + 1, sub id + 1
    @location(1) primitive: vec2<u32>, // triangle index + 1, 0 for none, in two 16-bit halves
}

// A triangle id as the two 16-bit halves the id target stores; 32-bit ids have no 4x format.
fn physical_primitive(primitive: u32) -> vec2<u32> {
    return vec2<u32>(primitive & 0xffffu, primitive >> 16u);
}
