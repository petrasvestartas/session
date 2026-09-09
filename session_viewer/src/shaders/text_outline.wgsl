// Imported PDF lettering retains the producer's exact positioned outlines. Coverage comes
// from the pass samples; this shader never lights, thickens or re-spaces the glyphs.
@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

// Same 96-byte instance record as the arena: model 0, color 64, flags 80,
// thickness 84, spacing 88; the storage-array stride rounds up to 96 bytes.
struct Instance {
    model: mat4x4<f32>,
    color: vec4<f32>,
    flags: u32,
    thickness: f32,
    spacing: f32,
}
@group(2) @binding(0) var<storage, read> instances: array<Instance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) object: u32,
}
struct Fragment {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) object: u32,
}

// Object-local point to the camera's rebased world frame.
fn place(object: u32, point: vec3<f32>) -> vec3<f32> {
    return (instances[object].model * vec4<f32>(point, 1.0)).xyz + translations[object].xyz;
}

// Apply the original object placement and unlit source/selection color.
@vertex
fn vs_main(vertex: Vertex) -> Fragment {
    let instance = instances[vertex.object];
    var out: Fragment;
    out.object = vertex.object;
    if (instance.flags & 2u) != 0u {
        out.position = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        out.color = vec4<f32>(0.0);
        return out;
    }
    let world = place(vertex.object, vertex.position);
    out.position = mvp * vec4<f32>(world, 1.0);
    let selected = (instance.flags & 1u) != 0u;
    out.color = vec4<f32>(select(vertex.color.rgb * instance.color.rgb,
        vec3<f32>(1.0, 1.0, 0.0), selected), vertex.color.a * instance.color.a);
    return out;
}

// Preserve straight-alpha color; the render pass supplies outline coverage samples.
@fragment
fn fs_main(fragment: Fragment) -> @location(0) vec4<f32> {
    return fragment.color;
}

// Keep the exact source object row available to the shared identity pass.
@fragment
fn fs_physical_id(fragment: Fragment) -> PhysicalId {
 return PhysicalId(vec2<u32>(fragment.object+1u, 0u), vec4<f32>(0.0));
}

@fragment
fn fs_id(fragment: Fragment) -> @location(0) vec2<u32> {
    return vec2<u32>(fragment.object + 1u, 0u);
}
