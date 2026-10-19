
    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        if !matches!(&action,Action::Zoom(_)|Action::ZoomAt{..}|Action::PanScreen{..}|Action::Pan(_,_)|Action::Orbit(_,_)|Action::Fit|Action::FitSelected|Action::Projection(_)|Action::ResetView|Action::Isometric|Action::Background|Action::NormalView(_)|Action::Grid(_)){self.mark=None;}
        match action {
            Action::StreamRefresh=>{self.snap_index=None;return Ok(Change::Scene);},
