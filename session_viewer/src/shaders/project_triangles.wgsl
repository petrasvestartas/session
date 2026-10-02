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
    let triangle = physical_clip_triangle(placed.x+1u, placed.y);
    var out: ProjectedTriangle;

    // otherwise the record stays all zero
    if (triangle.drawn) {
        let polygon = near_clipped_polygon(triangle.corners);
        let plane = screen_plane(triangle.corners);

        // behind the near plane, or edge-on: empty record
        if (polygon.count>=3u && plane.drawn) {
            // screen box of the part in front of the near plane
            var lo = polygon.points[0].xy;
            var hi = lo;
            // nearest corner depth
            var nearest = polygon.points[0].z;

            for (var i = 1u;i<polygon.count;i++) {
                lo = min(lo, polygon.points[i].xy);
                hi = max(hi, polygon.points[i].xy);
                nearest = max(nearest, polygon.points[i].z);
            }

            out.edge0 = vec4<f32>(plane.edges[0], plane.weights.x);
            out.edge1 = vec4<f32>(plane.edges[1], plane.weights.y);
            out.edge2 = vec4<f32>(plane.edges[2], plane.weights.z);
            out.edge3 = vec4<f32>(plane.depth.z, 0.0, 0.0, 3.0);
            let row = physical_row(placed.x*3u, placed.y);
            out.gradient = vec4<f32>(plane.depth.xy, nearest, instances[row].ao_radius);
            out.bounds = vec4<f32>(lo, hi);
        }
    }

    projected[index] = out;
}
// The three corners of a triangle in clip space.

struct ClipTriangle {
    corners: array<vec4<f32>, 3>,
    drawn: bool, // false for a hidden object or a triangle a clipping plane removed
};

// A triangle clipped to the near plane: up to four screen-space corners.
struct ProjectedPolygon {
    points: array<vec3<f32>,
    4>,
    count: u32, // corners; 0 = not drawn
};

// The triangle's depth as a plane over the canvas, with its edges.
struct ScreenPlane {
    edges: array<vec3<f32>, 3>, // edge lines in canvas px, unit normal inward
    weights: vec3<f32>, // depth per px of distance inward from each edge
    depth: vec3<f32>, // depth = x * depth.x + y * depth.y + depth.z, canvas px
    drawn: bool, // false for a triangle seen edge-on
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

// Triangle `primitive` (one-based) placed by `row` (SLOT_OWN_ROW: its own) in clip space.
fn physical_clip_triangle(primitive: u32, row: u32) -> ClipTriangle {
    var triangle: ClipTriangle;

    if (primitive == 0u || primitive > arrayLength(&physical_indices)/3u) {
        return triangle;
    }

    let base = (primitive-1u)*3u;
    let owner = physical_row(base, row);

    // 2 = FLAG_HIDDEN; hidden objects do not occlude, nor do triangles a clipping plane removed
    if ((instances[owner].flags & 2u) != 0u || physical_cut(base, owner)) {
        return triangle;
    }

    triangle.corners = array<vec4<f32>, 3>(physical_clip_corner(base, owner), physical_clip_corner(base+1u, owner), physical_clip_corner(base+2u, owner));
    triangle.drawn = true;
    return triangle;
}

// The triangle clipped to the near plane and projected: the part that is drawn, as screen
// corners with their depths. Its box and nearest depth are read from it, never its plane:
// a corner at the near plane can lie millions of pixels off screen at depth 1.
fn near_clipped_polygon(input: array<vec4<f32>, 3>) -> ProjectedPolygon {
    var polygon: ProjectedPolygon;
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

// A line through two homogeneous screen points, as a line over the canvas in px: the clip
// (x, y, w) of each corner stays small however near the eye the corner is.
fn canvas_line(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
    let ndc = cross(a, b);
    // x_ndc = 2 x / w - 1 and y_ndc = 1 - 2 y / h
    return vec3<f32>(2.0*ndc.x/line.vp_w, -2.0*ndc.y/line.vp_h, ndc.z-ndc.x+ndc.y);
}

// The depth plane and the edges of a triangle from its clip corners, in 2D homogeneous
// coordinates: a canvas point lies inside where its three barycentric weights, each an edge
// line evaluated there over the determinant, are positive, and its depth is the weighted sum
// of the corners' clip z. Nothing is divided by w, so a corner beside or behind the eye costs
// no precision, and inside the triangle the depth is a sum of positive terms.
fn screen_plane(corners: array<vec4<f32>, 3>) -> ScreenPlane {
    var plane: ScreenPlane;
    let v0 = corners[0].xyw;
    let v1 = corners[1].xyw;
    let v2 = corners[2].xyw;
    let l0 = cross(v1, v2);
    let determinant = dot(l0, v0);

    // edge-on or degenerate: it covers no pixel
    if (abs(determinant) <= 1e-7*length(v0)*length(v1)*length(v2)) {
        return plane;
    }

    let lines = array<vec3<f32>, 3>(canvas_line(v1, v2), canvas_line(v2, v0), canvas_line(v0, v1));
    let z = vec3<f32>(corners[0].z, corners[1].z, corners[2].z);
    plane.depth = (z.x*lines[0]+z.y*lines[1]+z.z*lines[2])/determinant;
    let inward = sign(determinant);

    for (var i = 0u; i < 3u; i++) {
        let edge = lines[i]*inward;
        let scale = max(length(edge.xy), 1e-30);
        plane.edges[i] = edge/scale;
        // corner i's clip z per px of distance inward from the edge facing it
        plane.weights[i] = z[i]*scale/abs(determinant);
    }

    plane.drawn = true;
    return plane;
}

#include "slot_table.wgsl"
#include "clip.wgsl"
#include "projected_triangle.wgsl"
