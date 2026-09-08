//! Compare displayed CAD chains with actual incident triangle edges before any depth test.
use session_rust::{BRep, Color, Mesh, Point, Session};
use session_viewer::app::walk::brep::QUALITY;
use session_viewer::app::walk::brep_edges::{EdgeUse, edge_chains, iso_chain};
use std::collections::HashSet;
use std::path::Path;

/// Signed zero has the same geometric position in both floating-point display formats.
fn position_key(point: [f64; 3]) -> [u64; 3] {
    let mut key = [0; 3];
    for axis in 0..3 {
        key[axis] = if point[axis] == 0.0 {
            0
        } else {
            point[axis].to_bits()
        };
    }
    key
}

/// Compare an undirected edge without depending on source winding or map iteration order.
fn edge_key(a: [f64; 3], b: [f64; 3]) -> [[u64; 3]; 2] {
    let a = position_key(a);
    let b = position_key(b);
    if a <= b { [a, b] } else { [b, a] }
}

/// Read the producer's f64 mesh position without constructing geometry or reevaluating a curve.
fn at(mesh: &Mesh, key: usize) -> [f64; 3] {
    let vertex = &mesh.vertex[&key];
    [vertex.x, vertex.y, vertex.z]
}

/// Match the exact f64-to-f32 conversion used by the face and boundary upload paths.
fn display_position(point: [f64; 3]) -> [f64; 3] {
    [
        point[0] as f32 as f64,
        point[1] as f32 as f64,
        point[2] as f32 as f64,
    ]
}

/// Every source face in this CAD meshing contract is already a triangle.
fn mesh_edges(mesh: &Mesh) -> HashSet<[[u64; 3]; 2]> {
    let mut edges = HashSet::new();
    for vertices in mesh.face.values() {
        assert_eq!(
            vertices.len(),
            3,
            "CAD producer supplied a non-triangular face"
        );
        for edge in 0..3 {
            edges.insert(edge_key(
                at(mesh, vertices[edge]),
                at(mesh, vertices[(edge + 1) % 3]),
            ));
        }
    }
    edges
}

/// Inspect actual RenderMesh indices and positions, including shading-vertex duplication.
fn rendered_edges(mesh: &Mesh) -> HashSet<[[u64; 3]; 2]> {
    let rendered = mesh.to_render();
    let mut edges = HashSet::new();
    for triangle in rendered.indices.chunks_exact(3) {
        for edge in 0..3 {
            let a = rendered.vertices[triangle[edge] as usize].position;
            let b = rendered.vertices[triangle[(edge + 1) % 3] as usize].position;
            edges.insert(edge_key(a.map(f64::from), b.map(f64::from)));
        }
    }
    edges
}

/// Compare the complete second seam use, permitting only a reversal of its polygon.
fn same_chain(mesh: &Mesh, owner: &[usize], other: &[usize]) -> bool {
    if owner.len() != other.len() {
        return false;
    }
    let mut forward = true;
    let mut reverse = true;
    for index in 0..owner.len() {
        forward &= at(mesh, owner[index]) == at(mesh, other[index]);
        reverse &= at(mesh, owner[index]) == at(mesh, other[other.len() - 1 - index]);
    }
    forward || reverse
}

/// Resolve a seam use before comparing its complete ordered polygon.
fn same_iso_use(brep: &BRep, mesh: &Mesh, owner: &[usize], use_: &EdgeUse) -> Option<bool> {
    let keys = iso_chain(brep, mesh, use_)?;
    Some(same_chain(mesh, owner, &keys))
}

/// Report exact per-use geometric incidence independently from rasterization or normal facing.
fn audit(brep: &BRep) -> serde_json::Value {
    let meshes = brep.face_meshes_q(Some(QUALITY));
    let chains = edge_chains(brep, &meshes);
    let mut source_edges = Vec::new();
    let mut display_edges = Vec::new();
    for mesh in &meshes {
        source_edges.push(mesh_edges(mesh));
        display_edges.push(rendered_edges(mesh));
    }
    let mut face_records = Vec::new();
    for (face, mesh) in meshes.iter().enumerate() {
        face_records.push(face_audit(face, mesh));
    }
    let mut records = Vec::new();
    for (edge, chain) in chains.iter().enumerate() {
        let Some(chain) = chain else {
            records.push(serde_json::json!({"edge":edge,"available":false,"degenerate":brep.m_edges[edge].degenerated}));
            continue;
        };
        let owner = &meshes[chain.face];
        let mut uses = Vec::new();
        for face_use in brep.edge_faces(edge) {
            let face = face_use.index as usize;
            let mut missing_f64 = Vec::new();
            let mut missing_f32 = Vec::new();
            for (segment, pair) in chain.keys.windows(2).enumerate() {
                let a = at(owner, pair[0]);
                let b = at(owner, pair[1]);
                if a == b {
                    continue;
                }
                if !source_edges[face].contains(&edge_key(a, b)) {
                    missing_f64.push(segment);
                }
                if !display_edges[face]
                    .contains(&edge_key(display_position(a), display_position(b)))
                {
                    missing_f32.push(segment);
                }
            }
            let exact_seam_use = if face == chain.face {
                same_iso_use(
                    brep,
                    owner,
                    &chain.keys,
                    &EdgeUse {
                        edge,
                        face,
                        orientation: face_use.orientation,
                    },
                )
            } else {
                None
            };
            uses.push(serde_json::json!({"face":face,"orientation":format!("{:?}",face_use.orientation),
                "missing_mesh_segments":missing_f64,"missing_render_segments":missing_f32,"same_iso_use":exact_seam_use}));
        }
        let mut analytic_chord_deviation = 0.0f64;
        let source = &brep.m_edges[edge];
        if let Some(curve) = brep.m_curves_3d.get(source.curve_3d_index as usize) {
            for pair in chain.keys.windows(2) {
                let a = at(owner, pair[0]);
                let b = at(owner, pair[1]);
                let midpoint = Point::new(
                    (a[0] + b[0]) * 0.5,
                    (a[1] + b[1]) * 0.5,
                    (a[2] + b[2]) * 0.5,
                );
                let on_curve = curve.point_at(curve.closest_parameter(&midpoint));
                analytic_chord_deviation =
                    analytic_chord_deviation.max(on_curve.distance(&midpoint, None));
            }
        }
        records.push(
            serde_json::json!({"edge":edge,"available":true,"owner":chain.face,
            "segments":chain.keys.len()-1,"closed":chain.keys.first()==chain.keys.last(),
            "curve_midpoint_deviation":analytic_chord_deviation,"uses":uses,"keys":chain.keys}),
        );
    }
    serde_json::json!({"name":brep.name,"guid":brep.guid(),"faces":meshes.len(),"face_data":face_records,"edges":records})
}

