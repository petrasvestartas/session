use super::State;
use crate::app::{feedback, splitting};

pub(super) struct Pending {
    pub target: u32,
    pub face: Option<usize>,
    pub cutters: Vec<u32>,
}

impl State {
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn split_status(&self) -> Option<(u32, Option<usize>, &[u32])> {
        self.pending_split
            .as_ref()
            .map(|p| (p.target, p.face, p.cutters.as_slice()))
    }

    pub(super) fn split_command(&mut self) -> Result<String, String> {
        if self.pending_split.is_some() {
            return self.finish_split();
        }

        let row = self
            .scene
            .selected
            .ok_or("Select a curve or Ctrl+Shift-select a face, then run Split")?;
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

    pub(super) fn cancel_split(&mut self) {
        if let Some(pending) = self.pending_split.take() {
            for row in pending.cutters {
                self.gpu.set_selected(row, false);
            }
        }
    }

    pub(super) fn pick_split_cutter(&mut self, row: u32) {
        let Some(pending) = self.pending_split.as_mut() else {
            return;
        };

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

    pub fn confirm_split(&mut self) {
        if self.pending_split.is_some() {
            let message = self.finish_split().unwrap_or_else(|error| error);
            self.status(&message);
            self.touch();
        }
    }

    fn finish_split(&mut self) -> Result<String, String> {
        let pending = self.pending_split.take().ok_or("Start Split first")?;

        if pending.cutters.is_empty() {
            self.pending_split = Some(pending);
            return Err("Select at least one cutter curve, then press Enter".into());
        }

        for &row in &pending.cutters {
            self.gpu.set_selected(row, false);
        }

        let identity = self.scene.identity_of(pending.target);
        let result = self
            .scene
            .split_rows(pending.target, pending.face, &pending.cutters);

        match result {
            Ok(regions) if regions > 1 => {
                self.after_history();
                let row = identity.and_then(|id| {
                    (0..self.scene.object_count() as u32)
                        .find(|&row| self.scene.identity_of(row).as_ref() == Some(&id))
                });
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
