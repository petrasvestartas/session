    pub pad: [f32; 2],                  // 440    8  -> 448
}
pub struct ClipView<'a>{pub view_proj:&'a Xform,pub anchor:[f64;3],pub height:u32,pub pixel_scale:f64,pub samples:u32,pub forward_depth:bool}


