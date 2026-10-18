    pub fn points(&self)->Vec<session_rust::Point> {self.points_at([0.;3])}
    pub fn points_at(&self,anchor:[f64;3])->Vec<session_rust::Point> {
        let model=crate::render_space::placed(&self.model,anchor);
        self.prepared.source.coordinates().into_iter().map(|p|model.transform_point(&session_rust::Point::new(p[0],p[1],p[2]))).collect()