// --8<-- [start:step-7a]
use super::buffers::{GpuCtx, bind_group, uniform_buffer};
use super::view::View;
use crate::camera::FOVY_DEG;
use crate::engine::pipelines::Layouts;
use session_rust::Xform;

/// Everything that changes per frame; the renderer keeps no camera or clock of its own.
pub struct FrameInput {
    pub view_proj: Xform,
    pub clear: wgpu::Color,
    pub now_ms: f64, // browser clock, milliseconds
}

/// `'a`: this struct must not outlive the View it borrows.
pub struct FrameCx<'a> {
    pub view: &'a View,
    pub anchor: [f32; 3],
    pub size: (u32, u32), // device pixels, not CSS pixels
    pub pixel_scale: f32,
}

/// Group 0 = camera matrix, group 1 = pen and view settings, group 2 = one row per object.
pub struct Binds<'a> {
    pub mvp: &'a wgpu::BindGroup,
    pub line: &'a wgpu::BindGroup,
    pub instances: &'a wgpu::BindGroup,
}

// `'_` = whatever lifetime this Binds was made with; the method has no reason to name it.
impl Binds<'_> {
    /// Every pipeline expects the same three groups, so each draw sets them in one call.
    pub fn set(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(0, self.mvp, &[]);
        pass.set_bind_group(1, self.line, &[]);
        pass.set_bind_group(2, self.instances, &[]);
    }
}

// --8<-- [end:step-7a]
// --8<-- [start:step-7b]
/// Pen and view settings every shader reads, 80 bytes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineUniform {
    pub thickness: f32, // line width, device pixels
    pub proj_y: f32, // perspective: clip units per mm at distance 1
    pub ortho_h: f32, // half the view height when orthographic; 0 = perspective
    pub vp_h: f32, // vp = viewport: the render target's size in pixels
    pub vp_w: f32,
    pub eye: [f32; 3],
    pub anchor: [f32; 3],
    pub feather: f32, // soft edge in px: lines fade out instead of stair-stepping
    pub lit: f32, // 0 flat, 1 lit, 2 lit + ambient occlusion
    pub backface: f32, // 1 = paint back faces red
    pub origin: [f32; 2], // where this render target sits in the canvas, px; non-zero only in a pick
    pub frame: [f32; 2], // canvas size, px
    pub opacity: f32, // mesh face alpha, 1 = opaque
    pub _pad: f32, // uniform sizes go in 16-byte steps: 76 becomes 80
}

// Checked at compile time: a moved field fails the build instead of drawing garbage.
const _: () = {
    assert!(std::mem::size_of::<LineUniform>() == 80);
    assert!(std::mem::offset_of!(LineUniform, lit) == 48);
    assert!(std::mem::offset_of!(LineUniform, backface) == 52);
    assert!(std::mem::offset_of!(LineUniform, origin) == 56);
    assert!(std::mem::offset_of!(LineUniform, frame) == 64);
    assert!(std::mem::offset_of!(LineUniform, opacity) == 72);
};

/// Point cloud settings, 48 bytes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CloudUniform {
    pub size: f32, // point size scale; the CPU applies it, shaders do not
    pub vp_w: f32,
    pub vp_h: f32,
    pub edl: f32, // EDL = eye-dome lighting: darkens depth jumps so a cloud reads as solid; 0 = off
    pub _pad0: f32,
    pub _pad1: f32,
    pub origin: [f32; 2],
    pub frame: [f32; 2],
    pub _pad: [f32; 2],
}

const _: () = {
    assert!(std::mem::size_of::<CloudUniform>() == 48);
    assert!(std::mem::offset_of!(CloudUniform, origin) == 24);
    assert!(std::mem::offset_of!(CloudUniform, frame) == 32);
};

// --8<-- [end:step-7b]
// --8<-- [start:step-7c]
/// A pick renders only a small window around the cursor, 13 x 13 px by default, not the whole canvas.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickView {
    pub x: u32, // canvas pixels from the top-left
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl PickView {
    /// .max(1): a 0 x 0 canvas would divide by zero later.
    pub fn whole(size: (u32, u32)) -> Self {
        Self {
            x: 0,
            y: 0,
            w: size.0.max(1),
            h: size.1.max(1),
        }
    }

    /// Stretch clip space so this small window fills -1..1, the pick target's whole area.
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

/// Column-major, like WGSL: out = left * right.
fn mat4_mul(left: &[f32; 16], right: &[f32; 16]) -> [f32; 16] {
    let mut out = [0.0; 16];

    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = (0..4).map(|k| left[k * 4 + row] * right[col * 4 + k]).sum();
        }
    }

    out
}

