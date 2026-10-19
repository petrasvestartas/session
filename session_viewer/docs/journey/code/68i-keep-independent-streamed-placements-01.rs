    }
}

#[derive(Clone)]
pub struct Row {pub id:crate::scene::ObjectId,pub guid:String,pub prepared:Rc<Prepared>,pub model:session_rust::Xform,pub point_px:f32,pub controls_visible:bool}
impl Row {
    pub fn bounds(&self)->Bounds {
        let mut result=Bounds {min:[f64::INFINITY;3],max:[f64::NEG_INFINITY;3]};let bounds=&self.prepared.bounds;
        for x in [bounds.min[0],bounds.max[0]] {for y in [bounds.min[1],bounds.max[1]] {for z in [bounds.min[2],bounds.max[2]] {let p=self.model.transform_point(&session_rust::Point::new(x,y,z));result.include([p[0],p[1],p[2]]);}}}result
    }
}
