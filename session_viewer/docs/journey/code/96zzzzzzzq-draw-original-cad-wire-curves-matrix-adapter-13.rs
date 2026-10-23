    }

    pub fn empty()->Self {
        Self {control_owners:Default::default(),groups:Default::default(),graphs:Default::default(),cad_walks:[0;2],instances:Default::default(),created:None,ids:Default::default(),view:Default::default(),forest:Default::default(),matrices:Vec::new(),sheets:Vec::new(),streamed:Vec::new(),clouds:Vec::new(),texts:Vec::new(),objects:Vec::new(),points:Vec::new(),paths:Vec::new(),lines:Vec::new(),next_id:1,extra:None}
    }

    pub fn demo() -> Self {
