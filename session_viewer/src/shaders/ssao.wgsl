const MSAA: bool = false;
@group(3) @binding(0) var<storage,read_write> corrections: array<u32>;
@group(3) @binding(1) var<storage,read_write> edge_flags: array<atomic<u32>>;
@group(3) @binding(2) var<storage,read_write> edge_tiles: array<u32>;
@group(3) @binding(4) var history_ao: texture_2d<f32>;
@group(3) @binding(5) var history_depth: texture_2d<f32>;
struct Draw { vertices: u32, instances: atomic<u32>, first_vertex: u32, first_instance: u32 };
@group(3) @binding(3) var<storage,read_write> draw_edges: Draw;
struct Ambient {
    inverse_mvp: mat4x4<f32>,
    mvp: mat4x4<f32>,
    params: vec4<f32>,
    extent: vec4<f32>,
    near: vec4<f32>,
    near_x: vec4<f32>,
    near_y: vec4<f32>,
    ray: vec4<f32>,
    ray_x: vec4<f32>,
    ray_y: vec4<f32>,
    previous_mvp: mat4x4<f32>,
};
@group(0) @binding(0) var depth: texture_depth_2d;
@group(0) @binding(1) var<uniform> ambient: Ambient;
@group(0) @binding(2) var physical: texture_2d<u32>;
// Geometry access
@group(1) @binding(0) var linear: texture_2d<f32>;
@group(1) @binding(1) var radii: texture_2d<u32>;
@group(2) @binding(0) var occlusion: texture_2d<f32>;
@group(2) @binding(1) var linear_sampler: sampler;

fn fullscreen(i: u32) -> vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2<f32>(-1.0,-1.0), vec2<f32>(3.0,-1.0), vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> { return fullscreen(i); }

fn world(pixel: vec2<f32>, z: f32) -> vec3<f32> {
    let uv = pixel / ambient.params.zw;
    let p = ambient.inverse_mvp * vec4<f32>(uv.x*2.0-1.0, 1.0-uv.y*2.0, z, 1.0);
    return p.xyz / p.w;
}
fn ray_origin(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.near.xyz+ambient.near_x.xyz*pixel.x+ambient.near_y.xyz*pixel.y;
}
fn ray_direction(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.ray.xyz+ambient.ray_x.xyz*pixel.x+ambient.ray_y.xyz*pixel.y;
}
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    return textureLoad(depth,clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1),sample);
}
fn surface(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    let z = depth_sample(pixel,sample);
    if z>0.0 { return vec4<f32>(world(pixel,z),1.0); }
    let near = ray_origin(pixel);
    let direction = ray_direction(pixel);
    if direction.z>=-1e-7 || near.z<ambient.params.x { return vec4<f32>(0.0); }
    return vec4<f32>(near+direction*((ambient.params.x-near.z)/direction.z),1.0);
}
fn full_pixel(pixel: vec2<f32>) -> vec2<f32> {
    return floor(pixel/ambient.extent.yz*ambient.params.zw)+0.5;
}
fn position_at(pixel: vec2<f32>, along: f32) -> vec3<f32> {
    return ray_origin(pixel)+ray_direction(pixel)*abs(along);
}
struct DepthRadius {
    @location(0) depth: f32,
    @location(1) radius: u32,
};
@fragment fn fs_prepare(@builtin(position) pixel: vec4<f32>) -> DepthRadius {
    let at = full_pixel(pixel.xy);
    let ray = ray_direction(at);
    let origin = ray_origin(at);
    let z = depth_sample(at,0);
    if z==0.0 {
        if ray.z>=-1e-7 || origin.z<ambient.params.x { return DepthRadius(0.0,0u); }
        return DepthRadius(-(ambient.params.x-origin.z)/ray.z,0u);
    }
    let along = dot(world(at,z)-origin,ray)/dot(ray,ray);
    let radius = triangle_at(at,0).w/ambient.params.y;
    let bits = (pack2x16float(vec2<f32>(radius,0.0))+2u)>>2u;
    let coord = vec2<u32>(pixel.xy);
    let encoded = (bits<<4u)|(coord.x&3u)|((coord.y&3u)<<2u);
    return DepthRadius(along,encoded);
}

