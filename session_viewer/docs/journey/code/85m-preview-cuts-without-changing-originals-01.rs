        for(i,cut)in self.cuts.iter().enumerate(){let back=cut.owner.model.inverse()?;let local=unit_ray(&ray.origin,&ray.direction,&back);for(part,mesh)in cut.parts.hit_meshes(){if cut.removed.contains(&part){continue}if let Some(hits)=intersection::ray_mesh(&local,mesh,1e-12,false){if let Some(hit)=hits.first(){let distance=hit.transformed(&cut.owner.model).distance(&ray.origin,None);if best.is_none_or(|(near,_)|distance<near){best=Some((distance,(i,part)))}}}}}best.map(|(_,hit)|hit)
    }
    pub fn remove(&mut self,cut:usize,part:usize)->Result<bool,String>{
        let cut=self.cuts.get_mut(cut).ok_or("Trim target is gone")?;if part>=cut.parts.count()||cut.removed.contains(&part){return Err("Click a remaining trim part".into())}if cut.parts.count()-cut.removed.len()<2{return Err("The last part stays; use Delete".into())}cut.removed.push(part);self.hover=None;Ok(!self.cuts.iter().any(|c|c.parts.count()-c.removed.len()>1))
    }
    pub fn refresh_preview(&mut self,scene:&Scene)->Result<(),String>{
        let mut preview=scene.clone();for cut in &self.cuts{cut.owner.valid(scene)?;if !cut.removed.is_empty(){let pieces=crate::trim_parts::kept(&cut.owner.source,&cut.parts,&cut.removed)?;crate::split_draft::adopt(&mut preview,cut.owner.id,pieces)?;}}
        self.preview=Some(preview);Ok(())
    }

    pub fn changed(&self)->bool{self.cuts.iter().any(|c|!c.removed.is_empty())}
    pub fn prepared_edits(&self,scene:&Scene)->Result<Vec<(ObjectId,Vec<Geometry>)>,String>{self.cuts.iter().filter(|c|!c.removed.is_empty()).map(|c|{c.owner.valid(scene)?;Ok((c.owner.id,crate::trim_parts::kept(&c.owner.source,&c.parts,&c.removed)?))}).collect()}
}
