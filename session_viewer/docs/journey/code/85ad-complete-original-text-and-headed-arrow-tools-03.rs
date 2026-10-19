    fn build(&self, points: &[[f64; 3]]) -> Result<Built, &'static str> {
        match (self, points) {
            (Self::Arrow,[a,b])=>{let mut line=session_rust::Line::new(a[0],a[1],a[2],b[0],b[1],b[2]);if line.length()<=1e-12{return Err("Arrow needs distinct endpoints")}line.arrowhead=session_rust::Arrowhead::END;Ok(Built::Polyline(crate::chain::PreparedChain::line(Rc::new(line))?))},
            (Self::Text,[p])=>Ok(Built::Text(*p)),
            (Self::Measure,[a,b])=>Ok(Built::Measured([*a,*b])),
            (Self::Point, [p]) => Ok(Built::Point(PreparedPoint::new(Rc::new(session_rust::Point::new(p[0], p[1], p[2])))?)),
