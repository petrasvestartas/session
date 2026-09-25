use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{checked, count, curves, diagonal};
use crate::app::command::{Action, Spec};
use session_rust::nurbsknot::{CurveInterpStyle, CurveNurbsKnotStyle};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives};

pub const SPEC: Spec = Spec {
    names: &["Nurbs Surface Network"],
    aliases: &[
        "nurbssurface_network",
        "nurbsnurbs_network",
        "Nurbs Nurbs Network",
    ],
    hint: "Nurbs Surface Network: click curves of both directions in any order, Enter builds the surface through them",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Network",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[Step::Curves {
        prompt: "select the network curves, both directions",
        min: 2, // fewer than four is refused with the reason
        max: 32,
    }],
    axis: super::loft::no_axis,
    build,
};

const DENSITY: usize = 6; // samples per span between crossings
const MOST: usize = 64; // samples per direction at most

/// Start the network; selected curves answer at once.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

/// Where two curves cross: the parameter on each, when they come within `tolerance`.
fn crossing(
    a: &NurbsCurve,
    b: &NurbsCurve,
    boxes: (&[[f64; 3]; 2], &[[f64; 3]; 2]),
    tolerance: f64,
) -> Option<(f64, f64)> {
    let (one, two) = boxes;

    if (0..3).any(|k| one[0][k] > two[1][k] + tolerance || two[0][k] > one[1][k] + tolerance) {
        return None;
    }

    let (s, t) = a.closest_parameters_curve(b);
    (a.point_at(s).distance(&b.point_at(t), None) <= tolerance).then_some((s, t))
}

/// The low and high corners of a curve's control points.
fn bounds(curve: &NurbsCurve) -> [[f64; 3]; 2] {
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];

    for p in (0..curve.cv_count()).filter_map(|i| curve.get_cv(i)) {
        for k in 0..3 {
            low[k] = low[k].min(p[k]);
            high[k] = high[k].max(p[k]);
        }
    }

    [low, high]
}

/// The curve through `points` at even parameters.
fn interpolated(points: &[Point]) -> NurbsCurve {
    NurbsCurve::create_interpolated(
        points,
        CurveNurbsKnotStyle::Uniform,
        CurveInterpStyle::Rhino,
    )
}

/// `curve` at `x` of its domain.
fn at(curve: &NurbsCurve, x: f64) -> Point {
    let (t0, t1) = curve.domain();
    curve.point_at(t0 + x * (t1 - t0))
}

/// The piecewise-linear map from even stations 0..1 to `params`, at `x`.
fn station(params: &[f64], x: f64) -> f64 {
    let last = params.len() - 1;
    let at = (x * last as f64).clamp(0.0, last as f64);
    let low = (at.floor() as usize).min(last - 1);
    let s = at - low as f64;
    params[low] + (params[low + 1] - params[low]) * s
}

/// Samples between crossings: `DENSITY` per span, at most `MOST` in all, crossings included.
fn sample_count(crossings: usize) -> usize {
    let spans = crossings - 1;
    DENSITY.min((MOST - 1) / spans).max(1) * spans + 1
}

