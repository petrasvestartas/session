    #[cfg(test)]
    pub(crate) fn colour_ready(&self)->bool { self.color.active.0.0.value.get().is_some() }

    pub fn clear_bindings(&mut self) { self.bindings.get_mut().take(); }
