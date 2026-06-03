//! Interactive drawing tools on `State`: click/typed point input, osnap, rubber-band
//! preview, and commit. Clicks are routed here (before selection/gumball) by `update()`
//! while a tool is active; typed lines are routed here by the CLI submit path.

use crate::camera::ProjMode;
use crate::coord_parser::Coord;
use crate::gpu_session::LineVertex;
use crate::pick::screen_to_world_ray;
use crate::snap::{self, SnapKind, SnapModes, SnapResult};
use crate::tool_state::DrawTool;
use crate::undo_state::{replace_or_push_nurbscurve, UndoAction};
use crate::{cad_plane, labels_from_session, State};
use session_rust::{Color, Line, NurbsCurve, Point, Polyline, TreeNode};

/// Snap aperture / candidate radius in screen pixels.
const SNAP_APERTURE_PX: f32 = 12.0;
/// Reserved guids for the world-space preview line + its identity-model instance.
const PREVIEW_LINE_GUID: &str = "__draw_preview__seg";
const PREVIEW_INSTANCE_GUID: &str = "__draw_preview__inst";

/// Project a world point (mm) through `view_proj` to physical screen pixels.
/// Returns None for points behind the camera (perspective).
pub(crate) fn project_point(view_proj: &[[f32; 4]; 4], vp: (f32, f32), p: &Point) -> Option<(f32, f32)> {
    let (px, py, pz) = (p[0] as f32, p[1] as f32, p[2] as f32);
    let cx = view_proj[0][0] * px + view_proj[1][0] * py + view_proj[2][0] * pz + view_proj[3][0];
    let cy = view_proj[0][1] * px + view_proj[1][1] * py + view_proj[2][1] * pz + view_proj[3][1];
    let cw = view_proj[0][3] * px + view_proj[1][3] * py + view_proj[2][3] * pz + view_proj[3][3];
    if cw <= 1e-6 {
        return None;
    }
    let ndc_x = cx / cw;
    let ndc_y = cy / cw;
    Some(((ndc_x + 1.0) * 0.5 * vp.0, (1.0 - ndc_y) * 0.5 * vp.1))
}

impl State {
    /// Start an interactive draw tool. Clears any selection gizmo; the caller logs the prompt.
    pub(crate) fn start_draw_tool(&mut self, tool: DrawTool) {
        self.gb.gumball = None;
        self.gb.gumball_input = None;
        self.tool.start(tool);
        self.clear_draw_preview();
    }

    /// Cursor moved while a tool is active: recompute the snap and flag a preview rebuild.
    pub(crate) fn tool_on_move(&mut self, cx: f32, cy: f32) {
        let r = self.compute_snap(cx, cy);
        self.tool.cursor_snap = Some(r);
        self.tool.preview_dirty = true;
    }

    /// Column-major 4×4 translation matrix.
    fn translate_cm(v: [f32; 3]) -> [[f32; 4]; 4] {
        [[1.0,0.0,0.0,0.0], [0.0,1.0,0.0,0.0], [0.0,0.0,1.0,0.0], [v[0],v[1],v[2],1.0]]
    }

    /// Bake the move base→target into the CPU geometry (objects) or the edit selection + undo.
    fn commit_move(&mut self, base: &Point, target: &Point) {
        let t = Self::translate_cm([(target[0]-base[0]) as f32, (target[1]-base[1]) as f32, (target[2]-base[2]) as f32]);
        if self.tool.move_edit {
            self.apply_edit_delta(&t);
            self.commit_edit_transform();
        } else {
            let origins = self.tool.move_origins.clone();
            for (guid, orig) in &origins {
                let model = crate::mat4_mul_cm(&t, orig);
                self.commit_object_transform(guid, model);
            }
        }
        self.log_prompt(&format!(
            "moved  (Δ {:.1}, {:.1}, {:.1})",
            target[0]-base[0], target[1]-base[1], target[2]-base[2]
        ));
    }

    /// Viewport click while a tool is active: resolve to a point and feed it in.
    pub(crate) fn tool_on_click(&mut self, cx: f32, cy: f32) {
        let p = self.resolve_point(cx, cy);
        self.push_point(p);
    }

