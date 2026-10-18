        };format!("{} · {question} · Esc cancels{}",self.kind.name(),if self.kind==Kind::Scale{format!(" · {}D",self.mode)}else{String::new()})
    }
    pub fn accept(&mut self,answer:Answer)->Result<Option<Xform>,String> {
        match answer {
            Answer::Number(value)=>{
                if !value.is_finite()||value.abs()>1e12{return Err("Transform value must be finite and within ±1e12".into());}
                let Some(&origin)=self.points.first() else{return Err("Pick the origin first".into())};let origin=Self::p(origin);let plane=self.plane();
                match self.kind {
                    Kind::Rotate=>Ok(Some(crate::transform_rotate::rotation(&origin,value.to_radians(),&plane))),
                    Kind::Scale=>{
                        if value<=0. {return Err("Scale wants a value above zero".into());}
                        let (from,factor)=match self.points.as_slice(){[_] if self.mode==1=>return Err("Scale 1D needs a reference direction".into()),[_]=>(origin.clone(),value),[_,from]=>{let from=Self::p(*from);let base=crate::transform_scale::reach(self.mode,&origin,&from,&from,&plane).filter(|v|*v>1e-12).ok_or("Reference point is on the origin")?;(from,value/base)},_=>return Err("Too many reference points".into())};Ok(Some(crate::transform_scale::scaling(self.mode,&origin,&from,factor,&plane)))
                    },_=>Err("Supply a target point or a distance along the cursor band".into())
                }
            },
            Answer::Point(point)=>{
                let point=crate::tool_coords::checked([point[0],point[1],point[2]]).map_err(str::to_owned)?;let mut points=self.points.clone();points.push(point);let plane=self.plane();
                let made=match(self.kind,points.as_slice()){
                    (Kind::Copy|Kind::Move,[from,to])=>Some(Xform::translation(to[0]-from[0],to[1]-from[1],to[2]-from[2])),
                    (Kind::Scale,[origin,from])=>{let(o,f)=(Self::p(*origin),Self::p(*from));if crate::transform_scale::reach(self.mode,&o,&f,&f,&plane).is_none_or(|v|v<=1e-12){return Err("Reference point is on the origin".into())}None},
                    (Kind::Scale,[origin,from,to])=>{let(o,f,t)=(Self::p(*origin),Self::p(*from),Self::p(*to));let factor=crate::transform_scale::factor(self.mode,&o,&f,&t,&plane).ok_or("Scale factor must be above zero")?;Some(crate::transform_scale::scaling(self.mode,&o,&f,factor,&plane))},
                    (Kind::Rotate,[origin,from])=>{let(o,f)=(Self::p(*origin),Self::p(*from));crate::transform_rotate::angle(&o,&f,&f,&plane).ok_or("Reference point is on the center")?;None},
                    (Kind::Rotate,[origin,from,to])=>{let(o,f,t)=(Self::p(*origin),Self::p(*from),Self::p(*to));let angle=crate::transform_rotate::angle(&o,&f,&t,&plane).ok_or("Second point is on the center")?;Some(crate::transform_rotate::rotation(&o,angle,&plane))},
                    (_,[_])=>None,_=>return Err("The transform already has its points".into()),
                };self.points=if made.is_some()&&self.kind==Kind::Copy{vec![points[0]]}else{points};Ok(made)
            }
        }
    }
}
