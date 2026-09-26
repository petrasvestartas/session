// --8<-- [start:draft]
use super::State;
use super::drag::Targets;
use crate::app::command::tool::Tool;
use crate::app::command::verbs::geometry::Draw;
use crate::app::{
    coords,
    cplane::CPlane,
    snap::{self, SnapKind},
};
use session_rust::{Plane, Point, Polyline, Vector, Xform};

/// The command being drawn: points placed so far, the plane they land on and the tool asking; none of it is in the document yet.
pub(crate) struct Draft {
    pub(super) verb: String, // point, line, polyline, curve, select, or the command asking
    draw: Option<&'static Draw>, // the drawing verb, when one draws
    prefix: String,          // what the points complete, e.g. `Clipping Plane XY`
    needed: usize,           // points that finish it; 0 = Enter finishes
    prompts: &'static [&'static str], // what each point is for, when a command asked
    construction: String,    // points, rectangle or polygon
    sides: usize,            // polygon side count
    pub(super) points: Vec<Point>, // the points placed so far
    plane: CPlane,           // the plane clicks land on
    pub(super) frame: Plane, // that plane as axes, z toward the viewer
    pub(super) targets: Option<Targets>, // nearby scene snaps, collected on demand
    pub(super) hover: Option<Point>, // where the cursor is now, in the scene
    pub(super) snapped: Option<SnapKind>, // what the cursor snapped to
    pub(super) tool: Option<Box<dyn Tool>>, // the command asking, when it is a tool
    pub(super) then: Option<String>, // a `select` draft runs this line once objects are picked
    pub(super) group: Vec<(u32, Xform)>, // rows a tool previews and their placements
    pub(super) moved: bool,  // those rows show a preview
}

impl Draft {
    /// Nothing placed yet, points landing on `plane`.
    pub(super) fn new(verb: &str, prefix: &str, plane: CPlane) -> Self {
        let (x, y) = axes(plane);
        Self {
            verb: verb.into(),
            draw: None,
            prefix: prefix.into(),
            needed: 0,
            prompts: &[],
            construction: "points".into(),
            sides: 6,
            points: Vec::new(),
            plane,
            frame: Plane::new(Point::new(0.0, 0.0, 0.0), x, y),
            targets: None,
            hover: None,
            snapped: None,
            tool: None,
            then: None,
            group: Vec::new(),
            moved: false,
        }
    }
}

impl State {
    /// True while a command is being typed or drawn.
    pub(crate) fn drafting(&self) -> bool {
        self.features.draft.is_some()
    }
}
// --8<-- [end:draft]

