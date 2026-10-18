    pub fn corners(&self)->[[f64;3];8] {
        std::array::from_fn(|corner|std::array::from_fn(|axis|if corner&(1<<axis)==0 {self.min[axis]}else{self.max[axis]}))
    }

    pub fn centre(&self) -> [f64; 3] {