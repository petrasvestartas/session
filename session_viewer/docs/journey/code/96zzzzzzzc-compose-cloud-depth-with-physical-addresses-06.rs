e);
    pass.set_bind_group(0, &t.group, &[]);
    pass.set_bind_group(1, &self.window_group, &[]);
    pass.set_bind_group(2,&self.settings_group,&[]);
    pass