2(level))-1u;
    let offset_coord = vec2<u32>(encoded&3u,(encoded>>2u)&3u)&vec2<u32>(mask);
    let source_coord = (vec2<u32>(coord)<<vec2<u32>(u32(level)))+offset_coord;
    let pixel = full_pixel(vec2<f32>(source_coord)+0.5);
    return Hit(position_at(pixel,along),along,unpack2x16float((encoded>>4u)<<2u).x*ambient.params.y);
}
fn stride() -> u32 {
    return select(1u,2u,ambient.near.w!=0.0);
}
fn phase(pixel: vec2<f32>, stride: u32) -> u32 {
    return (u32(pixel.x)+u32(pixel.y))%stride;
}

fn ground_contact(at: vec2<f32>, p: vec3<f32>, pixel_world: f32, noise: f32, phase: u32) -> f32 {
    let dims = vec2<i32>(textureDimensions(occlusion));
    let tile = clamp(vec2<i32>(at/ambient.params.zw*vec2<f32>(dims)),vec2<i32>(0),dims-1);
    if textureLoad(occlusion,tile,0).x==0.0 { return 0.0; }
    let radius_px = min(ambient.params.y*6.0/max(pixel_world,1e-6),64.0);
    let stride = stride();
    var sum = 0.0;
    for (var index=0u; index<24u/stride; index++) {
        let direction = index*stride+phase;
        let angle = (f32(direction)+noise)*0.261799388;
        let axis = vec2<f32>(cos(angle),sin(angle));
        var horizon = 0.0;
        for (var step=0u; step<6u; step++) {
            let jitter = fract(noise+f32(direction)*0.618034);
            let fraction = (f32(step)+0.5+jitter*0.5)/6.0;
            let span = max(1.5,radius_px*fraction*fraction);
            let scaled = span*ambient.extent.y/ambient.params.z;
            let level = i32(scaled>=8.0)+i32(scaled>=16.0);
            let hit = sample_hit(at,axis*span,level);
            let radius = hit.radius*6.0;
            if hit.along<=0.0 || radius<=0.0 { continue; }
            let delta = hit.point-p;
            if delta.z<=0.0 || delta.z>=radius { continue; }
            let elevation = max(delta.z/max(length(delta),1e-6)-0.04,0.0);
            let radial = 1.0-smoothstep(radius*0.1,radius,length(delta.xy));
            let vertical = 1.0-smoothstep(0.0,radius,delta.z);
            horizon = max(horizon,elevation*radial*vertical*vertical);
        }
        sum += horizon;
    }
    return clamp(sum*(2.6*f32(stride)/24.0),0.0,0.65);
}
@fragment fn fs_ground(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let at = pixel.xy/ambient.extent.xw*ambient.params.zw;
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    if ray.z>=-1e-7 || origin.z<ambient.params.x { return 0.0; }
    let along = (ambient.params.x-origin.z)/ray.z;
    let p = origin+ray*along;
    let pixel_world = max(length(ambient.near_x.xyz+ambient.ray_x.xyz*along),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    return ground_contact(at,p,pixel_world,noise,phase(pixel.xy,stride()));
}
fn filtered_ground(at: vec2<f32>) -> f32 {
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}
