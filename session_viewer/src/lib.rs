// ============================================================
// IMPORTS
// ============================================================
// Arc = Atomically Reference Counted smart pointer.
// Lets multiple owners share the same heap value across async code.
// We use it so the Window can be owned by both the event loop and our State.
use std::sync::Arc;
use std::iter;

// wasm_bindgen is the bridge between Rust and JavaScript.
// `prelude::*` imports the most commonly used items:
//   - #[wasm_bindgen] attribute macro
//   - JsValue (JavaScript value type)
//   - UnwrapThrowExt (.unwrap_throw() = like .unwrap() but sends the panic to JS console)
use wasm_bindgen::prelude::*;

// JsCast lets us cast JavaScript objects to specific types.
// e.g. cast a generic Element to HtmlCanvasElement.
use wasm_bindgen::JsCast;

// winit is the cross-platform window/event library.
// It handles: creating windows, receiving mouse/keyboard events, the event loop.
use winit::{
    // ApplicationHandler is a trait we implement to tell winit how to run our app.
    application::ApplicationHandler,
    // event::* gives us WindowEvent, KeyEvent, ElementState etc.
    event::*,
    // ActiveEventLoop: used inside event callbacks to control the loop (exit, create windows).
    // EventLoop: the main loop that drives the whole application.
    event_loop::{ActiveEventLoop, EventLoop},
    // KeyCode: physical key identifiers (e.g. KeyCode::Escape).
    // PhysicalKey: wrapper around KeyCode for physical keyboard keys.
    keyboard::{KeyCode, PhysicalKey},
    // EventLoopExtWebSys: adds .spawn_app() to EventLoop — required on web because
    //   the browser already has its own event loop; we can't block it with .run().
    // WindowAttributesExtWebSys: adds .with_canvas() to WindowAttributes so winit
    //   uses our HTML <canvas> element instead of creating a new OS window.
    platform::web::{EventLoopExtWebSys, WindowAttributesExtWebSys},
    // Window: the actual OS/browser window handle.
    window::Window,
};

// ============================================================
// STATE
// ============================================================
// State holds all GPU resources needed to render a frame.
// Right now it only has a Window — in future steps we will add:
//   - wgpu::Surface   (the drawable surface backed by the canvas)
//   - wgpu::Device    (the logical GPU — used to create buffers, textures, pipelines)
//   - wgpu::Queue     (sends commands to the GPU)
//
/// This will store the state of our application.
///
/// - `Arc`: https://doc.rust-lang.org/std/sync/struct.Arc.html
/// - `winit::window::Window`: https://docs.rs/winit/0.30/winit/window/struct.Window.html
pub struct State {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    is_surface_configured: bool,
    // Current background clear color — updated by mouse position.
    mouse_position: (f64, f64),
    clear_color: wgpu::Color,
}

