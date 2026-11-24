use crate::app::clipping::{self, Mode as ClipMode};
use crate::app::command::tool::{Next, Tool};
use crate::app::command::verbs::geometry::Draw;
use crate::app::command::verbs::measure::Mark;
use crate::app::coords;
use crate::app::cplane::CPlane;
use crate::app::gizmo::Gizmo;
use crate::app::hierarchy::Hierarchy;
use crate::app::layers;
use crate::app::scene::{FileDoc, READ_ONLY, Scene};
use crate::app::selection::{Controls, SelectionMode, SelectionTool};
use crate::app::snap::{Screen, Snapping};
use crate::app::splitting;
use crate::camera::Camera;
use crate::rows::{Instance, ObjectRow, TextLabel, world_box};
use session_rust::{AABB, Geometry, Plane, Point, Polyline, Xform};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

/// Boolean viewport visibility switches shared by `View` commands and `DisplaySettings`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ViewFlag {
    Points,
    Lines,
    Edges,
    Outline,
    Lighting,
    Backfaces,
    Wireframe,
    CullBackfaces,
}

/// Authoritative display settings for headless execution and `.session` persistence.
#[derive(Clone, Debug)]
pub struct DisplaySettings {
    pub ssao: bool,
    pub wireframe: bool,
    pub show_outlines: bool,
    pub show_points: bool,
    pub show_lines: bool,
    pub show_mesh_edges: bool,
    pub lit: bool,
    pub backface: bool,
    pub cull_backfaces: bool,
    pub opacity: f32,
    pub show_grid: bool,
    pub cloud_size: f32,
    pub show_selected_names: bool,
    pub clipping_enabled: bool,
    pub clipping_solid: bool,
    pub layers_open: bool,
    pub layers_expanded: bool,
    pub properties_open: bool,
    pub gizmo_align: crate::app::gizmo::GizmoAlign,
    pub gizmo_pivot: Option<Point>,
}

impl Default for DisplaySettings {
    fn default() -> Self {
        Self {
            ssao: false,
            wireframe: false,
            show_outlines: true,
            show_points: true,
            show_lines: true,
            show_mesh_edges: true,
            lit: true,
            backface: true,
            cull_backfaces: false,
            opacity: 0.95,
            show_grid: true,
            cloud_size: 2.0,
            show_selected_names: true,
            clipping_enabled: true,
            clipping_solid: false,
            layers_open: false,
            layers_expanded: false,
            properties_open: false,
            gizmo_align: crate::app::gizmo::GizmoAlign::World,
            gizmo_pivot: None,
        }
    }
}

/// One authoritative selection model shared by core and UI adapters.
#[derive(Clone, Debug, Default)]
pub struct SelectionState {
    pub mode: SelectionMode,
    pub tool: SelectionTool,
    pub primary: Option<u32>,
    pub order: Vec<u32>,
    pub highlighted: Vec<u32>,
    pub controls: Controls,
}

impl SelectionState {
    /// Every selected row in ascending order.
    pub fn selected_rows(&self) -> Vec<u32> {
        if self.highlighted.is_empty() {
            self.primary.into_iter().collect()
        } else {
            self.highlighted.clone()
        }
    }

    /// Selected rows in pick order.
    pub fn ordered_rows(&self) -> Vec<u32> {
        ordered_selection(&self.order, &self.selected_rows())
    }
}

/// Preserve `order` for rows still in `selected`, appending newly added rows at the end.
pub fn ordered_selection(order: &[u32], selected: &[u32]) -> Vec<u32> {
    let mut left: HashSet<u32> = selected.iter().copied().collect();
    let mut rows: Vec<u32> = Vec::with_capacity(selected.len());

    for row in order.iter().chain(selected) {
        if left.remove(row) {
            rows.push(*row);
        }
    }

    rows
}

/// UI- and GPU-independent execution seam for all command verbs and interactive tools.
pub trait CommandHost {
    fn scene(&self) -> &Scene;
    fn scene_mut(&mut self) -> &mut Scene;
    fn camera(&self) -> &Camera;
    fn camera_mut(&mut self) -> &mut Camera;

    fn selection(&self) -> &SelectionMode;
    fn set_selection_tool(&mut self, tool: SelectionTool);
    fn selected_rows(&self) -> Vec<u32>;
    fn ordered_rows(&self) -> Vec<u32>;
    fn select(&mut self, row: Option<u32>);
    fn select_rows(&mut self, rows: Vec<u32>, additive: bool);
    fn escape_selection(&mut self);
    fn enable_controls(&mut self);

    fn snap(&self) -> &Snapping;
    fn snap_mut(&mut self) -> &mut Snapping;
    fn gizmo_origin(&self) -> Option<Point>;
    fn place_gizmo(&mut self, row: Option<u32>);

    fn viewport(&self) -> (f64, f64);
    fn aspect(&self) -> f64 {
        let (w, h) = self.viewport();
        w.max(1.0) / h.max(1.0)
    }
    fn pixel_scale(&self) -> f64;
    fn view_screen(&self) -> Screen;
    fn refresh_bounds(&mut self);
    fn scene_bounds(&self) -> AABB;
    fn row_bounds(&self, row: u32) -> Option<AABB>;
    fn row_flags(&self, row: u32) -> Option<u32>;

    fn locked_reason(&mut self, rows: &[u32]) -> Option<String>;
    fn after_history(&mut self);
    fn commit_rows(&mut self);
    fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String>;
    fn copy_selection(&mut self, delta: &Xform) -> Result<String, String>;
    fn delete_selection(&mut self) -> Result<String, String>;
    fn hide_selected(&mut self);
    fn show_all(&mut self);
    fn fit_selected_or_all(&mut self);

    fn start_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String>;
    fn open_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String>;
    fn tool_running(&self) -> bool;
    fn drafting(&self) -> bool;
    fn cancel_drawing(&mut self);
    fn close_drawing(&mut self) -> Result<String, String>;
    fn drawing_cursor(&self) -> Option<&Point>;
    fn run_command(&mut self, line: &str) -> Result<String, String>;
    fn split_command(&mut self) -> Result<String, String>;

    fn set_row_selected(&mut self, row: u32, selected: bool);
    fn set_row_hidden(&mut self, row: u32, hidden: bool);
    fn restore_source_render(&mut self, row: u32);
    fn preview_geometry(&mut self, row: u32, geometry: Geometry) -> Result<(), String>;

    fn arctic(&self) -> bool;
    fn set_arctic(&mut self, on: bool);
    fn outlines(&self) -> bool;
    fn set_outlines(&mut self, on: bool);
    fn view_flag(&self, flag: ViewFlag) -> bool;
    fn set_view_flag(&mut self, flag: ViewFlag, on: bool);
    fn opacity(&self) -> f32;
    fn set_opacity(&mut self, value: f32);
    fn toggle_xray(&mut self);
    fn set_cloud_size(&mut self, size: f32);
    fn set_plane_size(&mut self, size: f64);
    fn toggle_selected_names(&mut self);
    fn show_attributes(&mut self, value: Option<bool>) -> bool;
    fn show_interactions(&mut self, value: Option<bool>) -> bool;