fn radius_at(coord: vec2<i32>) -> f32 {
    return unpack2x16float((textureLoad(radii,coord,0).x>>4u)<<2u).x*ambient.params.y;
}
struct Hit {
    point: vec3<f32>,
    along: f32,
    radius: f32,
};
fn sample_hit(at: vec2<f32>, offset: vec2<f32>, level: i32) -> Hit {
    let dims = vec2<i32>(textureDimensions(linear,level));
    let coord = vec2<i32>(floor((at+offset)/ambient.params.zw*ambient.extent.yz))>>vec2<u32>(u32(level));
    if any(coord<vec2<i32>(0)) || any(coord>=dims) { return Hit(vec3<f32>(0.0),0.0,0.0); }
    let along = textureLoad(linear,coord,level).x;
    let encoded = textureLoad(radii,coord,level).x;
    let mask = (1u<<u32(level))-1u;
    let offset_coord = vec2<u32>(encoded&3u,(encoded>>2u)&3u)&vec2<u32>(mask);
    let source_coord = (vec2<u32>(coord)<<vec2<u32>(u32(level)))+offset_coord;
    let pixel = full_pixel(vec2<f32>(source_coord)+0.5);
    return Hit(position_at(pixel,along),along,unpack2x16float((encoded>>4u)<<2u).x*ambient.params.y);
}
fn ground_contact(at: vec2<f32>, p: vec3<f32>, pixel_world: f32, noise: f32) -> f32 {
    let dims = vec2<i32>(textureDimensions(occlusion));
    let tile = clamp(vec2<i32>(at/ambient.params.zw*vec2<f32>(dims)),vec2<i32>(0),dims-1);
    if textureLoad(occlusion,tile,0).x==0.0 { return 0.0; }
    let radius_px = min(ambient.params.y*6.0/max(pixel_world,1e-6),64.0);
    var sum = 0.0;
    for (var direction=0u; direction<24u; direction++) {
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
    return clamp(sum*(2.6/24.0),0.0,0.65);
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
    return ground_contact(at,p,pixel_world,noise);
}
fn filtered_ground(at: vec2<f32>) -> f32 {
    return textureSampleLevel(occlusion,linear_sampler,at/ambient.params.zw,0.0).x;
}
// Integral of cosine-weighted visibility over a horizon slice (Jimenez et al., GTAO).
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
    let normal = normal_at(at,p,0);
    let view = -normalize(ray_direction(at));
    let radius_px = min(radius*8.0/pixel_world,128.0);
    let bias = radius*0.025;
    var occluded = 0.0;
    for (var slice=0u; slice<4u; slice++) {
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
    return clamp(occluded*(1.08/4.0),0.0,0.65);
}
fn weight_at(q: vec2<i32>, p: vec3<f32>, n: vec3<f32>, geometry: bool, radius: f32) -> f32 {
    let along = textureLoad(linear,q,0).x;
    if along==0.0 || (along>0.0)!=geometry { return 0.0; }
    let delta = position_at(full_pixel(vec2<f32>(q)+0.5),along)-p;
    let separation = abs(dot(delta,n));
    let tolerance = max(radius*0.024,length(delta)*0.08);
    let ground_weight = select(1.0-smoothstep(0.0,radius,length(delta.xy)),1.0,geometry);
    return exp(-separation/max(tolerance,1e-6))*ground_weight;
}
fn denoise(pixel: vec2<f32>, axis: vec2<i32>) -> vec3<f32> {
    let coord = vec2<i32>(pixel);
    var values: array<f32, 7>;
    var total = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        values[i] = textureLoad(occlusion,q,0).x;
        total += values[i];
    }
    if total==0.0 { return vec3<f32>(0.0); }
    let along = textureLoad(linear,coord,0).x;
    if along==0.0 { return vec3<f32>(0.0); }
    let at = full_pixel(pixel);
    let p = position_at(at,along);
    var n = vec3<f32>(0.0,0.0,1.0);
    if along>0.0 { n = normal_at(at,p,0); }
    let radius = select(ambient.params.y,radius_at(coord),along>0.0);
    var sum = 0.0;
    var weights = 0.0;
    var low = 1.0;
    var high = 0.0;
    for (var i=0; i<7; i++) {
        let q = clamp(coord+axis*(i-3),vec2<i32>(0),vec2<i32>(ambient.extent.yz)-1);
        let weight = exp(-f32((i-3)*(i-3))*0.12)*weight_at(q,p,n,along>0.0,radius);
        if weight>0.05 { low=min(low,values[i]); high=max(high,values[i]); }
        sum += values[i]*weight;
        weights += weight;
    }
    return vec3<f32>(sum/max(weights,1e-6),low,high);
}
@fragment fn fs_filter_x(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    return denoise(pixel.xy,vec2<i32>(1,0)).x;
}
@fragment fn fs_filter_y(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let current = denoise(pixel.xy,vec2<i32>(0,1));
    if ambient.near.w==0.0 || current.z==0.0 { return current.x; }
    let along = textureLoad(linear,vec2<i32>(pixel.xy),0).x;
    if along==0.0 { return current.x; }
    let p = position_at(full_pixel(pixel.xy),along);
    let clip = ambient.previous_mvp*vec4<f32>(p,1.0);
    let uv = vec2<f32>(clip.x,-clip.y)/clip.w*0.5+0.5;
    if clip.w<=0.0 || any(uv<=vec2<f32>(0.0)) || any(uv>=vec2<f32>(1.0)) { return current.x; }
    let expected = clip.z/clip.w;
    let dims = vec2<i32>(textureDimensions(history_depth));
    let base = vec2<i32>(floor(uv*vec2<f32>(dims)-0.5));
    var valid = false;
    for(var i=0; i<4; i++) {
        let q = clamp(base+vec2<i32>(i%2,i/2),vec2<i32>(0),dims-1);
        let depth = textureLoad(history_depth,q,0).x;
        valid = valid || (depth!=0.0 && (depth>0.0)==(along>0.0)
            && abs(abs(depth)-expected)<max(abs(expected)*0.0025,1e-6));
    }
    if !valid { return current.x; }
    let previous = textureSampleLevel(history_ao,linear_sampler,uv,0.0).x;
    return mix(current.x,clamp(previous,current.y,current.z),0.8);
}
@fragment fn fs_history(@builtin(position) pixel: vec4<f32>) -> @location(0) f32 {
    let at = pixel.xy/ambient.extent.xw*ambient.params.zw;
    let p = surface(at,0);
    if p.w==0.0 { return 0.0; }
    let clip = ambient.mvp*vec4<f32>(p.xyz,1.0);
    return select(-clip.z/clip.w,clip.z/clip.w,depth_sample(at,0)>0.0);
}
fn source_base(at: vec2<f32>) -> vec2<i32> {
    let base = vec2<i32>(floor(at/ambient.params.zw*ambient.extent.yz-0.5));
    return base-select(vec2<i32>(0),vec2<i32>(1),at<full_pixel(vec2<f32>(base)+0.5));
}
fn reconstruct(at: vec2<f32>, sample: i32, values: vec4<f32>) -> f32 {
    let base = source_base(at);
    let p = surface(at,sample);
    if p.w==0.0 { return 0.0; }
    let n = normal_at(at,p.xyz,sample);
    let geometry = depth_sample(at,sample)>0.0;
    let radius = select(ambient.params.y,triangle_at(at,sample).w,geometry);
    let origin = full_pixel(vec2<f32>(base)+0.5);
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
    let first = reconstruct(pixel.xy,0,values);
    if !MSAA { return first; }
    let center = depth_sample(pixel.xy,0);
    var ao = vec4<f32>(first);
    var different = false;
    for (var i=1; i<4; i++) {
        let z = depth_sample(pixel.xy,i);
        if (z>0.0)!=(center>0.0) || abs(z-center)>max(center*0.001,1e-7) {
            ao[i] = reconstruct(pixel.xy,i,values);
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
