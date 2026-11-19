use super::{Loaded, StageApp, paint, spatial::Scene3};
use crate::{
    State,
    app::mode::{self, Frame},
};

impl mode::Loaded<State> for Loaded {
    fn open(self: Box<Self>, now: f64) -> Box<dyn mode::Mode<State>> {
        Box::new(StageApp::new(*self, now))
    }
}

impl mode::Mode<State> for StageApp {
    fn key(&mut self, key: winit::keyboard::Key<&str>, now: f64) -> bool {
        StageApp::key(self, key, now)
    }

    fn pointer(
        &mut self,
        event: &winit::event::WindowEvent,
        scale: f64,
        now: f64,
        state: &mut State,
    ) -> bool {
        StageApp::pointer(self, event, scale, now, &mut state.camera)
    }

    // --8<-- [start:stage-frame-order]
    fn frame(
        &mut self,
        state: &mut State,
        context: &egui::Context,
        input: egui::RawInput,
        frame: Frame,
    ) -> (egui::FullOutput, bool) {
        let Frame {
            screen,
            logical,
            pixels_per_point: ppp,
            now,
        } = frame;
        let f = paint::Frame::new(screen, ppp)
            .stacked(screen, self.presentation.stage.split)
            .side(0.0);
        self.presentation.stage.tick(now);
        let ground = self.presentation.stage.palette("bg");
        state.gpu.view.dark = ground[0] + ground[1] + ground[2] < 1.5;
        state.gpu.view.show_grid = false;
        let (cam, busy) = self.presentation.stage.camera(now);
        let (serial, origin) = (
            self.presentation.stage.cam_serial,
            self.presentation.stage.origin(),
        );
        self.scene3
            .camera(state, cam, busy, serial, origin, &f, logical);
        self.presentation.stage.lens = Some(Scene3::lens(state, &f, logical));
        let mut more = false;
        let output = context.run_ui(input, &mut |root: &mut egui::Ui| {
            let painter = root.painter().with_clip_rect(screen);
            more = self.paint(&painter, screen, ppp, now);
        });
        let bg = self.presentation.stage.palette("bg");
        let (list, epoch) = (
            std::mem::take(&mut self.presentation.stage.objects3),
            self.presentation.stage.epoch,
        );
        self.scene3.sync(state, &list, epoch, bg, f.k as f64);
        self.presentation.stage.objects3 = list;
        (output, more)
    }
    // --8<-- [end:stage-frame-order]

    fn background(&self) -> [f32; 4] {
        <super::presentation::Presentation as mode::Mode<State>>::background(&self.presentation)
    }
}