    /// Typed command-line input while a tool is active. Returns a status string for the log.
    pub(crate) fn tool_on_text(&mut self, input: &str) -> String {
        let t = input.trim();
        let multi = matches!(self.tool.tool, DrawTool::Polyline | DrawTool::NurbsCurve { .. });
        if multi {
            match t.to_lowercase().as_str() {
                "c" | "close" if self.tool.tool == DrawTool::Polyline => {
                    self.commit_polyline(true);
                    self.tool.reset();
                    self.clear_draw_preview();
                    return "closed".to_string();
                }
                "u" | "undo" => {
                    self.tool.points.pop();
                    self.tool.preview_dirty = true;
                    return "removed last point".to_string();
                }
                _ => {}
            }
        }
        let coord = match crate::coord_parser::parse(t) {
            Some(c) => c,
            None => return format!("? could not parse '{t}'  (x,y,z | @dx,dy | @dist<ang)"),
        };
        let plane = cad_plane::resolve(&self.scene.camera);
        let last = self.tool.points.last().cloned();
        let p = match coord {
            Coord::Absolute(x, y, z) => Point::new(x as f64, y as f64, z as f64),
            Coord::Relative(dx, dy, dz) => {
                let l = last.unwrap_or_else(|| Point::new(0.0, 0.0, 0.0));
                Point::new(l[0] + dx as f64, l[1] + dy as f64, l[2] + dz as f64)
            }
            Coord::Polar(dist, ang) => {
                let l = last.unwrap_or_else(|| plane.origin());
                let r = ang.to_radians();
                let xa = plane.x_axis();
                let ya = plane.y_axis();
                Point::new(
                    l[0] + (xa[0] * r.cos() as f64 + ya[0] * r.sin() as f64) * dist as f64,
                    l[1] + (xa[1] * r.cos() as f64 + ya[1] * r.sin() as f64) * dist as f64,
                    l[2] + (xa[2] * r.cos() as f64 + ya[2] * r.sin() as f64) * dist as f64,
                )
            }
            Coord::Distance(dist) => {
                let l = last.unwrap_or_else(|| plane.origin());
                let dir = self.last_direction(&plane);
                Point::new(l[0] + (dir[0] * dist) as f64, l[1] + (dir[1] * dist) as f64, l[2] + (dir[2] * dist) as f64)
            }
        };
        self.push_point(p);
        String::new()
    }

    /// Direction of the last polyline segment, or the plane x-axis if none.
    fn last_direction(&self, plane: &session_rust::Plane) -> [f32; 3] {
        // Move: typed distance goes along base→cursor (the direction the user is snapping toward).
        if self.tool.tool == DrawTool::Move && self.tool.points.len() == 1 {
            if let Some(s) = &self.tool.cursor_snap {
                let a = &self.tool.points[0];
                let d = [s.point[0]-a[0], s.point[1]-a[1], s.point[2]-a[2]];
                let len = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
                if len > 1e-6 {
                    return [(d[0]/len) as f32, (d[1]/len) as f32, (d[2]/len) as f32];
                }
            }
        }
        let n = self.tool.points.len();
        if n >= 2 {
            let a = &self.tool.points[n - 2];
            let b = &self.tool.points[n - 1];
            let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
            let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
            if len > 1e-6 {
                return [(d[0] / len) as f32, (d[1] / len) as f32, (d[2] / len) as f32];
            }
        }
        let xa = plane.x_axis();
        [xa[0] as f32, xa[1] as f32, xa[2] as f32]
    }

    /// Finish a multi-point tool (Enter on empty input). No-op for single-shot tools.
    pub(crate) fn tool_finish_open(&mut self) {
        match self.tool.tool {
            DrawTool::Polyline => {
                if self.tool.points.len() >= 2 {
                    self.commit_polyline(false);
                }
            }
            DrawTool::NurbsCurve { degree } => {
                if self.tool.points.len() >= 2 {
                    self.commit_nurbscurve(degree);
                }
            }
            _ => {}
        }
        self.tool.reset();
        self.clear_draw_preview();
    }

    /// Cancel the active tool (Esc) and drop the in-progress preview. Idempotent: a
    /// no-op when no tool is active (so the egui and key-event paths can both call it).
    pub(crate) fn tool_cancel(&mut self) {
        if !self.tool.is_active() {
            return;
        }
        // Move: the live preview moved things — restore to start.
        if self.tool.tool == DrawTool::Move {
            if self.tool.move_edit {
                self.abort_edit_move();
            } else {
                let origins = self.tool.move_origins.clone();
                for (guid, orig) in &origins {
                    self.scene.gpu_session.update_transform(guid, *orig, &self.gpu.queue);
                }
            }
        }
        self.clear_draw_preview();
        self.tool.reset();
        self.log_prompt("cancelled");
    }

