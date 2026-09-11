// Imported PDF lettering retains the producer's exact positioned outlines. Coverage comes
// from the pass samples; this shader never lights, thickens or re-spaces the glyphs.

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

// Apply the original object placement and unlit source/selection color.
@vertex
fn vs_main(vertex: Vertex) -> Fragment {
    let instance = instances[vertex.object];
    var out: Fragment;
    out.object = vertex.object;
    if (instance.flags & FLAG_HIDDEN) != 0u {
        out.position = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        out.color = vec4<f32>(0.0);
        return out;
    }
    let world = place(vertex.object, vertex.position);
    out.position = mvp * vec4<f32>(world, 1.0);
    let selected = (instance.flags & FLAG_SELECTED) != 0u;
    out.color = vec4<f32>(select(vertex.color.rgb * instance.color.rgb,
        SELECT_COLOR, selected), vertex.color.a * instance.color.a);
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
