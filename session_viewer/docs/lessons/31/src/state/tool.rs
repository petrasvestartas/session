// --8<-- [start:tool-start]
use super::State;
use super::drawing::Draft;
use crate::app::command::tool::{Next, Overlay, Tool};
use crate::app::coords;
use session_rust::{Plane, Point, Xform};

impl State {
    /// Run `tool` on the selection: it asks for points until it is done; the gumball hides meanwhile.
    pub(crate) fn start_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String> {
        let rows = self.selected_rows();

        if let Some(reason) = self.locked_reason(&rows) {
            return Err(reason);
        }

        // a title or text plate has no geometry to move
        if rows.iter().any(|&row| self.scene.geometry(row).is_none()) {
            return Err("This selection cannot be edited".into());
        }

        self.cancel_drawing();
        self.cancel_split(); // register:split
        let mut draft = Draft::new(tool.name(), tool.name(), self.facing());

        // whole objects follow the cursor; a face, edge or control point commits without a preview
        if crate::app::deform::Target::selected(&self.selection).is_none() {
            draft.group = rows
                .iter()
                .filter_map(|&row| Some((row, self.scene.placement_of(row)?)))
                .collect();
        }

        draft.tool = Some(tool);
        self.features.draft = Some(draft);
        self.gpu.pick.cancel();
        self.place_gizmo(None);
        Ok(self.drawing_prompt())
    }

    /// Run `tool` with no selection needed: it picks objects or clicks the screen itself.
    pub(crate) fn open_tool(&mut self, tool: Box<dyn Tool>) -> Result<String, String> {
        self.cancel_drawing();
        self.cancel_split(); // register:split
        let mut draft = Draft::new(tool.name(), tool.name(), self.facing());
        draft.tool = Some(tool);
        self.features.draft = Some(draft);
        self.gpu.pick.cancel();
        self.place_gizmo(None);
        crate::app::feedback::command_line(true);
        let answer = self
            .call_tool(|tool, state, _, _| Some(tool.begin(state)))
            .unwrap_or_else(|| Ok(self.drawing_prompt()));
        self.touch();
        answer
    }

    // `impl FnOnce(..) -> T`: any closure that is called once; `T` is whatever it returns.
    /// Call the running tool. It is taken out of the draft, and the draft out of `self`, so the tool may borrow all of State; both go back after.
    fn with_tool<T>(&mut self, call: impl FnOnce(&mut dyn Tool, &mut State) -> T) -> Option<T> {
        let mut draft = self.features.draft.take()?;
        let Some(mut tool) = draft.tool.take() else {
            self.features.draft = Some(draft);
            return None;
        };
        let answer = call(tool.as_mut(), self);
        draft.tool = Some(tool);
        self.features.draft = Some(draft);
        Some(answer)
    }
    // --8<-- [end:tool-start]

    // --8<-- [start:tool-events]
    /// True while the running tool wants an object pick.
    pub(super) fn tool_picks(&self) -> bool {
        self.features
            .draft
            .as_ref()
            .and_then(|draft| draft.tool.as_ref())
            .is_some_and(|tool| tool.picks())
    }

    /// A click for a tool that takes clicks itself; None places a point. The bool asks for a redraw.
    pub(super) fn tool_click(&mut self, x: f64, y: f64) -> Option<bool> {
        // an object pick answers later; a redraw now would cancel it
        if self.tool_picks() {
            self.request_selection(x as u32, y as u32, false, false); // the GPU answers a frame later, in tool_picked
            return Some(false);
        }

        let answer = self.call_tool(|tool, state, _, _| tool.clicked(state, (x, y)))?;
        self.status(&answer.unwrap_or_else(|error| error));
        crate::app::feedback::command_line(self.features.draft.is_some());
        Some(true)
    }

    /// The object pick the tool asked for came back.
    pub(super) fn tool_picked(&mut self, row: Option<u32>) {
        if let Some(row) = row {
            self.scene.ask(row); // a released document comes back for the next click
        }

        let answer = self.call_tool(|tool, state, _, _| Some(tool.picked(state, row)));

        if let Some(answer) = answer {
            self.status(&answer.unwrap_or_else(|error| error));
        }

        self.touch();
    }

