//! Per-geometry-type adapters that produce MeshVertex/LineVertex/PointVertex
//! slices ready to upload via GpuArena::allocate.
//! Free functions (not impl blocks) so they can live in session_viewer
//! without violating the orphan rule.

use crate::gpu_session::{LineVertex, MeshVertex, PointVertex};
use session_rust::{Color, Line, Mesh, OBB, Plane, Point, PointCloud, Polyline};

// ---------- Named point crosshair ----------

/// Three axis-aligned line segments (6 verts, 6 indices) centred on the point.
pub fn named_point_to_cross_vertices(p: &Point) -> (Vec<LineVertex>, Vec<u32>) {
    let [x, y, z] = [p[0], p[1], p[2]];
    let arm = 15.0_f32;
    let color = color_to_rgba_u8(&p.pointcolor);
    let verts = vec![
        LineVertex { position: [x - arm, y, z], color },
        LineVertex { position: [x + arm, y, z], color },
        LineVertex { position: [x, y - arm, z], color },
        LineVertex { position: [x, y + arm, z], color },
        LineVertex { position: [x, y, z - arm], color },
        LineVertex { position: [x, y, z + arm], color },
    ];
    let inds = vec![0u32, 1, 2, 3, 4, 5];
    (verts, inds)
}

// ---------- Point ----------

pub fn point_to_vertex(p: &Point) -> PointVertex {
    PointVertex {
        position: [p[0], p[1], p[2]],
        color: color_to_rgba_u8(&p.pointcolor),
    }
}

// ---------- Line ----------

pub fn line_to_vertices(l: &Line) -> [LineVertex; 2] {
    let color = color_to_rgba_u8(&l.linecolor);
    [
        LineVertex { position: [l.start()[0], l.start()[1], l.start()[2]], color },
        LineVertex { position: [l.end()[0], l.end()[1], l.end()[2]], color },
    ]
}

// ---------- Polyline ----------

pub fn polyline_to_vertices(pl: &Polyline) -> (Vec<LineVertex>, Vec<u32>) {
    let color = color_to_rgba_u8(&pl.linecolor);
    let pts = pl.get_points();
    let verts: Vec<LineVertex> = pts
        .iter()
        .map(|p| LineVertex { position: [p[0], p[1], p[2]], color })
        .collect();
    let n = verts.len();
    let mut inds = Vec::with_capacity(n.saturating_sub(1) * 2);
    for i in 0..n.saturating_sub(1) {
        inds.push(i as u32);
        inds.push((i + 1) as u32);
    }
    (verts, inds)
}

// ---------- PointCloud ----------

pub fn pointcloud_to_vertices(pc: &PointCloud) -> Vec<PointVertex> {
    let pts = pc.get_points();
    let has_colors = pc.color_count() == pts.len();
    pts.iter()
        .enumerate()
        .map(|(i, p)| {
            let color = if has_colors {
                color_to_rgba_u8(&pc.get_color(i))
            } else {
                [255, 255, 255, 255]
            };
            PointVertex { position: [p[0], p[1], p[2]], color }
        })
        .collect()
}

// ---------- Mesh ----------

pub fn mesh_to_vertices(m: &Mesh) -> (Vec<MeshVertex>, Vec<u32>) {
    let mut keys: Vec<usize> = m.vertex.keys().copied().collect();
    keys.sort_unstable();
    let key_to_idx: std::collections::HashMap<usize, u32> = keys
        .iter()
        .enumerate()
        .map(|(i, &k)| (k, i as u32))
        .collect();

    let object_color_rgba = color_to_rgba_u8(m.objectcolor());
    let point_colors = m.get_pointcolors();
    let has_point_colors = point_colors.len() == keys.len();

    let mut verts: Vec<MeshVertex> = Vec::with_capacity(keys.len());
    let mut any_attr_normal = false;
    for (idx, k) in keys.iter().enumerate() {
        let v = &m.vertex[k];
        let nx = v.attributes.get("nx").copied();
        let ny = v.attributes.get("ny").copied();
        let nz = v.attributes.get("nz").copied();
        if nx.is_some() || ny.is_some() || nz.is_some() {
            any_attr_normal = true;
        }
        let normal = [nx.unwrap_or(0.0), ny.unwrap_or(0.0), nz.unwrap_or(0.0)];
        let color = if has_point_colors {
            color_to_rgba_u8(&point_colors[idx])
        } else {
            object_color_rgba
        };
        verts.push(MeshVertex { position: [v.x, v.y, v.z], normal, color });
    }

    let mut inds: Vec<u32> = Vec::new();
    let mut face_keys: Vec<usize> = m.face.keys().copied().collect();
    face_keys.sort_unstable();
    for fk in face_keys {
        if let Some(tris) = m.triangulation.get(&fk) {
            for tri in tris {
                if let (Some(&a), Some(&b), Some(&c)) = (
                    key_to_idx.get(&tri[0]),
                    key_to_idx.get(&tri[1]),
                    key_to_idx.get(&tri[2]),
                ) {
                    inds.push(a);
                    inds.push(b);
                    inds.push(c);
                }
            }
            continue;
        }
        let verts_of_face = &m.face[&fk];
        if verts_of_face.len() < 3 {
            continue;
        }
        let v0 = match key_to_idx.get(&verts_of_face[0]) {
            Some(&i) => i,
            None => continue,
        };
        for i in 1..(verts_of_face.len() - 1) {
            let a = match key_to_idx.get(&verts_of_face[i]) {
                Some(&i) => i,
                None => continue,
            };
            let b = match key_to_idx.get(&verts_of_face[i + 1]) {
                Some(&i) => i,
                None => continue,
            };
            inds.push(v0);
            inds.push(a);
            inds.push(b);
        }
    }

    if !any_attr_normal {
        compute_vertex_normals_in_place(&mut verts, &inds);
    }

    (verts, inds)
}