    fn set_construction_plane(&mut self, plane: Option<CPlane>);
    fn construction_plane_status(&self) -> serde_json::Value;
    fn clipping_half_size(&mut self) -> f64;
    fn pick_clipping_plane(&mut self, mode: ClipMode) -> String;
    fn create_clipping_plane(&mut self, plane: Plane) -> Result<String, String>;
    fn flip_clipping_planes(&mut self) -> Result<String, String>;
    fn set_clipping(&mut self, on: bool) -> String;
    fn set_clipping_fill(&mut self, solid: Option<bool>) -> String;

    fn add_text(&mut self, label: TextLabel);
    fn set_mark(&mut self, points: Vec<Point>, label: String);
    fn mark_selection(&mut self, label: String);
    fn refresh_layers(&mut self);
    fn expand_layers(&mut self);
    fn reveal_layer(&mut self, doc: usize, name: &str);
    fn set_layers_panel(&mut self, open: Option<bool>, expand_all: bool);
    fn set_properties_panel(&mut self, open: Option<bool>) -> String;
    fn set_object_alpha(&mut self, alpha: Option<f32>) -> Result<String, String>;
    fn lock_selected(&mut self, locked: bool) -> Result<String, String>;
    fn set_gizmo_align(&mut self, align: crate::app::gizmo::GizmoAlign) -> String;
    fn set_gizmo_pivot(&mut self, pivot: Option<Point>) -> String;
    fn export_obj(&mut self, path: Option<&str>) -> Result<String, String>;
    fn import_file(&mut self, path: Option<&str>) -> Result<String, String>;
    fn fetch_wanted(&mut self);
    fn save_session(&mut self) -> Result<String, String>;
    fn open_session_prompt(&mut self) -> Result<String, String>;
    fn download_report(&mut self) -> Result<String, String>;
    fn saved_view(&self) -> crate::app::session_io::SavedView;
    fn install_opened_session(&mut self, opened: crate::app::session_io::OpenedSession);
    fn clear_scene(&mut self);
    fn append_file(&mut self, doc: FileDoc);
    fn perf_stats(&self) -> serde_json::Value {
        let scene = self.scene();
        let (w, h) = self.viewport();
        serde_json::json!({
            "mode": "headless",
            "documents": scene.docs.len(),
            "objects": scene.object_count(),
            "rows": scene.row_count(),
            "viewport": [w, h],
            "pixel_scale": self.pixel_scale(),
        })
    }
    fn note_status(&mut self, _message: &str) {}
    fn ensure_graph_open(&mut self) {}
    fn touch(&mut self);
}

/// Active draft state for `HeadlessHost` (mirrors `State`'s drawing/tool state without a window or GPU).
struct HeadlessDraft {
    verb: String,
    draw: Option<&'static Draw>,
    prefix: String,
    needed: usize,
    prompts: &'static [&'static str],
    construction: String,
    sides: usize,
    points: Vec<Point>,
    plane: CPlane,
    frame: Plane,
    hover: Option<Point>,
    tool: Option<Box<dyn Tool>>,
    then: Option<String>,
}

impl HeadlessDraft {
    fn new(verb: &str, prefix: &str, plane: CPlane) -> Self {
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
            hover: None,
            tool: None,
            then: None,
        }
    }

    fn places_points(&self) -> bool {
        self.then.is_none() && self.tool.as_ref().is_none_or(|tool| tool.asks_points())
    }
}

/// Pending split state for `HeadlessHost`.
struct HeadlessSplit {
    target: u32,
    face: Option<usize>,
    cutters: Vec<u32>,
}

/// Pure CPU `CommandHost` with zero window, egui, wgpu, or browser dependencies.
pub struct HeadlessHost {
    pub scene: Scene,
    pub camera: Camera,
    pub selection: SelectionState,
    pub snap: Snapping,
    pub construction_plane: Option<CPlane>,
    pub display: DisplaySettings,
    pub gizmo: Option<Gizmo>,
    pub hierarchy: Hierarchy,
    pub mark: Option<Mark>,
    pub viewport_size: (f64, f64),
    pub dpr: f64,
    pub rows: HashMap<u32, ObjectRow>,
    pub bounds: AABB,
    pub last_command: Option<String>,
    pub saved_bytes: Option<Vec<u8>>,
    pub status_line: String,
    pub reports_requested: usize,
    pub open_prompts: usize,
    draft: Option<HeadlessDraft>,
    pending_split: Option<HeadlessSplit>,
}

impl Default for HeadlessHost {
    fn default() -> Self {
        Self::new()
    }
}

