use super::selecting;
use crate::State;
use crate::app::command::{Action, Spec};

pub const SPEC: Spec = Spec {
    names: &["Select By Name"],
    aliases: &[],
    hint: "Select By Name text · selects visible objects whose name contains the text, any case · Example: Select By Name beam",
    options: &[],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// The text after the verb, spaces kept between words, surrounding quotes dropped.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let joined = rest.join(" ");
    let text = joined.trim();
    let text = ['"', '\'']
        .iter()
        .find_map(|quote| text.strip_prefix(*quote)?.strip_suffix(*quote))
        .unwrap_or(text)
        .trim();

    if text.is_empty() {
        return Err("type a name after Select By Name; try `Select By Name beam`".into());
    }

    Ok(Box::new(SelectByName(text.to_owned())))
}

#[derive(Debug)]
struct SelectByName(String);

impl Action for SelectByName {
    /// Select every visible, unlocked object whose shown name contains the text.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let text = self.0.to_lowercase();
        let found: Vec<u32> = selecting::candidates(state)
            .into_iter()
            .filter(|&row| matches(state.scene.object_name(row), &text))
            .collect();

        if found.is_empty() {
            return Err(format!(
                "no visible object's name contains `{}`; the selection is unchanged",
                self.0
            ));
        }

        let count = selecting::apply(state, found, false, false);
        Ok(format!("{count} selected: names containing `{}`", self.0))
    }
}

/// True when `name` contains `text`, already lowercase, in any case.
fn matches(name: &str, text: &str) -> bool {
    name.to_lowercase().contains(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::{accept, parse};

    /// Equal names and substrings match in any case; a longer text does not.
    #[test]
    fn names_match_by_substring_in_any_case() {
        assert!(matches("Beam A1", "beam a1"));
        assert!(matches("my_Polyline", "poly"));
        assert!(matches("Träger 1", &"TRÄGER".to_lowercase()));
        assert!(matches("beam", "beam"));
        assert!(!matches("beam", "beams"));
    }

    /// The bare verb waits for its text; the text keeps single spaces and loses its quotes.
    #[test]
    fn select_by_name_waits_for_its_text() {
        assert_eq!(accept("Select By Name"), ("Select By Name ".into(), false));
        assert_eq!(accept("Select By Name "), ("Select By Name ".into(), false));
        assert_eq!(accept("selectbyname"), ("Select By Name ".into(), false));
        assert_eq!(accept("sel"), ("Select By Name ".into(), false));
        assert!(parse("Select By Name").is_err());
        assert!(parse("Select By Name \"\"").is_err());
        let text = |line: &str| parse(line).map(|action| format!("{action:?}"));
        assert_eq!(
            text("select by name  Beam   A1"),
            Ok("SelectByName(\"Beam A1\")".into())
        );
        assert_eq!(
            text("selectbyname \"my beam\""),
            Ok("SelectByName(\"my beam\")".into())
        );
        assert_eq!(text("Select By Name 'x'"), Ok("SelectByName(\"x\")".into()));
    }
}
