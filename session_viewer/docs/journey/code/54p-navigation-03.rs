use crate::shortcuts::{Shortcut,context_key};

#[test]
fn actual_command_name_and_popup_focus_own_shortcuts_like_current_panels() {
    let context=egui::Context::default();context.set_fonts(crate::command_dock::theme::fonts([include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf")]));
    let mut command=String::new();let mut name="Posts".to_owned();
    let mut frame=|id:&str|{
        context.memory_mut(|memory|memory.request_focus(egui::Id::new(id)));
        let _=context.run_ui(egui::RawInput {screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(640.,480.))),..Default::default()},|ui|{
            ui.add(egui::TextEdit::singleline(&mut command).id(egui::Id::new("command-input")));ui.add(egui::TextEdit::singleline(&mut name).id(egui::Id::new("layer-name-input")));
        });
    };
    frame("command-input");assert_eq!(context.memory(|memory|memory.focused()),Some(egui::Id::new("command-input")));assert_eq!(context_key(&context,"","a",true,false,false,false),Some(Shortcut::SelectAll));
    assert_eq!(context_key(&context,"Move","a",true,false,false,false),None);assert_eq!(context_key(&context,"","z",true,false,false,false),Some(Shortcut::Undo));
    frame("layer-name-input");assert_eq!(context.memory(|memory|memory.focused()),Some(egui::Id::new("layer-name-input")));assert_eq!(context_key(&context,"","a",true,false,false,false),None);assert_eq!(context_key(&context,"","z",true,false,false,false),None);
    egui::Popup::open_id(&context,egui::Id::new("layer-menu"));assert_eq!(context_key(&context,"","a",true,false,false,false),None);assert_eq!(context_key(&context,"","h",false,false,false,false),None);
}
