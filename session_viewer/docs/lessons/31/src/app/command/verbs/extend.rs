use super::geometry::{Edit, range};
use crate::State;
use crate::app::command::tool::cut::{
    Cutter, as_curve, as_plane, plane_moved, samples, segment_px,
};
use crate::app::command::tool::{Next, Overlay, Stroke, Tool, typed_number};
use crate::app::command::{Action, Spec, number};
use crate::app::modeling::Interval;
use crate::app::scene::Shape;
use reach::{End, Grown, Reach};
use session_rust::{AABB, Geometry, Mesh, Plane, Point, Xform};
use std::rc::Rc;

mod reach;

pub const SPEC: Spec = Spec {
    names: &["Extend"],
    aliases: &[],
    hint: "Extend · pick boundaries, Enter, click near curve ends · Extend Distance 5 grows ends by a length · Extend -0.2 1.2 extends a curve's domain",
    options: &["Extend Distance"],
    arity: None,
    wait_for_option: false,
    wait_after_option: true,
    parse,
};

const APERTURE_CSS: f64 = 12.0; // how near the cursor must be to a curve
const MAX_SAMPLES: usize = 200_000; // points kept for finding curves under the cursor
const RED: [u8; 3] = [210, 40, 40];

/// Grow curve ends to boundaries or by a length, or stretch a curve's domain.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let distance = |word: &str| word.eq_ignore_ascii_case("distance");

    match rest {
        [] => Ok(Box::new(Extend {
            distance: None,
            asking: false,
        })),
        [word] if distance(word) => Ok(Box::new(Extend {
            distance: None,
            asking: true,
        })),
        [word, value] if distance(word) => Ok(Box::new(Extend {
            distance: Some(length(value)?),
            asking: false,
        })),
        [value] => Ok(Box::new(Extend {
            distance: Some(length(value)?),
            asking: false,
        })),
        _ => {
            let (a, b) = range(rest)?;
            Ok(Box::new(Edit(Interval::Extend(a, b))))
        }
    }
}

/// A length above zero.
fn length(word: &str) -> Result<f64, String> {
    match number(Some(word), "Extend Distance 5")? {
        value if value > 0.0 => Ok(value),
        _ => Err("The distance must be above zero".into()),
    }
}

/// Start extending; selected objects are the boundaries.
#[derive(Debug)]
struct Extend {
    distance: Option<f64>, // grow by this much instead of to boundaries
    asking: bool,          // the length is typed next
}

impl Action for Extend {
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.open_tool(Box::new(Extending {
            distance: self.distance,
            asking: self.asking,
            ..Extending::default()
        }))
    }
}

/// The curve end under the cursor and what it would become.
struct Hover {
    row: u32,                                      // the curve
    end: End,                                      // its end
    grown: Result<(Geometry, Vec<Point>), String>, // the new curve and the added piece in the world
}

/// The running extend.
#[derive(Default)]
struct Extending {
    curves: bool,                                // picking curve ends, not boundaries
    before: Vec<u32>,                            // the selection before Extend
    boundaries: Vec<u32>,                        // rows that stop the ends
    distance: Option<f64>,                       // grow by this, world units
    asking: bool,                                // a typed length comes next
    cutters: Vec<(u32, Cutter)>,                 // boundary curves and planes in the world
    meshes: Vec<(u32, Rc<Mesh>, Xform)>,         // boundary meshes and their placements
    candidates: Vec<(u32, AABB, Vec<[f64; 3]>)>, // open curves as world points and their boxes
    waiting: Vec<u32>,                           // curves whose documents are released
    wanted: Vec<(usize, u32)>, // released documents asked for, with one of their rows
    hover: Option<Hover>,
}

impl std::fmt::Debug for Extending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extending({:?})", self.distance)
    }
}

