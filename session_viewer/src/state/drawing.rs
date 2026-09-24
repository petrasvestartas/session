use super::State;
use super::drag::Targets;
use crate::app::{
    coords,
    cplane::CPlane,
    snap::{self, SnapKind},
};
use session_rust::{Point, Polyline, Vector};

/// A shape being drawn, or points being picked for a command, not yet in the scene.
pub(crate) struct Draft {
    verb: String,                  // point, line, polyline, curve, or the command asking
    prefix: String,                // what the points complete, e.g. `Clipping Plane XY`
    needed: usize,                 // points that finish it; 0 = Enter finishes
    prompts: &'static [&'static str], // what each point is for, when a command asked
    construction: String,          // points, rectangle or polygon
    sides: usize,                  // polygon side count
    points: Vec<Point>,            // the points placed so far
    plane: CPlane,                 // the plane clicks land on
    targets: Option<Targets>,      // nearby scene snaps, collected on demand
    hover: Option<Point>,          // where the cursor is now, in the scene
    snapped: Option<SnapKind>,     // what the cursor snapped to
}

impl State {
    /// A command line entry while drawing; `None` if not one.
    pub(super) fn drawing_command(&mut self, text: &str) -> Option<Result<String, String>> {
        let words: Vec<_> = text.split_whitespace().collect();
        let verb = words.first().copied().unwrap_or("").to_ascii_lowercase();
        // Polyline Rectangle / Polygon / Points
        let construction = if verb == "polyline" {
            words
                .get(1)
                .map(|word| word.to_ascii_lowercase())
                .filter(|word| matches!(word.as_str(), "points" | "rectangle" | "polygon"))
        } else {
            None
        };
        // a drawing verb with too few points starts a draft
        if matches!(verb.as_str(), "point" | "line" | "polyline" | "curve")
            && (construction.is_some() || words.len() < if verb == "point" { 2 } else { 3 })
        {
            let start = if construction.is_some() { 2 } else { 1 }; // where the points begin
            self.cancel_split();
            // draw on the plane the camera faces most
            let plane = CPlane::facing(&self.camera.orientation.rotate_vector(Vector::y_axis()));
            let needed = match verb.as_str() {
                "point" => 1,
                "line" => 2,
                _ if construction.as_deref().is_some_and(|kind| kind != "points") => 2,
                _ => 0,
            };
            self.draft = Some(Draft {
                prefix: verb.clone(),
                verb,
                needed,
                prompts: &[],
                construction: construction.unwrap_or_else(|| "points".into()),
                sides: 6,
                points: Vec::new(),
                plane,
                targets: None,
                hover: None,
                snapped: None,
            });
            self.gpu.pick.cancel();
            // points typed on the same line count already
            return Some(if words.len() > start {
                self.accept_coordinates(&words[start..].join(" "))
            } else {
                Ok(self.drawing_prompt())
            });
        }
        let draft = self.draft.as_ref()?; // not drawing: not ours

        // one word naming an option of the verb asking switches to it, e.g. XY
        if let [word] = words.as_slice()
            && let Some(option) = crate::app::command::options(&draft.verb)
                .iter()
                .find(|option| option.eq_ignore_ascii_case(&format!("{} {word}", draft.verb)))
        {
            return Some(self.run_command(option));
        }

        // Sides N sets the polygon side count
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

    /// Start picking points for command `verb`; they finish `prefix`, one per prompt.
    pub(crate) fn ask_points(
        &mut self,
        verb: &str,
        prefix: &str,
        prompts: &'static [&'static str],
    ) -> String {
        self.cancel_split();
        // points land on the plane the camera faces most
        let plane = CPlane::facing(&self.camera.orientation.rotate_vector(Vector::y_axis()));
        self.draft = Some(Draft {
            verb: verb.into(),
            prefix: prefix.into(),
            needed: prompts.len(),
            prompts,
            construction: "points".into(),
            sides: 6,
            points: Vec::new(),
            plane,
            targets: None,
            hover: None,
            snapped: None,
        });
        self.gpu.pick.cancel();
        self.drawing_prompt()
    }

