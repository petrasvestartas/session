        for corner in bounds.corners() {
            let p: [f64; 3] = std::array::from_fn(|i|corner[i]-centre[i]);