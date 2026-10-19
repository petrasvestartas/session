        *self=candidate;Ok(geometry)
    }
    pub fn outline(&self,cursor:Option<[f64;3]>)->Vec<Vec<[f64;3]>> {
        let pending=cursor.and_then(|p|self.with(Answer::Point(Point::new(p[0],p[1],p[2]))).ok()).map(|(_,part)|part);
        (self.shape.outline)(pending.as_ref().unwrap_or(&self.part)).into_iter().map(|wire|wire.into_iter().map(|p|[p[0],p[1],p[2]]).collect()).collect()
    }
    pub fn on_axis(&self,camera:&crate::camera::Camera,screen:[f32;2])->Option<[f64;3]> {
        let ray=camera.ray(screen)?;let axis=&self.part.frame;
        let direction=&ray.direction;let (a,b)=(direction.dot(direction),direction.dot(&axis.z));let denominator=a-b*b;
        if denominator<=0.01*a {return None;}
        let offset=Vector::new(ray.origin[0]-axis.origin[0],ray.origin[1]-axis.origin[1],ray.origin[2]-axis.origin[2]);
        let height=(a*offset.dot(&axis.z)-b*offset.dot(direction))/denominator;
        let p=axis.at(0.,0.,height);crate::tool_coords::checked([p[0],p[1],p[2]]).ok()
    }
}
