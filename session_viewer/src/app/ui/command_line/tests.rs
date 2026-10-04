use super::super::theme::{BUNDLED, fonts};
use super::*;

fn frame(
    context: &egui::Context,
    model: &mut CommandLine,
    key: Option<egui::Key>,
) -> Option<String> {
    let mut input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1000.0, 700.0),
        )),
        ..Default::default()
    };
    if let Some(key) = key {
        for pressed in [true, false] {
            input.events.push(egui::Event::Key {
                key,
                physical_key: None,
                pressed,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            });
        }
    }
    let mut out = Output::default();
    let _ = context.run_ui(input, |ui| draw(ui, model, &mut None, &mut out));
    out.command
}

#[test]
fn a_space_the_name_spells_types_on() {
    assert_eq!(spelled("Orient 3 Points", "orient3p"), 10);
    assert_eq!(spelled("Orient 3 Points", "Orient"), 6);
    let context = egui::Context::default();
    context.set_fonts(fonts(BUNDLED));
    let mut model = CommandLine {
        focus_command: true,
        ..Default::default()
    };
    frame(&context, &mut model, None);

    // one key per frame, like a person typing
    for c in "Orient 3 Points".chars() {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 700.0),
            )),
            events: vec![egui::Event::Text(c.to_string())],
            ..Default::default()
        };
        let _ = context.run_ui(input, |ui| {
            draw(ui, &mut model, &mut None, &mut Output::default())
        });
    }

    assert_eq!(model.command, "Orient 3 Points");
}

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

#[test]
fn up_down_cycles_command_options_and_enter_accepts() {
    // one Enter runs a bare verb that needs no option; a space after it opens the options to browse
    for name in ["Element Features", "Layers", "Arctic", "Outline", "Snap"] {
        let context = egui::Context::default();
        context.set_fonts(fonts(BUNDLED));
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let mut model = CommandLine {
            command: format!("{name} "),
            focus_command: true,
            ..Default::default()
        };
        frame(&context, &mut model, None);
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
