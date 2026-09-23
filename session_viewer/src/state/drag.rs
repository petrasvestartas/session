use super::State;
use crate::app::cplane::CPlane;
use crate::app::selection::{SelectionMode, SelectionTool};
use crate::app::snap::{self, Bins, Screen, Snap, SnapKind};
use crate::engine::gpu::pick::PickMode;
use crate::engine::gpu::{Instance, Pick};
use crate::engine::performance::now_ms;
use session_rust::{Point, Vector, Xform};
use std::collections::HashSet;

/// Views closer to level than this, sin 20°, drag on a vertical plane.
const LEVEL: f64 = 0.34;

/// Snap reach, CSS pixels.
const APERTURE: f64 = 12.0;

/// Bin cell, device pixels.
const CELL: f64 = 24.0;

/// Rows binned per move, so a large scene fills its bins over a few moves instead of stalling one.
const BIN_BUDGET: u32 = 65_536;

/// Time a move may spend collecting new snap points, ms; the rest waits for the next move.
const COLLECT_MS: f64 = 4.0;

/// A mesh, BRep or surface with more control points offers no snaps to a drag.
const MAX_ROW_POINTS: usize = 65_536;

/// A BRep with more control links offers no Near snaps to a drag.
const MAX_ROW_WIRES: usize = 4_096;

/// A left drag that started on an object: a pick asks what the press landed on, then it follows.
pub struct ObjectDrag {
    down: (f64, f64),       // where the press landed, device pixels
    cursor: (f64, f64),     // where the pointer is now
    moving: Option<Moving>, // the grabbed objects, once the pick answered
    missed: bool,           // the press landed on nothing that moves
}

/// The objects a drag moves and where their grab point is.
struct Moving {
    row: u32,                  // the main selected row
    group: Vec<(u32, Xform)>,  // every moved row and where it started
    base: Point,               // the grab point
    plane: CPlane,             // the plane it slides on
    target: Point,             // where the grab point is now
    snapped: Option<SnapKind>, // what it snapped to
    targets: Option<Targets>,  // other objects' snap points, None with snaps off
}

/// Snap points of the objects near the cursor, collected as the drag reaches them and binned on screen.
pub(super) struct Targets {
    screen: Screen,                // the view the bins are made for
    rows: Bins,                    // object rows by screen cell
    moved: Vec<u32>,               // the dragged rows, which offer nothing
    seen: HashSet<u32>,            // rows collected, or found to offer nothing
    snaps: Vec<Snap>,              // points of the rows collected so far
    points: Bins,                  // those points by screen cell
    wires: Vec<(Vec<Point>, u32)>, // curves and edges of the rows collected so far
    strokes: Bins,                 // those wires by screen cell
    near: Vec<u32>,                // ids near the cursor, reused every move
    found: Vec<Snap>,              // Near and Perp points of this move, reused
}

impl Targets {
    /// Nothing collected yet for this view.
    pub(super) fn new(screen: Screen, moved: Vec<u32>) -> Self {
        Self {
            rows: Bins::new(screen.size, CELL),
            points: Bins::new(screen.size, CELL),
            strokes: Bins::new(screen.size, CELL),
            seen: moved.iter().copied().collect(),
            moved,
            screen,
            snaps: Vec::new(),
            wires: Vec::new(),
            near: Vec::new(),
            found: Vec::new(),
        }
    }
}

impl State {
    /// A plain press dragged past the click slop: ask the GPU what the press landed on.
    pub(crate) fn start_object_drag(&mut self, down: (f64, f64), at: (f64, f64)) -> bool {
        // F10 control points keep the left button
        if self.draft.is_some()
            || self.pending_split.is_some()
            || matches!(self.selection, SelectionMode::Controls { .. })
            || self.selection_tool != SelectionTool::Object
        {
            return false;
        }

        self.object_drag = Some(ObjectDrag {
            down,
            cursor: at,
            moving: None,
            missed: false,
        });
        self.probe_drag(down);
        true
    }

    /// Pick at the press; the answer comes to `take_drag_pick` a few frames later.
    fn probe_drag(&mut self, down: (f64, f64)) {
        self.cancel_cloud_query();
        self.requested = PickMode::Object;
        self.gpu.pick.configure(
            PickMode::Object,
            self.selection_radius_css,
            self.pixel_scale(),
        );
        self.gpu.pick.request(down.0 as u32, down.1 as u32);
        self.needs_frame = true;
    }

    /// The pointer moved: the grabbed objects follow; false while the pick is out.
    pub(crate) fn drag_object(&mut self, cursor: (f64, f64)) -> bool {
        let Some(drag) = self.object_drag.as_mut() else {
            return false;
        };
        drag.cursor = cursor;

        // a redraw now would cancel the pick; a pick something else cancelled is asked again
        if drag.moving.is_none() {
            let (down, lost) = (drag.down, !drag.missed && !self.gpu.pick.busy());

            if lost {
                self.probe_drag(down);
            }

            return false;
        }

        self.follow(cursor)
    }

