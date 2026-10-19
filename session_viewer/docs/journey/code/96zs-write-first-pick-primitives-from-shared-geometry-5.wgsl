@group(2) @binding(0) var<uniform> crop:Crop;
struct MeshPickOutput{@builtin(position) position:vec4<f32>,@location(0) world:vec3<f32>,@location(1) @interpolate(flat) flags:u32,@location(2) @interpolate(flat) primitive:u32}
@vertex fn pick_vertex(@builtin(vertex_index) occurrence:u32)->MeshPickOutput{
    let input=mesh_vertex(occurrence);let placed=geometry(input.position,input.colour,input.normal,object_settings);
    return MeshPickOutput(crop.transform*placed.position,placed.world,placed.clip_flags,(object_settings.selection.x>>1u)+occurrence/3u+1u);
}
struct PickFragment{@location(0) owner:vec2<u32>,@location(1) primitive:vec2<u32>}
@fragment fn pick_fragment(input:MeshPickOutput)->PickFragment{
    if((input.flags&32768u)!=0u){discard;}if(clip_active()&&clip_cut(input.flags,input.world)){discard;}
    return PickFragment(vec2<u32>(select(0u,object_settings.selection.y,crop.mode==0u),0u),vec2<u32>(input.primitive&0xffffu,input.primitive>>16u));
}
