
    pub fn result(&mut self, message: &str) {
        self.model.status = message.into();
        if self.number.number_prompt.is_some() {self.number.number_error=message.into();}
        if let Some(last) = self.model.history.back_mut()
            && let Some(at) = last.find('\n')
        {
