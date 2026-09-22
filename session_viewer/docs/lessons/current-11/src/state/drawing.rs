use super::State;
use crate::app::{
    coords,
    cplane::CPlane,
    selection::Controls,
    snap::{self, Snap, SnapKind},
};
use session_rust::{Geometry, Point, Vector};

/// A shape being drawn, not yet in the scene.
pub(crate) struct Draft {
    verb: String,               // point, line, polyline or curve
    points: Vec<Point>,         // the points placed so far
    plane: CPlane,              // the plane clicks land on
    candidates: Vec<Snap>,      // scene points the cursor can snap to
    hover: Option<Point>,       // where the cursor is now, in the scene
    snapped: Option<SnapKind>,  // what the cursor snapped to
}

impl State {
    /// A command line entry while drawing; `None` if not one.
    pub(super) fn drawing_command(&mut self, text: &str) -> Option<Result<String, String>> {
        let words: Vec<_> = text.split_whitespace().collect();
        let verb = words.first().copied().unwrap_or("").to_ascii_lowercase();
        // a drawing verb with too few points starts a draft
        if matches!(verb.as_str(), "point" | "line" | "polyline" | "curve")
            && words.len() < if verb == "point" { 2 } else { 3 }
        {
            if !self.scene.streamed.is_empty() || !self.scene.sheets.is_empty() {
                return Some(Err(
                    "geometry edits require a scene without streamed sources".into(),
                ));
            }
            self.cancel_split();
            // draw on the plane the camera faces most
            let plane = CPlane::facing(&self.camera.orientation.rotate_vector(Vector::y_axis()));
            let candidates = self.drawing_candidates();
            self.draft = Some(Draft {
                verb,
                points: Vec::new(),
                plane,
                candidates,
                hover: None,
                snapped: None,
            });
            self.gpu.pick.cancel();
            return Some(if words.len() > 1 {
                self.accept_coordinates(&words[1..].join(" "))
            } else {
                Ok(self.drawing_prompt())
            });
        }
        self.draft.as_ref()?; // not drawing: not ours
        // Enter alone finishes
        if text.trim().is_empty() {
            return Some(self.finish_drawing());
        }
        // a coordinate adds a point
        if coords::parse(words.first().copied().unwrap_or("")).is_some() {
            return Some(self.accept_coordinates(text));
        }
        None
    }

    /// Every scene point the cursor can snap to.
    fn drawing_candidates(&self) -> Vec<Snap> {
        let mut out = Vec::new();
        for row in 0..self.gpu.objects.len() {
            if !self.scene.selectable(row) {
                continue;
            }
            let (Some(geometry), Some(place)) =
                (self.scene.geometry(row), self.scene.placement_of(row))
            else {
                continue;
            };
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

    /// Add typed coordinates to the draft.
    fn accept_coordinates(&mut self, text: &str) -> Result<String, String> {
        // check every word before adding any
        let draft = self.draft.as_ref().unwrap();
        let mut points = draft.points.clone();
        let (x, y) = axes(draft.plane);
        for word in text.split_whitespace() {
            let typed = coords::parse(word).ok_or("Use x,y,z, @dx,dy,dz, or distance<angle")?;
            // the direction from the last point to the cursor
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
        // how many points the verb takes
        let limit = match draft.verb.as_str() {
            "point" => 1,
            "line" => 2,
            _ => crate::app::modeling::MAX_POINTS,
        };
        if points.len() > limit {
            return Err(format!("{} accepts at most {limit} points", draft.verb));
        }
        let previous = std::mem::replace(&mut self.draft.as_mut().unwrap().points, points);
        let result = self.advance_drawing();
        // a failed finish keeps the old points
        if result.is_err() {
            self.draft.as_mut().unwrap().points = previous;
        }
        result
    }

    /// Finish when the draft has all its points, else prompt for the next.
    fn advance_drawing(&mut self) -> Result<String, String> {
        let draft = self.draft.as_ref().unwrap();
        if (draft.verb == "point" && draft.points.len() == 1)
            || (draft.verb == "line" && draft.points.len() == 2)
        {
            self.finish_drawing()
        } else {
            Ok(self.drawing_prompt())
        }
    }

    /// Turn the draft into a typed command and run it.
    fn finish_drawing(&mut self) -> Result<String, String> {
        let draft = self.draft.take().unwrap();
        // "line 0,0,0 1,1,1"
        let mut command = draft.verb.clone();
        for p in &draft.points {
            command.push_str(&format!(" {},{},{}", p[0], p[1], p[2]));
        }
        // same path as a typed command: checks, selection, undo
        let result = crate::app::command::parse(&command).and_then(|_| self.run_command(&command));
        if result.is_err() {
            self.draft = Some(draft);
        }
        result
    }

    /// The draft as JSON, for the inspection tests.
    pub fn drawing_status(&self) -> serde_json::Value {
        let Some(draft) = &self.draft else {
            return serde_json::Value::Null;
        };
        serde_json::json!({"command":draft.verb,"points":draft.points.iter().map(|p| [p[0],p[1],p[2]]).collect::<Vec<_>>(),"hover":draft.hover.as_ref().map(|p| [p[0],p[1],p[2]]),"snap":draft.snapped.map(|k| format!("{k:?}"))})
    }

    /// The status line text while drawing.
    pub fn drawing_prompt(&self) -> String {
        let Some(draft) = &self.draft else {
            return String::new();
        };
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

    /// Move the cursor while drawing: snap or land on the plane.
    pub fn hover_drawing(&mut self, x: f64, y: f64) -> bool {
        let Some(draft) = &self.draft else {
            return false;
        };
        // the nearest snap point within 12 pixels
        let hit = if self.snap_enabled {
            snap::best(&draft.candidates, (x, y), 12.0 * self.pixel_scale(), |p| {
                self.project([p[0], p[1], p[2]])
            })
        } else {
            None
        };
        // otherwise, where the cursor ray meets the plane
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

    /// Click while drawing: place a point.
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
            self.draft.as_mut().unwrap().points.pop(); // a failed finish drops the point
            e
        });
        self.status(&message);
        crate::app::feedback::command_line(true);
        true
    }

    /// The draft as screen points for the preview, plus the snap name.
    pub fn drawing_overlay(&self) -> (Vec<(f64, f64)>, String) {
        let Some(draft) = &self.draft else {
            return (Vec::new(), String::new());
        };
        let points = draft
            .points
            .iter()
            .chain(draft.hover.iter())
            .filter_map(|p| self.project([p[0], p[1], p[2]]))
            .collect();
        (
            points,
            draft.snapped.map(|k| format!("{k:?}")).unwrap_or_default(),
        )
    }
}

/// The two axes of a construction plane.
fn axes(plane: CPlane) -> (Vector, Vector) {
    match plane {
        CPlane::Xy => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        CPlane::Xz => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
        CPlane::Yz => (Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
    }
}
