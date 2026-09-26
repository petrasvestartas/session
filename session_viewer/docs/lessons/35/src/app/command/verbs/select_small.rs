use super::selecting;
use crate::State;
use crate::app::command::{Action, Spec, number};
use session_rust::AABB;

pub const SPEC: Spec = Spec {
    names: &["Select Small"],
    aliases: &[],
    hint: "Select Small length · selects visible objects whose bounding-box diagonal is shorter, in scene units · Example: Select Small 10",
    options: &[],
    arity: Some(1),
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// One length above zero.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let length = number(rest.first().copied(), "Select Small 10")?;

    if length <= 0.0 {
        return Err("Select Small wants a length above zero".into());
    }

    Ok(Box::new(SelectSmall(length)))
}

#[derive(Debug)]
struct SelectSmall(f64);

impl Action for SelectSmall {
    /// Select every visible, unlocked object whose world box diagonal is below the length.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let found: Vec<u32> = selecting::candidates(state)
            .into_iter()
            .filter(|&row| {
                state
                    .gpu
                    .objects
                    .row_bounds(row)
                    .is_some_and(|bounds| small(&bounds, self.0))
            })
            .collect();

        if found.is_empty() {
            return Err(format!(
                "no visible object's bounding-box diagonal is below {}; the selection is unchanged",
                self.0
            ));
        }

        let count = selecting::apply(state, found, false, false);
        Ok(format!(
            "{count} selected: bounding-box diagonal below {}",
            self.0
        ))
    }
}

/// True when the box is valid and its diagonal is strictly shorter than `length`.
fn small(bounds: &AABB, length: f64) -> bool {
    bounds.is_valid() && bounds.diagonal() < length
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::{accept, parse};

    /// One number above zero, nothing else.
    #[test]
    fn select_small_takes_one_positive_length() {
        let text = |line: &str| parse(line).map(|action| format!("{action:?}"));
        assert_eq!(text("Select Small 10"), Ok("SelectSmall(10.0)".into()));
        assert_eq!(text("selectsmall 2.5"), Ok("SelectSmall(2.5)".into()));

        for line in [
            "Select Small 0",
            "Select Small -1",
            "Select Small abc",
            "Select Small 1 2",
            "Select Small",
        ] {
            assert!(parse(line).is_err(), "{line}");
        }

        assert_eq!(accept("Select Small"), ("Select Small ".into(), false));
        assert_eq!(accept("Select Sm"), ("Select Small ".into(), false));
    }

    /// The diagonal must be strictly shorter; an empty box is never small.
    #[test]
    fn only_a_shorter_diagonal_is_small() {
        let bounds = AABB::new(0.0, 0.0, 0.0, 3.0, 4.0, 0.0);
        assert!(!small(&bounds, 10.0));
        assert!(small(&bounds, 10.5));
        assert!(!small(&AABB::empty(), 1.0));
    }
}