// --8<-- [start:drawing-command]
impl State {
    /// A command line entry while drawing; `None` if not one.
    pub(super) fn drawing_command(&mut self, text: &str) -> Option<Result<String, String>> {
        let words: Vec<_> = text.split_whitespace().collect();
        let verb = words.first().copied().unwrap_or("").to_ascii_lowercase();

        // a drawing verb with too few points starts a draft
        if let Some((draw, count)) = crate::app::command::drawing(&words) {
            // one of its options names a construction, e.g. Polyline Rectangle
            let construction = words
                .get(count)
                .filter(|word| {
                    draw.spec.options.iter().any(|option| {
                        option.eq_ignore_ascii_case(&format!("{} {word}", draw.spec.names[0]))
                    })
                })
                .map(|word| word.to_ascii_lowercase());
            let start = count + usize::from(construction.is_some()); // where the points begin

            if construction.is_some() || words.len() - count < *draw.points.start() {
                self.cancel_drawing();
                self.cancel_split(); // register:split
                let needed = match &construction {
                    _ if !draw.open() => *draw.points.start(),
                    Some(kind) if kind != "points" => 2,
                    _ => 0,
                };
                let verb = words[..count].join(" ").to_ascii_lowercase();
                // draw on the plane the camera faces most
                let mut draft = Draft::new(&verb, &verb, self.facing());
                draft.draw = Some(draw);
                draft.needed = needed;
                draft.construction = construction.unwrap_or_else(|| "points".into());
                self.features.draft = Some(draft);
                self.gpu.pick.cancel();
                // points typed on the same line count already
                return Some(if words.len() > start {
                    self.accept_coordinates(&words[start..].join(" "))
                } else {
                    Ok(self.drawing_prompt())
                });
            }
        }
        self.features.draft.as_ref()?; // `?` on an Option: not drawing, so None tells run_command to parse the line as a command

        // a tool or an object pick takes its own words
        if let Some(result) = self.tool_command(text) {
            return Some(result);
        }

        let draft = self.features.draft.as_ref()?;

        // one word naming an option of the verb asking switches to it, e.g. XY
        if let [word] = words.as_slice()
            && let Some(option) = crate::app::command::options(&draft.verb)
                .iter()
                .find(|option| option.eq_ignore_ascii_case(&format!("{} {word}", draft.verb)))
        {
            return Some(self.run_command(option));
        }

        // Sides N sets the polygon side count
        if verb == "sides" && self.features.draft.as_ref()?.construction == "polygon" {
            return Some(match words.as_slice() {
                [_, count] => match count.parse::<usize>() {
                    Ok(count) if (3..crate::app::modeling::MAX_POINTS).contains(&count) => {
                        self.features.draft.as_mut().unwrap().sides = count;
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
    // --8<-- [end:drawing-command]

    // --8<-- [start:drawing-points]
    /// Start picking points for command `verb`; they finish `prefix`, one per prompt.
    pub(crate) fn ask_points(
        &mut self,
        verb: &str,
        prefix: &str,
        prompts: &'static [&'static str],
    ) -> String {
        self.cancel_drawing();
        self.cancel_split(); // register:split
        // points land on the plane the camera faces most
        let mut draft = Draft::new(verb, prefix, self.facing());
        draft.needed = prompts.len();
        draft.prompts = prompts;
        self.features.draft = Some(draft);
        self.gpu.pick.cancel();
        self.drawing_prompt()
    }

    /// The construction plane the camera faces most.
    pub(super) fn facing(&self) -> CPlane {
        CPlane::facing(&self.camera.orientation.rotate_vector(Vector::y_axis()))
    }

    /// Join the draft back to its first point and finish it.
    pub(crate) fn close_drawing(&mut self) -> Result<String, String> {
        let Some(draft) = self.features.draft.as_mut() else {
            return Err("Close works while drawing a polyline or curve".into());
        };
        if !draft.draw.is_some_and(Draw::open) || draft.construction != "points" {
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
            self.features.draft.as_mut().unwrap().points.pop();
        }
        result
    }

    /// Add typed coordinates to the draft.
    pub(super) fn accept_coordinates(&mut self, text: &str) -> Result<String, String> {
        // check every word before adding any
        let draft = self.features.draft.as_ref().unwrap();
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
        // `mem::replace` swaps the new points in and hands back the old ones, kept to restore if finishing fails.
        let previous = std::mem::replace(&mut self.features.draft.as_mut().unwrap().points, points);
        let result = self.advance_drawing();
        // a failed finish keeps the old points
        if result.is_err() {
            self.features.draft.as_mut().unwrap().points = previous;
        }
        result
    }

    /// Finish when the draft has all its points, else prompt for the next.
    fn advance_drawing(&mut self) -> Result<String, String> {
        let draft = self.features.draft.as_ref().unwrap();

        if draft.tool.is_some() {
            return self.tool_placed();
        }

        if draft.needed > 0 && draft.points.len() == draft.needed {
            self.finish_drawing()
        } else {
            Ok(self.drawing_prompt())
        }
    }

    /// Turn the draft into a typed command and run it.
    fn finish_drawing(&mut self) -> Result<String, String> {
        let draft = self.features.draft.as_ref().unwrap();

        // a command asking for points takes all of them
        if !draft.prompts.is_empty() && draft.points.len() < draft.needed {
            return Err(format!(
                "{} needs {} points",
                crate::app::command::canonical(&draft.verb),
                draft.needed
            ));
        }

        let points = draft.geometry_points()?;
        let draft = self.features.draft.take().unwrap();
        // "line 0,0,0 1,1,1"
        let mut command = draft.prefix.clone();
        for p in &points {
            command.push_str(&format!(" {},{},{}", p[0], p[1], p[2]));
        }
        // same path as a typed command: checks, selection, undo
        let result = crate::app::command::parse(&command).and_then(|_| self.run_command(&command));
        if result.is_err() {
            self.features.draft = Some(draft);
        }
        result
    }
    // --8<-- [end:drawing-points]

    // --8<-- [start:drawing-prompt]
    /// The draft as JSON, for the inspection tests.
    pub fn drawing_status(&self) -> serde_json::Value {
        let Some(draft) = &self.features.draft else {
            return serde_json::Value::Null;
        };
        serde_json::json!({"command":draft.verb,"tool":draft.tool.is_some(),"construction":draft.construction,"sides":draft.sides,"points":draft.points.iter().map(|p| [p[0],p[1],p[2]]).collect::<Vec<_>>(),"hover":draft.hover.as_ref().map(|p| [p[0],p[1],p[2]]),"snap":draft.snapped.map(|k| format!("{k:?}"))})
    }

    /// The status line text while drawing.
    pub fn drawing_prompt(&self) -> String {
        let Some(draft) = &self.features.draft else {
            return String::new();
        };

        if let Some(prompt) = self.tool_prompt() {
            return prompt;
        }

        // a command's own prompt for the next point
        if let Some(prompt) = draft.prompts.get(
            draft
                .points
                .len()
                .min(draft.prompts.len().saturating_sub(1)),
        ) {
            return format!(
                "{}: {prompt} · click or type x,y,z · Snap {} · Esc cancels",
                crate::app::command::canonical(&draft.verb),
                if self.features.snap.enabled {
                    "On"
                } else {
                    "Off"
                }
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
        let finish = if draft.draw.is_some_and(Draw::open) {
            " · Enter finishes"
        } else {
            ""
        };
        format!(
            "{}: {point} · click or type x,y,z · Snap {}{finish} · Esc cancels",
            crate::app::command::canonical(&draft.verb),
            if self.features.snap.enabled {
                "On"
            } else {
                "Off"
            }
        )
    }
    // --8<-- [end:drawing-prompt]

    // --8<-- [start:drawing-cursor]
    /// Move the cursor while drawing: snap or land on the plane.
    pub fn hover_drawing(&mut self, x: f64, y: f64) -> bool {
        // a tool that follows the cursor itself
        if let Some(changed) = self.tool_hover(x, y) {
            return changed;
        }

        // picking objects: the cursor is only a cursor
        if self
            .features
            .draft
            .as_ref()
            .is_none_or(|draft| draft.then.is_some())
        {
            return false;
        }

        // Take the draft out of `self`: its fields and `self`'s methods can then be borrowed together; it goes back at the end.
        let mut draft = self.features.draft.take().unwrap();
        let tool = draft.tool.is_some();
        let ray = self.camera.ray((x, y), self.viewport());
        // the nearest snap point within 12 pixels
        let hit = if self.features.snap.enabled {
            let screen = self.screen();
            // the draft's own points and segments, then the scene's
            let mut candidates = Vec::new();
            snap::from_polyline(&draft.points, false, OWN, &mut candidates);
            // of its own points only the start is a target: it closes the shape
            candidates.retain(|c| {
                c.kind == SnapKind::Mid || c.point.distance(&draft.points[0], None) <= 1e-12
            });
            // a tool's own points close nothing
            if draft.points.len() < 2 || tool {
                candidates.clear();
            }
            if let Some(ray) = &ray {
                let own = [(
                    if tool {
                        Vec::new()
                    } else {
                        draft.points.clone()
                    },
                    OWN,
                )];
                snap::along_wires(
                    &own,
                    ray,
                    draft.points.last(),
                    self.features.snap.modes,
                    &mut candidates,
                );
                let targets = draft
                    .targets
                    .get_or_insert_with(|| Targets::new(screen.clone(), Vec::new())); // collected on the first hover, reused for every move after
                if let Some(hit) = self.snap_near(targets, draft.points.last(), (x, y), ray) {
                    candidates.push(hit);
                }
            }
            snap::best(
                &candidates,
                self.features.snap.modes,
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
        self.features.draft = Some(draft);

        if tool {
            self.preview_tool();
        }

        true
    }

    /// Click while drawing: place a point.
    pub fn click_drawing(&mut self, x: f64, y: f64) -> bool {
        // a tool that picks objects or takes clicks itself
        if let Some(redraw) = self.tool_click(x, y) {
            return redraw;
        }

        // picking objects for a command: each click adds one; false, since a redraw would cancel the pick
        if self
            .features
            .draft
            .as_ref()
            .is_some_and(|draft| draft.then.is_some())
        {
            self.additive_selection = true;
            self.request_selection(x as u32, y as u32, false, false);
            return false;
        }

        if !self.hover_drawing(x, y) {
            return false;
        }
        let draft = self.features.draft.as_mut().unwrap();
        let Some(p) = draft.hover.clone() else {
            self.status("Point is outside the construction plane");
            return true;
        };
        if draft.points.len() >= crate::app::modeling::MAX_POINTS {
            self.status("Too many points");
            return true;
        }
        // the start point again closes the shape
        if draft.needed == 0
            && draft.tool.is_none()
            && draft.points.len() >= 2
            && draft.points[0].distance(&p, None) <= 1e-12
        {
            let message = self.close_drawing().unwrap_or_else(|e| e);
            self.status(&message);
            crate::app::feedback::command_line(true);
            return true;
        }
        draft.points.push(p);
        let message = self.advance_drawing().unwrap_or_else(|e| {
            self.features.draft.as_mut().unwrap().points.pop(); // a failed finish drops the point

            e
        });
        self.status(&message);
        crate::app::feedback::command_line(true);
        true
    }
    // --8<-- [end:drawing-cursor]

    // --8<-- [start:drawing-overlay]
    /// The draft as screen points for the preview, plus the snap name.
    pub fn drawing_overlay(&self) -> (Vec<(f64, f64)>, String) {
        let Some(draft) = &self.features.draft else {
            return (Vec::new(), String::new());
        };

        if let Some(overlay) = self.tool_overlay() {
            return overlay;
        }

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

    /// Buttons under the command line while drawing: (label, line it runs); "" is Enter.
    pub fn drawing_options(&self) -> &'static [(&'static str, &'static str)] {
        let Some(draft) = &self.features.draft else {
            return &[];
        };

        if let Some(tool) = &draft.tool {
            return tool.options();
        }

        if draft.verb == "select" {
            return &[("Done", ""), ("Cancel", "Escape")];
        }

        draft.draw.map_or(&[], |draw| draw.buttons)
    }
}
// --8<-- [end:drawing-overlay]

// --8<-- [start:drawing-construction]
const OWN: u32 = u32::MAX; // owner id of the draft's own snaps; no scene row is this large

/// The two axes of a construction plane; x × y faces the viewer in Top, Front and Right.
pub(super) fn axes(plane: CPlane) -> (Vector, Vector) {
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
// --8<-- [end:drawing-construction]

// --8<-- [start:drawing-tests]
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
// --8<-- [end:drawing-tests]
