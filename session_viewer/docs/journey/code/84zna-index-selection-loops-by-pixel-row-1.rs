use session_rust::element::ElementGeometry;
use session_rust::{BRep,Geometry,Mesh,NurbsCurve,NurbsSurface,Point};
use crate::source_cut::planar;
const CURVE_STEPS: usize = 64; // steps along a free curve
pub(crate) const EDGE_STEPS: usize = 16; // steps along a BRep edge
const FACE_STEPS: usize = 8; // steps across a curved BRep face, each way
const SURFACE_STEPS: usize = 16; // steps across a surface, each way
const CLOUD_SAMPLES: usize = 65_536; // most points tested per cloud

/// A closed loop indexed by pixel row: where each row's centre line crosses it, for the even-odd test.
pub(crate) struct Region {
    top: f64,               // the first row's top edge, device pixels
    rows: Vec<Vec<f64>>,    // crossings per row, ascending
    box_: Option<[f64; 4]>, // left, top, right, bottom when the loop is a rectangle
}

/// What a region takes: what lies wholly inside it, or whatever it touches.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Reach {
    Window,   // every sample inside
    Crossing, // a sample inside, or a stroke across the rectangle
}

impl Region {
    /// Index the loop through `points`; the last point joins the first.
    pub(crate) fn new(points: &[(f64, f64)]) -> Self {
        let top = points
            .iter()
            .map(|p| p.1)
            .fold(f64::INFINITY, f64::min)
            .floor();
        let bottom = points
            .iter()
            .map(|p| p.1)
            .fold(f64::NEG_INFINITY, f64::max)
            .ceil();
        let mut rows = vec![Vec::new(); (bottom - top).max(0.0) as usize + 1];

        for (index, a) in points.iter().enumerate() {
            let b = points[(index + 1) % points.len()];
            let (low, high) = (a.1.min(b.1), a.1.max(b.1));
            let mut row = (low - top - 0.5).ceil().max(0.0) as usize;

            // rows whose centre lies in [low, high); a flat edge crosses none
            while row < rows.len() && top + row as f64 + 0.5 < high {
                let y = top + row as f64 + 0.5;
                rows[row].push(a.0 + (y - a.1) * (b.0 - a.0) / (b.1 - a.1));
                row += 1;
            }
        }

        for row in &mut rows {
            row.sort_by(f64::total_cmp);
        }

        Self {
            top,
            rows,
            box_: None,
        }
    }

    /// The rectangle with corners `a` and `b`, device pixels.
    pub(crate) fn rectangle(a: (f64, f64), b: (f64, f64)) -> Self {
        let (left, right) = (a.0.min(b.0), a.0.max(b.0));
        let (top, bottom) = (a.1.min(b.1), a.1.max(b.1));
        Self {
            box_: Some([left, top, right, bottom]),
            ..Self::new(&[(left, top), (right, top), (right, bottom), (left, bottom)])
        }
    }

}

