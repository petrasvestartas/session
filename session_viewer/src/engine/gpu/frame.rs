//! The per-frame uniforms every shader reads: the camera matrix (group 0), the line/pen block
//! and the cloud block (group 1), written once per frame from a `FrameInput`. The eye and the
//! ortho half-height are solved here ONCE and read by the point records and the inside test.

use super::buffers::{GpuCtx, bind_group, uniform_buffer};
use super::view::View;
use crate::engine::pipelines::Layouts;
use crate::math::{FOVY_DEG, eye_from_view_proj, ortho_half_height};
use session_rust::Xform;

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
    /// Actual framebuffer pixels per CSS pixel (one for native regression targets).
    pub pixel_scale: f32,
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

/// The line/pen block (group 1), 80 B. `eye` and `anchor` are in the anchored frame the
/// instance rows use. Offsets: thickness 0, proj_y 4, ortho_h 8, vp_h 12, vp_w 16, eye 20,
/// anchor 32 (vec3 aligned to 16), feather 44, lit 48, backface 52, origin 56, frame 64,
/// opacity 72.
///
/// `vp_w`/`vp_h` are the pass's own attachment; `frame` is the canvas the scene was projected
/// for and `origin` where this attachment's top-left sits in it. They differ only in the
/// pick pass, which renders the window about the cursor into a window-sized target: pixel
/// arithmetic stays in attachment coordinates, and only the finite-triangle tiles, which were
/// binned for the whole canvas, are addressed through `origin`.
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
    pub feather: f32,     // antialiasing ramp of the ink lanes, px
    pub lit: f32,         // 1 = light the mesh faces, 0 = flat colour
    pub backface: f32,    // 1 = paint back faces red, 0 = their own colour
    pub origin: [f32; 2], // this attachment's top-left in canvas pixels (0 except in a pick)
    pub frame: [f32; 2],  // the canvas the tiles were binned for, px
    pub opacity: f32,     // alpha on shaded mesh faces only; lines and points ignore it
    pub _pad: f32,
}

const _: () = {
    assert!(std::mem::size_of::<LineUniform>() == 80);
    assert!(std::mem::offset_of!(LineUniform, lit) == 48);
    assert!(std::mem::offset_of!(LineUniform, backface) == 52);
    assert!(std::mem::offset_of!(LineUniform, origin) == 56);
    assert!(std::mem::offset_of!(LineUniform, frame) == 64);
    assert!(std::mem::offset_of!(LineUniform, opacity) == 72);
};

/// The cloud block (group 1 of the point lane), 16 B.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CloudUniform {
    pub size: f32, // global scale on per-cloud point sizes
    pub vp_w: f32,
    pub vp_h: f32,
    pub edl: f32, // Eye-Dome Lighting strength; 0 = off
    pub _pad0: f32,
    pub _pad1: f32,
    /// As in `LineUniform`: the canvas the point records were projected for, and where this
    /// attachment's top-left sits in it (zero except in a pick).
    pub origin: [f32; 2],
    pub frame: [f32; 2],
    pub _pad: [f32; 2],
}

const _: () = {
    assert!(std::mem::size_of::<CloudUniform>() == 48);
    assert!(std::mem::offset_of!(CloudUniform, origin) == 24);
    assert!(std::mem::offset_of!(CloudUniform, frame) == 32);
};

/// Where the pick pass draws: a window of the canvas rendered into an attachment of its own
/// size, so the ID targets cost the window, not the canvas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickView {
    /// Top-left of the attachment in canvas pixels.
    pub x: u32,
    pub y: u32,
    /// Attachment size.
    pub w: u32,
    pub h: u32,
}

impl PickView {
    /// The whole canvas: the pick uniforms then equal the frame's.
    pub fn whole(size: (u32, u32)) -> Self {
        Self {
            x: 0,
            y: 0,
            w: size.0.max(1),
            h: size.1.max(1),
        }
    }

    /// The clip-space map from the canvas projection to this window's: the same scene, seen
    /// through the sub-frustum whose viewport is the window. Column-major, like `mvp`.
    pub fn clip_transform(&self, frame: (u32, u32)) -> [f32; 16] {
        let (fw, fh) = (frame.0.max(1) as f32, frame.1.max(1) as f32);
        let (w, h) = (self.w.max(1) as f32, self.h.max(1) as f32);
        let sx = fw / w;
        let sy = fh / h;
        let tx = sx - 2.0 * self.x as f32 / w - 1.0;
        let ty = 1.0 - (fh - 2.0 * self.y as f32) / h;
        [
            sx, 0.0, 0.0, 0.0, //
            0.0, sy, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            tx, ty, 0.0, 1.0,
        ]
    }
}