    fn resolve_point(&mut self, cx: f32, cy: f32) -> Point {
        let r = self.compute_snap(cx, cy);
        let p = r.point.clone();
        self.tool.cursor_snap = Some(r);
        p
    }

    fn push_point(&mut self, p: Point) {
        match self.tool.tool {
            DrawTool::Point => {
                self.tool.points = vec![p];
                self.commit_point();
                self.tool.points.clear();
            }
            DrawTool::Line => {
                self.tool.points.push(p);
                if self.tool.points.len() >= 2 {
                    self.commit_line();
                    self.tool.reset();
                    self.clear_draw_preview();
                } else {
                    self.log_prompt("Line: end point");
                }
            }
            DrawTool::Polyline => {
                self.tool.points.push(p);
                let n = self.tool.points.len();
                self.log_prompt(&format!(
                    "Polyline: next point ({n}) — Enter=finish, c=close, u=undo, Esc=cancel"
                ));
            }
            DrawTool::NurbsCurve { degree } => {
                self.tool.points.push(p);
                let n = self.tool.points.len();
                self.log_prompt(&format!(
                    "Curve (deg {degree}): control point ({n}) — Enter=finish, u=undo, Esc=cancel"
                ));
            }
            DrawTool::Move => {
                if self.tool.points.is_empty() {
                    self.tool.points.push(p);
                    self.log_prompt("Move: target — click/snap, type distance, or x,y,z / @dx,dy,dz (Esc)");
                } else {
                    let base = self.tool.points[0].clone();
                    self.commit_move(&base, &p);
                    self.tool.reset();
                    self.clear_draw_preview();
                    return;
                }
            }
            DrawTool::Idle => {}
        }
        self.tool.preview_dirty = true;
    }

    fn compute_snap(&mut self, cx: f32, cy: f32) -> SnapResult {
        let view = self.scene.camera.view_matrix();
        let proj = self.scene.camera.proj_matrix();
        let vp = (self.gpu.config.width as f32, self.gpu.config.height as f32);
        let is_ortho = self.scene.camera.proj_mode == ProjMode::Ortho;
        let ray = screen_to_world_ray(&view, &proj, vp, (cx, cy), is_ortho);
        let plane = cad_plane::resolve(&self.scene.camera);
        let near_world = cad_plane::ray_to_plane(&ray, &plane);

        // Build the scene's static snap points once per tool session (End/Mid/Vertex). Per frame
        // we only PROJECT these and pick the nearest within the aperture — no `point_at`.
        if !self.tool.snap_cache_ready {
            let s = snap::build_static_snaps(&self.scene.session, &self.scene.hidden_guids);
            self.tool.snap_cache = s.points;
            self.tool.snap_edges = s.edges;
            self.tool.snap_cache_ready = true;
        }

        // The object being moved stays put during the move (we only show a rubber-band line and
        // bake on commit), so its ORIGINAL edges are valid snap targets — don't exclude it.
        let view_proj = self.scene.camera.view_proj();
        let modes = self.tool.snap_modes;
        let mode_on = |k: SnapKind| match k {
            SnapKind::Endpoint => modes.has(SnapModes::ENDPOINT),
            SnapKind::Midpoint => modes.has(SnapModes::MIDPOINT),
            SnapKind::Vertex   => modes.has(SnapModes::VERTEX),
            _ => true,
        };
        let mut best: Option<(u8, f32, Point, SnapKind)> = None; // (priority, px-dist, point, kind)
        for (p, kind, _guid) in &self.tool.snap_cache {
            if !mode_on(*kind) { continue; }
            if let Some((sx, sy)) = project_point(&view_proj, vp, p) {
                let dpx = ((sx - cx).powi(2) + (sy - cy).powi(2)).sqrt();
                if dpx > SNAP_APERTURE_PX { continue; }
                let prio = kind.priority();
                let better = match &best { None => true, Some((bp, bd, ..)) => prio < *bp || (prio == *bp && dpx < *bd) };
                if better { best = Some((prio, dpx, p.clone(), *kind)); }
            }
        }
        // Near: closest point ALONG any edge polyline (priority 5 — loses to End/Mid/Vertex when
        // those are within the aperture, fills in when hovering mid-edge).
        if modes.has(SnapModes::NEAR) {
            let prio = SnapKind::Near.priority();
            for (_guid, poly) in &self.tool.snap_edges {
                for w in poly.windows(2) {
                    let (a, b) = (project_point(&view_proj, vp, &w[0]), project_point(&view_proj, vp, &w[1]));
                    let (a, b) = match (a, b) { (Some(a), Some(b)) => (a, b), _ => continue };
                    let (ex, ey) = (b.0 - a.0, b.1 - a.1);
                    let len2 = ex*ex + ey*ey;
                    let t = if len2 > 1e-6 { (((cx - a.0)*ex + (cy - a.1)*ey) / len2).clamp(0.0, 1.0) } else { 0.0 };
                    let (sx, sy) = (a.0 + t*ex, a.1 + t*ey);
                    let dpx = ((sx - cx).powi(2) + (sy - cy).powi(2)).sqrt();
                    if dpx > SNAP_APERTURE_PX { continue; }
                    let better = match &best { None => true, Some((bp, bd, ..)) => prio < *bp || (prio == *bp && dpx < *bd) };
                    if better {
                        let td = t as f64;
                        let p3 = Point::new(
                            w[0][0] + (w[1][0]-w[0][0])*td,
                            w[0][1] + (w[1][1]-w[0][1])*td,
                            w[0][2] + (w[1][2]-w[0][2])*td,
                        );
                        best = Some((prio, dpx, p3, SnapKind::Near));
                    }
                }
            }
        }
        match best {
            Some((_, _, point, kind)) => SnapResult { point, kind },
            None => SnapResult { point: near_world, kind: SnapKind::Plane },
        }
    }

