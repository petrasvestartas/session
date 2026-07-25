@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct CloudUniform {
    size: f32,
    vp_w: f32,
    vp_h: f32,
    _pad: f32,
};
@group(1) @binding(0) var<uniform> cloud: CloudUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

struct CloudPoint {
    position: vec3<f32>,
    instance_id: u32,
    color: vec4<f32>,
};
@group(3) @binding(0) var<storage, read> points: array<CloudPoint>;

// One logical point = 3 verts (1 triangle); corner is vertex_index % 3
// Equilateral triangle whose INCIRCLE (radius 1 in corner-space) is the visible dot
// corners sit at distance 2 from center
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>( 0.0,        2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>( 1.7320508, -1.0),
);

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>, // triangle-local; the incircle (radius 1) is the dot
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let p = points[vid / 3u]; // 3 verts per point -> vid/3 = the point
    let model = instances[p.instance_id].model;
    let world = (model * vec4<f32>(p.position, 1.0)).xyz;
    let clip = mvp * vec4<f32>(world, 1.0);
    let corner = CORNERS[vid % 3u];
    let px = cloud.size; // the cloud's own global dot size - its own uniform
    let off = corner * px * 2.0 / vec2<f32>(cloud.vp_w, cloud.vp_h) * clip.w;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy+off, clip.zw);
    o.color = p.color;
    o.corner = corner;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    let d = length(in.corner); // SDF circle: soft, anti-aliased edge
    let a = clamp((1.0-d)*8.0, 0.0, 1.0);
    if (a < 0.01){
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a*a);
}