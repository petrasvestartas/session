            (self.frame.borrow().as_ref(), self.color.borrow().as_ref())
        {
            self.draw_with(pass, frame, &color.active);
            if self.visible {self.heads.draw(pass,&frame.groups,false);}
        }
    }

