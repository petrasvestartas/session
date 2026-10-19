    }
    pub fn outlines(&self)->Vec<Vec<[f64;3]>> {
        if let Some(points)=&self.plane_points{let mut points=points.clone();if let Some(cursor)=self.cursor{points.push(cursor)}return vec![points]}
        if let Some(extend)=&self.extend{return extend.hover.as_ref().map_or_else(Vec::new,|h|vec![h.tail.clone()])}
        if let Some(gather)=&self.gather {gather.outlines(self.cursor)}else if let Some(primitive)=&self.primitive {primitive.outline(self.cursor)}else{vec![self.outline()]}
    }
