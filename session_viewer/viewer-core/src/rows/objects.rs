use super::hull::{Hull, placed_box};
use session_rust::{AABB, Xform};

/// One object row as the CPU builds it.
#[derive(Clone)]
pub struct ObjectRow {
    pub place: Xform,       // world placement
    pub color: [f32; 4],    // rgba tint
    pub edge_color: u32,    // packed edge color
    pub flags: u32,         // Instance::FLAG_* bits
    pub bounds: AABB,       // box in the object's own space
    pub spacing: f32,       // vertex spacing, or point size for clouds
    pub faces: bool,        // true when the object drew faces
    pub hull: Option<Hull>, // extreme points in the object's own space, for an exact turned box
}

impl ObjectRow {
    /// A row with a placement and flags, everything else empty.
    pub fn new(place: Xform, flags: u32) -> Self {
        Self {
            place,
            color: [1.0; 4],
            edge_color: 0,
            flags,
            bounds: AABB::empty(),
            spacing: 0.0,
            faces: false,
            hull: None,
        }
    }
}

/// Object rows of one upload.
#[derive(Default)]
pub struct ObjectRows {
    pub rows: Vec<ObjectRow>, // one per object
}

/// The row's box in world space: exact over its extreme points, else its own box turned.
pub fn world_box(r: &ObjectRow) -> AABB {
    placed_bounds(&r.bounds, r.hull.as_deref(), &r.place)
}

/// `local` placed by `place`, exact when the extreme points are known.
pub fn placed_bounds(local: &AABB, hull: Option<&[[f32; 3]]>, place: &Xform) -> AABB {
    match hull {
        Some(points) if local.is_valid() => placed_box(points, place),
        _ => local.transformed(place),
    }
}