/// The surface through a grid of curves: Coons for four boundaries, else a Gordon surface.
pub fn network(input: &[NurbsCurve]) -> Result<NurbsSurface, String> {
    let two = "Network curves must cross in two directions";

    if input.len() > 32 {
        return Err("A network takes at most 32 curves".into());
    }

    if input.len() < 4 {
        return Err(two.into());
    }

    let points: Vec<Point> = input
        .iter()
        .flat_map(|curve| (0..curve.cv_count()).filter_map(|i| curve.get_cv(i)))
        .collect();
    let tolerance = 1e-3 * diagonal(&points);
    let boxes: Vec<[[f64; 3]; 2]> = input.iter().map(bounds).collect();
    let count = input.len();
    let mut cross = vec![vec![None; count]; count];

    for i in 0..count {
        for j in i + 1..count {
            if let Some((s, t)) = crossing(&input[i], &input[j], (&boxes[i], &boxes[j]), tolerance)
            {
                cross[i][j] = Some(s);
                cross[j][i] = Some(t);
            }
        }
    }

    // two directions: crossing curves take opposite sides, the first picked is u
    let mut side: Vec<Option<bool>> = vec![None; count];
    side[0] = Some(false);
    let mut queue = vec![0];

    while let Some(i) = queue.pop() {
        for j in 0..count {
            if cross[i][j].is_none() {
                continue;
            }

            match side[j] {
                None => {
                    side[j] = side[i].map(|s| !s);
                    queue.push(j);
                }
                Some(s) if Some(s) == side[i] => {
                    return Err("Two curves of the same direction cross".into());
                }
                _ => {}
            }
        }
    }

    if side.iter().any(Option::is_none) {
        return Err(two.into());
    }

    let mut us: Vec<usize> = (0..count).filter(|&i| side[i] == Some(false)).collect();
    let mut vs: Vec<usize> = (0..count).filter(|&i| side[i] == Some(true)).collect();

    if us.len() < 2 || vs.len() < 2 {
        return Err(two.into());
    }

    if us
        .iter()
        .any(|&i| vs.iter().any(|&j| cross[i][j].is_none()))
    {
        return Err("Every curve must cross each curve of the other direction once".into());
    }

    // v curves in order along the first u curve, u curves along the first v curve
    let first = us[0];
    vs.sort_by(|&a, &b| {
        cross[first][a]
            .unwrap()
            .total_cmp(&cross[first][b].unwrap())
    });
    let lead = vs[0];
    us.sort_by(|&a, &b| cross[lead][a].unwrap().total_cmp(&cross[lead][b].unwrap()));
    let mut curves: Vec<NurbsCurve> = input.to_vec();

    // each curve runs so its crossings increase
    for (own, others) in [(&us, &vs), (&vs, &us)] {
        for &i in own.iter() {
            let params: Vec<f64> = others.iter().map(|&j| cross[i][j].unwrap()).collect();

            if params.windows(2).all(|pair| pair[0] < pair[1]) {
                continue;
            }

            if !params.windows(2).all(|pair| pair[0] > pair[1]) {
                return Err("Every curve must cross each curve of the other direction once".into());
            }

            let (t0, t1) = curves[i].domain();
            curves[i].reverse();
            curves[i].set_domain(t0, t1);

            for &j in others.iter() {
                cross[i][j] = cross[i][j].map(|t| t0 + t1 - t);
            }
        }
    }

    if us.len() == 2 && vs.len() == 2 {
        let edge = Primitives::create_edge(
            &curves[us[0]],
            &curves[vs[1]],
            &curves[us[1]],
            &curves[vs[0]],
        );

        if edge.is_valid() {
            return Ok(edge);
        }
    }

    let (rows, columns) = (us.len(), vs.len());
    // u curve i crosses v curve j at s[i][j] on the u curve and r[i][j] on the v curve
    let s: Vec<Vec<f64>> = us
        .iter()
        .map(|&i| vs.iter().map(|&j| cross[i][j].unwrap()).collect())
        .collect();
    let r: Vec<Vec<f64>> = vs
        .iter()
        .map(|&j| us.iter().map(|&i| cross[j][i].unwrap()).collect())
        .collect();
    let crossings: Vec<Vec<Point>> = (0..rows)
        .map(|i| {
            (0..columns)
                .map(|j| {
                    let a = curves[us[i]].point_at(s[i][j]);
                    let b = curves[vs[j]].point_at(r[j][i]);
                    Point::new(
                        (a[0] + b[0]) / 2.0,
                        (a[1] + b[1]) / 2.0,
                        (a[2] + b[2]) / 2.0,
                    )
                })
                .collect()
        })
        .collect();
    let (kx, ky) = (sample_count(columns), sample_count(rows));
    let xs: Vec<f64> = (0..kx).map(|a| a as f64 / (kx - 1) as f64).collect();
    let ys: Vec<f64> = (0..ky).map(|b| b as f64 / (ky - 1) as f64).collect();
    // lofts across each family and the tensor through the crossings, sampled
    let across_u: Vec<Vec<Point>> = xs
        .iter()
        .map(|x| {
            let column: Vec<Point> = (0..rows)
                .map(|i| curves[us[i]].point_at(station(&s[i], *x)))
                .collect();
            let curve = interpolated(&column);
            ys.iter().map(|y| at(&curve, *y)).collect()
        })
        .collect();
    let across_v: Vec<Vec<Point>> = ys
        .iter()
        .map(|y| {
            let row: Vec<Point> = (0..columns)
                .map(|j| curves[vs[j]].point_at(station(&r[j], *y)))
                .collect();
            let curve = interpolated(&row);
            xs.iter().map(|x| at(&curve, *x)).collect()
        })
        .collect();
    let lines: Vec<NurbsCurve> = (0..columns)
        .map(|j| {
            interpolated(
                &(0..rows)
                    .map(|i| crossings[i][j].clone())
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let tensor: Vec<Vec<Point>> = ys
        .iter()
        .map(|y| {
            let row: Vec<Point> = lines.iter().map(|line| at(line, *y)).collect();
            let curve = interpolated(&row);
            xs.iter().map(|x| at(&curve, *x)).collect()
        })
        .collect();
    // fit: rows through the samples, then columns through the rows' control points
    let fitted: Vec<NurbsCurve> = (0..ky)
        .map(|b| {
            let row: Vec<Point> = (0..kx)
                .map(|a| {
                    let (p, q, t) = (&across_u[a][b], &across_v[b][a], &tensor[b][a]);
                    Point::new(p[0] + q[0] - t[0], p[1] + q[1] - t[1], p[2] + q[2] - t[2])
                })
                .collect();
            interpolated(&row)
        })
        .collect();
    let width = fitted[0].cv_count();
    let stacks: Vec<NurbsCurve> = (0..width)
        .map(|c| {
            interpolated(
                &fitted
                    .iter()
                    .filter_map(|row| row.get_cv(c))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();
    let (Some(u), Some(v)) = (fitted.first(), stacks.first()) else {
        return Err("The network surface could not be fitted".into());
    };
    let mut surface = NurbsSurface::new(3, false, u.order(), v.order(), width, v.cv_count());

    for i in 0..surface.nurbsknot_count(0) {
        surface.set_nurbsknot(0, i, u.nurbsknot(i).unwrap_or_default());
    }

    for i in 0..surface.nurbsknot_count(1) {
        surface.set_nurbsknot(1, i, v.nurbsknot(i).unwrap_or_default());
    }

    for (c, column) in stacks.iter().enumerate() {
        for d in 0..column.cv_count() {
            surface.set_cv(c, d, &column.get_cv(d).unwrap_or_default());
        }
    }

    Ok(surface)
}

/// The surface through the picked network.
fn build(input: &Input) -> Result<Made, String> {
    let network_curves = curves(&input.curves[0]);
    let surface = network(&network_curves)?;
    Ok(Made {
        geometries: vec![checked(surface, "network")?],
        message: format!(
            "Built a NURBS surface through {}",
            count(network_curves.len(), "curve")
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::tool::surfacing::tests::{curve, p};

    /// The middle of a surface's domain.
    fn middle(surface: &NurbsSurface) -> Point {
        let (u, v) = (surface.domain(0).unwrap(), surface.domain(1).unwrap());
        surface
            .point_at((u.0 + u.1) / 2.0, (v.0 + v.1) / 2.0)
            .unwrap()
    }

    /// The six rulings of z = xy/100, shuffled and one reversed.
    fn rulings() -> Vec<NurbsCurve> {
        vec![
            curve(&[p(0., 50., 0.), p(100., 50., 50.)]),
            curve(&[p(100., 0., 0.), p(100., 100., 100.)]),
            curve(&[p(100., 0., 0.), p(0., 0., 0.)]),
            curve(&[p(50., 0., 0.), p(50., 100., 50.)]),
            curve(&[p(0., 100., 0.), p(100., 100., 100.)]),
            curve(&[p(0., 0., 0.), p(0., 100., 0.)]),
        ]
    }

    /// A shuffled grid of rulings gives the saddle, containing every ruling.
    #[test]
    fn a_saddle_from_its_rulings() {
        let lines = rulings();
        let surface = network(&lines).unwrap();
        assert!(surface.is_valid());
        assert!(middle(&surface).distance(&p(50., 50., 25.), None) < 1e-6);

        for line in &lines {
            for q in line.divide_by_count(20, true).0 {
                assert!(surface.closest_point(&q).distance(&q, None) < 1e-3);
            }
        }
    }

    /// Four boundaries meeting at their ends take the Coons patch.
    #[test]
    fn four_boundaries_take_the_coons_patch() {
        let lines = rulings();
        let boundary = vec![
            lines[2].clone(),
            lines[1].clone(),
            lines[4].clone(),
            lines[5].clone(),
        ];
        let coons = network(&boundary).unwrap();
        assert!(coons.is_valid());
        assert!(middle(&coons).distance(&p(50., 50., 25.), None) < 1e-6);
        assert_eq!((coons.degree(0), coons.degree(1)), (1, 1));
    }

    /// Parallel lines and two crossing curves of one direction are refused.
    #[test]
    fn a_broken_network_is_refused() {
        let parallel = vec![
            curve(&[p(0., 0., 0.), p(10., 0., 0.)]),
            curve(&[p(0., 5., 0.), p(10., 5., 0.)]),
            curve(&[p(0., 10., 0.), p(10., 10., 0.)]),
            curve(&[p(0., 15., 0.), p(10., 15., 0.)]),
        ];
        assert_eq!(
            network(&parallel).unwrap_err(),
            "Network curves must cross in two directions"
        );
        assert_eq!(
            network(&parallel[..3]).unwrap_err(),
            "Network curves must cross in two directions"
        );
        let mut tangled = rulings();
        tangled.push(curve(&[p(0., 0., 0.), p(100., 100., 30.)]));
        assert!(network(&tangled).is_err());
    }
}
