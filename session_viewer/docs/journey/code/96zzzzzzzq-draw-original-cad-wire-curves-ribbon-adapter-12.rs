        self.strokes.visible=self.ink_visibility.lines;
        self.paths.ranges.mask=if self.ink_visibility.lines{u8::MAX}else{0};