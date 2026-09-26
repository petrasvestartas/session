use super::geometry::{Edit, range};
use crate::State;
use crate::app::command::tool::cut::{
    Cutter, as_curve, as_plane, fence_plane, nearest_px, plane_moved, unit_ray,
};
use crate::app::command::tool::{Next, Overlay, Stroke, Tool};
use crate::app::command::{Action, Spec};
use crate::app::cplane::CPlane;
use crate::app::modeling::Interval;
use parts::{Blade, Parts};
use session_rust::{Geometry, Line, Plane, Point, Vector, Xform, intersection};

mod parts;

pub const SPEC: Spec = Spec {
    names: &["Trim"],
    aliases: &[],
    hint: "Trim · pick objects, Enter, pick cutters, Enter, click the parts to remove · Esc cancels · Trim 0.2 0.8 keeps that part of a curve",
    options: &[],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

const APERTURE_CSS: f64 = 12.0; // how near a click must be to a curve part
const BLUE: [u8; 3] = [30, 110, 170];
const RED: [u8; 3] = [210, 40, 40];

/// Trim interactively, or keep the part of a curve between two parameters.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(Trim));
    }

    let (a, b) = range(rest)?;
    Ok(Box::new(Edit(Interval::Trim(a, b))))
}

/// Start trimming; selected objects are the targets.
#[derive(Debug)]
struct Trim;

impl Action for Trim {
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.open_tool(Box::new(Trimming::default()))
    }
}

/// Which question the trim asks.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
enum Phase {
    #[default]
    Targets, // objects to trim
    Cutters, // objects that cut
    Parts,   // the parts to remove
}

/// One target cut into parts.
struct Cut {
    row: u32,                  // the target
    source: Geometry,          // its geometry when the cut was made
    parts: Parts,              // its parts, in its own frame
    place: Xform,              // its frame in the world
    removed: Vec<usize>,       // parts clicked away
    outlines: Vec<Vec<Point>>, // curve parts in the world
    marks: Vec<Point>,         // cut points in the world
    section: Vec<Vec<Point>>,  // cut edges in the world
    previewed: bool,           // the GPU shows a trimmed preview
}

/// The running trim.
#[derive(Default)]
struct Trimming {
    phase: Phase,
    before: Vec<u32>,              // the selection before Trim
    targets: Vec<u32>,             // rows to trim
    cutters: Vec<u32>,             // rows that cut
    doc: Option<usize>,            // the targets' document
    cuts: Vec<Cut>,                // the crossed targets
    hover: Option<(usize, usize)>, // (cut, part) under the cursor
}

impl std::fmt::Debug for Trimming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Trimming({:?})", self.phase)
    }
}

/// True for a geometry Trim can cut.
fn trimmable(geometry: &Geometry) -> bool {
    matches!(
        geometry,
        Geometry::Line(_)
            | Geometry::Polyline(_)
            | Geometry::NurbsCurve(_)
            | Geometry::NurbsSurface(_)
            | Geometry::BRep(_)
            | Geometry::Mesh(_)
    )
}

impl Trimming {
    /// Add `row` as a target, or take it away again.
    fn toggle_target(&mut self, state: &mut State, row: u32) -> Result<(), String> {
        if let Some(at) = self.targets.iter().position(|r| *r == row) {
            self.targets.remove(at);
            state.gpu.set_selected(row, false);
            return Ok(());
        }

        if let Some(reason) = state.locked_reason(&[row]) {
            return Err(reason);
        }

        if !state.scene.selectable(row) {
            return Err("This object is locked".into());
        }

        let geometry = state
            .scene
            .geometry(row)
            .ok_or("This object has no editable geometry")?;

        if !trimmable(geometry) {
            return Err("Trim cuts lines, polylines, curves, surfaces, BReps and meshes".into());
        }

        let (doc, _) = state.scene.identity_of(row).ok_or("This object is gone")?;

        if self.doc.is_some_and(|first| first != doc) && !self.targets.is_empty() {
            return Err("Trim edits one document at a time".into());
        }

        self.doc = Some(doc);
        self.targets.push(row);
        state.gpu.set_selected(row, true);
        Ok(())
    }

