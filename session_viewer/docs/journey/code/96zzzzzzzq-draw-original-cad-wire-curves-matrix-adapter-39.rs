    fn nameplate_center_padding_clip_and_scale_share_one_coordinate_system() {
        let mut document = TextDocument::new();
        let mut label = TextLabel {
            bare: false, object: None,
            id: 1,
            text: "Sphere Ø25".into(),
            font_size: 18.0,
