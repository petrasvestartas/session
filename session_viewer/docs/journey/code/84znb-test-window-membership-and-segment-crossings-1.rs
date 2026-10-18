f {
            box_: Some([left, top, right, bottom]),
            ..Self::new(&[(left, top), (right, top), (right, bottom), (left, bottom)])
        }
    }

    /// True when the segment from `a` to `b` meets the rectangle; false for a loop that is no rectangle.
    fn crosses(&self, a: (f64, f64), b: (f64, f64)) -> bool {
        let Some([left, top, right, bottom]) = self.box_ else {
            return false;
        };
        // clip the segment's parameter range to each side in turn (Liang-Barsky)
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let (mut t0, mut t1) = (0.0_f64, 1.0_f64);

        for (p, q) in [
            (-dx, a.0 - left),
            (dx, right - a.0),
            (-dy, a.1 - top),
            (dy, bottom - a.1),
        ] {
            if p == 0.0 {
                if q < 0.0 {
                    return false;
                }
                continue;
            }

            let t = q / p;

            if p < 0.0 {
                t0 = t0.max(t);
            } else {
                t1 = t1.min(t);
            }
        }

        t0 <= t1
    }

    /// True when `at` is inside by the even-odd rule, judged on its row's centre line.
    pub(crate) fn contains(&self, at: (f64, f64)) -> bool {
        let row = (at.1 - self.top).floor();

        if !(row >= 0.0 && (row as usize) < self.rows.len()) {
            return false;
        }

        self.rows[row as usize].partition_point(|x| *x < at.0) % 2 == 1
    }
