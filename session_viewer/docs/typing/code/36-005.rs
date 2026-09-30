
impl State {
    /// Make elements slightly see-through the first time they arrive.
    fn dim_elements(&mut self, first_row: usize) {
        // an opacity was already chosen
        if self.features.opacity_chosen || self.gpu.view.opacity < 1.0 {
            return;
        }

        // does the new document have elements?
        let elements = (first_row..self.scene.row_count()).any(|row| {
            let row = row as u32;
            matches!(
                self.scene
                    .geometry(row)
                    .or_else(|| self.scene.instance_definition(row)),
                Some(session_rust::Geometry::Element(_))
            )
        });

        if elements {
            self.gpu.view.opacity = ELEMENT_OPACITY;
            self.features.opacity_chosen = true;
        }
    }

    /// Opacity <value>: 0 is x-ray, 1 is solid.
    pub fn set_opacity(&mut self, value: f32) {
        self.gpu.view.opacity = value.clamp(0.0, 1.0);
        self.features.opacity_chosen = true;
        self.touch();
    }
}
