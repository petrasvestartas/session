            let frame = frame(scale);
            assert_eq!(frame.scale().unwrap(), scale as f32);
            let label = TextLabel {
                bare: false, object: None,
                id: 1,
                text: "A".into(),
                font_size: 16.0,
