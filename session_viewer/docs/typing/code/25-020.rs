
impl State {
    /// Tell the GPU where the gizmo is and which handle lights up.
    pub fn upload_gizmo(&mut self) {
        let Some(gizmo) = self.features.gizmo.as_ref() else {
            self.gpu.widget.clear();
            return;
        };
        self.gpu.widget.placement = Some((
            [gizmo.origin[0], gizmo.origin[1], gizmo.origin[2]],
            self.world_per_px(), // keeps the widget the same pixel size
        ));
        // the dragged handle, else the one being typed for, else the hovered one
        let handle = self
            .features
            .dragging
            .as_ref()
            .map(|drag| drag.drag.handle)
            .or(gizmo.typing)
            .or(gizmo.hovered);
        // handle index for the shader: 0-2 move, 3-5 rotate, 6-8 scale, 9 uniform
        self.gpu.widget.active = match handle {
            Some(Handle::Translate(axis)) => axis as u32 as f32,
            Some(Handle::Rotate(axis)) => axis as u32 as f32 + 3.0,
            Some(Handle::Scale(axis)) => axis as u32 as f32 + 6.0,
            Some(Handle::ScaleUniform) => 9.0,
            None => -1.0,
        };
    }

    /// Light the gizmo handle under the pointer; true when it changed.
    pub fn hover_gizmo(&mut self, x: f64, y: f64) -> bool {
        let Some((from, dir)) = self.camera.ray((x, y), self.viewport()) else {
            return false;
        };
        let per_px = self.world_per_px();
        let Some(gizmo) = self.features.gizmo.as_mut() else {
            return false;
        };
        let hovered = gizmo.hit(&from, &dir, per_px);

        if gizmo.hovered == hovered {
            return false;
        }

        gizmo.hovered = hovered;
        self.upload_gizmo(); // register:gumball
        true
    }
}

#[cfg(test)]
mod egui_tests_25 {

    /// The widget draws three colored arms.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn the_widget_reaches_the_pixels() {
        use crate::camera::Camera;
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow, Upload};
        use session_rust::Xform;

        let mut gpu = pollster::block_on(Gpu::new_headless(256, 256)).unwrap();
        gpu.view.show_grid = false;
        let mut upload = Upload::default();
        upload.obj.rows.push(ObjectRow::new(Xform::identity(), 0));
        gpu.set_scene(&upload);

        let mut camera = Camera::new();
        camera.target = [0.0, 0.0, 0.0];
        camera.distance = 0.2; // meters: 200 scene units
        camera.update_position();
        let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
        let input = FrameInput {
            view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
            clear: wgpu::Color::BLACK,
            now_ms: 0.0,
        };
        let before = gpu.render_offscreen(&input);

        // 96 pixels at 0.5 units per pixel fits the view
        gpu.widget.placement = Some(([0.0; 3], 0.5));
        let after = gpu.render_offscreen(&input);

        let (mut red, mut green, mut blue, mut changed) = (0, 0, 0, 0);

        for (a, b) in before.chunks_exact(4).zip(after.chunks_exact(4)) {
            if a[..3] == b[..3] {
                continue;
            }

            changed += 1;
            let (r, g, bl) = (i32::from(b[0]), i32::from(b[1]), i32::from(b[2]));

            if r > g + 40 && r > bl + 40 {
                red += 1;
            } else if g > r + 40 && g > bl + 40 {
                green += 1;
            } else if bl > r + 40 && bl > g + 40 {
                blue += 1;
            }
        }

        assert!(
            changed > 100,
            "the widget changed the picture: {changed} pixels"
        );
        assert!(
            red > 10 && green > 10 && blue > 10,
            "three coloured arms: red {red}, green {green}, blue {blue}, of {changed} changed"
        );
    }
}
