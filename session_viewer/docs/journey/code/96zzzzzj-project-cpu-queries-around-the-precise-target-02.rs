impl Screen {
    pub fn camera(camera:&crate::camera::Camera,size:[f64;2])->Self {
        Self {matrix:camera.view_projection_anchored(camera.target).m,size,origin:camera.target}
    }