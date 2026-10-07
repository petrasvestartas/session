        for line in self.lines.iter().filter(|line| line.visible()) {
            for point in line.endpoints() {