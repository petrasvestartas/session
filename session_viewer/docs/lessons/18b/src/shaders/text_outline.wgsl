// --8<-- [start:outline-shader]
// No @location(1): print needs no normal, and an attribute the shader does not declare is ignored.
struct Vertex {
    @location(0) position: vec3<f32>, // object space
    @location(2) color: vec4<f32>,
    @location(3) object: u32, // object row
}

struct Fragment {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) @interpolate(flat) object: u32,
}

// Sheet fills and lettering are print: flat colour, no light, yellow when selected.
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

// In a fragment shader @builtin(position) is the pixel, so the test runs in canvas clip space.
fn cut(fragment: Fragment) -> bool {
    return clip_active() && clip_cut_ndc(0u, clip_ndc(fragment.position.xy + line.origin, line.frame, fragment.position.z));
}

@fragment
fn fs_main(fragment: Fragment) -> @location(0) vec4<f32> {
    if (cut(fragment)) {
        discard;
    }

    return fragment.color;
}

// Second output 0: print has no triangle id.
@fragment
fn fs_physical_id(fragment: Fragment) -> PhysicalId {
    if (cut(fragment)) {
        discard;
    }

 return PhysicalId(vec2<u32>(fragment.object+1u, 0u), vec2<u32>(0u));
}

@fragment
fn fs_id(fragment: Fragment) -> @location(0) vec2<u32> {
    if (cut(fragment)) {
        discard;
    }

    return vec2<u32>(fragment.object + 1u, 0u);
}
// --8<-- [end:outline-shader]
