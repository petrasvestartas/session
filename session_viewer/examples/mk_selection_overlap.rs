//! Selected-stroke fixtures with identical-geometry controls and original source identities.
//! Run with an output directory; the native check uses `VIEWER_SELECT=selected target`.

use serde::Serialize;
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};
use std::path::{Path, PathBuf};

type Position = [f64; 3];
type Segment = [Position; 2];

/// Original coordinates let both native captures and real browser clicks inspect one source.
#[derive(Serialize)]
struct Case {
    kind: &'static str,
    guid: String,
    pick: Position,
    segments: Vec<Segment>,
    focus: Option<Position>,
    hidden: Option<Segment>,
}

/// A black source polyline, with an exposed lead when its inspected part overlaps another row.
fn line(points: &[Position], name: &str) -> Polyline {
    let mut polyline = Polyline::new(
        points
            .iter()
            .map(|point| Point::new(point[0], point[1], point[2]))
            .collect(),
    );
    polyline.name = name.into();
    polyline.linecolor = Color::black();
    polyline
}

/// Controls suppress mesh ink while retaining exactly the same solid and physical occlusion.
fn mesh(size: Position, at: Position, reference: bool) -> Mesh {
    let mut mesh = Mesh::create_box(size[0], size[1], size[2]);
    mesh.transform(&Xform::translation(at[0], at[1], at[2]));
    mesh.set_objectcolor(Color::grey());
    let count = mesh.edges_with_colors().len();
    // One width per edge avoids the single-zero-width convention for print fills.
    mesh.set_linecolors(
        vec![Color::black(); count],
        vec![if reference { 0.0 } else { 1.0 }; count],
    );
    mesh
}

/// Perimeter, straight covered span, or a straight overlap with a unique picking lead.
fn target_points(kind: &str) -> Vec<Position> {
    match kind {
        "mesh_perimeter" => vec![
            [-420.0, -300.0, 0.0],
            [-300.0, -200.0, 0.0],
            [300.0, -200.0, 0.0],
            [300.0, 200.0, 0.0],
            [-300.0, 200.0, 0.0],
            [-300.0, -200.0, 0.0],
        ],
        "covered" => vec![[-450.0, 0.0, 0.0], [450.0, 0.0, 0.0]],
        _ => vec![[-420.0, -100.0, 0.0], [-300.0, 0.0, 0.0], [300.0, 0.0, 0.0]],
    }
}

/// Preserve source order and target GUID in the actual and transparent-ink control scenes.
fn scene(kind: &str, target: &Polyline, reference: bool) -> Session {
    let mut scene = Session::new(kind);
    let mut other = line(
        if kind == "crossing" {
            &[[0.0, -180.0, 0.0], [0.0, 180.0, 0.0]]
        } else {
            &[[-300.0, 0.0, 0.0], [300.0, 0.0, 0.0]]
        },
        "unselected black stroke",
    );
    if reference {
        other.linecolor = Color::new(0.0, 0.0, 0.0, 0.0);
    }
    match kind {
        "mesh_perimeter" => {
            scene.add_mesh(
                mesh([600.0, 400.0, 100.0], [0.0, 0.0, -50.0], reference),
                None,
            );
            scene.add_polyline(target.clone(), None);
        }
        "covered" => {
            scene.add_mesh(
                mesh([400.0, 250.0, 120.0], [0.0, 0.0, 80.0], reference),
                None,
            );
            scene.add_polyline(target.clone(), None);
        }
        "coincident_target_last" => {
            scene.add_polyline(other, None);
            scene.add_polyline(target.clone(), None);
        }
        _ => {
            scene.add_polyline(target.clone(), None);
            scene.add_polyline(other, None);
        }
    }
    scene
}

/// Measure long overlap cores, the exact crossing, and a strictly covered interior span.
fn specification(kind: &'static str, target: &Polyline) -> Case {
    let (pick, segments, focus, hidden) = match kind {
        "mesh_perimeter" => (
            [-390.0, -275.0, 0.0],
            vec![
                [[-280.0, -200.0, 0.0], [280.0, -200.0, 0.0]],
                [[300.0, -180.0, 0.0], [300.0, 180.0, 0.0]],
                [[280.0, 200.0, 0.0], [-280.0, 200.0, 0.0]],
                [[-300.0, 180.0, 0.0], [-300.0, -180.0, 0.0]],
            ],
            None,
            None,
        ),
        "covered" => (
            [-350.0, 0.0, 0.0],
            vec![
                [[-430.0, 0.0, 0.0], [-240.0, 0.0, 0.0]],
                [[240.0, 0.0, 0.0], [430.0, 0.0, 0.0]],
            ],
            None,
            Some([[-170.0, 0.0, 0.0], [170.0, 0.0, 0.0]]),
        ),
        _ => (
            [-390.0, -75.0, 0.0],
            vec![[[-280.0, 0.0, 0.0], [280.0, 0.0, 0.0]]],
            if kind == "crossing" {
                Some([0.0; 3])
            } else {
                None
            },
            None,
        ),
    };
    Case {
        kind,
        guid: target.guid().to_string(),
        pick,
        segments,
        focus,
        hidden,
    }
}

/// Write five paired PB fixtures and the source-coordinate inspection manifest.
fn generate(output: &Path) -> anyhow::Result<()> {
    std::fs::create_dir_all(output)?;
    let mut cases = Vec::new();
    for kind in [
        "mesh_perimeter",
        "coincident_target_first",
        "coincident_target_last",
        "crossing",
        "covered",
    ] {
        let target = line(&target_points(kind), "selected target");
        // Force the GUID before cloning so both files retain the same producer identity.
        cases.push(specification(kind, &target));
        for reference in [false, true] {
            let suffix = if reference { "-reference" } else { "" };
            let path = output.join(format!("{kind}{suffix}.pb"));
            scene(kind, &target, reference).pb_dump(path.to_str().expect("UTF-8 fixture path"));
        }
    }
    std::fs::write(
        output.join("cases.json"),
        serde_json::to_vec_pretty(&cases)?,
    )?;
    Ok(())
}

/// The test runner normally supplies its own output directory under Cargo's target tree.
fn main() -> anyhow::Result<()> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/selection-overlap/fixtures"));
    generate(&output)
}
