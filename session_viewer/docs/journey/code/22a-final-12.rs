pub struct Bounds {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl Bounds {
    pub fn point(point: [f64; 3]) -> Self {
        Self { min: point, max: point }
    }

    pub fn include(&mut self, point: [f64; 3]) {
        for i in 0..3 {
            self.min[i] = self.min[i].min(point[i]);
            self.max[i] = self.max[i].max(point[i]);
        }
    }

    pub fn centre(&self) -> [f64; 3] {
        std::array::from_fn(|i| (self.min[i] + self.max[i]) * 0.5)
    }

    pub fn radius(&self) -> f64 {
        let half = std::array::from_fn::<_, 3, _>(|i| (self.max[i] - self.min[i]) * 0.5);
        (half[0] * half[0] + half[1] * half[1] + half[2] * half[2]).sqrt()
    }
}
