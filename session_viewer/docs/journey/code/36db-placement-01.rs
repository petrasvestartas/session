    pub fn owner(&self) -> (u8, usize) {
        match self { Self::Line(line) => (0, Rc::as_ptr(line) as usize), Self::Polyline(line) => (1, Rc::as_ptr(line) as usize) }
    }

    pub fn coordinates