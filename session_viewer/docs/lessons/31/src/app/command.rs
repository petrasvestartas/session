use crate::app::coords;
use crate::app::gizmo::Axis;

/// One parsed command line.
#[derive(Clone, Debug, PartialEq)]
pub enum Command {
    Model(crate::app::modeling::Modeling), // create or edit geometry
    Move([f64; 3]),                        // move the selection by mm
    Rotate {
        axis: Axis,   // world axis
        degrees: f64, // turn about the selection centre
    },
    Scale(f64), // scale about the selection centre
    // --8<-- [start:step-1a]
    Split,      // cut a curve or face
    // --8<-- [end:step-1a]
    Save,       // download the scene
    Open,       // load a scene file
    Delete,
    Undo,
    Redo,
    Hide,                     // hide the selection
    ShowAll,                  // show everything hidden
    Fit,                      // zoom to selection or scene
    Escape,                   // clear the selection
}

/// Help text for the verb being typed.
pub fn hint(line: &str) -> &'static str {
    match line
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase()
        .as_str()
    {
        "point" => "Point x,y,z · Example: Point 0,0,0 · Enter creates the point",
        "line" => "Line start end · Example: Line 0,0,0 100,0,0",
        // --8<-- [start:step-1b]
        "curve" => "Curve control points… · Example: Curve 0,0,0 50,100,0 100,0,0",
        // --8<-- [end:step-1b]
        "polyline" => "Polyline points… · Example: Polyline 0,0,0 100,0,0 100,100,0",
        "move" | "m" => "Select an object, then Move dx,dy,dz · Example: Move 10,0,0",
        "rotate" | "rot" => "Select an object, then Rotate axis degrees · Example: Rotate z 45",
        "scale" | "s" => "Select an object, then Scale factor · Example: Scale 2",
        "trim" => "Select a line or curve · Trim 0.2 0.8 keeps that part of its length/domain",
        "extend" => "Select a line or curve · Extend -0.2 1.2 extends its domain at both ends",
        "explode" => "Select a polyline · Explode creates its individual line segments",
        // --8<-- [start:step-1c]
        "split" => {
            "Select a curve or face · Split · choose cutter curves · Enter confirms · Esc cancels"
        }
        // --8<-- [end:step-1c]
        "save" => "Save downloads the complete editable scene as a .session file",
        "open" => "Open restores a saved .session file",
        "fit" => "Fit zooms to the selection, or the whole scene when nothing is selected",
        _ => "Try Point 0,0,0 · Line 0,0,0 100,0,0 · Fit · Undo · Save · Enter or Run executes",
    }
}

/// Parse one line; the error is the message to show.
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
    let rest: Vec<&str> = words.collect(); // the arguments
    // verbs with a fixed argument count
    let expected = match verb.as_str() {
        "scale" | "s" => Some(1),
        "rotate" | "rot" => Some(2),
        // --8<-- [start:step-1d]
        "split" | "save" | "open" | "delete" | "del" | "undo" | "redo" | "hide" | "show"
        | "fit" | "escape" | "esc" => Some(0),
        // --8<-- [end:step-1d]
        _ => None,
    };

    if expected.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }

    match verb.as_str() {
        // --8<-- [start:step-1e]
        "point" | "line" | "polyline" | "curve" | "trim" | "extend" | "explode" => {
        // --8<-- [end:step-1e]
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
        // --8<-- [start:step-1f]
        "split" => Ok(Command::Split),
        // --8<-- [end:step-1f]
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

/// A move offset from `10 0 0`, `@10,0` or `10<45`.
fn offset(words: &[&str]) -> Result<[f64; 3], String> {
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
fn axis_and_number(words: &[&str], example: &str) -> Result<(Axis, f64), String> {
    let axis = match words.first().map(|w| w.to_ascii_lowercase()) {
        Some(a) if a == "x" => Axis::X,
        Some(a) if a == "y" => Axis::Y,
        Some(a) if a == "z" => Axis::Z,
        _ => return Err(format!("which axis? try `{example}`")),
    };
    Ok((axis, number(words.get(1).copied(), example)?))
}

/// A finite number, or a message with the example.
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

    /// Short forms parse like the full verb.
    #[test]
    fn the_verbs_and_their_short_forms() {
        assert_eq!(parse("delete"), Ok(Command::Delete));
        assert_eq!(parse("del"), Ok(Command::Delete));
        assert_eq!(parse("  UNDO "), Ok(Command::Undo));
        assert_eq!(parse("fit"), Ok(Command::Fit));
    }

    /// Move accepts every coordinate form.
    #[test]
    fn move_reads_the_same_coordinates_as_the_rest_of_the_viewer() {
        assert_eq!(parse("move 10 0 0"), Ok(Command::Move([10.0, 0.0, 0.0])));
        assert_eq!(parse("m @0 5"), Ok(Command::Move([0.0, 5.0, 0.0])));
        let Ok(Command::Move(polar)) = parse("move 10<90") else {
            panic!("a polar offset is an offset");
        };
        assert!((polar[1] - 10.0).abs() < 1e-9, "90 degrees is +y");
    }

    /// A bad line gets a message naming the problem.
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

    /// Modeling verbs check their point counts.
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

    /// Rotate carries its axis.
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

/// A modeling command from its verb and points.
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
        // --8<-- [start:step-1g]
        "point" | "line" | "polyline" | "curve" => {
        // --8<-- [end:step-1g]
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
                // --8<-- [start:step-1h]
                ("curve", 2..) => Ok(Modeling::Curve(points)),
                _ => {
                    Err("point needs one coordinate; line two; polyline/curve at least two".into())
                }
                // --8<-- [end:step-1h]
            }
        }
        _ => Err("try trim 0.2 0.8, extend -0.2 1.2, or explode".into()),
    }
}
