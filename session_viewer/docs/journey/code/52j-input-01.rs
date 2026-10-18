}

pub struct Panel {
    pub tree:crate::tree_panel::TreePanel,
    number:crate::number_box::NumberBox,
    commands: Commands,
    pub consumed: bool,