    /// The pick answered: take hold of what it hit; false when no drag was asking.
    pub(super) fn take_drag_pick(&mut self, pick: Option<Pick>) -> bool {
        // out of `self` while selecting, which would cancel it
        let Some(mut drag) = self.object_drag.take() else {
            return false;
        };

        if drag.moving.is_some() || drag.missed {
            self.object_drag = Some(drag);
            return false;
        }

        drag.moving = pick.and_then(|pick| self.grab(pick.row, drag.down));
        drag.missed = drag.moving.is_none();
        let cursor = drag.cursor;
        self.object_drag = Some(drag);
        self.follow(cursor); // the pointer is already past the slop
        true
    }

    /// Select `row` unless it is part of the selection, and hold the selection at the press.
    fn grab(&mut self, row: u32, down: (f64, f64)) -> Option<Moving> {
        if !self.scene.selectable(row) {
            return None;
        }

        // streamed sheets and clouds are looked at, never edited
        if self.scene.display_only(row)
            || self.scene.sheet_at(row).is_some()
            || self.streamed_slot(row).is_some()
        {
            self.status(crate::app::scene::READ_ONLY);
            return None;
        }

        if self.scene.geometry(row).is_none() {
            self.status("This object cannot be moved");
            return None;
        }

        // a selected edge, face or entity stays put; another object is grabbed whole
        match self.selection.parent() {
            Some(parent) if parent == row => return None,
            Some(_) => self.select_rows(vec![row], false),
            None if !self.selected_rows().contains(&row) => self.select_rows(vec![row], false),
            None => {}
        }

        let rows = self.selected_rows();

        if rows.iter().any(|row| self.scene.display_only(*row)) {
            self.status(crate::app::scene::READ_ONLY);
            return None;
        }

        let group: Vec<(u32, Xform)> = rows
            .iter()
            .filter_map(|&row| Some((row, self.scene.placement_of(row)?)))
            .collect();
        let main = self.scene.selected?;
        let ray = self.camera.ray(down, self.viewport())?;
        let plane = drag_plane(&self.camera.orientation.rotate_vector(Vector::y_axis()));
        let screen = self.screen();
        // with snaps on, the press takes the objects' own point under it
        let own = if self.snap_enabled {
            self.own_snap(&group, &screen, down, &ray)
        } else {
            None
        };
        // else where the press ray meets the plane through the gumball centre
        let base = match own {
            Some(snap) => snap.point,
            None => {
                let origin = match self.gizmo.as_ref() {
                    Some(gizmo) => gizmo.origin.clone(),
                    None => self.gpu.objects.row_bounds(main)?.center(),
                };
                plane.hit(&origin, &ray.0, &ray.1)?
            }
        };
        let moved = group.iter().map(|(row, _)| *row).collect();
        let targets = self.snap_enabled.then(|| Targets::new(screen, moved));
        Some(Moving {
            row: main,
            group,
            target: base.clone(),
            base,
            plane,
            snapped: None,
            targets,
        })
    }

    /// Put the grab point under the cursor, snapped or on the plane, and show the objects there.
    fn follow(&mut self, cursor: (f64, f64)) -> bool {
        let Some(mut moving) = self
            .object_drag
            .as_mut()
            .and_then(|drag| drag.moving.take())
        else {
            return false;
        };
        let aimed = self.aim(&mut moving, cursor);

        if aimed {
            self.show(&moving, &moving.target);
        }

        if let Some(drag) = self.object_drag.as_mut() {
            drag.moving = Some(moving);
        }

        aimed
    }

    /// Where the grab point goes for this cursor; false when the ray misses the plane.
    fn aim(&self, moving: &mut Moving, cursor: (f64, f64)) -> bool {
        let Some(ray) = self.camera.ray(cursor, self.viewport()) else {
            return false;
        };
        let snap = match moving.targets.as_mut() {
            Some(targets) => self.snap_near(targets, Some(&moving.base), cursor, &ray),
            None => None,
        };
        let target = match &snap {
            Some(snap) => Some(snap.point.clone()),
            None => moving.plane.hit(&moving.base, &ray.0, &ray.1),
        };
        let Some(target) = target else {
            return false;
        };
        moving.snapped = snap.map(|snap| snap.kind);
        moving.target = target;
        true
    }

