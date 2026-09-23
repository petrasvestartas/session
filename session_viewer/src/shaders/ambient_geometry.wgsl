struct Object {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    ao_radius: f32,
    spacing: f32,
    padding: u32,
};
@group(0) @binding(3) var<storage,read> vertices: array<u32>;
@group(0) @binding(4) var<storage,read> owners: array<u32>;
@group(0) @binding(5) var<storage,read> indices: array<u32>;
@group(0) @binding(6) var<storage,read> objects: array<Object>;

fn primitive_at(pixel: vec2<f32>, sample: i32) -> u32 {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    let halves = textureLoad(physical,xy,sample).xy;
    let primitive = halves.x|(halves.y<<16u);
    return select(primitive,0u,primitive>arrayLength(&indices)/3u);
}
fn triangle_at(pixel: vec2<f32>, sample: i32) -> vec4<f32> {
    let primitive = primitive_at(pixel,sample);
    if primitive==0u { return vec4<f32>(0.0); }
    let vertex = indices[(primitive-1u)*3u];
    return vec4<f32>(0.0,0.0,0.0,objects[owners[vertex]].ao_radius);
}
fn vertex_position(index: u32) -> vec3<f32> {
    return bitcast<vec3<f32>>(vec3<u32>(vertices[index*5u],vertices[index*5u+1u],vertices[index*5u+2u]));
}
fn normal_at(pixel: vec2<f32>, p: vec3<f32>, sample: i32) -> vec3<f32> {
    if depth_sample(pixel,sample)==0.0 { return vec3<f32>(0.0,0.0,1.0); }
    let primitive = primitive_at(pixel,sample);
    var cross_n = vec3<f32>(0.0);
    if primitive>0u {
        let base = (primitive-1u)*3u;
        let first = indices[base];
        let origin = vertex_position(first);
        let model = objects[owners[first]].model;
        let a = (model*vec4<f32>(vertex_position(indices[base+1u])-origin,0.0)).xyz;
        let b = (model*vec4<f32>(vertex_position(indices[base+2u])-origin,0.0)).xyz;
        cross_n = cross(a,b);
    } else {
        let a = surface(pixel+vec2<f32>(1.0,0.0),sample);
        let b = surface(pixel-vec2<f32>(1.0,0.0),sample);
        let c = surface(pixel+vec2<f32>(0.0,1.0),sample);
        let d = surface(pixel-vec2<f32>(0.0,1.0),sample);
        let dx = select(p-b.xyz,a.xyz-p,a.w>0.0 && (b.w==0.0 || distance(a.xyz,p)<distance(b.xyz,p)));
        let dy = select(p-d.xyz,c.xyz-p,c.w>0.0 && (d.w==0.0 || distance(c.xyz,p)<distance(d.xyz,p)));
        cross_n = cross(dx,dy);
    }
    let normal = cross_n/max(length(cross_n),1e-12);
    return select(-normal,normal,dot(normal,-ray_direction(pixel))>0.0);
}
