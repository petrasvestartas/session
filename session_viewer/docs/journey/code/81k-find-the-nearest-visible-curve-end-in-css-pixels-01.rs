        Ok(Reach::Boundaries { cutters, meshes })
    }
    pub fn nearest(
        &self,
        screen: &crate::snap_math::Screen,
        at: [f64; 2],
    ) -> Option<(f64, usize, End)> {
        let mut best: Option<(f64, usize, End)> = None;
        for (index, (_, points, bounds)) in self.candidates.iter().enumerate() {
            if !boxed(screen, bounds, at, 12.) {
                continue;
            }
            let total = points.windows(2).map(|p| apart(p[0], p[1])).sum::<f64>();
            let mut walked = 0.;
            for pair in points.windows(2) {
                let step = apart(pair[0], pair[1]);
                if let (Some(a), Some(b)) = (screen.point(pair[0]), screen.point(pair[1])) {
                    let distance =
                        source_cut::segment_px((a[0], a[1]), (b[0], b[1]), (at[0], at[1]));
                    if distance <= 12. && best.is_none_or(|(near, ..)| distance < near) {
                        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
                        let t = (((at[0] - a[0]) * dx + (at[1] - a[1]) * dy)
                            / (dx * dx + dy * dy).max(1e-12))
                        .clamp(0., 1.);
                        best = Some((
                            distance,
                            index,
                            if walked + t * step < total * 0.5 {
                                End::Start
                            } else {
                                End::End
                            },
                        ));
                    }
                }
                walked += step;
            }
        }
        best
    }
}
fn apart(a: [f64; 3], b: [f64; 3]) -> f64 {
