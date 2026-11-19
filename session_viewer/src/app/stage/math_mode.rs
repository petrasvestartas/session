use super::presentation::Presentation;
use crate::app::mode::{Frame, Mode};

// --8<-- [start:math-mode]
impl<Host> Mode<Host> for Presentation {
    fn key(&mut self, key: winit::keyboard::Key<&str>, now: f64) -> bool {
        Presentation::key(self, key, now)
    }

    fn pointer(&mut self, _: &winit::event::WindowEvent, _: f64, _: f64, _: &mut Host) -> bool {
        false
    }

    fn frame(
        &mut self,
        _: &mut Host,
        context: &egui::Context,
        input: egui::RawInput,
        frame: Frame,
    ) -> (egui::FullOutput, bool) {
        self.stage.tick(frame.now);
        let mut more = false;
        let output = context.run_ui(input, |root| {
            more = self.paint(root.painter(), frame.screen, frame.pixels_per_point);
        });
        (output, more)
    }

    fn background(&self) -> [f32; 4] {
        self.stage.palette("bg")
    }
}
// --8<-- [end:math-mode]
