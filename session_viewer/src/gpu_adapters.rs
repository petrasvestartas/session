//! Per-geometry-type adapters that produce MeshVertex/LineVertex/PointVertex
//! slices ready to upload via GpuArena::allocate.
//! Free functions (not impl blocks) so they can live in session_viewer
//! without violating the orphan rule.

use crate::gpu_session::{CloudPoint, CylinderSegment, GlyphPoint, LineVertex, MeshVertex, PointVertex, TemplateVertex};
use session_rust::{Color, Line, Mesh, OBB, Plane, Point, PointCloud, Polyline};

#[allow(dead_code)]
pub const CYLINDER_RADIUS: f32 = 15.0;
pub const SPHERE_RADIUS:   f32 = 25.0;
pub const N_CYL_INDICES:   u32 = 144;  // SIDES=12: 12*6 + 12*3 + 12*3
pub const N_SPHERE_INDICES: u32 = 432; // LONS=12,LATS=6: 12*3 + 5*12*6 + 12*3

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

pub fn point_to_vertex(p: &Point) -> [PointVertex; 6] {
    let v = PointVertex {
        position: [p[0], p[1], p[2]],
        color: color_to_rgba_u8(&p.pointcolor),
    };
    [v, v, v, v, v, v]
}

// ---------- Unit cylinder template (GPU-side instancing) ----------

pub fn unit_cylinder_template() -> (Vec<TemplateVertex>, Vec<u32>) {
    const SIDES: usize = 12;
    let pi = std::f32::consts::PI;
    let mut verts: Vec<TemplateVertex> = Vec::with_capacity(50);
    let mut inds:  Vec<u32>           = Vec::with_capacity(144);

    // Side top ring (indices 0..SIDES): z=0.5, outward radial normals
    for i in 0..SIDES {
        let t = i as f32 * 2.0 * pi / SIDES as f32;
        verts.push(TemplateVertex { position: [t.cos(), t.sin(),  0.5], normal: [t.cos(), t.sin(), 0.0] });
    }
    // Side bottom ring (indices SIDES..2*SIDES): z=-0.5
    for i in 0..SIDES {
        let t = i as f32 * 2.0 * pi / SIDES as f32;
        verts.push(TemplateVertex { position: [t.cos(), t.sin(), -0.5], normal: [t.cos(), t.sin(), 0.0] });
    }
    // Side quads (CCW from outside)
    for i in 0..SIDES {
        let t0 = i as u32;
        let t1 = ((i + 1) % SIDES) as u32;
        let b0 = (SIDES + i) as u32;
        let b1 = (SIDES + (i + 1) % SIDES) as u32;
        inds.extend_from_slice(&[t0, t1, b0, t1, b1, b0]);
    }
    // Top cap: center + rim (z=0.5, +Z normal)
    let tc = verts.len() as u32;
    verts.push(TemplateVertex { position: [0.0, 0.0, 0.5], normal: [0.0, 0.0, 1.0] });
    let tr = verts.len() as u32;
    for i in 0..SIDES {
        let t = i as f32 * 2.0 * pi / SIDES as f32;
        verts.push(TemplateVertex { position: [t.cos(), t.sin(), 0.5], normal: [0.0, 0.0, 1.0] });
    }
    for i in 0..SIDES {
        inds.extend_from_slice(&[tc, tr + i as u32, tr + ((i + 1) % SIDES) as u32]);
    }
    // Bottom cap: center + rim (z=-0.5, -Z normal), reversed winding
    let bc = verts.len() as u32;
    verts.push(TemplateVertex { position: [0.0, 0.0, -0.5], normal: [0.0, 0.0, -1.0] });
    let br = verts.len() as u32;
    for i in 0..SIDES {
        let t = i as f32 * 2.0 * pi / SIDES as f32;
        verts.push(TemplateVertex { position: [t.cos(), t.sin(), -0.5], normal: [0.0, 0.0, -1.0] });
    }
    for i in 0..SIDES {
        inds.extend_from_slice(&[bc, br + ((i + 1) % SIDES) as u32, br + i as u32]);
    }
    (verts, inds)
}

