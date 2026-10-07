impl Source {
    pub fn guid(&self) -> &str { match self { Self::Line(line) => line.guid(), Self::Polyline(line) => line.guid() } }

    pub fn coordinates