fn compute_vertex_normals_in_place(verts: &mut [MeshVertex], inds: &[u32]) {
    for v in verts.iter_mut() {
        v.normal = [0.0, 0.0, 0.0];
    }
    let mut i = 0;
    while i + 2 < inds.len() {
        let ia = inds[i] as usize;
        let ib = inds[i + 1] as usize;
        let ic = inds[i + 2] as usize;
        i += 3;
        if ia >= verts.len() || ib >= verts.len() || ic >= verts.len() {
            continue;
        }
        let a = verts[ia].position;
        let b = verts[ib].position;
        let c = verts[ic].position;
        let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
        let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];
        let nx = ab[1]*ac[2] - ab[2]*ac[1];
        let ny = ab[2]*ac[0] - ab[0]*ac[2];
        let nz = ab[0]*ac[1] - ab[1]*ac[0];
        for &idx in &[ia, ib, ic] {
            verts[idx].normal[0] += nx;
            verts[idx].normal[1] += ny;
            verts[idx].normal[2] += nz;
        }
    }
    for v in verts.iter_mut() {
        let n = v.normal;
        let len = (n[0]*n[0] + n[1]*n[1] + n[2]*n[2]).sqrt();
        v.normal = if len > 1e-12 {
            [n[0]/len, n[1]/len, n[2]/len]
        } else {
            [0.0, 0.0, 1.0]
        };
    }
}

// ---------- Plane ----------

pub fn plane_to_mesh_vertices(pl: &Plane, size: f32) -> (Vec<MeshVertex>, Vec<u32>) {
    let o = pl.origin();
    let x = pl.x_axis();
    let y = pl.y_axis();
    let z = pl.z_axis();
    let h = size * 0.5;
    let color = color_to_rgba_u8(&pl.linecolor);
    let mk = |sx: f32, sy: f32| MeshVertex {
        position: [
            o[0] + sx*x[0] + sy*y[0],
            o[1] + sx*x[1] + sy*y[1],
            o[2] + sx*x[2] + sy*y[2],
        ],
        normal: [z[0], z[1], z[2]],
        color,
    };
    let verts = vec![mk(-h,-h), mk(h,-h), mk(h,h), mk(-h,h)];
    let inds = vec![0u32, 1, 2, 0, 2, 3];
    (verts, inds)
}

// ---------- OBB ----------

pub fn obb_to_line_vertices(bb: &OBB) -> (Vec<LineVertex>, Vec<u32>) {
    let corners = bb.corners();
    let color = [255u8, 255, 255, 255];
    let verts: Vec<LineVertex> = corners
        .iter()
        .map(|p| LineVertex { position: [p[0], p[1], p[2]], color })
        .collect();
    let inds = vec![
        0u32,1, 1,2, 2,3, 3,0,
        4,5, 5,6, 6,7, 7,4,
        0,4, 1,5, 2,6, 3,7,
    ];
    (verts, inds)
}

// ---------- Conversion helpers ----------

pub fn color_to_rgba_f32(c: &Color) -> [f32; 4] {
    [c.r, c.g, c.b, c.a]
}

pub fn color_to_rgba_u8(c: &Color) -> [u8; 4] {
    [(c.r*255.0) as u8, (c.g*255.0) as u8, (c.b*255.0) as u8, (c.a*255.0) as u8]
}
