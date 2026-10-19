lse{2},0)
}

fn multisampled(mut source:String,stride:u64)->String {
    source=source.replace("var finite_depth:texture_depth_2d;","var finite_depth:texture_depth_multisampled_2d;")
        .replace("var finite_primitive:texture_2d<u32>;","var finite_primitive:texture_multisampled_2d<u32>;")
        .replace("textureLoad(finite_depth,at,0)","textureLoad(finite_depth,at,i32(sample))")
        .replace("textureLoad(finite_primitive,at,0)","textureLoad(finite_primitive,at,i32(sample))")
        .replace("@interpolate(linear) finite_pixel:","@interpolate(linear,sample) finite_pixel:")
        .replace("@interpolate(linear) pixel:","@interpolate(linear,sample) pixel:");
    if stride==32 {
        source=source.replace("@location(10) @interpolate(flat) finite_depths:vec2<f32>,",
            "@location(10) @interpolate(flat) finite_depths:vec2<f32>,\n    @location(11) @interpolate(linear,sample) finite_pixel:vec2<f32>,")
            .replace("colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0))", "colour, 0.0,flags,vec4<f32>(0.0),vec2<f32>(0.0),vec2<f32>(0.0))")
            .replace("Output(position, colour, radius,flags,axis,depths)","Output(position, colour, radius,flags,axis,depths,finite_screen(position))")
            .replace("input.position.xy","input.finite_pixel");
    }
    source
}
