//! Device negotiation: instance -> surface -> adapter -> device + queue -> surface format.
//! Produces one `DeviceSetup` and owns nothing afterwards: no buffer, no pipeline, no frame
//! state. Headless callers pass no window and get no surface.

use std::sync::Arc;
use winit::window::Window;

/// What `open` negotiated: the surface (None when headless), the device/queue pair, and the
/// surface configuration it was configured with.
pub struct DeviceSetup {
    pub surface: Option<wgpu::Surface<'static>>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
}

/// Set up the wgpu objects in order: Instance -> Surface -> Adapter -> Device + Queue -> configure.
/// `size` is the canvas in pixels; a zero side is clamped to 1 so the surface can be configured.
pub async fn open(window: Option<Arc<Window>>, size: (u32, u32)) -> anyhow::Result<DeviceSetup> {
    // 1. Instance — the driver entry point. WebGPU only in the browser, never WebGL.
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: if cfg!(target_arch = "wasm32") {
            wgpu::Backends::BROWSER_WEBGPU
        } else {
            wgpu::Backends::PRIMARY //Vulkan / Metal / DX12 for native selftest
        },
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });

    // 2. Surface — the drawable canvas. 3. Adapter — a physical GPU compatible with it.
    let surface = match &window { Some(w) => Some(instance.create_surface(w.clone())?), None => None };
    // LowPower = the iGPU the compositor runs on. On hybrid laptops the discrete GPU renders
    // fine but its frames can't be shared to the compositor - the canvas stays black.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: surface.as_ref(),
            force_fallback_adapter: false,
        })
        .await?;
    let info = adapter.get_info();
    log::info!("adapter: {} ({:?}, {:?})", info.name, info.device_type, info.backend);
    if info.device_type == wgpu::DeviceType::Cpu {
        log::warn!("software adapter - rendering on the CPU will be slow");
    }

    // Limit to 128 mb, then the flat merge becomes the grid
    let mut limits = wgpu::Limits::default();
    let hw = adapter.limits();
    limits.max_storage_buffer_binding_size = hw.max_storage_buffer_binding_size;
    limits.max_buffer_size = hw.max_buffer_size;

    // 4. Device (creates resources) + Queue (submits work).
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: limits,  // unlock the WEBGpu storage buffers
            memory_hints: Default::default(),
            ..Default::default()
        })
        .await?;

    device.on_uncaptured_error(Arc::new(|e|{ log::error!("wgpu on_uncaptured_error: {e}") }));

    // 5. Configure the surface: pixel format (prefer sRGB), size, vsync.
    // Headless has no capabilities to ask, so pick the format the readback path wants.
    let (format, present_mode, alpha_mode) = match &surface {
        Some(s) => {
            let caps = s.get_capabilities(&adapter);
            let f = caps.formats.iter().find(|f| f.is_srgb()).copied().unwrap_or(caps.formats[0]);
            (f, caps.present_modes[0], caps.alpha_modes[0])
        }
        None => (
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::PresentMode::Fifo,
            wgpu::CompositeAlphaMode::Auto,
        ),
    };
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.0.max(1),
        height: size.1.max(1),
        present_mode,
        alpha_mode,
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    if let Some(s) = &surface { s.configure(&device, &config); }

    Ok(DeviceSetup { surface, device, queue, config })
}