/// The one uniform the text ID pipelines bind: the pick pass's clip-space map. Its own layout,
/// shared by those pipelines and the bind group, since the text lanes do not see `Layouts`.
pub fn pick_transform_layout(ctx: &GpuCtx) -> wgpu::BindGroupLayout {
    ctx.device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("pick.transform.layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        })
}

/// `left * right` for column-major 4x4 matrices.
fn mat4_mul(left: &[f32; 16], right: &[f32; 16]) -> [f32; 16] {
    let mut out = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = (0..4).map(|k| left[k * 4 + row] * right[col * 4 + k]).sum();
        }
    }
    out
}

/// The three uniform buffers with their bind groups, plus this frame's solved camera facts.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    cloud_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    pub cloud_group: wgpu::BindGroup,
    /// The same three blocks for the pick pass, written per pick from the frame's values and
    /// the pick window; `pick_transform_group` is the bare clip-space map for the text lanes,
    /// whose vertices are already in clip space.
    pick_mvp_buffer: wgpu::Buffer,
    pick_line_buffer: wgpu::Buffer,
    pick_cloud_buffer: wgpu::Buffer,
    pick_transform_buffer: wgpu::Buffer,
    pub pick_mvp_group: wgpu::BindGroup,
    pub pick_line_group: wgpu::BindGroup,
    pub pick_cloud_group: wgpu::BindGroup,
    pub pick_transform_group: wgpu::BindGroup,
    line: LineUniform,
    cloud: CloudUniform,
    /// This frame's camera matrix as f32: the point lane's static-skip key and record fold.
    pub mvp_f32: [f32; 16],
    /// Ortho half-height this frame (0 = perspective).
    pub ortho_h: f32,
    /// Eye in anchored world units, for the inside test and the LOD screen-error test.
    pub eye: [f32; 3],
}

impl FrameUniforms {
    /// Application-owned buffer allocation capacity in bytes; excludes driver overhead.
    pub fn allocated_bytes(&self) -> u64 {
        self.mvp_buffer.size()
            + self.line_buffer.size()
            + self.cloud_buffer.size()
            + self.pick_mvp_buffer.size()
            + self.pick_line_buffer.size()
            + self.pick_cloud_buffer.size()
            + self.pick_transform_buffer.size()
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
            lit: 0.0,
            backface: 0.0,
            origin: [0.0; 2],
            frame: [size.0 as f32, size.1 as f32],
            opacity: 1.0,
            _pad: 0.0,
        };
        let line_buffer = uniform_buffer(&ctx.device, "line.buffer", &line);
        let cloud = CloudUniform {
            size: 1.0,
            vp_w: size.0 as f32,
            vp_h: size.1 as f32,
            edl: 0.0,
            _pad0: 0.0,
            _pad1: 0.0,
            origin: [0.0; 2],
            frame: [size.0 as f32, size.1 as f32],
            _pad: [0.0; 2],
        };
        let cloud_buffer = uniform_buffer(&ctx.device, "cloud.buffer", &cloud);
        let identity = Xform::identity().to_f32();
        let pick_mvp_buffer = uniform_buffer(&ctx.device, "pick.mvp.buffer", &identity);
        let pick_line_buffer = uniform_buffer(&ctx.device, "pick.line.buffer", &line);
        let pick_cloud_buffer = uniform_buffer(&ctx.device, "pick.cloud.buffer", &cloud);
        let pick_transform_buffer = uniform_buffer(&ctx.device, "pick.transform.buffer", &identity);

        let mvp_group = bind_group(ctx, &l.mvp, "mvp.bind_group", &[&mvp_buffer]);
        let line_group = bind_group(ctx, &l.line, "line.bind_group", &[&line_buffer]);
        let cloud_group = bind_group(ctx, &l.line, "cloud.bind_group", &[&cloud_buffer]);
        let pick_mvp_group = bind_group(ctx, &l.mvp, "pick.mvp.bind_group", &[&pick_mvp_buffer]);
        let pick_line_group =
            bind_group(ctx, &l.line, "pick.line.bind_group", &[&pick_line_buffer]);
        let pick_cloud_group =
            bind_group(ctx, &l.line, "pick.cloud.bind_group", &[&pick_cloud_buffer]);
        let pick_transform_group = bind_group(
            ctx,
            &pick_transform_layout(ctx),
            "pick.transform.bind_group",
            &[&pick_transform_buffer],
        );

