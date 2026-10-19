    pub fn valid(&self)->bool {
        self.min.into_iter().chain(self.max).all(f64::is_finite)&&(0..3).all(|i|self.min[i]<=self.max[i])
    }

    pub fn corners(&self)->[[f64;3];8] {