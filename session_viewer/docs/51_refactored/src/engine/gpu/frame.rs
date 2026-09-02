//! The per-frame uniforms every shader reads: the camera matrix (group 0), the line/pen block
//! and the cloud block (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the splat records and the inside test.

use crate::engine::pipelines::Layouts;
use crate::math::{eye_from_view_proj, ortho_half_height};
use session_rust::Xform;
use super::buffers::GpuCtx;
use super::view::View;
use wgpu::util::DeviceExt;

/// What one frame needs from the caller: the camera, the clear colour and the frame's ONE
/// timestamp (ms) - the re-anchor throttle and the fps counter both read it, neither reads a clock.
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
}

/// The three bind groups every family draw needs, borrowed for one pass.
pub struct Binds<'a> {
    pub mvp: &'a wgpu::BindGroup,
    pub line: &'a wgpu::BindGroup,
    pub instances: &'a wgpu::BindGroup,
}

/// The line/pen block (group 1), 48 B - three vec4s; the mirror test checks the shaders' copy.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct LineUniform {
    thickness: f32, // on-screen width, px
    proj_y: f32, // vertical projection scale x unit scale
    ortho_h: f32, // ortho world half-height x unit scale
    vp_h: f32, // framebuffer height, px
    vp_w: f32, // framebuffer width, px - flat linework needs the aspect
    // Camera position, in the SAME anchored frame the instance rows use - so a shader can build
    // the view ray to a point as `eye - p`. That is what the per-edge facing test needs, and it
    // has to be the real eye rather than a constant forward direction: at this 60 degree FOV a
    // constant direction is off by up to 30 degrees at the frame corner, and it is precisely the
    // near-silhouette edges - the ones whose classification is in doubt - that would flip.
    eye: [f32; 3],   // 12 B - and it fills the pad WGSL leaves before `anchor`'s 16 B alignment
    // The camera-relative ANCHOR, world units. Instance rows are rebased about it, so anything
    // NOT an instance - the grid, the axes - has to subtract it too or it drifts away from the
    // scene every time re-anchoring fires.
    anchor: [f32; 3],
    _pad1: f32, // 4 B - struct size rounds up to the 16 B alignment
} // 48 B - three vec4s

// The shaders declare this same struct with `anchor: vec3<f32>`, which WGSL aligns to 16 - so the
// uniform is 48 B there, not the 32 B a naive Rust layout gives. A mismatch is not a compile error:
// it surfaces at run time as "buffer bound with size 32 ... requires at least 48 bytes", every
// frame, from every pipeline that binds group 1.
const _: () = assert!(std::mem::size_of::<LineUniform>() == 48);

// Points global attributes
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CloudUniform{
    size: f32, // global point-cloud size SCALE ([ and ] keys)
    vp_w: f32, // framebuffer width, px
    vp_h: f32, // framebuffer height, px
    _pad: f32,
} // 16 B - one vec4; its own buffer + bind group

/// The three uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    pub(super) mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    pub(super) cloud_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    pub cloud_group: wgpu::BindGroup,
    /// This frame's camera matrix as f32: the splat static-skip key and the record fold.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective), for the splat k.
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test and the LOD screen-error test.
    pub eye: [f32; 3],
}

impl FrameUniforms {
    /// The three buffers and bind groups with no camera yet: identity mvp, a 2 px pen, size-4
    /// clouds. The cloud block reuses the line layout (one uniform at binding 0).
    pub fn new(ctx: &GpuCtx, l: &Layouts, size: (u32, u32)) -> Self {
        let mvp_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("mvp.buffer"),
            contents: bytemuck::cast_slice(&Xform::identity().to_f32()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let mvp_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mvp.bind_group"),
            layout: &l.mvp,
            entries: &[wgpu::BindGroupEntry{
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            }],
        });

        let line_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("line.buffer"),
            contents: bytemuck::bytes_of(&LineUniform {
                thickness: 2.0,
                proj_y: 1.0,
                ortho_h: 0.0,
                vp_h: size.1 as f32,
                vp_w: size.0 as f32,
                eye: [0.0; 3],   // no camera until the first frame writes one
                anchor: [0.0; 3],   // no anchor until the first frame rebases the table
                _pad1: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let line_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("line.bind_group"),
            layout: &l.line,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: line_buffer.as_entire_binding()
            }],
        });

        let cloud_buffer = ctx.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cloud.buffer"),
            contents: bytemuck::bytes_of(&CloudUniform {
                size: 4.0,
                vp_w: size.0 as f32,
                vp_h: size.1 as f32,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cloud_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("cloud.bind_group"),
            layout: &l.line,
            entries: &[wgpu::BindGroupEntry {binding: 0, resource: cloud_buffer.as_entire_binding()}],
        });

        Self {
            mvp_buffer,
            line_buffer,
            cloud_buffer,
            mvp_group,
            line_group,
            cloud_group,
            mvp_f32: [0.0; 16],
            ortho_h: 0.0,
            eye: [0.0; 3],
        }
    }

    /// Per-frame uniforms: camera, the line/pen block, and the cloud block. The eye and the
    /// ortho half-height are solved once here and kept for the rest of the frame.
    pub fn write(&mut self, ctx: &GpuCtx, input: &FrameInput, cx: &FrameCx) {
        self.mvp_f32 = input.view_proj.to_f32();
        self.ortho_h = ortho_half_height(&input.view_proj);
        self.eye = eye_from_view_proj(&input.view_proj);
        ctx.queue.write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));

        let line = LineUniform{
            thickness: cx.view.thickness_px,
            proj_y: 1.0 / (30.0_f32).to_radians().tan() * 0.001, // cot(fovy/2) mm-m unit scale
            ortho_h: self.ortho_h,
            vp_h: cx.size.1 as f32,
            vp_w: cx.size.0 as f32,
            eye: self.eye,
            anchor: cx.anchor,
            _pad1: 0.0,
        };
        ctx.queue.write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
        ctx.queue.write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&CloudUniform{
            size: cx.view.cloud_size,
            vp_w: cx.size.0 as f32,
            vp_h: cx.size.1 as f32,
            _pad: cx.view.edl_strength, // EDL strength, read by the splat resolve
        }));
    }
}