impl State {
    /// Create a new `State`.
    // This is async because initializing a wgpu Surface/Device/Queue is async on web.
    // anyhow::Result lets us return any error type with `?` without defining our own error enum.
    //
    /// - `anyhow::Result`: https://docs.rs/anyhow/1.0/anyhow/type.Result.html
    /// - async fn: https://doc.rust-lang.org/std/keyword.async.html
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU.
        // Use: get information about graphics card, name, use of backend.
        // We use it to create Device and Surface.
        // On web we only use BROWSER_WEBGPU (native WebGPU) with GL (WebGL2) as fallback.
        // No Vulkan/Metal/DX12 — those are desktop-only backends.
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,
        });

        // Create the surface from the window (which is already backed by our HTML canvas).
        // A Surface is the thing you draw onto - it represents the connection between wgpu and html canvas.
        let surface = instance.create_surface(window.clone()).unwrap();


        // Request an adapter — a handle to the actual GPU.
        // No pollster::block_on — we just .await directly since new() is already async.
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(), // LowPower or HighPerformance
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {                                  
                label: None,                                            
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                memory_hints: Default::default(),
                ..Default::default()
            })
            .await?;

        // Enume how to sync rhe surface with the display.
        let surface_caps = surface.get_capabilities(&adapter);

        // Shader code in this tutorial assumes an sRGB surface texture.
        // Using a different one will result in all the color coming out darker.
        // If you want to support non-sRGB surfaces, you will need to change that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
    
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT, // Texture will be used to write to the screen.
            format: surface_format, // How surface textures should be stored in memory (e.g. Rgba8UnormSrgb).
            width: size.width, // Size of texture is equal to the window size, that will be resized later.
            height: size.height,
            present_mode: surface_caps.present_modes[0], // If you do not want runtime selection PresentMode::Fifo will cap the display rate at the display's framerate: https://docs.rs/wgpu/latest/wgpu/enum.PresentMode.html
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![], // A list of TextureFormats that you can use when create TextureViews.
            desired_maximum_frame_latency: 2,
        };


        // Initialize all the fields of our State struct and return it.
        Ok(Self { 
            surface, 
            device, 
            queue, 
            config, 
            is_surface_configured: false,
            clear_color: wgpu::Color { r: 0.9, g: 0.9, b: 0.9, a: 1.0 },
            window,
            mouse_position: (0.0, 0.0),
        })

    }

    /// Called when the window is resized.
    // We will use width/height later to resize the wgpu Surface.
    // The underscore prefix (_width, _height) tells Rust we know about these
    // parameters but are intentionally not using them yet.
    //
    /// - Resize event: https://docs.rs/winit/0.30/winit/event/enum.WindowEvent.html#variant.Resized
    pub fn resize(&mut self, width: u32, height: u32) {
        // Zero-sized surfaces are invalid, so we only resize if both dimensions are > 0.
        // Note the max supported WebGL is 2048 px. If you have larger display cap the size:
        //   let max = 2048;
        //   self.config.width = width.min(max);
        //   self.config.height = height.min(max);
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;
        }
    }

    pub fn update(&mut self) {
        // Update game logic, animations, etc. here.
        // This is called once per frame before render().
    }

    /// Request the window to redraw.
    // In a wgpu app the render loop is not automatic — we must ask the OS/browser
    // to call us back for the next frame by calling request_redraw().
    // This triggers a WindowEvent::RedrawRequested in the next event loop tick.
    //
    /// - `Window::request_redraw`: https://docs.rs/winit/0.30/winit/window/struct.Window.html#method.request_redraw
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        // We cannot render unless the surface is configured.
        if !self.is_surface_configured {
            return Ok(());
        }

        // get_current_texture() returns CurrentSurfaceTexture enum in wgpu 29.
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            // Surface slightly out of sync — reconfigure and use frame anyway.
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                self.surface.configure(&self.device, &self.config);
                texture
            }
            // Surface lost or outdated (e.g. resize race) — reconfigure and skip frame.
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            // Frame took too long, hidden, or validation error — skip frame.
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
            // GPU lost — unrecoverable.
            wgpu::CurrentSurfaceTexture::Lost => {
                anyhow::bail!("Lost GPU device");
            }
        };

        // We must create a TextureView to control how the render code interacts with the texture.
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // We also need to create a CommandEncoder to create the actual commands to send to the GPU.
        // Most frameworks expect commannds to be stored in a command buffer before being send to GPU.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Now we can actuall clear the screen.
        // We need encoder to create a RenderPass
        // We enclose this with curly braces because render_pass borrows encoder mutably.
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }

        // Tell wgpu to finish the command buffer and submit to the GPU's render queue.
        self.queue.submit(iter::once(encoder.finish()));
        output.present();


        Ok(())
    }


    /// Update clear color based on mouse position.
    // r = x / width, g = y / height — maps mouse coords to [0, 1] color range.
    pub fn handle_mouse_moved(&mut self, x: f64, y: f64) {
         self.mouse_position = (x, y);
    }

    /// Handle keyboard input.
    pub fn handle_key(&self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        //   For a web viewer it makes more sense to use Escape for
        //   things like:
        //   - Deselecting objects
        //   - Closing a modal/panel
        //   - Exiting fullscreen mode (document.exitFullscreen())
        match (code, is_pressed) {
            (KeyCode::Escape, true) => event_loop.exit(),
            _ => {}
        }
    }

}

// ============================================================
// APP
// ============================================================
// App is the struct we hand to the winit event loop.
// It owns the State (once created) and the proxy used to send State from async code.
//
// Why Option<State>?
//   State is created asynchronously inside `resumed`. It doesn't exist yet when
//   App is constructed, so we store it as Option and fill it in later.
//
// Why proxy?
//   `resumed` is not async, but State::new() is. On web we can't block, so we
//   spawn an async task and use the proxy to send the finished State back into
//   the event loop via user_event().
//
// We need to tell winit how to use our `State` struct as the application state.
// The state variable stores State struct as an option.
pub struct App {
    proxy: Option<winit::event_loop::EventLoopProxy<State>>,
    state: Option<State>,
}

impl App {
    pub fn new(event_loop: &EventLoop<State>) -> Self {
        Self {
            // create_proxy() gives us a handle we can use to send custom events
            // (our State) back into the event loop from an async context.
            proxy: Some(event_loop.create_proxy()),
            state: None,
        }
    }

    // To run all the application we use this as the main entry point.
    pub fn run() -> anyhow::Result<()> {
        // Route Rust log:: calls to the browser's console (console.log / console.error).
        // Without this, wgpu errors would be silently swallowed.
        console_log::init_with_level(log::Level::Info).unwrap_throw();

        // Build the winit event loop. `with_user_event()` means the loop can also
        // receive our custom event type (State) via the proxy — not just OS events.
        let event_loop = EventLoop::with_user_event().build()?;

        let app = App::new(&event_loop);

        // spawn_app() hands control to the browser's existing RAF (requestAnimationFrame) loop.
        // On native we'd use run_app() which blocks the thread — not allowed in a browser.
        event_loop.spawn_app(app);

        Ok(())
    }
}

