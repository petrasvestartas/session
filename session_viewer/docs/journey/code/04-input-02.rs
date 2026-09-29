#[derive(Default)]
pub struct Background {
    light: bool,
}

impl Background {
    pub fn toggle(&mut self) {
        self.light = !self.light;
    }

    pub fn rgb(&self) -> [f64; 3] {
        if self.light {
            [0.9, 0.9, 0.9]
        } else {
            [0.03, 0.09, 0.20]
        }
    }
}
