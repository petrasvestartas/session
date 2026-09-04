//! The plate faces a free outline can lie on. A polyline drawn exactly on a mesh face (a
//! plate outline, a contact area) must lift off that face like the mesh's own wires and
//! never through the plate: it takes the face's normal and the mesh's thickness with it.

use std::collections::HashMap;
use session_rust::{Geometry, Mesh, Session};
use super::bounds::mesh_thickness;
use super::mesh::MESH_RAW_MIN;

/// A point is on a plane within this distance, local units (mm).
const ON_PLANE: f32 = 0.5;

/// One face plane of one mesh and that mesh's thickness.
struct HostPlane {
    n: [f32; 3],
    d: f32,
    thickness: f32,
}

/// What a hosted polyline inherits: the face normal and the host's thickness.
pub struct Host {
    pub normal: [f32; 3],
    pub thickness: f32,
}

/// Every distinct face plane of the file's meshes (huge meshes skipped: outlines lie on
/// plates, not on scans), so a polyline can find the face it lies on.
pub struct Hosts {
    planes: Vec<HostPlane>,
}

impl Hosts {
    /// No planes: nothing to host.
    pub fn empty() -> Self {
        Self { planes: Vec::new() }
    }

    /// The planes of a session's meshes, built only when the session has polylines to host.
    pub fn from_session(s: &Session) -> Self {
        let mut hosts = Self::empty();
        if s.objects.polylines.is_empty() {
            return hosts;
        }
        for g in s.order() {
            if let Some(Geometry::Mesh(m)) = s.lookup.get(&g) {
                hosts.add_mesh(m);
            }
        }
        hosts
    }

    /// One mesh's distinct face planes with its thickness.
    fn add_mesh(&mut self, m: &Mesh) {
        if m.number_of_faces() > MESH_RAW_MIN || m.number_of_faces() == 0 {
            return;
        }
        let keys = m.vertices();
        let mut slot: HashMap<usize, u32> = HashMap::with_capacity(keys.len());
        let mut pts: Vec<[f32; 3]> = Vec::with_capacity(keys.len());
        for (i, k) in keys.iter().enumerate() {
            let v = &m.vertex[k];
            slot.insert(*k, i as u32);
            pts.push([v.x as f32, v.y as f32, v.z as f32]);
        }
        let mut tris: Vec<u32> = Vec::new();
        let mut faces: Vec<Vec<u32>> = Vec::new();
        for f in m.faces() {
            let Some(vs) = m.face_vertices(f) else { continue };
            let mut ids: Vec<u32> = Vec::with_capacity(vs.len());
            for k in vs {
                if let Some(i) = slot.get(k) {
                    ids.push(*i);
                }
            }
            for i in 2..ids.len() {
                tris.extend_from_slice(&[ids[0], ids[i - 1], ids[i]]);
            }
            faces.push(ids);
        }
        let thickness = mesh_thickness(&pts, &tris);
        let mut seen: HashMap<[i32; 4], ()> = HashMap::new();
        for ids in &faces {
            let Some((n, d)) = face_plane(&pts, ids) else { continue };
            let key = [(n[0] * 1000.0) as i32, (n[1] * 1000.0) as i32, (n[2] * 1000.0) as i32, (d * 2.0) as i32];
            if seen.insert(key, ()).is_none() {
                self.planes.push(HostPlane { n, d, thickness });
            }
        }
    }

    /// The plane every point of `pts` lies on, if there is one.
    pub fn find(&self, pts: &[[f32; 3]]) -> Option<Host> {
        if pts.len() < 2 {
            return None;
        }
        for p in &self.planes {
            let mut on = true;
            for q in pts {
                if (q[0] * p.n[0] + q[1] * p.n[1] + q[2] * p.n[2] - p.d).abs() > ON_PLANE {
                    on = false;
                    break;
                }
            }
            if on {
                return Some(Host { normal: p.n, thickness: p.thickness });
            }
        }
        None
    }
}

/// The unit normal and offset of a polygon's plane (Newell), or `None` when degenerate.
fn face_plane(pts: &[[f32; 3]], ids: &[u32]) -> Option<([f32; 3], f32)> {
    let mut n = [0.0f32; 3];
    for i in 0..ids.len() {
        let (a, b) = (pts[ids[i] as usize], pts[ids[(i + 1) % ids.len()] as usize]);
        n[0] += (a[1] - b[1]) * (a[2] + b[2]);
        n[1] += (a[2] - b[2]) * (a[0] + b[0]);
        n[2] += (a[0] - b[0]) * (a[1] + b[1]);
    }
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len <= 0.0 {
        return None;
    }
    let n = [n[0] / len, n[1] / len, n[2] / len];
    let a = pts[ids[0] as usize];
    Some((n, a[0] * n[0] + a[1] * n[1] + a[2] * n[2]))
}
