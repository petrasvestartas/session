// Two half-float gradients and a lossless triangle address in the same 8-byte sample.
const PLANE_SCALE: f32 = 65536.0;
const PLANE_INVALID: f32 = 65504.0;
struct PhysicalColor { @location(0) color: vec4<f32>, @location(1) gradient: vec4<f32>, };
struct PhysicalId { @location(0) id: vec2<u32>, @location(1) gradient: vec4<f32>, };
fn physical_gradient(depth: f32) -> vec4<f32> {
 let scaled=vec2<f32>(dpdx(depth),dpdy(depth))*PLANE_SCALE;
 if (any(abs(scaled)>=vec2<f32>(PLANE_INVALID))) { return vec4<f32>(PLANE_INVALID,PLANE_INVALID,0.0,0.0); }
 return vec4<f32>(scaled,0.0,0.0);
}
// Each 14-bit word becomes a finite NORMAL half float by skipping exponent zero.
// This preserves all 28 address bits through RGBA16Float without NaNs or denormals.
fn physical_triangle(depth: f32, primitive: u32) -> vec4<f32> {
 let words=vec2<u32>(primitive&0x3fffu,primitive>>14u)+vec2<u32>(0x400u);
 let address=unpack2x16float(words.x|(words.y<<16u));
 return vec4<f32>(physical_gradient(depth).xy,address);
}
