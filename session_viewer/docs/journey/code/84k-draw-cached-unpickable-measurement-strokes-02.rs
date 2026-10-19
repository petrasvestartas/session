use crate::{measure_mark::Mark, stroke_gpu::Lane};
pub struct Measurement {
    pub mark: Option<Mark>,
    strokes: Lane,
}
impl Measurement {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            mark: None,
            strokes: Lane::overlay(device, format),
        }
    }
    pub fn set(&mut self, device: &wgpu::Device, mark: Option<&Mark>) -> Result<(), &'static str> {
        if self.mark.as_ref() == mark {
            return Ok(());
        }
        let mut strokes = Vec::new();
        if let Some(mark) = mark {
            for p in mark.points.windows(2) {
                if p[0] != p[1] {
                    strokes.push(crate::stroke::Stroke {
                        start: p[0].map(|v| v as f32),
                        end: p[1].map(|v| v as f32),
                        width: 1.5,
                        colour: [20. / 255., 20. / 255., 20. / 255., 1.],
                    });
                }
            }
        }
        self.strokes.set(device, strokes)?;
        self.mark = mark.cloned();
        Ok(())
    }
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        self.strokes.view(queue, transform, size, density)
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) {
        self.strokes.draw(pass)
    }
    pub fn bytes(&self) -> u64 {
        self.strokes.usage()[1]
    }
}
