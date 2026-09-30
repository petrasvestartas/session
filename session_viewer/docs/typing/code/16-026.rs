
/// A geometry's type, kept for the rows of a released document.
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

/// Where a released document's kernel objects come back from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fetch {
    Idle,    // nothing asked for it
    Wanted,  // an edit needs it
    Loading, // on its way
    Failed,  // the last fetch failed; only an edit asks again
}

/// A document whose kernel objects were dropped after the walk; its rows stay drawn.
pub struct Released {
    pub url: String,            // the file, fetched again to edit it
    pub token: u64,             // this release; an older fetch is ignored
    first: u32,                 // its first row
    shapes: Vec<Option<Shape>>, // type of each row from `first`
    names: Vec<u32>,            // name of each row from `first`, in `table`
    table: Vec<Box<str>>,       // the distinct object names
    pub fetch: Fetch,           // whether it is being fetched
}

