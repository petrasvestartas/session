    return output;
}
fn coverage(input:Output)->f32 {return input.colour.a*clamp(input.half_width+0.5-abs(input.side),0.0,1.0);}
@vertex
fn vertex(@location(0) start:vec3<f32>,@location(1) radius:f32,@location(2) end:vec3<f32>,@location(3) packed:u32,@builtin(vertex_index) v:u32,@builtin(instance_index) i:u32)->Output {return geometry(start,end,radius,packed,v,i);}
@fragment
fn fragment(input:Output)->@location(0) vec4<f32> {return vec4<f32>(input.colour.rgb,coverage(input));}
