//! Wavefront OBJ -> Mesh / Polylines.
use session_rust::{Mesh, Point, Polyline};
use std::io;

pub fn write_file_obj_to_string(mesh: &Mesh) -> String {
    let (vertices, faces) = mesh.to_vertices_and_faces();
    let mut s = String::new();
    for p in vertices {
        s.push_str(&format!("v {} {} {}\n", p[0], p[1], p[2]));
    }
    for f in faces {
        if f.len() >= 3 {
            let indices: Vec<String> = f.iter().map(|i| (i + 1).to_string()).collect();
            s.push_str(&format!("f {}\n", indices.join(" ")));
        }
    }
    s
}

pub fn write_file_obj(mesh: &Mesh, filepath: &str) -> io::Result<()> {
    std::fs::write(filepath, write_file_obj_to_string(mesh))
}

pub fn read_file_obj(filepath: &str) -> io::Result<Mesh> {
    let content = std::fs::read_to_string(filepath)?;
    Ok(read_file_obj_from_str(&content))
}

pub fn read_file_obj_from_str(content: &str) -> Mesh {
    let mut verts: Vec<Point> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with("v ") {
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            let x: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
            let y: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
            let z: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
            verts.push(Point::new(x, y, z));
        } else if line.starts_with("f ") {
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            let mut face: Vec<usize> = Vec::new();
            for tok in parts {
                let first = tok.split('/').next().unwrap_or("");
                if first.is_empty() {
                    continue;
                }
                let idx: i64 = first.parse().unwrap_or(0);
                if idx == 0 {
                    continue;
                }
                let vidx = if idx > 0 {
                    (idx - 1) as usize
                } else {
                    (verts.len() as i64 + idx) as usize
                };
                face.push(vidx);
            }
            if face.len() >= 3 {
                faces.push(face);
            }
        }
    }

    let mut mesh = Mesh::new();
    let mut vkeys: Vec<usize> = Vec::with_capacity(verts.len());
    for p in verts {
        vkeys.push(mesh.add_vertex(p, None));
    }
    for f in faces {
        let vlist: Vec<usize> = f.into_iter().map(|i| vkeys[i]).collect();
        let _ = mesh.add_face(vlist, None);
    }
    mesh
}

pub fn read_file_obj_polylines(filepath: &str) -> io::Result<Vec<Polyline>> {
    let content = std::fs::read_to_string(filepath)?;
    let mut verts: Vec<Point> = Vec::new();
    let mut polylines: Vec<Polyline> = Vec::new();
    let mut curv_indices: Vec<i64> = Vec::new();
    let mut in_curv = false;

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if line.starts_with("v ") {
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            let x: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
            let y: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
            let z: f64 = parts.next().unwrap_or("0").parse().unwrap_or(0.0);
            verts.push(Point::new(x, y, z));
        } else if line.starts_with("curv ") {
            let mut parts = line.split_whitespace();
            let _ = parts.next();
            let _u0 = parts.next();
            let _u1 = parts.next();
            curv_indices.clear();
            for tok in parts {
                if let Ok(idx) = tok.parse::<i64>() { curv_indices.push(idx); }
            }
            in_curv = true;
        } else if line.starts_with("end") && in_curv {
            if !curv_indices.is_empty() {
                let mut pts: Vec<Point> = Vec::new();
                for idx in &curv_indices {
                    let vi = (*idx - 1) as usize;
                    if vi < verts.len() { pts.push(verts[vi].clone()); }
                }
                if pts.len() >= 3 { polylines.push(Polyline::new(pts)); }
            }
            in_curv = false;
        }
    }
    Ok(polylines)
}
