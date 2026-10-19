#[cfg(any(test,target_arch="wasm32"))]
pub fn draw(ui:&mut egui::Ui,key:&str,edge:bool,colour:Option<[u8;3]>,controls:&mut Option<Vec<crate::command_dock::Control>>)->Option<[u8;3]> {
    use crate::command_dock::record;
    let mut answer=None;
        ui.separator();
        for colours in [[("Red",[230,65,55]),("Orange",[240,145,45]),("Yellow",[240,210,60])],[("Green",[60,170,100]),("Blue",[65,130,225]),("Violet",[160,85,210])],[("White",[245,245,245]),("Gray",[150,150,150]),("Black",[35,35,35])]] {
            ui.horizontal(|ui| {for (name,rgb) in colours {let button=ui.add_sized([48.,28.],egui::Button::new(egui::RichText::new("■").color(egui::Color32::from_rgb(rgb[0],rgb[1],rgb[2])))).on_hover_text(name);record(controls,&format!("layer/colour/{key}/{edge}/{name}"),name,&button);if button.clicked() {answer=Some(rgb);ui.close();}}});
        }
        ui.separator();let mut rgb=colour.unwrap_or([180;3]);let mut changed=false;
        for (name,value) in ["R","G","B"].into_iter().zip(rgb.iter_mut()) {let slider=ui.add(egui::Slider::new(value,0..=255).text(name));record(controls,&format!("layer/rgb/{key}/{edge}/{name}"),name,&slider);changed|=slider.changed();}
        if changed {answer=Some(rgb);}
    answer
}
