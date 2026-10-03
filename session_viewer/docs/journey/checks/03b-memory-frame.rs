#[path = "../src/command_dock/mod.rs"]
mod command_dock;

fn draw(renderer: &Renderer, target: &wgpu::TextureView) {
    verify_memory();
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
