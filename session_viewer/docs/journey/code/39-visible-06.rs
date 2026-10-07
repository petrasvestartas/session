        for row in self.points.iter().filter(|row| row.visible()) {
            let p = row.point();