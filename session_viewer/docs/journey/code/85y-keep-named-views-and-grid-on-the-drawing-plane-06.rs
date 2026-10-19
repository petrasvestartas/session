    }

    pub fn reach(self,target:[f64;3])->Result<f64,&'static str>{self.reach_on(crate::cplane::CPlane::Xy,target)}
    pub fn reach_on(self,plane:crate::cplane::CPlane,target:[f64;3])->Result<f64,&'static str>{
        if target.iter().any(|v|!v.is_finite()){return Err("Grid target must be finite")}
        let reach=self.points()?.into_iter().map(|p|{let p=plane.world(p);(p[0]-target[0]).hypot(p[1]-target[1]).hypot(p[2]-target[2])}).fold(0.,f64::max);
        if !reach.is_finite(){return Err("Grid target exceeds the depth range")}Ok(reach)
    }
    pub fn strokes_on(self,plane:crate::cplane::CPlane)->Result<Vec<Stroke>,&'static str>{
        let mut strokes=self.strokes()?;
        for stroke in &mut strokes{for p in [&mut stroke.start,&mut stroke.end]{let world=plane.world(p.map(f64::from));let values=[world[0]as f32,world[1]as f32,world[2]as f32];if values.iter().any(|v|!v.is_finite()){return Err("Grid plane exceeds display range")}*p=values;}}
        Ok(strokes)
    }

