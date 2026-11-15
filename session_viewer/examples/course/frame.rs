// --8<-- [start:course-frame]
use super::canvas::{self, Drawing};
use crate::engine::gpu::device::DeviceSetup;

pub fn start() -> Result<(), wasm_bindgen::JsValue> {
    canvas::run(|_, _| Ok(Box::new(Frame)))
}

pub fn clear(encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, color: wgpu::Color) {
    let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("lesson clear"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: wgpu::StoreOp::Store,
            },
        })],
        ..Default::default()
    });
}

struct Frame;
impl Drawing for Frame {
    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, _: f32, _: f64) -> bool {
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        clear(&mut encoder, view, wgpu::Color::WHITE);
        gpu.queue.submit([encoder.finish()]);
        false
    }
}
// --8<-- [end:course-frame]
