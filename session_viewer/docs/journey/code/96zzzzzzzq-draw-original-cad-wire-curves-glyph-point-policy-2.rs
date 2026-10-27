            let span = values.iter().max().unwrap() - values.iter().min().unwrap() + 1;
            assert!((span as f32 - 12.).abs() <= 1., "DPR {density}, axis {axis}, physical span {span}");
        }
