use std::sync::Arc;
use winit::window::Window;

/// Everything `open` set up.
pub struct DeviceSetup {
    pub surface: Option<wgpu::Surface<'static>>, // the canvas; None when headless
    pub device: wgpu::Device,                    // creates GPU resources
    pub queue: wgpu::Queue,                      // runs GPU commands
    pub config: wgpu::SurfaceConfiguration,      // size and format of the canvas
    pub device_type: wgpu::DeviceType,           // discrete, integrated or CPU
    pub failure: Arc<std::sync::Mutex<Option<String>>>, // first GPU error, read each frame
}

/// The APIs the instance opens: WebGPU in the browser, native APIs otherwise.
const BACKENDS: wgpu::Backends = if cfg!(target_arch = "wasm32") {
    wgpu::Backends::BROWSER_WEBGPU
} else {
    wgpu::Backends::PRIMARY
};

/// Open the GPU: instance, surface, adapter, device, surface config.
pub async fn open(window: Option<Arc<Window>>, size: (u32, u32)) -> anyhow::Result<DeviceSetup> {
    let (surface, adapter) = surface_and_adapter(window.as_ref()).await?;
    let info = adapter.get_info();
    crate::app::feedback::diagnostic("adapter", &format!("{info:?}"));
    log::info!(
        "adapter: {} ({:?}, {:?})",
        info.name,
        info.device_type,
        info.backend
    );

    if info.device_type == wgpu::DeviceType::Cpu {
        log::warn!("software adapter - rendering on the CPU will be slow");
    }

    // allow storage buffers up to 256 MiB
    let limits = wgpu::Limits {
        max_storage_buffer_binding_size: adapter
            .limits()
            .max_storage_buffer_binding_size
            .min(256 * 1024 * 1024),
        ..wgpu::Limits::default()
    };

    // natively, `VIEWER_GPU_TIMING` asks for timestamps between passes
    let timing =
        !cfg!(target_arch = "wasm32") && super::view::knob("VIEWER_GPU_TIMING", "").is_some();
    let timestamps =
        wgpu::Features::TIMESTAMP_QUERY | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS;
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: if timing {
                adapter.features() & timestamps
            } else {
                wgpu::Features::empty()
            },
            required_limits: limits,
            memory_hints: Default::default(),
            ..Default::default()
        })
        .await?;
    // browser errors are stored, not thrown
    let failure = Arc::new(std::sync::Mutex::new(None));
    #[cfg(target_arch = "wasm32")]
    {
        let errors = failure.clone();
        let redraw = window.clone();
        device.on_uncaptured_error(Arc::new(move |error| {
            remember_gpu_error(&errors, error);
            if let Some(window) = &redraw {
                window.request_redraw();
            }
        }));
        let lost = failure.clone();
        // ask for a frame so the loss is seen
        let redraw = window.clone();
        device.set_device_lost_callback(move |reason, message| {
            remember_device_loss(&lost, reason, &message);

            if let Some(window) = &redraw {
                window.request_redraw();
            }
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    device.on_uncaptured_error(Arc::new(report_gpu_error));

    // prefer an sRGB canvas format
    let (format, present_mode, alpha_mode) = match &surface {
        Some(s) => {
            let caps = s.get_capabilities(&adapter);
            let mut f = caps.formats[0];

            for format in &caps.formats {
                if format.is_srgb() {
                    f = *format;
                    break;
                }
            }

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

    if let Some(s) = &surface {
        // size the surface now
        s.configure(&device, &config);
    }

    Ok(DeviceSetup {
        surface,
        device,
        queue,
        config,
        device_type: info.device_type,
        failure,
    })
}

/// A fresh instance of the backends.
fn new_instance() -> wgpu::Instance {
    wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: BACKENDS,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    })
}

/// The window's drawing surface and the GPU to draw it; natively without a window, the process's shared adapter.
async fn surface_and_adapter(
    window: Option<&Arc<Window>>,
) -> anyhow::Result<(Option<wgpu::Surface<'static>>, wgpu::Adapter)> {
    #[cfg(not(target_arch = "wasm32"))]
    if window.is_none() {
        return Ok((None, shared_adapter()?));
    }

    let instance = new_instance();
    let surface = match window {
        Some(w) => Some(instance.create_surface(w.clone())?),
        None => None,
    };
    let adapter = choose_adapter(&instance, surface.as_ref()).await?;
    Ok((surface, adapter))
}

/// Natively, the adapter every headless GPU of the process opens its device on, chosen once from one instance the adapter keeps alive.
/// The Vulkan loader unloads the drivers without a device while it enumerates adapters, without the lock its object-naming calls take,
/// so an instance enumerating while another thread's device names an object crashes inside libvulkan: parallel GPU tests did.
#[cfg(not(target_arch = "wasm32"))]
fn shared_adapter() -> anyhow::Result<wgpu::Adapter> {
    static SHARED: std::sync::OnceLock<Result<wgpu::Adapter, String>> = std::sync::OnceLock::new();

    SHARED
        .get_or_init(|| {
            pollster::block_on(choose_adapter(&new_instance(), None)).map_err(|error| error.to_string())
        })
        .clone()
        .map_err(anyhow::Error::msg)
}

/// The GPU: named by `VIEWER_ADAPTER`, else the preferred power, else the default.
async fn choose_adapter(
    instance: &wgpu::Instance,
    surface: Option<&wgpu::Surface<'static>>,
) -> anyhow::Result<wgpu::Adapter> {
    // browser picks the GPU; native prefers the low-power one
    let default_power = if cfg!(target_arch = "wasm32") {
        wgpu::PowerPreference::None
    } else {
        wgpu::PowerPreference::LowPower
    };
    // `?gpu=high` asks for the fast GPU
    let preferred = if super::view::knob("VIEWER_GPU", "gpu").as_deref() == Some("high") {
        wgpu::PowerPreference::HighPerformance
    } else {
        default_power
    };
    let options = |power_preference| wgpu::RequestAdapterOptions {
        power_preference,
        compatible_surface: surface,
        force_fallback_adapter: false,
    };

    if let Some(named) = named_adapter(instance, BACKENDS).await {
        return Ok(named);
    }

    match instance.request_adapter(&options(preferred)).await {
        Ok(adapter) => Ok(adapter),
        Err(_) if preferred != default_power => Ok(instance.request_adapter(&options(default_power)).await?),
        Err(error) => Err(error.into()),
    }
}

/// Pick the native GPU named by `VIEWER_ADAPTER`, if any.
async fn named_adapter(
    instance: &wgpu::Instance,
    backends: wgpu::Backends,
) -> Option<wgpu::Adapter> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (instance, backends);
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let want = std::env::var("VIEWER_ADAPTER").ok()?.to_lowercase();
        let mut selected = None;

        for adapter in instance.enumerate_adapters(backends).await {
            if adapter.get_info().name.to_lowercase().contains(&want) {
                selected = Some(adapter);
                break;
            }
        }

        selected
    }
}

