        if !meshes.iter().any(|mesh|mesh.printed()&&mesh.geometry.source.print_text()==text){return;}
        let color=self.color.borrow();let Some(color)=color.as_ref()else{return;};