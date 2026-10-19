
pub struct Renderer {
    text:std::cell::RefCell<Option<crate::scene_text_gpu::Lane>>,
    text_error:std::cell::RefCell<Option<String>>,
    format:wgpu::TextureFormat,
