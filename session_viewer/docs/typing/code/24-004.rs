
#[test]
fn browsing_starts_from_the_typed_option() {
    let context = egui::Context::default();
    context.set_fonts(fonts(BUNDLED));
    let mut model = CommandLine {
        command: "Snap Off".into(),
        focus_command: true,
        ..Default::default()
    };
    frame(&context, &mut model, None);
    frame(&context, &mut model, Some(egui::Key::ArrowDown));
    assert_eq!(model.command, "Snap End");
}
