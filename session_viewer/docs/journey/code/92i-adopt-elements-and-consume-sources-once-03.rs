    match (a, b) {
        (Geometry::PointCloud(a), Geometry::PointCloud(b)) => Rc::ptr_eq(a,b),
        (Geometry::Element(a), Geometry::Element(b))=>Rc::ptr_eq(a,b),
        (Geometry::Plane(a), Geometry::Plane(b)) => Rc::ptr_eq(a,b),
        (Geometry::OBB(a), Geometry::OBB(b)) => Rc::ptr_eq(a, b),
