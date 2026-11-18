use super::deck::FlatKind;
use super::run::{Draw, Stage};
use super::tex::Rgba;
use super::tween::with_alpha;
use std::f32::consts::TAU;

pub const ARROWHEAD_LENGTH: f32 = 24.0;
pub const ARROWHEAD_ASPECT: f32 = 0.30;

fn path(pts: Vec<[f32; 2]>, color: Rgba, width: f32, dash: bool) -> Draw {
    Draw::Path {
        pts,
        closed: false,
        width,
        color,
        dash,
        fill: None,
    }
}

fn circle(at: [f32; 2], radius: f32) -> Vec<[f32; 2]> {
    (0..32)
        .map(|i| {
            let a = i as f32 * TAU / 32.0;
            [at[0] + radius * a.cos(), at[1] + radius * a.sin()]
        })
        .collect()
}

fn grow(points: &[[f64; 2]], t: f32) -> Vec<[f32; 2]> {
    let n = ((points.len() - 1) as f32 * t).clamp(0.0, (points.len() - 1) as f32);
    let whole = n.floor() as usize;
    let mut out: Vec<[f32; 2]> = points[..=whole]
        .iter()
        .map(|p| [p[0] as f32, p[1] as f32])
        .collect();
    if whole + 1 < points.len() {
        let f = n.fract();
        let a = points[whole];
        let b = points[whole + 1];
        out.push([
            (a[0] + (b[0] - a[0]) * f as f64) as f32,
            (a[1] + (b[1] - a[1]) * f as f64) as f32,
        ]);
    }
    out
}

fn arrow(a: [f32; 2], b: [f32; 2], color: Rgba, width: f32, dash: bool) -> Vec<Draw> {
    let d = [b[0] - a[0], b[1] - a[1]];
    let len = d[0].hypot(d[1]);
    if len < 1e-4 {
        return Vec::new();
    }
    let u = [d[0] / len, d[1] / len];
    let head = ARROWHEAD_LENGTH.min(len * 0.45);
    let base = [b[0] - head * u[0], b[1] - head * u[1]];
    let half = head * ARROWHEAD_ASPECT / 2.0;
    let n = [-u[1] * half, u[0] * half];
    vec![
        path(vec![a, base], color, width, dash),
        Draw::Path {
            pts: vec![
                b,
                [base[0] + n[0], base[1] + n[1]],
                [base[0] - n[0], base[1] - n[1]],
            ],
            closed: true,
            width: 0.0,
            color,
            dash: false,
            fill: Some(color),
        },
    ]
}

impl Stage {
    /// Screen-space construction ink; the 1600 × 900 authoring frame scales it with the slide.
    pub(super) fn flat_draws(&self, i: usize, now: f64, t: f32, alpha: f32) -> Vec<Draw> {
        let o = &self.objs[i];
        let super::deck::Content::Drawing(shapes) = &o.content else {
            return Vec::new();
        };
        let origin = o.pos.at(now);
        let dim = alpha * o.opacity * (1.0 - 0.7 * o.dim.at(now));
        let global = o.colors.get(&usize::MAX).map(|c| c.at(now));
        let mut out = Vec::new();
        for (j, shape) in shapes.iter().enumerate() {
            let mut color = self.color(shape.color.as_deref(), "ink");
            color = o
                .colors
                .get(&j)
                .map(|c| c.at(now))
                .or(global)
                .unwrap_or(color);
            let alpha = dim * shape.opacity.unwrap_or(1.0) as f32;
            color = with_alpha(color, alpha);
            let mut points = grow(&shape.points, t);
            for p in &mut points {
                p[0] += origin[0];
                p[1] += origin[1];
            }
            let width = shape.width.map_or(o.width, |w| w as f32);
            let fill = shape
                .fill
                .as_deref()
                .map(|name| with_alpha(self.color(Some(name), "ink"), alpha));
            match shape.kind {
                FlatKind::Point => {
                    let radius = shape.size.map_or(5.5, |s| s as f32) * t.sqrt();
                    out.push(Draw::Path {
                        pts: circle(points[0], radius),
                        closed: true,
                        width,
                        color,
                        dash: false,
                        fill: Some(with_alpha(self.palette("bg"), alpha)),
                    });
                }
                FlatKind::Line => out.push(path(points, color, width, shape.dash)),
                FlatKind::Arrow => {
                    out.extend(arrow(points[0], points[1], color, width, shape.dash));
                }
                FlatKind::Polyline => out.push(path(points, color, width, shape.dash)),
                FlatKind::Polygon => out.push(Draw::Path {
                    pts: points,
                    closed: true,
                    width,
                    color,
                    dash: shape.dash,
                    fill,
                }),
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_arrowheads_are_elongated_and_end_at_the_vector_tip() {
        let draws = arrow([0.0, 0.0], [200.0, 0.0], [1.0; 4], 3.2, false);
        let Draw::Path { pts, .. } = &draws[1] else {
            panic!("arrowhead polygon")
        };
        let length = pts[0][0] - (pts[1][0] + pts[2][0]) / 2.0;
        let width = (pts[1][1] - pts[2][1]).abs();
        assert_eq!(length, ARROWHEAD_LENGTH);
        assert_eq!(width / length, ARROWHEAD_ASPECT);
        assert_eq!(pts[0], [200.0, 0.0]);
    }
}
