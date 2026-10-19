    }
    pub fn outlines(&self)->Vec<Vec<[f64;3]>> {
        if let Some(gather)=&self.gather {gather.outlines(self.cursor)}else if let Some(primitive)=&self.primitive {primitive.outline(self.cursor)}else{vec![self.outline()]}
    }
    pub fn outline(&self)->Vec<[f64;3]> {
