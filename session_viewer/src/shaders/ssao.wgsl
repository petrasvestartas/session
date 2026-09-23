// Screen-space ambient occlusion: how much sky each pixel sees.

// Uniform: camera matrices, ground height, radius, canvas size.
struct Ambient {
    inverse_mvp: mat4x4<f32>, // clip space back to world
    mvp: mat4x4<f32>, // camera matrix
    params: vec4<f32>, // ground z, largest contact radius, canvas width, height
    quarter: vec4<f32>, // x: which quarter of the samples this frame adds, 0..3; yz: occlusion texels in use; w: 1 in a drag
    near: vec4<f32>, // world point of pixel (0,0) on the near plane
    near_x: vec4<f32>, // its step per pixel in x
    near_y: vec4<f32>, // and in y
    ray: vec4<f32>, // near plane to depth 0.5 at pixel (0,0)
    ray_x: vec4<f32>, // its step per pixel in x
    ray_y: vec4<f32>, // and in y
};
@group(0) @binding(0) var depth: texture_depth_2d; // scene depth
@group(0) @binding(1) var<uniform> ambient: Ambient; // settings
@group(0) @binding(2) var physical: texture_2d<u32>; // triangle index + 1 per pixel in 16-bit halves, 0 for none
// Projected triangles, six vec4 each; word 4 is the depth slope, nearest depth and contact radius.
@group(0) @binding(3) var<storage, read> triangles: array<vec4<f32>>;
@group(1) @binding(0) var occlusion: texture_2d<f32>; // occlusion from the previous pass
@group(1) @binding(1) var linear: texture_2d<f32>; // position along the pixel ray per occlusion pixel

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
// Near plane point of a pixel's ray.
fn ray_origin(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.near.xyz+ambient.near_x.xyz*pixel.x+ambient.near_y.xyz*pixel.y;
}
// Direction of a pixel's ray, near plane to depth 0.5.
fn ray_direction(pixel: vec2<f32>) -> vec3<f32> {
    return ambient.ray.xyz+ambient.ray_x.xyz*pixel.x+ambient.ray_y.xyz*pixel.y;
}
// Scene depth at a pixel, clamped to the canvas.
fn depth_sample(pixel: vec2<f32>, sample: i32) -> f32 {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    return textureLoad(depth, xy, sample);
}
// Scene depth at sample 0.
fn depth_at(pixel: vec2<f32>) -> f32 { return depth_sample(pixel,0); }
// Word 4 of the triangle under a pixel: depth slope, nearest depth, contact radius; zero for none.
fn triangle_at(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    let halves = textureLoad(physical,xy,sample).xy;
    let primitive = halves.x|(halves.y<<16u);

    if primitive==0u {
        return vec4<f32>(0.0);
    }

    // no record, past the storage limit: no slope, so normal_at fits one from the neighbours
    if primitive>arrayLength(&triangles)/6u {
        return vec4<f32>(1.0,1.0,0.0,0.0);
    }

    return triangles[(primitive-1u)*6u+4u];
}
// Contact radius of the object under a pixel, or 0.
fn contact_radius(pixel: vec2<f32>, sample: i32) -> f32 {
    return triangle_at(pixel,sample).w;
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
    // the triangle's depth slope; flat for the grid and points
    let gradient = triangle_at(pixel,sample).xy;
    if all(abs(gradient)<vec2<f32>(0.99951171875)) {
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
// Occlusion texels in use: the texture, at most 1920 px wide, or its top-left half in a drag.
fn ao_size() -> vec2<f32> {
    return ambient.quarter.yz;
}
// True when the occlusion has a texel per canvas pixel: a still frame at most 1920 px wide.
fn flat() -> bool {
    return all(ao_size()==ambient.params.zw);
}
// Occlusion from summed samples; y < 0 marks ground.
fn finalize(sums: vec2<f32>) -> f32 {
    let scale = 4.0/(ambient.quarter.x+1.0);
    if sums.y<0.0 { return clamp(sums.x*scale/16.0*2.6,0.0,0.65); }
    return clamp(max(sums.x*scale/64.0,sums.y*scale/64.0*0.75)*1.08,0.0,0.65);
}
// Shadow on the virtual ground from geometry just above it, summed over this frame's directions.
fn ground_contact(at: vec2<f32>, p: vec3<f32>, noise: f32) -> f32 {
    let clip = ambient.mvp*vec4<f32>(p,1.0);
    let pixel_world = distance(world(at+vec2<f32>(1.0,0.0),clip.z/clip.w),p);
    let search_radius = ambient.params.y*6.0;
    let radius_px = min(search_radius/max(pixel_world,1e-6),64.0);
    var sum = 0.0;
    // 4 of 16 directions per frame, 8 steps each
    for (var direction=u32(ambient.quarter.x); direction<16u; direction+=4u) {
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
    return sum;
}
// Occlusion sums and ray position of one pixel.
struct Sums {
    @location(0) sums: vec4<f32>, // near or ground, far; added over frames
    @location(1) linear: vec4<f32>, // x: along the ray; + geometry, - ground, 0 none
};
// Occlusion sums at one pixel, added to the previous frames': ground shadow, or hemisphere samples on geometry.
@fragment fn fs_main(@builtin(position) pixel: vec4<f32>) -> Sums {
    let at = floor(pixel.xy/ao_size()*ambient.params.zw)+0.5;
    let center = surface(at,0);
    if center.w == 0.0 { return Sums(vec4<f32>(0.0),vec4<f32>(0.0)); }
    let p = center.xyz;
    let n = normal_at(at,p,0);
    let ground = depth_at(at)==0.0;
    let line = ray_direction(at);
    let along = dot(p-ray_origin(at),line)/dot(line,line);
    let linear = vec4<f32>(select(along,-along,ground),0.0,0.0,0.0);
    // per-pixel noise so the sample pattern does not tile
    let noise = fract(52.9829189*fract(dot(floor(pixel.xy),vec2<f32>(0.06711056,0.00583715))));
    if ground { return Sums(vec4<f32>(ground_contact(at,p,noise),-1.0,0.0,1.0),linear); }
    let local_radius = contact_radius(at,0);
    if local_radius<=0.0 { return Sums(vec4<f32>(0.0),linear); }
    // tangent frame around the normal
    let up = select(vec3<f32>(0.0,1.0,0.0),vec3<f32>(1.0,0.0,0.0),abs(n.y)>0.9);
    let tangent = normalize(cross(up,n));
    let bitangent = cross(n,tangent);
    let view = normalize(world(at,1.0)-p);
    let bias = local_radius*0.025*(1.0+3.0*(1.0-max(dot(n,view),0.0)));
    var near_occ = 0.0;
    var far_occ = 0.0;
    // 64 near samples, then 64 far ones; every fourth per frame
    for (var i=u32(ambient.quarter.x); i<128u; i+=4u) {
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
    return Sums(vec4<f32>(near_occ,far_occ,0.0,1.0),linear);
}
// Blur along x, keeping edges.
@fragment fn fs_filter_x(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(reconstruct(floor(pixel.xy/ao_size()*ambient.params.zw)+0.5,vec2<i32>(1,0),0),0.0,0.0,1.0);
}
// Blur along y, keeping edges.
@fragment fn fs_filter_y(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(reconstruct(floor(pixel.xy/ao_size()*ambient.params.zw)+0.5,vec2<i32>(0,1),0),0.0,0.0,1.0);
}
// Occlusion at a pixel's first sample: its own texel, or blurred up from fewer texels.
fn occlusion_at(pixel: vec2<f32>) -> f32 {
    if flat() {
        return textureLoad(occlusion,vec2<i32>(pixel),0).r;
    }

    return reconstruct(pixel,vec2<i32>(0),0);
}
// Darken the scene: black with the occlusion as alpha.
@fragment fn fs_composite(@builtin(position) pixel: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0,0.0,0.0,occlusion_at(pixel.xy));
}
// Color and the samples it covers.
struct Shade {
    @location(0) color: vec4<f32>, // black with the occlusion as alpha
    @builtin(sample_mask) mask: u32, // samples that take it
};
// True when `sample` shows the same triangle as sample 0.
fn like_first(xy: vec2<i32>, sample: i32) -> bool {
    let same = all(textureLoad(physical,xy,sample).xy==textureLoad(physical,xy,0).xy);
    return same && (textureLoad(depth,xy,sample)>0.0)==(textureLoad(depth,xy,0)>0.0);
}
// 4x: shade once for every sample on the first sample's triangle; in a drag, for every sample.
@fragment fn fs_composite_pixel(@builtin(position) pixel: vec4<f32>) -> Shade {
    let xy = vec2<i32>(pixel.xy);
    var mask = 15u;
    // a still frame leaves the samples on another triangle to the edge pass
    if ambient.quarter.w==0.0 {
        mask = 1u;
        for (var sample=1; sample<4; sample++) {
            if like_first(xy,sample) { mask |= 1u<<u32(sample); }
        }
    }
    return Shade(vec4<f32>(0.0,0.0,0.0,occlusion_at(pixel.xy)),mask);
}
// 4x: shade the samples on another triangle one by one.
@fragment fn fs_composite_edge(@builtin(position) pixel: vec4<f32>, @builtin(sample_index) sample: u32) -> @location(0) vec4<f32> {
    if sample==0u || like_first(vec2<i32>(pixel.xy),i32(sample)) { discard; }
    return vec4<f32>(0.0,0.0,0.0,reconstruct(pixel.xy,vec2<i32>(0),i32(sample)));
}
// Occlusion at a pixel, blurred over neighbours on the same surface.
fn reconstruct(pixel: vec2<f32>, axis: vec2<i32>, depth_index: i32) -> f32 {
    let center = surface(pixel,depth_index);
    if center.w == 0.0 { return 0.0; }
    let n = normal_at(pixel,center.xyz,depth_index);
    let geometry = depth_sample(pixel,depth_index)>0.0;
    let local_radius = select(ambient.params.y,contact_radius(pixel,depth_index),geometry);
    let dims = vec2<i32>(ao_size());
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
        let along = textureLoad(linear,q,0).x;
        let delta = ray_origin(full)+ray_direction(full)*abs(along)-center.xyz;
        let separation = abs(dot(delta,n));
        let tolerance = max(local_radius*0.024,length(delta)*0.08);
        let distance = vec2<f32>(q)-source;
        let spatial = exp(-dot(distance,distance)*select(1.5,0.04,filtering));
        let ground_weight = select(
            1.0-smoothstep(0.0,local_radius,length(delta.xy)),1.0,geometry);
        let w = spatial*exp(-separation/max(tolerance,1e-6))*ground_weight;
        let valid = select(0.0,w,along!=0.0 && ((along>0.0) == geometry));
        let texel = textureLoad(occlusion,q,0);
        // the x blur reads the sums
        sum += select(texel.r,finalize(texel.rg),axis.x==1)*valid;
        weight += valid;
    }
    return sum/max(weight,1e-6);
}
