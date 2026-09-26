// --8<-- [start:002-modules]
pub mod buffers;
pub mod device;

// --8<-- [start:004-modules]
pub mod present; // register:present
pub mod render; // register:render
// --8<-- [end:004-modules]
// --8<-- [start:003-module]
pub mod targets; // register:targets
// --8<-- [end:003-module]
pub mod view; // register:knobs

use buffers::GpuCtx;
use device::DeviceSetup;
// --8<-- [start:003-use]
use targets::Targets; // register:targets
// --8<-- [end:003-use]

// --8<-- [start:004-use]
pub use present::FrameInput; // register:present
// --8<-- [end:004-use]
// --8<-- [end:002-modules]

// --8<-- [start:002-gpu]
/// Everything on the GPU: the device, the frame and one field per lane.
pub struct Gpu {
    pub surface: Option<wgpu::Surface<'static>>, // the canvas; None when headless
    pub ctx: GpuCtx,                             // device and queue
    pub config: wgpu::SurfaceConfiguration,      // canvas size and format
// --8<-- [start:003-field]
    pub targets: Targets,                        // depth and color textures; register:targets
// --8<-- [end:003-field]
    pub logical_size: [f64; 2],                  // canvas size in CSS pixels
    pub failure: std::sync::Arc<std::sync::Mutex<Option<String>>>, // first GPU error
}
// --8<-- [end:002-gpu]

// --8<-- [start:002-open]
impl Gpu {

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
            failure,
        } = device::open(window, size).await?;
        let ctx = GpuCtx::new(device, queue);
        let size = (config.width, config.height);
// --8<-- [start:003-build]
        // start without MSAA; retarget flips it later
        let samples = 1; // register:targets
        let targets = Targets::new(&ctx, size, config.format, samples); // register:targets
// --8<-- [end:003-build]

        log::info!(
            "viewer init OK - surface {}x{}, format {:?}",
            config.width,
            config.height,
            config.format
        );
        let mut gpu = Self {
            surface,
            ctx,
            config,
// --8<-- [start:003-init]
            targets,                // register:targets
// --8<-- [end:003-init]
            logical_size: [size.0 as f64, size.1 as f64],
            failure,
        };
        Ok(gpu)
    }

}
// --8<-- [end:002-open]
