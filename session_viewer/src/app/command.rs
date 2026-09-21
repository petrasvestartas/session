use crate::app::coords;
use crate::app::gizmo::Axis;

/// What a line asked for.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Model(crate::app::modeling::Modeling),
    Move([f64; 3]), // Move the selection by a world offset, in millimetres.
    Rotate {
        // Turn the selection about one axis through its own centre, in degrees.
        axis: Axis,
        degrees: f64,
    },
    Scale(f64), // Scale the selection about its own centre.
    Split,
    Save,
    Open,
    Delete,
    Undo,
    Redo,
    Hide,
    ShowAll,
    Fit,
    Layers(Option<bool>),
    Attributes(Option<bool>), // Show or hide every `attributes` group: the outlines, axes, sections and centroids beside the elements.
    Selection(crate::app::selection::SelectionTool),
    Controls,
    Ssao(Option<bool>),
    Snap(Option<bool>),
    Escape, // Clear the selection.
}

/// Contextual syntax shown while typing, including immediately usable examples.
pub fn hint(line: &str) -> &'static str {
    match line
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "point" => "Point · Enter then click or type x,y,z",
        "line" => "Line · Enter then click or type endpoints · Example: Line 0,0,0 100,0,0",
        "curve" => "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
        "polyline" => "Polyline · click points, or choose Rectangle / Polygon · Enter finishes",
        "move" | "m" => "Select an object, then Move dx,dy,dz · Example: Move 10,0,0",
        "rotate" | "rot" => "Select an object, then Rotate axis degrees · Example: Rotate z 45",
        "scale" | "s" => "Select an object, then Scale factor · Example: Scale 2",
        "trim" => "Select a line or curve · Trim 0.2 0.8 keeps that part of its length/domain",
        "extend" => "Select a line or curve · Extend -0.2 1.2 extends its domain at both ends",
        "explode" => "Select a polyline · Explode creates its individual line segments",
        "split" => {
            "Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels"
        }
        "save" => "Save downloads the complete editable scene as a .session file",
        "open" => "Open restores a saved .session file",
        "fit" => "Fit zooms to the selection, or the whole scene when nothing is selected",
        "layers" => "Layers (On Off): show or hide the layer panel",
        "attributes" => "Attributes (On Off): draw or remove the element features, moving with their element",
        "snap" => "Snap (On Off): endpoints, vertices and midpoints within 12 pixels",
        "ssao" | "arctic" => {
            "SSAO (On Off): soft contact shading and studio lighting · G toggles in the viewport"
        }
        _ => "Type a command · Up/Down browse · Tab completes · Enter executes · Esc cancels",
    }
}

/// Parse one line. `Err` carries what to show the person who typed it.
pub fn parse(line: &str) -> Result<Command, String> {
    if line.len() > 65536 {
        return Err("command exceeds 64 KiB".into());
    }

    let line = line.trim();

    if line.is_empty() {
        return Err("nothing typed".into());
    }

    let mut words = line.split_whitespace();
    let verb = words.next().unwrap_or_default().to_ascii_lowercase();
    let rest: Vec<&str> = words.collect();
    let expected = match verb.as_str() {
        "scale" | "s" => Some(1),
        "rotate" | "rot" => Some(2),
        "split" | "save" | "open" | "delete" | "del" | "undo" | "redo" | "hide" | "show"
        | "fit" | "escape" | "esc" => Some(0),
        _ => None,
    };

    if expected.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }

    match verb.as_str() {
        "layers" => match rest.as_slice() {
            [] => Ok(Command::Layers(None)),
            [value] if value.eq_ignore_ascii_case("on") => Ok(Command::Layers(Some(true))),
            [value] if value.eq_ignore_ascii_case("off") => Ok(Command::Layers(Some(false))),
            _ => Err("Layers (On Off)".into()),
        },
        "attributes" => match rest.as_slice() {
            [] => Ok(Command::Attributes(None)),
            [value] if value.eq_ignore_ascii_case("on") => Ok(Command::Attributes(Some(true))),
            [value] if value.eq_ignore_ascii_case("off") => Ok(Command::Attributes(Some(false))),
            _ => Err("Attributes (On Off)".into()),
        },
        "snap" => match rest.as_slice() {
            [] => Ok(Command::Snap(None)),
            [value] if value.eq_ignore_ascii_case("on") => Ok(Command::Snap(Some(true))),
            [value] if value.eq_ignore_ascii_case("off") => Ok(Command::Snap(Some(false))),
            _ => Err("Snap (On Off)".into()),
        },
        "ssao" | "arctic" => match rest.as_slice() {
            [] => Ok(Command::Ssao(None)),
            [value] if value.eq_ignore_ascii_case("on") => Ok(Command::Ssao(Some(true))),
            [value] if value.eq_ignore_ascii_case("off") => Ok(Command::Ssao(Some(false))),
            _ => Err("SSAO (On Off)".into()),
        },
        "object" | "edge" | "face" | "controls" if rest.is_empty() => {
            use crate::app::selection::SelectionTool;
            Ok(match verb.as_str() {
                "object" => Command::Selection(SelectionTool::Object),
                "edge" => Command::Selection(SelectionTool::Edge),
                "face" => Command::Selection(SelectionTool::Face),
                _ => Command::Controls,
            })
        }
        "point" | "line" | "polyline" | "curve" | "trim" | "extend" | "explode" => {
            model(&verb, &rest).map(Command::Model)
        }
        "move" | "m" => offset(&rest).map(Command::Move),
        "rotate" | "rot" => {
            let (axis, degrees) = axis_and_number(&rest, "rotate x 90")?;
            Ok(Command::Rotate { axis, degrees })
        }
        "scale" | "s" => {
            let k = number(rest.first().copied(), "scale 2")?;

            if k <= 0.0 {
                return Err("scale wants a factor above zero".into());
            }

            Ok(Command::Scale(k))
        }
        "split" => Ok(Command::Split),
        "save" => Ok(Command::Save),
        "open" => Ok(Command::Open),
        "delete" | "del" => Ok(Command::Delete),
        "undo" => Ok(Command::Undo),
        "redo" => Ok(Command::Redo),
        "hide" => Ok(Command::Hide),
        "show" => Ok(Command::ShowAll),
        "fit" => Ok(Command::Fit),
        "escape" | "esc" => Ok(Command::Escape),
        other => Err(format!("no command `{other}`")),
    }
}

