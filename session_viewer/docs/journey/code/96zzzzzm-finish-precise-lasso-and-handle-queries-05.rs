pub fn camera_scale(camera:&crate::camera::Camera,origin:[f64;3],height:f64)->Option<f64> {
    let matrix=camera.view_projection_anchored(camera.target);
    world_per_pixel(&matrix,std::array::from_fn(|i|origin[i]-camera.target[i]),height)
}

// Measuring the projected pivot keeps a 96-pixel handle the same size while the camera zooms or changes projection.