    /// Join the draft back to its first point and finish it.
    pub(crate) fn close_drawing(&mut self) -> Result<String, String> {
        let Some(draft) = self.draft.as_mut() else {
            return Err("Close works while drawing a polyline or curve".into());
        };
        if !matches!(draft.verb.as_str(), "polyline" | "curve") || draft.construction != "points" {
            return Err("Close works while drawing a polyline or curve".into());
        }
        if draft.points.len() < 3 {
            return Err("Close needs at least three points".into());
        }
        let first = draft.points[0].clone();
        draft.points.push(first);
        let result = self.finish_drawing();
        // a failed finish drops the closing point
        if result.is_err() {
            self.draft.as_mut().unwrap().points.pop();
        }
        result
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
        let limit = match draft.needed {
            0 => crate::app::modeling::MAX_POINTS,
            needed => needed,
        };
        if points.len() > limit {
            return Err(format!(
                "{} accepts at most {limit} points",
                crate::app::command::canonical(&draft.verb)
            ));
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
        if draft.needed > 0 && draft.points.len() == draft.needed {
            self.finish_drawing()
        } else {
            Ok(self.drawing_prompt())
        }
    }

    /// Turn the draft into a typed command and run it.
    fn finish_drawing(&mut self) -> Result<String, String> {
        let draft = self.draft.as_ref().unwrap();

        // a command asking for points takes all of them
        if !draft.prompts.is_empty() && draft.points.len() < draft.needed {
            return Err(format!(
                "{} needs {} points",
                crate::app::command::canonical(&draft.verb),
                draft.needed
            ));
        }

        let points = draft.geometry_points()?;
        let draft = self.draft.take().unwrap();
        // "line 0,0,0 1,1,1"
        let mut command = draft.prefix.clone();
        for p in &points {
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
        serde_json::json!({"command":draft.verb,"construction":draft.construction,"sides":draft.sides,"points":draft.points.iter().map(|p| [p[0],p[1],p[2]]).collect::<Vec<_>>(),"hover":draft.hover.as_ref().map(|p| [p[0],p[1],p[2]]),"snap":draft.snapped.map(|k| format!("{k:?}"))})
    }

    /// The verb being drawn, or empty.
    pub fn drawing_verb(&self) -> &str {
        self.draft.as_ref().map_or("", |draft| draft.verb.as_str())
    }

    /// The status line text while drawing.
    pub fn drawing_prompt(&self) -> String {
        let Some(draft) = &self.draft else {
            return String::new();
        };
        // a command's own prompt for the next point
        if let Some(prompt) = draft
            .prompts
            .get(draft.points.len().min(draft.prompts.len().saturating_sub(1)))
        {
            return format!(
                "{}: {prompt} · click or type x,y,z · Snap {} · Esc cancels",
                crate::app::command::canonical(&draft.verb),
                if self.snap_enabled { "On" } else { "Off" }
            );
        }
        // rectangle and polygon ask for two special points
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
            crate::app::command::canonical(&draft.verb),
            if self.snap_enabled { "On" } else { "Off" }
        )
    }

    /// Move the cursor while drawing: snap or land on the plane.
    pub fn hover_drawing(&mut self, x: f64, y: f64) -> bool {
        let Some(mut draft) = self.draft.take() else {
            return false;
        };
        let ray = self.camera.ray((x, y), self.viewport());
        // the nearest snap point within 12 pixels
        let hit = if self.snap_enabled {
            let screen = self.screen();
            // the draft's own points and segments, then the scene's
            let mut candidates = Vec::new();
            snap::from_polyline(&draft.points, false, OWN, &mut candidates);
            // of its own points only the start is a target: it closes the shape
            candidates.retain(|c| {
                c.kind == SnapKind::Mid || c.point.distance(&draft.points[0], None) <= 1e-12
            });
            if draft.points.len() < 2 {
                candidates.clear();
            }
            if let Some(ray) = &ray {
                let own = [(draft.points.clone(), OWN)];
                snap::along_wires(
                    &own,
                    ray,
                    draft.points.last(),
                    self.snap_modes,
                    &mut candidates,
                );
                let targets = draft
                    .targets
                    .get_or_insert_with(|| Targets::new(screen.clone(), Vec::new()));
                if let Some(hit) = self.snap_near(targets, draft.points.last(), (x, y), ray) {
                    candidates.push(hit);
                }
            }
            snap::best(
                &candidates,
                self.snap_modes,
                (x, y),
                12.0 * self.pixel_scale(),
                |p| screen.point(p),
            )
        } else {
            None
        };
        // otherwise, where the cursor ray meets the plane
        let free = ray.and_then(|(p, d)| {
            draft.plane.hit(
                draft.points.last().unwrap_or(&Point::new(0.0, 0.0, 0.0)),
                &p,
                &d,
            )
        });
        draft.snapped = hit.as_ref().map(|s| s.kind);
        draft.hover = hit.map(|s| s.point).or(free);
        self.draft = Some(draft);
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
        // the start point again closes the shape
        if draft.needed == 0 && draft.points.len() >= 2 && draft.points[0].distance(&p, None) <= 1e-12 {
            let message = self.close_drawing().unwrap_or_else(|e| e);
            self.status(&message);
            crate::app::feedback::command_line(true);
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
        // the placed points plus the cursor
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

const OWN: u32 = u32::MAX; // owner of the draft's own snaps

/// The two axes of a construction plane.
fn axes(plane: CPlane) -> (Vector, Vector) {
    match plane {
        CPlane::Xy => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0)),
        CPlane::Xz => (Vector::new(1.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
        CPlane::Yz => (Vector::new(0.0, 1.0, 0.0), Vector::new(0.0, 0.0, 1.0)),
    }
}

impl Draft {
    /// The final polyline points.
    fn geometry_points(&self) -> Result<Vec<Point>, String> {
        construction_points(&self.construction, self.plane, self.sides, &self.points)
    }
}

/// A rectangle or polygon from two points, or the points as they are.
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
    // a to b, measured along the plane axes
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
    let (cos, sin) = (u / radius, v / radius); // turn so a corner lands on b
    Ok(Polyline::from_sides(sides, radius, true)
        .get_points()
        .iter()
        .map(|p| {
            // rotate, then place on the plane
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