/// Finite choices appear as clickable words beside the prompt.
pub fn options(line: &str) -> &'static [&'static str] {
    match line
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "polyline" => &["Polyline Points", "Polyline Rectangle", "Polyline Polygon"],
        "layers" => &["Layers On", "Layers Off"],
        "attributes" => &["Attributes On", "Attributes Off"],
        "ssao" => &["SSAO On", "SSAO Off"],
        "arctic" => &["Arctic On", "Arctic Off"],
        "snap" => &["Snap On", "Snap Off"],
        "rotate" | "rot" => &["Rotate x", "Rotate y", "Rotate z"],
        _ => &[],
    }
}

/// Discover commands before typing; options use the same scrollable completion list.
pub fn completions(line: &str) -> Vec<&'static str> {
    const COMMANDS: &[&str] = &[
        "Arctic", "Attributes", "Controls", "Curve", "Delete", "Edge", "Escape", "Explode", "Extend", "Face",
        "Fit", "Hide", "Layers", "Line", "Move", "Object", "Open", "Point", "Polyline", "Redo",
        "Rotate", "Save", "Scale", "Show", "Snap", "Split", "SSAO", "Trim", "Undo",
    ];
    let lower = line.to_ascii_lowercase();
    let choices = if lower.contains(' ') {
        options(line)
    } else {
        COMMANDS
    };
    choices
        .iter()
        .copied()
        .filter(|name| name.to_ascii_lowercase().starts_with(&lower))
        .collect()
}

/// Browsing keeps every command reachable; matching prefixes lead the list.
/// Options belong to their command and never include unrelated verbs.
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

/// Accept the first matching completion. False means keep a prompt for its arguments.
pub fn accept(line: &str) -> (String, bool) {
    if line.trim().is_empty() {
        return (String::new(), true);
    }
    let choices = completions(line);
    let text = choices.first().copied().unwrap_or(line).trim();
    let words: Vec<_> = text.split_whitespace().collect();
    if (!options(text).is_empty() && words.len() == 1 && !text.eq_ignore_ascii_case("polyline"))
        || (words.len() == 2
            && words[0].eq_ignore_ascii_case("rotate")
            && ["x", "y", "z"]
                .iter()
                .any(|axis| words[1].eq_ignore_ascii_case(axis)))
    {
        (format!("{text} "), false)
    } else {
        (text.to_owned(), true)
    }
}

