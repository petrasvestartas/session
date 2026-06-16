use std::collections::HashMap;
use crate::gumball::{Gumball, HandleKind};
use crate::gpu_session::{CylinderSegment, GlyphPoint, make_geom_bind_group};

/// State for the numeric-entry popup shown when a gumball handle is clicked
/// (pressed and released without dragging).
#[derive(Clone)]
pub struct GumballInput {
    pub handle: HandleKind,
    /// Text being typed by the user (parsed as f32 on apply).
    pub buffer: String,
    /// Cursor position in physical pixels where the popup is anchored.
    pub screen_pos: (f32, f32),
    /// One-shot: request keyboard focus for the text field on the next frame.
    pub focus: bool,
}

pub struct GumballState {
    pub gumball: Option<Gumball>,
    /// When false the gumball gizmo is neither drawn nor interactive (selection
    /// is unaffected — re-enabling shows it again on the current selection).
    pub show_gumball: bool,
    pub gumball_scale: f32,
    /// A gumball handle was pressed: (handle, press_x, press_y) in physical px.
    /// Drag only begins once the cursor moves past the click/drag threshold; a
    /// release with no movement opens [`GumballInput`] instead.
    pub gumball_press: Option<(HandleKind, f64, f64)>,
    /// Active numeric-entry popup, if any.
    pub gumball_input: Option<GumballInput>,
    /// True once a press has moved past the drag threshold this interaction, so the
    /// release is treated as the end of a drag rather than a click on the handle.
    pub gumball_dragged: bool,
    pub drag_origins: HashMap<String, [[f32; 4]; 4]>,
    /// Geometry snapshot taken at drag-start for non-BRep objects so undo can restore exact state.
    pub drag_geom_snapshots: HashMap<String, session_rust::session::Geometry>,
    /// NurbsSurface snapshot taken at drag-start for NURBS objects.
    pub drag_nurbs_snapshots: HashMap<String, session_rust::NurbsSurface>,
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
