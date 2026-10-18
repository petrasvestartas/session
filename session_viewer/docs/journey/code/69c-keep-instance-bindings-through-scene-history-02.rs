
    pub fn empty()->Self {
        Self {instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

