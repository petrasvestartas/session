    pub fn visible(&self) -> bool {
        match self { Self::Line(line) => line.is_visible, Self::Polyline(line) => line.is_visible, Self::Curve(curve, _) => curve.is_visible }
    }

    pub fn guid(&self) -> &str {