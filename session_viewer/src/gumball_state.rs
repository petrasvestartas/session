use std::collections::HashMap;
use crate::gumball::Gumball;
use crate::gpu_session::{CylinderSegment, GlyphPoint, make_geom_bind_group};

pub struct GumballState {
    pub gumball: Option<Gumball>,
    pub gumball_scale: f32,
    pub drag_origins: HashMap<String, [[f32; 4]; 4]>,
    #[allow(dead_code)]
    pub gumball_instance_buf: wgpu::Buffer,
    pub gumball_bind_group: wgpu::BindGroup,
    pub gumball_seg_buf: wgpu::Buffer,
    pub gumball_seg_bg: wgpu::BindGroup,
    pub gumball_cone_buf: wgpu::Buffer,
    pub gumball_cone_bg: wgpu::BindGroup,
    pub gumball_glyph_buf: wgpu::Buffer,
    pub gumball_glyph_bg: wgpu::BindGroup,
}

impl GumballState {
    /// Upload computed gumball geometry to GPU buffers for this frame.
    /// Resizes buffers if needed. Caller must extract GPU refs before calling
    /// (to avoid borrow conflicts with &mut self):
    ///   let (dev, q, bgl) = (&gpu.device, &gpu.queue, &gpu.pipelines.geom_bgl);
    ///   gb.upload_geometry(dev, q, bgl, &segs, &cone_segs, &glyph_pts);
    pub fn upload_geometry(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        geom_bgl: &wgpu::BindGroupLayout,
        segs: &[CylinderSegment],
        cone_segs: &[CylinderSegment],
        glyph_pts: &[GlyphPoint],
    ) {
        let seg_bytes = bytemuck::cast_slice::<_, u8>(segs);
        if !seg_bytes.is_empty() && seg_bytes.len() as u64 > self.gumball_seg_buf.size() {
            self.gumball_seg_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gumball.segments"),
                size: seg_bytes.len() as u64 * 2,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.gumball_seg_bg = make_geom_bind_group(device, geom_bgl, &self.gumball_seg_buf);
        }
        if !seg_bytes.is_empty() {
            queue.write_buffer(&self.gumball_seg_buf, 0, seg_bytes);
        }

        let cone_bytes = bytemuck::cast_slice::<_, u8>(cone_segs);
        if !cone_bytes.is_empty() && cone_bytes.len() as u64 > self.gumball_cone_buf.size() {
            self.gumball_cone_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gumball.cones"),
                size: cone_bytes.len() as u64 * 2,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.gumball_cone_bg = make_geom_bind_group(device, geom_bgl, &self.gumball_cone_buf);
        }
        if !cone_bytes.is_empty() {
            queue.write_buffer(&self.gumball_cone_buf, 0, cone_bytes);
        }

        let glyph_bytes = bytemuck::cast_slice::<_, u8>(glyph_pts);
        if !glyph_bytes.is_empty() && glyph_bytes.len() as u64 > self.gumball_glyph_buf.size() {
            self.gumball_glyph_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("gumball.glyphs"),
                size: glyph_bytes.len() as u64 * 2,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.gumball_glyph_bg = make_geom_bind_group(device, geom_bgl, &self.gumball_glyph_buf);
        }
        if !glyph_bytes.is_empty() {
            queue.write_buffer(&self.gumball_glyph_buf, 0, glyph_bytes);
        }
    }
}
