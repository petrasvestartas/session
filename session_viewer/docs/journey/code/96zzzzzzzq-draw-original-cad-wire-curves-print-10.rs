    pub fn printed(&self)->bool{self.geometry.source.print_fill()}
    pub fn closed(&self)->bool{!self.printed()&&!self.xray&&self.cap_flags&crate::solid_orientation::CLOSED!=0}