impl Extending {
    /// Add `row` as a boundary, or take it away again.
    fn toggle(&mut self, state: &mut State, row: u32) -> Result<(), String> {
        if let Some(at) = self.boundaries.iter().position(|r| *r == row) {
            self.boundaries.remove(at);
            state.gpu.set_selected(row, false);
            return Ok(());
        }

        // a released document is fetched for the next click
        if state.scene.released_doc(row).is_some() {
            return Err(state.locked_reason(&[row]).unwrap_or_default());
        }

        let geometry = state
            .scene
            .geometry(row)
            .ok_or("This object has no geometry to stop at")?;
        let fits = as_curve(geometry).is_some()
            || as_plane(geometry).is_some()
            || matches!(geometry, Geometry::Mesh(_) | Geometry::BRep(_));

        if !fits {
            return Err("Stop at a line, polyline, curve, Plane, surface, mesh or BRep".into());
        }

        self.boundaries.push(row);
        state.gpu.set_selected(row, true);
        Ok(())
    }

    /// Go on to the curve ends: boundaries in the world, open curves sampled once.
    fn start_curves(&mut self, state: &mut State) {
        self.curves = true;
        self.asking = false;
        self.cutters.clear();
        self.meshes.clear();

        for &row in &self.boundaries {
            let (Some(geometry), Some(place)) =
                (state.scene.geometry(row), state.scene.placement_of(row))
            else {
                continue;
            };

            if let Some(plane) = as_plane(geometry).and_then(|plane| plane_moved(&plane, &place)) {
                self.cutters.push((row, Cutter::Plane(plane)));
            } else if let Some(curve) = as_curve(geometry) {
                self.cutters
                    .push((row, Cutter::Curve(curve.transformed(&place))));
            } else if let Geometry::Mesh(mesh) = geometry {
                self.meshes.push((row, Rc::clone(mesh), place));
            } else if let Geometry::BRep(brep) = geometry {
                for mesh in brep.face_meshes_q(Some(crate::app::walk::brep::QUALITY)) {
                    self.meshes.push((row, Rc::new(mesh), place.clone()));
                }
            }
        }

        self.candidates.clear();
        self.waiting.clear();
        let mut total = 0;

        for row in 0..state.scene.row_count() as u32 {
            if total >= MAX_SAMPLES {
                break;
            }

            if let Some(points) = sampled(state, row) {
                total += points.len();
                self.candidates.push(candidate(row, points));
            } else if released_curve(state, row) {
                self.waiting.push(row);
            }
        }
    }

    /// Ask for the released documents of curves near `at` on screen; the message to show.
    fn fetch_near(&mut self, state: &mut State, at: (f64, f64)) -> String {
        let screen = state.view_screen();
        let aperture = APERTURE_CSS * state.pixel_scale();
        let mut message = "Click near the end of a line, polyline or curve".to_string();

        for &row in &self.waiting {
            let Some(bounds) = state.gpu.objects.row_bounds(row) else {
                continue;
            };
            let Some((x, y, radius)) = screen.circle(&bounds) else {
                continue;
            };

            if (x - at.0).hypot(y - at.1) > radius + aperture
                || !boxed(&screen, &bounds, at, aperture)
            {
                continue;
            }

            let Some(doc) = state.scene.released_doc(row) else {
                continue;
            };

            if let Err(error) = state.scene.editable(doc) {
                message = error;

                if !self.wanted.iter().any(|(owner, _)| *owner == doc) {
                    self.wanted.push((doc, row));
                }
            }
        }

        state.fetch_wanted();
        message
    }

    /// Sample the waiting curves once a wanted document came back.
    fn arrived(&mut self, state: &State) {
        let count = self.wanted.len();
        self.wanted
            .retain(|(_, row)| state.scene.released_doc(*row).is_some());

        if self.wanted.len() == count {
            return;
        }

        let (back, waiting) = self
            .waiting
            .iter()
            .partition(|row| state.scene.released_doc(**row).is_none());
        self.waiting = waiting;

        for row in back {
            if let Some(points) = sampled(state, row) {
                self.candidates.push(candidate(row, points));
            }
        }
    }

