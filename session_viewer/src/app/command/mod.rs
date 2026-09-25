pub mod tool;
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

/// Every verb the command line knows.
pub const REGISTRY: &[&Spec] = &[
    &verbs::point::SPEC,      // register:point
    &verbs::line::SPEC,       // register:line
    &verbs::polyline::SPEC,   // register:polyline
    &verbs::curve::SPEC,      // register:curve
    &verbs::close::SPEC,      // register:close
    &verbs::trim::SPEC,       // register:trim
    &verbs::extend::SPEC,     // register:extend
    &verbs::explode::SPEC,    // register:explode
    &verbs::r#move::SPEC,     // register:move
    &verbs::rotate::SPEC,     // register:rotate
    &verbs::scale::SPEC,      // register:scale
    &verbs::copy::SPEC,       // register:copy
    &verbs::orient_3_points::SPEC, // register:orient_3_points
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
    &verbs::arctic::SPEC,     // register:arctic
    &verbs::outline::SPEC,    // register:outline
    &verbs::object::SPEC,     // register:object
    &verbs::edge::SPEC,       // register:edge
    &verbs::face::SPEC,       // register:face
    &verbs::controls::SPEC,   // register:controls
    &verbs::select_lasso::SPEC, // register:select_lasso
    &verbs::select_by_name::SPEC, // register:select_by_name
    &verbs::select_small::SPEC, // register:select_small
    &verbs::clipping_plane::SPEC, // register:clipping_plane
    &verbs::r#box::SPEC,      // register:box
    &verbs::sphere::SPEC, // register:sphere
    &verbs::cylinder::SPEC, // register:cylinder
    &verbs::cone::SPEC, // register:cone
    &verbs::pyramid::SPEC, // register:pyramid
    &verbs::torus::SPEC, // register:torus
    &verbs::block_with_hole::SPEC, // register:block_with_hole
    &verbs::tetrahedron::SPEC, // register:tetrahedron
    &verbs::octahedron::SPEC, // register:octahedron
    &verbs::dodecahedron::SPEC, // register:dodecahedron
    &verbs::icosahedron::SPEC, // register:icosahedron
    &verbs::quad_sphere::SPEC, // register:quad_sphere
    &verbs::capsule::SPEC, // register:capsule
    &verbs::nurbs_curve_circle::SPEC, // register:nurbs_curve_circle
    &verbs::nurbs_curve_ellipse::SPEC, // register:nurbs_curve_ellipse
    &verbs::nurbs_curve_arc::SPEC, // register:nurbs_curve_arc
    &verbs::nurbs_curve_parabola::SPEC, // register:nurbs_curve_parabola
    &verbs::loft::SPEC, // register:loft
    &verbs::extrude::SPEC, // register:extrude
    &verbs::nurbs_surface_loft::SPEC, // register:nurbs_surface_loft
    &verbs::nurbs_surface_network::SPEC, // register:nurbs_surface_network
    &verbs::nurbs_surface_revolve::SPEC, // register:nurbs_surface_revolve
    &verbs::nurbs_surface_4_points::SPEC, // register:nurbs_surface_4_points
    &verbs::nurbs_surface_sweep1::SPEC, // register:nurbs_surface_sweep1
    &verbs::nurbs_surface_sweep2::SPEC, // register:nurbs_surface_sweep2
    &verbs::text::SPEC, // register:text
    &verbs::project_to_plane::SPEC, // register:project_to_plane
    &verbs::measure_distance::SPEC, // register:measure_distance
    &verbs::length::SPEC, // register:length
    &verbs::area::SPEC, // register:area
    &verbs::volume::SPEC, // register:volume
    &verbs::add_group::SPEC, // register:add_group
    &verbs::add_edge::SPEC, // register:add_edge
];

/// A name lowercased without its spaces, so `clippingplane` spells `Clipping Plane`.
fn compact(name: &str) -> String {
    name.split_whitespace().collect::<String>().to_ascii_lowercase()
}

/// How many leading words spell `name`, ignoring case and the spaces inside it.
fn spells(name: &str, words: &[&str]) -> Option<usize> {
    let mut rest = compact(name);

    for (index, word) in words.iter().enumerate() {
        let word = word.to_ascii_lowercase();
        let tail = rest.strip_prefix(word.as_str())?.to_owned();

        if tail.is_empty() {
            return Some(index + 1);
        }

        rest = tail;
    }

    None
}

