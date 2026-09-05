//! The per-frame uniforms every shader reads: the camera matrix (group 0), the line/pen block
//! and the cloud block (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the point records and the inside test.

use crate::engine::pipelines::Layouts;
use crate::math::{eye_from_view_proj, ortho_half_height, FOVY_DEG};
use session_rust::Xform;
use super::buffers::{bind_group, uniform_buffer, GpuCtx};
use super::view::View;

/// What one frame needs from the caller: the camera, the clear colour and the frame's ONE
/// timestamp (ms) - the re-anchor throttle and the fps counter both read it.
pub struct FrameInput {
    pub view_proj: Xform,
    pub clear: wgpu::Color,
    pub now_ms: f64,
}

/// What `FrameUniforms::write` needs besides the camera: the knobs, the anchor the instance
/// rows are rebased about, and the framebuffer size in pixels.
pub struct FrameCx<'a> {
    pub view: &'a View,
    pub anchor: [f32; 3],
    pub size: (u32, u32),
    pub occluder_rect: [f32; 4],
}

/// The three bind groups every lane draw needs, borrowed for one pass.
pub struct Binds<'a> {
    pub mvp: &'a wgpu::BindGroup,
    pub line: &'a wgpu::BindGroup,
    pub instances: &'a wgpu::BindGroup,
}

impl Binds<'_> {
    /// Bind groups 0, 1 and 2 - the prefix of every lane draw.
    pub fn set(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(0, self.mvp, &[]);
        pass.set_bind_group(1, self.line, &[]);
        pass.set_bind_group(2, self.instances, &[]);
    }
}

/// The line/pen block (group 1), 64 B; the physical screen rectangle starts at byte 48.
/// `eye` and `anchor` are in the anchored frame the instance rows use.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineUniform {
    pub thickness: f32, // on-screen pen width, px
    pub proj_y: f32,    // cot(fovy/2) x unit scale
    pub ortho_h: f32,   // ortho half-height x unit scale, 0 = perspective
    pub vp_h: f32,      // framebuffer height, px
    pub vp_w: f32,      // framebuffer width, px
    pub eye: [f32; 3],  // camera position, anchored world units
    pub anchor: [f32; 3],
    pub feather: f32, // antialiasing ramp of the ink lanes, px
    pub occluder_rect: [f32; 4], // conservative physical pixel bounds: left, top, right, bottom
}

const _: () = {
    assert!(std::mem::size_of::<LineUniform>() == 64);
    assert!(std::mem::offset_of!(LineUniform, occluder_rect) == 48);
};

/// The cloud block (group 1 of the point lane), 16 B.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CloudUniform {
    pub size: f32, // global scale on per-cloud point sizes
    pub vp_w: f32,
    pub vp_h: f32,
    pub edl: f32,  // Eye-Dome Lighting strength; 0 = off
}

/// The three uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    cloud_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    pub cloud_group: wgpu::BindGroup,
    /// This frame's camera matrix as f32: the point lane's static-skip key and record fold.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective).
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test and the LOD screen-error test.
    pub eye: [f32; 3],
}

impl FrameUniforms {
    /// The exact camera/line buffers shared with the angular compute filter.
    pub(super) fn face_filter_uniforms(&self) -> (&wgpu::Buffer, &wgpu::Buffer) {
        (&self.mvp_buffer, &self.line_buffer)
    }

    /// The three buffers and bind groups with no camera yet.
    pub fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let mvp_buffer = uniform_buffer(&ctx.device, "mvp.buffer", &Xform::identity().to_f32());
        let line = LineUniform {
            thickness: 2.0,
            proj_y: 1.0,
            ortho_h: 0.0,
            vp_h: size.1 as f32,
            vp_w: size.0 as f32,
            eye: [0.0; 3],
            anchor: [0.0; 3],
            feather: 1.5,
            occluder_rect: [1.0, 1.0, -1.0, -1.0],
        };
        let line_buffer = uniform_buffer(&ctx.device, "line.buffer", &line);
        let cloud = CloudUniform { size: 1.0, vp_w: size.0 as f32, vp_h: size.1 as f32, edl: 0.0 };
        let cloud_buffer = uniform_buffer(&ctx.device, "cloud.buffer", &cloud);

        let mvp_group = bind_group(ctx, &l.mvp, "mvp.bind_group", &[&mvp_buffer]);
        let line_group = bind_group(ctx, &l.line, "line.bind_group", &[&line_buffer]);
        let cloud_group = bind_group(ctx, &l.line, "cloud.bind_group", &[&cloud_buffer]);

        Self { mvp_buffer, line_buffer, cloud_buffer, mvp_group, line_group, cloud_group, mvp_f32: [0.0; 16], ortho_h: 0.0, eye: [0.0; 3] }
    }

    /// Per-frame uniforms: camera, the line/pen block, and the cloud block. The eye and the
    /// ortho half-height are solved once here and kept for the rest of the frame.
    pub fn write(&mut self, ctx: &GpuCtx, input: &FrameInput, cx: &FrameCx) {
        self.mvp_f32 = input.view_proj.to_f32();
        self.ortho_h = ortho_half_height(&input.view_proj);
        self.eye = eye_from_view_proj(&input.view_proj);
        ctx.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));

        let line = LineUniform {
            thickness: cx.view.thickness_px,
            feather: cx.view.feather_px,
            proj_y: 1.0 / (FOVY_DEG as f32 * 0.5).to_radians().tan() * 0.001,
            ortho_h: self.ortho_h,
            vp_h: cx.size.1 as f32,
            vp_w: cx.size.0 as f32,
            eye: self.eye,
            anchor: cx.anchor,
            occluder_rect: cx.occluder_rect,
        };
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));

        let cloud = CloudUniform { size: cx.view.cloud_size, vp_w: cx.size.0 as f32, vp_h: cx.size.1 as f32, edl: cx.view.edl_strength };
        ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&cloud));
    }
}
