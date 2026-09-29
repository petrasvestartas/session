use std::mem::{align_of, offset_of, size_of};

#[repr(C)]
struct Point {
    position: [f32; 3],
    weight: f32,
}

#[repr(C, align(16))]
struct Record {
    position: [f32; 4],
    colour: [f32; 4],
}

fn main() {
    assert_eq!(size_of::<Point>(), 16);
    assert_eq!(offset_of!(Point, weight), 12);
    assert_eq!(size_of::<Record>(), 32);
    assert_eq!(align_of::<Record>(), 16);
    assert_eq!(offset_of!(Record, colour), 16);
}
