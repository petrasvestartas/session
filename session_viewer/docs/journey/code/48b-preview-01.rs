use crate::{stroke_gpu::Lane, tool::Draft};

pub struct Preview { strokes: Lane, markers: Lane }

impl Preview {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self { strokes: Lane::new(device, format), markers: Lane::markers(device, format) }
    }
    pub fn set(&mut self, device: &wgpu::Device, draft: Option<&Draft>) -> Result<(), &'static str> {
        let yellow = [1., 0.75, 0.05, 1.]; let mut strokes = Vec::new(); let mut markers = Vec::new();
        if let Some(draft) = draft { if let Some(cursor) = draft.cursor {
            markers.push(crate::marker::Marker { center: cursor.map(|v| v as f32), diameter: 8., colour: yellow });
            if let Some(&start) = draft.points.last().filter(|&&p| p != cursor) {
                strokes.push(crate::stroke::Stroke { start: start.map(|v| v as f32), end: cursor.map(|v| v as f32), width: 2., colour: yellow });
            }
        } }
        self.strokes.set(device, strokes)?; self.markers.set_points(device, markers)?; Ok(())
    }
    pub fn view(&self, queue: &wgpu::Queue, transform: &[f32; 16], size: [u32; 2], density: f32) {
        self.strokes.view(queue, transform, size, density); self.markers.view(queue, transform, size, density);
    }
    pub fn draw(&self, pass: &mut wgpu::RenderPass<'_>) { self.strokes.draw(pass); self.markers.draw(pass); }
    pub fn bytes(&self) -> u64 { self.strokes.usage()[1] + self.markers.usage()[1] }
}
