//! Per-frame camera + lighting uniform uploaded to bind group 0, binding 0.

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj:    [[f32; 4]; 4],
    pub key_light_ws: [f32; 4],
    pub fill_light_ws:[f32; 4],
    pub screen_size:  [f32; 2],
    pub point_size:   f32,
    pub flags:        u32,   // bit 0 = no shading (unlit)
}

impl Default for CameraUniform {
    fn default() -> Self {
        let ident = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self {
            view_proj:    ident,
            key_light_ws: [0.0, 1.0, 0.0, 0.0],
            fill_light_ws:[1.0, 0.0, 0.0, 0.0],
            screen_size:  [1.0, 1.0],
            point_size:   5.0,
            flags:        0,
        }
    }
}
