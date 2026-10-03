use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

fn error(value: impl ToString) -> JsValue { JsValue::from_str(&value.to_string()) }

pub async fn run() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document.get_element_by_id("canvas").ok_or("No canvas")?.dyn_into()?;
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone())).map_err(error)?;
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface), ..Default::default()
    }).await.map_err(error)?;
    let (device, queue) = adapter.request_device(&Default::default()).await.map_err(error)?;
    let width = window.inner_width()?.as_f64().ok_or("No width")? as u32;
    let height = window.inner_height()?.as_f64().ok_or("No height")? as u32;
    canvas.set_width(width); canvas.set_height(height);
    let config = surface.get_default_config(&adapter, width, height).ok_or("No surface format")?;
    surface.configure(&device, &config);
    let renderer = Renderer::new(device, queue);
    let frame = match surface.get_current_texture() {
        wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
        _ => return Err("No surface image".into()),
    };
    renderer.draw(&frame.texture.create_view(&Default::default()));
    frame.present();
    document.get_element_by_id("status").ok_or("No status")?.set_text_content(Some("The GPU painted the canvas."));
    Ok(())
}
