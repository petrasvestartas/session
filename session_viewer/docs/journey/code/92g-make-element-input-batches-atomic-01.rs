 pub fn valid(&self,scene:&Scene)->Result<(),String>{for o in &self.picked{o.valid(scene).map_err(str::to_owned)?;}Ok(())}
 pub fn build(&self,scene:&Scene)->Result<Vec<Made>,String>{self.valid(scene)?;crate::element_input::build(self.kind,&self.params,&self.picked.iter().map(|o|(o.source.clone(),o.model.clone())).collect::<Vec<_>>())}
 pub fn text(&mut self,text:&str)->Result<bool,String>{let mut next=self.clone();if text.trim().is_empty(){if next.asking.take().is_none(){return Ok(true)}}else{for word in text.split_whitespace(){next.take(word)?;}}*self=next;Ok(false)}
 pub fn values(&self)->String{self.params.iter().map(|p|format!("{} {}",p.name,p.value)).collect::<Vec<_>>().join(" · ")}
 pub fn prompt(&self)->String{if let Some(i)=self.asking{format!("{} <{}>: type a number · Enter keeps it",self.params[i].name,self.params[i].value)}else{format!("{} · pick original sources ({} picked) · Enter creates · Esc cancels",self.kind.name(),self.picked.len())}}