    /// The cursor moved over a tool that follows it itself; None when it snaps points.
    pub(super) fn tool_hover(&mut self, x: f64, y: f64) -> Option<bool> {
        self.with_tool(|tool, state| tool.hovered(state, (x, y)))
            .flatten()
    }

    /// A left press the running tool takes as the start of its own drag.
    pub(crate) fn tool_press(&mut self, x: f64, y: f64) -> bool {
        self.with_tool(|tool, state| tool.pressed(state, (x, y)))
            .unwrap_or(false)
    }

    /// The pointer moved during the tool's drag; true redraws.
    pub(crate) fn tool_drag(&mut self, x: f64, y: f64) -> bool {
        self.with_tool(|tool, state| tool.dragged(state, (x, y)))
            .unwrap_or(false)
    }

    /// The tool's drag ended; Shift adds, Ctrl removes. True redraws.
    pub(crate) fn tool_release(&mut self, add: bool, remove: bool) -> bool {
        let Some(answer) =
            self.call_tool(|tool, state, _, _| Some(tool.released(state, add, remove)))
        else {
            return false;
        };
        self.status(&answer.unwrap_or_else(|error| error));
        self.touch();
        true
    }

    /// A drag the tool was following is lost; the tool keeps running.
    pub(crate) fn tool_abandon(&mut self) {
        self.with_tool(|tool, _| tool.abandoned());
    }

    /// What the running tool draws over the scene.
    pub fn tool_marks(&self) -> Option<Overlay> {
        self.features.draft.as_ref()?.tool.as_ref()?.marks(self)
    }

