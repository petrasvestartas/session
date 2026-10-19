        let setting=match &action {Action::Lighting(on)=>Some((&mut self.lit,*on,"Lighting")),Action::Backfaces(on)=>Some((&mut self.backfaces,*on,"Backfaces")),_=>None};
        if let Some((value,on,label))=setting {*value=on.unwrap_or(!*value);self.tool_status=format!("{label} {}",if *value{"On"}else{"Off"});return Ok(Change::View)}
        if let Action::Arctic(on)=&action{