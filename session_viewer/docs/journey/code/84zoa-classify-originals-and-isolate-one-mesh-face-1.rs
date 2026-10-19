use session_rust::Geometry;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Box,
    BRep,
    Element,
    Line,
    Mesh,
    Curve,
    Surface,
    Plane,
    Point,
    Cloud,
    Polyline,
}

impl Shape {
    /// The type of one geometry.
    pub fn of(geometry: &Geometry) -> Self {
        match geometry {
            Geometry::OBB(_) => Shape::Box,
            Geometry::BRep(_) => Shape::BRep,
            Geometry::Element(_) => Shape::Element,
            Geometry::Line(_) => Shape::Line,
            Geometry::Mesh(_) => Shape::Mesh,
            Geometry::NurbsCurve(_) => Shape::Curve,
            Geometry::NurbsSurface(_) => Shape::Surface,
            Geometry::Plane(_) => Shape::Plane,
            Geometry::Point(_) => Shape::Point,
            Geometry::PointCloud(_) => Shape::Cloud,
            Geometry::Polyline(_) => Shape::Polyline,
        }
    }

    /// The name shown for an unnamed object.
    pub fn label(self) -> &'static str {
        match self {
            Shape::Box => "Box",
            Shape::BRep => "BRep",
            Shape::Element => "Element",
            Shape::Line => "Line",
            Shape::Mesh => "Mesh",
            Shape::Curve => "NURBS curve",
            Shape::Surface => "NURBS surface",
            Shape::Plane => "Plane",
            Shape::Point => "Point",
            Shape::Cloud => "Point cloud",
            Shape::Polyline => "Polyline",
        }
    }
}

