use super::State;
use crate::app::{feedback, splitting};

/// A split waiting for its cutters.
pub(super) struct Pending {
    pub target: u32,         // the row being split
    pub face: Option<usize>, // the face of it, for a BRep
    pub cutters: Vec<u32>,   // the rows chosen as cutters
}

impl State {
    /// The pending split, for the inspection tests.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn split_status(&self) -> Option<(u32, Option<usize>, &[u32])> {
        self.pending_split
            .as_ref()
            .map(|p| (p.target, p.face, p.cutters.as_slice()))
    }

    /// Split: start choosing cutters, or finish when they are chosen.
    pub(crate) fn split_command(&mut self) -> Result<String, String> {
        // Split again finishes
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

        // a selected face, else the whole curve
        let selected = match self.selection {
            crate::app::selection::SelectionMode::Face { face, .. } => Some(face),
            _ => None,
        };
        let face = splitting::face_index(
            self.scene.geometry(row).ok_or("Source unavailable")?,
            selected,
        )?;
        self.pending_split = Some(Pending {
            target: row,
            face,
            cutters: vec![],
        });
        self.place_gizmo(None);
        feedback::command_line(false);
        feedback::focus_canvas();
        Ok("Split: select cutter lines, polylines or curves, then press Enter or tap Split again. Esc cancels. Faces require on-surface cutters.".into())
    }

    /// Drop the pending split and its cutter highlights.
    pub(crate) fn cancel_split(&mut self) {
        if let Some(pending) = self.pending_split.take() {
            for row in pending.cutters {
                self.gpu.set_selected(row, false);
            }
        }
    }

    /// Add a clicked row to the cutters, or remove it again.
    pub(super) fn pick_split_cutter(&mut self, row: u32) {
        let Some(pending) = self.pending_split.as_mut() else {
            return;
        };

        // a released curve comes back for the next click
        self.scene.ask(row);

        // only an unlocked curve, not the target
        if row == pending.target
            || !self.scene.selectable(row)
            || !self.scene.geometry(row).is_some_and(splitting::is_cutter)
        {
            self.status(
                "Choose an unlocked line, polyline or NURBS curve distinct from the target",
            );
            return;
        }

        if let Some(at) = pending.cutters.iter().position(|item| *item == row) {
            pending.cutters.remove(at);
            self.gpu.set_selected(row, false);
        } else if pending.cutters.len() < 64 {
            pending.cutters.push(row);
            self.gpu.set_selected(row, true);
        }

        let count = pending.cutters.len();
        self.status(&format!(
            "Split: {count} cutter curves selected. Enter or Split confirms; Esc cancels."
        ));
        self.touch();
    }

    /// Enter: finish the split.
    pub fn confirm_split(&mut self) {
        if self.pending_split.is_some() {
            let message = self.finish_split().unwrap_or_else(|error| error);
            self.status(&message);
            self.touch();
        }
    }

    /// Run the split with the chosen cutters.
    fn finish_split(&mut self) -> Result<String, String> {
        let pending = self.pending_split.take().ok_or("Start Split first")?;

        if pending.cutters.is_empty() {
            self.pending_split = Some(pending);
            return Err("Select at least one cutter curve, then press Enter".into());
        }

        for &row in &pending.cutters {
            self.gpu.set_selected(row, false);
        }

        let identity = self.scene.identity_of(pending.target); // to select it again after the sync
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
                self.place_gizmo(self.scene.selected);
                Ok("No division: cutters must cross the curve or lie on the selected face.".into())
            }
            Err(error) => {
                self.place_gizmo(self.scene.selected);
                Err(error)
            }
        }
    }
}
