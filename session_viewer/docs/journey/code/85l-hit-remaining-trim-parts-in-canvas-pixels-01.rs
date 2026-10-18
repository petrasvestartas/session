        self.cuts=cuts;self.phase=Phase::Parts;self.missed=missed;Ok(())
    }
    pub fn part_at(&self,camera:&crate::camera::Camera,screen:[f32;2],size:[f64;2],surfaces:bool)->Option<(usize,usize)>{
        let project=crate::snap_math::Screen{matrix:camera.view_projection().m,size};let at=((screen[0]as f64+1.)*size[0]/2.,(1.-screen[1]as f64)*size[1]/2.);let mut lines=Vec::new();let mut owners=Vec::new();
        for(i,cut)in self.cuts.iter().enumerate(){for(part,line)in cut.outlines.iter().enumerate(){if cut.removed.contains(&part)||line.is_empty(){continue}lines.push(line.iter().filter_map(|p|project.point([p[0],p[1],p[2]]).map(|p|(p[0],p[1]))).collect());owners.push((i,part));}}
        if let Some(hit)=nearest_px(&lines,at,12.){return Some(owners[hit])}if !surfaces{return None}
        let ray=camera.ray(screen)?;let mut best=None::<(f64,(usize,usize))>;
        for(i,cut)in self.cuts.iter().enumerate(){let back=cut.owner.model.inverse()?;let local=unit_ray(&ray.origin,&ray.direction,&back);for(part,mesh)in cut.parts.hit_meshes(){if cut.removed.contains(&part){continue}if let Some(hits)=intersection::ray_mesh(&local,mesh,1e-12,false){if let Some(hit)=hits.first(){let distance=hit.transformed(&cut.owner.model).distance(&ray.origin,None);if best.is_none_or(|(near,_)|distance<near){best=Some((distance,(i,part)))}}}}}best.map(|(_,hit)|hit)
    }


