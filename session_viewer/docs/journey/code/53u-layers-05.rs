use crate::{command_dock::Control,editor::{Action,Editor},tree_panel::{self,TreePanel},layer_edit::Edit};

fn setup()->(egui::Context,TreePanel,String) {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();e.apply(Action::AddNested).unwrap();let root=crate::hierarchy::rows(&e.scene)[0].key.clone();e.apply(Action::LayerEdit {key:root,edit:Edit::New {sublayer:true}}).unwrap();
    let mut model=TreePanel::default();model.refresh(&e.scene,vec![]);model.all();let key=model.rows.iter().find(|row|row.label=="Layer 01").unwrap().key.clone();
    let context=egui::Context::default();context.options_mut(|options|options.max_passes=1.try_into().unwrap());(context,model,key)
}

fn frame(context:&egui::Context,model:&mut TreePanel,events:Vec<egui::Event>)->Vec<Control> {
    let mut controls=Some(Vec::new());let input=egui::RawInput {screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(1000.,800.))),events,..Default::default()};
    let _=context.run_ui(input,|ui|tree_panel::draw(ui,model,&mut controls));controls.unwrap()
}

fn center(controls:&[Control],key:&str)->egui::Pos2 {let rect=controls.iter().find(|control|control.key==key).unwrap_or_else(||panic!("Missing actual control {key}")).rect;egui::pos2((rect[0]+rect[2])/2.,(rect[1]+rect[3])/2.)}
fn click(context:&egui::Context,model:&mut TreePanel,at:egui::Pos2,button:egui::PointerButton)->Vec<Control> {
    for pressed in [true,false] {frame(context,model,vec![egui::Event::PointerMoved(at),egui::Event::PointerButton {pos:at,button,pressed,modifiers:Default::default()}]);}frame(context,model,vec![])
}

#[test]
fn empty_group_context_menu_renames_with_real_text_focus_and_escape_owns_the_key() {
    let (context,mut model,key)=setup();frame(&context,&mut model,vec![]);let controls=frame(&context,&mut model,vec![]);assert!(model.rect.unwrap().width()>=330.);
    let controls=click(&context,&mut model,center(&controls,&format!("tree/select/{key}")),egui::PointerButton::Secondary);assert!(!model.menu_rects.is_empty());
    let controls=click(&context,&mut model,center(&controls,&format!("layer/menu/{key}/Rename")),egui::PointerButton::Primary);assert!(model.rename.is_some());assert!(controls.iter().any(|control|control.key=="layer/name"));
    assert_eq!(context.memory(|memory|memory.focused()),Some(egui::Id::new("layer-name-input")));
    frame(&context,&mut model,vec![egui::Event::Text(" HS".into())]);assert_eq!(model.rename.as_ref().unwrap().1,"Layer 01 HS");assert!(model.edit.is_none());
    frame(&context,&mut model,vec![egui::Event::Key {key:egui::Key::Enter,physical_key:None,pressed:true,repeat:false,modifiers:Default::default()}]);assert!(matches!(model.edit.take(),Some((_,Edit::Rename(text))) if text=="Layer 01 HS"));
    model.rename_result(Some("This name is already used"));frame(&context,&mut model,vec![]);assert_eq!(model.rename.as_ref().unwrap().1,"Layer 01 HS");assert!(!model.rename_error.is_empty());
    frame(&context,&mut model,vec![egui::Event::Key {key:egui::Key::Escape,physical_key:None,pressed:true,repeat:false,modifiers:Default::default()}]);assert!(model.rename.is_none());assert!(model.edit.is_none());assert_eq!(context.memory(|memory|memory.focused()),None);
}

#[test]
fn actual_colour_popup_has_nine_swatches_rgb_and_independent_face_edge_channels() {
    let (context,mut model,_)=setup();let group=model.rows.iter().find(|row|row.label=="Posts").unwrap().key.clone();frame(&context,&mut model,vec![]);let controls=frame(&context,&mut model,vec![]);
    let controls=click(&context,&mut model,center(&controls,&format!("layer/colour/{group}")),egui::PointerButton::Primary);assert!(!model.menu_rects.is_empty());
    for name in ["Red","Orange","Yellow","Green","Blue","Violet","White","Gray","Black"] {assert!(controls.iter().any(|control|control.key==format!("layer/colour/{group}/false/{name}")));}
    for name in ["R","G","B"] {assert!(controls.iter().any(|control|control.key==format!("layer/rgb/{group}/false/{name}")));}
    let controls=click(&context,&mut model,center(&controls,&format!("layer/channel/{group}/Edges")),egui::PointerButton::Primary);assert!(controls.iter().any(|control|control.key==format!("layer/original/{group}/true")));
    click(&context,&mut model,center(&controls,&format!("layer/colour/{group}/true/Blue")),egui::PointerButton::Primary);assert!(matches!(model.property,Some((_,crate::layer_properties::Property::Colour {edge:true,rgb:Some([65,130,225])}))));
}

