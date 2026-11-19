use winit::{event::WindowEvent, keyboard::Key};

pub struct Frame {
    pub screen: egui::Rect,
    pub logical: [f64; 2],
    pub pixels_per_point: f32,
    pub now: f64,
}

// --8<-- [start:application-mode]
pub trait Loaded<Host> {
    fn open(self: Box<Self>, now: f64) -> Box<dyn Mode<Host>>;
}

pub trait Mode<Host> {
    fn key(&mut self, key: Key<&str>, now: f64) -> bool;
    fn pointer(&mut self, event: &WindowEvent, scale: f64, now: f64, host: &mut Host) -> bool;
    fn frame(
        &mut self,
        host: &mut Host,
        context: &egui::Context,
        input: egui::RawInput,
        frame: Frame,
    ) -> (egui::FullOutput, bool);
    fn background(&self) -> [f32; 4];
}
// --8<-- [end:application-mode]