impl HeadlessHost {
    /// Empty headless host with a 1280×800 viewport.
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            camera: Camera::new(),
            selection: SelectionState::default(),
            snap: Snapping::default(),
            construction_plane: None,
            display: DisplaySettings::default(),
            gizmo: None,
            hierarchy: Hierarchy::default(),
            mark: None,
            viewport_size: (1280.0, 800.0),
            dpr: 1.0,
            rows: HashMap::new(),
            bounds: AABB::empty(),
            last_command: None,
            saved_bytes: None,
            status_line: String::new(),
            reports_requested: 0,
            open_prompts: 0,
            draft: None,
            pending_split: None,
        }
    }

    /// Construct a headless host from an existing `Scene`, syncing all walked rows to CPU bounds.
    pub fn from_scene(mut scene: Scene) -> Self {
        let mut host = Self::new();
        scene.drain_changes(&mut host.rows);
        host.scene = scene;
        host.recompute_bounds();
        host
    }

    /// Append a `FileDoc` and sync its rows into the CPU row table.
    pub fn add_file(&mut self, doc: FileDoc) {
        self.scene.add_file(doc);
        self.scene.drain_changes(&mut self.rows);
        self.recompute_bounds();
        self.camera.grow_extent(&self.bounds);
    }

    /// Clear every document and reset selection/state.
    pub fn clear(&mut self) {
        self.cancel_drawing();
        self.pending_split = None;
        self.selection = SelectionState::default();
        self.gizmo = None;
        self.hierarchy = Hierarchy::default();
        self.scene.clear_cpu(&mut self.rows);
        self.bounds = AABB::empty();
    }

    /// Deliver an object-pick result to a running interactive tool (e.g. `Loft`, `Trim`, `Extend`).
    pub fn tool_pick(&mut self, row: Option<u32>) -> Option<Result<String, String>> {
        self.call_tool(|tool, host, _, _| Some(tool.picked(host, row)))
    }

    /// Deliver a screen-space click `(x, y)` in device pixels to a running interactive tool.
    pub fn tool_click(&mut self, at: (f64, f64)) -> Option<Result<String, String>> {
        self.call_tool(|tool, host, _, _| tool.clicked(host, at))
    }

    /// Choose a cutter row during an active `Split` command.
    pub fn pick_split_cutter(&mut self, row: u32) {
        let Some(pending) = self.pending_split.as_mut() else {
            return;
        };
        if row == pending.target
            || !self.scene.selectable(row)
            || !self.scene.geometry(row).is_some_and(splitting::is_cutter)
        {
            return;
        }
        if let Some(at) = pending.cutters.iter().position(|r| *r == row) {
            pending.cutters.remove(at);
        } else if pending.cutters.len() < 64 {
            pending.cutters.push(row);
        }
    }

    fn recompute_bounds(&mut self) {
        let mut total = AABB::empty();
        for object in self.rows.values() {
            if object.flags & (Instance::FLAG_HIDDEN | Instance::FLAG_DEAD) == 0 {
                let wb = world_box(object);
                if wb.is_valid() {
                    total.union_with(&wb);
                }
            }
        }
        self.bounds = total;
        self.scene.bounds_stale = false;
    }

    fn clipping_rows(&self) -> Vec<u32> {
        let mut out: Vec<u32> = self
            .rows
            .iter()
            .filter_map(|(&row, obj)| {
                (obj.flags & Instance::FLAG_DEAD == 0
                    && obj.flags & Instance::FLAG_CLIPPING_PLANE != 0)
                    .then_some(row)
            })
            .collect();
        out.sort_unstable();
        out
    }

    fn facing(&self) -> CPlane {
        if let Some(plane) = self.construction_plane {
            return plane;
        }
        let forward = self
            .camera
            .orientation
            .rotate_vector(session_rust::Vector::y_axis());
        CPlane::view(&forward)
    }

    fn drawing_prompt(&self) -> String {
        let Some(draft) = self.draft.as_ref() else {
            return String::new();
        };
        if draft.then.is_some() {
            return "Select objects, then press Enter".into();
        }
        if let Some(tool) = draft.tool.as_ref() {
            return format!("{}: {}", tool.name(), tool.prompt(&draft.points));
        }
        if !draft.prompts.is_empty() {
            let step = draft.points.len().min(draft.prompts.len() - 1);
            return format!("{}: {}", draft.verb, draft.prompts[step]);
        }
        format!("{}: point {}", draft.verb, draft.points.len() + 1)
    }

    fn call_tool(
        &mut self,
        call: impl FnOnce(&mut dyn Tool, &mut Self, &[Point], &Plane) -> Option<Result<Next, String>>,
    ) -> Option<Result<String, String>> {
        let mut draft = self.draft.take()?;
        let Some(mut tool) = draft.tool.take() else {
            self.draft = Some(draft);
            return None;
        };
        let answer = call(tool.as_mut(), self, &draft.points, &draft.frame);
        draft.tool = Some(tool);
        let Some(step) = answer else {
            self.draft = Some(draft);
            return None;
        };
        match step {
            Ok(Next::More) => {
                self.draft = Some(draft);
                Some(Ok(self.drawing_prompt()))
            }
            Ok(Next::Repeat(message)) => {
                draft.points.truncate(1);
                self.draft = Some(draft);
                self.place_gizmo(None);
                Some(Ok(format!("{message} · {}", self.drawing_prompt())))
            }
            Ok(Next::Done(message)) => {
                self.place_gizmo(self.scene.selected);
                Some(Ok(message))
            }
            Err(error) => {
                self.draft = Some(draft);
                Some(Err(error))
            }
        }
    }

    fn accept_coordinates(&mut self, text: &str) -> Result<String, String> {
        for word in text.split_whitespace() {
            let (typed, world) =
                coords::parse_world(word).ok_or("Use x,y,z, @dx,dy,dz, or distance<angle")?;
            let draft = self.draft.as_ref().ok_or("No active draft")?;
            let (x, y) = draft.plane.axes();
            let previous = draft.points.last();
            let point = coords::place(typed, world, &x, &y, previous, None)
                .ok_or("This coordinate needs a previous point or a cursor direction")?;
            if (0..3).any(|i| !point[i].is_finite() || point[i].abs() > 1e12) {
                return Err("Coordinates must be finite and within ±1e12".into());
            }
            self.draft.as_mut().unwrap().points.push(point);

            if self.draft.as_ref().is_some_and(|d| d.tool.is_some()) {
                return self
                    .call_tool(|tool, host, points, plane| Some(tool.placed(host, points, plane)))
                    .unwrap_or_else(|| Ok(self.drawing_prompt()));
            }

            let done = self.draft.as_ref().is_some_and(|d| {
                (d.needed > 0 && d.points.len() >= d.needed)
                    || (!d.prompts.is_empty() && d.points.len() >= d.prompts.len())
            });
            if done {
                return self.finish_drawing();
            }
        }
        Ok(self.drawing_prompt())
    }

    fn finish_drawing(&mut self) -> Result<String, String> {
        let Some(mut draft) = self.draft.take() else {
            return Err("Nothing is being drawn".into());
        };
        if let Some(mut tool) = draft.tool.take() {
            let res = tool.enter(self, &draft.points);
            return match res {
                Ok(Next::Done(msg) | Next::Repeat(msg)) => Ok(msg),
                Ok(Next::More) => {
                    draft.tool = Some(tool);
                    self.draft = Some(draft);
                    Ok(self.drawing_prompt())
                }
                Err(err) => {
                    draft.tool = Some(tool);
                    self.draft = Some(draft);
                    Err(err)
                }
            };
        }
        if !draft.prompts.is_empty() {
            if draft.points.len() < draft.prompts.len() {
                let need = draft.prompts.len();
                self.draft = Some(draft);
                return Err(format!("Needs {need} points"));
            }
            let coords_text: Vec<String> = draft
                .points
                .iter()
                .map(|p| format!("{},{},{}", p[0], p[1], p[2]))
                .collect();
            return self.run_command(&format!("{} {}", draft.prefix, coords_text.join(" ")));
        }
        let Some(draw) = draft.draw else {
            return Err("Nothing is being drawn".into());
        };
        let world: Vec<[f64; 3]> = if draft.construction != "points" {
            headless_construction_points(
                &draft.construction,
                draft.plane,
                draft.sides,
                &draft.points,
            )?
            .into_iter()
            .map(|p| [p[0], p[1], p[2]])
            .collect()
        } else {
            draft.points.iter().map(|p| [p[0], p[1], p[2]]).collect()
        };
        let (doc, guid) = self.scene.model(draw, &world)?;
        self.after_history();
        let row = self.scene.row_of(doc, &guid);
        self.select(row);
        Ok(format!("created {} ({guid})", draw.what))
    }

    fn drawing_command(&mut self, text: &str) -> Option<Result<String, String>> {
        let words: Vec<_> = text.split_whitespace().collect();
        if let Some((draw, count)) = crate::app::command::drawing(&words) {
            let construction = words
                .get(count)
                .filter(|word| {
                    draw.spec.options.iter().any(|option| {
                        option.eq_ignore_ascii_case(&format!("{} {word}", draw.spec.names[0]))
                    })
                })
                .map(|word| word.to_ascii_lowercase());
            let start = count + usize::from(construction.is_some());
            if construction.is_some() || words.len() - count < *draw.points.start() {
                self.cancel_drawing();
                self.pending_split = None;
                let needed = match &construction {
                    _ if !draw.open() => *draw.points.start(),
                    Some(kind) if kind != "points" => 2,
                    _ => 0,
                };
                let verb = words[..count].join(" ").to_ascii_lowercase();
                let mut draft = HeadlessDraft::new(&verb, &verb, self.facing());
                draft.draw = Some(draw);
                draft.needed = needed;
                draft.construction = construction.unwrap_or_else(|| "points".into());
                self.draft = Some(draft);
                return Some(if words.len() > start {
                    self.accept_coordinates(&words[start..].join(" "))
                } else {
                    Ok(self.drawing_prompt())
                });
            }
        }
        self.draft.as_ref()?;

        if let Some(then) = self.draft.as_ref().and_then(|d| d.then.clone()) {
            if text.trim().is_empty() {
                if self.scene.selected.is_none() {
                    return Some(Err(
                        "Nothing is selected; click objects, then press Enter".into()
                    ));
                }
                self.cancel_drawing();
                return Some(self.run_command(&then));
            }
            if words.first().and_then(|w| coords::parse(w)).is_some() {
                return Some(Err("Select objects, then press Enter".into()));
            }
            return None;
        }

        if self.draft.as_ref().is_some_and(|d| d.tool.is_some()) {
            if text.trim().is_empty() {
                return self.call_tool(|tool, host, points, _| Some(tool.enter(host, points)));
            }
            let mut answer = None;
            for word in &words {
                if self.draft.as_ref().is_none_or(|d| d.tool.is_none()) {
                    break;
                }
                let taken = self
                    .call_tool(|tool, host, points, plane| tool.word(host, word, points, plane));
                let step = match taken {
                    Some(step) => step,
                    None if coords::parse(word).is_some() => self.accept_coordinates(word),
                    None if answer.is_none() => return None,
                    None => Err(format!("`{word}` is not a point or a value")),
                };
                if step.is_err() {
                    return Some(step);
                }
                answer = Some(step);
            }
            return answer;
        }

        let verb_name = self.draft.as_ref()?.verb.clone();
        if self
            .draft
            .as_ref()
            .is_some_and(HeadlessDraft::places_points)
        {
            let word = words.concat().to_ascii_lowercase();
            let preset = match word.as_str() {
                "xy" => Some(CPlane::Xy),
                "xz" => Some(CPlane::Xz),
                "yz" => Some(CPlane::Yz),
                _ => None,
            };
            if let Some(plane) = preset {
                self.set_construction_plane(Some(plane));
                return Some(Ok(self.drawing_prompt()));
            }
        }
        if let [word] = words.as_slice()
            && let Some(option) = crate::app::command::options(&verb_name)
                .iter()
                .find(|option| option.eq_ignore_ascii_case(&format!("{verb_name} {word}")))
        {
            return Some(self.run_command(option));
        }

        if text.trim().is_empty() {
            return Some(self.finish_drawing());
        }
        if words.first().and_then(|w| coords::parse(w)).is_some() {
            return Some(self.accept_coordinates(text));
        }
        None
    }

    fn command_action(&mut self, line: &str) -> Result<String, String> {
        self.mark = None;
        if let Some(result) = self.drawing_command(line) {
            return result;
        }
        let action = match crate::app::command::parse(line) {
            Ok(action) => action,
            Err(error) => match crate::app::command::choices(line) {
                Some((verb, options)) => {
                    return self.open_tool(Box::new(
                        crate::app::command::tool::choices::Choices::new(verb, options),
                    ));
                }
                None => return Err(error),
            },
        };

        let tool_yields = self
            .draft
            .as_ref()
            .and_then(|d| d.tool.as_ref())
            .is_some_and(|t| t.yields());
        if !action.keeps_draft() || tool_yields {
            self.cancel_drawing();
        }
        if !action.keeps_split() {
            self.pending_split = None;
        }
        if action.needs_selection() && self.scene.selected.is_none() {
            self.cancel_drawing();
            self.pending_split = None;
            let mut draft = HeadlessDraft::new("select", "select", self.facing());
            draft.then = Some(line.to_owned());
            self.draft = Some(draft);
            self.place_gizmo(None);
            return Ok(self.drawing_prompt());
        }

        action.run(self)
    }

    fn finish_split(&mut self) -> Result<String, String> {
        let pending = self.pending_split.take().ok_or("Start Split first")?;
        if pending.cutters.is_empty() {
            self.pending_split = Some(pending);
            return Err("Select at least one cutter curve, then press Enter".into());
        }
        let identity = self.scene.identity_of(pending.target);
        let result = self
            .scene
            .split_rows(pending.target, pending.face, &pending.cutters);
        match result {
            Ok(regions) if regions > 1 => {
                self.after_history();
                let row = identity.and_then(|(doc, guid)| self.scene.row_of(doc, &guid));
                self.select(row);
                Ok(format!(
                    "Split into {regions} regions. The BRep stays joined; Undo restores the original. Cutters are retained."
                ))
            }
            Ok(_) => {
                Ok("No division: cutters must cross the curve or lie on the selected face.".into())
            }
            Err(error) => Err(error),
        }
    }
}

