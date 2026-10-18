#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shortcut { Undo, Redo, Fit, Hide, Show, Delete, SelectAll, Controls }

impl Shortcut {
    pub fn command(self) -> &'static str {
        match self { Self::Undo => "Undo", Self::Redo => "Redo", Self::Fit => "Fit", Self::Hide=>"Hide",Self::Show=>"Show",Self::Delete=>"Delete",Self::SelectAll=>"Select All",Self::Controls=>"Controls" }
    }
}
