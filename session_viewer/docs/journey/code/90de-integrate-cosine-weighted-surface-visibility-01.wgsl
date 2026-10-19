mbient.params.x { return 0.0; }
    let along = (ambient.params.x-origin.z)/ray.z;
    let p = origin+ray*along;
    let pixel_world = max(length(ambient.near_x.xyz+ambient.ray_x.xyz*along),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    return ground_contact(at,p,pixel_world,noise,phase(pixel.xy,stride()));
}
fn filtered_ground(at: vec2<f32>) -> f32 {
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}

fn arc(h: f32, n: f32) -> f32 {
    return (cos(n)+2.0*h*sin(n)-cos(2.0*h-n))*0.25;
}
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let coord = vec2<i32>(pixel.xy);
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return 0.0; }
    let at = full_pixel(pixel.xy);
    let p = position_at(at,along);
    let step_x = ambient.near_x.xyz+ambient.ray_x.xyz*abs(along);
    let step_y = ambient.near_y.xyz+ambient.ray_y.xyz*abs(along);
    let pixel_world = max(length(step_x),1e-6);
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    if along<0.0 { return filtered_ground(at); }
    let radius = radius_at(coord);
    if radius<=0.0 { return 0.0; }
    let normal = normal_half(coord);
    let view = -normalize(ray_direction(at));
    let radius_px = min(radius*8.0/pixel_world,128.0);
    let bias = radius*0.025;
    let stride = stride();
    let phase = phase(pixel.xy,stride);
    var occluded = 0.0;
    for (var index=0u; index<4u/stride; index++) {
        let slice = index*stride+phase;
        let angle = (f32(slice)+noise)*0.785398163;
        let axis = vec2<f32>(cos(angle),sin(angle));
        let tangent = step_x*axis.x+step_y*axis.y;
        let side = normalize(tangent-view*dot(tangent,view));
        let plane_normal = cross(side,view);
        let projected = normal-plane_normal*dot(normal,plane_normal);
        let normal_length = length(projected);
        let n = atan2(dot(projected,side),dot(projected,view));
        var slice_occ = 0.0;
        for (var sign=-1; sign<=1; sign+=2) {
            let signed_n = n*f32(sign);
            let low = cos(signed_n+1.570796327);
            var horizon = low;
            for (var step=0u; step<6u; step++) {
                let fraction = (f32(step)+0.5)/6.0;
                let step_length = max(1.5,radius_px*fraction*fraction);
                let scaled = step_length*ambient.extent.y/ambient.params.z;
                let level = i32(scaled>=8.0)+i32(scaled>=16.0);
                let hit = sample_hit(at,axis*f32(sign)*step_length,level);
                if hit.along<=0.0 { continue; }
                let delta = hit.point-p;
                let span = max(length(delta),1e-6);
                if dot(delta,normal)<=span*0.07+bias { continue; }
                let weight = max(1.0-smoothstep(radius*0.5,radius*2.0,span),0.75*(1.0-smoothstep(radius*4.0,radius*16.0,span)));
                horizon = max(horizon,mix(low,dot(delta/span,view),weight));
            }
            let h = acos(clamp(horizon,-1.0,1.0));
            slice_occ += arc(signed_n+1.570796327,signed_n)-arc(h,signed_n);
        }
        occluded += normal_length*slice_occ;
    }
    return clamp(occluded*(1.08*f32(stride)/4.0),0.0,0.65);
}
