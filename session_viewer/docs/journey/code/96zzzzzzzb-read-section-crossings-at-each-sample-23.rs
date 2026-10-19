e}
 }
}

fn count_source(source:&str,samples:u32)->String {
 let source=crate::clipping_gpu::source(source,2,0);
 if samples==1 {source}else{source.replace("@location(0) world_pos:","@location(0) @interpolate(perspective,sample) world_pos:")}
}
fn colour_source(samples:u32)->String {
 let source=include_str!("cap_color.wgsl");if samples==1 {return source.into();}
 source.replace("var counts:texture_2d<f32>;","var counts:texture_multisampled_2d<f32>;")
 .replace("const SAMPLES:u32=1u;","const SAMPLES:u32=4u;")
 .replace("textureLoad(counts,at,0)","textureLoad(counts,at,i32(sample))")
 .replace("@location(0) @interpolate(flat) plane: u32,","@location(0) @interpolate(flat) plane: u32,\n    @location(1) @interpolate(linear,sample) pixel:vec2f,")
 .replace("out.plane = plane;","out.plane = plane;out.pixel=(out.pos.xy/out.pos.w*vec2f(0.5,-0.5)+0.5)*cap_frame.size;")
 .replace("in.pos.xy","in.pixel")
 .replace("let samples = clip_samples(t, gradient);","let samples = select(0u,0xffffffffu,t>=0.);")

}
