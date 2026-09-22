pub mod verbs;

use crate::State;
use crate::app::coords;

/// What a parsed command does.
pub trait Action: std::fmt::Debug {
    /// Carry it out; the answer is what to show the person.
    fn run(&self, state: &mut State) -> Result<String, String>;

    /// True when it acts on the selection.
    fn needs_selection(&self) -> bool {
        false
    }

    /// True when a shape being drawn survives it.
    fn keeps_draft(&self) -> bool {
        false
    }

    /// True when a split waiting for cutters survives it.
    fn keeps_split(&self) -> bool {
        false
    }

    /// True when it refuses a scene with streamed sources.
    fn needs_complete_scene(&self) -> bool {
        false
    }
}

/// One verb: how it is typed, completed and parsed.
pub struct Spec {
    pub names: &'static [&'static str], // shown in completion, first is canonical
    pub aliases: &'static [&'static str], // accepted when typed, never shown
    pub hint: &'static str,             // help line, empty for the generic one
    pub options: &'static [&'static str], // clickable choices
    pub arity: Option<usize>,           // exact argument count, when fixed
    pub wait_for_option: bool,          // completing the bare verb waits for an option
    pub wait_after_option: bool,        // completing verb and option waits for more
    pub parse: fn(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String>,
}

impl Spec {
    /// True when this verb is typed as `word`.
    fn answers_to(&self, word: &str) -> bool {
        let same = |name: &&str| name.eq_ignore_ascii_case(word);
        self.names.iter().any(same) || self.aliases.iter().any(same)
    }
}

/// Every verb the command line knows.
pub const REGISTRY: &[&Spec] = &[
    &verbs::point::SPEC,      // register:point
    &verbs::line::SPEC,       // register:line
    &verbs::polyline::SPEC,   // register:polyline
    &verbs::curve::SPEC,      // register:curve
    &verbs::trim::SPEC,       // register:trim
    &verbs::extend::SPEC,     // register:extend
    &verbs::explode::SPEC,    // register:explode
    &verbs::r#move::SPEC,     // register:move
    &verbs::rotate::SPEC,     // register:rotate
    &verbs::scale::SPEC,      // register:scale
    &verbs::split::SPEC,      // register:split
    &verbs::save::SPEC,       // register:save
    &verbs::open::SPEC,       // register:open
    &verbs::delete::SPEC,     // register:delete
    &verbs::undo::SPEC,       // register:undo
    &verbs::redo::SPEC,       // register:redo
    &verbs::hide::SPEC,       // register:hide
    &verbs::show::SPEC,       // register:show
    &verbs::fit::SPEC,        // register:fit
    &verbs::escape::SPEC,     // register:escape
    &verbs::layers::SPEC,     // register:layers
    &verbs::opacity::SPEC,    // register:opacity
    &verbs::attributes::SPEC, // register:attributes
    &verbs::snap::SPEC,       // register:snap
    &verbs::ssao::SPEC,       // register:ssao
    &verbs::arctic::SPEC,     // register:arctic
    &verbs::object::SPEC,     // register:object
    &verbs::edge::SPEC,       // register:edge
    &verbs::face::SPEC,       // register:face
    &verbs::controls::SPEC,   // register:controls
];

/// The verb typed at the start of the line.
fn first_word(line: &str) -> String {
    line.split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
}

/// The spec a typed word names, alias or not.
fn spec(word: &str) -> Option<&'static Spec> {
    REGISTRY
        .iter()
        .copied()
        .find(|spec| spec.answers_to(word))
}

/// The spec whose canonical name is exactly this word.
fn spec_by_name(word: &str) -> Option<&'static Spec> {
    REGISTRY
        .iter()
        .copied()
        .find(|spec| spec.names.iter().any(|name| name.eq_ignore_ascii_case(word)))
}

/// Help text for the verb being typed.
pub fn hint(line: &str) -> &'static str {
    match spec(&first_word(line)) {
        Some(spec) if !spec.hint.is_empty() => spec.hint,
        _ => "Type a command · Up/Down browse · Tab completes · Enter executes · Esc cancels",
    }
}