// ---------- Unit sphere template (GPU-side instancing) ----------

pub fn unit_sphere_template() -> (Vec<TemplateVertex>, Vec<u32>) {
    const LONS: usize = 12;
    const LATS: usize = 6;
    let pi = std::f32::consts::PI;
    let mut verts: Vec<TemplateVertex> = Vec::with_capacity(74);
    let mut inds:  Vec<u32>           = Vec::with_capacity(432);

    verts.push(TemplateVertex { position: [0.0, 0.0, 1.0], normal: [0.0, 0.0, 1.0] });
    for k in 1..=LATS {
        let phi = k as f32 * pi / (LATS + 1) as f32;
        let z = phi.cos();
        let r = phi.sin();
        for i in 0..LONS {
            let t = i as f32 * 2.0 * pi / LONS as f32;
            let x = r * t.cos();
            let y = r * t.sin();
            verts.push(TemplateVertex { position: [x, y, z], normal: [x, y, z] });
        }
    }
    let south = verts.len() as u32;
    verts.push(TemplateVertex { position: [0.0, 0.0, -1.0], normal: [0.0, 0.0, -1.0] });

    // Top cap
    for i in 0..LONS {
        let r0 = 1 + i as u32;
        let r1 = 1 + ((i + 1) % LONS) as u32;
        inds.extend_from_slice(&[0, r0, r1]);
    }
    // Middle bands
    for k in 0..(LATS - 1) {
        let ra = (1 + k * LONS) as u32;
        let rb = (1 + (k + 1) * LONS) as u32;
        for i in 0..LONS {
            let a0 = ra + i as u32;
            let a1 = ra + ((i + 1) % LONS) as u32;
            let b0 = rb + i as u32;
            let b1 = rb + ((i + 1) % LONS) as u32;
            inds.extend_from_slice(&[a0, a1, b0, a1, b1, b0]);
        }
    }
    // Bottom cap (reversed winding)
    let lr = (1 + (LATS - 1) * LONS) as u32;
    for i in 0..LONS {
        let r0 = lr + i as u32;
        let r1 = lr + ((i + 1) % LONS) as u32;
        inds.extend_from_slice(&[south, r1, r0]);
    }
    (verts, inds)
}

// ---------- Line → cylinder segment ----------

pub fn line_to_segment(l: &Line, instance_id: u32) -> CylinderSegment {
    let s = l.start();
    let e = l.end();
    CylinderSegment { p0: [s[0], s[1], s[2]], radius: 0.0, p1: [e[0], e[1], e[2]], instance_id, color: [0.0;4] }
}

// ---------- Polyline → cylinder segments ----------

pub fn polyline_to_segments(pl: &Polyline, instance_id: u32) -> Vec<CylinderSegment> {
    let pts = pl.get_points();
    pts.windows(2).map(|w| CylinderSegment {
        p0: [w[0][0], w[0][1], w[0][2]],
        radius: 0.0,
        p1: [w[1][0], w[1][1], w[1][2]],
        instance_id,
        color: [0.0; 4],
    }).collect()
}

// ---------- Line / Polyline endpoint spheres ----------

pub fn line_endpoint_glyphs(l: &Line, instance_id: u32) -> Vec<GlyphPoint> {
    let s = l.start();
    vec![GlyphPoint { center: [s[0], s[1], s[2]], radius: 0.0, color: [1.0; 4], instance_id, _pad: [0; 3] }]
}

pub fn polyline_endpoint_glyphs(pl: &Polyline, instance_id: u32) -> Vec<GlyphPoint> {
    let pts = pl.get_points();
    if pts.is_empty() { return Vec::new(); }
    vec![
        GlyphPoint { center: [pts[0][0], pts[0][1], pts[0][2]], radius: 0.0, color: [1.0; 4], instance_id, _pad: [0; 3] },
        GlyphPoint { center: [pts[pts.len()-1][0], pts[pts.len()-1][1], pts[pts.len()-1][2]], radius: 0.0, color: [1.0; 4], instance_id, _pad: [0; 3] },
    ]
}

