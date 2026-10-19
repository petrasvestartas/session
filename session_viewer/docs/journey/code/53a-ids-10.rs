    }

    pub fn contains(&self,id:ObjectId)->bool {self.ids.contains(&id)}

    pub fn next(&self, current: Option<ObjectId>) -> Option<ObjectId> {
