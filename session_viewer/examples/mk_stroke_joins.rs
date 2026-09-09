//! Source-coordinate fixtures for joined-stroke continuity and subdivision opacity.

use serde::Serialize;
use session_rust::{Color, Point, Polyline, Session};
use std::path::PathBuf;

/// The checker projects these original vertices using each capture's actual camera.
#[derive(Serialize)]
struct Case {
    name: &'static str,
    points: Vec<[f64; 3]>,
}

fn main() -> anyhow::Result<()> {
    let output = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "stroke-joins".into()),
    );
    std::fs::create_dir_all(&output)?;
    let mut circle: Vec<_> = (0..24)
        .map(|index| {
            let angle = std::f64::consts::TAU * index as f64 / 24.0;
            [200.0 * angle.cos(), 200.0 * angle.sin(), 0.0]
        })
        .collect();
    circle.push(circle[0]); // Exact closure also tests the last-to-first adjacency.
    let cases = [
        Case {
            name: "straight",
            points: vec![[-200.0, 0.0, 0.0], [200.0, 0.0, 0.0]],
        },
        Case {
            name: "dense_straight",
            points: (0..=2048)
                .map(|index| [-200.0 + 400.0 * index as f64 / 2048.0, 0.0, 0.0])
                .collect(),
        },
        Case {
            name: "circle",
            points: circle,
        },
        Case {
            name: "acute",
            points: vec![[-200.0, 0.0, 0.0], [0.0, 0.0, 0.0], [-180.0, 20.0, 0.0]],
        },
    ];
    for case in &cases {
        let mut scene = Session::new(case.name);
        let mut line = Polyline::new(
            case.points
                .iter()
                .map(|point| Point::new(point[0], point[1], point[2]))
                .collect(),
        );
        line.name = "joined stroke".into();
        line.linecolor = Color::blue();
        scene.add_polyline(line, None);
        scene.pb_dump(output.join(format!("{}.pb", case.name)).to_str().unwrap());
    }
    std::fs::write(
        output.join("cases.json"),
        serde_json::to_vec_pretty(&cases)?,
    )?;
    Ok(())
}