pub fn mesh_vertex_glyphs(m: &Mesh, instance_id: u32) -> Vec<GlyphPoint> {
    m.vertex.iter().map(|(_, v)| GlyphPoint {
        center: [v.x, v.y, v.z], radius: 0.0, color: [1.0; 4], instance_id, _pad: [0; 3],
    }).collect()
}

// ---------- PointCloud → instanced cloud points ----------

pub fn pointcloud_to_cloud_points(pc: &PointCloud, instance_id: u32) -> Vec<CloudPoint> {
    let pts = pc.get_points();
    let has_colors = pc.color_count() == pts.len();
    pts.iter().enumerate().map(|(i, p)| {
        let c = if has_colors { pc.get_color(i) } else { Color::white() };
        CloudPoint {
            position: [p[0], p[1], p[2]],
            instance_id,
            color: [c.r, c.g, c.b, c.a],
            half_size: 5.0,
            _pad: [0; 3],
        }
    }).collect()
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
            if !tris.is_empty() {
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

    if any_attr_normal {
        patch_zero_normals(&mut verts, &inds);
    } else {
        compute_vertex_normals_in_place(&mut verts, &inds);
    }

    (verts, inds)
}



/// All unique polygon edges of a mesh as cylinder segments (thickness-controllable).
pub fn mesh_edges_to_segments(m: &Mesh, instance_id: u32) -> Vec<CylinderSegment> {
    let edges = m.edges();
    edges.iter().filter_map(|(a, b)| {
        let va = m.vertex.get(a)?;
        let vb = m.vertex.get(b)?;
        Some(CylinderSegment {
            p0: [va.x, va.y, va.z], radius: 0.0,
            p1: [vb.x, vb.y, vb.z], instance_id, color: [0.0, 0.0, 0.0, 1.0],
        })
    }).collect()
}

/// Boundary (naked) edges of a mesh as cylinder segments.
pub fn mesh_naked_edges_to_segments(m: &Mesh, instance_id: u32) -> Vec<CylinderSegment> {
    let edges = m.naked_edges(true);
    edges.iter().filter_map(|(a, b)| {
        let va = m.vertex.get(a)?;
        let vb = m.vertex.get(b)?;
        Some(CylinderSegment {
            p0: [va.x, va.y, va.z], radius: 0.0,
            p1: [vb.x, vb.y, vb.z], instance_id, color: [0.0, 0.0, 0.0, 1.0],
        })
    }).collect()
}

/// A sequence of points (polyline) as cylinder segments.
/// Filters out zero-length segments (degenerate tessellation artefacts).
pub fn pts_to_segments(pts: &[session_rust::Point], instance_id: u32) -> Vec<CylinderSegment> {
    pts.windows(2).filter_map(|w| {
        let p0 = [w[0][0], w[0][1], w[0][2]];
        let p1 = [w[1][0], w[1][1], w[1][2]];
        let dist2 = (p1[0]-p0[0]).powi(2) + (p1[1]-p0[1]).powi(2) + (p1[2]-p0[2]).powi(2);
        if dist2 < 1e-6 { return None; }
        Some(CylinderSegment { p0, radius: 0.0, p1, instance_id, color: [0.0, 0.0, 0.0, 1.0] })
    }).collect()
}

fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let ab = [b[0]-a[0], b[1]-a[1], b[2]-a[2]];
    let ac = [c[0]-a[0], c[1]-a[1], c[2]-a[2]];
    let nx = ab[1]*ac[2] - ab[2]*ac[1];
    let ny = ab[2]*ac[0] - ab[0]*ac[2];
    let nz = ab[0]*ac[1] - ab[1]*ac[0];
    [nx, ny, nz]
}

