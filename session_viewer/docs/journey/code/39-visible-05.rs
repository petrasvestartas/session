        for row in self.paths.iter().filter(|row| row.visible()) {
            for point in row.bounds_points() {