/// Parse one line; the error is the message to show.
pub fn parse(line: &str) -> Result<Box<dyn Action>, String> {
    if line.len() > 65536 {
        return Err("command exceeds 64 KiB".into());
    }

    let line = line.trim();

    if line.is_empty() {
        return Err("nothing typed".into());
    }

    let mut words = line.split_whitespace();
    let verb = words.next().unwrap_or_default().to_ascii_lowercase();
    let rest: Vec<&str> = words.collect(); // the arguments
    let Some(spec) = spec(&verb) else {
        return Err(format!("no command `{verb}`"));
    };

    if spec.arity.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }

    (spec.parse)(&verb, &rest)
}

/// Clickable choices for the verb being typed.
pub fn options(line: &str) -> &'static [&'static str] {
    spec(&first_word(line)).map_or(&[], |spec| spec.options)
}

/// Commands or options starting with the typed text.
pub fn completions(line: &str) -> Vec<&'static str> {
    let lower = line.to_ascii_lowercase();
    // after a space, complete the option instead
    if lower.contains(' ') {
        return options(line)
            .iter()
            .copied()
            .filter(|name| name.to_ascii_lowercase().starts_with(&lower))
            .collect();
    }

    let mut names: Vec<&'static str> = REGISTRY.iter().flat_map(|spec| spec.names).copied().collect();
    names.sort_by_key(|name| name.to_ascii_lowercase());
    names.retain(|name| name.to_ascii_lowercase().starts_with(&lower));
    names
}

/// Every command, matching ones first.
pub fn browse(line: &str) -> Vec<&'static str> {
    if line.contains(' ') {
        return options(line).to_vec();
    }

    let all = completions("");
    let lower = line.to_ascii_lowercase();
    all.iter()
        .copied()
        .filter(|name| name.to_ascii_lowercase().starts_with(&lower))
        .chain(
            all.iter()
                .copied()
                .filter(|name| !name.to_ascii_lowercase().starts_with(&lower)),
        )
        .collect()
}

/// Take the first completion; false when arguments are still needed.
pub fn accept(line: &str) -> (String, bool) {
    if line.trim().is_empty() {
        return (String::new(), true);
    }

    let choices = completions(line);
    let text = choices.first().copied().unwrap_or(line).trim();
    let words: Vec<_> = text.split_whitespace().collect();
    let typed = spec_by_name(words.first().copied().unwrap_or(""));
    let bare_verb_waits =
        words.len() == 1 && typed.is_some_and(|spec| spec.wait_for_option);
    let option_waits = words.len() == 2
        && typed.is_some_and(|spec| spec.wait_after_option)
        && spec_option_continues(typed, words[1]);

    if bare_verb_waits || option_waits {
        (format!("{text} "), false)
    } else {
        (text.to_owned(), true)
    }
}

/// True when the typed option is one this verb continues after.
fn spec_option_continues(spec: Option<&'static Spec>, word: &str) -> bool {
    spec.is_some_and(|spec| {
        spec.options
            .iter()
            .any(|option| option.split_whitespace().nth(1) == Some(word))
    })
}

/// A move offset from `10 0 0`, `@10,0` or `10<45`.
pub fn offset(words: &[&str]) -> Result<[f64; 3], String> {
    let joined = words.join(" ");
    // spaces between numbers become commas
    let text = if joined.contains(',') || joined.contains('<') {
        joined.replace(' ', "")
    } else {
        words.join(",").replacen("@,", "@", 1)
    };
    let Some(typed) = coords::parse(&text) else {
        return Err(format!("`{joined}` is not an offset; try `move 10 0 0`"));
    };

    match typed {
        coords::Typed::Absolute { x, y, z } | coords::Typed::Relative { x, y, z } => {
            Ok([x, y, z.unwrap_or(0.0)])
        }
        coords::Typed::Polar { distance, degrees } => {
            // polar in the world XY plane
            let r = degrees.to_radians();
            Ok([distance * r.cos(), distance * r.sin(), 0.0])
        }
        coords::Typed::Distance(d) => Ok([d, 0.0, 0.0]), // a bare number moves along x
    }
}

