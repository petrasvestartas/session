#[path = "../src/command_dock/mod.rs"]
mod command_dock;
#[path = "../src/command_dock/tool_buttons_tests.rs"]
mod tool_buttons_tests;

#[test]
fn an_open_dock_does_not_take_another_fields_focus() {
    let context = egui::Context::default();
    context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
    let mut model = command_dock::CommandLine { command_open: true, ..Default::default() };
    let other = egui::Id::new("layer-name");
    let command = egui::Id::new("command-input");
    let _ = context.run_ui(Default::default(), |ui| {
        ui.memory_mut(|memory| memory.request_focus(other));
        command_dock::keys(ui, &mut model, None, command);
        assert_eq!(ui.memory(|memory| memory.focused()), Some(other));
    });
}

#[test]
fn an_explicit_focus_request_is_consumed_once() {
    let context = egui::Context::default();
    context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
    let mut model = command_dock::CommandLine { focus_command: true, ..Default::default() };
    let command = egui::Id::new("command-input");
    let _ = context.run_ui(Default::default(), |ui| {
        command_dock::keys(ui, &mut model, None, command);
        assert!(ui.memory(|memory| memory.has_focus(command)));
        assert!(!model.focus_command);
        assert!(model.command_open);
    });
}
