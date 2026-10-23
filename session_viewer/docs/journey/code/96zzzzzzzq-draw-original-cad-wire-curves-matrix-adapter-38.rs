        let mut frame = frame(2.0);
        frame.origin = [1_000_000.0, 0.0, 0.0];
        let mut label = TextLabel {
            bare: false, object: None,
            id: 1,
            text: "A".into(),
            font_size: 16.0,
