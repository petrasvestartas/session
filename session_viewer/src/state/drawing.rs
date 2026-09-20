use super::State;
use crate::app::{
    coords,
    cplane::CPlane,
    selection::Controls,
    snap::{self, Snap, SnapKind},
};
use session_rust::{Geometry, Point, Polyline, Vector};

/// A draft owns no document objects until its final point is accepted.
pub(crate) struct Draft {
    verb: String,
    construction: String,
    sides: usize,
    points: Vec<Point>,
    plane: CPlane,
    candidates: Vec<Snap>,
    hover: Option<Point>,
    snapped: Option<SnapKind>,
}

impl State {
    pub(super) fn drawing_command(&mut self, text: &str) -> Option<Result<String, String>> {
        let words: Vec<_> = text.split_whitespace().collect();
        let verb = words.first().copied().unwrap_or("").to_ascii_lowercase();
        let construction = if verb == "polyline" {
            words
                .get(1)
                .map(|word| word.to_ascii_lowercase())
                .filter(|word| matches!(word.as_str(), "points" | "rectangle" | "polygon"))
        } else {
            None
        };
        if matches!(verb.as_str(), "point" | "line" | "polyline" | "curve")
            && (construction.is_some() || words.len() < if verb == "point" { 2 } else { 3 })
        {
            if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
                return Some(Err(
                    "geometry edits require a scene without streamed sources".into(),
                ));
            }
            let start = if construction.is_some() { 2 } else { 1 };
            self.cancel_split();
            let plane = CPlane::facing(&self.camera.orientation.rotate_vector(Vector::y_axis()));
            let candidates = self.drawing_candidates();
            self.draft = Some(Draft {
                verb,
                construction: construction.unwrap_or_else(|| "points".into()),
                sides: 6,
                points: Vec::new(),
                plane,
                candidates,
                hover: None,
                snapped: None,
            });
            self.gpu.pick.cancel();
            return Some(if words.len() > start {
                self.accept_coordinates(&words[start..].join(" "))
            } else {
                Ok(self.drawing_prompt())
            });
        }
        self.draft.as_ref()?;
        if verb == "sides" && self.draft.as_ref()?.construction == "polygon" {
            return Some(match words.as_slice() {
                [_, count] => match count.parse::<usize>() {
                    Ok(count) if (3..crate::app::modeling::MAX_POINTS).contains(&count) => {
                        self.draft.as_mut().unwrap().sides = count;
                        Ok(self.drawing_prompt())
                    }
                    _ => Err("Polygon sides must be between 3 and 4095".into()),
                },
                _ => Err("Use Sides 6".into()),
            });
        }
        if text.trim().is_empty() {
            return Some(self.finish_drawing());
        }
        if coords::parse(words.first().copied().unwrap_or("")).is_some() {
            return Some(self.accept_coordinates(text));
        }
        None
    }

    fn drawing_candidates(&self) -> Vec<Snap> {
        let mut out = Vec::new();
        let placements: Vec<_> = self
            .scene
            .docs
            .iter()
            .map(|doc| doc.session.world_xforms())
            .collect();
        for row in 0..self.gpu.objects.len() {
            if !self.scene.selectable(row)
                || self.gpu.objects.row(row).is_some_and(|object| {
                    object.flags & crate::engine::gpu::Instance::FLAG_HIDDEN != 0
                })
            {
                continue;
            }
            let (Some(geometry), Some((doc, guid))) =
                (self.scene.geometry(row), self.scene.identity_of(row))
            else {
                continue;
            };
            let file = &self.scene.docs[doc];
            let place = placements[doc]
                .get(guid.as_ref())
                .map_or_else(|| file.place.clone(), |world| &file.place * world);
            match geometry {
                Geometry::Point(p) => out.push(Snap {
                    point: p.transformed(&place),
                    kind: SnapKind::End,
                    owner: row,
                }),
                Geometry::Line(line) => snap::from_polyline(
                    &[
                        line.start().transformed(&place),
                        line.end().transformed(&place),
                    ],
                    false,
                    row,
                    &mut out,
                ),
                Geometry::Polyline(line) => {
                    let points: Vec<_> = line
                        .get_points()
                        .iter()
                        .map(|p| p.transformed(&place))
                        .collect();
                    snap::from_polyline(&points, false, row, &mut out);
                }
                Geometry::NurbsCurve(curve) => {
                    let (a, b) = curve.domain();
                    for t in [a, b] {
                        out.push(Snap {
                            point: curve.point_at(t).transformed(&place),
                            kind: SnapKind::End,
                            owner: row,
                        });
                    }
                }
                Geometry::Mesh(_) | Geometry::BRep(_) | Geometry::NurbsSurface(_) => {
                    let controls = Controls::from_geometry(geometry);
                    out.extend(controls.points.iter().map(|p| {
                        Snap {
                            point: Point::new(p.position[0], p.position[1], p.position[2])
                                .transformed(&place),
                            kind: SnapKind::Vertex,
                            owner: row,
                        }
                    }));
                }
                _ => {}
            }
        }
        out
    }

    fn accept_coordinates(&mut self, text: &str) -> Result<String, String> {
        // Validate the entire entry first; a malformed endpoint never commits half an entry.
        let draft = self.draft.as_ref().unwrap();
        let mut points = draft.points.clone();
        let (x, y) = axes(draft.plane);
        for word in text.split_whitespace() {
            let typed = coords::parse(word).ok_or("Use x,y,z, @dx,dy,dz, or distance<angle")?;
            let along = points.last().zip(draft.hover.as_ref()).and_then(|(p, h)| {
                let delta = Vector::new(h[0] - p[0], h[1] - p[1], h[2] - p[2]);
                let length =
                    (delta[0] * delta[0] + delta[1] * delta[1] + delta[2] * delta[2]).sqrt();
                (length > 1e-12)
                    .then(|| Vector::new(delta[0] / length, delta[1] / length, delta[2] / length))
            });
            let p = coords::resolve(
                typed,
                &Point::new(0.0, 0.0, 0.0),
                &x,
                &y,
                points.last(),
                along.as_ref(),
            )
            .ok_or("This coordinate needs a previous point or a cursor direction")?;
            if (0..3).any(|i| !p[i].is_finite() || p[i].abs() > 1e12) {
                return Err("Coordinates must be finite and within ±1e12".into());
            }
            points.push(p);
        }
        let limit = match draft.verb.as_str() {
            "point" => 1,
            "line" => 2,
            _ if draft.construction != "points" => 2,
            _ => crate::app::modeling::MAX_POINTS,
        };
        if points.len() > limit {
            return Err(format!("{} accepts at most {limit} points", draft.verb));
        }
        let previous = std::mem::replace(&mut self.draft.as_mut().unwrap().points, points);
        let result = self.advance_drawing();
        if result.is_err() {
            self.draft.as_mut().unwrap().points = previous;
        }
        result
    }

    fn advance_drawing(&mut self) -> Result<String, String> {
        let draft = self.draft.as_ref().unwrap();
        if (draft.verb == "point" && draft.points.len() == 1)
            || ((draft.verb == "line" || draft.construction != "points") && draft.points.len() == 2)
        {
            self.finish_drawing()
        } else {
            Ok(self.drawing_prompt())
        }
    }

    fn finish_drawing(&mut self) -> Result<String, String> {
        let points = self.draft.as_ref().unwrap().geometry_points()?;
        let draft = self.draft.take().unwrap();
        let mut command = draft.verb.clone();
        for p in &points {
            command.push_str(&format!(" {},{},{}", p[0], p[1], p[2]));
        }
        // Use the same validation, selection and undo path as a fully typed command.
        let result = crate::app::command::parse(&command).and_then(|_| self.run_command(&command));
        if result.is_err() {
            self.draft = Some(draft);
        }
        result
    }

    pub fn drawing_status(&self) -> serde_json::Value {
        let Some(draft) = &self.draft else {
            return serde_json::Value::Null;
        };
        serde_json::json!({"command":draft.verb,"construction":draft.construction,"sides":draft.sides,"points":draft.points.iter().map(|p| [p[0],p[1],p[2]]).collect::<Vec<_>>(),"hover":draft.hover.as_ref().map(|p| [p[0],p[1],p[2]]),"snap":draft.snapped.map(|k| format!("{k:?}"))})
    }

    pub fn drawing_verb(&self) -> &str {
        self.draft.as_ref().map_or("", |draft| draft.verb.as_str())
    }

    pub fn drawing_prompt(&self) -> String {
        let Some(draft) = &self.draft else {
            return String::new();
        };
        if draft.construction != "points" {
            let point = match (draft.construction.as_str(), draft.points.is_empty()) {
                ("rectangle", true) => "First corner",
                ("rectangle", false) => "Opposite corner",
                (_, true) => "Center",
                (_, false) => "Radius point",
            };
            let sides = if draft.construction == "polygon" {
                format!(" · {} sides (type Sides N)", draft.sides)
            } else {
                String::new()
            };
            return format!(
                "Polyline {}: {point} · click or type x,y,z{sides} · Esc cancels",
                draft.construction
            );
        }
        let point = if draft.points.is_empty() {
            "First point"
        } else {
            "Next point"
        };
        let finish = if matches!(draft.verb.as_str(), "curve" | "polyline") {
            " · Enter finishes"
        } else {
            ""
        };
        format!(
            "{}: {point} · click or type x,y,z · Snap {}{finish} · Esc cancels",
            draft.verb,
            if self.snap_enabled { "On" } else { "Off" }
        )
    }

    pub fn hover_drawing(&mut self, x: f64, y: f64) -> bool {
        let Some(draft) = &self.draft else {
            return false;
        };
        let hit = if self.snap_enabled {
            let origin = self.camera.origin();
            let matrix = self.camera.view_proj_anchored(self.aspect(), &origin).m;
            let (width, height) = self.viewport();
            snap::best(&draft.candidates, (x, y), 12.0 * self.pixel_scale(), |p| {
                let v = [p[0] - origin[0], p[1] - origin[1], p[2] - origin[2]];
                let clip: [f64; 4] = std::array::from_fn(|r| {
                    matrix[r] * v[0] + matrix[r + 4] * v[1] + matrix[r + 8] * v[2] + matrix[r + 12]
                });
                (clip[3] > 0.0).then(|| {
                    (
                        (clip[0] / clip[3] * 0.5 + 0.5) * width,
                        (0.5 - clip[1] / clip[3] * 0.5) * height,
                    )
                })
            })
        } else {
            None
        };
        let free = self.camera.ray((x, y), self.viewport()).and_then(|(p, d)| {
            draft.plane.hit(
                draft.points.last().unwrap_or(&Point::new(0.0, 0.0, 0.0)),
                &p,
                &d,
            )
        });
        let draft = self.draft.as_mut().unwrap();
        draft.snapped = hit.as_ref().map(|s| s.kind);
        draft.hover = hit.map(|s| s.point).or(free);
        true
    }

    pub fn click_drawing(&mut self, x: f64, y: f64) -> bool {
        if !self.hover_drawing(x, y) {
            return false;
        }
        let draft = self.draft.as_mut().unwrap();
        let Some(p) = draft.hover.clone() else {
            self.status("Point is outside the construction plane");
            return true;
        };
        if draft.points.len() >= crate::app::modeling::MAX_POINTS {
            self.status("Too many points");
            return true;
        }
        draft.points.push(p);
        let message = self.advance_drawing().unwrap_or_else(|e| {
            self.draft.as_mut().unwrap().points.pop();
            e
        });
        self.status(&message);
        crate::app::feedback::command_line(true);
        true
    }

    /// Screen-space preview is separate from scene geometry and cannot be picked or saved.
    pub fn drawing_overlay(&self) -> (Vec<(f64, f64)>, String) {
        let Some(draft) = &self.draft else {
            return (Vec::new(), String::new());
        };
        let mut preview = draft.points.clone();
        preview.extend(draft.hover.iter().cloned());
        let preview = construction_points(&draft.construction, draft.plane, draft.sides, &preview)
            .unwrap_or(preview);
        let points = preview
            .iter()
            .filter_map(|p| self.project([p[0], p[1], p[2]]))
            .collect();
        (
            points,
            draft.snapped.map(|k| format!("{k:?}")).unwrap_or_default(),
        )
    }
}

