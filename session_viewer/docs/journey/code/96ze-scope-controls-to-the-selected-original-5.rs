            Action::EnableControls => {
                let parent=self.selected.ok_or("Select one object before Controls")?;
                if self.pick_mode==crate::pick_sources::Mode::Control && (self.scene.control_markers().iter().any(|m|m.parent==parent) || self.scene.clouds().iter().any(|r|r.id==parent&&r.controls_visible) || self.scene.streams().iter().any(|r|r.id==parent&&r.controls_visible)) {return Ok(Change::View);}
                if self.selected_objects().len()!=1 {return Err("Select one object before Controls");}
                self.scene.owner_controls(parent)?;
                self.pick_mode=crate::pick_sources::Mode::Control;
                self.objects.clear();self.selected_part=None;
            },
            Action::Controls(visible) => {