        let scene_far = self.distance + self.radius * 2.0;
        let far = self.grid.and_then(|grid| grid.reach(self.target).ok()).map_or(scene_far, |reach|
            (self.distance * 10.0).max(self.distance + 2.0 * self.radius.max(reach)));