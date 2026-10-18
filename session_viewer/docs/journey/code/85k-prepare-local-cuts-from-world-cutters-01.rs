        list.push(owner);Ok(())
    }
    pub fn prepare(&mut self,scene:&Scene,camera:&crate::camera::Camera)->Result<(),String>{
        if self.phase==Phase::Targets{if self.targets.is_empty(){return Err("Select objects to trim first".into())}self.phase=Phase::Cutters;return Ok(())}
        if self.cutters.is_empty(){return Err("Select cutting objects first".into())}
        let normal=crate::cplane::CPlane::facing(&camera.orientation.rotate_vector(Vector::y_axis())).normal();let mut blades=Vec::new();
        for owner in &self.cutters{owner.valid(scene)?;
            if let Some(plane)=as_plane(&owner.source){if let Some(plane)=plane_moved(&plane,&owner.model){blades.push((Cutter::Plane(plane),None))}continue}
            let curve=as_curve(&owner.source).ok_or("Unsupported cutter")?.transformed(&owner.model);
            let fence=matches!(&owner.source,Geometry::Line(_)).then(||fence_plane(&Line::from_points(&curve.point_at_start(),&curve.point_at_end()),&normal).ok()).flatten();blades.push((Cutter::Curve(curve),fence));
        }
        let mut cuts=Vec::new();let mut missed=0;let mut refusal=None;
        for owner in &self.targets{owner.valid(scene)?;let back=owner.model.inverse().ok_or("Trim placement cannot be inverted")?;
            let local=blades.iter().filter_map(|(cutter,fence)|Some(Blade{cutter:cutter.moved(&back)?,fence:fence.as_ref().and_then(|p|plane_moved(p,&back))})).collect::<Vec<_>>();
            match crate::trim_parts::cut(&owner.source,&local){Ok(parts)=>{let world=|p:&Point|p.transformed(&owner.model);let outlines=(0..parts.count()).map(|i|parts.outline(i).iter().map(world).collect()).collect();let marks=parts.cut_points().iter().map(world).collect();let section=parts.section().iter().map(|line|line.iter().map(world).collect()).collect();cuts.push(Cut{owner:owner.clone(),parts,removed:Vec::new(),outlines,marks,section});},Err(error)if error=="not crossed"=>missed+=1,Err(error)=>refusal=Some(error)}
        }
        if cuts.is_empty(){return Err(refusal.unwrap_or_else(||"No crossings: pick other cutters or Esc".into()))}
        self.cuts=cuts;self.phase=Phase::Parts;self.missed=missed;Ok(())
    }






}
