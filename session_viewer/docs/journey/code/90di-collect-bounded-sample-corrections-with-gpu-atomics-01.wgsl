e)+0.5);
    let blend = (at-origin)/(full_pixel(vec2<f32>(base)+1.5)-origin);
    var sum = 0.0;
    var weights = 0.0;
    for (var i=0; i<4; i++) {
        let offset = vec2<i32>(i%2,i/2);
        let q = clamp(base+offset,vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let spatial = mix(1.0-blend,blend,vec2<f32>(offset));
        let w = spatial.x*spatial.y*weight_at(q,p.xyz,n,geometry,radius);
        sum += values[i]*w;
        weights += w;
    }
    return sum/max(weights,1e-6);
}

@fragment fn fs_upsample(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let xy = vec2<u32>(pixel.xy);
    if MSAA { corrections[xy.y*u32(ambient.params.z)+xy.x] = 0u; }
    let base_coord = source_base(pixel.xy);
    var values = vec4<f32>(0.0);
    for (var i=0; i<4; i++) {
        let q = clamp(base_coord+vec2<i32>(i%2,i/2),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
    }
    if all(values==vec4<f32>(0.0)) { return 0.0; }
    let center = depth_sample(pixel.xy,0);
    let first = reconstruct(pixel.xy,0,center,values);
    if !MSAA { return first; }
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) {
            ao[i] = reconstruct(pixel.xy,i,z,values);
            different = different || ao[i]!=first;
        }
    }
    if !different { return first; }
    let base = round(min(min(ao.x,ao.y),min(ao.z,ao.w))*255.0)/255.0;
    let packed = pack4x8unorm((ao-vec4<f32>(base))/max(1.0-base,1e-6));
    if packed!=0u {
        corrections[xy.y*u32(ambient.params.z)+xy.x] = packed;
        let tile_width = (u32(ambient.params.z)+15u)/16u;
        let tile = (xy.y/16u)*tile_width+xy.x/16u;
        if atomicOr(&edge_flags[tile],1u)==0u {
            let index = atomicAdd(&draw_edges.instances,4u)/4u;
            edge_tiles[index] = tile;
        }
    }
    return base;
}
