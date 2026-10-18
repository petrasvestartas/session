struct FragmentOutput{
    @location(0) color:vec4<f32>,
    @location(1) primitive:vec2<u32>,
}
@fragment
fn fragment(input:VertexOutput,@builtin(front_facing) front:bool)->FragmentOutput{
    return FragmentOutput(shade(input,front),vec2<u32>(input.primitive&0xffffu,input.primitive>>16u));
}

@group(3) @binding(0) var<storage, read> mesh_values: