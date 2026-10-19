    pub fn apply(&mut self, action: Action) -> Result<Change, &'static str> {
        let action=match action{Action::Touch(actions)=>{let mut change=Change::View;for action in actions{let next=self.apply(action)?;if next!=Change::View{change=next;}}return Ok(change)},action=>action};
        if let Action::Opacity(value)=&action{if !value.is_finite()||!(0.0..=1.0).contains(value){return Err("Opacity takes a value from0 to1")}self.opacity=*value;self.tool_status=format!("Opacity {value}");return Ok(Change::View)}
        if let Action::ViewOutline(on)=&action{self.outlines=on.unwrap_or(!self.outlines);self.tool_status=format!("View Outline {}",if self.outlines{"On"}else{"Off"});return Ok(Change::View)}
        if let Action::Arctic(on)=&action{self.arctic=on.unwrap_or(!self.arctic);if self.arctic{self.outlines=true;}self.tool_status=format!("Arctic {}",if self.arctic{"On"}else{"Off"});return Ok(Change::View)}
