    pub fn show_controls(&mut self, visible: bool) {
        let owners = Rc::make_mut(&mut self.control_owners);
        owners.clear();
        if visible { owners.extend(self.objects.iter().map(|row| row.id)); }
