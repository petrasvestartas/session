
pub struct Renderer {
    text:std::cell::RefCell<Option<crate::text_atlas::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    format:wgpu::TextureFormat,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