/// Match the longest command name, leaving its arguments untouched.
fn command_words(words: &[&str]) -> Option<(&'static Spec, usize)> {
    REGISTRY
        .iter()
        .flat_map(|spec| {
            spec.names.iter().chain(spec.aliases).filter_map(|name| {
                spells(name, words).map(|count| (*spec, count, compact(name).len()))
            })
        })
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(spec, count, _)| (spec, count))
}

/// The line with its command spelled as shown, e.g. `clippingplane xy` becomes `Clipping Plane XY`.
pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    let Some((spec, count)) = command_words(&words) else {
        return line.trim().to_owned();
    };
    let name = spec.names[0];
    let mut text = name.to_owned();
    let mut rest = &words[count..];
    // the longest option the next words spell takes its shown case
    let option = spec
        .options
        .iter()
        .filter_map(|option| {
            let tail: Vec<_> = option
                .split_whitespace()
                .skip(name.split_whitespace().count())
                .collect();
            (!tail.is_empty()
                && rest.len() >= tail.len()
                && tail.iter().zip(rest).all(|(a, b)| a.eq_ignore_ascii_case(b)))
            .then_some(tail)
        })
        .max_by_key(|tail| tail.len());

    if let Some(tail) = option {
        text.push(' ');
        text.push_str(&tail.join(" "));
        rest = &rest[tail.len()..];
    }

    for word in rest {
        text.push(' ');
        text.push_str(word);
    }

    text
}

/// True once the command name has been followed by a space or an option.
pub fn choosing_option(line: &str) -> bool {
    let words: Vec<_> = line.split_whitespace().collect();
    command_words(&words).is_some_and(|(_, count)| words.len() > count || line.ends_with(' '))
}

/// The option without its command name, for the inline buttons.
pub fn option_label(line: &str) -> &str {
    let words: Vec<_> = line.split_whitespace().collect();
    let count = command_words(&words).map_or(1, |(_, count)| count);
    line.splitn(count + 1, ' ').last().unwrap_or(line)
}

/// The shown name of the line's command, e.g. `Move` for `m 10 0 0`.
pub fn name_of(line: &str) -> &'static str {
    command_words(&line.split_whitespace().collect::<Vec<_>>()).map_or("", |(spec, _)| spec.names[0])
}

/// Help text for the verb being typed.
pub fn hint(line: &str) -> &'static str {
    match command_words(&line.split_whitespace().collect::<Vec<_>>()) {
        Some((spec, _)) if !spec.hint.is_empty() => spec.hint,
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

    let words: Vec<_> = line.split_whitespace().collect();
    let Some((spec, count)) = command_words(&words) else {
        return Err(format!("no command `{}`", words[0].to_ascii_lowercase()));
    };
    let verb = spec.names[0];
    let rest = &words[count..];

    if spec.arity.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }

    (spec.parse)(verb, rest)
}

/// Clickable choices for the verb being typed.
pub fn options(line: &str) -> &'static [&'static str] {
    command_words(&line.split_whitespace().collect::<Vec<_>>())
        .map_or(&[], |(spec, _)| spec.options)
}

/// Commands or options starting with the typed text.
pub fn completions(line: &str) -> Vec<&'static str> {
    // after a space, complete the option instead
    if choosing_option(line) {
        let mut typed = canonical(line).to_ascii_lowercase();

        if line.ends_with(' ') {
            typed.push(' ');
        }

        return options(line)
            .iter()
            .copied()
            .filter(|name| name.to_ascii_lowercase().starts_with(&typed))
            .collect();
    }

    let typed = compact(line);
    let mut names: Vec<&'static str> = REGISTRY
        .iter()
        .flat_map(|spec| spec.names)
        .copied()
        .collect();
    names.sort_by_key(|name| name.to_ascii_lowercase());
    names.retain(|name| compact(name).starts_with(&typed));

    // a whole alias puts its command first, so `m` stays Move beside Measure Distance
    if let Some((spec, _)) = command_words(&line.split_whitespace().collect::<Vec<_>>())
        && let Some(index) = names.iter().position(|name| *name == spec.names[0])
    {
        let name = names.remove(index);
        names.insert(0, name);
    }

    names
}

/// Every command, matching ones first.
pub fn browse(line: &str) -> Vec<&'static str> {
    if choosing_option(line) {
        return options(line).to_vec();
    }

    let typed = compact(line);
    let (matching, rest): (Vec<_>, Vec<_>) = completions("")
        .into_iter()
        .partition(|name| compact(name).starts_with(&typed));
    matching.into_iter().chain(rest).collect()
}