    /// The option button the running tool shows as chosen.
    pub fn drawing_chosen(&self) -> Option<&'static str> {
        self.features.draft.as_ref()?.tool.as_ref()?.chosen()
    }

    /// Where the cursor is in the scene while drawing.
    pub(crate) fn drawing_cursor(&self) -> Option<&Point> {
        self.features.draft.as_ref()?.hover.as_ref()
    }

    /// The running tool as JSON, null when none runs.
    pub fn tool_status(&self) -> serde_json::Value {
        self.features
            .draft
            .as_ref()
            .and_then(|draft| draft.tool.as_ref())
            .map_or(serde_json::Value::Null, |tool| tool.status())
    }

    /// The view as it maps the scene to device pixels now, for many points at once.
    pub(crate) fn view_screen(&self) -> crate::app::snap::Screen {
        self.screen()
    }

    /// Nothing is selected for `line`: clicks pick objects until Enter runs it.
    pub(crate) fn ask_for_objects(&mut self, line: &str) -> Result<String, String> {
        self.cancel_drawing();
        self.cancel_split(); // register:split
        let mut draft = Draft::new("select", "select", self.facing());
        draft.then = Some(line.to_owned());
        self.features.draft = Some(draft);
        self.place_gizmo(None);
        Ok(self.drawing_prompt())
    }
    // --8<-- [end:tool-events]

    // --8<-- [start:tool-command]
    /// A command line entry for a running tool or object pick; None when it is some other command.
    pub(super) fn tool_command(&mut self, text: &str) -> Option<Result<String, String>> {
        let draft = self.features.draft.as_ref()?;

        // picking objects: Enter runs the command on them
        if let Some(line) = draft.then.clone() {
            if !text.trim().is_empty() {
                let point = text.split_whitespace().next().and_then(coords::parse);
                return point.map(|_| Err("Select objects, then press Enter".into()));
            }

            if self.scene.selected.is_none() {
                return Some(Err(
                    "Nothing is selected; click objects, then press Enter".into()
                ));
            }

            self.cancel_drawing();
            return Some(self.run_command(&line));
        }

        draft.tool.as_ref()?;

        if text.trim().is_empty() {
            return self.call_tool(|tool, state, points, _| Some(tool.enter(state, points)));
        }

        let mut answer = None;

        for word in text.split_whitespace() {
            // a tool that finished takes no more words
            if self
                .features
                .draft
                .as_ref()
                .is_none_or(|draft| draft.tool.is_none())
            {
                break;
            }

            let taken =
                self.call_tool(|tool, state, points, plane| tool.word(state, word, points, plane));
            let step = match taken {
                Some(step) => step,
                None if coords::parse(word).is_some() => self.accept_coordinates(word),
                None if answer.is_none() => return None, // the first word is some other command
                None => Err(format!("`{word}` is not a point or a value")),
            };

            if step.is_err() {
                return Some(step);
            }

            answer = Some(step);
        }

        answer
    }

    /// The tool takes the point just placed.
    pub(super) fn tool_placed(&mut self) -> Result<String, String> {
        self.call_tool(|tool, state, points, plane| Some(tool.placed(state, points, plane)))
            .unwrap_or_else(|| Ok(self.drawing_prompt()))
    }

    /// Call the tool with the draft taken out of `self`, then carry on as it says.
    fn call_tool(
        &mut self,
        call: impl FnOnce(&mut dyn Tool, &mut State, &[Point], &Plane) -> Option<Result<Next, String>>,
    ) -> Option<Result<String, String>> {
        let mut draft = self.features.draft.take()?;
        let Some(mut tool) = draft.tool.take() else {
            self.features.draft = Some(draft);
            return None;
        };

        // the document places the rows again before anything is committed
        if draft.moved {
            self.show_group(&draft.group, None);
            draft.moved = false;
        }

        let answer = call(tool.as_mut(), self, &draft.points, &draft.frame);
        draft.tool = Some(tool);
        let Some(answer) = answer else {
            self.features.draft = Some(draft);
            self.preview_tool();
            return None;
        };
        Some(self.resume(draft, answer))
    }

    /// Put the draft back for another point, or finish.
    fn resume(&mut self, mut draft: Draft, answer: Result<Next, String>) -> Result<String, String> {
        match answer {
            Ok(Next::More) => {
                self.features.draft = Some(draft);
                self.preview_tool();
                Ok(self.drawing_prompt())
            }
            Ok(Next::Repeat(message)) => {
                draft.points.truncate(1); // Repeat keeps the base point: Copy places copy after copy from it
                draft.targets = None; // what it made offers snaps too
                self.features.draft = Some(draft);
                self.place_gizmo(None);
                Ok(format!("{message} · {}", self.drawing_prompt()))
            }
            Ok(Next::Done(message)) => {
                self.place_gizmo(self.scene.selected);
                self.update_label();
                self.touch();
                Ok(message)
            }
            Err(error) => {
                self.features.draft = Some(draft);
                self.preview_tool();
                Err(error)
            }
        }
    }
    // --8<-- [end:tool-command]

    // --8<-- [start:tool-show]
    /// The prompt of a running tool or object pick.
    pub(super) fn tool_prompt(&self) -> Option<String> {
        let draft = self.features.draft.as_ref()?;

        if let Some(line) = &draft.then {
            return Some(format!(
                "Select objects for {}, then press Enter · Esc cancels",
                crate::app::command::name_of(line)
            ));
        }

        let tool = draft.tool.as_ref()?;

        if !tool.asks_points() {
            return Some(format!(
                "{}: {} · Esc cancels",
                tool.name(),
                tool.prompt(&draft.points)
            ));
        }

        Some(format!(
            "{}: {} · click or type x,y,z · Snap {} · Esc cancels",
            tool.name(),
            tool.prompt(&draft.points),
            if self.features.snap.enabled {
                "On"
            } else {
                "Off"
            }
        ))
    }

    /// The rubber band on screen and the text beside the cursor.
    pub(super) fn tool_overlay(&self) -> Option<(Vec<(f64, f64)>, String)> {
        let draft = self.features.draft.as_ref()?;

        if draft.then.is_some() {
            return Some((Vec::new(), String::new()));
        }

        let tool = draft.tool.as_ref()?;
        let guide = tool.guide(&draft.points, draft.hover.as_ref());
        let points = guide
            .iter()
            .filter_map(|p| self.project([p[0], p[1], p[2]]))
            .collect();
        let snap = draft.snapped.map(|kind| format!("{kind:?}"));
        let readout = draft
            .hover
            .as_ref()
            .map(|cursor| tool.readout(&draft.points, cursor, &draft.frame))
            .filter(|text| !text.is_empty());
        let label = match (snap, readout) {
            (Some(snap), Some(readout)) => format!("{snap} · {readout}"),
            (snap, readout) => snap.or(readout).unwrap_or_default(),
        };
        Some((points, label))
    }

    /// Show the selection where the tool would put it for the cursor; only GPU placements change.
    pub(super) fn preview_tool(&mut self) {
        let Some(draft) = self.features.draft.as_ref() else {
            return;
        };
        let (Some(tool), Some(cursor)) = (draft.tool.as_ref(), draft.hover.as_ref()) else {
            return;
        };

        if draft.group.is_empty() {
            return;
        }

        let delta = tool.preview(&draft.points, cursor, &draft.frame);

        // nothing to show and nothing shown: no work
        if delta.is_none() && !draft.moved {
            return;
        }

        let mut draft = self.features.draft.take().unwrap();
        self.show_group(&draft.group, delta.as_ref());
        draft.moved = delta.is_some();
        self.features.draft = Some(draft);
    }

    /// Place `group` at `delta` times its placement, or back at its placement.
    fn show_group(&mut self, group: &[(u32, Xform)], delta: Option<&Xform>) {
        for (row, place) in group {
            let placed = match delta {
                Some(delta) => delta * place, // the tool's move applied after the object's own placement
                None => place.clone(),
            };
            self.gpu.objects.set_placement(&self.gpu.ctx, *row, &placed);

            self.gpu.grew_bounds(*row);
        }

        self.update_label();
        self.touch();
    }

    /// Esc or another command: drop the draft; a previewed selection goes back, the selection stays.
    pub(crate) fn cancel_drawing(&mut self) {
        let Some(mut draft) = self.features.draft.take() else {
            return;
        };
        self.additive_selection = false;

        if let Some(tool) = draft.tool.as_mut() {
            tool.cancel(self);
        }

        if draft.moved {
            self.show_group(&draft.group, None);
        }

        if draft.tool.is_some() || draft.then.is_some() {
            self.place_gizmo(self.scene.selected);
            self.update_label();
            self.touch();
        }
    }

    /// True while a tool or an object pick runs: the gumball stays hidden.
    pub(crate) fn tool_running(&self) -> bool {
        self.features
            .draft
            .as_ref()
            .is_some_and(|draft| draft.tool.is_some() || draft.then.is_some())
    }
}
// --8<-- [end:tool-show]

// --8<-- [start:tool-hooks]
// A second `impl State` block: these are the hooks other files call through lines tagged `register:`.
impl State {
    /// Cancel a tool that asks for points.
    pub(super) fn cancel_running_tool(&mut self) {
        if self.tool_running() {
            self.cancel_drawing();
        }
    }

    /// Cancel the command being typed or drawn; false when there is none.
    pub(super) fn cancel_draft(&mut self) -> bool {
        if self.features.draft.is_none() {
            return false;
        }

        self.cancel_drawing();
        true
    }

    /// Enter: run the command being drawn, or finish a split.
    pub fn enter(&mut self) {
        if self.features.draft.is_some() {
            let result = self.run_command("");
            crate::app::feedback::status(&result.unwrap_or_else(|e| e));
        } else {
            self.confirm_split(); // register:split
        }
    }

    /// A tool asked for this object: it takes the pick.
    pub(super) fn take_tool_pick(&mut self, pick: Option<crate::engine::gpu::Pick>) -> bool {
        if !self.tool_picks() {
            return false;
        }

        self.tool_picked(pick.map(|pick| pick.row));
        true
    }
}
// --8<-- [end:tool-hooks]
