    pub fn prepare_pick(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        matrix: [f32; 16],
        window: crate::pick_window::Window,
        density: f32,
        opacity: f32,
