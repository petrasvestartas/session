//! A finite foreground triangle must not occlude ink outside its own projected footprint.

use anyhow::Context;
use session_rust::{Color, Mesh, Point, Session};
use std::path::Path;

/// Make one rectangular face, drawing only the shared seam at x = z = 0.
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
    // A width for every edge avoids the single-zero-width print-fill convention.
    mesh.set_linecolors(vec![Color::black(); edges.len()], widths);
    mesh
}

/// The clean and nearby scenes have identical bounds and therefore the same fitted camera.
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
        // In the fitted perspective camera, the seam ray intersects z=0.2 near x=-0.181,
        // outside the nearby strip's finite x interval [-4,-1]. Extending its depth plane
        // beyond the triangle incorrectly hides the seam. The wider z=20 strip really
        // covers the seam and must suppress every ink pixel; it has no ink of its own.
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

/// Write the clean seam, nearby non-occluding strip and genuinely covering strip fixtures.
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
