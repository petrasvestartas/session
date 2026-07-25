@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;
@group(1) @binding(0) var<uniform> line: LineUniform;

struct Instance{
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
};
@group(2) @binding(0) var<storage, read> instances: array<Instance>;

// Matches the Rust GlyphPoint (48 B) — same table the sphere pipeline reads.
struct GlyphPoint{
    center: vec3<f32>,
    radius: f32,
    color: vec4<f32>,
    instance_id: u32,
};
@group(3) @binding(0) var<storage, read> glyphs: array<GlyphPoint>;

struct LineUniform{
    thickness: f32, // on-screen width, px
    proj_y: f32,    // vertical projection scale × unit scale
    ortho_h: f32,   // ortho world half-height × unit scale; 0.0 in perspective
    vp_h: f32,      // framebuffer height, px
    vp_w: f32,      // framebuffer width, px (billboard aspect)
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
};

// One dot = 3 verts (1 triangle) — 48× fewer triangles than the sphere template.
// Equilateral triangle whose INCIRCLE (radius 1 in corner space) is the visible disc.
const CORNERS = array<vec2<f32>, 3>(
    vec2<f32>( 0.0,        2.0),
    vec2<f32>(-1.7320508, -1.0),
    vec2<f32>( 1.7320508, -1.0),
);

struct VsOut{
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner: vec2<f32>, // triangle-local; the incircle is the dot
};

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VsOut{
    let g = glyphs[vid / 3u];
    let model = instances[g.instance_id].model;
    let centre = (model * vec4<f32>(g.center, 1.0)).xyz;
    let clip = mvp * vec4<f32>(centre, 1.0);

    // px radius: the global thickness, unless the glyph carries a world radius (>0) —
    // the inverse of cylinder.wgsl's screen_radius, solved for pixels.
    var px = line.thickness;
    if (g.radius > 0.0) {
        if (line.ortho_h > 0.0) {
            px = g.radius * line.vp_h / line.ortho_h;          // ortho: depth-independent
        } else {
            px = g.radius * line.proj_y * line.vp_h / clip.w;  // perspective: shrinks with depth
        }
    }

    let corner = CORNERS[vid % 3u];
    let off = corner * px * 2.0 / vec2<f32>(line.vp_w, line.vp_h) * clip.w;
    var o: VsOut;
    o.pos = vec4<f32>(clip.xy + off, clip.zw);
    o.color = g.color;
    o.corner = corner;
    return o;
}

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32>{
    if (length(in.corner) > 1.0) { discard; } // SDF circle; MSAA smooths the rim
    return in.color;
}