fn axes(plane: CPlane) -> (Vector, Vector) {
    match plane {
        CPlane::Xy => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        CPlane::Xz => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
        CPlane::Yz => (Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
    }
}

impl Draft {
    fn geometry_points(&self) -> Result<Vec<Point>, String> {
        construction_points(&self.construction, self.plane, self.sides, &self.points)
    }
}

fn construction_points(
    kind: &str,
    plane: CPlane,
    sides: usize,
    points: &[Point],
) -> Result<Vec<Point>, String> {
    if kind == "points" {
        return Ok(points.to_vec());
    }
    let [a, b] = points else {
        return Err("Pick two points to complete this polyline".into());
    };
    let (x, y) = axes(plane);
    let u: f64 = (0..3).map(|i| (b[i] - a[i]) * x[i]).sum();
    let v: f64 = (0..3).map(|i| (b[i] - a[i]) * y[i]).sum();
    if kind == "rectangle" {
        if u.abs() <= 1e-12 || v.abs() <= 1e-12 {
            return Err("Rectangle corners must define a nonzero width and height".into());
        }
        return Ok(Polyline::rectangle(a, &x, &y, u, v, true).get_points());
    }
    let radius = u.hypot(v);
    if radius <= 1e-12 {
        return Err("Polygon radius must be above zero".into());
    }
    let (cos, sin) = (u / radius, v / radius);
    Ok(Polyline::from_sides(sides, radius, true)
        .get_points()
        .iter()
        .map(|p| {
            let px = cos * p[0] - sin * p[1];
            let py = sin * p[0] + cos * p[1];
            Point::new(
                a[0] + px * x[0] + py * y[0],
                a[1] + px * x[1] + py * y[1],
                a[2] + px * x[2] + py * y[2],
            )
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_and_polygon_follow_the_construction_plane() {
        let points = [Point::new(10.0, 20.0, 30.0), Point::new(14.0, 20.0, 33.0)];
        let rectangle = construction_points("rectangle", CPlane::Xz, 6, &points).unwrap();
        assert_eq!(rectangle.len(), 5);
        assert_eq!(
            [rectangle[2][0], rectangle[2][1], rectangle[2][2]],
            [14.0, 20.0, 33.0]
        );
        let polygon = construction_points("polygon", CPlane::Xz, 5, &points).unwrap();
        assert_eq!(polygon.len(), 6);
        for p in &polygon {
            assert!(((p[0] - 10.0).hypot(p[2] - 30.0) - 5.0).abs() < 1e-9);
            assert_eq!(p[1], 20.0);
        }
        for i in 0..3 {
            assert!((polygon[0][i] - points[1][i]).abs() < 1e-9);
            assert!((polygon[0][i] - polygon[5][i]).abs() < 1e-9);
        }
        assert!(construction_points("rectangle", CPlane::Xy, 6, &points).is_err());
        assert!(construction_points("polygon", CPlane::Xy, 6, &points[..1]).is_err());
        assert!(
            construction_points(
                "polygon",
                CPlane::Xy,
                6,
                &[points[0].clone(), points[0].clone()]
            )
            .is_err()
        );
    }
}
