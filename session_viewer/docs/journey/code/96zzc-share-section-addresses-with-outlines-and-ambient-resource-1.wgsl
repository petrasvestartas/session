fn physical_primitive(primitive:u32)->vec2<u32>{
    return vec2<u32>(primitive&0xffffu,primitive>>16u);
}
fn physical_address(halves:vec2<u32>)->u32{
    return halves.x|(halves.y<<16u);
}
