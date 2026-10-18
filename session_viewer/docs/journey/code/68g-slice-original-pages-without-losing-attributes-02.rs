        } else {rows.clone().collect()};
        if ids.iter().copied().collect::<std::collections::HashSet<_>>().len()!=count {return Err("Repeated original point IDs");}
        let display=coordinates.iter().map(|v|*v as f32).collect();Ok(Self {rows,coordinates,display,ids,colors:Vec::new(),normals:Vec::new()})
    }
    pub fn slice(&self,rows:Range<u32>)->Result<Self,&'static str> {
        if rows.start<self.rows.start||rows.end>self.rows.end||rows.start>=rows.end {return Err("Slice is outside its original page");}
        let a=(rows.start-self.rows.start) as usize;let b=(rows.end-self.rows.start) as usize;
        Ok(Self {rows,coordinates:self.coordinates[a*3..b*3].to_vec(),display:self.display[a*3..b*3].to_vec(),ids:self.ids[a..b].to_vec(),colors:if self.colors.is_empty() {Vec::new()} else {self.colors[a..b].to_vec()},normals:if self.normals.is_empty() {Vec::new()} else {self.normals[a..b].to_vec()}})
    }
    pub fn attributes(mut self,colors:Vec<u32>,normals:Vec<u32>)->Result<Self,&'static str> {
        let count=(self.rows.end-self.rows.start) as usize;
        if !colors.is_empty()&&colors.len()!=count||!normals.is_empty()&&normals.len()!=count {return Err("Source page attributes do not match its point rows");}
        self.colors=colors;self.normals=normals;Ok(self)
    }
    pub fn bytes(&self)->usize {self.coordinates.capacity()*8+self.display.capacity()*4+self.ids.capacity()*4+self.colors.capacity()*4+self.normals.capacity()*4}
}

