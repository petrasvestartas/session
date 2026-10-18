        })
    }
    pub fn radius(&self, index: usize) -> f32 {
        let width = self.widths[index];
        if width.is_finite() && width > 0. {
            width * 0.5
        } else {
            0.
        }
    }
    pub fn bounds(&self) -> Option<crate::bounds::Bounds> {
        let mut points = self.coordinates.chunks_exact(3);
        let p = points.next()?;
        let mut bounds = crate::bounds::Bounds::point([p[0], p[1], p[2]]);
        for p in points {
            bounds.include([p[0], p[1], p[2]]);
        }
        Some(bounds)
    }
    pub fn bytes(&self) -> usize {
        self.coordinates.capacity() * 8
            + self.display.capacity() * 4
            + (self.colors.capacity() + self.widths.capacity() + self.entities.capacity()) * 4
    }
}
impl crate::page_cache::ByteSize for Page {
    fn bytes(&self) -> usize {
        Page::bytes(self)
    }
}

