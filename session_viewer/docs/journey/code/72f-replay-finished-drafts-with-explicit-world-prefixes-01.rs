        prompt
    }
    pub fn submit(&mut self, point: [f64; 3]) -> Result<Option<Built>, &'static str> { self.submit_many(&[point]) }
    pub fn submit_many(&mut self, added: &[[f64;3]]) -> Result<Option<Built>, &'static str> {
        let mut points=self.points.clone();
        for &point in added { points.push(crate::tool_coords::checked(point)?); }
        let count=self.tool.count(); let limit=if count==0 {MAX_POINTS} else {count};
        if points.len()>limit {return Err("The tool has enough points");}
        let built=if count>0 && points.len()==count {Some(self.build_world(&points)?)} else {None};
        self.points=points;self.cursor=None;self.snapped=None;Ok(built)
    }
    pub fn finish(&self) -> Result<Built,&'static str> { self.build_world(&self.points) }
    // Finished coordinates already belong to the world; their prefix prevents applying a fixed plane a second time during replay.
    pub fn command(&self, points:&[[f64;3]]) -> String {
        let mut line=self.tool.name().to_owned();
        for p in points {line.push_str(&format!(" w{},{},{}",p[0],p[1],p[2]));}line
    }
    pub fn build_world(&self, points:&[[f64;3]]) -> Result<Built,&'static str> {
        let command=self.command(points);let text=command.split_once(' ').map_or("",|(_,text)|text);
        let world=crate::drawing_input::points(text,crate::drawing_coords::plane(),self.frame,&[],None)?;
        self.tool.build(&world)
    }
    pub fn hover(&mut self, point: Option<[f64; 3]>) -> Result<(), &'static str> {