        Self {
            mvp_buffer,
            line_buffer,
            cloud_buffer,
            mvp_group,
            line_group,
            cloud_group,
            pick_mvp_buffer,
            pick_line_buffer,
            pick_cloud_buffer,
            pick_transform_buffer,
            pick_mvp_group,
            pick_line_group,
            pick_cloud_group,
            pick_transform_group,
            line,
            cloud,
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
        ctx.queue
            .write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));

        let line = LineUniform {
            thickness: cx.view.thickness_px * cx.pixel_scale,
            feather: cx.view.feather_px,
            proj_y: 1.0 / (FOVY_DEG as f32 * 0.5).to_radians().tan() * 0.001,
            ortho_h: self.ortho_h,
            vp_h: cx.size.1 as f32,
            vp_w: cx.size.0 as f32,
            eye: self.eye,
            anchor: cx.anchor,
            lit: f32::from(cx.view.lit),
            backface: f32::from(cx.view.backface),
            origin: [0.0; 2],
            frame: [cx.size.0 as f32, cx.size.1 as f32],
            opacity: cx.view.opacity,
            _pad: 0.0,
        };
        ctx.queue
            .write_buffer(&self.line_buffer, 0, bytemuck::bytes_of(&line));
        self.line = line;

        let cloud = CloudUniform {
            size: cx.view.cloud_size,
            vp_w: cx.size.0 as f32,
            vp_h: cx.size.1 as f32,
            edl: cx.view.edl_strength,
            _pad0: 0.0,
            _pad1: 0.0,
            origin: [0.0; 2],
            frame: [cx.size.0 as f32, cx.size.1 as f32],
            _pad: [0.0; 2],
        };
        ctx.queue
            .write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&cloud));
        self.cloud = cloud;
    }

    /// The pick pass's blocks: this frame's camera seen through `view`'s sub-frustum. Pixel
    /// sizes are preserved by scaling the projection factors with the attachment height, so a
    /// marker or a pen is as wide in the window as on the canvas; `origin` and `frame` let the
    /// visibility test address the canvas-wide tiles. Call after `write` for the same frame.
    pub fn write_pick(&self, ctx: &GpuCtx, view: PickView, frame: (u32, u32)) {
        let transform = view.clip_transform(frame);
        let mvp = mat4_mul(&transform, &self.mvp_f32);
        ctx.queue
            .write_buffer(&self.pick_mvp_buffer, 0, bytemuck::cast_slice(&mvp));
        ctx.queue.write_buffer(
            &self.pick_transform_buffer,
            0,
            bytemuck::cast_slice(&transform),
        );
        let ratio = frame.1.max(1) as f32 / view.h.max(1) as f32;
        let line = LineUniform {
            proj_y: self.line.proj_y * ratio,
            ortho_h: self.line.ortho_h / ratio,
            vp_w: view.w as f32,
            vp_h: view.h as f32,
            origin: [view.x as f32, view.y as f32],
            ..self.line
        };
        ctx.queue
            .write_buffer(&self.pick_line_buffer, 0, bytemuck::bytes_of(&line));
        let cloud = CloudUniform {
            vp_w: view.w as f32,
            vp_h: view.h as f32,
            origin: [view.x as f32, view.y as f32],
            ..self.cloud
        };
        ctx.queue
            .write_buffer(&self.pick_cloud_buffer, 0, bytemuck::bytes_of(&cloud));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The whole canvas is the identity; a window maps its own corners to the clip square.
    #[test]
    fn pick_view_clip_transform() {
        let whole = PickView::whole((1600, 1000)).clip_transform((1600, 1000));
        assert_eq!(whole, Xform::identity().to_f32());
        let view = PickView {
            x: 100,
            y: 250,
            w: 19,
            h: 19,
        };
        let t = view.clip_transform((1600, 1000));
        // A canvas pixel `p` has ndc x = p / 800 - 1 and ndc y = 1 - p / 500.
        let ndc = |px: f32, py: f32| [px / 800.0 - 1.0, 1.0 - py / 500.0];
        let apply = |ndc: [f32; 2]| [t[0] * ndc[0] + t[12], t[5] * ndc[1] + t[13]];
        let left_top = apply(ndc(100.0, 250.0));
        let right_bottom = apply(ndc(119.0, 269.0));
        assert!((left_top[0] + 1.0).abs() < 1e-4 && (left_top[1] - 1.0).abs() < 1e-4);
        assert!((right_bottom[0] - 1.0).abs() < 1e-4 && (right_bottom[1] + 1.0).abs() < 1e-4);
    }
}