    /// The reach of `row`'s end in its own frame.
    fn reach(&self, row: u32, geometry: &Geometry, end: End, place: &Xform) -> Option<Reach> {
        let back = place.inverse()?;

        if let Some(distance) = self.distance {
            // a world length along the end's direction, measured in the curve's frame
            let (tip, direction) = reach::tip(geometry, end)?;
            let from = tip.transformed(place);
            let mut along = &(&tip + &direction).transformed(place) - &from;
            along.normalize_self().then_some(())?;
            let to = (&from + &(&along * distance)).transformed(&back);
            return Some(Reach::Distance(to.distance(&tip, None)));
        }

        let cutters = self
            .cutters
            .iter()
            .filter(|(owner, _)| *owner != row)
            .filter_map(|(_, cutter)| cutter.moved(&back))
            .collect();
        let meshes = self
            .meshes
            .iter()
            .filter(|(owner, ..)| *owner != row)
            .filter_map(|(_, mesh, at)| Some((Rc::clone(mesh), &at.inverse()? * place)))
            .collect();
        Some(Reach::Boundaries { cutters, meshes })
    }

    /// The curve end nearest `at` on screen and its growth; the last answer is kept while it stays.
    fn follow(&mut self, state: &State, at: (f64, f64)) -> bool {
        if !self.wanted.is_empty() {
            self.arrived(state);
        }

        let screen = state.view_screen();
        let aperture = APERTURE_CSS * state.pixel_scale();
        let mut best: Option<(f64, u32, End)> = None;
        let mut pixels = Vec::new(); // one candidate on screen, reused

        for (row, bounds, points) in &self.candidates {
            // a curve whose box is far from the cursor costs one projection
            if let Some((x, y, radius)) = screen.circle(bounds)
                && (x - at.0).hypot(y - at.1) > radius + aperture
            {
                continue;
            }

            pixels.clear();
            pixels.extend(points.iter().map(|p| screen.xyz(*p)));
            let mut walked = 0.0; // world length up to each segment
            let total: f64 = points
                .windows(2)
                .map(|pair| apart(&pair[0], &pair[1]))
                .sum();

            for (index, pair) in pixels.windows(2).enumerate() {
                let step = apart(&points[index], &points[index + 1]);

                if let [Some(a), Some(b)] = pair {
                    let distance = segment_px(*a, *b, at);

                    if distance <= aperture && best.is_none_or(|(near, ..)| distance < near) {
                        // the half the cursor is on picks the end
                        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                        let t = (((at.0 - a.0) * dx + (at.1 - a.1) * dy)
                            / (dx * dx + dy * dy).max(1e-12))
                        .clamp(0.0, 1.0);
                        let end = if walked + t * step < total * 0.5 {
                            End::Start
                        } else {
                            End::End
                        };
                        best = Some((distance, *row, end));
                    }
                }

                walked += step;
            }
        }

        let Some((_, row, end)) = best else {
            let changed = self.hover.is_some();
            self.hover = None;
            return changed;
        };

        if self
            .hover
            .as_ref()
            .is_some_and(|hover| hover.row == row && hover.end == end)
        {
            return false;
        }

        let grown = self.grow(state, row, end);
        self.hover = Some(Hover { row, end, grown });
        true
    }

    /// `row` grown at `end`, and the added piece in the world.
    fn grow(&self, state: &State, row: u32, end: End) -> Result<(Geometry, Vec<Point>), String> {
        let geometry = state.scene.geometry(row).ok_or("This curve is gone")?;
        let place = state
            .scene
            .placement_of(row)
            .ok_or("This curve has no placement")?;
        let reach = self
            .reach(row, geometry, end, &place)
            .ok_or("This curve cannot be measured")?;
        let Grown { geometry, tail } = reach::extended(geometry, end, &reach)?;
        Ok((
            geometry,
            tail.iter().map(|p| p.transformed(&place)).collect(),
        ))
    }

    /// Clear the boundary highlights.
    fn unmark(&self, state: &mut State) {
        for &row in &self.boundaries {
            state.gpu.set_selected(row, false);
        }
    }
}