    fn commit_point(&mut self) {
        let p = self.tool.points[0].clone();
        let name = self.next_name("pt");
        let mut pt = Point::new(p[0], p[1], p[2]);
        pt.name = name.clone();
        pt.pointcolor = Color::new(1.0, 0.8, 0.2, 1.0);
        let guid = pt.guid().to_string();
        self.scene.session.add_point(pt, None);
        self.register_added(&guid);
        self.log_prompt(&format!("+ {name}  ({:.2}, {:.2}, {:.2})", p[0], p[1], p[2]));
    }

    fn commit_line(&mut self) {
        let a = self.tool.points[0].clone();
        let b = self.tool.points[1].clone();
        let name = self.next_name("line");
        let mut l = Line::from_points(&a, &b);
        l.name = name.clone();
        let guid = l.guid().to_string();
        self.scene.session.add_line(l, None);
        self.register_added(&guid);
        self.log_prompt(&format!("+ {name}"));
    }

    fn commit_polyline(&mut self, closed: bool) {
        if self.tool.points.len() < 2 {
            return;
        }
        let mut pts: Vec<Point> = self.tool.points.iter().map(|p| Point::new(p[0], p[1], p[2])).collect();
        let n = pts.len();
        if closed {
            pts.push(pts[0].clone());
        }
        let name = self.next_name("poly");
        let mut pl = Polyline::new(pts);
        pl.name = name.clone();
        pl.linecolor = Color::new(0.4, 0.9, 1.0, 1.0);
        let guid = pl.guid().to_string();
        self.scene.session.add_polyline(pl, None);
        self.register_added(&guid);
        self.log_prompt(&format!("+ {name}  ({n} pts{})", if closed { ", closed" } else { "" }));
    }

    /// Commit a control-point NURBS curve. Lives in `objects.nurbscurves` (not lookup),
    /// registered like the cone/torus surfaces: GPU upload -> tree node -> undo -> list.
    fn commit_nurbscurve(&mut self, degree: usize) {
        if self.tool.points.len() < 2 {
            return;
        }
        let pts: Vec<Point> = self.tool.points.iter().map(|p| Point::new(p[0], p[1], p[2])).collect();
        let deg = degree.min(pts.len() - 1).max(1);
        let mut nc = NurbsCurve::create(false, deg, &pts);
        if !nc.is_valid() {
            self.log_prompt("curve invalid — need more control points");
            return;
        }
        let name = self.next_name("curve");
        nc.set_guid(name.clone());
        nc.name = name.clone();
        nc.linecolors = vec![Color::new(0.9, 0.7, 1.0, 1.0)];
        let guid = nc.guid().to_string();
        self.scene.gpu_session.add_nurbscurve(&nc, &self.gpu.device, &self.gpu.queue);
        let node = TreeNode::new(nc.guid());
        self.scene.session.tree.add(&node, None);
        self.hist.push(UndoAction::AddNurbsCurve { nc: nc.clone() });
        replace_or_push_nurbscurve(&mut self.scene.session.objects.nurbscurves, nc);
        self.scene.geom_guid_set.insert(guid);
        self.scene.leaf_cache_dirty = true;
        self.log_prompt(&format!("+ {name}  (NURBS curve, deg {deg}, {} cv)", pts.len()));
    }

