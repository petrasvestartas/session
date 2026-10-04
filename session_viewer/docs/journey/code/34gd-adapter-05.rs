    pub fn failure(&self) -> Option<&Event> { self.failure.as_ref() }

    pub fn set_adapter(&mut self, info: crate::adapter_info::AdapterInfo) -> Result<(), &'static str> {
        if !info.valid() { return Err("Invalid adapter identity"); }
        self.adapter = Some(info); Ok(())
    }