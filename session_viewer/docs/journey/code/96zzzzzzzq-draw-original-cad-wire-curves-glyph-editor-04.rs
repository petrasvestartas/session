            let enabled=on.unwrap_or(self.opacity!=0.);if enabled!=(self.opacity==0.){self.opacity=if enabled{0.}else{1.};}
            self.tool_status=format!("View Xray {}",if enabled{"On"}else{"Off"});return Ok(Change::View);
        }
        if let Action::PointSize(size)=&action{self.display.point_size(*size)?;self.tool_status=format!("View Point Size {}",self.display.cloud_scale);return Ok(Change::View)}
        if let Action::PlaneSize(size)=&action{if !size.is_finite()||*size<=0.0||!(*size as f32).is_finite(){return Err("View Plane Size needs a positive number of mm")}self.plane_size=*size;self.tool_status=format!("View Plane Size {size}");return Ok(Change::View)}
        if let Action::ViewOutline(on)=&action{self.outlines=on.unwrap_or(!self.outlines);self.tool_status=format!("View Outline {}",if self.outlines{"On"}else{"Off"});return Ok(Change::View)}
        let setting=match &action {Action::Lighting(on)=>Some((&mut self.lit,*on,"Lighting")),Action::Backfaces(on)=>Some((&mut self.backfaces,*on,"Backfaces")),_=>None};