    /// Draw every moved row with its grab point at `at`; only GPU placements change.
    fn show(&mut self, moving: &Moving, at: &Point) {
        let delta = offset(&moving.base, at);

        for (row, place) in &moving.group {
            self.gpu
                .objects
                .set_placement(&self.gpu.ctx, *row, &(&delta * place));
            self.gpu.grew_bounds(*row);
        }

        self.place_gizmo(Some(moving.row));
        self.update_label();
        self.touch();
    }

    /// Release: the document moves the objects by the whole drag in one undo step.
    pub(crate) fn end_object_drag(&mut self) -> bool {
        let Some(drag) = self.object_drag.take() else {
            return false;
        };
        let Some(moving) = drag.moving else {
            // a flick released before the pick answered moves nothing
            if !drag.missed {
                self.gpu.pick.cancel();
            }

            return false;
        };
        self.show(&moving, &moving.base);

        // back where it started: nothing to record
        if (0..3).all(|i| moving.target[i] == moving.base[i]) {
            return true;
        }

        if let Err(error) = self.apply(offset(&moving.base, &moving.target), "move") {
            self.status(&error);
        }

        true
    }

    /// Drop a drag that will never be released; everything goes back.
    pub(super) fn cancel_object_drag(&mut self) {
        let Some(drag) = self.object_drag.take() else {
            return;
        };

        match drag.moving {
            Some(moving) => self.show(&moving, &moving.base),
            None if !drag.missed => self.gpu.pick.cancel(),
            None => {}
        }
    }

    /// The grabbed objects' own snap point under the press, if one is in reach.
    fn own_snap(
        &self,
        group: &[(u32, Xform)],
        screen: &Screen,
        down: (f64, f64),
        ray: &(Point, Vector),
    ) -> Option<Snap> {
        let reach = APERTURE * self.pixel_scale();
        let (mut snaps, mut wires, mut found) = (Vec::new(), Vec::new(), Vec::new());
        let start = now_ms();

        for (row, place) in group {
            // a large selection offers what it could collect in time
            if now_ms() - start > COLLECT_MS {
                break;
            }

            // only an object under the press offers the grab point
            let under = self
                .gpu
                .objects
                .row_bounds(*row)
                .and_then(|bounds| screen.circle(&bounds))
                .is_some_and(|(x, y, radius)| (x - down.0).hypot(y - down.1) <= radius + reach);

            if under
                && let Some(geometry) = self.scene.geometry(*row)
                && snap::control_count(geometry) <= MAX_ROW_POINTS
            {
                snap::of_geometry(geometry, place, *row, MAX_ROW_WIRES, &mut snaps, &mut wires);
            }
        }

        snap::along_wires(&wires, ray, None, self.snap_modes, &mut found);
        snap::best_in(snaps.iter().chain(&found), self.snap_modes, down, reach, |p| {
            screen.point(p)
        })
    }

    /// The best snap among the objects near the cursor; the moved ones never offer any.
    pub(super) fn snap_near(
        &self,
        targets: &mut Targets,
        base: Option<&Point>,
        cursor: (f64, f64),
        ray: &(Point, Vector),
    ) -> Option<Snap> {
        let screen = self.screen();

        // the view changed under the drag: everything is binned again
        if screen != targets.screen {
            *targets = Targets::new(screen, std::mem::take(&mut targets.moved));
        }

        let objects = &self.gpu.objects;
        let view = &targets.screen;
        targets.rows.fill(objects.len(), BIN_BUDGET, |row| {
            view.circle(&objects.row_bounds(row)?)
        });
        let reach = APERTURE * self.pixel_scale();
        let mut near = std::mem::take(&mut targets.near);
        targets.rows.near(cursor, reach, &mut near);
        // how far a row's box is from the cursor, in pixels
        let gap = |row: u32| {
            let (x, y, radius) = view.circle(&objects.row_bounds(row)?)?;
            Some(((x - cursor.0).hypot(y - cursor.1) - radius).max(0.0))
        };
        // rows not collected yet whose boxes reach the cursor, nearest first, while this move has time
        near.retain(|&row| !targets.seen.contains(&row) && gap(row).is_some_and(|d| d <= reach));
        near.sort_by_cached_key(|&row| gap(row).map_or(u64::MAX, |d| (d * 1024.0) as u64));
        let start = now_ms();

        for &row in &near {
            if now_ms() - start > COLLECT_MS {
                break;
            }

            self.collect(targets, row);
        }

        // Near and Perp on the wires in reach, then every point in reach
        targets.found.clear();
        targets.strokes.near(cursor, reach, &mut near);

        for &wire in &near {
            let wires = &targets.wires[wire as usize..=wire as usize];
            snap::along_wires(wires, ray, base, self.snap_modes, &mut targets.found);
        }

        targets.points.near(cursor, reach, &mut near);
        let points = near.iter().map(|&index| &targets.snaps[index as usize]);
        let best = snap::best_in(
            points.chain(&targets.found),
            self.snap_modes,
            cursor,
            reach,
            |p| targets.screen.point(p),
        );
        targets.near = near;
        best
    }

