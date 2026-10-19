@group(1) @binding(0) var<uniform> crop:Crop;
struct PickOutput{@builtin(position)position:vec4<f32>}
@vertex fn pick_vertex(@location(0)start:vec3<f32>,@location(1)end:vec3<f32>,@location(2)colour:vec4<f32>,@location(3)width:f32,@location(4)dashed:u32,@builtin(vertex_index)index:u32)->PickOutput{return PickOutput(crop.transform*geometry(start,end,colour,width,index,dashed).position);}
@fragment fn pick_fragment()->@location(0)vec2<u32>{return vec2<u32>(0u);}