#[test]
fn populated_layer_deletion_requires_the_actual_confirmation_button() {
    let (context,mut model,_)=setup();let group=model.rows.iter().find(|row|row.label=="Posts").unwrap().key.clone();frame(&context,&mut model,vec![]);let controls=frame(&context,&mut model,vec![]);
    let controls=click(&context,&mut model,center(&controls,&format!("tree/select/{group}")),egui::PointerButton::Secondary);let controls=click(&context,&mut model,center(&controls,&format!("layer/menu/{group}/Delete")),egui::PointerButton::Primary);
    assert_eq!(model.confirm.as_ref(),Some(&(group,2)));assert!(model.edit.is_none());click(&context,&mut model,center(&controls,"layer/delete/Cancel"),egui::PointerButton::Primary);assert!(model.confirm.is_none());assert!(model.edit.is_none());
}

#[test]
fn long_source_labels_keep_the_wider_panel_within_the_canvas() {
    let mut e=Editor::default();e.apply(Action::Close).unwrap();let mut mesh=session_rust::Mesh::create_box(1.,1.,1.);mesh.name="A very long source name ".repeat(100);e.scene.insert(crate::prepared::PreparedMesh::new(mesh).unwrap()).unwrap();
    let context=egui::Context::default();let mut model=TreePanel::default();model.refresh(&e.scene,vec![]);model.all();frame(&context,&mut model,vec![]);let controls=frame(&context,&mut model,vec![]);let panel=model.rect.unwrap();
    assert!(panel.min.x>=0.&&panel.max.x<=1000.);assert!(panel.width()<380.);assert!(controls.iter().all(|control|control.rect[0]>=panel.min.x&&control.rect[2]<=panel.max.x));
    assert!(model.rows.iter().any(|row|row.label.len()>1000),"Clipping the label does not shorten original source metadata");
}

struct Vocabulary;
impl crate::command_dock::Commands for Vocabulary {
    fn canonical(&self, text:&str)->String {text.into()}
    fn choosing_option(&self,_:&str)->bool {false}
    fn accept(&self,text:&str)->(String,bool) {(text.into(),true)}
    fn completions(&self,_:&str)->Vec<&'static str> {vec![]}
    fn browse(&self,_:&str)->Vec<&'static str> {vec![]}
}

#[test]
fn an_open_command_dock_does_not_steal_layer_editing_focus() {
    let (context,mut tree,key)=setup();
    let mut dock=crate::command_dock::CommandLine::default();dock.command_open=true;
    let run=|tree:&mut TreePanel,dock:&mut crate::command_dock::CommandLine,events| {
        let mut command=None;let mut controls=Some(Vec::new());
        let _=context.run_ui(egui::RawInput {screen_rect:Some(egui::Rect::from_min_size(egui::Pos2::ZERO,egui::vec2(1000.,800.))),events,..Default::default()},|ui| {
            tree_panel::draw(ui,tree,&mut controls);
            crate::command_dock::draw(ui,dock,&mut controls,&mut command,&Vocabulary,true);
        });
        assert!(command.is_none());
    };
    run(&mut tree,&mut dock,vec![]);run(&mut tree,&mut dock,vec![]);
    tree.rename=Some((key,"Layer 01".into()));tree.rename_result(Some("Retry this name"));
    run(&mut tree,&mut dock,vec![]);run(&mut tree,&mut dock,vec![egui::Event::Text(" HS".into())]);
    assert_eq!(context.memory(|m|m.focused()),Some(egui::Id::new("layer-name-input")));
    assert_eq!(tree.rename.as_ref().unwrap().1,"Layer 01 HS");assert!(dock.command.is_empty());
    assert_eq!(crate::shortcuts::context_key(&context,&dock.command,"a",true,false,false,false),None);
}
