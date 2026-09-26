use crate::State;
use crate::app::scene::Hydrated;

/// What waits for released documents to come back.
pub(crate) enum Resume {
    Controls(u32), // F10 on this row
    Rewalk,        // walk the lanes again: a compaction or a features toggle
}

impl State {
    /// Fetch the released documents edits asked for.
    pub(crate) fn fetch_wanted(&mut self) {
        for (doc, url, token) in self.scene.take_wanted() {
            #[cfg(target_arch = "wasm32")]
            crate::app::loader::spawn_hydrate(doc, url, token);
            #[cfg(not(target_arch = "wasm32"))]
            let _ = (doc, url, token);
        }
    }

    /// Run `resume` once the documents it needs are back.
    pub(crate) fn resume_after(&mut self, resume: Resume) {
        self.features.resume.push(resume);
        self.fetch_wanted();
    }

    /// Why `rows` cannot be edited now: a streamed shell never, a released document once it is back.
    pub(crate) fn locked_reason(&mut self, rows: &[u32]) -> Option<String> {
        let mut reason = None;

        for &row in rows {
            if let Some(doc) = self.scene.released_doc(row) {
                reason = self.scene.editable(doc).err();
            } else if self.scene.display_only(row) {
                return Some(crate::app::scene::READ_ONLY.into());
            }
        }

        if reason.is_some() {
            self.fetch_wanted();
        }

        reason
    }

    /// A released document's objects came back, or could not.
    pub fn hydrated(&mut self, back: Hydrated) {
        // a fetch for a replaced scene or an older release is dropped quietly
        if !self.scene.awaits(back.doc, back.token) {
            return;
        }

        let name = self.scene.docs[back.doc].name.clone();
        let result = back
            .session
            .and_then(|session| self.scene.hydrate(back.doc, back.token, session));

        match result {
            Ok(()) => self.status(&format!("'{name}' is editable ({:.0} ms)", back.ms)),
            Err(error) => {
                self.scene.fetch_failed(back.doc, back.token);
                self.features.resume.clear();
                self.status(&format!("Cannot edit '{name}': {error}"));
                return;
            }
        }

        let waiting = std::mem::take(&mut self.features.resume);

        for resume in waiting {
            self.run_resume(resume);
        }

        self.touch();
    }

    /// Idle work between edits: one kernel purge step, frames kept coming until the cycle ends.
    pub(crate) fn purge_idle(&mut self) {
        let gesture = self.features.dragging.is_some() || self.features.control_drag.is_some();

        if !gesture && self.scene.purge_step() {
            self.needs_frame = true;
        }
    }

    /// Run one waiting edit when its documents are back, else keep it.
    fn run_resume(&mut self, resume: Resume) {
        match resume {
            Resume::Controls(row) if self.scene.released_doc(row).is_none() => {
                if self.scene.selected == Some(row) {
                    self.enable_controls();
                }
            }
            Resume::Rewalk if !self.scene.has_released() => {
                self.scene.rewalk_editable(&mut self.gpu);
                self.reselect_face();
                self.place_gizmo(None);
                self.update_label();
            }
            waiting => self.features.resume.push(waiting),
        }
    }
}

impl State {
    /// F10 on a released document: fetch it and show the controls once it is back; true while it comes.
    pub(super) fn controls_after_hydrate(&mut self, parent: u32) -> bool {
        let Some(doc) = self.scene.released_doc(parent) else {
            return false;
        };
        self.scene.want(doc);
        self.status(&format!(
            "Loading '{}' for its control points",
            self.scene.docs[doc].name
        ));
        self.resume_after(Resume::Controls(parent));
        true
    }

    /// Snapping asked for a released document: fetch it.
    pub(super) fn fetch_if_wanting(&mut self) {
        if self.scene.wanting() {
            self.fetch_wanted();
        }
    }
}
