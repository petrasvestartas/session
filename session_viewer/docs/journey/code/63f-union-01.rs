    }

    pub fn selected(&self)->bool {u32::from_ne_bytes(self.previous[64..68].try_into().unwrap())!=0}

    pub fn settings_bytes(&self) -> u64 { self.model.size() }