/// An axis letter and a number.
pub fn axis_and_number(
    words: &[&str],
    example: &str,
) -> Result<(crate::app::gizmo::Axis, f64), String> {
    use crate::app::gizmo::Axis;
    let axis = match words.first().map(|w| w.to_ascii_lowercase()) {
        Some(a) if a == "x" => Axis::X,
        Some(a) if a == "y" => Axis::Y,
        Some(a) if a == "z" => Axis::Z,
        _ => return Err(format!("which axis? try `{example}`")),
    };
    Ok((axis, number(words.get(1).copied(), example)?))
}

/// A finite number, or a message with the example.
pub fn number(word: Option<&str>, example: &str) -> Result<f64, String> {
    let Some(word) = word else {
        return Err(format!("missing a number; try `{example}`"));
    };

    match word.parse::<f64>() {
        Ok(v) if v.is_finite() => Ok(v),
        _ => Err(format!("`{word}` is not a number")),
    }
}

/// `On`, `Off`, or nothing for a toggle.
pub fn on_off(words: &[&str], usage: &str) -> Result<Option<bool>, String> {
    match words {
        [] => Ok(None),
        [value] if value.eq_ignore_ascii_case("on") => Ok(Some(true)),
        [value] if value.eq_ignore_ascii_case("off") => Ok(Some(false)),
        _ => Err(usage.into()),
    }
}

