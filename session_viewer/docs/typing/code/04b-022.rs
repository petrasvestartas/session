            LaneId::Pipes => self.segments.kill(&self.ctx, lane, first, count, sink), // register:strokes
            LaneId::Ribbons => self.segments.kill(&self.ctx, lane, first, count, sink), // register:strokes
