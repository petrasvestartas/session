use crate::command_dock;

pub(crate) struct Commands(pub Vec<&'static str>);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        crate::commands::canonical(line)
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        let prefix = line.to_ascii_lowercase();
        let mut choices = self.0.iter().copied()
            .filter(|name| name.to_ascii_lowercase().starts_with(&prefix))
            .collect::<Vec<_>>();
        choices.sort_by_key(|name| {
            if name.eq_ignore_ascii_case(line) { 0 }
            else if name.get(line.len()..).is_some_and(|rest| rest.starts_with(' ')) { 1 }
            else { 2 }
        });
        choices
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

