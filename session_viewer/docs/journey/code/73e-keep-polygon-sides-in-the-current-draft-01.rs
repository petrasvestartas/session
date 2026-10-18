    fn name(&self) -> &'static str { match self { Self::Point => "Point", Self::Line => "Line", Self::Move => "Move", Self::Polyline => "Polyline", Self::Rectangle=>"Polyline Rectangle",Self::Polygon {..}=>"Polyline Polygon" } }
    fn count(&self) -> usize { match self { Self::Point => 1, Self::Line | Self::Move => 2, Self::Polyline => 0,Self::Rectangle|Self::Polygon {..}=>2 } }
    fn sides(&self)->Option<usize> {if let Self::Polygon {sides}=self {Some(*sides)}else{None}}
    fn set_sides(&mut self,value:usize)->Result<(),&'static str> {
        if !(3..MAX_POINTS).contains(&value) {return Err("Polygon sides must be between 3 and 4095");}
        let Self::Polygon {sides}=self else {return Err("Sides works while drawing a polygon")};*sides=value;Ok(())
    }
    fn build_on_plane(&self,points:&[[f64;3]],plane:crate::cplane::CPlane)->Result<Built,&'static str> {
        let (kind,sides)=match self {Self::Rectangle=>("rectangle",4),Self::Polygon {sides}=>("polygon",*sides),_=>return self.build(points)};
