#[path = "../src/command_dock/mod.rs"]
mod command_dock;

fn draw(renderer: &Renderer, target: &wgpu::TextureView) {
    verify_memory();
    verify_vocabulary();
    verify_caret();
    renderer.draw(target);
}

fn verify_memory() {
    let mut model = command_dock::CommandLine::default();
    assert!(model.command.is_empty() && model.history.is_empty());
    model.command = "Help".into();
    model.status = "Ready".into();
    model.history.push_back("> Help\nReady".into());
    assert_eq!(command_dock::placeholder("", &model.status, false), "Ready");
    assert_eq!(command_dock::placeholder("Choose", &model.status, false), "Choose");
    assert_eq!(command_dock::placeholder("", &model.status, true), "Type a command");
    assert_eq!(command_dock::placeholder("", "", false), "Type a command");
    assert_eq!(command_dock::placeholder("", &model.status, false).as_ptr(), model.status.as_ptr());
    let field = &mut model.command;
    field.push_str(" View");
    assert_eq!(model.command, "Help View");
    assert_eq!(model.history.front().unwrap(), "> Help\nReady");
    println!("Owned text/history and borrowed placeholder priority pass.");
}

fn verify_vocabulary() {
    let context = egui::Context::default();
    context.set_fonts(command_dock::theme::fonts([
        include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
        include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
        include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
    ]));
    let mut response = None;
    context.run_ui(Default::default(), |ui| {
        response = Some(ui.label("Command:"));
    });
    let response = response.unwrap();
    let mut disabled = None;
    command_dock::record(&mut disabled, "command/input", "Command:", &response);
    assert!(disabled.is_none());
    let mut controls = Some(Vec::new());
    command_dock::record(&mut controls, "command/input", "Command:", &response);
    let controls = controls.unwrap();
    assert_eq!(controls.len(), 1);
    let json = serde_json::to_value(&controls[0]).unwrap();
    assert_eq!(json["key"], "command/input");
    assert_eq!(json["label"], "Command:");
    assert_eq!(json["rect"], serde_json::json!([
        response.rect.min.x, response.rect.min.y, response.rect.max.x, response.rect.max.y
    ]));
    println!("Optional inspection and exact serialized bounds from actual egui layout pass.");
}

fn verify_caret() {
    assert_eq!(command_dock::spelled("Orient 3 P", "orient3p"), 10);
    assert_eq!(command_dock::spelled("Orient 3 P", "Ori"), 3);
    assert_eq!(command_dock::spelled("é🙂 x", "é🙂x"), 4);
    let context = egui::Context::default();
    let id = egui::Id::new("caret-proof");
    command_dock::command_cursor_end(&context, id, "é🙂x");
    assert!(egui::TextEdit::load_state(&context, id).is_none());
    egui::TextEdit::store_state(&context, id, Default::default());
    command_dock::command_cursor_end(&context, id, "é🙂x");
    let range = egui::TextEdit::load_state(&context, id).unwrap().cursor.char_range().unwrap();
    assert_eq!([range.primary.index, range.secondary.index], [3, 3]);
    command_dock::command_cursor_select(&context, id, 1, 3);
    let range = egui::TextEdit::load_state(&context, id).unwrap().cursor.char_range().unwrap();
    let mut limits = [range.primary.index, range.secondary.index];
    limits.sort();
    assert_eq!(limits, [1, 3]);
    println!("Unicode scalar counts, missing-state handling and actual stored caret range pass.");
}

fn verify_pixels(pixels: &[u8]) {
    let mode = std::env::var("COURSE_FRAME").expect("The course runner supplies the expected frame");
    let background = if mode == "light" { [243, 243, 243] } else { [255, 255, 255] };
    assert!(near(&pixels[..3], background), "Unexpected background: {:?}", &pixels[..3]);
    let changed = pixels.chunks_exact(4).filter(|pixel| !near(&pixel[..3], background)).count();

    if mode == "clear" {
        assert_eq!(changed, 0, "The clear frame contains unexpected geometry");
    } else {
        assert!((49_000..52_000).contains(&changed), "Triangle coverage: {changed}");
        let centre = (240 * 640 + 320) * 4;
        assert!(near(&pixels[centre..centre + 3], [243, 137, 179]), "Missing pink triangle centre");
    }
}

fn near(pixel: &[u8], expected: [u8; 3]) -> bool {
    pixel.iter().zip(expected).all(|(&a, b)| a.abs_diff(b) <= 2)
}
