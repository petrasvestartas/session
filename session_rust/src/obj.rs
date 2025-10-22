use crate::{Mesh, Point};
use std::io;

pub fn write_obj(mesh: &Mesh, filepath: &str) -> io::Result<()> {
    let (vertices, faces) = mesh.to_vertices_and_faces();
    let mut s = String::new();
    for p in vertices {
        s.push_str(&format!("v {} {} {}\n", p.x(), p.y(), p.z()));
    }
    for f in faces {
        if f.len() >= 3 {
            let indices: Vec<String> = f.iter().map(|i| (i + 1).to_string()).collect();
            s.push_str(&format!("f {}\n", indices.join(" ")));
        }
    }
    std::fs::write(filepath, s)
}

pub fn read_obj(filepath: &str) -> io::Result<Mesh> {
    let content = std::fs::read_to_string(filepath)?;
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
    Ok(mesh)
}
