use super::buffers::{GpuCtx, bind_group, uniform_buffer};
use super::view::View;
use crate::engine::pipelines::Layouts;
pub use super::present::FrameInput;
use session_rust::Xform;

/// Extra inputs for writing the frame uniforms.
pub struct FrameCx<'a> {
    pub view: &'a View,   // display settings
    pub anchor: [f32; 3], // world point the scene is centered on
    pub size: (u32, u32), // framebuffer size, px
    pub pixel_scale: f32, // framebuffer pixels per CSS pixel
}

/// The three bind groups every draw starts with.
pub struct Binds<'a> {
    pub mvp: &'a wgpu::BindGroup,       // group 0: camera matrix
    pub line: &'a wgpu::BindGroup,      // group 1: pen and view settings
    pub instances: &'a wgpu::BindGroup, // group 2: object rows
}

impl Binds<'_> {
    /// Set groups 0, 1 and 2 on the pass.
    pub fn set(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_bind_group(0, self.mvp, &[]);
        pass.set_bind_group(1, self.line, &[]);
        pass.set_bind_group(2, self.instances, &[]);
    }
}

/// Pen and view settings every shader reads, 80 bytes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineUniform {
    pub thickness: f32,   // pen width, px
    pub proj_y: f32,      // perspective scale factor
    pub ortho_h: f32,     // ortho half-height; 0 = perspective
    pub vp_h: f32,        // target height, px
    pub vp_w: f32,        // target width, px
    pub eye: [f32; 3],    // camera position
    pub anchor: [f32; 3], // world point the scene is centered on
    pub feather: f32,     // edge softness of lines, px
    pub lit: f32,         // 0 flat, 1 lit, 2 lit with SSAO
    pub backface: f32,    // 1 = paint back faces red
    pub origin: [f32; 2], // top-left of this target in the canvas, px
    pub frame: [f32; 2],  // canvas size, px
    pub opacity: f32,     // alpha of mesh faces
    pub _pad: f32,        // keeps the size a multiple of 16
}

// the shaders read these byte offsets
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
    pub size: f32,        // point size scale; the CPU applies it, shaders do not
    pub vp_w: f32,        // target width, px
    pub vp_h: f32,        // target height, px
    pub edl: f32,         // eye-dome lighting strength; 0 = off
    pub _pad0: f32,       // padding
    pub _pad1: f32,       // padding
    pub origin: [f32; 2], // top-left of this target in the canvas, px
    pub frame: [f32; 2],  // canvas size, px
    pub _pad: [f32; 2],   // padding
}

const _: () = {
    assert!(std::mem::size_of::<CloudUniform>() == 48);
    assert!(std::mem::offset_of!(CloudUniform, origin) == 24);
    assert!(std::mem::offset_of!(CloudUniform, frame) == 32);
};

/// The part of the canvas a pick renders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickView {
    pub x: u32, // left edge in canvas pixels
    pub y: u32, // top edge in canvas pixels
    pub w: u32, // width, px
    pub h: u32, // height, px
}

impl PickView {
    /// A pick view covering the whole canvas.
    pub fn whole(size: (u32, u32)) -> Self {
        Self {
            x: 0,
            y: 0,
            w: size.0.max(1),
            h: size.1.max(1),
        }
    }

    /// Matrix that maps canvas clip space onto this window.
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

/// Bind group layout for the pick clip-space matrix.
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

/// Multiply two column-major 4x4 matrices.
fn mat4_mul(left: &[f32; 16], right: &[f32; 16]) -> [f32; 16] {
    let mut out = [0.0; 16];

    for col in 0..4 {
        for row in 0..4 {
            out[col * 4 + row] = (0..4).map(|k| left[k * 4 + row] * right[col * 4 + k]).sum();
        }
    }

    out
}

/// Uniform buffers and bind groups for a frame and its pick pass.
pub struct FrameUniforms {
    mvp_buffer: wgpu::Buffer,                  // camera matrix
    line_buffer: wgpu::Buffer,                 // LineUniform
    cloud_buffer: wgpu::Buffer,                // CloudUniform
    clip_buffer: wgpu::Buffer, // ClipUniform, bound beside the line and cloud blocks
    pub mvp_group: wgpu::BindGroup, // group 0
    pub line_group: wgpu::BindGroup, // group 1
    pub cloud_group: wgpu::BindGroup, // group 1 of the point lane
    pick_mvp_buffer: wgpu::Buffer, // same three blocks, for the pick window
    pick_line_buffer: wgpu::Buffer, // LineUniform for the pick
    pick_cloud_buffer: wgpu::Buffer, // CloudUniform for the pick
    pick_transform_buffer: wgpu::Buffer, // canvas-to-window matrix
    pub pick_mvp_group: wgpu::BindGroup, // group 0 for the pick
    pub pick_line_group: wgpu::BindGroup, // group 1 for the pick
    pub pick_cloud_group: wgpu::BindGroup, // point lane group 1 for the pick
    pub pick_transform_group: wgpu::BindGroup, // text lanes' pick group
    line: LineUniform,         // last written values
    cloud: CloudUniform,       // last written values
    clip: ClipUniform,         // last written clipping planes
    pub mvp_f32: [f32; 16],    // this frame's camera matrix
    pub ortho_h: f32,          // ortho half-height; 0 = perspective
    pub eye: [f32; 3],         // camera position this frame
}

impl FrameUniforms {
    /// Bind groups 0-2 for a scene draw.
    pub fn binds<'a>(&'a self, instances: &'a wgpu::BindGroup) -> Binds<'a> {
        Binds {
            mvp: &self.mvp_group,
            line: &self.line_group,
            instances,
        }
    }

