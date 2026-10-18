@group(0) @binding(0) var global_alpha:texture_2d<f32>;
@group(0) @binding(1) var selected_alpha:texture_2d<f32>;
@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);}
@fragment fn fs_main(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{let xy=vec2<i32>(p.xy);return vec4<f32>(0.,0.,0.,max(textureLoad(global_alpha,xy,0).r,textureLoad(selected_alpha,xy,0).r));}
