    fn fits(&self) -> bool {
        serde_json::to_vec_pretty(self).is_ok_and(|bytes| bytes.len() <= crate::report_store::MAX_BYTES)
    }

    fn trim_phases(&mut self) {
        // JSON escapes can expand one character into six bytes; trim the oldest measurements against the encoded budget, not character counts.
        while self.phases.len() > 256 || (!self.phases.is_empty() && !self.fits()) {
            self.phases.pop_front(); self.phases_dropped = self.phases_dropped.saturating_add(1);
        }
    }

    pub fn phase(&mut self, time: String, phase: crate::load_phase::Phase) -> Result<(), &'static str> {
        if time.is_empty() || time.chars().count() > 64 || !phase.valid() { return Err("Invalid load phase"); }
        let message = serde_json::to_string(&phase).map_err(|_| "Cannot encode phase")?;
        self.record(time, phase.elapsed_ms, "phase", &message)?;
        self.phases.push_back(phase); self.trim_phases(); Ok(())
    }

    pub fn record(&mut self, time: String, elapsed_ms: f64, kind: &str, message: &str) -> Result<(), &'static str> {