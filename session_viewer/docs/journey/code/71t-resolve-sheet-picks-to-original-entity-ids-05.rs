    pub fn describe(self) -> String {
        match self.part {
            Part::SheetEntity(id)=>format!("Object {} · original sheet entity {id}",self.owner.value()),
            Part::CloudPoint(id)=>format!("Object {} · original point {id}",self.owner.value()),
            Part::Object => format!("Object {}", self.owner.value()),