    /// Remember `row`'s snap points and wires where they land on screen; hidden, locked and very large objects offer none.
    fn collect(&self, targets: &mut Targets, row: u32) {
        targets.seen.insert(row);
        let shown = self
            .gpu
            .objects
            .row(row)
            .is_some_and(|object| object.flags & Instance::FLAG_HIDDEN == 0);

        if !shown || !self.scene.selectable(row) {
            return;
        }

        let Some(geometry) = self.scene.geometry(row) else {
            return;
        };
        let Some(place) = self.scene.placement_of(row) else {
            return;
        };

        if snap::control_count(geometry) > MAX_ROW_POINTS {
            return;
        }

        let (snaps, wires) = (targets.snaps.len(), targets.wires.len());
        snap::of_geometry(
            geometry,
            &place,
            row,
            MAX_ROW_WIRES,
            &mut targets.snaps,
            &mut targets.wires,
        );

        for (index, snap) in targets.snaps.iter().enumerate().skip(snaps) {
            if let Some(at) = targets.screen.point(&snap.point) {
                targets.points.insert(index as u32, at, 0.0);
            }
        }

        for (index, (points, _)) in targets.wires.iter().enumerate().skip(wires) {
            if let Some((x, y, radius)) = targets.screen.footprint(points) {
                targets.strokes.insert(index as u32, (x, y), radius);
            }
        }
    }

    /// The view as it maps the scene to device pixels now.
    pub(super) fn screen(&self) -> Screen {
        let origin = self.camera.origin();
        let matrix = self.camera.view_proj_anchored(self.aspect(), &origin).m;
        Screen::new(matrix, [origin[0], origin[1], origin[2]], self.viewport())
    }

    /// The snap marker while dragging: the grab point on screen and the kind it snapped to.
    pub fn drag_overlay(&self) -> Option<(Vec<(f64, f64)>, String)> {
        let moving = self.object_drag.as_ref()?.moving.as_ref()?;
        let kind = moving.snapped?;
        let at = &moving.target;
        Some((
            vec![self.project([at[0], at[1], at[2]])?],
            format!("{kind:?}"),
        ))
    }

    /// The drag as JSON, for the inspection tests.
    pub fn object_drag_status(&self) -> serde_json::Value {
        let Some(drag) = &self.object_drag else {
            return serde_json::Value::Null;
        };
        let Some(moving) = &drag.moving else {
            let phase = if drag.missed { "missed" } else { "probing" };
            return serde_json::json!({ "phase": phase });
        };
        let point = |p: &Point| [p[0], p[1], p[2]];
        serde_json::json!({
            "phase": "moving",
            "rows": moving.group.iter().map(|(row, _)| *row).collect::<Vec<_>>(),
            "base": point(&moving.base),
            "target": point(&moving.target),
            "snap": moving.snapped.map(|kind| format!("{kind:?}")),
            "plane": format!("{:?}", moving.plane),
        })
    }
}

/// The plane a drag slides on: the ground, unless the view is within about 20° of level.
fn drag_plane(forward: &Vector) -> CPlane {
    if forward[2].abs() >= LEVEL {
        CPlane::Xy
    } else {
        CPlane::facing(forward)
    }
}

/// The move that takes `from` to `to`.
fn offset(from: &Point, to: &Point) -> Xform {
    Xform::translation(to[0] - from[0], to[1] - from[1], to[2] - from[2])
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ground, except for views close to level.
    #[test]
    fn a_drag_slides_on_the_ground_unless_the_view_is_level() {
        let plane = |x, y, z| drag_plane(&Vector::new(x, y, z));
        assert_eq!(
            plane(0.433, 0.75, -0.5),
            CPlane::Xy,
            "the start-up iso view"
        );
        assert_eq!(plane(0.0, 0.0, -1.0), CPlane::Xy, "top");
        assert_eq!(plane(0.0, 1.0, 0.0), CPlane::Xz, "front");
        assert_eq!(plane(-1.0, 0.0, 0.0), CPlane::Yz, "side");
        assert_eq!(plane(0.1, 0.98, -0.17), CPlane::Xz, "nearly level");
    }

    /// The offset takes the first point onto the second.
    #[test]
    fn an_offset_takes_the_grab_point_to_the_target() {
        let from = Point::new(0.0, -10000.0, 0.0);
        let to = Point::new(1500.0, -9500.0, 0.0);
        let moved = offset(&from, &to).transform_point(&from);
        assert_eq!([moved[0], moved[1], moved[2]], [1500.0, -9500.0, 0.0]);
    }
}
