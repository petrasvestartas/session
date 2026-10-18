    for projection in [Projection::Perspective, Projection::Orthographic] {
        let mut camera = Camera::default(); camera.projection = projection; camera.isometric();
        let bounds = crate::bounds::Bounds { min: [-0.1; 3], max: [0.1; 3] }; camera.fit(&bounds);
        let original = (camera.target, camera.radius, camera.distance); let hidden = camera.view_projection(); let without = scene_only_depth_control(&camera);
        camera.grid = Some(Settings::default()); let with = camera.view_projection(); assert_eq!((camera.target, camera.radius, camera.distance), original);
        assert_eq!(with.m, hidden.m, "Grid visibility cannot change the current camera range");
        let mut previously_clipped = 0;
        for point in Settings::default().points().unwrap() {
            let p = session_rust::Point::new(point[0], point[1], point[2]);
