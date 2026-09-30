//! Fixture scenes for selected-stroke overlap checks, each with a control scene.

use serde::Serialize;
use session_rust::{Color, Mesh, Point, Polyline, Session, Xform};
use std::path::{Path, PathBuf};

type Position = [f64; 3];
type Segment = [Position; 2];

/// One case: what to pick and which segments must show.
#[derive(Serialize)]
struct Case {
    kind: &'static str,      // case name
    guid: String,            // the target polyline
    pick: Position,          // where to click
    segments: Vec<Segment>,  // spans that must show
    focus: Option<Position>, // point to zoom to
    hidden: Option<Segment>, // span that must not show
}

/// A black polyline through `points`.
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

/// A grey box; the reference version draws no edges.
fn mesh(size: Position, at: Position, reference: bool) -> Mesh {
    let mut mesh = Mesh::create_box(size[0], size[1], size[2]);
    mesh.transform(&Xform::translation(at[0], at[1], at[2]));
    mesh.set_objectcolor(Color::grey());
    let count = mesh.edges_with_colors().len();
    // one width per edge
    mesh.set_linecolors(
        vec![Color::black(); count],
        vec![if reference { 0.0 } else { 1.0 }; count],
    );
    mesh
}

/// The target polyline's points for `kind`.
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

/// The scene for `kind`; `reference` is the control without mesh ink.
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

/// The pick and the spans to check for `kind`.
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

/// Write every case as two scenes plus `cases.json`.
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
        // same guid in both files
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

/// Write into `argv[1]`, default `target/selection-overlap`.
fn main() -> anyhow::Result<()> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/selection-overlap/fixtures"));
    generate(&output)
}