// --8<-- [end:step-7c]
// --8<-- [start:step-7d]
/// Two copies of every uniform: one for the frame, one for the pick window, so a pick never disturbs the picture.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,
    line_buffer: wgpu::Buffer,
    cloud_buffer: wgpu::Buffer,
    pub mvp_group: wgpu::BindGroup,
    pub line_group: wgpu::BindGroup,
    pub cloud_group: wgpu::BindGroup, // the point lane's group 1
    pick_mvp_buffer: wgpu::Buffer, // the same three, for the pick window
    pick_line_buffer: wgpu::Buffer,
    pick_cloud_buffer: wgpu::Buffer,
    pick_transform_buffer: wgpu::Buffer, // canvas-to-window matrix
    pub pick_mvp_group: wgpu::BindGroup,
    pub pick_line_group: wgpu::BindGroup,
    pub pick_cloud_group: wgpu::BindGroup,
    pub pick_transform_group: wgpu::BindGroup, // text lanes' pick group
    line: LineUniform, // kept to derive the pick copy
    cloud: CloudUniform,
    pub mvp_f32: [f32; 16],
    pub ortho_h: f32,
    pub eye: [f32; 3],
}

// --8<-- [end:step-7d]
// --8<-- [start:step-7e]
impl FrameUniforms {
    /// `'a` on both inputs: the Binds may outlive neither self nor instances.
    pub fn binds<'a>(&'a self, instances: &'a wgpu::BindGroup) -> Binds<'a> {
        Binds {
            mvp: &self.mvp_group,
            line: &self.line_group,
            instances,
        }
    }

    pub fn pick_binds<'a>(&'a self, instances: &'a wgpu::BindGroup) -> Binds<'a> {
        Binds {
            mvp: &self.pick_mvp_group,
            line: &self.pick_line_group,
            instances,
        }
    }

    /// For the memory counters: 448 bytes in all.
    pub fn allocated_bytes(&self) -> u64 {
        self.mvp_buffer.size()
            + self.line_buffer.size()
            + self.cloud_buffer.size()
            + self.pick_mvp_buffer.size()
            + self.pick_line_buffer.size()
            + self.pick_cloud_buffer.size()
            + self.pick_transform_buffer.size()
    }

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
            // --8<-- [end:step-7e]
        // --8<-- [start:step-7f]
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
        let cloud_group = bind_group(ctx, &l.line, "cloud.bind_group", &[&cloud_buffer]); // same layout: one uniform buffer
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

    /// Called once per frame before any draw; the copies land on the GPU at the next submit.
    pub fn write(&mut self, ctx: &GpuCtx, input: &FrameInput, cx: &FrameCx) {
        self.mvp_f32 = input.view_proj.to_f32();
        self.ortho_h = input.view_proj.ortho_half_height() as f32;
        let eye = input.view_proj.eye();
        self.eye = [eye[0] as f32, eye[1] as f32, eye[2] as f32];
        ctx.queue
            .write_buffer(&self.mvp_buffer, 0, bytemuck::cast_slice(&self.mvp_f32));

        let line = LineUniform {
            thickness: cx.view.thickness_px * cx.pixel_scale,
            feather: cx.view.feather_px,
            // world mm to clip units at distance 1
            proj_y: 1.0 / (FOVY_DEG as f32 * 0.5).to_radians().tan() * 0.001,
            ortho_h: self.ortho_h,
            vp_h: cx.size.1 as f32,
            vp_w: cx.size.0 as f32,
            eye: self.eye,
            anchor: cx.anchor,
            // --8<-- [end:step-7f]
            // --8<-- [start:step-7g]
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
            // --8<-- [end:step-7g]
            // --8<-- [start:step-7h]
            origin: [0.0; 2],
            frame: [cx.size.0 as f32, cx.size.1 as f32],
            _pad: [0.0; 2],
        };
        ctx.queue
            .write_buffer(&self.cloud_buffer, 0, bytemuck::bytes_of(&cloud));
        self.cloud = cloud;
    }

    /// Write the same settings for the pick window; call after `write`.
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
        // keeps pixel sizes equal in the smaller window
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

// --8<-- [end:step-7h]
// --8<-- [start:step-7i]
#[cfg(test)]
mod tests {
    use super::*;

    /// Whole canvas maps to identity; a window maps its corners to ±1.
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
        // canvas pixel to clip space
        let ndc = |px: f32, py: f32| [px / 800.0 - 1.0, 1.0 - py / 500.0];
        // --8<-- [end:step-7i]
        // --8<-- [start:step-7j]
        let apply = |ndc: [f32; 2]| [t[0] * ndc[0] + t[12], t[5] * ndc[1] + t[13]];
        let left_top = apply(ndc(100.0, 250.0));
        let right_bottom = apply(ndc(119.0, 269.0));
        assert!((left_top[0] + 1.0).abs() < 1e-4 && (left_top[1] - 1.0).abs() < 1e-4);
        assert!((right_bottom[0] - 1.0).abs() < 1e-4 && (right_bottom[1] + 1.0).abs() < 1e-4);
    }
}
// --8<-- [end:step-7j]
