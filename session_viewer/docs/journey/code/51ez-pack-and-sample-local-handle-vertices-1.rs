use crate::gizmo::{ARM, BALL_AT, HUB};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub(crate) position: [f32; 3],
    pub(crate) color: u32,        
    pub(crate) handle: u32,       
}

const COLORS: [u32; 3] = [0xff2424e8, 0xff30b820, 0xffef6628];

const SIDES: usize = 24;

const SHAFT: f32 = 2.2;

const TIP: f32 = 14.0;

fn orient(p: [f32; 3], axis: usize) -> [f32; 3] {
    match axis {
        0 => p,
        1 => [p[2], p[0], p[1]],
        _ => [p[1], p[2], p[0]],
    }
}

fn surface(
    out: &mut Vec<Vertex>,
    rows: usize,
    columns: usize,
    color: u32,
    handle: u32,
    sample: impl Fn(f32, f32) -> [f32; 3],
) {
    for row in 0..rows {
        for column in 0..columns {
            for (i, j) in [(0, 0), (1, 0), (1, 1), (0, 0), (1, 1), (0, 1)] {
                let position = sample(
                    (row + i) as f32 / rows as f32,
                    (column + j) as f32 / columns as f32,
                );
                out.push(Vertex {
                    position,
                    color,
                    handle,
                });
            }
        }
    }
}


impl Vertex {
    pub fn bytes(self) -> [u8;20] {
        let mut bytes=[0;20];
        for (i,value) in self.position.into_iter().enumerate() { bytes[i*4..i*4+4].copy_from_slice(&value.to_ne_bytes()); }
        bytes[12..16].copy_from_slice(&self.color.to_ne_bytes()); bytes[16..20].copy_from_slice(&self.handle.to_ne_bytes()); bytes
    }
}
