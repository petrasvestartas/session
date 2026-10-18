    pub fn shortcut(&self, event: &web_sys::KeyboardEvent) -> Option<crate::shortcuts::Shortcut> {
        if self.number.number_prompt.is_some() || event.type_() != "keydown" || event.repeat() { return None; }
        crate::shortcuts::context_key(&self.context,&self.model.command,&event.key(),event.ctrl_key()||event.meta_key(),event.shift_key(),event.alt_key(),event.is_composing())
    }

