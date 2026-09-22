// Uniform: camera matrices, ground height, radius, canvas size.
struct Ambient {
    inverse_mvp: mat4x4<f32>, // clip space back to world
    mvp: mat4x4<f32>, // camera matrix
    params: vec4<f32>, // ground z, largest contact radius, canvas width, height
};
@group(0) @binding(0) var depth: texture_depth_2d; // scene depth
@group(0) @binding(1) var<uniform> ambient: Ambient; // settings
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
// depth at a pixel
fn depth_at(pixel: vec2<f32>) -> f32 {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    return textureLoad(depth, xy, 0);
}
// A virtual receiver beneath the model: no giant quad, depth writes or pick IDs.
fn surface(pixel: vec2<f32>) -> vec4<f32> {
    let z = depth_at(pixel);
    if z > 0.0 { return vec4<f32>(world(pixel,z),1.0); }
    let near = world(pixel,1.0);
    let direction = world(pixel,0.5)-near;
    if direction.z >= -1e-7 || near.z < ambient.params.x { return vec4<f32>(0.0); }
    let t = (ambient.params.x-near.z)/direction.z;
    return vec4<f32>(near+direction*t,1.0);
}
// surface normal at a pixel from its neighbours
fn normal_at(pixel: vec2<f32>, p: vec3<f32>) -> vec3<f32> {
    if depth_at(pixel) == 0.0 { return vec3<f32>(0.0,0.0,1.0); }
    let a = surface(pixel+vec2<f32>(1.0,0.0));
    let b = surface(pixel-vec2<f32>(1.0,0.0));
    let c = surface(pixel+vec2<f32>(0.0,1.0));
    let d = surface(pixel-vec2<f32>(0.0,1.0));
    // nearest neighbour per axis, so edges do not mix surfaces
    let dx = select(p-b.xyz,a.xyz-p,a.w>0.0 && (b.w==0.0 || distance(a.xyz,p)<distance(b.xyz,p)));
    let dy = select(p-d.xyz,c.xyz-p,c.w>0.0 && (d.w==0.0 || distance(c.xyz,p)<distance(d.xyz,p)));
    let cross_n = cross(dx,dy);
    let n = cross_n / max(length(cross_n),1e-12);
    return select(-n,n,dot(n,world(pixel,1.0)-p)>0.0);
}
// Occlusion texture size: the canvas, at most 1920 px wide.
fn ao_size() -> vec2<f32> {
    let scale = min(0.5,960.0/max(ambient.params.z,ambient.params.w));
    return ceil(ambient.params.zw*scale);
}
// Shadow on the virtual ground from geometry just above it, 0..0.65.
fn ground_contact(at: vec2<f32>, p: vec3<f32>, noise: f32) -> f32 {
    let clip = ambient.mvp*vec4<f32>(p,1.0);
    let pixel_world = distance(world(at+vec2<f32>(1.0,0.0),clip.z/clip.w),p);
    let radius = ambient.params.y*8.0;
    let radius_px = clamp(radius/max(pixel_world,1e-6),4.0,180.0);
    var sum = 0.0;
    for (var direction=0u; direction<8u; direction++) {
        let angle = (f32(direction)+noise)*0.78539816;
        let axis = vec2<f32>(cos(angle),sin(angle));
        let jitter = fract(noise+f32(direction)*0.618034);
        var horizon = 0.0;
        for (var step=0u; step<4u; step++) {
            let fraction = (f32(step)+0.5+jitter*0.5)/4.0;
            let qxy = floor(at+axis*max(1.5,radius_px*fraction*fraction))+0.5;
            if any(qxy<vec2<f32>(0.0)) || any(qxy>=ambient.params.zw) { continue; }
            let z = depth_at(qxy);
            if z==0.0 { continue; }
            let delta = world(qxy,z)-p;
            let span = length(delta);
            let height = max(delta.z,0.0);
            let elevation = max(height/max(span,1e-6)-0.04,0.0);
            let radial = 1.0-smoothstep(radius*0.1,radius,length(delta.xy));
            let weight = radial*exp(-height/(radius*0.5));
            horizon = max(horizon,elevation*weight);
        }
        sum += horizon;
    }
    return clamp(sum/8.0*2.6,0.0,0.65);
}
// Occlusion at one pixel: ground shadow, or hemisphere samples on geometry.
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    let at = floor(pixel.xy/ao_size()*ambient.params.zw)+0.5;
    let center = surface(at);
    if center.w == 0.0 { return vec4<f32>(0.0); }
    let p = center.xyz;
    let n = normal_at(at,p);
    let ground = depth_at(at)==0.0;
    // A 4x4 tiled rotation is removed by the small bilateral filter. Unlike
    let tile = vec2<f32>(vec2<u32>(pixel.xy) & vec2<u32>(3u));
    let noise = fract(52.9829189*fract(dot(tile,vec2<f32>(0.06711056,0.00583715))));
    if ground { return vec4<f32>(ground_contact(at,p,noise),0.0,0.0,1.0); }
    // tangent frame around the normal
    let up = select(vec3<f32>(0.0,1.0,0.0),vec3<f32>(1.0,0.0,0.0),abs(n.y)>0.9);
    let tangent = normalize(cross(up,n));
    let bitangent = cross(n,tangent);
    let view = normalize(world(at,1.0)-p);
    let bias = ambient.params.y*0.025*(1.0+3.0*(1.0-max(dot(n,view),0.0)));
    var near_occ = 0.0;
    var far_occ = 0.0;
    // Sixteen local and sixteen broad sky samples, all at half resolution.
    for (var i=0u; i<32u; i++) {
        let j = f32(i%16u);
        let far = i>=16u;
        let radius = ambient.params.y*select(1.0,8.0,far);
        let angle = j*2.39996323+noise*6.2831853;
        let z = 0.2+0.8*(j+0.5)/16.0;
        let xy = sqrt(1.0-z*z);
        let direction = tangent*(cos(angle)*xy)+bitangent*(sin(angle)*xy)+n*z;
        // Permute sample lengths so elevations and distances are uncorrelated.
        let fraction = (f32((i*7u)%16u)+0.5)/16.0;
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
    return vec4<f32>(clamp(max(near_occ/16.0,far_occ/16.0*0.75)*1.08,0.0,0.65),0.0,0.0,1.0);
}
// Denoise at the small resolution so a broad, soft kernel stays cheap.
@fragment fn fs_filter(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    let at = floor(pixel.xy/ao_size()*ambient.params.zw)+0.5;
    return vec4<f32>(reconstruct(at,3),0.0,0.0,1.0);
}
// Darken the scene: black with the occlusion as alpha.
@fragment fn fs_composite(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0,0.0,0.0,reconstruct(pixel.xy,0));
}
// occlusion at a pixel from nearby depths
fn reconstruct(pixel: vec2<f32>, radius: i32) -> f32 {
    let center = surface(pixel);
    if center.w == 0.0 { return 0.0; }
    let n = normal_at(pixel,center.xyz);
    let geometry = depth_at(pixel)>0.0;
    let dims = vec2<i32>(textureDimensions(occlusion));
    let source = pixel/ambient.params.zw*vec2<f32>(dims)-0.5;
    let base = vec2<i32>(floor(source));
    var sum = 0.0;
    var weight = 0.0;
    // Four-tap depth-aware upsample, or a 7x7 Gaussian for the small denoise pass.
    let lo = select(0,-3,radius>0);
    let hi = select(1,3,radius>0);
    for (var y=lo; y<=hi; y++) {
        for (var x=lo; x<=hi; x++) {
            let q = clamp(base+vec2<i32>(x,y),vec2<i32>(0),dims-1);
            let full = floor((vec2<f32>(q)+0.5)/vec2<f32>(dims)*ambient.params.zw)+0.5;
            let sample = surface(full);
            let delta = sample.xyz-center.xyz;
            let separation = abs(dot(delta,n));
            let tolerance = max(ambient.params.y*0.024,length(delta)*0.08);
            let offset = vec2<f32>(q)-source;
            let tent = max(vec2<f32>(0.0),vec2<f32>(1.0)-abs(offset));
            let spatial = select(tent.x*tent.y,exp(-dot(offset,offset)*0.16),radius>0);
            let w = spatial*exp(-separation/max(tolerance,1e-6));
            let valid = select(0.0,w,sample.w>0.0 && ((depth_at(full)>0.0) == geometry));
            sum += textureLoad(occlusion,q,0).r*valid;
            weight += valid;
        }
    }
    return sum/max(weight,1e-6);
}
