        let row=scene.paths().iter().find(|row|row.id==id)?;Some(match &row.prepared.source{crate::chain::Source::Line(source)=>Geometry::Line(source.clone()),crate::chain::Source::Polyline(source)=>Geometry::Polyline(source.clone()),crate::chain::Source::Curve(source,_)=>Geometry::NurbsCurve(source.clone())})
    }
    pub fn valid(&self,scene:&Scene)->Result<(),String>{
        for(id,source,place)in &self.originals{if !self.progress.picked().contains(&id.value()){continue}let current=Self::source(scene,*id).ok_or("Picked curve was removed")?;let same=match(source,current){(Geometry::Line(a),Geometry::Line(b))=>std::rc::Rc::ptr_eq(a,&b),(Geometry::Polyline(a),Geometry::Polyline(b))=>std::rc::Rc::ptr_eq(a,&b),(Geometry::NurbsCurve(a),Geometry::NurbsCurve(b))=>std::rc::Rc::ptr_eq(a,&b),_=>false};if !same||scene.placement(*id).is_none_or(|p|p.m!=place.m)||scene.pick_id(id.value())!=Some(*id){return Err("Picked curve changed; start again".into())}}Ok(())
    }
    pub fn input(&self)->Result<Input,String>{
        let curves=self.progress.rows.iter().map(|rows|rows.iter().map(|row|self.curves.iter().find(|(id,_)|id.value()==*row).map(|(_,curve)|curve.clone()).ok_or_else(||"Picked source is missing".into())).collect::<Result<Vec<_>,String>>()).collect::<Result<Vec<_>,_>>()?;
        let(x,y)=self.frame.axes();Ok(Input{curves,answers:self.answers[..self.progress.answered.min(self.answers.len())].to_vec(),choice:self.recipe.choices().nth(self.choice).unwrap_or(""),normal:x.cross(&y)})
    }
    pub fn build(&self)->Result<Option<Made>,String>{if self.progress.done(self.recipe.steps){(self.recipe.build)(&self.input()?).map(Some)}else{Ok(None)}}
    pub fn prompt(&self)->String{let step=self.current().map_or("complete",Step::prompt);format!("{} · {step} · Esc cancels",self.recipe.name)}
    pub fn axis(&self)->Result<(Point,Vector),String>{Ok((self.recipe.axis)(&self.input()?))}
}
