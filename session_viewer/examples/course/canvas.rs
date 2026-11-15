// --8<-- [start:course-canvas]
use crate::engine::gpu::device::{self, DeviceSetup};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoopProxy},
    platform::web::WindowAttributesExtWebSys,
    window::{Window, WindowId},
};

pub trait Drawing {
    fn event(&mut self, _event: &WindowEvent, _now: f64) {}
    fn draw(&mut self, gpu: &DeviceSetup, view: &wgpu::TextureView, scale: f32, now: f64) -> bool;
}

type Factory = fn(&Arc<Window>, &DeviceSetup) -> Result<Box<dyn Drawing>, String>;
type Ready = Result<(Arc<Window>, DeviceSetup), String>;

struct Canvas {
    factory: Factory,
    proxy: Option<EventLoopProxy<Ready>>,
    active: Option<(Arc<Window>, DeviceSetup, Box<dyn Drawing>)>,
    presented: bool,
}

pub fn run(factory: Factory) -> Result<(), wasm_bindgen::JsValue> {
    crate::app::launch::run(|proxy| Canvas {
        factory,
        proxy: Some(proxy),
        active: None,
        presented: false,
    })
    .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))
}

fn report(message: &str) {
    web_sys::console::error_1(&message.into());
    if let Some(canvas) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("canvas"))
    {
        let _ = canvas.set_attribute("data-course-error", message);
    }
}

pub(super) fn now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map_or(0.0, |p| p.now())
}

impl ApplicationHandler<Ready> for Canvas {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let Some(proxy) = self.proxy.take() else {
            return;
        };
        let Some(canvas) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("canvas"))
            .and_then(|element| element.dyn_into::<web_sys::HtmlCanvasElement>().ok())
        else {
            report("The lesson needs a canvas with id=canvas");
            return;
        };
        // The DOM layout exists before winit's first resize notification.
        let scale = web_sys::window().unwrap().device_pixel_ratio();
        let size = (
            (canvas.client_width() as f64 * scale).round().max(1.0) as u32,
            (canvas.client_height() as f64 * scale).round().max(1.0) as u32,
        );
        let window = match event_loop
            .create_window(Window::default_attributes().with_canvas(Some(canvas)))
        {
            Ok(window) => Arc::new(window),
            Err(error) => {
                report(&error.to_string());
                return;
            }
        };
        wasm_bindgen_futures::spawn_local(async move {
            let options = device::Options {
                diagnostic: |kind, text| {
                    if kind == "fatal" {
                        report(text)
                    }
                },
                ..Default::default()
            };
            let ready = device::open(Some(window.clone()), size, options)
                .await
                .map(|gpu| (window, gpu))
                .map_err(|error| error.to_string());
            let _ = proxy.send_event(ready);
        });
    }

    fn user_event(&mut self, _: &ActiveEventLoop, ready: Ready) {
        let ready = ready.and_then(|(window, gpu)| {
            let drawing = (self.factory)(&window, &gpu)?;
            Ok((window, gpu, drawing))
        });
        match ready {
            Ok(active) => {
                active.0.request_redraw();
                self.active = Some(active);
            }
            Err(error) => report(&error),
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let Some((window, gpu, drawing)) = self.active.as_mut() else {
            return;
        };
        if gpu.failure.lock().unwrap().is_some() {
            return;
        }
        drawing.event(&event, now());
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                let limit = gpu.device.limits().max_texture_dimension_2d;
                let width = size.width.min(limit);
                let height = size.height.min(limit);
                if (width, height) == (gpu.config.width, gpu.config.height) {
                    return;
                }
                gpu.config.width = width;
                gpu.config.height = height;
                gpu.surface
                    .as_ref()
                    .unwrap()
                    .configure(&gpu.device, &gpu.config);
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let surface = gpu.surface.as_ref().unwrap();
                let output = match surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(frame)
                    | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
                    _ => {
                        surface.configure(&gpu.device, &gpu.config);
                        window.request_redraw();
                        return;
                    }
                };
                let view = output.texture.create_view(&Default::default());
                let more = drawing.draw(gpu, &view, window.scale_factor() as f32, now());
                output.present();
                if !self.presented {
                    if let Some(canvas) = web_sys::window()
                        .and_then(|w| w.document())
                        .and_then(|d| d.get_element_by_id("canvas"))
                    {
                        let _ = canvas.set_attribute("data-course-presented", "true");
                    }
                    self.presented = true;
                }
                if more {
                    window.request_redraw()
                }
            }
            _ => window.request_redraw(),
        }
    }
}
// --8<-- [end:course-canvas]