/// Measure each face independently and retain the exact mesh for seam/node diagnostic plots.
fn face_audit(face: usize, mesh: &Mesh) -> serde_json::Value {
    let mut vertices = Vec::new();
    let mut reference = [0.0; 3];
    let mut origin = [0.0; 3];
    let mut largest_area = 0.0;
    for triangle in mesh.face.values() {
        let a = at(mesh, triangle[0]);
        let b = at(mesh, triangle[1]);
        let c = at(mesh, triangle[2]);
        let e = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let f = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let n = [
            e[1] * f[2] - e[2] * f[1],
            e[2] * f[0] - e[0] * f[2],
            e[0] * f[1] - e[1] * f[0],
        ];
        let area = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if area > largest_area {
            largest_area = area;
            origin = a;
            reference = [n[0] / area, n[1] / area, n[2] / area];
        }
    }
    let mut plane_residual = 0.0f64;
    let mut normal_min_dot = 1.0f64;
    for (&key, vertex) in &mesh.vertex {
        let position = at(mesh, key);
        let delta = [
            position[0] - origin[0],
            position[1] - origin[1],
            position[2] - origin[2],
        ];
        plane_residual = plane_residual.max(
            (delta[0] * reference[0] + delta[1] * reference[1] + delta[2] * reference[2]).abs(),
        );
        let normal = vertex.normal().unwrap();
        normal_min_dot = normal_min_dot.min(
            (normal[0] * reference[0] + normal[1] * reference[1] + normal[2] * reference[2]).abs(),
        );
        vertices.push(serde_json::json!({"key":key,"position":position,"normal":[normal[0],normal[1],normal[2]],"u":vertex.attributes.get("u"),"v":vertex.attributes.get("v")}));
    }
    serde_json::json!({"face":face,"vertices":vertices,"triangles":mesh.face,"plane_residual":plane_residual,"normal_min_abs_dot_plane":normal_min_dot,"plane_normal":reference})
}

/// Emit reusable primitive PB fixtures and audit any supplied complete Session files.
fn main() {
    let mut arguments = std::env::args().skip(1);
    let output = arguments
        .next()
        .expect("output directory, followed by optional Session PB paths");
    let output = Path::new(&output);
    std::fs::create_dir_all(output).unwrap();
    let mut records = Vec::new();
    for (name, mut brep) in [
        ("cylinder", BRep::create_cylinder(120.0, 240.0)),
        ("cylinder_tall", BRep::create_cylinder(150.0, 400.0)),
        ("cone", BRep::create_cone(120.0, 240.0)),
        ("sphere", BRep::create_sphere(160.0)),
        ("torus", BRep::create_torus(160.0, 50.0)),
    ] {
        brep.name = name.into();
        brep.surfacecolor = Color::grey();
        records.push(audit(&brep));
        let mut session = Session::new(name);
        session.add_brep(brep, None);
        session.pb_dump(output.join(format!("{name}.pb")).to_str().unwrap());
    }
    for path in arguments {
        let session = Session::pb_load(&path);
        for brep in &session.objects.breps {
            records.push(audit(brep));
            let mut isolated = Session::new(&brep.name);
            isolated.add_brep((**brep).clone(), None);
            if let Some(transform) = session.xforms.get(brep.guid()) {
                isolated.set_xform(brep.guid(), transform.clone());
            }
            isolated.pb_dump(
                output
                    .join(format!("exact-{}.pb", brep.name))
                    .to_str()
                    .unwrap(),
            );
        }
    }
    let json = serde_json::to_string_pretty(&records).unwrap();
    std::fs::write(output.join("geometry.json"), json).unwrap();
    println!(
        "Audited {} BReps; geometry.json contains every original edge and incident use",
        records.len()
    );
}
