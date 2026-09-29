
#[test]
fn up_down_cycles_command_options_and_enter_accepts() {
    for name in ["Element Features", "Layers", "Arctic", "Outline", "Snap"] {
        let context = egui::Context::default();
        context.set_fonts(fonts(BUNDLED));
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let mut model = CommandLine {
            command: name.into(),
            focus_command: true,
            ..Default::default()
        };
        frame(&context, &mut model, None);
        assert_eq!(frame(&context, &mut model, Some(egui::Key::Enter)), None);
        assert_eq!(model.command, format!("{name} "));
        let options = crate::app::command::options(&model.command);
        for key in [egui::Key::ArrowLeft, egui::Key::ArrowRight] {
            frame(&context, &mut model, Some(key));
            assert_eq!(model.command, format!("{name} "));
        }
        frame(&context, &mut model, Some(egui::Key::ArrowDown));
        assert_eq!(model.command, options[1]);
        frame(&context, &mut model, Some(egui::Key::ArrowDown));
        assert_eq!(model.command, options[2 % options.len()]);
        frame(&context, &mut model, Some(egui::Key::ArrowUp));
        assert_eq!(model.command, options[1]);
        let (expected, run) = crate::app::command::accept(options[1]);
        let command = frame(&context, &mut model, Some(egui::Key::Enter));
        assert_eq!(command, run.then_some(expected.clone()));
        if !run {
            assert_eq!(model.command, expected);
        }
    }
}
