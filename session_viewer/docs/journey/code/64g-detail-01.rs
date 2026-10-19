    stack: Vec<(usize, usize)>, // nodes still to visit, with parent
    visits: Vec<Visit>,         // nodes visited
}

impl LodWalk {
    pub fn select(&mut self, p: &Projection, c: &Cloud, model: &[f32; 16]) {
        self.ranges.clear();

        if c.node_count == 0 || p.lod_px <= 0.0 || c.resident < LOD_MIN_POINTS {
            self.ranges.push(Range {
                first: 0,
                count: c.resident,
                spacing: c.spacing,
                tile: false,
            });
            return;
        }

        let base = c.node_first as usize;
        let scale = Xform::from_matrix(model.map(f64::from)).uniform_scale();
        self.stack.clear();
        self.visits.clear();
        self.stack.push((0, usize::MAX));

        while let Some((n, parent)) = self.stack.pop() {
            let Some(node) = p.nodes.get(base + n) else {
                continue;
            };

            if node.first >= c.resident {
                continue;
            }

            let count = node.count.min(c.resident - node.first);
            let slot = self.visits.len();
            self.visits.push(Visit {
                first: node.first,
                count,
                spacing: node.spacing,
                parent,
            });

            if projected_spacing(p, node, model, scale) > p.lod_px as f64 {
                for &child in &node.children {
                    if child >= 0 {
                        self.stack.push((child as usize, slot));
                    }
                }
            }
        }

        for i in (0..self.visits.len()).rev() {
            let (fine, parent) = (self.visits[i].spacing, self.visits[i].parent);

            if parent != usize::MAX && fine < self.visits[parent].spacing {
                self.visits[parent].spacing = fine;
            }
        }

        for v in &self.visits {
            if v.count > 0 {
                self.ranges.push(Range {
                    first: v.first,
                    count: v.count,
                    spacing: v.spacing,
                    tile: true,
                });
            }
        }
    }
}

