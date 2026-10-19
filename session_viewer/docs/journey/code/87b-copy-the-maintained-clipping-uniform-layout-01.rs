use session_rust::Xform;
use crate::clipping_geometry::{ClipPlane,dot};
pub const MAX_PLANES:usize=6;
const HATCH_CSS_PX:f64=8.;
#[repr(C)]
#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct ClipUniform {
    // offset size
    pub planes: [[f32; 4]; MAX_PLANES], //   0   96  kept side relative to the anchor
    pub screen: [[f32; 4]; MAX_PLANES], //  96   96  the same over canvas clip space
    pub hatch: [[f32; 4]; MAX_PLANES],  // 192   96  hatch coordinate numerator over (x, y, 1)
    pub hatch_w: [[f32; 4]; MAX_PLANES], // 288   96  its denominator
    pub sides: [[f32; 4]; 2],           // 384   32  side of each plane the eye is on
    pub count: u32,                     // 416    4  planes in use
    pub samples: u32,                   // 420    4  scene samples per pixel
    pub fill: u32,                      // 424    4  0 hatch, 1 solid light grey
    pub spacing: f32,                   // 428    4  hatch spacing, px
    pub width: f32,                     // 432    4  hatch line width, px
    pub outline: f32,                   // 436    4  cut boundary width, px
    pub pad: [f32; 2],                  // 440    8  -> 448
}



