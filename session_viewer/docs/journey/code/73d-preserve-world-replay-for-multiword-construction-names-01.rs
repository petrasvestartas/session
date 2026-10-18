    }
    pub fn build_world(&self, points:&[[f64;3]]) -> Result<Built,&'static str> {
        let command=self.command(points);let text=&command[self.tool.name().len()..];
        let world=crate::drawing_input::points(text,crate::drawing_coords::plane(),self.frame,&[],None)?;
        self.tool.build_on_plane(&world,self.frame)
    }
    pub fn outline(&self)->Vec<[f64;3]> {
        let mut points=self.points.clone();if let Some(cursor)=self.cursor {points.push(cursor);}
        self.tool.outline(&points,self.frame)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