/// A modeling command from its verb and points.
pub fn model(verb: &str, words: &[&str]) -> Result<crate::app::modeling::Modeling, String> {
    use crate::app::modeling::Modeling;

    match verb {
        "explode" if words.is_empty() => Ok(Modeling::Explode),
        "trim" | "extend" if words.len() == 2 => {
            let a = number(words.first().copied(), "trim 0.2 0.8")?;
            let b = number(words.get(1).copied(), "trim 0.2 0.8")?;
            Ok(if verb == "trim" {
                Modeling::Trim(a, b)
            } else {
                Modeling::Extend(a, b)
            })
        }
        "point" | "line" | "polyline" | "curve" => {
            let mut points = Vec::new();

            if words.len() > crate::app::modeling::MAX_POINTS {
                return Err("too many points".into());
            }

            for word in words {
                let Some(coords::Typed::Absolute { x, y, z }) = coords::parse(word) else {
                    return Err("use world coordinates x,y,z separated by spaces".into());
                };
                points.push([x, y, z.unwrap_or(0.0)]);
            }

            match (verb, points.len()) {
                ("point", 1) => Ok(Modeling::Point(points[0])),
                ("line", 2) => Ok(Modeling::Line(points[0], points[1])),
                ("polyline", 2..) => Ok(Modeling::Polyline(points)),
                ("curve", 2..) => Ok(Modeling::Curve(points)),
                _ => {
                    Err("point needs one coordinate; line two; polyline/curve at least two".into())
                }
            }
        }
        _ => Err("try trim 0.2 0.8, extend -0.2 1.2, or explode".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// What a parsed line becomes, for comparing in tests.
    fn parsed(line: &str) -> Result<String, String> {
        parse(line).map(|action| format!("{action:?}"))
    }

    /// Tab completes the verb, then its option.
    #[test]
    fn partial_entries_accept_commands_then_options() {
        assert_eq!(accept("Lay"), ("Layers ".into(), false));
        assert_eq!(accept("Layers "), ("Layers On".into(), true));
        assert_eq!(accept("Layers of"), ("Layers Off".into(), true));
        assert_eq!(accept("Lin"), ("Line".into(), true));
        assert_eq!(accept("Point"), ("Point".into(), true));
        assert_eq!(accept("Polyline"), ("Polyline".into(), true));
        assert_eq!(accept("Polyline rec"), ("Polyline Rectangle".into(), true));
        assert_eq!(accept("Polyline pol"), ("Polyline Polygon".into(), true));
        assert_eq!(accept("Rotate "), ("Rotate x ".into(), false));
        assert_eq!(accept("Rotate x 45"), ("Rotate x 45".into(), true));
        assert_eq!(accept(""), (String::new(), true));
        assert_eq!(browse("la")[0], "Layers");
        assert_eq!(browse("la").len(), completions("").len());
        assert_eq!(browse("forgot"), completions(""));
        assert_eq!(browse("Layers o"), vec!["Layers On", "Layers Off"]);
    }

    /// Completion and parsing ignore case.
    #[test]
    fn discovery_and_layer_options_are_case_insensitive() {
        assert_eq!(completions("la"), vec!["Layers"]);
        assert_eq!(completions("Layers "), vec!["Layers On", "Layers Off"]);
        assert!(completions("").contains(&"Controls"));
        assert_eq!(parsed("Layers OFF"), Ok("Layers(Some(false))".into()));
        assert_eq!(parsed("Attributes off"), Ok("Attributes(Some(false))".into()));
        assert_eq!(parsed("Attributes"), Ok("Attributes(None)".into()));
        assert_eq!(parsed("Opacity 0.5"), Ok("Opacity(0.5)".into()));
        assert!(parsed("Opacity 2").is_err());
        assert!(parsed("Opacity").is_err());
        assert!(parsed("Layers maybe").is_err());
    }

    /// The whole command list, in the order the command line shows it.
    #[test]
    fn every_verb_is_offered_in_alphabetical_order() {
        assert_eq!(
            completions(""),
            vec![
                "Arctic",
                "Attributes",
                "Controls",
                "Curve",
                "Delete",
                "Edge",
                "Escape",
                "Explode",
                "Extend",
                "Face",
                "Fit",
                "Hide",
                "Layers",
                "Line",
                "Move",
                "Object",
                "Opacity",
                "Open",
                "Point",
                "Polyline",
                "Redo",
                "Rotate",
                "Save",
                "Scale",
                "Show",
                "Snap",
                "Split",
                "SSAO",
                "Trim",
                "Undo",
            ]
        );
    }

    /// Short forms parse like the full verb.
    #[test]
    fn the_verbs_and_their_short_forms() {
        assert_eq!(parsed("delete"), Ok("Delete".into()));
        assert_eq!(parsed("del"), Ok("Delete".into()));
        assert_eq!(parsed("  UNDO "), Ok("Undo".into()));
        assert_eq!(parsed("fit"), Ok("Fit".into()));
    }

    /// Move accepts every coordinate form.
    #[test]
    fn move_reads_the_same_coordinates_as_the_rest_of_the_viewer() {
        assert_eq!(parsed("move 10 0 0"), Ok("Move([10.0, 0.0, 0.0])".into()));
        assert_eq!(parsed("m @0 5"), Ok("Move([0.0, 5.0, 0.0])".into()));
        let polar = offset(&["10<90"]).expect("a polar offset is an offset");
        assert!((polar[1] - 10.0).abs() < 1e-9, "90 degrees is +y");
    }

    /// A bad line gets a message naming the problem.
    #[test]
    fn a_line_it_cannot_do_says_so() {
        assert_eq!(parsed("fly 3"), Err("no command `fly`".into()));
        assert_eq!(parsed(""), Err("nothing typed".into()));
        assert!(
            parsed("scale 0").is_err(),
            "a zero scale collapses the object"
        );
        assert!(parsed("rotate 90").is_err(), "no axis");
        assert!(parsed("rotate x").is_err(), "no angle");
        assert!(parsed("move sideways").is_err());
    }

    /// Modeling verbs check their point counts.
    #[test]
    fn modeling_commands_validate_arity_and_coordinates() {
        assert_eq!(
            parsed("point 1,2,3"),
            Ok("Model(Point([1.0, 2.0, 3.0]))".into())
        );
        assert_eq!(parsed("trim 0.2 0.8"), Ok("Model(Trim(0.2, 0.8))".into()));
        assert_eq!(parsed("explode"), Ok("Model(Explode)".into()));

        for line in [
            "point @1,2,3",
            "line 0,0,0",
            "trim 0 1 extra",
            "explode extra",
            "scale 2 extra",
            "delete extra",
        ] {
            assert!(parsed(line).is_err(), "{line}");
        }

        assert!(parsed(&"x".repeat(65537)).is_err());
    }

    /// Rotate carries its axis.
    #[test]
    fn rotation_names_its_axis() {
        assert_eq!(
            parsed("rotate z 45"),
            Ok("Rotate { axis: Z, degrees: 45.0 }".into())
        );
    }
}