    /// Add `row` as a cutter, or take it away again.
    fn toggle_cutter(&mut self, state: &mut State, row: u32) -> Result<(), String> {
        if let Some(at) = self.cutters.iter().position(|r| *r == row) {
            self.cutters.remove(at);
            state.gpu.set_selected(row, false);
            return Ok(());
        }

        if self.targets.contains(&row) {
            return Err("A target cannot cut itself".into());
        }

        // a released document is fetched for the next click
        if state.scene.released_doc(row).is_some() {
            return Err(state.locked_reason(&[row]).unwrap_or_default());
        }

        let geometry = state
            .scene
            .geometry(row)
            .ok_or("This object has no geometry to cut with")?;

        if as_curve(geometry).is_none() && as_plane(geometry).is_none() {
            return Err("Cut with a line, polyline, curve, Plane or planar surface".into());
        }

        self.cutters.push(row);
        state.gpu.set_selected(row, true);
        Ok(())
    }

    /// Every cutter in the world, a straight line with its vertical plane too.
    fn blades(&self, state: &State) -> Vec<(Cutter, Option<Plane>)> {
        let forward = state.camera.orientation.rotate_vector(Vector::y_axis());
        let normal = CPlane::facing(&forward).normal();
        let mut blades = Vec::new();

        for &row in &self.cutters {
            let (Some(geometry), Some(place)) =
                (state.scene.geometry(row), state.scene.placement_of(row))
            else {
                continue;
            };

            // a flat object cuts as a plane; a line or curve as itself
            if let Some(plane) = as_plane(geometry) {
                if let Some(plane) = plane_moved(&plane, &place) {
                    blades.push((Cutter::Plane(plane), None));
                }

                continue;
            }

            let Some(curve) = as_curve(geometry).map(|curve| curve.transformed(&place)) else {
                continue;
            };
            let fence = matches!(geometry, Geometry::Line(_))
                .then(|| {
                    fence_plane(
                        &Line::from_points(&curve.point_at_start(), &curve.point_at_end()),
                        &normal,
                    )
                    .ok()
                })
                .flatten();
            blades.push((Cutter::Curve(curve), fence));
        }

        blades
    }

    /// Cut every target; the ones not crossed drop out.
    fn preview(&mut self, state: &mut State) -> Result<String, String> {
        let blades = self.blades(state);
        let mut missed = 0;
        let mut refusal = None;

        for &row in &self.targets {
            let (Some(source), Some(place)) = (
                state.scene.geometry(row).cloned(),
                state.scene.placement_of(row),
            ) else {
                continue;
            };
            let Some(back) = place.inverse() else {
                continue;
            };
            // the cutters in the target's frame
            let local: Vec<Blade> = blades
                .iter()
                .filter_map(|(cutter, fence)| {
                    Some(Blade {
                        cutter: cutter.moved(&back)?,
                        fence: fence.as_ref().and_then(|fence| plane_moved(fence, &back)),
                    })
                })
                .collect();

            match parts::cut(&source, &local) {
                Ok(parts) => {
                    let outlines = (0..parts.count())
                        .map(|index| world(&parts.outline(index), &place))
                        .collect();
                    let marks = world(&parts.cut_points(), &place);
                    let section = parts
                        .section()
                        .iter()
                        .map(|line| world(line, &place))
                        .collect();
                    self.cuts.push(Cut {
                        row,
                        source,
                        parts,
                        place,
                        removed: Vec::new(),
                        outlines,
                        marks,
                        section,
                        previewed: false,
                    });
                }
                Err(error) if error == "not crossed" => missed += 1,
                Err(error) => refusal = Some(error),
            }
        }

        if self.cuts.is_empty() {
            return Err(refusal.unwrap_or_else(|| "No crossings: pick other cutters or Esc".into()));
        }

        // curves are drawn as their parts over the scene
        for cut in &self.cuts {
            if matches!(cut.parts, Parts::Curve { .. }) {
                state.gpu.set_hidden(cut.row, true);
            }
        }

        for &row in &self.targets {
            state.gpu.set_selected(row, false);
        }

        self.phase = Phase::Parts;
        Ok(match missed {
            0 => String::new(),
            1 => "1 object is not crossed".into(),
            count => format!("{count} objects are not crossed"),
        })
    }

