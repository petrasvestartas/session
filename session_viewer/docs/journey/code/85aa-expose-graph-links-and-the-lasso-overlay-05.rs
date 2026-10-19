        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            if self.lasso.len()>1{let painter=root.ctx().layer_painter(egui::LayerId::new(egui::Order::Middle,egui::Id::new("selection-lasso")));for pair in self.lasso.iter().copied().chain(self.lasso.first().copied()).collect::<Vec<_>>().windows(2){painter.line_segment([egui::pos2(pair[0][0],pair[0][1]),egui::pos2(pair[1][0],pair[1][1])],egui::Stroke::new(1.5,egui::Color32::from_rgb(30,110,170)));}}
            if let Some(rectangle)=self.rectangle {
                let painter=root.ctx().layer_painter(egui::LayerId::new(egui::Order::Middle,egui::Id::new("selection-rectangle")));
