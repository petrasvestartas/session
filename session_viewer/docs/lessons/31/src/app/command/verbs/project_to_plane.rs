use crate::State;
use crate::app::command::tool::{Next, Tool};
use crate::app::command::{Action, Spec};
use crate::app::coords;
use crate::app::cplane::CPlane;
use session_rust::{Geometry, Plane, Point, Vector, Xform};
use std::rc::Rc;

pub const SPEC: Spec = Spec {
    names: &["Project To Plane"],
    aliases: &[],
    hint: "Project To Plane: flatten the selected points, lines, polylines, curves and meshes · CPlane is the plane the view faces · XY / YZ / ZX through the origin · 3Point picks the plane · Example: Project To Plane XY",
    options: &[
        "Project To Plane CPlane",
        "Project To Plane XY",
        "Project To Plane YZ",
        "Project To Plane ZX",
        "Project To Plane 3Point",
    ],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// A plane word, or 3Point with none or three typed points.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let usage = "try Project To Plane XY, CPlane, YZ, ZX or 3Point 0,0,0 1,0,0 0,1,0";
    let word = rest
        .first()
        .map_or("cplane".into(), |word| word.to_ascii_lowercase());
    let plane = match word.as_str() {
        "cplane" => "CPlane",
        "xy" => "XY",
        "yz" => "YZ",
        "zx" | "xz" => "ZX",
        "3point" | "3points" => "3Point",
        _ => return Err(usage.into()),
    };
    let points = rest
        .iter()
        .skip(1)
        .map(|word| match coords::parse(word) {
            Some(coords::Typed::Absolute { x, y, z }) => Ok([x, y, z.unwrap_or(0.0)]),
            _ => Err(format!("`{word}` is not a world point; use x,y,z")),
        })
        .collect::<Result<Vec<_>, _>>()?;

    match (plane, points.len()) {
        ("3Point", 0 | 3) => {}
        ("3Point", _) => return Err("3Point takes three x,y,z points, or none to pick them".into()),
        (_, 0) => {}
        _ => return Err(usage.into()),
    }

    Ok(Box::new(Project { plane, points }))
}

/// Project the selection onto a named plane, or onto three points.
#[derive(Debug)]
struct Project {
    plane: &'static str,   // CPlane, XY, YZ, ZX or 3Point
    points: Vec<[f64; 3]>, // the three typed points of 3Point
}

impl Action for Project {
    /// Project at once, or start picking three points.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let normal = match self.plane {
            "3Point" => {
                if let Some(reason) = state.locked_reason(&state.selected_rows()) {
                    return Err(reason);
                }

                // texts in the selection are skipped at the end, as with a named plane
                let prompt = state.open_tool(Box::new(Picking))?;

                if self.points.is_empty() {
                    return Ok(prompt);
                }

                // typed points go in as picks, so a refused third one leaves two waiting
                let typed: Vec<String> = self
                    .points
                    .iter()
                    .map(|[x, y, z]| format!("{x},{y},{z}"))
                    .collect();
                return state.run_command(&typed.join(" "));
            }
            "XY" => CPlane::Xy.normal(),
            "YZ" => CPlane::Yz.normal(),
            "ZX" => CPlane::Xz.normal(),
            _ => CPlane::facing(&state.camera.orientation.rotate_vector(Vector::y_axis())).normal(),
        };
        let plane = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), normal, None);
        let name = match self.plane {
            "CPlane" => "the construction plane".to_string(),
            word => format!("the {word} plane"),
        };
        project(state, &plane, &name)
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Picks the plane's origin, a point on its x axis and a third point on it.
#[derive(Clone, Debug)]
struct Picking;

impl Tool for Picking {
    fn name(&self) -> &'static str {
        "Project To Plane"
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "Plane origin".into(),
            1 => "Point on the plane's x axis".into(),
            _ => "Third point on the plane".into(),
        }
    }

    /// Two points must differ and three must not be in a line; the third projects the selection.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        match points {
            [a, b] if a.distance(b, None) <= 1e-12 => {
                Err("Pick a point away from the first one".into())
            }
            [a, b, c] => {
                let plane = plane_of(a, b, c)?;
                project(state, &plane, "the picked plane").map(Next::Done)
            }
            _ => Ok(Next::More),
        }
    }
}

