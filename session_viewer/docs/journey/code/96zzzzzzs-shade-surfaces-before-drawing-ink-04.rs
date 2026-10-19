        self.widget.draw(&mut encoder,view);
        if let Some(pyramid)=self.pyramid.borrow_mut().as_mut(){pyramid.encode