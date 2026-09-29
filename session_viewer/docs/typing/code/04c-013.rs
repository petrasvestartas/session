            LaneId::Spheres => self.glyphs.kill(&self.ctx, true, first, count, sink), // register:markers
            LaneId::Dots => self.glyphs.kill(&self.ctx, false, first, count, sink), // register:markers
