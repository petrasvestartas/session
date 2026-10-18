        }
    }
    pub fn text(&mut self,text:&str,cursor:Option<[f64;3]>,fixed:Option<CPlane>)->Result<Outcome,String> {
        let mut next=self.clone();let mut result=Outcome{transforms:Vec::new(),finished:false};let words:Vec<_>=text.split_whitespace().collect();
        if words.is_empty(){if self.kind!=Kind::Copy{return Err("The transform needs its next answer".into())}result.finished=true;return Ok(result)}
        for word in words {
            if result.finished{return Err("Extra input after the completed transform".into())}
            if next.kind==Kind::Scale {if let Some(mode)=["1D","2D","3D"].iter().position(|mode|word.eq_ignore_ascii_case(mode)){next.mode=mode as u8+1;continue;}}
            let answer=if let Ok(value)=word.parse::<f64>() {if matches!(next.kind,Kind::Copy|Kind::Move){let points=crate::drawing_input::points(word,fixed,next.frame,&next.points,cursor).map_err(str::to_owned)?;Answer::Point(Self::p(points[0]))}else{Answer::Number(value)}}else{let points=crate::drawing_input::points(word,fixed,next.frame,&next.points,cursor).map_err(str::to_owned)?;Answer::Point(Self::p(points[0]))};
            if let Some(transform)=next.accept(answer)?{result.transforms.push(transform);result.finished=next.kind!=Kind::Copy;}
        }*self=next;Ok(result)
    }
    pub fn preview(&self,cursor:Option<[f64;3]>)->Option<Xform> {if self.kind==Kind::Copy{return None}let point=cursor?;let mut draft=self.clone();draft.accept(Answer::Point(Self::p(point))).ok().flatten()}
    pub fn buttons(&self)->Vec<(String,String)> {let mut b=vec![("Cancel".into(),"Escape".into())];if self.kind==Kind::Copy{b.insert(0,("Finish".into(),String::new()));}if self.kind==Kind::Scale{for mode in ["1D","2D","3D"]{b.push((mode.into(),mode.into()));}}b}
}