/// The plane through three points, x toward the second; refused when they are in a line.
fn plane_of(a: &Point, b: &Point, c: &Point) -> Result<Plane, String> {
    let (x, y) = (b - a, c - a);

    if x.cross(&y).magnitude() <= 1e-9 * x.magnitude() * y.magnitude() || x.magnitude() <= 1e-12 {
        return Err("The three points lie on one line; pick a point off the line".into());
    }

    Ok(Plane::from_points(vec![a.clone(), b.clone(), c.clone()]))
}

/// Replace every projectable selected object by its projection, in one undo step.
fn project(state: &mut State, plane: &Plane, name: &str) -> Result<String, String> {
    let rows = state.selected_rows();

    if let Some(reason) = state.locked_reason(&rows) {
        return Err(reason);
    }

    let mut edits = Vec::new();
    let mut collapsed = 0;
    let mut unsupported = 0;

    for &row in &rows {
        let (Some(geometry), Some(place)) =
            (state.scene.geometry(row), state.scene.placement_of(row))
        else {
            unsupported += 1; // a text, a streamed shell
            continue;
        };

        match projected(geometry, &place, plane) {
            Ok(geometry) => edits.push((row, geometry)),
            Err(Skip::Collapsed) => collapsed += 1,
            Err(Skip::Unsupported) => unsupported += 1,
        }
    }

    let mut skipped = Vec::new();

    if collapsed > 0 {
        skipped.push(format!("{collapsed} that would collapse to a point"));
    }

    if unsupported > 0 {
        let verb = if unsupported == 1 { "is" } else { "are" };
        skipped.push(format!(
            "{unsupported} that {verb} not a point, line, polyline, curve or mesh"
        ));
    }

    if edits.is_empty() {
        return Err(format!(
            "Nothing projected; skipped {}",
            skipped.join(" and ")
        ));
    }

    let count = state.scene.replace_rows(edits, "project to plane")?;
    state.commit_rows();
    state.place_gizmo(state.scene.selected);
    let plural = if count == 1 { "" } else { "s" };
    let skipped = if skipped.is_empty() {
        String::new()
    } else {
        format!("; skipped {}", skipped.join(" and "))
    };
    Ok(format!(
        "Projected {count} object{plural} onto {name}{skipped}. Undo restores them."
    ))
}

/// Why an object is left as it is.
#[derive(Debug, PartialEq)]
enum Skip {
    Collapsed,   // every point would land on one
    Unsupported, // not a point, line, polyline, curve or mesh
}

/// The object's geometry projected onto a world plane, in its own frame under `place`.
fn projected(geometry: &Geometry, place: &Xform, plane: &Plane) -> Result<Geometry, Skip> {
    let back = place.inverse().ok_or(Skip::Unsupported)?;
    let local = &(&back * &Xform::project_to_plane(plane)) * place; // the projection in the object's frame
    let (before, after, geometry) = match geometry {
        Geometry::Point(point) => return Ok(Geometry::Point(Rc::new(point.transformed(&local)))),
        Geometry::Line(line) => {
            let out = line.transformed(&local);
            (
                vec![line.start(), line.end()],
                vec![out.start(), out.end()],
                Geometry::Line(Rc::new(out)),
            )
        }
        Geometry::Polyline(polyline) => {
            let out = polyline.transformed(&local);
            (
                polyline.get_points(),
                out.get_points(),
                Geometry::Polyline(Rc::new(out)),
            )
        }
        Geometry::NurbsCurve(curve) => {
            let out = curve.transformed(&local);
            let cvs = |curve: &session_rust::NurbsCurve| {
                (0..curve.cv_count())
                    .filter_map(|i| curve.get_cv(i))
                    .collect()
            };
            (cvs(curve), cvs(&out), Geometry::NurbsCurve(Rc::new(out)))
        }
        Geometry::Mesh(mesh) => {
            let mut out = mesh.transformed(&local);

            // the old normals no longer fit the flat faces
            for vertex in out.vertex.values_mut() {
                for key in ["nx", "ny", "nz"] {
                    vertex.attributes.remove(key);
                }
            }

            let points = |mesh: &session_rust::Mesh| {
                mesh.vertex
                    .values()
                    .map(|vertex| vertex.position())
                    .collect()
            };
            (points(mesh), points(&out), Geometry::Mesh(Rc::new(out)))
        }
        _ => return Err(Skip::Unsupported),
    };

    if extent(&after) <= 1e-9 * extent(&before) {
        return Err(Skip::Collapsed);
    }

    Ok(geometry)
}

