use session_rust::{AABB,Plane};
use crate::production_ribbon::encode_width;
use crate::production_curve_walk::Row;
pub const DEFAULT_SIZE:f64=100.0;
pub const SOURCE:&str=include_str!("production_plane.wgsl");
pub const PROVENANCE:&str=include_str!("production_plane_provenance.json");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaneRow {
    // offset size
    pub origin: [f32; 3], //   0   12  object space
    pub instance_id: u32, //  12    4  object row
    pub x: [f32; 3],      //  16   12  unit x axis
    pub radius: f32,      //  28    4  pen: 0 = default; > 0 world mm; < 0 px
    pub y: [f32; 3],      //  32   12  unit y axis
    pub pad: u32,         //  44    4  -> 48, a multiple of 16
}

const _: () = assert!(std::mem::size_of::<PlaneRow>() == 48);

/// Plane rows of one upload; the walk writes them with `up.lanes.get_mut::<PlaneRows>()`.
#[derive(Default)]
pub struct PlaneRows {
    pub rows: Vec<PlaneRow>, // one per plane
}

pub fn walk_plane(lanes: &mut PlaneRows, pl: &Plane, row: u32) -> Row {
    let (o, x, y, z) = (pl.origin(), pl.x_axis(), pl.y_axis(), pl.z_axis());
    let f32s = |v: [f64; 3]| v.map(|c| c as f32);
    lanes.rows.push(PlaneRow {
        origin: f32s([o[0], o[1], o[2]]),
        instance_id: row,
        x: f32s([x[0], x[1], x[2]]),
        radius: encode_width(pl.width),
        y: f32s([y[0], y[1], y[2]]),
        pad: 0,
    });
    // the grid's corners and the z arrow's tip, at the size walked with
    let size = DEFAULT_SIZE;
    let mut bounds = AABB::empty();

    for (a, b, c) in [
        (1.0, 1.0, 0.0),
        (-1.0, -1.0, 0.0),
        (1.0, -1.0, 0.0),
        (-1.0, 1.0, 0.0),
        (0.0, 0.0, 1.0),
    ] {
        bounds.union_with_point(
            o[0] + (x[0] * a + y[0] * b + z[0] * c) * size,
            o[1] + (x[1] * a + y[1] * b + z[1] * c) * size,
            o[2] + (x[2] * a + y[2] * b + z[2] * c) * size,
        );
    }

    Row {flags:0,bounds}
}

pub fn bytes(row:&PlaneRow,out:&mut Vec<u8>) {
    use crate::production_ribbon::floats;
    floats(out,row.origin);out.extend(row.instance_id.to_ne_bytes());
    floats(out,row.x);floats(out,[row.radius]);floats(out,row.y);out.extend(row.pad.to_ne_bytes());
}
