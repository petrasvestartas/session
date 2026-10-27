        let mut display = Self::from_render(render)?;
        display.print_fill=mesh.widths().len()==1&&mesh.widths()[0]==0.;
        display.print_text=display.print_fill&&mesh.name=="text";