/// Store the first error message; later ones are ignored.
#[cfg(any(target_arch = "wasm32", test))]
fn remember_failure(failure: &std::sync::Mutex<Option<String>>, message: String) {
    if let Ok(mut state) = failure.lock() {
        if state.is_none() {
            crate::app::feedback::diagnostic("fatal", &message);
            *state = Some(message);
        }
    }
}

/// Native: a GPU error stops the program.
#[cfg(not(target_arch = "wasm32"))]
fn report_gpu_error(e: wgpu::Error) {
    panic!("wgpu: {e}");
}

/// A broken shader must reach the error callback.
#[cfg(all(test, not(target_arch = "wasm32")))]
#[test]
#[ignore = "requires a native GPU adapter"]
#[should_panic(expected = "wgpu: Validation Error")]
fn invalid_gpu_shader_is_fatal() {
    let setup = pollster::block_on(open(None, (1, 1))).expect("open native adapter");
    let _ = setup
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("intentional verification failure"),
            source: wgpu::ShaderSource::Wgsl(
                "@compute @workgroup_size(1) fn main() { let broken: u32 = true; }".into(),
            ),
        });
}

/// Store a browser GPU error.
#[cfg(target_arch = "wasm32")]
fn remember_gpu_error(failure: &std::sync::Mutex<Option<String>>, error: wgpu::Error) {
    remember_failure(failure, format!("WebGPU error: {error}"));
}

/// Store a browser device-loss reason.
#[cfg(target_arch = "wasm32")]
fn remember_device_loss(
    failure: &std::sync::Mutex<Option<String>>,
    reason: wgpu::DeviceLostReason,
    message: &str,
) {
    remember_failure(
        failure,
        format!("WebGPU device lost ({reason:?}): {message}"),
    );
}

#[cfg(test)]
#[test]
/// The first stored error stays when more arrive.
fn first_gpu_error_survives_follow_on_submission_errors() {
    let failure = std::sync::Mutex::new(None);
    remember_failure(&failure, "texture allocation failed".into());
    remember_failure(&failure, "invalid command buffer".into());
    assert_eq!(
        failure.lock().unwrap().as_deref(),
        Some("texture allocation failed")
    );
}
