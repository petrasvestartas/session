
    fn prepare(&mut self) {
        let size = self.screen.size_in_pixels;
        #[cfg(target_arch="wasm32")]
        let density=web_sys::window().map_or(1.0,|window|window.device_pixel_ratio() as f32);
        #[cfg(not(target_arch="wasm32"))]
        let density=1.0;
        self.screen.pixels_per_point=density;
        let mut input = egui::RawInput {
