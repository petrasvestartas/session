            self.markers.draw(&mut pass);
            self.preview.draw(&mut pass);
        }
        self.widget.draw(&mut encoder,view);
        self.queue.submit([encoder.finish()]);
    }
}
