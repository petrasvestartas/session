use crate::command_dock::{self, CommandLine};
use crate::renderer::Renderer;
use wasm_bindgen::{JsCast, JsValue};

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
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
        let prefix = line.to_ascii_lowercase();
        self.0
            .iter()
            .copied()
            .filter(|name| name.to_ascii_lowercase().starts_with(&prefix))
            .collect()
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

pub struct Panel {
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    context: egui::Context,
    model: CommandLine,
    painter: egui_wgpu::Renderer,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(
        renderer: &Renderer,
        format: wgpu::TextureFormat,
        commands: &'static [&'static str],
    ) -> Self {
        let context = egui::Context::default();
        // Text input must be handled once, even when a widget requests another layout pass.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        context.set_fonts(command_dock::theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        context.set_theme(egui::Theme::Light);
        context.set_visuals(command_dock::theme::visuals());
        Self {
            commands: Commands(commands),
            consumed: false,
            pointer_owned: false,
            top: f32::INFINITY,
            context,
            model: CommandLine {
                command_expanded: true,
                history: ["Command history lives here.".into()].into(),
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
        _event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, JsValue> {
        let rect = canvas.get_bounding_client_rect();
        let size = egui::vec2(rect.width() as f32, rect.height() as f32);
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            focused: true,
            ..Default::default()
        };
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / size.x;
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point =
            Some(self.screen.pixels_per_point);
        let mut line = None;
        let mut controls = Some(Vec::new());
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(
                root,
                &mut self.model,
                &mut controls,
                &mut line,
                &self.commands,
                &[],
                true,
            );
        });
        if let Some(control) = controls
            .as_ref()
            .and_then(|items| items.iter().find(|item| item.key == "command/resize"))
        {
            self.top = control.rect[1];
        }
        canvas.set_attribute("data-command-ui", &serde_json::json!({
            "controls": controls, "command": self.model.command, "history": self.model.history,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
        }).to_string())?;
        if let Some(previous) = self.output.as_mut() {
            previous.append(output);
        } else {
            self.output = Some(output);
        }
        let Some(line) = line else {
            return Ok(None);
        };
        if line == "Escape" {
            return Ok(None);
        }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        if !self
            .commands
            .0
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&line))
        {
            self.answer(&line, "Unknown command. Type Help.");
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line.to_ascii_lowercase()))
    }

    pub fn draw(&mut self, renderer: &Renderer, view: &wgpu::TextureView) {
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
