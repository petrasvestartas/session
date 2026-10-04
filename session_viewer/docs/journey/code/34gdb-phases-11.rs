    pub fn refresh_context(&mut self, context: Context) {
        self.context = context; self.trim_phases();
    }

    pub fn phase(&mut self, time: String, phase: crate::load_phase::Phase) -> Result<(), &'static str> {