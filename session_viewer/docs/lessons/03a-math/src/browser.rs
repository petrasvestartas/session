use crate::app::stage::{
    deck,
    paint::{self, Atlas, Frame},
    run::Stage,
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};

fn error(e: impl ToString) -> JsValue {
    JsValue::from_str(&e.to_string())
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().ok_or("No window")?;
    let document = window.document().ok_or("No document")?;
    let canvas: web_sys::HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("No canvas")?
        .dyn_into()?;
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::BROWSER_WEBGPU,
        flags: Default::default(),
        memory_budget_thresholds: Default::default(),
        backend_options: Default::default(),
        display: None,
    });
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
        .map_err(error)?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        })
        .await
        .map_err(error)?;
    let (device, queue) = adapter
        .request_device(&Default::default())
        .await
        .map_err(error)?;
    let density = window.device_pixel_ratio().max(1.0) as f32;
    let limit = device.limits().max_texture_dimension_2d;
    let width = ((window.inner_width()?.as_f64().ok_or("No width")? as f32 * density) as u32)
        .clamp(1, limit);
    let height = ((window.inner_height()?.as_f64().ok_or("No height")? as f32 * density) as u32)
        .clamp(1, limit);
    canvas.set_width(width);
    canvas.set_height(height);
    let config = surface
        .get_default_config(&adapter, width, height)
        .ok_or("No surface format")?;
    surface.configure(&device, &config);
    let fonts = Rc::new(crate::fonts().map_err(error)?);
    let mut atlas = Atlas::new(&fonts);
    let index =
        deck::index("name: A slide of mathematics\nchapters: [example.yaml]").map_err(error)?;
    let chapter = deck::chapter(include_str!("../example.yaml"), "example.yaml").map_err(error)?;
    let stage = Rc::new(RefCell::new(Stage::new(fonts, index, vec![chapter])));
    stage.borrow_mut().next(0.0);
    let input_stage = stage.clone();
    let key = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
        move |event: web_sys::KeyboardEvent| {
            let mut stage = input_stage.borrow_mut();
            let now = stage.now;
            match event.key().as_str() {
                "ArrowRight" | " " => stage.next(now),
                "ArrowLeft" => stage.prev(now),
                "t" | "T" => {
                    let theme = stage.theme.toggled();
                    stage.set_theme(theme, now);
                }
                _ => return,
            }
            event.prevent_default();
        },
    );
    window.add_event_listener_with_callback("keydown", key.as_ref().unchecked_ref())?;
    key.forget();
    let context = egui::Context::default();
    context.options_mut(|o| o.max_passes = 1.try_into().unwrap());
    let mut painter = egui_wgpu::Renderer::new(&device, config.format, Default::default());
    let callback = Rc::new(RefCell::new(
        None::<wasm_bindgen::closure::Closure<dyn FnMut(f64)>>,
    ));
    let next = callback.clone();
    let animation_window = window.clone();
    *callback.borrow_mut() = Some(wasm_bindgen::closure::Closure::new(move |now: f64| {
        let result = (|| -> Result<(), JsValue> {
            let mut stage = stage.borrow_mut();
            let redraw = stage.changed || stage.animating();
            stage.tick(now);
            if !redraw {
                return Ok(());
            }
            let frame = match surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(frame)
                | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
                other => return Err(error(format!("Surface unavailable: {other:?}"))),
            };
            let view = frame.texture.create_view(&Default::default());
            let screen = egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(width as f32, height as f32) / density,
            );
            let mut input = egui::RawInput {
                screen_rect: Some(screen),
                time: Some(now / 1000.0),
                ..Default::default()
            };
            input
                .viewports
                .get_mut(&egui::ViewportId::ROOT)
                .unwrap()
                .native_pixels_per_point = Some(density);
            let output = context.run_ui(input, |root| {
                // --8<-- [start:column]
                let f = Frame::new(screen, density).stacked(screen, stage.split);
                paint::paint(&mut stage, &mut atlas, root.painter(), &f, screen);
                // --8<-- [end:column]
            });
            for (id, delta) in &output.textures_delta.set {
                painter.update_texture(&device, &queue, *id, delta);
            }
            let jobs = context.tessellate(output.shapes, output.pixels_per_point);
            let descriptor = egui_wgpu::ScreenDescriptor {
                size_in_pixels: [width, height],
                pixels_per_point: density,
            };
            let mut encoder = device.create_command_encoder(&Default::default());
            let mut buffers =
                painter.update_buffers(&device, &queue, &mut encoder, &jobs, &descriptor);
            let bg = stage.palette("bg");
            let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("math lesson"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: bg[0] as f64,
                            g: bg[1] as f64,
                            b: bg[2] as f64,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            painter.render(&mut pass.forget_lifetime(), &jobs, &descriptor);
            buffers.push(encoder.finish());
            queue.submit(buffers);
            frame.present();
            for id in output.textures_delta.free {
                painter.free_texture(&id);
            }
            let (done, total) = stage.progress();
            document
                .get_element_by_id("status")
                .ok_or("No status")?
                .set_text_content(Some(&format!(
                    "Step {done}/{total}; animated: {}",
                    stage.animating()
                )));
            if !stage.errors.is_empty() {
                return Err(error(stage.errors.join("; ")));
            }
            stage.changed = atlas.overflowed;
            atlas.overflowed = false;
            Ok(())
        })();
        if let Err(problem) = result {
            wasm_bindgen::throw_val(problem);
        }
        animation_window
            .request_animation_frame(next.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }));
    window.request_animation_frame(callback.borrow().as_ref().unwrap().as_ref().unchecked_ref())?;
    Ok(())
}
