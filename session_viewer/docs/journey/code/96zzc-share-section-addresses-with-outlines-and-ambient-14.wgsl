fn source_primitive(at:vec2<i32>)->u32{
 let size=vec2<i32>(textureDimensions(source_primitives));
 let halves=textureLoad(source_primitives,clamp(at,vec2<i32>(0),size-1),0).xy;
 return physical_address(halves);
}
fn source_radius(pixel:vec2<f32>)->f32{