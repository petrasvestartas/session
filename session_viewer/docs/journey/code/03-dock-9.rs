use crate::command_dock::*;

struct Vocabulary;
impl Commands for Vocabulary {
    fn canonical(&self, line: &str) -> String { line.into() }
    fn choosing_option(&self, _: &str) -> bool { false }
    fn accept(&self, line: &str) -> (String, bool) { (line.into(), true) }
    fn completions(&self, _: &str) -> Vec<&'static str> { vec!["Thickness", "Help"] }
    fn browse(&self, line: &str) -> Vec<&'static str> { self.completions(line) }
}

fn frame(context: &egui::Context, model: &mut CommandLine, events: Vec<egui::Event>) -> (Option<String>, Vec<Control>) {
    let mut line = None; let mut controls = Some(Vec::new());
    let _ = context.run_ui(egui::RawInput { screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(960., 640.))),
        focused: true, events, ..Default::default() }, |root| { draw(root, model, &mut controls, &mut line, &Vocabulary, true); });
    (line, controls.unwrap())
}

fn context() -> egui::Context {
    let context = egui::Context::default(); context.options_mut(|o| o.max_passes = 1.try_into().unwrap());
    context.set_fonts(theme::fonts([
        include_bytes!("../../assets/text/NotoSans-Regular.subset.ttf"),
        include_bytes!("../../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
        include_bytes!("../../assets/text/NotoSansSymbols-Regular.subset.ttf"),
    ])); context
}

fn model() -> CommandLine {
    CommandLine { drawing_prompt: "Thickness <40>: type a number · Enter keeps it".into(), command_open: true,
        focus_command: true, options: vec![("Thickness 40".into(), "Thickness".into()), ("Create".into(), "".into()), ("Cancel".into(), "Escape".into())], ..Default::default() }
}

#[test]
fn real_value_buttons_return_their_option_and_keep_keyboard_focus() {
    let context = context(); let mut model = model();
    let (_, controls) = frame(&context, &mut model, vec![]);
    let button = controls.iter().find(|c| c.key == "command/option/Thickness 40").unwrap();
    let pos = egui::pos2((button.rect[0] + button.rect[2]) / 2., (button.rect[1] + button.rect[3]) / 2.);
    let rect = model.options_rect.unwrap(); assert!(rect.contains(pos)); assert!(rect.right() <= model.command_rect.unwrap().left(), "Value buttons must end before the answer field");
    frame(&context, &mut model, vec![egui::Event::PointerMoved(pos), egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() }]);
    let (line, _) = frame(&context, &mut model, vec![egui::Event::PointerButton { pos, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() }]);
    assert_eq!(line.as_deref(), Some("Thickness")); assert!(model.command_open && model.focus_command);
    frame(&context, &mut model, vec![egui::Event::Text("60".into())]); assert_eq!(model.command, "60", "Active values must not acquire an unrelated command suffix");
    let (line, _) = frame(&context, &mut model, vec![egui::Event::Key { key: egui::Key::Enter, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() }]);
    assert_eq!(line.as_deref(), Some("60")); assert!(model.completion_rect.is_none());
}

#[test]
fn a_running_tool_suppresses_suggestions_and_empty_enter_reaches_the_owner() {
    let context = context(); let mut model = model();
    frame(&context, &mut model, vec![]); frame(&context, &mut model, vec![egui::Event::Text("Th".into())]);
    assert_eq!(model.command, "Th"); assert!(!model.inline_suffix); assert!(model.completion_rect.is_none());
    model.command.clear(); let (line, _) = frame(&context, &mut model, vec![egui::Event::Key { key: egui::Key::Enter, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() }]);
    assert_eq!(line.as_deref(), Some(""), "The tool decides whether Enter keeps a value or creates geometry");
    model.options.clear(); model.drawing_prompt.clear(); frame(&context, &mut model, vec![egui::Event::Text("Th".into())]);
    assert!(model.inline_suffix, "Ordinary command completion returns after the tool ends");
}
