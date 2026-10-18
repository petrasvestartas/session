}

#[derive(Clone, Copy, Debug)]
pub enum Shape { Point, Line, Move }

impl Tool for Shape {
    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move" } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2 } }
    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
