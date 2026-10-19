}

pub fn on_plane(camera:&crate::camera::Camera, screen:[f32;2], frame:CPlane) -> Option<[f64;3]> {
    on_at(camera,screen,frame,None)
}

pub fn on_at(camera:&crate::camera::Camera,screen:[f32;2],frame:CPlane,through:Option<[f64;3]>) -> Option<[f64;3]> {
    let ray=camera.ray(screen)?;
    let o=through.map(|p|Point::new(p[0],p[1],p[2])).unwrap_or_else(||frame.origin());
    let from=Point::new(ray.origin[0],ray.origin[1],ray.origin[2]);
    // Edge-on planes use the cursor's view depth before projection, avoiding an unstable intersection with a nearly parallel ray.
    let hit=if frame.grazed(&ray.direction) {
        let forward=camera.orientation.rotate_vector(Vector::y_axis());let denominator=ray.direction.dot(&forward);
        if denominator.abs()<1e-12 {return None;}
        let delta=Vector::new(camera.target[0]-from[0],camera.target[1]-from[1],camera.target[2]-from[2]);
        let distance=delta.dot(&forward)/denominator;
        if !distance.is_finite() || distance<=0. || distance>ray.max_distance {return None;}
        let at=Point::new(from[0]+ray.direction[0]*distance,from[1]+ray.direction[1]*distance,from[2]+ray.direction[2]*distance);
        frame.project(&o,&at)
    }else{frame.hit(&o,&from,&ray.direction)?};
    let distance=(0..3).map(|i|(hit[i]-from[i]).powi(2)).sum::<f64>().sqrt();
    if distance>ray.max_distance {return None;}crate::tool_coords::checked([hit[0],hit[1],hit[2]]).ok()
}