// ============================================================
// APPLICATION HANDLER
// ============================================================
// ApplicationHandler<State> is a trait from winit.
// The generic <State> is our custom user-event type — it lets us send a fully
// constructed State through the proxy into user_event().
//
// We must implement at minimum: resumed(), window_event().
// user_event() is optional but needed because we use the proxy pattern.
impl ApplicationHandler<State> for App {

    // resumed() is called by winit when the app is ready to create its window.
    // On web this happens once the page is loaded and the browser is ready.
    //
    // It defines attributes about the window including web.
    // We use those attributes to create the window.
    // We create a future that creates our State struct.
    // On web we run the future asynchronously which sends the results to the user_event function.
    // The user_event function serves as a landing point for State future.
    // Resumed isn't async so we need to offload the future and send the results somewhere.
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        const CANVAS_ID: &str = "canvas";

        // Access the browser's window object (JavaScript `window`).
        // wgpu re-exports web_sys which gives us raw browser API bindings.
        let window = wgpu::web_sys::window().unwrap_throw();

        // Get the HTML document from the browser window.
        let document = window.document().unwrap_throw();

        // Find our <canvas id="canvas"> element in index.html.
        let canvas = document.get_element_by_id(CANVAS_ID).unwrap_throw();

        // Cast the generic Element to HtmlCanvasElement so winit can use it.
        // unchecked_into() skips the JS instanceof check — safe here because
        // we know the element is a canvas.
        let html_canvas_element = canvas.unchecked_into();

        // WindowAttributes describes how to create the window.
        // with_canvas() tells winit to render into our HTML canvas
        // instead of creating a new browser window.
        let window_attributes = Window::default_attributes()
            .with_canvas(Some(html_canvas_element));

        // Actually create the winit Window backed by our canvas.
        // Arc wraps it so we can share ownership with the async State::new() future.
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Run the future asynchronously and use the proxy to send results to the event loop.
        // proxy.take() moves the proxy out of self so we can move it into the async block.
        if let Some(proxy) = self.proxy.take() {
            // spawn_local runs a future on the browser's microtask queue (like Promise).
            // We can't use std threads in wasm, so this is how we do async work.
            wasm_bindgen_futures::spawn_local(async move {
                // Await State::new() — this is where we'll later init the wgpu GPU resources.
                // send_event() delivers the finished State to user_event() below.
                assert!(proxy
                    .send_event(
                        State::new(window)
                            .await
                            .expect("Unable to create canvas!")
                    )
                    .is_ok());
            });
        }
    }

    // user_event() is called when proxy.send_event(state) delivers our State.
    // This is the landing point after the async State::new() completes.
    // This is where proxy.send_event() ends up.
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut event: State) {
        // Immediately request the first frame and sync the size.
        event.window.request_redraw();
        event.resize(
            event.window.inner_size().width,
            event.window.inner_size().height,
        );
        // Store the fully constructed State — from here on window_event() can use it.
        self.state = Some(event);
    }

    // window_event() is called for every OS/browser event on our window:
    // mouse moves, key presses, resize, close, redraw requests, etc.
    //
    // This is where we can process events such as keyboard inputs, and mouse movements.
    // As well as other events when the window want to draw or is resized.
    // We can call the methods we defined on `State` here.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // If State hasn't been created yet (still waiting for async), ignore all events.
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            // User clicked the X button (or pressed Alt+F4 etc.).
            WindowEvent::CloseRequested => event_loop.exit(),

            // Browser window or tab was resized — tell State to resize the GPU surface.
            WindowEvent::Resized(size) => state.resize(size.width, size.height),

            // The browser is ready for us to draw a new frame.
            WindowEvent::RedrawRequested => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(e) => {
                        // Log the error and exit gracefully
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }

            // A physical key was pressed or released.
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(code),
                    state: key_state,
                    ..  // ignore repeat, text, location, etc.
                },
                ..  // ignore device_id, is_synthetic
            } => state.handle_key(event_loop, code, key_state.is_pressed()),

            // Mouse moved — update the clear color based on cursor position.
            WindowEvent::CursorMoved { position, .. } => {
                state.handle_mouse_moved(position.x, position.y);
            }

            // Ignore all other events (scroll, focus, etc.) for now.
            _ => {}
        }
    }
}

// ============================================================
// WASM ENTRY POINT
// ============================================================
// #[wasm_bindgen(start)] marks this function as the entry point called
// automatically by the browser when the WASM module is loaded.
// It replaces `fn main()` for web targets.
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_bindgen::JsValue> {
    // Redirect Rust panics to the browser console with a readable stack trace.
    // Without this, a panic just shows "unreachable" in the browser with no info.
    console_error_panic_hook::set_once();

    // Start the application. unwrap_throw() converts a Rust panic into a JS exception.
    App::run().map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}
