// --8<-- [start:step-8]
// Uniform: camera matrices, ground height, radius, canvas size.
struct Ambient {
    inverse_mvp: mat4x4<f32>, // clip space back to world
    mvp: mat4x4<f32>, // camera matrix
    params: vec4<f32>, // ground z, largest contact radius, canvas width, height
};
@group(0) @binding(0) var depth: texture_depth_2d; // scene depth
@group(0) @binding(1) var<uniform> ambient: Ambient; // settings
@group(0) @binding(2) var physical: texture_2d<f32>; // depth slope and triangle id per pixel
// Projected triangles, six vec4 each; word 4.w is the contact radius.
@group(0) @binding(3) var<storage, read> triangles: array<vec4<f32>>;
@group(1) @binding(0) var occlusion: texture_2d<f32>; // occlusion from the previous pass

// Fullscreen triangle.
@vertex fn vs_main(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let p = array<vec2<f32>, 3>(vec2<f32>(-1.0,-1.0), vec2<f32>(3.0,-1.0), vec2<f32>(-1.0,3.0));
    return vec4<f32>(p[i],0.0,1.0);
}
// World position of a pixel at depth `z`.
fn world(pixel: vec2<f32>, z: f32) -> vec3<f32> {
    let uv = pixel / ambient.params.zw;
    let p = ambient.inverse_mvp * vec4<f32>(uv.x*2.0-1.0, 1.0-uv.y*2.0, z, 1.0);
    return p.xyz / p.w;
}
// Scene depth at a pixel, clamped to the canvas.
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    return textureLoad(depth, xy, sample);
}
// Scene depth at sample 0.
fn depth_at(pixel: vec2<f32>) -> f32 { return depth_sample(pixel,0); }
// Contact radius of the object under a pixel, or 0.
fn contact_radius(pixel: vec2<f32>, sample: i32) -> f32 {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    let packed = pack2x16float(textureLoad(physical,xy,sample).zw);
    let words = vec2<u32>(packed&0xffffu,packed>>16u);
    if any(words<vec2<u32>(0x400u)) { return 0.0; }
    let address = words-vec2<u32>(0x400u);
    let primitive = address.x|(address.y<<14u);
    if primitive==0u || primitive>arrayLength(&triangles)/6u { return 0.0; }
    return triangles[(primitive-1u)*6u+4u].w;
}
// world point under a pixel; w = 0 for none
fn surface(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    let z = depth_sample(pixel,sample);
    if z > 0.0 { return vec4<f32>(world(pixel,z),1.0); }
    let near = world(pixel,1.0);
    let direction = world(pixel,0.5)-near;
    if direction.z >= -1e-7 || near.z < ambient.params.x { return vec4<f32>(0.0); }
    let t = (ambient.params.x-near.z)/direction.z;
    return vec4<f32>(near+direction*t,1.0);
}
// Surface normal at a pixel, from the stored slope or neighbouring pixels.
fn normal_at(pixel: vec2<f32>, p: vec3<f32>, sample: i32) -> vec3<f32> {
    let z = depth_sample(pixel,sample);
    if z == 0.0 { return vec3<f32>(0.0,0.0,1.0); }
    let encoded = textureLoad(physical,vec2<i32>(pixel),sample).xy;
    // stored slope available
    if all(abs(encoded)<vec2<f32>(65504.0)) {
        let gradient = encoded/65536.0;
        let dx = world(pixel+vec2<f32>(1.0,0.0),z+gradient.x)-p;
        let dy = world(pixel+vec2<f32>(0.0,1.0),z+gradient.y)-p;
        let cross_n = cross(dx,dy);
        if dot(cross_n,cross_n)>1e-24 {
            let n = normalize(cross_n);
            return select(-n,n,dot(n,world(pixel,1.0)-p)>0.0);
        }
    }
    let a = surface(pixel+vec2<f32>(1.0,0.0),sample);
    let b = surface(pixel-vec2<f32>(1.0,0.0),sample);
    let c = surface(pixel+vec2<f32>(0.0,1.0),sample);
    let d = surface(pixel-vec2<f32>(0.0,1.0),sample);
    // nearest neighbour per axis, so edges do not mix surfaces
    let dx = select(p-b.xyz,a.xyz-p,a.w>0.0 && (b.w==0.0 || distance(a.xyz,p)<distance(b.xyz,p)));
    let dy = select(p-d.xyz,c.xyz-p,c.w>0.0 && (d.w==0.0 || distance(c.xyz,p)<distance(d.xyz,p)));
    let cross_n = cross(dx,dy);
    let n = cross_n / max(length(cross_n),1e-12);
    return select(-n,n,dot(n,world(pixel,1.0)-p)>0.0);
}
// Occlusion texture size: the canvas, at most 1920 px wide.
fn ao_size() -> vec2<f32> {
    let scale = min(1.0,1920.0/max(ambient.params.z,ambient.params.w));
    return ceil(ambient.params.zw*scale);
}
// Shadow on the virtual ground from geometry just above it, 0..0.65.
fn ground_contact(at: vec2<f32>, p: vec3<f32>, noise: f32) -> f32 {
    let clip = ambient.mvp*vec4<f32>(p,1.0);
    let pixel_world = distance(world(at+vec2<f32>(1.0,0.0),clip.z/clip.w),p);
    let search_radius = ambient.params.y*6.0;
    let radius_px = min(search_radius/max(pixel_world,1e-6),64.0);
    var sum = 0.0;
    // 16 directions, 8 steps each
    for (var direction=0u; direction<16u; direction++) {
        let angle = (f32(direction)+noise)*0.39269908;
        let axis = vec2<f32>(cos(angle),sin(angle));
        let jitter = fract(noise+f32(direction)*0.618034);
        var horizon = 0.0;
        for (var step=0u; step<8u; step++) {
            let fraction = (f32(step)+0.5+jitter*0.5)/8.0;
            let qxy = floor(at+axis*max(1.5,radius_px*fraction*fraction))+0.5;
            if any(qxy<vec2<f32>(0.0)) || any(qxy>=ambient.params.zw) { continue; }
            let z = depth_at(qxy);
            if z==0.0 { continue; }
            let radius = contact_radius(qxy,0)*6.0;
            if radius<=0.0 { continue; }
            let delta = world(qxy,z)-p;
            let span = length(delta);
            let height = delta.z;
            if height<=0.0 || height>=radius { continue; }
            let elevation = max(height/max(span,1e-6)-0.04,0.0);
            let radial = 1.0-smoothstep(radius*0.1,radius,length(delta.xy));
            let vertical = 1.0-smoothstep(0.0,radius,height);
            let weight = radial*vertical*vertical;
            horizon = max(horizon,elevation*weight);
        }
        sum += horizon;
    }
    return clamp(sum/16.0*2.6,0.0,0.65);
}
// Occlusion at one pixel: ground shadow, or hemisphere samples on geometry.
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    let at = floor(pixel.xy/ao_size()*ambient.params.zw)+0.5;
    let center = surface(at,0);
    if center.w == 0.0 { return vec4<f32>(0.0); }
    let p = center.xyz;
    let n = normal_at(at,p,0);
    let ground = depth_at(at)==0.0;
    // per-pixel noise so the sample pattern does not tile
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    if ground { return vec4<f32>(ground_contact(at,p,noise),0.0,0.0,1.0); }
    let local_radius = contact_radius(at,0);
    if local_radius<=0.0 { return vec4<f32>(0.0); }
    // tangent frame around the normal
    let up = select(vec3<f32>(0.0,1.0,0.0),vec3<f32>(1.0,0.0,0.0),abs(n.y)>0.9);
    let tangent = normalize(cross(up,n));
    let bitangent = cross(n,tangent);
    let view = normalize(world(at,1.0)-p);
    let bias = local_radius*0.025*(1.0+3.0*(1.0-max(dot(n,view),0.0)));
    var near_occ = 0.0;
    var far_occ = 0.0;
    // 64 near samples, then 64 far ones
    for (var i=0u; i<128u; i++) {
        let j = f32(i%64u);
        let far = i>=64u;
        let radius = local_radius*select(1.0,8.0,far);
        let angle = j*2.39996323+noise*6.2831853;
        let z = 0.2+0.8*(j+0.5)/64.0;
        let xy = sqrt(1.0-z*z);
        let direction = tangent*(cos(angle)*xy)+bitangent*(sin(angle)*xy)+n*z;
        // sample distance, shuffled against the angle
        let fraction = (f32((i*23u)%64u)+0.5)/64.0;
        let distance = radius*mix(select(0.1,0.25,far),1.0,fraction*fraction);
        let probe = p+direction*distance;
        let clip = ambient.mvp*vec4<f32>(probe,1.0);
        if clip.w<=0.0 { continue; }
        let uv = vec2<f32>(clip.x/clip.w*0.5+0.5,0.5-clip.y/clip.w*0.5);
        if any(uv<vec2<f32>(0.0)) || any(uv>=vec2<f32>(1.0)) { continue; }
        let qxy = floor(uv*ambient.params.zw)+0.5;
        let qz = depth_at(qxy);
        if qz==0.0 || all(qxy==at) { continue; }
        let q = world(qxy,qz);
        let delta = q-p;
        // blocked when the hit is in front of the probe and above the surface
        let ray = normalize(world(qxy,1.0)-probe);
        let blocked = dot(q-probe,ray)>max(bias,distance*0.015)
            && dot(delta,n)>length(delta)*0.07+bias;
        let weight = 1.0-smoothstep(radius*0.5,radius*2.0,length(delta));
        if blocked {
            if far { far_occ+=weight; } else { near_occ+=weight; }
        }
    }
    return vec4<f32>(clamp(max(near_occ/64.0,far_occ/64.0*0.75)*1.08,0.0,0.65),0.0,0.0,1.0);
}
// Blur along x, keeping edges.
@fragment fn fs_filter_x(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(reconstruct(floor(pixel.xy/ao_size()*ambient.params.zw)+0.5,vec2<i32>(1,0),0),0.0,0.0,1.0);
}
// Blur along y, keeping edges.
@fragment fn fs_filter_y(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(reconstruct(floor(pixel.xy/ao_size()*ambient.params.zw)+0.5,vec2<i32>(0,1),0),0.0,0.0,1.0);
}
// Darken the scene: black with the occlusion as alpha.
@fragment fn fs_composite(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0,0.0,0.0,reconstruct(pixel.xy,vec2<i32>(0),0));
}
// Occlusion at a pixel, blurred over neighbours on the same surface.
fn reconstruct(pixel: vec2<f32>, axis: vec2<i32>, depth_index: i32) -> f32 {
    let center = surface(pixel,depth_index);
    if center.w == 0.0 { return 0.0; }
    let n = normal_at(pixel,center.xyz,depth_index);
    let geometry = depth_sample(pixel,depth_index)>0.0;
    let local_radius = select(ambient.params.y,contact_radius(pixel,depth_index),geometry);
    let dims = vec2<i32>(textureDimensions(occlusion));
    let source = pixel/ambient.params.zw*vec2<f32>(dims)-0.5;
    let filtering = any(axis!=vec2<i32>(0));
    let base = vec2<i32>(floor(source+0.5));
    var sum = 0.0;
    var weight = 0.0;
    // 3x3 block, or 13 along one axis
    for (var i=0; i<select(9,13,filtering); i++) {
        let offset = select(vec2<i32>(i%3-1,i/3-1),axis*(i-6),filtering);
        let q = clamp(base+offset,vec2<i32>(0),dims-1);
        let full = floor((vec2<f32>(q)+0.5)/vec2<f32>(dims)*ambient.params.zw)+0.5;
        let sample = surface(full,0);
        let delta = sample.xyz-center.xyz;
        let separation = abs(dot(delta,n));
        let tolerance = max(local_radius*0.024,length(delta)*0.08);
        let distance = vec2<f32>(q)-source;
        let spatial = exp(-dot(distance,distance)*select(1.5,0.04,filtering));
        let ground_weight = select(
            1.0-smoothstep(0.0,local_radius,length(delta.xy)),1.0,geometry);
        let w = spatial*exp(-separation/max(tolerance,1e-6))*ground_weight;
        let valid = select(0.0,w,sample.w>0.0 && ((depth_at(full)>0.0) == geometry));
        sum += textureLoad(occlusion,q,0).r*valid;
        weight += valid;
    }
    return sum/max(weight,1e-6);
}
// --8<-- [end:step-8]