    /// Shared tail: upload to GPU, push one undo entry, refresh caches/labels.
    fn register_added(&mut self, guid: &str) {
        if let Some(geom) = self.scene.session.lookup.get(guid) {
            self.scene.gpu_session.add_geometry(guid, geom, &self.gpu.device, &self.gpu.queue);
            self.hist.push(UndoAction::AddLookup { guid: guid.to_string(), geom: geom.clone() });
        }
        self.scene.geom_guid_set.insert(guid.to_string());
        self.scene.leaf_cache_dirty = true;
        self.scene.text_labels = labels_from_session(&self.scene.session);
        self.tool.snap_cache_ready = false; // new geometry → rebuild snap cache
    }

    fn next_name(&mut self, prefix: &str) -> String {
        let n = self.shell.cmd_counter;
        self.shell.cmd_counter += 1;
        format!("{prefix}_{n}")
    }

    fn log_prompt(&mut self, msg: &str) {
        self.shell.cmd_log.push(msg.to_string());
        if self.shell.cmd_log.len() > 200 {
            let drain = self.shell.cmd_log.len() - 200;
            self.shell.cmd_log.drain(0..drain);
        }
    }

    /// Rebuild the world-space rubber-band: committed points chained, plus a segment
    /// to the current cursor snap. Injected into the `line` arena under a reserved guid.
    pub(crate) fn rebuild_draw_preview(&mut self) {
        self.clear_draw_preview();
        if !self.tool.is_active() {
            return;
        }
        let mut chain: Vec<[f32; 3]> = self.tool.points.iter().map(|p| [p[0] as f32, p[1] as f32, p[2] as f32]).collect();
        if let Some(s) = &self.tool.cursor_snap {
            chain.push([s.point[0] as f32, s.point[1] as f32, s.point[2] as f32]);
        }
        if chain.len() < 2 {
            return;
        }
        let color: [u8; 4] = [255, 160, 40, 255];
        let mut verts: Vec<LineVertex> = Vec::with_capacity((chain.len() - 1) * 2);
        for i in 0..chain.len() - 1 {
            verts.push(LineVertex { position: chain[i], color });
            verts.push(LineVertex { position: chain[i + 1], color });
        }
        // Ghost: object-move preview — draw the moving objects' edges translated to the cursor,
        // so you see where they will land while the originals stay put for snapping.
        if self.tool.tool == DrawTool::Move && !self.tool.move_edit && self.tool.points.len() == 1 {
            if let Some(s) = &self.tool.cursor_snap {
                let base = self.tool.points[0].clone();
                let v = [(s.point[0]-base[0]) as f32, (s.point[1]-base[1]) as f32, (s.point[2]-base[2]) as f32];
                let gc: [u8; 4] = [120, 200, 255, 220];
                let moving: std::collections::HashSet<&String> =
                    self.tool.move_origins.iter().map(|(g, _)| g).collect();
                for (guid, poly) in &self.tool.snap_edges {
                    if !moving.contains(guid) { continue; }
                    for w in poly.windows(2) {
                        verts.push(LineVertex { position: [w[0][0] as f32+v[0], w[0][1] as f32+v[1], w[0][2] as f32+v[2]], color: gc });
                        verts.push(LineVertex { position: [w[1][0] as f32+v[0], w[1][1] as f32+v[1], w[1][2] as f32+v[2]], color: gc });
                    }
                }
            }
        }
        // Look the preview instance up from the pick table each time (robust to a
        // `clear`/rebuild that resets the table); allocate + upload an identity model
        // the first time it is missing.
        let inst = match self.scene.gpu_session.pick.instance_id(PREVIEW_INSTANCE_GUID) {
            Some(id) => id,
            None => {
                let id = self.scene.gpu_session.pick.allocate(PREVIEW_INSTANCE_GUID);
                self.scene
                    .gpu_session
                    .write_instance(id, [1.0, 1.0, 1.0, 1.0], &self.gpu.device, &self.gpu.queue);
                id
            }
        };
        self.scene
            .gpu_session
            .line
            .allocate(PREVIEW_LINE_GUID, &verts, None, inst, &self.gpu.device, &self.gpu.queue);
    }

    pub(crate) fn clear_draw_preview(&mut self) {
        self.scene.gpu_session.line.free(PREVIEW_LINE_GUID);
    }
}
