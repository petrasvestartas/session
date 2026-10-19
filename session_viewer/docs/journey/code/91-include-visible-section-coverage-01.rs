@group(0) @binding(0) var sections:texture_2d<f32>;
@group(0) @binding(1) var<uniform> radius:vec4<f32>;
@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
struct Coverage{@location(0) value:f32,@builtin(frag_depth) depth:f32}
@fragment fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{
 if(any(vec2<u32>(p.xy)>=textureDimensions(sections))){discard;}
 let cap=textureLoad(sections,vec2<i32>(p.xy),0).xy;
 if(cap.y<0.5 || (radius.y>0.5 && cap.y<8.5)){discard;}
 return Coverage(1.,cap.x);
}
