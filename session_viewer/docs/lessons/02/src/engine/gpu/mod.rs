pub mod backdrop; // register:backdrop
pub mod buffers; // register:buffers
pub mod device; // register:device
pub mod frame; // register:frame
pub mod hull; // register:hull
pub mod instance; // register:instance
pub mod objects;

pub mod lane; // register:lane
pub mod pass; // register:pass
pub mod present; // register:present
pub mod render; // register:render
pub mod targets; // register:targets
pub mod upload; // register:upload
pub mod view; // register:view

use crate::engine::performance::Performance;
use crate::engine::pipelines::{Layouts, Target};
use session_rust::{AABB, Point};

use backdrop::BackdropLane;
use buffers::GpuCtx;
use device::DeviceSetup;
use frame::FrameUniforms;
use lane::{Lane, RowLane};
use objects::InstanceTable;
use pass::Pass;
use targets::Targets;

pub use frame::FrameInput;
pub use instance::Instance;
pub use objects::{ObjectRow, Rebase};
pub use upload::Upload;
pub use view::View;

/// Everything on the GPU: the device, the frame and one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // the canvas; None when headless
    pub ctx: GpuCtx,                             // device and queue
    pub config: wgpu::SurfaceConfiguration,      // canvas size and format
    pub layouts: Layouts,                        // shared bind group layouts
    pub frame: FrameUniforms,                    // per-frame uniform buffers
    pub targets: Targets,                        // depth and color textures
    pub view: View,                              // display settings
    pub objects: InstanceTable,                  // one row per object
    pub backdrop: BackdropLane,                  // background and grid
    pub selection_revision: u64,                 // bumps on every selection change
    pub logical_size: [f64; 2],                  // canvas size in CSS pixels
    registered: Vec<Box<dyn RowLane>>,           // lanes from lane::REGISTRY
    passes: Vec<Box<dyn Pass>>,                  // passes from pass::PASSES, in frame order
    pub performance: Performance, // frame timing
    pub bounds: AABB,                     // world box of everything uploaded
    device_type: wgpu::DeviceType,        // discrete, integrated or CPU
    pub failure: std::sync::Arc<std::sync::Mutex<Option<String>>>, // first GPU error
}

/// Every lane, once. Adding one means writing its `Lane` impl and one line here.
macro_rules! lane_list {
    ($apply:ident, $g:ident) => {
        $apply!($g;
            frame,             // register:frame
            objects,
            backdrop,          // register:backdrop
        )
    };
}

/// The lanes, shared.
macro_rules! shared {
    ($g:ident; $($lane:ident),* $(,)?) => { [$(&$g.$lane as &dyn Lane),*] };
}

/// The lanes, mutable. Each field is borrowed once, so `ctx` stays free.
macro_rules! owned {
    ($g:ident; $($lane:ident),* $(,)?) => { [$(&mut $g.$lane as &mut dyn Lane),*] };
}

impl Gpu {
    /// Bytes reserved on the GPU: (buffers, textures).
    pub fn allocated_bytes(&self) -> (u64, u64) {
        let mut buffers = 0;
        let mut textures = 0;
        for lane in lane_list!(shared, self) {
            let (b, t) = lane.bytes();
            buffers += b;
            textures += t;
        }
        for lane in &self.registered {
            let (b, t) = lane.bytes();
            buffers += b;
            textures += t;
        }
        for pass in &self.passes {
            let (b, t) = pass.bytes();
            buffers += b;
            textures += t;
        }
        let pixels = u64::from(self.config.width) * u64::from(self.config.height);
        let samples = u64::from(self.targets.samples);
        // per sample: 4 color + 4 depth + 4 triangle id; at 1x no color copy
        let frame_textures =
            pixels * if samples > 1 { samples * 12 } else { 8 } + if samples > 1 { 8 } else { 32 };
        (buffers, textures + frame_textures)
    }

    /// Open the GPU for a window.
    pub async fn new(window: std::sync::Arc<winit::window::Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        Self::build(Some(window), (size.width, size.height)).await
    }

    /// Open the GPU with no window, drawing into a texture.
    pub async fn new_headless(width: u32, height: u32) -> anyhow::Result<Self> {
        Self::build(None, (width, height)).await
    }

