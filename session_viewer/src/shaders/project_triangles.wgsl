@group(0) @binding(0) var<uniform> mvp: mat4x4<f32>; // camera matrix

// The first five fields of LineUniform; this shader needs no more.
struct ProjectLine {
    thickness: f32, // pen width, px
    proj_y: f32, // perspective scale factor
    ortho_h: f32, // ortho half-height; 0 = perspective
    vp_h: f32, // target height, px
    vp_w: f32, // target width, px
};

@group(1) @binding(0) var<uniform> line: ProjectLine; // view settings
@group(1) @binding(1) var<uniform> clipping: ClipUniform; // clipping planes

// Instance under another name; field order must match the Rust row.
struct ProjectInstance {
    model: mat4x4<f32>, // rotation and scale
    color: vec4<f32>, // rgba tint
    flags: u32, // FLAG_* bits
    ao_radius: f32, // SSAO contact radius
    spacing: f32, // vertex spacing
};

@group(2) @binding(0) var<storage, read> instances: array<ProjectInstance>; // one row per object
@group(2) @binding(1) var<storage, read> translations: array<vec4<f32>>; // position per object
@group(3) @binding(0) var<storage, read> physical_vertices: array<f32>; // mesh vertices, five words each
@group(3) @binding(1) var<storage, read> physical_objects: array<u32>; // object row per vertex
@group(3) @binding(2) var<storage, read> physical_indices: array<u32>; // triangle indices
@group(3) @binding(3) var<storage, read_write> projected: array<ProjectedTriangle>; // output: one record per triangle
@group(3) @binding(4) var<uniform> live_count: vec4<u32>; // x = triangle ids, the instances' included
@group(3) @binding(5) var slot_table: texture_2d<u32>; // where the instances' triangle ids lie

// Project one triangle id per invocation into `projected`: an arena triangle, or one instance's
// copy of its definition's.
@compute @workgroup_size(64)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // rows of 32768 workgroups (PROJECT_ROW_GROUPS): a dispatch dimension holds at most 65535
    let index = id.x + id.y * 2097152u;

    if (index>=live_count.x) {
        return;
    }

    let placed = placed_triangle(index+1u, arrayLength(&physical_indices)/3u);
    let polygon = project_physical_triangle(placed.x+1u, placed.y);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
    if (polygon.count>=3u) {
        // screen box
        var lo = polygon.points[0].xy;
        var hi = lo;

        for (var i = 1u;i<polygon.count;i++) {
            lo = min(lo, polygon.points[i].xy);
            hi = max(hi, polygon.points[i].xy);
        }

        let area = physical_polygon_area(polygon);

        // degenerate: empty record
        if (abs(area)<1e-12) {
            projected[index] = out;
            return;
        }

        // one inward line equation per edge
        var equations: array<vec4<f32>, 4>;

        for (var i = 0u;i<polygon.count;i++) {
            let a = polygon.points[i].xy;
            let edge = polygon.points[(i+1u)%polygon.count].xy-a;
            let normal = sign(area)*vec2<f32>(-edge.y, edge.x)/length(edge);
            equations[i] = vec4<f32>(normal, -dot(normal, a), 0.0);
        }

        let ab = polygon.points[1]-polygon.points[0];
        let ac = polygon.points[2]-polygon.points[0];
        // depth change per screen pixel
        let gradient = vec2<f32>(ab.z*ac.y-ac.z*ab.y, ab.x*ac.z-ac.x*ab.z)/area;
        out.edge0 = vec4<f32>(equations[0].xyz, polygon.points[0].x);
        out.edge1 = vec4<f32>(equations[1].xyz, polygon.points[0].y);
        out.edge2 = vec4<f32>(equations[2].xyz, polygon.points[0].z);
        out.edge3 = vec4<f32>(equations[3].xyz, f32(polygon.count));
        // nearest corner depth
        var nearest = polygon.points[0].z;

        for (var i = 1u;i<polygon.count;i++) {
            nearest = max(nearest, polygon.points[i].z);
        }

        let row = physical_row(placed.x*3u, placed.y);
        out.gradient = vec4<f32>(gradient, nearest, instances[row].ao_radius);
        out.bounds = vec4<f32>(lo, hi);
    }

    projected[index] = out;
}
// A triangle clipped to the near plane: up to four screen-space corners.

struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
};

// Row drawing index `index`: `row`, or the vertex's own for SLOT_OWN_ROW.
fn physical_row(index: u32, row: u32) -> u32 {
    return select(row, physical_objects[physical_indices[index]], row == SLOT_OWN_ROW);
}

// Vertex `index` of the mesh in scene space, placed by `row`.
fn physical_world_corner(index: u32, row: u32) -> vec3<f32> {
    let vertex = physical_indices[index];
    // five words per vertex: position, normal, color
    let base = vertex * 5u;
    let point = vec3<f32>(physical_vertices[base], physical_vertices[base+1u], physical_vertices[base+2u]);
    return (instances[row].model * vec4<f32>(point, 1.0)).xyz + translations[row].xyz;
}

// Vertex `index` of the mesh in clip space, placed by `row`.
fn physical_clip_corner(index: u32, row: u32) -> vec4<f32> {
    return mvp * vec4<f32>(physical_world_corner(index, row), 1.0);
}

// True when one clipping plane cuts all three corners of the triangle at `base`, placed by `row`, away.
fn physical_cut(base: u32, row: u32) -> bool {
    if (!clip_active() || (instances[row].flags & FLAG_CLIPPING_PLANE) != 0u) {
        return false;
    }

    let a = physical_world_corner(base, row);
    let b = physical_world_corner(base+1u, row);
    let c = physical_world_corner(base+2u, row);

    for (var i = 0u; i < clipping.count; i++) {
        if (clip_distance(i, a) < 0.0 && clip_distance(i, b) < 0.0 && clip_distance(i, c) < 0.0) {
            return true;
        }
    }

    return false;
}

// Triangle `primitive` (one-based) placed by `row` (SLOT_OWN_ROW: its own), projected and
// clipped to the near plane.
fn project_physical_triangle(primitive: u32, row: u32) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return polygon;
    }

    let base = (primitive-1u)*3u;
    let owner = physical_row(base, row);

    // 2 = FLAG_HIDDEN; hidden objects do not occlude, nor do triangles a clipping plane removed
    if ((instances[owner].flags & 2u) != 0u || physical_cut(base, owner)) {
        return polygon;
    }

    let input = array<vec4<f32>, 3>(physical_clip_corner(base, owner), physical_clip_corner(base+1u, owner), physical_clip_corner(base+2u, owner));
    var clipped: array<vec4<f32>, 4>;
    var previous = input[2];
    var previous_distance = previous.w - previous.z;

    // clip each edge against the near plane
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

    // clip space to screen pixels
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

// 2D cross product.
fn physical_cross(a: vec2<f32>, b: vec2<f32>) -> f32 {
    return a.x*b.y-a.y*b.x;
}

// Twice the signed area of the first three corners.
fn physical_polygon_area(polygon: ProjectedPolygon) -> f32 {
    // zero only for a degenerate polygon
    return physical_cross(polygon.points[1].xy-polygon.points[0].xy, polygon.points[2].xy-polygon.points[0].xy);
}