/// `move` borrows the coordinate parser's SYNTAX - `10,0,0`, `@10,0` and `10<45` all parse -
/// but not all of its meanings: a typed move is always an OFFSET, so absolute and relative
/// collapse to the same thing here, and a bare distance is along +x rather than along a
/// direction the caller established. Polar is in the world XY plane, not the construction one.
///
/// A command line is typed with spaces, and the coordinate syntax separates with commas, so
/// spaces between the numbers become commas first. `@10 0` and `@10,0` are then the same line,
/// which is what a person typing at speed expects.
fn offset(words: &[&str]) -> Result<[f64; 3], String> {
    let joined = words.join(" ");
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
            let r = degrees.to_radians();
            Ok([distance * r.cos(), distance * r.sin(), 0.0])
        }
        coords::Typed::Distance(d) => Ok([d, 0.0, 0.0]),
    }
}

fn axis_and_number(words: &[&str], example: &str) -> Result<(Axis, f64), String> {
    let axis = match words.first().map(|w| w.to_ascii_lowercase()) {
        Some(a) if a == "x" => Axis::X,
        Some(a) if a == "y" => Axis::Y,
        Some(a) if a == "z" => Axis::Z,
        _ => return Err(format!("which axis? try `{example}`")),
    };
    Ok((axis, number(words.get(1).copied(), example)?))
}

fn number(word: Option<&str>, example: &str) -> Result<f64, String> {
    let Some(word) = word else {
        return Err(format!("missing a number; try `{example}`"));
    };

    match word.parse::<f64>() {
        Ok(v) if v.is_finite() => Ok(v),
        _ => Err(format!("`{word}` is not a number")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn discovery_and_layer_options_are_case_insensitive() {
        assert_eq!(completions("la"), vec!["Layers"]);
        assert_eq!(completions("Layers "), vec!["Layers On", "Layers Off"]);
        assert!(completions("").contains(&"Controls"));
        assert_eq!(parse("Layers OFF"), Ok(Command::Layers(Some(false))));
        assert_eq!(parse("Attributes off"), Ok(Command::Attributes(Some(false))));
        assert_eq!(parse("Attributes"), Ok(Command::Attributes(None)));
        assert!(parse("Layers maybe").is_err());
    }

    #[test]
    fn the_verbs_and_their_short_forms() {
        assert_eq!(parse("delete"), Ok(Command::Delete));
        assert_eq!(parse("del"), Ok(Command::Delete));
        assert_eq!(parse("  UNDO "), Ok(Command::Undo));
        assert_eq!(parse("fit"), Ok(Command::Fit));
    }

    #[test]
    fn move_reads_the_same_coordinates_as_the_rest_of_the_viewer() {
        assert_eq!(parse("move 10 0 0"), Ok(Command::Move([10.0, 0.0, 0.0])));
        assert_eq!(parse("m @0 5"), Ok(Command::Move([0.0, 5.0, 0.0])));
        let Ok(Command::Move(polar)) = parse("move 10<90") else {
            panic!("a polar offset is an offset");
        };
        assert!((polar[1] - 10.0).abs() < 1e-9, "90 degrees is +y");
    }

    /// A refusal names what was wrong, and a verb the viewer does not have is one of them: a
    /// command line that swallows a typo teaches the typist nothing.
    #[test]
    fn a_line_it_cannot_do_says_so() {
        assert_eq!(parse("fly 3"), Err("no command `fly`".into()));
        assert_eq!(parse(""), Err("nothing typed".into()));
        assert!(
            parse("scale 0").is_err(),
            "a zero scale collapses the object"
        );
        assert!(parse("rotate 90").is_err(), "no axis");
        assert!(parse("rotate x").is_err(), "no angle");
        assert!(parse("move sideways").is_err());
    }

    #[test]
    fn modeling_commands_validate_arity_and_coordinates() {
        use crate::app::modeling::Modeling;
        assert_eq!(
            parse("point 1,2,3"),
            Ok(Command::Model(Modeling::Point([1.0, 2.0, 3.0])))
        );
        assert_eq!(
            parse("trim 0.2 0.8"),
            Ok(Command::Model(Modeling::Trim(0.2, 0.8)))
        );
        assert_eq!(parse("explode"), Ok(Command::Model(Modeling::Explode)));

        for line in [
            "point @1,2,3",
            "line 0,0,0",
            "trim 0 1 extra",
            "explode extra",
            "scale 2 extra",
            "delete extra",
        ] {
            assert!(parse(line).is_err(), "{line}");
        }

        assert!(parse(&"x".repeat(65537)).is_err());
    }

    #[test]
    fn rotation_names_its_axis() {
        assert_eq!(
            parse("rotate z 45"),
            Ok(Command::Rotate {
                axis: Axis::Z,
                degrees: 45.0
            })
        );
    }
}

fn model(verb: &str, words: &[&str]) -> Result<crate::app::modeling::Modeling, String> {
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