/// Take the first completion; false when arguments are still needed.
pub fn accept(line: &str) -> (String, bool) {
    let line = line.trim_start(); // a stray leading space must not hide the verb

    if line.is_empty() {
        return (String::new(), true);
    }

    let choices = completions(line);
    let text = choices.first().copied().unwrap_or(line).trim();
    let words: Vec<_> = text.split_whitespace().collect();
    let typed = command_words(&words);
    let bare_verb_waits =
        typed.is_some_and(|(spec, count)| words.len() == count && spec.wait_for_option);
    let option_waits = typed.is_some_and(|(spec, count)| {
        words.len() == count + 1
            && spec.wait_after_option
            && spec
                .options
                .iter()
                .any(|option| option.eq_ignore_ascii_case(text))
    });

    if bare_verb_waits || option_waits {
        (format!("{text} "), false)
    } else {
        (text.to_owned(), true)
    }
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
        return Err(format!("`{joined}` is not an offset; try `Move 10 0 0`"));
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

    let verb = verb.to_ascii_lowercase();

    match verb.as_str() {
        "trim" | "extend" if words.len() == 2 => {
            let a = number(words.first().copied(), "Trim 0.2 0.8")?;
            let b = number(words.get(1).copied(), "Trim 0.2 0.8")?;
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

            match (verb.as_str(), points.len()) {
                ("point", 1) => Ok(Modeling::Point(points[0])),
                ("line", 2) => Ok(Modeling::Line(points[0], points[1])),
                ("polyline", 2..) => Ok(Modeling::Polyline(points)),
                ("curve", 2..) => Ok(Modeling::Curve(points)),
                _ => {
                    Err("Point needs one coordinate, Line two, Polyline and Curve at least two".into())
                }
            }
        }
        _ => Err("try Trim 0.2 0.8 or Extend -0.2 1.2".into()),
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
        assert_eq!(completions("Element F"), vec!["Element Features"]);
        assert_eq!(accept("Element F"), ("Element Features ".into(), false));
        assert_eq!(
            accept("Element Features "),
            ("Element Features On".into(), true)
        );
        assert_eq!(
            accept("Element Features of"),
            ("Element Features Off".into(), true)
        );
        assert_eq!(
            browse("Element Features Off"),
            vec!["Element Features On", "Element Features Off"]
        );
        assert_eq!(option_label("Element Features Off"), "Off");
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

    /// One Enter on a partial name runs the verb, or opens its options when it needs one.
    #[test]
    fn one_enter_runs_a_partial_verb_or_opens_its_options() {
        assert_eq!(accept("Del"), ("Delete".into(), true));
        assert_eq!(accept("clo"), ("Close".into(), true));
        assert_eq!(accept(" clo"), ("Close".into(), true));
        assert_eq!(accept("cur"), ("Curve".into(), true));
        assert_eq!(accept("Sn"), ("Snap ".into(), false));
        assert_eq!(accept("Snap ne"), ("Snap Near".into(), true));
        assert_eq!(accept("Snap pe"), ("Snap Perp".into(), true));
    }

    /// Snap turns snapping on or off, or toggles one kind.
    #[test]
    fn snap_takes_a_switch_or_a_kind() {
        assert_eq!(
            parsed("Snap Off"),
            Ok("Snap { on: Some(false), mode: 0 }".into())
        );
        assert_eq!(parsed("snap"), Ok("Snap { on: None, mode: 0 }".into()));
        assert_eq!(parsed("Snap near"), Ok("Snap { on: None, mode: 2 }".into()));
        assert_eq!(
            parsed("Snap Center"),
            Ok("Snap { on: None, mode: 8 }".into())
        );
        assert!(parsed("Snap sideways").is_err());
        assert_eq!(parsed("Close"), Ok("Close".into()));
        assert!(parsed("Close 3").is_err());
    }

    /// Completion and parsing ignore case.
    #[test]
    fn discovery_and_layer_options_are_case_insensitive() {
        assert_eq!(completions("la"), vec!["Layers"]);
        assert_eq!(completions("Layers "), vec!["Layers On", "Layers Off"]);
        assert!(completions("").contains(&"Controls"));
        assert_eq!(parsed("Layers OFF"), Ok("Layers(Some(false))".into()));
        assert_eq!(
            parsed("Element Features off"),
            Ok("ElementFeatures(Some(false))".into())
        );
        assert_eq!(
            parsed("Element Features"),
            Ok("ElementFeatures(None)".into())
        );
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
                "Add Edge",
                "Add Group",
                "Arctic",
                "Area",
                "Block With Hole",
                "Box",
                "Capsule",
                "Clipping Plane",
                "Close",
                "Cone",
                "Controls",
                "Copy",
                "Curve",
                "Cylinder",
                "Delete",
                "Dodecahedron",
                "Edge",
                "Element Features",
                "Escape",
                "Explode",
                "Extend",
                "Extrude",
                "Face",
                "Fit",
                "Hide",
                "Icosahedron",
                "Layers",
                "Length",
                "Line",
                "Loft",
                "Measure Distance",
                "Move",
                "Nurbs Curve Arc",
                "Nurbs Curve Circle",
                "Nurbs Curve Ellipse",
                "Nurbs Curve Parabola",
                "Nurbs Surface 4 Points",
                "Nurbs Surface Loft",
                "Nurbs Surface Network",
                "Nurbs Surface Revolve",
                "Nurbs Surface Sweep1",
                "Nurbs Surface Sweep2",
                "Object",
                "Octahedron",
                "Opacity",
                "Open",
                "Orient 3 Points",
                "Outline",
                "Point",
                "Polyline",
                "Project To Plane",
                "Pyramid",
                "Quad Sphere",
                "Redo",
                "Rotate",
                "Save",
                "Scale",
                "Select By Name",
                "Select Lasso",
                "Select Small",
                "Show",
                "Snap",
                "Sphere",
                "Split",
                "Tetrahedron",
                "Text",
                "Torus",
                "Trim",
                "Undo",
                "Volume",
            ]
        );
    }

    /// Every shown name is Title Case words: no underscores, each word capitalised or a number.
    #[test]
    fn every_name_and_option_is_title_case_words() {
        for spec in REGISTRY {
            for name in spec.names.iter().chain(spec.options) {
                assert!(!name.contains('_'), "{name}");
                assert!(
                    name.split_whitespace()
                        .next()
                        .is_some_and(|word| word.starts_with(|c: char| c.is_ascii_uppercase())),
                    "{name}"
                );
            }

            for name in spec.names {
                assert!(
                    name.split_whitespace().all(|word| {
                        word.starts_with(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit())
                    }),
                    "{name}"
                );
            }
        }
    }

    /// A several-word name parses with or without its spaces, in any case.
    #[test]
    fn several_word_names_ignore_case_and_spaces() {
        for line in [
            "Clipping Plane",
            "clipping plane",
            "ClippingPlane",
            "clippingplane",
            "CLIPPING PLANE",
            "  clipping   plane ",
        ] {
            assert_eq!(parsed(line), Ok("Pick(Normal)".into()), "{line}");
        }

        assert_eq!(parsed("clippingplane off"), Ok("Switch(false)".into()));
        assert_eq!(parsed("Clipping Plane Fill Solid"), Ok("Fill(Some(true))".into()));
        assert_eq!(parsed("elementfeatures on"), Ok("ElementFeatures(Some(true))".into()));
        assert_eq!(parsed("poly line"), parsed("Polyline"));
        assert_eq!(parsed("clipping"), Err("no command `clipping`".into()));
        assert_eq!(parsed("clipping_plane"), Err("no command `clipping_plane`".into()));
        assert!(
            parsed("Clipping Plane sideways").is_err(),
            "the words after the name are its options"
        );
    }

    /// The shown spelling replaces what was typed, for history lines.
    #[test]
    fn canonical_spells_the_command_as_shown() {
        assert_eq!(canonical("clippingplane xy 0,0,1"), "Clipping Plane XY 0,0,1");
        assert_eq!(canonical("clipping plane fill hatch"), "Clipping Plane Fill Hatch");
        assert_eq!(canonical("  snap   near "), "Snap Near");
        assert_eq!(canonical("m 10 0 0"), "Move 10 0 0");
        assert_eq!(canonical("0,0,0"), "0,0,0");
        assert_eq!(canonical(""), "");
    }

    /// Completion and one-Enter accept work on several-word names typed any way.
    #[test]
    fn several_word_names_complete() {
        assert_eq!(completions("clip"), vec!["Clipping Plane"]);
        assert_eq!(completions("clipping p"), vec!["Clipping Plane"]);
        assert_eq!(completions("ClippingP"), vec!["Clipping Plane"]);
        assert_eq!(completions("elementf"), vec!["Element Features"]);
        assert_eq!(accept("clip"), ("Clipping Plane".into(), true));
        assert_eq!(accept("clippingplane"), ("Clipping Plane".into(), true));
        assert_eq!(accept("clippingplane x"), ("Clipping Plane XY".into(), true));
        assert_eq!(accept("clipping plane fi"), ("Clipping Plane Fill".into(), true));
        assert_eq!(
            completions("clippingplane fill "),
            vec!["Clipping Plane Fill Hatch", "Clipping Plane Fill Solid"]
        );
        assert_eq!(accept("elementf"), ("Element Features ".into(), false));
        assert_eq!(browse("clip")[0], "Clipping Plane");
        assert_eq!(browse("clippingplane o"), options("Clipping Plane"));
        assert_eq!(hint("clippingplane"), hint("Clipping Plane"));
        assert_eq!(option_label("Clipping Plane Fill Hatch"), "Fill Hatch");
        assert!(choosing_option("clippingplane "));
        assert!(!choosing_option("clipping"));
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
        assert_eq!(parsed("explode"), Ok("Explode".into()));

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

    /// A bare transform verb starts picking points; typed values still act at once.
    #[test]
    fn bare_transform_verbs_start_picking() {
        for (line, tool) in [
            ("move", "Moving"),
            ("rotate", "Rotating"),
            ("scale", "Scaling { mode: 3 }"),
            ("copy", "Copying"),
            ("Orient 3 Points", "Orienting"),
            ("orient3points", "Orienting"),
            ("orient3pt", "Orienting"),
        ] {
            assert_eq!(parsed(line), Ok(tool.into()), "{line}");
        }

        assert_eq!(parsed("move 10 0 0"), Ok("Move([10.0, 0.0, 0.0])".into()));
        assert_eq!(parsed("m @0 5"), Ok("Move([0.0, 5.0, 0.0])".into()));
        assert_eq!(parsed("rotate z 45"), Ok("Rotate { axis: Z, degrees: 45.0 }".into()));
        assert_eq!(parsed("scale 2"), Ok("Scale(2.0)".into()));
        assert_eq!(parsed("copy 0,5,0"), Ok("Copy([0.0, 5.0, 0.0])".into()));
        assert!(parsed("orient3pt 0,0,0 1,0,0 0,1,0 5,5,0 5,6,0 4,5,0").is_ok());

        for line in ["scale 0", "rotate 90", "rotate x", "scale 2 extra", "rotate z 45 extra", "orient3pt 0,0,0"] {
            assert!(parsed(line).is_err(), "{line}");
        }

        assert_eq!(accept("Rot"), ("Rotate".into(), true));
        assert_eq!(accept("Rotate "), ("Rotate x ".into(), false));
        assert_eq!(accept("ori"), ("Orient 3 Points".into(), true));
        assert_eq!(completions("Co"), vec!["Cone", "Controls", "Copy"]);
        assert_eq!(name_of("m 10 0 0"), "Move");
    }

    /// Creation verbs take a leading option, then typed answers; anything else is refused.
    #[test]
    fn creation_verbs_parse_options_and_answers() {
        assert_eq!(
            parsed("Box Mesh 0,0,0 100 50 30"),
            Ok(r#"Start { shape: Box, option: 1, words: ["0,0,0", "100", "50", "30"] }"#.into())
        );
        assert_eq!(
            parsed("nurbscurvearc 3 points 0,0,0 10,0,0 5,5,0"),
            Ok(r#"Start { shape: Nurbs Curve Arc, option: 1, words: ["0,0,0", "10,0,0", "5,5,0"] }"#.into())
        );
        assert_eq!(parsed("sphere"), Ok("Start { shape: Sphere, option: 0, words: [] }".into()));
        assert!(parsed("Quad Sphere Brep").is_err());
        assert!(parsed("Sphere 0,0,0 wide").is_err());
        assert_eq!(accept("Bo"), ("Box".into(), true));
        assert_eq!(accept("Box "), ("Box Brep".into(), true));
        assert_eq!(accept("quad"), ("Quad Sphere".into(), true));
        assert_eq!(canonical("blockwithhole mesh 0,0,0"), "Block With Hole Mesh 0,0,0");
        assert_eq!(canonical("nurbscurvearc 3 points"), "Nurbs Curve Arc 3 Points");
    }

    /// Surface verbs take options and values typed after them; the old snake names still work.
    #[test]
    fn the_surface_verbs_parse_their_typed_forms() {
        for line in [
            "Loft",
            "Loft Closed",
            "Loft Open Closed",
            "Nurbs Surface Loft Straight",
            "nurbssurface_loft straight",
            "nurbsnurbs_network",
            "nurbssurfacenetwork",
            "Nurbs Surface 4 Points 0,0,0 10,0,0 10,10,3 0,10,0",
            "Nurbs Surface 4 Points",
            "Nurbs Surface Revolve 0,0,0 0,0,1 90",
            "Nurbs Surface Revolve",
            "Nurbs Surface Sweep1",
            "Nurbs Surface Sweep2",
            "Extrude 10",
            "Extrude 0,0,5",
            "Extrude Cap Off 10",
            "extrude cap on",
        ] {
            assert!(parsed(line).is_ok(), "{line}");
        }

        for line in [
            "Nurbs Surface 4 Points 0,0,0 1,0,0 1,1,0",
            "Nurbs Surface Revolve 0,0,0 0,0,1 400",
            "Extrude sideways",
            "Loft 5",
        ] {
            assert!(parsed(line).is_err(), "{line}");
        }

        assert_eq!(name_of("nurbsnurbs_network"), "Nurbs Surface Network");
        assert_eq!(canonical("nurbssurface_revolve 0,0,0 0,0,1"), "Nurbs Surface Revolve 0,0,0 0,0,1");
        assert_eq!(
            completions("nurbs s"),
            vec![
                "Nurbs Surface 4 Points",
                "Nurbs Surface Loft",
                "Nurbs Surface Network",
                "Nurbs Surface Revolve",
                "Nurbs Surface Sweep1",
                "Nurbs Surface Sweep2",
            ]
        );
        assert_eq!(completions("Extrude cap "), vec!["Extrude Cap On", "Extrude Cap Off"]);
    }

    /// Text waits for its words and keeps them, spaced once.
    #[test]
    fn text_waits_for_its_words_and_keeps_them() {
        assert_eq!(accept("Tex"), ("Text ".into(), false));
        assert_eq!(accept("Text "), ("Text ".into(), false));
        assert_eq!(parsed("text Hello   world"), Ok("Text(\"Hello world\")".into()));
        assert_eq!(canonical("text Hello   World"), "Text Hello World");
        assert!(parsed("text").is_err());
        assert!(parsed(&format!("text {}", "x".repeat(81))).is_err());
        assert!(!hint("text x").is_empty());
    }

    /// Project To Plane offers its five planes, CPlane first.
    #[test]
    fn project_to_plane_offers_five_planes() {
        assert_eq!(
            completions("Project To Plane "),
            vec![
                "Project To Plane CPlane",
                "Project To Plane XY",
                "Project To Plane YZ",
                "Project To Plane ZX",
                "Project To Plane 3Point",
            ]
        );
        assert_eq!(accept("projectt"), ("Project To Plane ".into(), false));
        assert_eq!(accept("Project To Plane "), ("Project To Plane CPlane".into(), true));
        assert_eq!(canonical("projecttoplane xy"), "Project To Plane XY");
        assert!(parsed("project to plane xz").unwrap().contains("ZX"));
        assert!(parsed("Project To Plane 3Point 0,0,0 1,0,0 0,1,0").is_ok());

        for line in [
            "Project To Plane 3Point 0,0,0 1,0,0",
            "Project To Plane sideways",
            "Project To Plane XY extra",
        ] {
            assert!(parsed(line).is_err(), "{line}");
        }
    }

    /// Add Group takes an optional name; Add Edge takes nothing.
    #[test]
    fn session_structure_verbs_parse() {
        assert_eq!(parsed("Add Group"), Ok("AddGroup(\"\")".into()));
        assert_eq!(parsed("addgroup North wall"), Ok("AddGroup(\"North wall\")".into()));
        assert!(parsed("Add Group a/b").is_err());
        assert_eq!(parsed("add edge"), Ok("AddEdge".into()));
        assert!(parsed("Add Edge x").is_err());
        assert_eq!(accept("addg"), ("Add Group".into(), true));
        assert_eq!(completions("add"), vec!["Add Edge", "Add Group"]);
        assert_eq!(canonical("addedge"), "Add Edge");
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
