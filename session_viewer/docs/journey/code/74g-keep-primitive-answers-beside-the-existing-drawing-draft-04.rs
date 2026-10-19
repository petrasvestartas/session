        self.tool.build_on_plane(&world,self.frame)
    }
    pub fn outlines(&self)->Vec<Vec<[f64;3]>> {
        if let Some(primitive)=&self.primitive {primitive.outline(self.cursor)}else{vec![self.outline()]}
    }
    pub fn outline(&self)->Vec<[f64;3]> {
        let mut points=self.points.clone();if let Some(cursor)=self.cursor {points.push(cursor);}