/// A visible, editable line, polyline or curve as world points; a closed one says it has no free end.
fn sampled(state: &State, row: u32) -> Option<Vec<[f64; 3]>> {
    let geometry = state.scene.geometry(row)?;
    let curve = match geometry {
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_) => as_curve(geometry)?,
        _ => return None,
    };

    if !state.scene.selectable(row) || state.scene.display_only(row) {
        return None;
    }

    if state
        .scene
        .identity_of(row)
        .is_some_and(|id| state.scene.hidden.contains(&id))
    {
        return None;
    }

    let place = state.scene.placement_of(row)?;
    let world = |p: &Point| {
        let q = place.transform_point(p);
        [q[0], q[1], q[2]]
    };
    Some(samples(&curve, 32).iter().map(world).collect())
}

/// True for a visible, unlocked line, polyline or curve whose document is released.
fn released_curve(state: &State, row: u32) -> bool {
    state.scene.released_doc(row).is_some()
        && matches!(
            state.scene.shape(row),
            Some(Shape::Line | Shape::Polyline | Shape::Curve)
        )
        && state.scene.selectable(row)
        && !state
            .scene
            .identity_of(row)
            .is_some_and(|id| state.scene.hidden.contains(&id))
}

/// True when `at` is within `aperture` pixels of the screen rectangle around a world box.
fn boxed(screen: &crate::app::snap::Screen, bounds: &AABB, at: (f64, f64), aperture: f64) -> bool {
    let (mut low, mut high) = ([f64::MAX; 2], [f64::MIN; 2]);

    for corner in 0..8 {
        let sign = |bit: i32| if corner & bit == 0 { -1.0 } else { 1.0 };
        let p = [
            bounds.cx + sign(1) * bounds.hx,
            bounds.cy + sign(2) * bounds.hy,
            bounds.cz + sign(4) * bounds.hz,
        ];
        let Some((x, y)) = screen.xyz(p) else {
            return true; // behind the eye: keep it
        };
        low = [low[0].min(x), low[1].min(y)];
        high = [high[0].max(x), high[1].max(y)];
    }

    at.0 >= low[0] - aperture
        && at.0 <= high[0] + aperture
        && at.1 >= low[1] - aperture
        && at.1 <= high[1] + aperture
}

/// A candidate curve with the box around its points.
fn candidate(row: u32, points: Vec<[f64; 3]>) -> (u32, AABB, Vec<[f64; 3]>) {
    let (mut low, mut high) = ([f64::MAX; 3], [f64::MIN; 3]);

    for p in &points {
        for axis in 0..3 {
            low[axis] = low[axis].min(p[axis]);
            high[axis] = high[axis].max(p[axis]);
        }
    }

    let bounds = AABB::new(
        (low[0] + high[0]) * 0.5,
        (low[1] + high[1]) * 0.5,
        (low[2] + high[2]) * 0.5,
        (high[0] - low[0]) * 0.5,
        (high[1] - low[1]) * 0.5,
        (high[2] - low[2]) * 0.5,
    );
    (row, bounds, points)
}