    /// The part under a click or the cursor: a curve part on screen, else the nearest surface hit.
    fn part_at(&self, state: &State, at: (f64, f64), surfaces: bool) -> Option<(usize, usize)> {
        let screen = state.view_screen();
        let mut lines = Vec::new(); // screen polylines of the curve parts left
        let mut owners = Vec::new();

        for (index, cut) in self.cuts.iter().enumerate() {
            for (part, outline) in cut.outlines.iter().enumerate() {
                if cut.removed.contains(&part) || outline.is_empty() {
                    continue;
                }

                lines.push(outline.iter().filter_map(|p| screen.point(p)).collect());
                owners.push((index, part));
            }
        }

        if let Some(hit) = nearest_px(&lines, at, APERTURE_CSS * state.pixel_scale()) {
            return Some(owners[hit]);
        }

        if !surfaces {
            return None;
        }

        // a ray from the eye, into each target's frame
        let (origin, direction) = state.camera.ray(at, state.viewport())?;
        let mut best: Option<(f64, (usize, usize))> = None;

        for (index, cut) in self.cuts.iter().enumerate() {
            let Some(back) = cut.place.inverse() else {
                continue;
            };
            let ray = unit_ray(&origin, &direction, &back);

            for (part, mesh) in cut.parts.hit_meshes() {
                if cut.removed.contains(&part) {
                    continue;
                }

                let Some(hits) = intersection::ray_mesh(&ray, mesh, 1e-12, false) else {
                    continue;
                };
                let Some(hit) = hits.first() else {
                    continue;
                };
                let distance = hit.transformed(&cut.place).distance(&origin, None);

                if best.is_none_or(|(near, _)| distance < near) {
                    best = Some((distance, (index, part)));
                }
            }
        }

        best.map(|(_, hit)| hit)
    }

    /// Draw `row` without its removed parts, or as its document has it.
    fn show(state: &mut State, cut: &mut Cut) {
        // the document's geometry now, which an undo during the trim may have changed
        if cut.removed.is_empty() {
            if let Some(geometry) = state.scene.geometry(cut.row).cloned() {
                state.scene.redraw(cut.row, &geometry, false);
                state.scene.upload_to(&mut state.gpu);
            }

            cut.previewed = false;
            return;
        }

        let Ok(mut kept) = parts::kept(&cut.source, &cut.parts, &cut.removed) else {
            return;
        };
        cut.previewed = state
            .scene
            .preview_geometry(cut.row, kept.remove(0), &mut state.gpu)
            .is_ok();
    }

    /// Clear every highlight and GPU-only change.
    fn restore(&mut self, state: &mut State) {
        for &row in self.targets.iter().chain(&self.cutters) {
            state.gpu.set_selected(row, false);
        }

        for cut in &mut self.cuts {
            let hidden = state
                .scene
                .identity_of(cut.row)
                .is_some_and(|id| state.scene.hidden.contains(&id));

            if matches!(cut.parts, Parts::Curve { .. }) && !hidden {
                state.gpu.set_hidden(cut.row, false);
            }

            // the document's own geometry again
            if cut.previewed {
                cut.removed.clear();
                Self::show(state, cut);
            }
        }

        self.cuts.clear();
        self.targets.clear();
        self.cutters.clear();
        self.hover = None;
    }

    /// Write every trimmed target in one undo step.
    fn commit(&mut self, state: &mut State) -> Result<Next, String> {
        let edits: Result<Vec<(u32, Vec<Geometry>)>, String> = self
            .cuts
            .iter()
            .filter(|cut| !cut.removed.is_empty())
            .map(|cut| Ok((cut.row, parts::kept(&cut.source, &cut.parts, &cut.removed)?)))
            .collect();
        self.restore_flags(state);
        let result = edits.and_then(|edits| state.scene.commit_trim(&edits));

        // a refused trim leaves no preview behind
        if result.is_err() {
            for cut in &mut self.cuts {
                cut.removed.clear();

                if cut.previewed {
                    Self::show(state, cut);
                }
            }
        }

        self.cuts.clear();

        match result {
            Ok(count) => {
                state.after_history();
                let noun = if count == 1 { "object" } else { "objects" };
                Ok(Next::Done(format!(
                    "Trimmed {count} {noun} · Undo restores them"
                )))
            }
            Err(error) => {
                state.after_history();
                Ok(Next::Done(format!(
                    "Trim failed: {error} · nothing changed"
                )))
            }
        }
    }