/// The largest distance from the first point to any other.
fn extent(points: &[Point]) -> f64 {
    let Some(first) = points.first() else {
        return 0.0;
    };
    points
        .iter()
        .map(|point| first.distance(point, None))
        .fold(0.0, f64::max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{Line, Mesh, NurbsCurve, Polyline};

    /// A point from coordinates.
    fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// The typed forms parse; wrong counts and words are refused.
    #[test]
    fn the_planes_and_points_parse() {
        let parsed =
            |rest: &[&str]| parse("Project To Plane", rest).map(|action| format!("{action:?}"));
        assert!(parsed(&["xz"]).unwrap().contains("\"ZX\""));
        assert!(parsed(&[]).unwrap().contains("\"CPlane\""));
        assert!(
            parsed(&["3points", "0,0,0", "1,0,0", "0,1,0"])
                .unwrap()
                .contains("\"3Point\"")
        );
        assert!(parsed(&["3Point"]).is_ok());

        for rest in [
            &["3Point", "0,0,0", "1,0,0"][..],
            &["3Point", "0,0,0", "1,0,0", "0,1,0", "1,1,1"],
            &["sideways"],
            &["XY", "extra"],
        ] {
            assert!(parsed(rest).is_err(), "{rest:?}");
        }
    }

    /// Point, line, polyline, rational curve and mesh land on XY; x and y stay, normals go.
    #[test]
    fn every_supported_kind_lands_on_the_plane() {
        let plane = Plane::xy_plane();
        let place = Xform::identity();
        let flat = |point: &Point, x: f64, y: f64| {
            point[2].abs() < 1e-12 && (point[0] - x).abs() < 1e-12 && (point[1] - y).abs() < 1e-12
        };

        let Ok(Geometry::Point(point)) =
            projected(&Geometry::Point(Rc::new(p(1.0, 2.0, 3.0))), &place, &plane)
        else {
            panic!("a point");
        };
        assert!(flat(&point, 1.0, 2.0));

        let line = Line::from_points(&p(0.0, 0.0, 1.0), &p(5.0, 5.0, 8.0));
        let Ok(Geometry::Line(line)) = projected(&Geometry::Line(Rc::new(line)), &place, &plane)
        else {
            panic!("a line");
        };
        assert!(flat(&line.end(), 5.0, 5.0));

        let polyline = Polyline::new(vec![
            p(0.0, 0.0, 0.0),
            p(100.0, 0.0, 50.0),
            p(100.0, 100.0, 100.0),
        ]);
        let Ok(Geometry::Polyline(polyline)) =
            projected(&Geometry::Polyline(Rc::new(polyline)), &place, &plane)
        else {
            panic!("a polyline");
        };
        assert!(flat(&polyline.get_points()[2], 100.0, 100.0));

        let mut curve = NurbsCurve::create(
            false,
            2,
            &[p(0.0, 0.0, 0.0), p(50.0, 100.0, 40.0), p(100.0, 0.0, 80.0)],
        );
        assert!(curve.set_weight(1, 0.5));
        let Ok(Geometry::NurbsCurve(out)) = projected(
            &Geometry::NurbsCurve(Rc::new(curve.clone())),
            &place,
            &plane,
        ) else {
            panic!("a curve");
        };
        assert!(flat(
            &out.get_cv(1).unwrap(),
            curve.get_cv(1).unwrap()[0],
            curve.get_cv(1).unwrap()[1]
        ));
        assert!(
            (out.weight(1) - curve.weight(1)).abs() < 1e-12,
            "weights stay"
        );

        let mut mesh = Mesh::new();
        let key = mesh.add_vertex(p(0.0, 0.0, 5.0), None);
        mesh.add_vertex(p(10.0, 0.0, 6.0), None);
        mesh.add_vertex(p(0.0, 10.0, 7.0), None);
        mesh.vertex
            .get_mut(&key)
            .unwrap()
            .attributes
            .insert("nx".into(), 1.0);
        let Ok(Geometry::Mesh(mesh)) = projected(&Geometry::Mesh(Rc::new(mesh)), &place, &plane)
        else {
            panic!("a mesh");
        };
        assert!(
            mesh.vertex
                .values()
                .all(|vertex| vertex.position()[2].abs() < 1e-12)
        );
        assert!(
            mesh.vertex
                .values()
                .all(|vertex| vertex.attributes.get("nx").is_none())
        );
    }

    /// Under a rotated, scaled and moved placement the world result lies on the world plane.
    #[test]
    fn a_placed_object_lands_on_the_world_plane() {
        let place = &(&Xform::translation(100.0, 0.0, 50.0) * &Xform::rotation_x(90.0, true))
            * &Xform::scale_xyz(10.0, 10.0, 10.0);
        let polyline = Polyline::new(vec![p(0.0, 0.0, 0.0), p(1.0, 2.0, 3.0), p(-4.0, 5.0, 6.0)]);
        let Ok(Geometry::Polyline(out)) = projected(
            &Geometry::Polyline(Rc::new(polyline.clone())),
            &place,
            &Plane::xy_plane(),
        ) else {
            panic!("a polyline");
        };

        for (before, after) in polyline.get_points().iter().zip(out.get_points()) {
            let (world, moved) = (before.transformed(&place), after.transformed(&place));
            assert!(moved[2].abs() < 1e-9);
            assert!((moved[0] - world[0]).abs() < 1e-9 && (moved[1] - world[1]).abs() < 1e-9);
        }
    }

    /// A vertical line collapses on XY; a box is not projectable.
    #[test]
    fn collapsing_and_unsupported_objects_are_skipped() {
        let line = Line::from_points(&p(5.0, 5.0, 0.0), &p(5.0, 5.0, 100.0));
        assert_eq!(
            projected(
                &Geometry::Line(Rc::new(line)),
                &Xform::identity(),
                &Plane::xy_plane()
            )
            .err(),
            Some(Skip::Collapsed)
        );
        let plane = Geometry::Plane(Rc::new(Plane::xy_plane()));
        assert_eq!(
            projected(&plane, &Xform::identity(), &Plane::xy_plane()).err(),
            Some(Skip::Unsupported)
        );
    }

    /// Points in a line make no plane; a tilted plane has the expected normal.
    #[test]
    fn three_points_in_a_line_make_no_plane() {
        assert!(plane_of(&p(0.0, 0.0, 0.0), &p(10.0, 0.0, 0.0), &p(20.0, 0.0, 0.0)).is_err());
        assert!(plane_of(&p(0.0, 0.0, 0.0), &p(0.0, 0.0, 0.0), &p(0.0, 1.0, 0.0)).is_err());
        let normal = plane_of(
            &p(0.0, 0.0, 0.0),
            &p(100.0, 0.0, 0.0),
            &p(0.0, 100.0, 100.0),
        )
        .unwrap()
        .z_axis();
        let expected = Vector::new(0.0, -1.0, 1.0).normalized();
        assert!(normal.cross(&expected).magnitude() < 1e-9);
    }
}
