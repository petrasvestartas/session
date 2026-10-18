        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            if let Some(rectangle)=self.rectangle {
                let painter=root.ctx().layer_painter(egui::LayerId::new(egui::Order::Middle,egui::Id::new("selection-rectangle")));
                let [x0,y0,x1,y1]=rectangle.bounds.map(|v|v as f32);let rect=egui::Rect::from_min_max(egui::pos2(x0,y0),egui::pos2(x1,y1));
                painter.rect_filled(rect,0.,egui::Color32::from_rgba_unmultiplied(160,160,160,40));
                let stroke=egui::Stroke::new(1.,egui::Color32::from_gray(110));
                let corners=[rect.left_top(),rect.right_top(),rect.right_bottom(),rect.left_bottom(),rect.left_top()];
                for pair in corners.windows(2) {if rectangle.crossing {
                    let length=pair[0].distance(pair[1]);let direction=(pair[1]-pair[0])/length;
                    for n in 0..((length/8.).ceil() as usize).min(16384) {let at=n as f32*8.;painter.line_segment([pair[0]+direction*at,pair[0]+direction*(at+4.).min(length)],stroke);}
                } else {painter.line_segment([pair[0],pair[1]],stroke);}}
            }
            crate::tree_panel::draw(root,&mut self.tree,&mut self.controls);
            crate::number_box::draw(root,&mut self.number,&mut self.controls);
