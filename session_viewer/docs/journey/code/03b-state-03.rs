
pub(crate) mod theme;
pub(crate) mod view;

/// The command vocabulary supplied by the application using the dock.
pub trait Commands {
    fn canonical(&self, line: &str) -> String;
    fn choosing_option(&self, line: &str) -> bool;
    fn draws(&self, line: &str) -> bool;
    fn options(&self, line: &str) -> &'static [&'static str];
    fn option_label<'a>(&self, line: &'a str) -> &'a str;
    fn accept(&self, line: &str) -> (String, bool);
    fn completions(&self, line: &str) -> Vec<&'static str>;
    fn browse(&self, line: &str) -> Vec<&'static str>;
}

/// What the command line shows and remembers between frames.
#[derive(Default)]
