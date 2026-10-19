        self.answers=answers;self.part=part;Ok(geometry)
    }
    pub fn text(&mut self,text:&str,cursor:Option<[f64;3]>,fixed:Option<CPlane>)->Result<Option<Geometry>,String> {
        // A cloned answer set makes a multiword input atomic, including its final build and any extra invalid words.
        let mut candidate=self.clone();let mut geometry=None;
        let words:Vec<_>=text.split_whitespace().collect();
        if words.is_empty() {
            let value=candidate.next().and_then(|ask|ask.default).ok_or_else(||format!("{} needs the {}",candidate.shape.name,candidate.next().map_or("next answer",|ask|ask.prompt)))?;
            geometry=candidate.accept(Answer::Number(value))?;
        }else{
            let mut index=0;
            while index<words.len() {
                if geometry.is_some() {return Err("The shape already has all its answers".into());}
                if let Some((option,count))=candidate.shape.option_in(&words[index..]) {
                    if candidate.shape.upfront && !candidate.answers.is_empty() && candidate.option!=option {return Err("Choose this option before the first point".into());}
                    candidate.option=option;candidate.set_plane(candidate.plane)?;index+=count;continue;
                }
                let ask=candidate.next().ok_or("The shape already has all its answers")?;
                let answer=match (ask.kind,words[index].parse::<f64>().ok()) {
                    (Kind::Size|Kind::Height,Some(value))=>Answer::Number(value),
                    _=>{
                        let previous=candidate.answers.iter().filter_map(|answer|match answer {Answer::Point(p)=>Some([p[0],p[1],p[2]]),_=>None}).collect::<Vec<_>>();
                        let points=crate::drawing_input::points(words[index],fixed,candidate.plane,&previous,cursor).map_err(str::to_owned)?;
                        let p=points[0];Answer::Point(Point::new(p[0],p[1],p[2]))
                    }
                };
                geometry=candidate.accept(answer)?;index+=1;
            }
        }
        *self=candidate;Ok(geometry)
    }
}
