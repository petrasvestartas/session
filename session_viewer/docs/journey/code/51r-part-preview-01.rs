    Ok(keys)
}

// Part previews deform a cloned original source; the saved source and all other owners remain shared until one commit.
pub struct Preview {base:Scene,pub scene:Scene,target:Target,transform:Xform}

impl Preview {
    pub fn new(scene:&Scene,target:Target) -> Result<Self,&'static str> {
        points(scene,target)?;
        scene.placement(target.owner).and_then(|m|m.inverse()).ok_or("Part placement cannot be inverted")?;
        Ok(Self {base:scene.clone(),scene:scene.clone(),target,transform:Xform::identity()})
    }
    pub fn scenes(&self)->[&Scene;2] {[&self.base,&self.scene]}
    pub fn matrix(&self)->&[f64;16] {&self.transform.m}
    pub fn set(&mut self,transform:Xform) -> Result<(), &'static str> {
        if !crate::placement::valid(&transform.m) {return Err("Invalid part transform");}
        let mut scene=self.base.clone();
        if transform.m!=Xform::identity().m {scene.deform_part(self.target,&transform)?;}
        self.scene=scene;self.transform=transform;Ok(())
    }
    pub fn commit(&self,scene:&mut Scene,history:&mut crate::history::History) -> Result<bool,&'static str> {
        if !self.target.valid(scene) || scene.placement(self.target.owner).map(|m|m.m)!=self.base.placement(self.target.owner).map(|m|m.m)
            || !same_source(scene,&self.base,self.target.owner) {return Err("Part source changed; start editing again");}
        if self.transform.m==Xform::identity().m {return Ok(false);}
        history.try_edit(scene,|scene|scene.deform_part(self.target,&self.transform))?;Ok(true)
    }
}

fn same_source(a:&Scene,b:&Scene,id:ObjectId) -> bool {
    if let Some(row)=a.objects().iter().find(|r|r.id==id) {
        return b.objects().iter().find(|r|r.id==id).and_then(|r|r.geometry()).zip(row.geometry()).is_some_and(|(a,b)|Rc::ptr_eq(a,b));
    }
    a.paths().iter().find(|r|r.id==id).zip(b.paths().iter().find(|r|r.id==id)).is_some_and(|(a,b)|a.prepared.source.owner()==b.prepared.source.owner())
}
