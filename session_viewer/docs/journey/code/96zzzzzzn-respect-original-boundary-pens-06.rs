    pub fn pen(&self,index:usize)->(f32,[f32;4]) {
        let &(width,colour)=self.pens.get(if self.pens.len()==1 {0} else {index}).unwrap_or(&(1.,crate::source_topology_colour::BLACK));
        (width,colour.to_le_bytes().map(|v|v as f32/255.))
    }

    pub fn bytes(&self) -> usize {