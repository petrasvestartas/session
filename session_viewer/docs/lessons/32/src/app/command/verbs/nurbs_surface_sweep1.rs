use crate::app::command::tool::gather::{self, Input, Made, Recipe, Step};
use crate::app::command::tool::surfacing::{SAMPLES, checked, count, curves};
use crate::app::command::{Action, Spec};
use session_rust::{NurbsCurve, NurbsSurface, Point, Primitives, Xform};

pub const SPEC: Spec = Spec {
    names: &["Nurbs Surface Sweep1"],
    aliases: &["nurbssurface_sweep1"],
    hint: "Nurbs Surface Sweep1: the rail, then profile curves, Enter sweeps",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

pub static RECIPE: Recipe = Recipe {
    name: "Nurbs Surface Sweep1",
    chips: &[("Finish", ""), ("Cancel", "Escape")],
    steps: &[
        Step::Curves {
            prompt: "select the rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
    ],
    axis: super::loft::no_axis,
    build,
};

/// Start sweeping; selected curves are the rail, then profiles.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    gather::start(&RECIPE, rest)
}

/// The rail parameter nearest the profile's middle and world to the rail frame there.
fn on_rail(rail: &NurbsCurve, profile: &NurbsCurve) -> (f64, Xform) {
    let mut points = profile.divide_by_count(SAMPLES, true).0;

    // a closed profile's end repeats its start
    if profile.is_closed() {
        points.pop();
    }

    let count = points.len().max(1) as f64;
    let sum = points.iter().fold([0.0; 3], |sum, p| {
        [sum[0] + p[0], sum[1] + p[1], sum[2] + p[2]]
    });
    let middle = Point::new(sum[0] / count, sum[1] / count, sum[2] / count);
    let t = rail.closest_parameter(&middle);
    let frame = rail.perpendicular_plane_at(t, false);
    let local = Xform::world_to_frame(
        &frame.origin(),
        &frame.x_axis(),
        &frame.y_axis(),
        &frame.z_axis(),
    );
    (t, local)
}

/// The profiles carried along the rail, each kept where it sits; between two they blend.
pub fn sweep1(rail: &NurbsCurve, profiles: &[NurbsCurve]) -> Result<NurbsSurface, String> {
    if profiles.is_empty() {
        return Err("Select at least 1 profile".into());
    }

    let mut placed: Vec<(f64, NurbsCurve)> = profiles
        .iter()
        .map(|profile| {
            let (t, local) = on_rail(rail, profile);
            (t, profile.transformed(&local))
        })
        .collect();
    placed.sort_by(|a, b| a.0.total_cmp(&b.0));
    let (stations, locals): (Vec<f64>, Vec<NurbsCurve>) = placed.into_iter().unzip();
    let shared = if locals.len() == 1 {
        locals
    } else {
        // a straight loft makes the profiles share knots; its rows are the profiles again
        let bridge = Primitives::create_loft(&locals, 1);
        let rows: Vec<NurbsCurve> = bridge
            .get_nurbsknots(1)
            .iter()
            .take(locals.len())
            .filter_map(|v| bridge.iso_curve(0, *v))
            .collect();

        if rows.len() != locals.len() {
            return Err("The profiles could not be matched to each other".into());
        }

        rows
    };

    // frames at even lengths plus one at every profile, so the surface runs through them
    let count = (rail.span_count() * 2 + 1).clamp(5, 200);
    let mut params = rail.divide_by_count(count + 1, true).1;
    let (t0, t1) = rail.domain();
    let near = 1e-6 * (t1 - t0);
    params.retain(|t| stations.iter().all(|s| (t - s).abs() > near));
    params.extend_from_slice(&stations);
    params.sort_by(f64::total_cmp);
    params.dedup_by(|a, b| (*a - *b).abs() <= near);
    let mut sections = Vec::with_capacity(params.len());

    for t in params {
        let upper = stations.partition_point(|s| *s < t);
        let mut section = match upper {
            0 => shared[0].clone(),
            n if n == stations.len() => shared[n - 1].clone(),
            n => blend(
                &shared[n - 1],
                &shared[n],
                (t - stations[n - 1]) / (stations[n] - stations[n - 1]),
            ),
        };
        section.transform(&Xform::to_frame(&rail.perpendicular_plane_at(t, false)));
        sections.push(section);
    }

    let surface = Primitives::create_loft(&sections, 3);

    if !surface.is_valid() {
        return Err("The profiles could not be swept along the rail".into());
    }

    Ok(surface)
}

/// The curve `s` of the way from `a` to `b`, control point by control point.
fn blend(a: &NurbsCurve, b: &NurbsCurve, s: f64) -> NurbsCurve {
    let mut section = a.clone();

    for i in 0..a.cv_count() {
        let (Some(p), Some(q)) = (a.get_cv_4d(i), b.get_cv_4d(i)) else {
            continue;
        };
        let mix = |x: f64, y: f64| x + (y - x) * s;
        section.set_cv_4d(
            i,
            mix(p.0, q.0),
            mix(p.1, q.1),
            mix(p.2, q.2),
            mix(p.3, q.3),
        );
    }

    section
}

/// The sweep of the profiles along the rail.
fn build(input: &Input) -> Result<Made, String> {
    let profiles = curves(&input.curves[1]);
    let surface = sweep1(&input.curves[0][0].curve, &profiles)?;
    Ok(Made {
        geometries: vec![checked(surface, "sweep")?],
        message: format!("Swept {} along the rail", count(profiles.len(), "profile")),
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

    /// A closed polyline square of half size `h` around `(x, 0, 0)` in the plane x = `x`.
    fn square(x: f64, h: f64) -> NurbsCurve {
        let corners = [
            p(x, -h, -h),
            p(x, h, -h),
            p(x, h, h),
            p(x, -h, h),
            p(x, -h, -h),
        ];
        let mut square = NurbsCurve::create(false, 1, &corners);
        square.set_domain(0.0, 1.0);
        square
    }

    /// True when every control point of `profile` lies on `surface`.
    fn runs_through(surface: &NurbsSurface, profile: &NurbsCurve) -> bool {
        (0..profile.cv_count())
            .filter_map(|i| profile.get_cv(i))
            .all(|q| surface.closest_point(&q).distance(&q, None) < 1e-6)
    }

    /// A profile stays where it is drawn and is carried rigidly along the rail.
    #[test]
    fn one_profile_keeps_its_place_on_the_rail() {
        let rail = curve(&[p(0., 0., 0.), p(50., 50., 0.), p(100., 0., 0.)]);
        let profile = square(0.0, 5.0);
        let surface = sweep1(&rail, std::slice::from_ref(&profile)).unwrap();
        assert!(surface.is_closed(0));
        assert!(runs_through(&surface, &profile));
        let off = middle(&surface).distance(&rail.point_at(0.5), None);
        assert!(
            (off - 50f64.sqrt()).abs() < 0.2,
            "middle {off} from the rail"
        );
    }

    /// Profiles sweep in rail order whatever order they were picked, each kept in place.
    #[test]
    fn profiles_blend_between_their_own_places() {
        let rail = curve(&[p(0., 0., 0.), p(50., 50., 0.), p(100., 0., 0.)]);
        let (big, small) = (square(0.0, 5.0), square(100.0, 2.0));
        let one = sweep1(&rail, &[big.clone(), small.clone()]).unwrap();
        let two = sweep1(&rail, &[small.clone(), big.clone()]).unwrap();
        assert!(runs_through(&one, &big) && runs_through(&one, &small));
        assert!(middle(&one).distance(&middle(&two), None) < 1e-9);
        assert!(sweep1(&rail, &[]).is_err());
    }
}
