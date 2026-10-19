        })
    }
    pub fn accept(&mut self, added: &[[f64; 3]]) -> Result<Option<Plane>, String> {
        let mut points = self.points.clone();
        for point in added {
            points.push(crate::tool_coords::checked(*point).map_err(str::to_owned)?);
            if points.len() > 3 {
                return Err("Project 3Point takes three plane points".into());
            }
            if points.len() == 2
                && (0..3)
                    .map(|i| (points[1][i] - points[0][i]).powi(2))
                    .sum::<f64>()
                    <= 1e-24
            {
                return Err("Pick a point away from the first one".into());
            }
        }
        let plane = if points.len() == 3 {
            let p = points
                .iter()
                .map(|p| Point::new(p[0], p[1], p[2]))
                .collect::<Vec<_>>();
            Some(crate::project_geometry::plane_of(&p[0], &p[1], &p[2])?)
        } else {
            None
        };
        self.points = points;
        Ok(plane)
    }
}
