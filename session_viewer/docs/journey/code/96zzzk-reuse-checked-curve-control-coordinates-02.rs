    pub fn position(&self) -> Result<[f64; 3], &'static str> {
        position(&self.source,self.index)
    }
