        format!("{} · {}{default} · Esc cancels",self.shape.name,ask.prompt)
    }
    pub fn set_plane(&mut self,plane:CPlane)->Result<(),String> {
        let (x,y)=plane.axes();let frame=frame_of(&Plane::new(plane.origin(),x,y),&self.answers);
        let part=(self.shape.read)(&frame,&self.answers,self.choice())?;self.plane=plane;self.part=part;Ok(())
    }
    fn with(&self,answer:Answer)->Result<(Vec<Answer>,Part),String> {
        match &answer {
            Answer::Point(p)=>{crate::tool_coords::checked([p[0],p[1],p[2]]).map_err(str::to_owned)?;},
            Answer::Number(value) if !value.is_finite() || value.abs()>1e12=>return Err("Shape sizes must be finite and within ±1e12".into()),
            _=>{}
        }
        if self.next().is_none() {return Err("The shape already has all its answers".into());}
        let mut answers=self.answers.clone();answers.push(answer);
        if answers.len()>crate::tool::MAX_POINTS {return Err("Too many shape answers".into());}
        let (x,y)=self.plane.axes();let frame=frame_of(&Plane::new(self.plane.origin(),x,y),&answers);
        let part=(self.shape.read)(&frame,&answers,self.choice())?;Ok((answers,part))
    }
}
