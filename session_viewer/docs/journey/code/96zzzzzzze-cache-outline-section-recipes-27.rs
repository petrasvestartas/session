yout)
}

fn source(samples:u32)->String {
 let source=include_str!("outline_caps.wgsl");if samples==1{return source.into();}
 source.replace("var primitives:texture_2d<u32>;","var primitives:texture_multisampled_2d<u32>;")
 .replace(r#"@vertex fn vs_main(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{
 let p=array<vec2<f32>,3>(vec2<f32>(-1.,-1.),vec2<f32>(3.,-1.),vec2<f32>(-1.,3.));return vec4<f32>(p[i],0.,1.);
}
"#,r#"struct Vertex{@builtin(position) position:vec4f,@location(0) @interpolate(linear,sample) pixel:vec2f}
@vertex fn vs_main(@builtin(vertex_index) i:u32)->Vertex {
 let p=array<vec2f,3>(vec2f(-1,-1),vec2f(3,-1),vec2f(-1,3))[i];return Vertex(vec4f(p,0,1),p*vec2f(0.5,-0.5)+0.5);
}
"#)
 .replace("fn fs_main(@builtin(position) p:vec4<f32>)->Coverage{","fn fs_main(input:Vertex,@builtin(sample_index) sample:u32)->Coverage{let p=vec4f(input.pixel*vec2f(textureDimensions(primitives)),0,1);")
 .replace("textureLoad(primitives,vec2<i32>(p.xy),0)","textureLoad(primitives,vec2<i32>(p.xy),i32(sample))")
}
