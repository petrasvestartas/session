use crate::editor::Action;
pub const NAMES: &[&str] = &[
    "Project To Plane",
    "Project To Plane CPlane",
    "Project To Plane XY",
    "Project To Plane YZ",
    "Project To Plane ZX",
    "Project To Plane 3Point",
    "Extend",
    "Extend Distance",
];
pub fn action(name: &str, words: &[&str]) -> Option<Result<Action, String>> {
    if let Some(option) = name.strip_prefix("Project To Plane") {
        let args = option
            .split_whitespace()
            .chain(words.iter().copied())
            .map(str::to_owned)
            .collect();
        return Some(Ok(Action::Project(args)));
    }
    if let Some(option) = name.strip_prefix("Extend") {
        let words = option
            .split_whitespace()
            .chain(words.iter().copied())
            .collect::<Vec<_>>();
        let positive = |word: &str| {
            word.parse::<f64>()
                .ok()
                .filter(|v| v.is_finite() && *v > 0.)
                .ok_or_else(|| "The distance must be above zero".to_owned())
        };
        return Some(match words.as_slice() {
            [] => Ok(Action::ExtendBegin {
                distance: None,
                asking: false,
            }),
            [name] if name.eq_ignore_ascii_case("Distance") => Ok(Action::ExtendBegin {
                distance: None,
                asking: true,
            }),
            [name, value] if name.eq_ignore_ascii_case("Distance") => {
                positive(value).map(|value| Action::ExtendBegin {
                    distance: Some(value),
                    asking: false,
                })
            }
            [value] => positive(value).map(|value| Action::ExtendBegin {
                distance: Some(value),
                asking: false,
            }),
            [a, b] => match (a.parse::<f64>(), b.parse::<f64>()) {
                (Ok(a), Ok(b)) => Ok(Action::Interval(
                    crate::interval_geometry::Interval::Extend(a, b),
                )),
                _ => Err("Extend needs a distance or two domain parameters".into()),
            },
            _ => Err("Extend needs a distance or two domain parameters".into()),
        });
    }
    None
}
