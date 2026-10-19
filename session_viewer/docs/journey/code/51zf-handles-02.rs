use crate::{number_box::{NumberBox,NumberPrompt},gizmo::{Handle,Axis},command_dock::*};

struct Vocabulary;
impl Commands for Vocabulary {
    fn canonical(&self,line:&str)->String {line.into()}
    fn choosing_option(&self,_:&str)->bool {false}
    fn accept(&self,line:&str)->(String,bool) {(line.into(),true)}
    fn completions(&self,_:&str)->Vec<&'static str> {vec!["Help"]}
    fn browse(&self,line:&str)->Vec<&'static str> {self.completions(line)}
}
fn context()->egui::Context {let context=egui::Context::default();context.options_mut(|o|o.max_passes=1.try_into().unwrap());context.set_fonts(theme::fonts([
    include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
]));context}
fn prompt(handle:Handle)->NumberPrompt {NumberPrompt {handle,title:handle.title(),unit:handle.labels().2,hint:if handle.labels().2=="factor" {"1"} else {"0"},at:[300.,200.]}}
fn frame(context:&egui::Context,number:&mut NumberBox,model:&mut CommandLine,events:Vec<egui::Event>)->(Option<String>,Option<String>,Vec<Control>) {
    let mut line=None;let mut controls=Some(Vec::new());
    let _=context.run_ui(egui::RawInput {screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(640.,480.))),focused:true,events,..Default::default()},|root| {
        crate::number_box::draw(root,number,&mut controls);draw(root,model,&mut controls,&mut line,&Vocabulary,true);
    });(number.take(),line,controls.unwrap())
}
fn key(key:egui::Key)->egui::Event {egui::Event::Key {key,physical_key:None,pressed:true,repeat:false,modifiers:Default::default()}}

#[test]
fn real_handle_number_field_owns_text_enter_and_escape_without_submitting_a_command() {
    let context=context();let mut number=NumberBox::default();number.set(Some(prompt(Handle::Translate(Axis::X))));let mut model=CommandLine::default();
    for _ in 0..3 {frame(&context,&mut number,&mut model,vec![]);}
    assert_eq!(context.memory(|m|m.focused()),Some(egui::Id::new("number-input")));
    let (_,_,controls)=frame(&context,&mut number,&mut model,vec![egui::Event::Text("12.3456789012345".into())]);
    let field=controls.iter().find(|c|c.key=="number/input").unwrap();assert!(number.number_rect.unwrap().contains(egui::pos2(field.rect[0],field.rect[1])));
    assert_eq!(number.number,"12.3456789012345");assert!(model.command.is_empty());
    let (answer,command,_)=frame(&context,&mut number,&mut model,vec![key(egui::Key::Enter)]);assert_eq!(answer.as_deref(),Some("12.3456789012345"));assert!(command.is_none());
    number.number_error="A finite number is required".into();let (_,_,controls)=frame(&context,&mut number,&mut model,vec![]);assert!(controls.iter().any(|c|c.key=="number/input"));assert_eq!(number.number,"12.3456789012345");
    let (answer,command,_)=frame(&context,&mut number,&mut model,vec![key(egui::Key::Escape)]);assert_eq!(answer.as_deref(),Some("Escape"));assert!(command.is_none());
}

#[test]
fn switching_handle_clears_old_text_and_error_and_closed_box_releases_its_rectangle() {
    let context=context();let mut number=NumberBox::default();let mut model=CommandLine::default();number.set(Some(prompt(Handle::Translate(Axis::X))));
    frame(&context,&mut number,&mut model,vec![]);frame(&context,&mut number,&mut model,vec![egui::Event::Text("bad".into())]);number.number_error="Invalid answer".into();
    number.set(Some(prompt(Handle::ScaleUniform)));frame(&context,&mut number,&mut model,vec![]);assert!(number.number.is_empty() && number.number_error.is_empty());
    let (answer,_,_)=frame(&context,&mut number,&mut model,vec![key(egui::Key::Enter)]);assert_eq!(answer.as_deref(),Some(""));
    number.set(None);frame(&context,&mut number,&mut model,vec![]);assert!(number.number_rect.is_none());
}