fn headless_construction_points(
    kind: &str,
    plane: CPlane,
    sides: usize,
    points: &[Point],
) -> Result<Vec<Point>, String> {
    let [a, b] = points else {
        return Err("Pick two points to complete this polyline".into());
    };
    let (x, y) = plane.axes();
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

impl CommandHost for HeadlessHost {
    fn scene(&self) -> &Scene {
        &self.scene
    }

    fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    fn camera(&self) -> &Camera {
        &self.camera
    }

    fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    fn selection(&self) -> &SelectionMode {
        &self.selection.mode
    }

    fn set_selection_tool(&mut self, tool: SelectionTool) {
        self.selection.tool = tool;
    }

    fn selected_rows(&self) -> Vec<u32> {
        self.selection.selected_rows()
    }

    fn ordered_rows(&self) -> Vec<u32> {
        self.selection.ordered_rows()
    }

    fn select(&mut self, row: Option<u32>) {
        self.pending_split = None;
        let row = row.filter(|r| self.scene.selectable(*r));
        for old in std::mem::take(&mut self.selection.highlighted) {
            self.set_row_selected(old, false);
        }
        if let Some(old) = self.selection.primary.take() {
            self.set_row_selected(old, false);
        }
        self.selection.mode = SelectionMode::Object;
        self.selection.controls = Controls::default();
        if let Some(r) = row {
            self.set_row_selected(r, true);
        }
        self.selection.primary = row;
        self.scene.selected = row;
        self.selection.order = row.into_iter().collect();
        self.place_gizmo(row);
    }

    fn select_rows(&mut self, rows: Vec<u32>, additive: bool) {
        let mut selected = if additive {
            self.ordered_rows()
        } else {
            Vec::new()
        };
        selected.extend(rows.into_iter().filter(|r| {
            self.scene.selectable(*r)
                && self
                    .scene
                    .identity_of(*r)
                    .is_some_and(|id| !self.scene.hidden.contains(&id))
        }));
        let order = selected.clone();
        selected.sort_unstable();
        selected.dedup();
        self.select(None);
        self.selection.order = ordered_selection(&order, &selected);
        let primary = selected.first().copied();
        self.selection.primary = primary;
        self.scene.selected = primary;
        for &row in &selected {
            self.set_row_selected(row, true);
        }
        if selected.len() > 1 {
            self.selection.highlighted = selected;
        }
        self.place_gizmo(primary);
    }

    fn escape_selection(&mut self) {
        let parent = self.selection.mode.escape();
        self.select(parent);
        self.mark = None;
        self.status_line.clear();
    }

    fn enable_controls(&mut self) {
        let Some(parent) = self.scene.selected else {
            self.status_line = "Select one object before running Controls".into();
            return;
        };
        if matches!(self.selection.mode, SelectionMode::Controls { parent: active, .. } if active == parent)
        {
            return;
        }
        let Some(geometry) = self.scene.geometry(parent) else {
            self.status_line =
                "Source controls are unavailable for this display-only object".into();
            return;
        };
        let controls = Controls::from_geometry(geometry);
        if !controls.cloud && controls.points.is_empty() {
            self.status_line = "This object has no selectable source controls".into();
            return;
        }
        self.place_gizmo(None);
        self.selection
            .mode
            .enable_controls(Some(parent), controls.cloud);
        self.set_row_selected(parent, false);
        self.selection.controls = controls;
        self.status_line = "Control points: click to select; Esc to leave".into();
    }

    fn snap(&self) -> &Snapping {
        &self.snap
    }

    fn snap_mut(&mut self) -> &mut Snapping {
        &mut self.snap
    }

    fn gizmo_origin(&self) -> Option<Point> {
        self.gizmo.as_ref().map(|g| g.origin.clone())
    }

    fn place_gizmo(&mut self, row: Option<u32>) {
        let Some(r) = row else {
            self.gizmo = None;
            return;
        };
        let mut bounds = AABB::empty();
        for sel in self.selected_rows() {
            if let Some(b) = self.row_bounds(sel) {
                bounds.union_with(&b);
            }
        }
        if !bounds.is_valid()
            && let Some(b) = self.row_bounds(r)
        {
            bounds = b;
        }
        if bounds.is_valid() {
            self.gizmo = Some(Gizmo::new(Point::new(bounds.cx, bounds.cy, bounds.cz)));
        } else {
            self.gizmo = None;
        }
    }

    fn viewport(&self) -> (f64, f64) {
        self.viewport_size
    }

    fn pixel_scale(&self) -> f64 {
        self.dpr
    }

    fn view_screen(&self) -> Screen {
        let origin = self.camera.origin();
        let mvp = self.camera.view_proj_anchored(self.aspect(), &origin);
        Screen::new(mvp.m, [origin[0], origin[1], origin[2]], self.viewport_size)
    }

    fn refresh_bounds(&mut self) {
        if self.scene.bounds_stale {
            self.recompute_bounds();
        }
    }

    fn scene_bounds(&self) -> AABB {
        self.bounds
    }

    fn row_bounds(&self, row: u32) -> Option<AABB> {
        self.rows
            .get(&row)
            .filter(|o| o.flags & Instance::FLAG_DEAD == 0)
            .map(world_box)
            .filter(AABB::is_valid)
    }

    fn row_flags(&self, row: u32) -> Option<u32> {
        self.rows.get(&row).map(|o| o.flags)
    }

    fn locked_reason(&mut self, rows: &[u32]) -> Option<String> {
        let mut reason = None;
        for &row in rows {
            if let Some(doc) = self.scene.released_doc(row) {
                reason = self.scene.editable(doc).err();
            } else if self.scene.display_only(row) {
                return Some(READ_ONLY.into());
            }
        }
        reason
    }

    fn after_history(&mut self) {
        self.cancel_drawing();
        self.hierarchy.page = 0;
        self.selection.mode = SelectionMode::Object;
        self.select(None);
        self.commit_rows();
        self.place_gizmo(None);
    }

    fn commit_rows(&mut self) {
        self.scene.sync();
        self.scene.drain_changes(&mut self.rows);
        for row in std::mem::take(&mut self.scene.appearance_rows) {
            if let Some(id) = self.scene.identity_of(row)
                && let Some(obj) = self.rows.get_mut(&row)
            {
                if self.scene.hidden.contains(&id) {
                    obj.flags |= Instance::FLAG_HIDDEN;
                } else {
                    obj.flags &= !Instance::FLAG_HIDDEN;
                }
                if let Some(c) = self.scene.colors.get(&id) {
                    obj.color[0] = c[0] as f32 / 255.0;
                    obj.color[1] = c[1] as f32 / 255.0;
                    obj.color[2] = c[2] as f32 / 255.0;
                    obj.flags |= Instance::FLAG_COLOR;
                } else {
                    obj.flags &= !Instance::FLAG_COLOR;
                }
                if let Some(c) = self.scene.edge_colors.get(&id) {
                    obj.edge_color = u32::from_le_bytes([c[0], c[1], c[2], 255]);
                    obj.flags |= Instance::FLAG_EDGE_COLOR;
                } else {
                    obj.flags &= !Instance::FLAG_EDGE_COLOR;
                }
                if let Some(&a) = self.scene.alphas.get(&id) {
                    obj.color[3] = a;
                    obj.flags |= Instance::FLAG_ALPHA;
                } else {
                    obj.color[3] = 1.0;
                    obj.flags &= !Instance::FLAG_ALPHA;
                }
            }
        }
        if self.scene.compaction_due() {
            self.scene.rewalk_editable_cpu(&mut self.rows);
        }
        self.recompute_bounds();
        let gone = |row: &u32| self.scene.identity_of(*row).is_none();
        let lost = self.selected_rows().iter().any(gone)
            || self.selection.mode.parent().is_some_and(|r| gone(&r));
        let survivors: Vec<u32> = self
            .selected_rows()
            .into_iter()
            .filter(|row| !gone(row))
            .collect();
        if self
            .pending_split
            .as_ref()
            .is_some_and(|s| gone(&s.target) || s.cutters.iter().any(gone))
        {
            self.pending_split = None;
        }
        if lost {
            self.select_rows(survivors, false);
        }
        self.refresh_layers();
    }

    fn apply(&mut self, delta: Xform, label: &str) -> Result<String, String> {
        let Some(row) = self.scene.selected else {
            return Err("nothing is selected".into());
        };
        let rows = self.selected_rows();
        if let Some(reason) = self.locked_reason(&rows) {
            return Err(reason);
        }
        if let Some(target) = crate::app::deform::Target::selected(&self.selection.mode) {
            self.scene.edit_subobject(row, target, &delta, label)?;
            self.commit_rows();
            self.place_gizmo(Some(row));
            return Ok(label.into());
        }
        self.scene
            .transform_rows(&rows, &delta, label)
            .ok_or("this selection cannot be edited")?;
        self.commit_rows();
        self.place_gizmo(Some(row));
        Ok(label.into())
    }

    fn copy_selection(&mut self, delta: &Xform) -> Result<String, String> {
        let rows = self.selected_rows();
        if let Some(reason) = self.locked_reason(&rows) {
            return Err(reason);
        }
        if crate::app::deform::Target::selected(&self.selection.mode).is_some() {
            return Err(
                "Copy works on whole objects; press Esc to leave the face, edge or control point"
                    .into(),
            );
        }
        let copies = self.scene.copy_rows(&rows, delta)?;
        self.commit_rows();
        Ok(format!("{} copied", copies.len()))
    }

    fn delete_selection(&mut self) -> Result<String, String> {
        let rows = self.selected_rows();
        if rows.is_empty() {
            return Err("nothing is selected".into());
        }
        let mut free = Vec::with_capacity(rows.len());
        let mut reason = None;
        for &row in &rows {
            match self.locked_reason(&[row]) {
                Some(why) => reason = Some(why),
                None => free.push(row),
            }
        }
        let skipped = rows.len() - free.len();
        let deleted = self.scene.delete_rows(&free);
        if deleted == 0 {
            return Err(reason.unwrap_or_else(|| "This object cannot be deleted".into()));
        }
        self.after_history();
        let message = match deleted {
            1 => "deleted".to_string(),
            _ => format!("deleted {deleted} objects"),
        };
        match reason {
            Some(why) => Ok(format!("{message}; skipped {skipped}: {why}")),
            None => Ok(message),
        }
    }

    fn hide_selected(&mut self) {
        let rows = self.selected_rows();
        if rows.is_empty() {
            return;
        }
        self.select(None);
        for row in self.scene.set_rows_hidden(&rows, true, "hide") {
            self.set_row_hidden(row, true);
        }
        self.scene.appearance_rows.clear();
        self.recompute_bounds();
        self.refresh_layers();
    }

    fn show_all(&mut self) {
        for row in self.scene.show_all_identities() {
            self.set_row_hidden(row, false);
        }
        self.scene.appearance_rows.clear();
        self.hierarchy.off.clear();
        self.recompute_bounds();
        self.refresh_layers();
    }

    fn fit_selected_or_all(&mut self) {
        let mut sel = self
            .selected_rows()
            .into_iter()
            .filter_map(|row| self.row_bounds(row));
        if let Some(mut b) = sel.next() {
            for next in sel {
                b.union_with(&next);
            }
            self.camera.fit_in(&b, self.aspect(), [0.0, 0.0, 1.0, 1.0]);
            self.camera.grow_extent(&self.bounds);
        } else {
            self.refresh_bounds();
            self.camera
                .fit_in(&self.bounds, self.aspect(), [0.0, 0.0, 1.0, 1.0]);
        }
    }

    fn start_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String> {
        let rows = self.selected_rows();
        if let Some(reason) = self.locked_reason(&rows) {
            return Err(reason);
        }
        if rows.iter().any(|&row| self.scene.geometry(row).is_none()) {
            return Err("This selection cannot be edited".into());
        }
        self.cancel_drawing();
        self.pending_split = None;
        let mut draft = HeadlessDraft::new(tool.name(), tool.name(), self.facing());
        draft.tool = Some(tool);
        self.draft = Some(draft);
        self.place_gizmo(None);
        Ok(self.drawing_prompt())
    }

    fn open_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String> {
        self.cancel_drawing();
        self.pending_split = None;
        let mut draft = HeadlessDraft::new(tool.name(), tool.name(), self.facing());
        draft.tool = Some(tool);
        self.draft = Some(draft);
        self.place_gizmo(None);
        self.call_tool(|tool, host, _, _| Some(tool.begin(host)))
            .unwrap_or_else(|| Ok(self.drawing_prompt()))
    }

    fn tool_running(&self) -> bool {
        self.draft.as_ref().is_some_and(|d| d.tool.is_some())
    }

    fn drafting(&self) -> bool {
        self.draft.is_some()
    }

    fn cancel_drawing(&mut self) {
        if let Some(mut draft) = self.draft.take()
            && let Some(mut tool) = draft.tool.take()
        {
            tool.cancel(self);
        }
    }

    fn close_drawing(&mut self) -> Result<String, String> {
        let Some(draft) = self.draft.as_ref() else {
            return Err("Close works while drawing Polyline or Curve".into());
        };
        if !matches!(draft.verb.as_str(), "polyline" | "curve") {
            return Err("Close works while drawing Polyline or Curve".into());
        }
        if draft.points.len() < 3 {
            return Err("Place at least three points before Close".into());
        }
        let first = draft.points[0].clone();
        self.draft.as_mut().unwrap().points.push(first);
        self.finish_drawing()
    }

    fn drawing_cursor(&self) -> Option<&Point> {
        self.draft.as_ref()?.hover.as_ref()
    }

    fn run_command(&mut self, line: &str) -> Result<String, String> {
        let line = crate::app::command::canonical(line);
        let result = self.command_action(&line);
        if result.is_ok()
            && let Some(command) = crate::app::command::repeat(&line)
        {
            self.last_command = Some(command);
        }
        result
    }

    fn split_command(&mut self) -> Result<String, String> {
        if self.pending_split.is_some() {
            return self.finish_split();
        }
        let row = self
            .scene
            .selected
            .ok_or("Select a curve or Ctrl+Shift-select a face, then run Split")?;
        if let Some(reason) = self.locked_reason(&[row]) {
            return Err(reason);
        }
        let selected_face = match self.selection.mode {
            SelectionMode::Face { face, .. } => Some(face),
            _ => None,
        };
        let face = splitting::face_index(
            self.scene.geometry(row).ok_or("Source unavailable")?,
            selected_face,
        )?;
        self.pending_split = Some(HeadlessSplit {
            target: row,
            face,
            cutters: Vec::new(),
        });
        self.place_gizmo(None);
        Ok("Split: select cutter lines, polylines or curves, then press Enter or tap Split again. Esc cancels. Faces require on-surface cutters.".into())
    }

    fn set_row_selected(&mut self, row: u32, selected: bool) {
        if let Some(obj) = self.rows.get_mut(&row) {
            if selected {
                obj.flags |= Instance::FLAG_SELECTED;
            } else {
                obj.flags &= !Instance::FLAG_SELECTED;
            }
        }
    }

    fn set_row_hidden(&mut self, row: u32, hidden: bool) {
        if let Some(obj) = self.rows.get_mut(&row) {
            if hidden {
                obj.flags |= Instance::FLAG_HIDDEN;
            } else {
                obj.flags &= !Instance::FLAG_HIDDEN;
            }
        }
    }

    fn restore_source_render(&mut self, row: u32) {
        if let Some(geometry) = self.scene.geometry(row).cloned() {
            self.scene.redraw(row, &geometry, false);
            self.scene.drain_changes(&mut self.rows);
            self.recompute_bounds();
        }
    }

    fn preview_geometry(&mut self, row: u32, geometry: Geometry) -> Result<(), String> {
        let (doc, _) = self
            .scene
            .identity_of(row)
            .ok_or("Source is not editable")?;
        if self.scene.docs.get(doc).is_none_or(|f| f.display_only) {
            return Err(READ_ONLY.into());
        }
        self.scene.redraw(row, &geometry, true);
        self.scene.drain_changes(&mut self.rows);
        self.recompute_bounds();
        Ok(())
    }

    fn arctic(&self) -> bool {
        self.display.ssao
    }

    fn set_arctic(&mut self, on: bool) {
        self.display.ssao = on;
        if on {
            self.display.show_outlines = true;
            self.display.opacity = 1.0;
        }
    }

    fn outlines(&self) -> bool {
        self.display.show_outlines
    }

    fn set_outlines(&mut self, on: bool) {
        self.display.show_outlines = on;
    }

    fn view_flag(&self, flag: ViewFlag) -> bool {
        match flag {
            ViewFlag::Points => self.display.show_points,
            ViewFlag::Lines => self.display.show_lines,
            ViewFlag::Edges => self.display.show_mesh_edges,
            ViewFlag::Outline => self.display.show_outlines,
            ViewFlag::Lighting => self.display.lit,
            ViewFlag::Backfaces => self.display.backface,
            ViewFlag::Wireframe => self.display.wireframe,
            ViewFlag::CullBackfaces => self.display.cull_backfaces,
        }
    }

    fn set_view_flag(&mut self, flag: ViewFlag, on: bool) {
        match flag {
            ViewFlag::Points => self.display.show_points = on,
            ViewFlag::Lines => self.display.show_lines = on,
            ViewFlag::Edges => self.display.show_mesh_edges = on,
            ViewFlag::Outline => self.display.show_outlines = on,
            ViewFlag::Lighting => self.display.lit = on,
            ViewFlag::Backfaces => self.display.backface = on,
            ViewFlag::Wireframe => {
                self.display.wireframe = on;
                self.scene.wireframe = on;
                if on && self.scene.has_large_meshes() {
                    self.scene.rewalk_editable_cpu(&mut self.rows);
                }
            }
            ViewFlag::CullBackfaces => self.display.cull_backfaces = on,
        }
    }

    fn opacity(&self) -> f32 {
        self.display.opacity
    }

    fn set_opacity(&mut self, value: f32) {
        self.display.opacity = value.clamp(0.0, 1.0);
    }

    fn toggle_xray(&mut self) {
        self.display.opacity = if self.display.opacity > 0.0 { 0.0 } else { 1.0 };
    }

    fn set_cloud_size(&mut self, size: f32) {
        self.display.cloud_size = size.clamp(0.25, 8.0);
    }

    fn set_plane_size(&mut self, size: f64) {
        crate::rows::set_plane_size(size);
    }

    fn toggle_selected_names(&mut self) {
        self.display.show_selected_names = !self.display.show_selected_names;
    }

    fn show_attributes(&mut self, value: Option<bool>) -> bool {
        let show = value.unwrap_or(!self.scene.attributes);
        self.scene.attributes = show;
        self.select(None);
        let children = layers::attribute_rows(&self.scene);
        for row in children {
            if let Some(id) = self.scene.identity_of(row) {
                if show {
                    self.scene.hidden.remove(&id);
                } else {
                    self.scene.hidden.insert(id);
                }
                self.set_row_hidden(row, !show);
            }
        }
        self.scene.rewalk_editable_cpu(&mut self.rows);
        self.recompute_bounds();
        self.refresh_layers();
        show
    }

    fn show_interactions(&mut self, value: Option<bool>) -> bool {
        let show = value.unwrap_or(!self.scene.interactions);
        self.scene.interactions = show;
        self.select(None);
        self.scene.rewalk_editable_cpu(&mut self.rows);
        self.recompute_bounds();
        self.refresh_layers();
        show
    }

    fn set_construction_plane(&mut self, plane: Option<CPlane>) {
        self.construction_plane = plane;
        coords::set_plane(plane);
        let facing = self.facing();
        if let Some(draft) = self.draft.as_mut() {
            let (x, y) = facing.axes();
            draft.plane = facing;
            draft.frame = Plane::new(facing.origin(), x, y);
        }
    }

    fn construction_plane_status(&self) -> serde_json::Value {
        let plane = self.facing();
        let (x, y) = plane.axes();
        let o = plane.origin();
        let mode = match self.construction_plane {
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

    fn clipping_half_size(&mut self) -> f64 {
        self.refresh_bounds();
        let b = &self.bounds;
        let half = b.hx.max(b.hy).max(b.hz);
        if b.is_valid() && half.is_finite() && half > 0.0 {
            half * 1.1
        } else {
            500.0
        }
    }

    fn pick_clipping_plane(&mut self, mode: ClipMode) -> String {
        let prefix = format!("{} {}", clipping::NAME, mode.word());
        self.cancel_drawing();
        self.pending_split = None;
        let mut draft = HeadlessDraft::new(clipping::NAME, &prefix, self.facing());
        draft.prompts = mode.prompts();
        draft.needed = draft.prompts.len();
        self.draft = Some(draft);
        self.drawing_prompt()
    }

    fn create_clipping_plane(&mut self, plane: Plane) -> Result<String, String> {
        if self.clipping_rows().len() >= crate::rows::MAX_PLANES {
            return Err(format!(
                "at most {} clipping planes",
                crate::rows::MAX_PLANES
            ));
        }
        let (doc, guid) = self
            .scene
            .create_geometry(Geometry::Plane(Rc::new(plane)))?;
        self.after_history();
        let row = self.scene.row_of(doc, &guid);
        self.select(row);
        Ok(
            "Created Clipping Plane · the gumball moves the cut · Clipping Plane Off shows everything · Undo removes it"
                .into(),
        )
    }

    fn flip_clipping_planes(&mut self) -> Result<String, String> {
        let clip_rows = self.clipping_rows();
        let rows: Vec<u32> = self
            .selected_rows()
            .into_iter()
            .filter(|row| clip_rows.binary_search(row).is_ok())
            .collect();
        if rows.is_empty() {
            return Err("select a clipping plane to flip".into());
        }
        for &row in &rows {
            let flipped = match self.scene.geometry(row) {
                Some(Geometry::Plane(plane)) => clipping::flipped(plane),
                _ => continue,
            };
            self.scene.commit_geometry(
                row,
                Geometry::Plane(Rc::new(flipped)),
                "flip clipping plane",
            )?;
        }
        self.commit_rows();
        self.select_rows(rows, false);
        Ok("Flipped: the other side is cut away · Undo flips it back".into())
    }

    fn set_clipping(&mut self, on: bool) -> String {
        self.display.clipping_enabled = on;
        if on {
            "Clipping Plane On: the planes cut again".into()
        } else {
            "Clipping Plane Off: everything shows, the planes stay".into()
        }
    }

    fn set_clipping_fill(&mut self, solid: Option<bool>) -> String {
        let solid = solid.unwrap_or(!self.display.clipping_solid);
        self.display.clipping_solid = solid;
        format!(
            "Clipping Plane Fill {}",
            if solid { "Solid" } else { "Hatch" }
        )
    }

    fn add_text(&mut self, label: TextLabel) {
        self.scene.add_text(label);
        self.scene.drain_changes(&mut self.rows);
    }

    fn set_mark(&mut self, points: Vec<Point>, label: String) {
        self.mark = Some(Mark {
            points,
            label,
            revision: self.scene.row_revision,
        });
    }

    fn mark_selection(&mut self, label: String) {
        let mut bounds = AABB::empty();
        for row in self.selected_rows() {
            if let Some(object) = self.row_bounds(row) {
                bounds.union_with(&object);
            }
        }
        if bounds.is_valid() {
            self.set_mark(vec![Point::new(bounds.cx, bounds.cy, bounds.cz)], label);
        }
    }

    fn refresh_layers(&mut self) {
        self.hierarchy.refresh(&self.scene);
    }

    fn expand_layers(&mut self) {
        self.hierarchy.refresh(&self.scene);
        for index in 0..self.hierarchy.nodes.len() {
            if self.hierarchy.nodes[index].end > index + 1 {
                self.hierarchy.open.insert(index);
            }
        }
        self.display.layers_expanded = true;
    }

    fn reveal_layer(&mut self, doc: usize, name: &str) {
        self.hierarchy.refresh(&self.scene);
        if let Some(node) = self.hierarchy.index_of(doc, name) {
            self.hierarchy.reveal(node);
            self.hierarchy.open.insert(node);
        }
    }

    fn set_layers_panel(&mut self, open: Option<bool>, expand_all: bool) {
        self.display.layers_open = open.unwrap_or(!self.display.layers_open);
        self.refresh_layers();
        if expand_all {
            self.expand_layers();
        }
    }

    fn set_properties_panel(&mut self, open: Option<bool>) -> String {
        let next = open.unwrap_or(!self.display.properties_open);
        self.display.properties_open = next;
        format!("Properties {}", if next { "On" } else { "Off" })
    }

    fn set_object_alpha(&mut self, alpha: Option<f32>) -> Result<String, String> {
        let rows = self.selected_rows();
        if rows.is_empty() {
            return Err("Select an object first".into());
        }
        let changed = self.scene.set_rows_alpha(&rows, alpha);
        self.commit_rows();
        match alpha {
            Some(a) => Ok(format!(
                "Alpha {:.2} on {} object(s)",
                a,
                changed.len().max(1)
            )),
            None => Ok("Alpha reset".into()),
        }
    }

    fn lock_selected(&mut self, locked: bool) -> Result<String, String> {
        if locked {
            let rows = self.selected_rows();
            if rows.is_empty() {
                return Err("Select an object to lock".into());
            }
            self.select(None);
            let n = self.scene.set_rows_locked(&rows, true);
            self.refresh_layers();
            Ok(format!("Locked {n} object(s)"))
        } else {
            let n = self.scene.unlock_all_identities();
            self.refresh_layers();
            Ok(format!("Unlocked {n} object(s)"))
        }
    }

    fn set_gizmo_align(&mut self, align: crate::app::gizmo::GizmoAlign) -> String {
        self.display.gizmo_align = align;
        if let Some(row) = self.scene.selected {
            self.place_gizmo(Some(row));
        }
        match align {
            crate::app::gizmo::GizmoAlign::World => "Gumball aligned to World".into(),
            crate::app::gizmo::GizmoAlign::Object => "Gumball aligned to Object".into(),
            crate::app::gizmo::GizmoAlign::CPlane => "Gumball aligned to CPlane".into(),
        }
    }

    fn set_gizmo_pivot(&mut self, pivot: Option<Point>) -> String {
        self.display.gizmo_pivot = pivot.clone();
        if let Some(gizmo) = self.gizmo.as_mut() {
            gizmo.custom_pivot = pivot.clone();
            if let Some(p) = &pivot {
                gizmo.set_origin(p.clone());
            }
        }
        if pivot.is_none()
            && let Some(row) = self.scene.selected
        {
            self.place_gizmo(Some(row));
        }
        match pivot {
            Some(p) => format!("Gumball pivot ({:.2}, {:.2}, {:.2})", p[0], p[1], p[2]),
            None => "Gumball pivot reset".into(),
        }
    }

    fn export_obj(&mut self, path: Option<&str>) -> Result<String, String> {
        let sel = self.selected_rows();
        let filter = (!sel.is_empty()).then_some(sel.as_slice());
        let obj = crate::app::session_io::export_obj(&self.scene, filter)?;
        let bytes = obj.len();
        if let Some(path) = path {
            #[cfg(not(target_arch = "wasm32"))]
            {
                std::fs::write(path, &obj).map_err(|e| format!("Cannot write {path}: {e}"))?;
                return Ok(format!("Exported OBJ to {path} ({bytes} bytes)"));
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = path;
            }
        }
        Ok(format!("Exported OBJ ({bytes} bytes)"))
    }

    fn import_file(&mut self, path: Option<&str>) -> Result<String, String> {
        let Some(path) = path else {
            self.open_prompts += 1;
            return Ok("Choose a .pb, .obj or .session file to import".into());
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let bytes = std::fs::read(path).map_err(|e| format!("Cannot read {path}: {e}"))?;
            if path.to_ascii_lowercase().ends_with(".session") {
                let opened = crate::app::session_io::open_with_state(&bytes)?;
                self.scene = opened.scene;
                if let Some(cam) = opened.camera {
                    cam.apply_to(&mut self.camera);
                }
                if let Some(v) = opened.view {
                    self.display.ssao = v.arctic;
                    self.display.wireframe = v.wireframe;
                    self.scene.wireframe = v.wireframe;
                    self.display.show_outlines = v.show_outlines;
                    self.display.lit = v.lit;
                    self.display.backface = v.backface;
                    self.display.cull_backfaces = v.cull_backfaces;
                    self.display.opacity = v.opacity;
                    self.display.show_grid = v.show_grid;
                    self.display.cloud_size = v.cloud_size;
                }
                self.rows.clear();
                self.scene.drain_changes(&mut self.rows);
                self.recompute_bounds();
                self.refresh_layers();
                return Ok(format!("Opened session {path}"));
            }
            let doc = crate::app::session_io::import_bytes(&mut self.scene, path, &bytes)?;
            self.scene.drain_changes(&mut self.rows);
            self.recompute_bounds();
            self.refresh_layers();
            Ok(format!(
                "Imported {} ({} objects)",
                self.scene.docs[doc].name,
                self.scene.docs[doc].session.lookup.len()
            ))
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = path;
            Err(
                "Direct filesystem paths are native-only; run `Import` with no path in the browser"
                    .into(),
            )
        }
    }

    fn fetch_wanted(&mut self) {}

    fn save_session(&mut self) -> Result<String, String> {
        let cam = crate::app::session_io::SavedCamera::from_camera(&self.camera);
        let view = crate::app::session_io::SavedView {
            arctic: self.display.ssao,
            wireframe: self.display.wireframe,
            show_outlines: self.display.show_outlines,
            lit: self.display.lit,
            backface: self.display.backface,
            cull_backfaces: self.display.cull_backfaces,
            opacity: self.display.opacity,
            show_grid: self.display.show_grid,
            cloud_size: self.display.cloud_size,
        };
        let bytes = crate::app::session_io::save_with_state(&self.scene, Some(cam), Some(view))?;
        let len = bytes.len();
        self.saved_bytes = Some(bytes);
        Ok(format!("Saved complete session ({len} bytes)"))
    }

    fn open_session_prompt(&mut self) -> Result<String, String> {
        self.open_prompts += 1;
        Ok("Choose a .session file".into())
    }

    fn download_report(&mut self) -> Result<String, String> {
        self.reports_requested += 1;
        Ok("Report downloaded".into())
    }

    fn saved_view(&self) -> crate::app::session_io::SavedView {
        crate::app::session_io::SavedView {
            arctic: self.display.ssao,
            wireframe: self.display.wireframe,
            show_outlines: self.display.show_outlines,
            lit: self.display.lit,
            backface: self.display.backface,
            cull_backfaces: self.display.cull_backfaces,
            opacity: self.display.opacity,
            show_grid: self.display.show_grid,
            cloud_size: self.display.cloud_size,
        }
    }

    fn install_opened_session(&mut self, opened: crate::app::session_io::OpenedSession) {
        self.clear();
        self.scene = opened.scene;
        if let Some(cam) = opened.camera {
            cam.apply_to(&mut self.camera);
        }
        if let Some(v) = opened.view {
            self.display.ssao = v.arctic;
            self.display.wireframe = v.wireframe;
            self.scene.wireframe = v.wireframe;
            self.display.show_outlines = v.show_outlines;
            self.display.lit = v.lit;
            self.display.backface = v.backface;
            self.display.cull_backfaces = v.cull_backfaces;
            self.display.opacity = v.opacity;
            self.display.show_grid = v.show_grid;
            self.display.cloud_size = v.cloud_size;
        }
        self.rows.clear();
        self.scene.drain_changes(&mut self.rows);
        self.recompute_bounds();
        self.refresh_layers();
    }

    fn clear_scene(&mut self) {
        self.clear();
    }

    fn append_file(&mut self, doc: FileDoc) {
        self.add_file(doc);
    }

    fn note_status(&mut self, message: &str) {
        self.status_line = message.into();
    }

    fn touch(&mut self) {}
}