fn normalize3(n: [f32; 3]) -> [f32; 3] {
    let len = (n[0]*n[0] + n[1]*n[1] + n[2]*n[2]).sqrt();
    if len > 1e-12 { [n[0]/len, n[1]/len, n[2]/len] } else { [0.0, 0.0, 1.0] }
}

fn compute_vertex_normals_in_place(verts: &mut [MeshVertex], inds: &[u32]) {
    for v in verts.iter_mut() { v.normal = [0.0, 0.0, 0.0]; }
    let nt = inds.len() / 3;
    let nv = verts.len();
    for ti in 0..nt {
        let ia = inds[ti*3] as usize;
        let ib = inds[ti*3+1] as usize;
        let ic = inds[ti*3+2] as usize;
        if ia >= nv || ib >= nv || ic >= nv { continue; }
        let n = face_normal(verts[ia].position, verts[ib].position, verts[ic].position);
        for &idx in &[ia, ib, ic] {
            verts[idx].normal[0] += n[0];
            verts[idx].normal[1] += n[1];
            verts[idx].normal[2] += n[2];
        }
    }
    for v in verts.iter_mut() { v.normal = normalize3(v.normal); }
}

/// Patch any zero-length stored normals (e.g. at degenerate surface poles) with
/// face-averaged normals so they don't render as black artefacts.
fn patch_zero_normals(verts: &mut [MeshVertex], inds: &[u32]) {
    let nv = verts.len();
    let zero: Vec<bool> = verts.iter().map(|v| {
        let n = v.normal;
        n[0]*n[0] + n[1]*n[1] + n[2]*n[2] < 1e-10
    }).collect();
    if !zero.iter().any(|&z| z) { return; }
    let nt = inds.len() / 3;
    for ti in 0..nt {
        let ia = inds[ti*3] as usize;
        let ib = inds[ti*3+1] as usize;
        let ic = inds[ti*3+2] as usize;
        if ia >= nv || ib >= nv || ic >= nv { continue; }
        if !zero[ia] && !zero[ib] && !zero[ic] { continue; }
        let n = face_normal(verts[ia].position, verts[ib].position, verts[ic].position);
        for &idx in &[ia, ib, ic] {
            if zero[idx] {
                verts[idx].normal[0] += n[0];
                verts[idx].normal[1] += n[1];
                verts[idx].normal[2] += n[2];
            }
        }
    }
    for (i, v) in verts.iter_mut().enumerate() {
        if zero[i] { v.normal = normalize3(v.normal); }
    }
}

// ---------- Plane ----------

#[allow(dead_code)]
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

pub fn plane_to_axis_segments(pl: &Plane, scale: f32, instance_id: u32) -> Vec<CylinderSegment> {
    let o = pl.origin();
    let x = pl.x_axis();
    let y = pl.y_axis();
    let z = pl.z_axis();
    vec![
        CylinderSegment { p0: [o[0], o[1], o[2]], radius: 0.0,
            p1: [o[0]+x[0]*scale, o[1]+x[1]*scale, o[2]+x[2]*scale],
            instance_id, color: [1.0, 0.2, 0.2, 1.0] },
        CylinderSegment { p0: [o[0], o[1], o[2]], radius: 0.0,
            p1: [o[0]+y[0]*scale, o[1]+y[1]*scale, o[2]+y[2]*scale],
            instance_id, color: [0.2, 1.0, 0.2, 1.0] },
        CylinderSegment { p0: [o[0], o[1], o[2]], radius: 0.0,
            p1: [o[0]+z[0]*scale, o[1]+z[1]*scale, o[2]+z[2]*scale],
            instance_id, color: [0.3, 0.6, 1.0, 1.0] },
    ]
}

pub fn plane_origin_glyph(pl: &Plane, instance_id: u32) -> GlyphPoint {
    let o = pl.origin();
    GlyphPoint { center: [o[0], o[1], o[2]], radius: 0.0,
        color: [1.0, 1.0, 1.0, 1.0], instance_id, _pad: [0; 3] }
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
