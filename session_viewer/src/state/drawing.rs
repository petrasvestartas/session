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

/// A shape being drawn, or points being picked for a command, not yet in the scene.
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
    plane_points: Option<Vec<Point>>, // `3 Point` picking a construction plane mid-draw: origin, x axis, y side
}

impl Draft {
    /// Nothing placed yet, points landing on `plane`.
    pub(super) fn new(verb: &str, prefix: &str, plane: CPlane) -> Self {
        let (x, y) = plane.axes();
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
            frame: Plane::new(plane.origin(), x, y),
            targets: None,
            hover: None,
            snapped: None,
            tool: None,
            then: None,
            group: Vec::new(),
            moved: false,
            plane_points: None,
        }
    }

    /// Points now land on `plane`.
    fn land_on(&mut self, plane: CPlane) {
        let (x, y) = plane.axes();
        self.plane = plane;
        self.frame = Plane::new(plane.origin(), x, y);
    }

    /// True while clicks place points, not pick objects.
    fn places_points(&self) -> bool {
        self.then.is_none() && self.tool.as_ref().is_none_or(|tool| tool.asks_points())
    }
}

impl State {
    /// True while a command is being typed or drawn.
    pub(crate) fn drafting(&self) -> bool {
        self.features.draft.is_some()
    }
}

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
        self.features.draft.as_ref()?; // not drawing: not ours

        // XY, XZ, YZ or 3 Point switches the construction plane
        if let Some(result) = self.plane_word(&words) {
            return Some(result);
        }

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

    /// The construction plane points land on: the fixed one, else the one the camera faces most.
    pub(super) fn facing(&self) -> CPlane {
        self.features.construction_plane.unwrap_or_else(|| {
            CPlane::facing(&self.camera.orientation.rotate_vector(Vector::y_axis()))
        })
    }

    /// Fix the construction plane, or None to follow the view again; the grid moves onto it and a running draft lands on it.
    pub(crate) fn set_construction_plane(&mut self, plane: Option<CPlane>) {
        self.features.construction_plane = plane;
        let shown = plane.unwrap_or(CPlane::Xy);
        self.gpu.backdrop.set_grid_frame(&self.gpu.ctx, shown.matrix());
        self.camera.grid = plane.map(|plane| {
            let o = plane.origin();
            [o[0], o[1], o[2]]
        });
        let facing = self.facing();

        if let Some(draft) = self.features.draft.as_mut() {
            draft.land_on(facing);
        }

        self.touch();
    }

    /// The construction plane as the inspection snapshot shows it: mode, origin and axes.
    pub(crate) fn construction_plane_status(&self) -> serde_json::Value {
        let plane = self.facing();
        let (x, y) = plane.axes();
        let o = plane.origin();
        let mode = match self.features.construction_plane {
            None => "View",
            Some(CPlane::Xy) => "XY",
            Some(CPlane::Xz) => "XZ",
            Some(CPlane::Yz) => "YZ",
            Some(CPlane::Frame { .. }) => "3 Point",
        };
        serde_json::json!({
            "mode": mode,
            "origin": [o[0], o[1], o[2]],
            "x": [x[0], x[1], x[2]],
            "y": [y[0], y[1], y[2]],
        })
    }

    /// `XY`, `XZ`, `YZ` or `3 Point` while placing points: the plane changes without leaving the command.
    fn plane_word(&mut self, words: &[&str]) -> Option<Result<String, String>> {
        let draft = self.features.draft.as_mut()?;

        if !draft.places_points() {
            return None;
        }

        let word = words.concat().to_ascii_lowercase();
        let preset = match word.as_str() {
            "xy" => CPlane::Xy,
            "xz" => CPlane::Xz,
            "yz" => CPlane::Yz,
            "3point" => {
                draft.plane_points = Some(Vec::new());
                return Some(Ok(self.drawing_prompt()));
            }
            _ => return None,
        };
        self.set_construction_plane(Some(preset));
        Some(Ok(self.drawing_prompt()))
    }

    /// One point of a `3 Point` plane picked mid-draw; the third sets the plane and drawing goes on.
    fn take_plane_point(&mut self, point: Point) -> Result<String, String> {
        let draft = self.features.draft.as_mut().unwrap();
        let points = draft.plane_points.get_or_insert_with(Vec::new);
        points.push(point);

        if points.len() < 3 {
            return Ok(self.drawing_prompt());
        }

        let picked = draft.plane_points.take().unwrap();
        let plane = CPlane::from_3_points(&picked[0], &picked[1], &picked[2])
            .ok_or("The three points are in line; pick them again")?;
        self.set_construction_plane(Some(plane));
        Ok(self.drawing_prompt())
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

        // typed points of a 3 Point plane, in world coordinates
        if draft.plane_points.is_some() {
            let mut answer = Ok(self.drawing_prompt());
            for word in text.split_whitespace() {
                let p = coords::parse(word)
                    .and_then(|typed| {
                        let (x, y) = CPlane::Xy.axes();
                        let last = self.features.draft.as_ref()?.plane_points.as_ref()?.last().cloned();
                        coords::resolve(typed, &Point::new(0.0, 0.0, 0.0), &x, &y, last.as_ref(), None)
                    })
                    .ok_or("Use x,y,z or @dx,dy,dz for the plane's points")?;
                answer = self.take_plane_point(p);
            }
            return answer;
        }

        let mut points = draft.points.clone();
        let (x, y) = draft.plane.axes();
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

        // a 3 Point plane picked mid-draw
        if let Some(points) = &draft.plane_points {
            let what = ["Plane origin", "Point on the plane's x axis", "Point on the plane's y side"]
                [points.len().min(2)];
            return format!("{what} · click or type x,y,z · Esc cancels");
        }

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
                    .get_or_insert_with(|| Targets::new(screen.clone(), Vec::new()));
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
                &draft.points.last().cloned().unwrap_or_else(|| draft.plane.origin()),
                &p,
                &d,
            )
        });
        // off every object, Grid Snap rounds the plane point to the grid
        let grid = self.features.snap.grid && hit.is_none() && free.is_some();
        let free = match grid {
            true => {
                free.map(|p| snap::on_grid(&p, draft.plane, self.features.snap.grid_step))
            }
            false => free,
        };
        draft.snapped = hit
            .as_ref()
            .map(|s| s.kind)
            .or(grid.then_some(SnapKind::Grid));
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
        // a point of a 3 Point plane, not of the shape
        if draft.plane_points.is_some() {
            let message = self.take_plane_point(p).unwrap_or_else(|e| e);
            self.status(&message);
            return true;
        }
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

    /// The running command's buttons beside the command line, values included: (label, line it runs); "" is Enter.
    pub fn tool_buttons(&self) -> Vec<(String, String)> {
        let Some(draft) = self.features.draft.as_ref() else {
            return Vec::new();
        };
        let mut buttons: Vec<(String, String)> = match &draft.tool {
            Some(tool) => tool.buttons(),
            None => self
                .drawing_options()
                .iter()
                .map(|(label, line)| (label.to_string(), line.to_string()))
                .collect(),
        };

        // while placing points the construction plane is one click away
        if draft.places_points()
            && draft.plane_points.is_none()
            && draft.verb != crate::app::command::verbs::construction_plane::NAME
        {
            let planes = ["XY", "XZ", "YZ", "3 Point"].map(|word| (word.to_string(), word.to_string()));
            buttons.splice(0..0, planes);
        }

        buttons
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

const OWN: u32 = u32::MAX; // owner of the draft's own snaps

/// The two axes of a construction plane; x × y faces the viewer in Top, Front and Right.
pub(super) fn axes(plane: CPlane) -> (Vector, Vector) {
    plane.axes()
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
    fn a_draft_switches_to_a_tilted_plane_and_draws_on_it() {
        let mut draft = Draft::new("polyline", "polyline", CPlane::Xy);
        assert!(draft.places_points());
        let tilted = CPlane::from_3_points(
            &Point::new(0.0, 0.0, 0.0),
            &Point::new(1000.0, 0.0, 1000.0),
            &Point::new(0.0, 1000.0, 0.0),
        )
        .unwrap();
        draft.land_on(tilted);
        assert_eq!(draft.plane, tilted);
        let (x, _) = tilted.axes();
        assert!((draft.frame.x_axis()[0] - x[0]).abs() < 1e-12 && (draft.frame.x_axis()[2] - x[2]).abs() < 1e-12);
        // a ray straight down lands on the tilted plane, z rising with x
        let hit = draft
            .plane
            .hit(&draft.plane.origin(), &Point::new(500.0, 200.0, 9000.0), &Vector::new(0.0, 0.0, -1.0))
            .unwrap();
        assert!((hit[2] - 500.0).abs() < 1e-9);
        // a rectangle on it keeps its corners on the plane
        let corners = [tilted.world([0.0, 0.0, 0.0]), tilted.world([300.0, 200.0, 0.0])];
        let rectangle = construction_points("rectangle", tilted, 6, &corners).unwrap();
        assert_eq!(rectangle.len(), 5);
        for p in &rectangle {
            assert!(tilted.local(p)[2].abs() < 1e-9, "on the plane");
        }
        let far = tilted.local(&rectangle[2]);
        assert!((far[0] - 300.0).abs() < 1e-9 && (far[1] - 200.0).abs() < 1e-9);
    }

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
