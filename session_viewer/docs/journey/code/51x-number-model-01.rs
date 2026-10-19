#[derive(Clone)]
pub struct NumberPrompt {pub handle:crate::gizmo::Handle,pub title:String,pub unit:&'static str,pub hint:&'static str,pub at:[f32;2]}

#[derive(Default)]
pub struct NumberBox {
    pub number_prompt:Option<NumberPrompt>,number_handle:Option<crate::gizmo::Handle>,pub number:String,
    pub number_error:String,pub number_rect:Option<egui::Rect>,typed:Option<String>,closed:bool,
}
impl NumberBox {
    pub fn take(&mut self)->Option<String> {if std::mem::take(&mut self.closed) {Some("Escape".into())} else {self.typed.take()}}
    pub fn set(&mut self,prompt:Option<NumberPrompt>) {self.number_prompt=prompt;if self.number_prompt.is_none() {self.number_handle=None;self.number_rect=None;}}
}

