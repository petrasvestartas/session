            Action::ViewInk(..)|Action::ViewXray(_)=>unreachable!("View presentation returned before document mutation"),
            Action::ViewOutline(_)=>unreachable!