    /// Bind groups 0-2 for a pick draw.
    pub fn pick_binds<'a>(&'a self, instances: &'a wgpu::BindGroup) -> Binds<'a> {
        Binds {
            mvp: &self.pick_mvp_group,
            line: &self.pick_line_group,
            instances,
        }
    }

    /// Bytes reserved on the GPU by these buffers.
    pub fn allocated_bytes(&self) -> u64 {
        self.mvp_buffer.size()
            + self.line_buffer.size()
            + self.cloud_buffer.size()
            + self.pick_mvp_buffer.size()
            + self.pick_line_buffer.size()
            + self.pick_cloud_buffer.size()
            + self.pick_transform_buffer.size()
            + self.clip_buffer.size()
    }

    /// Create the buffers and bind groups with default values.
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
        let clip = ClipUniform::default();
        let clip_buffer = uniform_buffer(&ctx.device, "clip.buffer", &clip);
        let identity = Xform::identity().to_f32();
        let pick_mvp_buffer = uniform_buffer(&ctx.device, "pick.mvp.buffer", &identity);
        let pick_line_buffer = uniform_buffer(&ctx.device, "pick.line.buffer", &line);
        let pick_cloud_buffer = uniform_buffer(&ctx.device, "pick.cloud.buffer", &cloud);
        let pick_transform_buffer = uniform_buffer(&ctx.device, "pick.transform.buffer", &identity);

        let mvp_group = bind_group(ctx, &l.mvp, "mvp.bind_group", &[&mvp_buffer]);
        let line_group = bind_group(
            ctx,
            &l.line,
            "line.bind_group",
            &[&line_buffer, &clip_buffer],
        );
        let cloud_group = bind_group(
            ctx,
            &l.line,
            "cloud.bind_group",
            &[&cloud_buffer, &clip_buffer],
        );
        let pick_mvp_group = bind_group(ctx, &l.mvp, "pick.mvp.bind_group", &[&pick_mvp_buffer]);
        let pick_line_group = bind_group(
            ctx,
            &l.line,
            "pick.line.bind_group",
            &[&pick_line_buffer, &clip_buffer],
        );
        let pick_cloud_group = bind_group(
            ctx,
            &l.line,
            "pick.cloud.bind_group",
            &[&pick_cloud_buffer, &clip_buffer],
        );
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
            clip_buffer,
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
            clip,
            mvp_f32: [0.0; 16],
            ortho_h: 0.0,
            eye: [0.0; 3],
        }
    }

    /// Write this frame's camera, pen and cloud settings.
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
            lit: if cx.view.ssao {
                2.0
            } else {
                f32::from(cx.view.lit)
            },
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

    /// Write the clipping planes when they changed; the main frame and the pick share them.
    pub fn write_clip(&mut self, ctx: &GpuCtx, clip: &ClipUniform) {
        if bytemuck::bytes_of(&self.clip) == bytemuck::bytes_of(clip) {
            return;
        }

        self.clip = *clip;
        ctx.queue
            .write_buffer(&self.clip_buffer, 0, bytemuck::bytes_of(clip));
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
        let apply = |ndc: [f32; 2]| [t[0] * ndc[0] + t[12], t[5] * ndc[1] + t[13]];
        let left_top = apply(ndc(100.0, 250.0));
        let right_bottom = apply(ndc(119.0, 269.0));
        assert!((left_top[0] + 1.0).abs() < 1e-4 && (left_top[1] - 1.0).abs() < 1e-4);
        assert!((right_bottom[0] - 1.0).abs() < 1e-4 && (right_bottom[1] + 1.0).abs() < 1e-4);
    }
}

impl super::lane::Lane for FrameUniforms {
    fn bytes(&self) -> (u64, u64) {
        (self.allocated_bytes(), 0)
    }
}

/// Most clipping planes cutting at once.
pub const MAX_PLANES: usize = 6;

/// The clipping planes as the shaders read them, 448 bytes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ClipUniform {
    // offset size
    pub planes: [[f32; 4]; MAX_PLANES], //   0   96  kept side relative to the anchor
    pub screen: [[f32; 4]; MAX_PLANES], //  96   96  the same over canvas clip space
    pub hatch: [[f32; 4]; MAX_PLANES],  // 192   96  hatch coordinate numerator over (x, y, 1)
    pub hatch_w: [[f32; 4]; MAX_PLANES], // 288   96  its denominator
    pub sides: [[f32; 4]; 2],           // 384   32  side of each plane the eye is on
    pub count: u32,                     // 416    4  planes in use
    pub samples: u32,                   // 420    4  scene samples per pixel
    pub fill: u32,                      // 424    4  0 hatch, 1 solid light grey
    pub spacing: f32,                   // 428    4  hatch spacing, px
    pub width: f32,                     // 432    4  hatch line width, px
    pub outline: f32,                   // 436    4  cut boundary width, px
    pub pad: [f32; 2],                  // 440    8  -> 448
}

const _: () = assert!(std::mem::size_of::<ClipUniform>() == 448);

/// Vertical field of view in degrees.
pub const FOVY_DEG: f64 = 60.0;
