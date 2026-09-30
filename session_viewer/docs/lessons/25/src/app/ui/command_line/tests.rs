use super::super::theme::{BUNDLED, fonts};
use super::*;

// --8<-- [start:23-test-frame]
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
// --8<-- [end:23-test-frame]

// --8<-- [start:23-test-a_space_the_name_spells_types_on]
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
// --8<-- [end:23-test-a_space_the_name_spells_types_on]

// --8<-- [start:24-test-browsing_starts_from_the_typed_option]
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
// --8<-- [end:24-test-browsing_starts_from_the_typed_option]