    /// Before a commit: highlights off, curves shown again.
    fn restore_flags(&mut self, state: &mut State) {
        for &row in self.targets.iter().chain(&self.cutters) {
            state.gpu.set_selected(row, false);
        }

        for cut in &self.cuts {
            if matches!(cut.parts, Parts::Curve { .. }) {
                state.gpu.set_hidden(cut.row, false);
            }
        }
    }

    /// True while some target can still lose a part.
    fn removable(&self) -> bool {
        self.cuts
            .iter()
            .any(|cut| cut.parts.count() - cut.removed.len() >= 2)
    }
}

/// Local points in the world.
fn world(points: &[Point], place: &Xform) -> Vec<Point> {
    points.iter().map(|p| p.transformed(place)).collect()
}

impl Tool for Trimming {
    fn name(&self) -> &'static str {
        "Trim"
    }

    fn prompt(&self, _points: &[Point]) -> String {
        match self.phase {
            Phase::Targets => {
                "select objects to trim (curves, surfaces, BReps, meshes) · Enter continues".into()
            }
            Phase::Cutters => format!(
                "select cutting objects for {} (line, polyline, curve, Plane, planar surface) · Enter previews",
                match self.targets.len() {
                    1 => "1 object".to_string(),
                    count => format!("{count} objects"),
                }
            ),
            Phase::Parts => "click the part to remove · Enter applies".into(),
        }
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        match self.phase {
            Phase::Parts => &[("Apply", ""), ("Cancel", "Escape")],
            _ => &[("Next", ""), ("Cancel", "Escape")],
        }
    }

    fn asks_points(&self) -> bool {
        false
    }

    /// Selected objects become the targets.
    fn begin(&mut self, state: &mut State) -> Result<Next, String> {
        self.before = state.selected_rows();
        state.select(None);
        state.place_gizmo(None);
        let mut skipped = None;

        for row in self.before.clone() {
            if let Err(reason) = self.toggle_target(state, row) {
                skipped = Some(reason);
            }
        }

        if !self.targets.is_empty() {
            for &row in &self.targets {
                state.gpu.set_selected(row, false);
            }

            self.phase = Phase::Cutters;
        }

        match skipped {
            Some(reason) => Err(format!(
                "Skipped a selected object: {reason} · Trim: {}",
                self.prompt(&[])
            )),
            None => Ok(Next::More),
        }
    }

    fn picks(&self) -> bool {
        self.phase != Phase::Parts
    }

    fn picked(&mut self, state: &mut State, row: Option<u32>) -> Result<Next, String> {
        let Some(row) = row else {
            return Err(format!("Nothing there · Trim: {}", self.prompt(&[])));
        };

        match self.phase {
            Phase::Targets => self.toggle_target(state, row)?,
            Phase::Cutters => self.toggle_cutter(state, row)?,
            Phase::Parts => {}
        }

        Ok(Next::More)
    }

    /// Placed points mean nothing to Trim.
    fn placed(
        &mut self,
        _state: &mut State,
        _points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        Err("Trim picks objects: click them".into())
    }

    fn enter(&mut self, state: &mut State, _points: &[Point]) -> Result<Next, String> {
        match self.phase {
            Phase::Targets if self.targets.is_empty() => Err("Select objects to trim first".into()),
            Phase::Targets => {
                for &row in &self.targets {
                    state.gpu.set_selected(row, false);
                }

                self.phase = Phase::Cutters;
                Ok(Next::More)
            }
            Phase::Cutters if self.cutters.is_empty() => Err("Select cutting objects first".into()),
            Phase::Cutters => match self.preview(state)? {
                note if note.is_empty() => Ok(Next::More),
                note => Ok(Next::Repeat(note)),
            },
            Phase::Parts if self.cuts.iter().all(|cut| cut.removed.is_empty()) => {
                self.cancel(state);
                Ok(Next::Done("Trim cancelled · nothing changed".into()))
            }
            Phase::Parts => self.commit(state),
        }
    }

    fn clicked(&mut self, state: &mut State, at: (f64, f64)) -> Option<Result<Next, String>> {
        let Some((index, part)) = self.part_at(state, at, true) else {
            return Some(Err("Click a part of a trimmed object".into()));
        };
        let cut = &mut self.cuts[index];

        if cut.parts.count() - cut.removed.len() < 2 {
            return Some(Err("The last part stays; use Delete".into()));
        }

        cut.removed.push(part);
        self.hover = None;

        if !self.removable() {
            return Some(self.commit(state));
        }

        // a surface or mesh shows what is left
        let cut = &mut self.cuts[index];

        if !matches!(cut.parts, Parts::Curve { .. }) {
            Self::show(state, cut);
        }

        Some(Ok(Next::More))
    }

    fn hovered(&mut self, state: &mut State, at: (f64, f64)) -> Option<bool> {
        if self.phase != Phase::Parts {
            return Some(false);
        }

        let hover = self.part_at(state, at, false);
        let changed = hover != self.hover;
        self.hover = hover;
        Some(changed)
    }

    fn marks(&self, state: &State) -> Option<Overlay> {
        if self.phase != Phase::Parts {
            return None;
        }

        let screen = state.view_screen();
        let on_screen = |points: &[Point]| {
            points
                .iter()
                .filter_map(|p| screen.point(p))
                .collect::<Vec<_>>()
        };
        let mut overlay = Overlay::default();

        for (index, cut) in self.cuts.iter().enumerate() {
            for (part, outline) in cut.outlines.iter().enumerate() {
                if cut.removed.contains(&part) {
                    continue;
                }

                let hovered = self.hover == Some((index, part));
                let points = on_screen(outline);

                if hovered {
                    overlay.label = points
                        .get(points.len() / 2)
                        .map(|p| (*p, "remove".to_string()));
                }

                overlay.strokes.push(Stroke {
                    points,
                    color: if hovered { RED } else { BLUE },
                    width: 2.0,
                    dashed: hovered,
                });
            }

            for line in &cut.section {
                overlay.strokes.push(Stroke {
                    points: on_screen(line),
                    color: RED,
                    width: 2.0,
                    dashed: false,
                });
            }

            overlay.marks.extend(on_screen(&cut.marks));
        }

        Some(overlay)
    }

    fn cancel(&mut self, state: &mut State) {
        self.restore(state);
        state.select_rows(std::mem::take(&mut self.before), false);
        crate::app::feedback::status("Trim cancelled · nothing changed");
    }

    fn status(&self) -> serde_json::Value {
        let phase = match self.phase {
            Phase::Targets => "targets",
            Phase::Cutters => "cutters",
            Phase::Parts => "parts",
        };
        let parts: Vec<_> = self
            .cuts
            .iter()
            .map(|cut| serde_json::json!({"row": cut.row, "count": cut.parts.count(), "removed": cut.removed}))
            .collect();
        let hidden: Vec<u32> = self
            .cuts
            .iter()
            .filter(|cut| matches!(cut.parts, Parts::Curve { .. }))
            .map(|cut| cut.row)
            .collect();
        serde_json::json!({
            "command": "Trim",
            "phase": phase,
            "targets": self.targets,
            "cutters": self.cutters,
            "parts": parts,
            "hover": self.hover.map(|(cut, part)| [cut, part]),
            "hidden": hidden,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::app::command::{accept, completions, parse};

    /// What a parsed line becomes.
    fn parsed(line: &str) -> Result<String, String> {
        parse(line).map(|action| format!("{action:?}"))
    }

    /// Bare Trim runs the tool; two numbers keep the old parametric trim.
    #[test]
    fn trim_parses_both_forms() {
        assert_eq!(parsed("Trim"), Ok("Trim".into()));
        assert_eq!(parsed("trim 0.2 0.8"), Ok("Edit(Trim(0.2, 0.8))".into()));
        assert!(parsed("trim 0 1 extra").is_err());
        assert_eq!(completions("Tr"), vec!["Trim"]);
        assert_eq!(accept("Tri"), ("Trim".into(), true));
    }
}
