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
@group(0) @binding(7) var slot_table: texture_2d<u32>;

// (arena triangle, row) under a pixel: an instance's triangle takes the instance row
fn primitive_at(pixel: vec2<f32>, sample: i32) -> vec2<u32> {
    let xy = clamp(vec2<i32>(pixel),vec2<i32>(0),vec2<i32>(ambient.params.zw)-1);
    let halves = textureLoad(physical,xy,sample).xy;
    let placed = placed_triangle(halves.x|(halves.y<<16u),arrayLength(&indices)/3u);
    if placed.x==NO_TRIANGLE { return placed; }
    return vec2<u32>(placed.x,select(placed.y,owners[indices[placed.x*3u]],placed.y==SLOT_OWN_ROW));
}
fn vertex_position(index: u32) -> vec3<f32> {
    return bitcast<vec3<f32>>(vec3<u32>(vertices[index*5u],vertices[index*5u+1u],vertices[index*5u+2u]));
}
// Normal of an already looked-up primitive; a pixel without one takes its depth neighbours.
fn normal_of(placed: vec2<u32>, pixel: vec2<f32>, p: vec3<f32>, sample: i32) -> vec3<f32> {
    var cross_n = vec3<f32>(0.0);
    if placed.x!=NO_TRIANGLE {
        let base = placed.x*3u;
        let first = indices[base];
        let origin = vertex_position(first);
        let model = objects[placed.y].model;
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
