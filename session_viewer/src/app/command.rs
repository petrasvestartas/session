//! The command line: one typed line becomes one named action.
//!
//! Parsing is here and doing is in `State`, so what a line MEANS can be tested without a
//! window, a device or a scene. A verb the viewer does not have is an error with the line
//! quoted back, never a silent no-op: a command line that ignores what you typed is worse
//! than one that refuses it.
//!
//! Every verb is an action the viewer already has. The command line is a second way to reach
//! them, not a second implementation of them.

use crate::app::coords;
use crate::app::gizmo::Axis;

/// What a line asked for.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Command {
    /// Move the selection by a world offset, in millimetres.
    Move([f64; 3]),
    /// Turn the selection about one axis through its own centre, in degrees.
    Rotate { axis: Axis, degrees: f64 },
    /// Scale the selection about its own centre.
    Scale(f64),
    Delete,
    Undo,
    Redo,
    Hide,
    ShowAll,
    Fit,
    /// Clear the selection.
    Escape,
}

/// Parse one line. `Err` carries what to show the person who typed it.
pub fn parse(line: &str) -> Result<Command, String> {
    let line = line.trim();
    if line.is_empty() {
        return Err("nothing typed".into());
    }
    let mut words = line.split_whitespace();
    let verb = words.next().unwrap_or_default().to_ascii_lowercase();
    let rest: Vec<&str> = words.collect();
    match verb.as_str() {
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

/// `move` takes the coordinate parser's own syntax, so `10,0,0`, `@10,0` and `10<45` mean here
/// what they mean anywhere else in the viewer.
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
        assert!(parse("scale 0").is_err(), "a zero scale collapses the object");
        assert!(parse("rotate 90").is_err(), "no axis");
        assert!(parse("rotate x").is_err(), "no angle");
        assert!(parse("move sideways").is_err());
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
