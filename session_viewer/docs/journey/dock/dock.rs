use crate::command_dock::{self, CommandLine};
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

struct Commands;

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        if line.trim().eq_ignore_ascii_case("Background") {
            "Background".into()
        } else {
            line.trim().into()
        }
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn draws(&self, _: &str) -> bool {
        false
    }
    fn options(&self, _: &str) -> &'static [&'static str] {
        &[]
    }
    fn option_label<'a>(&self, line: &'a str) -> &'a str {
        line
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        if "background".starts_with(&line.to_ascii_lowercase()) {
            vec!["Background"]
        } else {
            Vec::new()
        }
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

pub struct Dock {
    context: egui::Context,
    model: CommandLine,
    painter: egui_wgpu::Renderer,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Dock {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat) -> Self {
        let context = egui::Context::default();
        context.set_fonts(command_dock::theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        context.set_theme(egui::Theme::Light);
        context.set_visuals(command_dock::theme::visuals());
        Self {
            context,
            model: CommandLine {
                focus_command: true,
                ..Default::default()
            },
            painter: egui_wgpu::Renderer::new(&renderer.device, format, Default::default()),
            output: None,
            screen: egui_wgpu::ScreenDescriptor {
                size_in_pixels: [640, 480],
                pixels_per_point: 1.0,
            },
        }
    }

    pub fn answer(&mut self, line: &str, message: &str) {
        if self.model.history.len() == 200 {
            self.model.history.pop_front();
        }
        self.model.history.push_back(format!("> {line}\n{message}"));
        self.model.status = message.into();
    }

    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, JsValue> {
        let rect = canvas.get_bounding_client_rect();
        let size = egui::vec2(rect.width() as f32, rect.height() as f32);
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            focused: true,
            ..Default::default()
        };
        if let Some(event) = event {
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                let pos = egui::pos2(
                    (pointer.client_x() as f64 - rect.left()) as f32,
                    (pointer.client_y() as f64 - rect.top()) as f32,
                );
                input.events.push(egui::Event::PointerMoved(pos));
                if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
                    let button = match pointer.button() {
                        0 => egui::PointerButton::Primary,
                        1 => egui::PointerButton::Middle,
                        _ => egui::PointerButton::Secondary,
                    };
                    input.events.push(egui::Event::PointerButton {
                        pos,
                        button,
                        pressed: event.type_() == "pointerdown",
                        modifiers: Default::default(),
                    });
                    if event.type_() == "pointerdown" {
                        let options = web_sys::FocusOptions::new();
                        options.set_prevent_scroll(true);
                        canvas.focus_with_options(&options)?;
                        event.prevent_default();
                    }
                }
            } else if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                if !key.is_composing() {
                    input.modifiers = egui::Modifiers {
                        alt: key.alt_key(),
                        ctrl: key.ctrl_key(),
                        shift: key.shift_key(),
                        mac_cmd: key.meta_key(),
                        command: key.ctrl_key() || key.meta_key(),
                    };
                    if let Some(code) = egui::Key::from_name(&key.key()) {
                        input.events.push(egui::Event::Key {
                            key: code,
                            physical_key: None,
                            pressed: event.type_() == "keydown",
                            repeat: key.repeat(),
                            modifiers: input.modifiers,
                        });
                    }
                    if event.type_() == "keydown"
                        && key.key().chars().count() == 1
                        && !input.modifiers.command
                        && !input.modifiers.alt
                    {
                        input.events.push(egui::Event::Text(key.key()));
                    }
                    if self.context.egui_wants_keyboard_input() {
                        event.prevent_default();
                    }
                }
            } else if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() {
                input.events.push(egui::Event::MouseWheel {
                    unit: match wheel.delta_mode() {
                        1 => egui::MouseWheelUnit::Line,
                        2 => egui::MouseWheelUnit::Page,
                        _ => egui::MouseWheelUnit::Point,
                    },
                    phase: egui::TouchPhase::Move,
                    delta: egui::vec2(-wheel.delta_x() as f32, -wheel.delta_y() as f32),
                    modifiers: Default::default(),
                });
                if self.context.egui_wants_pointer_input() {
                    event.prevent_default();
                }
            }
        }
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / size.x;
        self.context
            .set_pixels_per_point(self.screen.pixels_per_point);
        let mut line = None;
        let mut controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(
                root,
                &mut self.model,
                &mut controls,
                &mut line,
                &Commands,
                &[],
                true,
            );
        });
        canvas.set_attribute("data-command-ui", &serde_json::json!({
            "controls": controls, "command": self.model.command, "history": self.model.history,
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
        }).to_string())?;
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
        } else {
            self.output = Some(output);
        }
        Ok(line)
    }

    pub fn paint(&mut self, renderer: &Renderer, view: &wgpu::TextureView) {
        let Some(output) = self.output.take() else {
            return;
        };
        for (id, delta) in &output.textures_delta.set {
            self.painter
                .update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self
            .context
            .tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device,
            &renderer.queue,
            &mut encoder,
            &jobs,
            &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command dock"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        self.painter
            .render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free {
            self.painter.free_texture(&id);
        }
    }
}
