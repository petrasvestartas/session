// One mesh vertex, as the arena stores it.
struct Vertex {
    @location(0) position: vec3<f32>,
    @location(2) color: vec4<f32>, // location 1, the normal, is not read: text is unlit
    @location(3) object: u32, // which row of instances[] this vertex belongs to
}

// What the vertex shader hands the fragment shader.
struct Fragment {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) object: u32, // flat: an integer cannot be blended, so one vertex's value is used
}

// Place the vertex; color is flat, yellow when selected.
@vertex
fn vs_main(vertex: Vertex) -> Fragment {
    let instance = instances[vertex.object];
    var out: Fragment;
    out.object = vertex.object;

    // hidden: x = 3 lies outside clip space, so nothing is drawn
    if (instance.flags & FLAG_HIDDEN) != 0u {
        out.position = vec4<f32>(3.0, 3.0, 0.5, 1.0);
        out.color = vec4<f32>(0.0);
        return out;
    }

    let world = place(vertex.object, vertex.position);
    out.position = mvp * vec4<f32>(world, 1.0);
    let selected = (instance.flags & FLAG_SELECTED) != 0u;
    // select(a, b, c) = if c then b else a
    out.color = vec4<f32>(select(vertex.color.rgb * instance.color.rgb,
        SELECT_COLOR, selected), vertex.color.a * instance.color.a);
    return out;
}

@fragment
fn fs_main(fragment: Fragment) -> @location(0) vec4<f32> {
    return fragment.color;
}

// Object id, writing depth slope too.
@fragment
fn fs_physical_id(fragment: Fragment) -> PhysicalId {
 // --8<-- [start:step-7]
 return PhysicalId(vec2<u32>(fragment.object+1u, 0u), vec4<f32>(0.0));
 // --8<-- [end:step-7]
}

@fragment
// Picking: row + 1 into an integer target; 0 means nothing was hit.
fn fs_id(fragment: Fragment) -> @location(0) vec2<u32> {
    return vec2<u32>(fragment.object + 1u, 0u);
}