    /// Open the device and create every lane, empty.
    async fn build(
        window: Option<std::sync::Arc<winit::window::Window>>,
        size: (u32, u32),
    ) -> anyhow::Result<Self> {
        let DeviceSetup {
            surface,
            device,
            queue,
            config,
            device_type,
            failure,
        } = device::open(window, size).await?;
        crate::engine::performance::mark("device ready");
        let ctx = GpuCtx::new(device, queue);
        let size = (config.width, config.height);
        // start without MSAA; retarget flips it later
        let target = Target {
            format: config.format,
            samples: 1,
        };

        let layouts = Layouts::new(&ctx.device);
        let frame = FrameUniforms::new(&ctx, &layouts, size);
        let targets = Targets::new(&ctx, size, config.format, target.samples);
        let objects = InstanceTable::new(&ctx, &layouts);
        let backdrop = BackdropLane::new(&ctx, &layouts, target);
        let registered = lane::REGISTRY
            .iter()
            .map(|lane| (lane.make)(&ctx, &layouts, target))
            .collect();
        let passes = pass::PASSES.iter().map(|make| make(&ctx, target)).collect();

        log::info!(
            "viewer init OK - surface {}x{}, format {:?}",
            config.width,
            config.height,
            config.format
        );
        crate::engine::performance::mark("gpu built");
        let mut gpu = Self {
            surface,
            ctx,
            config,
            layouts,
            frame,
            targets,
            view: View::from_env(),
            objects,
            backdrop,
            selection_revision: 0,
            logical_size: [size.0 as f64, size.1 as f64],
            registered,
            passes,
            performance: Performance::new(),
            bounds: AABB::empty(),
            device_type,
            failure,
        };
        Ok(gpu)
    }

    /// Append one upload to every lane.
    pub fn set_scene(&mut self, up: &Upload) {
        self.objects.append(&self.ctx, &self.layouts, &up.obj);
        for lane in &mut self.registered {
            lane.on_append(&self.ctx, &self.layouts, up);
        }

        self.bounds.union_with(&up.bounds);
        self.retarget(false);
    }

    /// Current color format and sample count.
    pub(super) fn target(&self) -> Target {
        Target {
            format: self.config.format,
            samples: self.targets.samples,
        }
    }

    /// Remake targets and pipelines when the sample count changes.
    fn retarget(&mut self, resized: bool) {
        let samples = self.msaa_now();
        let flip = samples != self.targets.samples;

        if flip || resized {
            self.targets.destroy();
            self.targets = Targets::new(
                &self.ctx,
                (self.config.width, self.config.height),
                self.config.format,
                samples,
            );
        }

        if flip {
            self.rebuild_pipelines();
            log::info!("msaa: {}x", samples);
        }
    }

    /// Every lane builds its pipelines again for the current target; the cache compiles only new ones.
    fn rebuild_pipelines(&mut self) {
        let target = self.target();
        let ctx = &self.ctx;
        let layouts = &self.layouts;
        for lane in lane_list!(owned, self) {
            lane.on_retarget(ctx, layouts, target);
        }
        for lane in &mut self.registered {
            lane.on_retarget(ctx, layouts, target);
        }
        for pass in &mut self.passes {
            pass.on_retarget(ctx, layouts, target);
        }
    }

    /// Pixels this GPU can afford at 4x MSAA.
    pub fn msaa_budget(&self) -> Option<u32> {
        Targets::msaa_budget(self.device_type)
    }

    /// Pick the MSAA sample count again, after live rows came or went.
    pub(crate) fn refresh_samples(&mut self) {
        self.retarget(false);
    }

    /// MSAA samples for the current scene: 4x only with solid geometry.
    fn msaa_now(&self) -> u32 {
        let mut solid = false;
        Targets::samples_for(
            solid,
            self.config.width * self.config.height,
            self.view.msaa_forced,
            self.msaa_budget(),
            self.config.width as f32 / self.logical_size[0].max(1.0) as f32,
        )
    }

    /// Resize the canvas and every texture that follows it.
    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        if let Some(s) = &self.surface {
            s.configure(&self.ctx.device, &self.config);
        }

        self.retarget(true);
    }

    /// Forget every row; keep the buffers.
    pub fn reset(&mut self) {
        let ctx = &self.ctx;
        for lane in lane_list!(owned, self) {
            lane.on_reset(ctx);
        }
        for lane in &mut self.registered {
            lane.on_reset(ctx);
        }
        for pass in &mut self.passes {
            pass.on_reset(ctx);
        }
        self.bounds = AABB::empty();
    }

    /// Forget every row and free the buffers.
    pub fn release(&mut self) {
        let ctx = &self.ctx;
        let layouts = &self.layouts;
        for lane in lane_list!(owned, self) {
            lane.on_release(ctx, layouts);
        }
        for lane in &mut self.registered {
            lane.on_release(ctx, layouts);
        }
        for pass in &mut self.passes {
            pass.on_release(ctx, layouts);
        }
        self.bounds = AABB::empty();
        self.retarget(false);
    }
}

/// Every lane's shader sources, for the tests.
#[cfg(test)]
pub(crate) fn lane_shaders() -> Vec<(&'static str, &'static str)> {
    let mut out = Vec::new();
    out.extend_from_slice(backdrop::SHADERS);
    out
}
