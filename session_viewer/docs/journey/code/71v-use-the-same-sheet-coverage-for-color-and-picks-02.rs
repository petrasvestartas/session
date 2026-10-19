@group(2) @binding(0) var<uniform> crop:Crop;
@vertex
fn pick_vertex(@location(0) start:vec3<f32>,@location(1) radius:f32,@location(2) end:vec3<f32>,@location(3) packed:u32,@builtin(vertex_index) v:u32,@builtin(instance_index) i:u32)->Output {var output=geometry(start,end,radius,packed,v,i);output.position=crop.transform*output.position;return output;}
@fragment
fn pick_fragment(input:Output)->@location(0) vec2<u32> {if (!wanted(input.id.y,crop.mode)||coverage(input)<=0.0) {discard;}return input.id;}
