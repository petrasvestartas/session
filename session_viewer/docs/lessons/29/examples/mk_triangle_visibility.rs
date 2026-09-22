//! Fixture scenes: a seam that a nearby strip must not hide and a wide strip must.

use anyhow::Context;
use session_rust::{Color, Mesh, Point, Session};
use std::path::Path;

/// One grey quad; only its edge on x = z = 0 draws.
fn face(points: [[f64; 3]; 4]) -> Mesh {
    let mut mesh = Mesh::new();
    for point in points {
        mesh.add_vertex(Point::new(point[0], point[1], point[2]), None);
    }
    mesh.add_face(vec![0, 1, 2, 3], None);
    mesh.set_objectcolor(Color::grey());
    let edges = mesh.edges_with_colors();
    let widths = edges
        .iter()
        .map(|(first, second, _)| {
            let a = mesh.vertex_point(*first).unwrap();
            let b = mesh.vertex_point(*second).unwrap();
            if a[0] == 0.0 && b[0] == 0.0 && a[2] == 0.0 && b[2] == 0.0 {
                -1.0
            } else {
                0.0
            }
        })
        .collect();
    // one width per edge
    mesh.set_linecolors(vec![Color::black(); edges.len()], widths);
    mesh
}

/// The two quads, plus a strip for `nearby` and `hidden`.
fn scene(name: &str) -> Session {
    let mut scene = Session::new(name);
    scene.add_mesh(
        face([
            [0.0, -1000.0, 0.0],
            [0.0, -1000.0, 1000.0],
            [0.0, 1000.0, 1000.0],
            [0.0, 1000.0, 0.0],
        ]),
        None,
    );
    scene.add_mesh(
        face([
            [0.0, -1000.0, 0.0],
            [0.0, 1000.0, 0.0],
            [-1000.0, 1000.0, 0.0],
            [-1000.0, -1000.0, 0.0],
        ]),
        None,
    );
    if name != "clean" {
        // nearby: thin strip beside the seam; hidden: wide strip over it
        let (left, right, z, end_y) = if name == "hidden" {
            (-40.0, 40.0, 20.0, 1100.0)
        } else {
            (-4.0, -1.0, 0.2, 1000.0)
        };
        let mut strip = Mesh::new();
        for point in [
            [left, -end_y, z],
            [right, -end_y, z],
            [right, end_y, z],
            [left, end_y, z],
        ] {
            strip.add_vertex(Point::new(point[0], point[1], point[2]), None);
        }
        strip.add_face(vec![0, 1, 2, 3], None);
        strip.set_objectcolor(Color::grey());
        strip.set_linecolors(vec![Color::black(); 4], vec![0.0; 4]);
        scene.add_mesh(strip, None);
    }
    scene
}

/// Write clean.pb, nearby.pb and hidden.pb into `argv[1]`.
fn main() -> anyhow::Result<()> {
    let output = std::env::args()
        .nth(1)
        .context("usage: mk_triangle_visibility OUTPUT_DIRECTORY")?;
    let output = Path::new(&output);
    std::fs::create_dir_all(output)?;
    for name in ["clean", "nearby", "hidden"] {
        let file = output.join(format!("{name}.pb"));
        scene(name).pb_dump(file.to_str().context("fixture path must be UTF-8")?);
    }
    Ok(())
}
