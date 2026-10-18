    /// A knot vector, each knot rounded to `SAME_NUMBER` of the whole span.
    fn push_knots(&mut self, knots: &[f64]) {
        let span = knots.last().zip(knots.first()).map_or(0.0, |(z, a)| z - a);
        self.words.push(knots.len() as u64);

        if !(span.is_finite() && span > 0.0) {
            self.words.extend(knots.iter().map(|k| k.to_bits()));
            return;
        }

        for knot in knots {
            self.words
                .push(((knot - knots[0]) / (span * SAME_NUMBER)).round() as i64 as u64);
        }

        self.push_number(knots[0] / span);
        self.push_number(span);
    }

    /// Pcurve control values, rounded to `SAME_NUMBER` of the largest of them.
    fn push_parameters(&mut self, values: &[f64]) {
        let size = values.iter().fold(1.0, |m: f64, v| m.max(v.abs()));
        self.words.push(values.len() as u64);
        self.words.extend(
            values
                .iter()
                .map(|v| (v / (size * SAME_NUMBER)).round() as i64 as u64),
        );
    }

    /// A 3D point in the BRep's frame, rounded to the step.
    fn push_point(&mut self, p: [f64; 3]) {
        for value in self.frame.local(p) {
            self.words.push((value / self.step).round() as i64 as u64);
        }
    }

    fn push_color(&mut self, color: &Color) {
        for value in [color.r, color.g, color.b, color.a] {
            self.words.push(value.to_bits() as u64);
        }
    }

    fn push_curve_shape(&mut self, curve: &NurbsCurve) {
        self.push_words(&[
            curve.m_dim as u64,
            curve.m_is_rat as u64,
            curve.m_order as u64,
            curve.m_cv_count as u64,
        ]);
        self.push_knots(&curve.m_nurbsknot);
    }

    fn push_refs(&mut self, refs: &[BRepRef]) {
        self.words.push(refs.len() as u64);

        for r in refs {
            self.push_words(&[r.index as i64 as u64, r.orientation as u64]);
        }
    }

    fn push_counts(&mut self, b: &BRep) {
        self.push_words(&[
            b.m_surfaces.len() as u64,
            b.m_curves_3d.len() as u64,
            b.m_curves_2d.len() as u64,
            b.m_vertices.len() as u64,
            b.m_edges.len() as u64,
            b.m_wires.len() as u64,
            b.m_faces.len() as u64,
            b.m_shells.len() as u64,
            b.m_solids.len() as u64,
        ]);
    }

    fn push_topology(&mut self, b: &BRep) {
        for edge in &b.m_edges {
            self.push_words(&[
                edge.curve_3d_index as i64 as u64,
                edge.start_vertex as i64 as u64,
                edge.end_vertex as i64 as u64,
                edge.degenerated as u64,
                edge.pcurves.len() as u64,
            ]);

            for pcurve in &edge.pcurves {
                self.push_words(&[
                    pcurve.surface_index as i64 as u64,
                    pcurve.curve_2d_index as i64 as u64,
                    pcurve.curve_2d_index_2 as i64 as u64,
                ]);
            }
        }

        for wire in &b.m_wires {
            self.push_refs(&wire.edges);
        }

        for face in &b.m_faces {
            self.words.push(face.surface_index as i64 as u64);
            self.push_refs(&face.wires);

            match &face.facecolor {
                Some(color) => self.push_color(color),
                None => self.words.push(u64::MAX),
            }
        }

        for shell in &b.m_shells {
            self.push_refs(&shell.faces);
        }

        for solid in &b.m_solids {
            self.push_refs(&solid.shells);
        }
    }
