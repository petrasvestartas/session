@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>;

struct ProjectLine {
    thickness: f32, proj_y: f32, ortho_h: f32, vp_h: f32, vp_w: f32
};
@group(1) @binding(0) var<uniform> line: ProjectLine;

struct ProjectInstance {
    model: mat4x4<f32>, color: vec4<f32>, flags: u32, thickness: f32, spacing: f32
};
@group(2) @binding(0) var<storage, read> instances: array<ProjectInstance>;
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>;
@group(3) @binding(0) var<storage, read> physical_vertices: array<f32>;
@group(3) @binding(1) var<storage, read> physical_objects: array<u32>;
@group(3) @binding(2) var<storage, read> physical_indices: array<u32>;
@group(3) @binding(3) var<storage, read_write> projected: array<ProjectedTriangle>;
@group(3) @binding(4) var<uniform> live_count: vec4<u32>;

@compute @workgroup_size(64) fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let index = id.x;
    if (index>=live_count.x) {
        return;
    }
    let polygon = project_physical_triangle(index+1u);
    var out: ProjectedTriangle;
    if (polygon.count>=3u) {
        var lo = polygon.points[0].xy;
        var hi = lo;
        for (var i = 1u;i<polygon.count;i++) {
            lo = min(lo, polygon.points[i].xy);
            hi = max(hi, polygon.points[i].xy);
        }
        let area = physical_polygon_area(polygon);
        if (abs(area)<1e-12) {
            projected[index] = out;
            return;
        }
        var equations: array<vec4<f32>, 4>;
        for (var i = 0u;i<polygon.count;i++) {
            let a = polygon.points[i].xy;
            let edge = polygon.points[(i+1u)%polygon.count].xy-a;
            let normal = sign(area)*vec2<f32>(-edge.y, edge.x)/length(edge);
            equations[i] = vec4<f32>(normal, -dot(normal, a), 0.0);
        }
        let ab = polygon.points[1]-polygon.points[0];
        let ac = polygon.points[2]-polygon.points[0];
        let gradient = vec2<f32>(ab.z*ac.y-ac.z*ab.y, ab.x*ac.z-ac.x*ab.z)/area;
        out.edge0 = vec4<f32>(equations[0].xyz, polygon.points[0].x);
        out.edge1 = vec4<f32>(equations[1].xyz, polygon.points[0].y);
        out.edge2 = vec4<f32>(equations[2].xyz, polygon.points[0].z);
        out.edge3 = vec4<f32>(equations[3].xyz, f32(polygon.count));
        var nearest = polygon.points[0].z;
        for (var i = 1u;i<polygon.count;i++) {
            nearest = max(nearest, polygon.points[i].z);
        }
        out.gradient = vec4<f32>(gradient, nearest, 0.0);
        out.bounds = vec4<f32>(lo, hi);
    }
    projected[index] = out;
}
// Shared by conservative tile binning and the finite axis/triangle visibility test.
// Near-plane clipping happens before division, including triangles crossing the eye.

struct ProjectedPolygon {
    points: array<vec3<f32>, 4>,
    count: u32,
};

fn physical_clip_corner(index: u32) -> vec4<f32> {
    let vertex = physical_indices[index];
    let base = vertex * 10u;
    let point = vec3<f32>(physical_vertices[base], physical_vertices[base+1u], physical_vertices[base+2u]);
    let owner = physical_objects[vertex];
    let world = (instances[owner].model * vec4<f32>(point, 1.0)).xyz + translations[owner].xyz;
    return mvp * vec4<f32>(world, 1.0);
}

fn project_physical_triangle(primitive: u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;
    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }
    let base = (primitive-1u)*3u;
    let owner = physical_objects[physical_indices[base]];
    if ((instances[owner].flags & 2u) != 0u) {
        return polygon;
    }
    let input = array<vec4<f32>, 3>(physical_clip_corner(base), physical_clip_corner(base+1u), physical_clip_corner(base+2u));
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;
    for (var i = 0u; i<3u; i++) {
        let current = input[i];
        let distance = current.w - current.z;
        if ((distance > 0.0 && previous_distance < 0.0) || (distance < 0.0 && previous_distance > 0.0)) {
            let t = previous_distance / (previous_distance-distance);
            clipped[polygon.count] = mix(previous, current, t);
            polygon.count++;
        }
        if (distance >= 0.0) {
            clipped[polygon.count] = current;
            polygon.count++;
        }
        previous = current;
        previous_distance = distance;
    }
    for (var i = 0u; i<polygon.count; i++) {
        let clip = clipped[i];
        if (clip.w <= 0.0) {
            polygon.count = 0u;
            return polygon;
        }
        let ndc = clip.xyz/clip.w;
        polygon.points[i] = vec3<f32>((ndc.x*0.5+0.5)*line.vp_w, (0.5-ndc.y*0.5)*line.vp_h, ndc.z);
    }
    return polygon;
}

fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}

fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // A triangle clipped to a quad has non-collinear first three corners except at a
    // degenerate near-plane intersection; that zero-area polygon cannot occlude a line.
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}
