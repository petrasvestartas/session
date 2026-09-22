// Depth slope is stored times this.
const PLANE_SCALE: f32 = 65536.0;
// Slope value that means "no plane".
const PLANE_INVALID: f32 = 65504.0;

// Output of a face fragment: color and depth slope.
struct PhysicalColor {
    @location(0) color: vec4<f32>, // rgba
    @location(1) gradient: vec4<f32>, // depth slope, for the visibility test
}

// Output of a face id fragment: ids and depth slope.
struct PhysicalId {
    @location(0) id: vec2<u32>, // object row + 1, sub id + 1
    @location(1) gradient: vec4<f32>, // depth slope, for the visibility test
}

// Depth slope of this fragment, or the invalid marker.
fn physical_gradient(depth: f32) -> vec4<f32> {
 let scaled=vec2<f32>(dpdx(depth), dpdy(depth))*PLANE_SCALE;

 if (any(abs(scaled)>=vec2<f32>(PLANE_INVALID))) {
     return vec4<f32>(PLANE_INVALID, PLANE_INVALID, 0.0, 0.0);
 }

 return vec4<f32>(scaled, 0.0, 0.0);
}

// Depth slope plus the triangle index, packed into two half floats.
fn physical_triangle(depth: f32, primitive: u32) -> vec4<f32> {
 let words=vec2<u32>(primitive&0x3fffu, primitive>>14u)+vec2<u32>(0x400u);
 let address=unpack2x16float(words.x|(words.y<<16u));
 return vec4<f32>(physical_gradient(depth).xy, address);
}