/// The distance between two points given as coordinates.
fn apart(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

impl Tool for Extending {
    fn name(&self) -> &'static str {
        "Extend"
    }

    fn prompt(&self, _points: &[Point]) -> String {
        match (self.curves, self.distance) {
            _ if self.asking => "type the distance to extend by".into(),
            (false, _) => {
                "select boundary objects · Enter continues · Distance extends by a length".into()
            }
            (true, Some(distance)) => format!(
                "click near the end of a line, polyline or curve to add {distance} · type a number to change · Enter ends"
            ),
            (true, None) => format!(
                "click near the end of a line, polyline or curve to reach the {} {} · Enter ends",
                self.boundaries.len(),
                if self.boundaries.len() == 1 {
                    "boundary"
                } else {
                    "boundaries"
                }
            ),
        }
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        match self.curves {
            false => &[("Distance", "Distance"), ("Next", ""), ("Cancel", "Escape")],
            true => &[("Distance", "Distance"), ("Done", ""), ("Cancel", "Escape")],
        }
    }

    fn asks_points(&self) -> bool {
        false
    }

    /// Selected objects become the boundaries; a length goes straight to the curves.
    fn begin(&mut self, state: &mut State) -> Result<Next, String> {
        self.before = state.selected_rows();
        state.select(None);
        state.place_gizmo(None);

        if self.distance.is_some() {
            self.start_curves(state);
            return Ok(Next::More);
        }

        for row in self.before.clone() {
            let _ = self.toggle(state, row);
        }

        Ok(Next::More)
    }

    fn picks(&self) -> bool {
        !self.curves
    }

    fn picked(&mut self, state: &mut State, row: Option<u32>) -> Result<Next, String> {
        let row = row.ok_or("Nothing there: click a boundary object")?;
        self.toggle(state, row)?;
        Ok(Next::More)
    }

    /// `Distance` asks for a length; a number sets it.
    fn word(
        &mut self,
        state: &mut State,
        word: &str,
        _points: &[Point],
        _plane: &Plane,
    ) -> Option<Result<Next, String>> {
        if word.eq_ignore_ascii_case("distance") {
            self.asking = true;
            return Some(Ok(Next::More));
        }

        let value = typed_number(word)?;

        if value <= 0.0 {
            return Some(Err("The distance must be above zero".into()));
        }

        self.distance = Some(value);
        self.hover = None;

        if !self.curves {
            self.unmark(state);
            self.start_curves(state);
        }

        self.asking = false;
        Some(Ok(Next::More))
    }

    fn placed(
        &mut self,
        _state: &mut State,
        _points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        Err("Extend: click near a curve end, or type a distance".into())
    }

    fn enter(&mut self, state: &mut State, _points: &[Point]) -> Result<Next, String> {
        if !self.curves {
            if self.boundaries.is_empty() {
                return Err("Select boundary objects, or type Distance and a length".into());
            }

            self.start_curves(state);
            return Ok(Next::More);
        }

        self.unmark(state);
        state.select_rows(std::mem::take(&mut self.before), false);
        Ok(Next::Done("Extend done".into()))
    }

    /// A click near a curve end grows it in one undo step; the tool stays for more.
    fn clicked(&mut self, state: &mut State, at: (f64, f64)) -> Option<Result<Next, String>> {
        self.follow(state, at);
        let Some(hover) = self.hover.take() else {
            return Some(Err(self.fetch_near(state, at)));
        };
        let (geometry, tail) = match hover.grown {
            Ok(grown) => grown,
            Err(error) => return Some(Err(error)),
        };

        if let Err(error) = state.scene.commit_geometry(hover.row, geometry, "extend") {
            return Some(Err(error));
        }

        state.commit_rows();

        for &row in &self.boundaries {
            state.gpu.set_selected(row, true);
        }

        // the grown curve is found where it is now
        let fresh = sampled(state, hover.row);
        self.candidates.retain(|(row, ..)| *row != hover.row);
        self.candidates
            .extend(fresh.map(|points| candidate(hover.row, points)));
        let added: f64 = tail
            .windows(2)
            .map(|pair| pair[0].distance(&pair[1], None))
            .sum();
        Some(Ok(Next::Repeat(format!(
            "Extended by {added:.3} · Undo reverts it"
        ))))
    }

    fn hovered(&mut self, state: &mut State, at: (f64, f64)) -> Option<bool> {
        if !self.curves {
            return Some(false);
        }

        Some(self.follow(state, at))
    }

    fn marks(&self, state: &State) -> Option<Overlay> {
        let hover = self.hover.as_ref()?;
        let screen = state.view_screen();
        let mut overlay = Overlay::default();

        match &hover.grown {
            Ok((_, tail)) => {
                let points: Vec<(f64, f64)> = tail.iter().filter_map(|p| screen.point(p)).collect();
                let added: f64 = tail
                    .windows(2)
                    .map(|pair| pair[0].distance(&pair[1], None))
                    .sum();
                overlay.marks.extend(points.last().copied());
                overlay.label = points.last().map(|p| (*p, format!("+{added:.3}")));
                overlay.strokes.push(Stroke {
                    points,
                    color: RED,
                    width: 2.0,
                    dashed: true,
                });
            }
            Err(error) => {
                let geometry = state.scene.geometry(hover.row)?;
                let place = state.scene.placement_of(hover.row)?;
                let (tip, _) = reach::tip(geometry, hover.end)?;
                let at = screen.point(&tip.transformed(&place))?;
                overlay.marks.push(at);
                overlay.label = Some((at, error.clone()));
            }
        }

        Some(overlay)
    }

    fn cancel(&mut self, state: &mut State) {
        self.unmark(state);
        state.select_rows(std::mem::take(&mut self.before), false);
    }

    fn status(&self) -> serde_json::Value {
        let hover = self.hover.as_ref().map(|hover| {
            let to = hover
                .grown
                .as_ref()
                .ok()
                .and_then(|(_, tail)| tail.last())
                .map(|p| [p[0], p[1], p[2]]);
            serde_json::json!({
                "row": hover.row,
                "end": if hover.end == End::Start { "start" } else { "end" },
                "to": to,
                "error": hover.grown.as_ref().err(),
            })
        });
        serde_json::json!({
            "command": "Extend",
            "phase": if self.curves { "curves" } else { "boundaries" },
            "boundaries": self.boundaries,
            "curves": self.candidates.len(),
            "loading": self.wanted.len(),
            "distance": self.distance,
            "asking": self.asking,
            "hover": hover,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::{accept, parse};
    use session_rust::Line;

    /// A flat box is near a click on its rectangle or within the aperture, not beside it.
    #[test]
    fn a_click_near_a_box_on_screen() {
        let mut matrix = [0.0; 16];

        for i in [0, 5, 10, 15] {
            matrix[i] = 1.0;
        }

        let screen = crate::app::snap::Screen::new(matrix, [0.0; 3], (200.0, 100.0));
        let wide = AABB::new(0.0, 0.0, 0.0, 0.5, 0.0, 0.0);
        assert!(boxed(&screen, &wide, (100.0, 50.0), 4.0));
        assert!(boxed(&screen, &wide, (150.0, 53.0), 4.0));
        assert!(!boxed(&screen, &wide, (100.0, 60.0), 4.0));
        assert!(!boxed(&screen, &wide, (190.0, 50.0), 4.0));
    }

    /// A world length becomes the same world length in a document placed a thousand times larger.
    #[test]
    fn a_world_distance_in_a_scaled_document() {
        let tool = Extending {
            distance: Some(25.0),
            ..Extending::default()
        };
        let line = Geometry::Line(Rc::new(Line::from_points(
            &Point::new(0., 0., 0.),
            &Point::new(1., 0., 0.),
        )));
        let place = Xform::scale_xyz(1000.0, 1000.0, 1000.0);
        let Some(Reach::Distance(local)) = tool.reach(0, &line, End::End, &place) else {
            panic!("a distance")
        };
        assert!((local - 0.025).abs() < 1e-12);
    }

    /// What a parsed line becomes.
    fn parsed(line: &str) -> Result<String, String> {
        parse(line).map(|action| format!("{action:?}"))
    }

    /// Bare Extend picks boundaries; a length grows ends; two numbers stretch the domain.
    #[test]
    fn extend_parses_every_form() {
        assert_eq!(
            parsed("Extend"),
            Ok("Extend { distance: None, asking: false }".into())
        );
        assert_eq!(
            parsed("Extend Distance 5"),
            Ok("Extend { distance: Some(5.0), asking: false }".into())
        );
        assert_eq!(
            parsed("extend 5"),
            Ok("Extend { distance: Some(5.0), asking: false }".into())
        );
        assert_eq!(
            parsed("Extend Distance"),
            Ok("Extend { distance: None, asking: true }".into())
        );
        assert!(parsed("Extend Distance -1").is_err());
        assert!(parsed("Extend Distance 0").is_err());
        assert_eq!(
            parsed("Extend -0.2 1.2"),
            Ok("Edit(Extend(-0.2, 1.2))".into())
        );
        assert_eq!(accept("Extend Dis"), ("Extend Distance ".into(), false));
    }
}
