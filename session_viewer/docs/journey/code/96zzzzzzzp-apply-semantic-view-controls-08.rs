    if name=="View Xray"{let on=crate::element_input::visibility(&words[count..])?;return Ok(Command::Editor(Action::ViewXray(on)));}
    if name=="View Lighting"||name=="View Backfaces"