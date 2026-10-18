        let point=std::array::from_fn::<_,3,_>(|i|point[i]-self.origin[i]);
        let clip: [f64; 4] = std::array::from_fn