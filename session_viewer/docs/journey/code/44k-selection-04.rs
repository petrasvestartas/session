    fn default() -> Self {
        Self {
            scene: Scene::demo(),
            selected: None, selected_part: None, pick_mode: Default::default(),
            camera: Camera::default(),
            background: Background::default(),
            normal_view: false,
