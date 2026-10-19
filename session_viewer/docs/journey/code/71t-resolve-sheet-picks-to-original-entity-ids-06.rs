                markers.push(crate::marker::Marker {center:p.to_f32(),diameter:11.,colour:yellow});
            }
            Part::SheetEntity(_)|Part::Object => {}
        } }
        self.strokes.set(device, strokes)?; self.paths.set_chains(device, paths)?; self.markers.set_points(device, markers)?; Ok(())
