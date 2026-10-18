    }) {menus.push(menu.response.rect);}
}

#[cfg(any(test,target_arch="wasm32"))]
fn dialogs(ui:&mut egui::Ui,model:&mut TreePanel,controls:&mut Option<Vec<Control>>) {
    if let Some((key,text))=model.rename.as_mut() {
        let id=egui::Id::new("layer-name-input");
        if std::mem::take(&mut model.rename_focus) {ui.memory_mut(|memory|memory.request_focus(id));}
        let cancel=ui.input_mut(|input|input.consume_key(egui::Modifiers::NONE,egui::Key::Escape));
        let mut submit=ui.input_mut(|input|input.consume_key(egui::Modifiers::NONE,egui::Key::Enter));
        ui.horizontal(|ui| {let field=ui.add(egui::TextEdit::singleline(text).id(id).char_limit(128).desired_width(130.));record(controls,"layer/name","Layer name",&field);let apply=ui.button("Rename");record(controls,"layer/name/apply","Rename",&apply);submit|=apply.clicked();});
        if !model.rename_error.is_empty() {ui.colored_label(egui::Color32::from_rgb(170,30,30),&model.rename_error);}
        if submit {model.edit=Some((key.clone(),crate::layer_edit::Edit::Rename(text.clone())));}
        if cancel {model.rename=None;model.rename_error.clear();ui.memory_mut(|memory|memory.surrender_focus(id));}
    }
    if let Some((key,count))=model.confirm.clone() {
        ui.label(format!("Delete this layer and {count} objects?"));
        ui.horizontal(|ui| {for (name,yes) in [("Delete",true),("Cancel",false)] {let button=ui.button(name);record(controls,&format!("layer/delete/{name}"),name,&button);if button.clicked() {if yes {model.edit=Some((key.clone(),crate::layer_edit::Edit::Delete));}model.confirm=None;}}});
    }
}
