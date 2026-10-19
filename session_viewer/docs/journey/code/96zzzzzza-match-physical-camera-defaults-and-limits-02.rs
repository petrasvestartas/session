let scale=self.unit.to_meters();
            self.distance=(self.distance/factor as f64).clamp(1.0e-6/scale,1.0e15/scale);