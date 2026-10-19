        if let Action::ViewInk(kind,on)=&action {
            let(flag,name)=match kind{crate::ink_ranges::Family::Edges=>(&mut self.ink_visibility.edges,"Edges"),crate::ink_ranges::Family::Lines=>(&mut self.ink_visibility.lines,"Lines"),crate::ink_ranges::Family::Points=>(&mut self.ink_visibility.points,"Points")};
            *flag=*on;self.tool_status=format!("View {name} {}",if *on{"On"}else{"Off"});return Ok(Change::View);
        }
        if let Action::ViewXray(on)=&action {
            let enabled=on.unwrap_or(self.opacity!=0.);if enabled!=(self.opacity==0.){self.opacity=if enabled{0.}else{1.};}
            self.tool_status=format!("View Xray {}",if enabled{"On"}else{"Off"});return Ok(Change::View);
        }
        if let Action::ViewOutline(on)=&action