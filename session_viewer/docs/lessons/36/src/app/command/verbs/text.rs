// --8<-- [start:text-parse]
use crate::State;
use crate::app::command::tool::{Next, Tool, typed_number};
use crate::app::command::{Action, Spec};
use crate::engine::text::{TextLabel, TextPlacement};
use session_rust::{Plane, Point, Vector};
use std::cell::Cell;

pub const SPEC: Spec = Spec {
    names: &["Text"],
    aliases: &[],
    hint: "Text Hello world · then click or type the lower-left point · Height N sets the letter height",
    options: &[],
    arity: None,
    wait_for_option: true, // completing `Te` gives `Text ` and waits for the words instead of running
    wait_after_option: false,
    parse,
};

const MAX_CHARS: usize = 80; // longest text, in characters
const FONT_SIZE: f32 = 18.0; // em size the label is shaped at
const LINE_HEIGHT: f32 = 26.0; // its line box

thread_local! {
    static HEIGHT: Cell<f64> = const { Cell::new(0.0) }; // the last height used, 0 before any
}

/// The words after the verb, joined by single spaces.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let text = rest.join(" ");

    if !(1..=MAX_CHARS).contains(&text.chars().count()) {
        return Err(format!(
            "Text takes 1 to {MAX_CHARS} characters · Example: Text Hello world"
        ));
    }

    Ok(Box::new(Text(text)))
}

/// Ask where to put the text.
#[derive(Debug)]
struct Text(String);

impl Action for Text {
    /// Start asking for the insertion point.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let height = match HEIGHT.get() {
            // The first text is a 25th of the view distance, rounded to a tidy 1, 2 or 5.
            0.0 => nice(state.camera.distance_world() / 25.0),
            last => last,
        };
        state.open_tool(Box::new(Placing {
            text: self.0.clone(),
            height,
            asking: false,
        }))
    }
}
// --8<-- [end:text-parse]

// --8<-- [start:placing]
/// Picks the lower-left point; `Height N` changes the letter height first.
#[derive(Debug)]
struct Placing {
    text: String, // what to write
    height: f64,  // letter height in scene units
    asking: bool, // the next word is the height
}

impl Tool for Placing {
    fn name(&self) -> &'static str {
        "Text"
    }

    fn prompt(&self, _points: &[Point]) -> String {
        if self.asking {
            return "Type the letter height".into();
        }

        format!(
            "Lower-left point of \"{}\" · Height {} (type Height N)",
            self.text, self.height
        )
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        &[("Height", "Height"), ("Cancel", "Escape")]
    }

    /// `Height`, then a number above zero.
    fn word(
        &mut self,
        _state: &mut State,
        word: &str,
        _points: &[Point],
        _plane: &Plane,
    ) -> Option<Result<Next, String>> {
        if word.eq_ignore_ascii_case("height") {
            self.asking = true;
            return Some(Ok(Next::More));
        }

        if !self.asking {
            return None;
        }

        match typed_number(word) {
            Some(height) if height > 0.0 && height <= 1e9 => {
                self.height = height;
                self.asking = false;
                Some(Ok(Next::More))
            }
            _ => Some(Err("Height must be a number above zero".into())),
        }
    }

    /// The point is the lower-left corner; the text reads from the current view.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        plane: &Plane,
    ) -> Result<Next, String> {
        let camera = &state.camera.orientation;
        // The text lies in the drawing plane, turned so it reads left to right from where the camera looks.
        let (right, up) = reading_axes(
            &plane.x_axis(),
            &plane.y_axis(),
            &camera.rotate_vector(Vector::x_axis()),
            &camera.rotate_vector(Vector::z_axis()),
        );
        let label = label(&self.text, &points[0], right, up, self.height);
        // The label joins the scene texts as one undo step.
        state.add_text(label);
        HEIGHT.set(self.height);
        Ok(Next::Done(format!(
            "Created text \"{}\", height {}. Undo removes it.",
            self.text, self.height
        )))
    }
}
// --8<-- [end:placing]

// --8<-- [start:text-label]
/// A white-on-black label standing on `point` in the plane of `right` and `up`.
fn label(text: &str, point: &Point, right: [f64; 3], up: [f64; 3], height: f64) -> TextLabel {
    let lift = height * f64::from(LINE_HEIGHT) / f64::from(FONT_SIZE); // the line box above the point
    TextLabel {
        id: 0,
        object: None,
        text: text.into(),
        font_size: FONT_SIZE,
        line_height: LINE_HEIGHT,
        color: [255; 4],
        placement: TextPlacement::WorldPlane {
            world: [0, 1, 2].map(|axis| point[axis] + up[axis] * lift),
            right,
            up,
            world_height: height,
        },
        clip: None,
    }
}

/// Of the plane axes `a` and `b`, the one along the screen's right and the other toward its top.
fn reading_axes(a: &Vector, b: &Vector, right: &Vector, up: &Vector) -> ([f64; 3], [f64; 3]) {
    // The plane axis closest to the screen's right runs along the text; each axis is flipped if it points backward.
    let (along, across) = if a.dot(right).abs() >= b.dot(right).abs() {
        (a, b)
    } else {
        (b, a)
    };
    let sign = |axis: &Vector, toward: &Vector| {
        let flip = if axis.dot(toward) < 0.0 { -1.0 } else { 1.0 };
        [axis[0] * flip, axis[1] * flip, axis[2] * flip]
    };
    (sign(along, right), sign(across, up))
}

/// `value` rounded down to 1, 2 or 5 times a power of ten; 1 when it is not a positive number.
fn nice(value: f64) -> f64 {
    if !(value > 0.0 && value.is_finite()) {
        return 1.0;
    }

    let power = 10f64.powf(value.log10().floor());
    let step = match value / power {
        m if m >= 5.0 => 5.0,
        m if m >= 2.0 => 2.0,
        _ => 1.0,
    };
    step * power
}
// --8<-- [end:text-label]

// --8<-- [start:text-tests]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::{Camera, View};

    /// The words join with single spaces; empty and too long are refused.
    #[test]
    fn the_words_become_the_text() {
        assert_eq!(
            format!("{:?}", parse("Text", &["Hello", "world"]).unwrap()),
            "Text(\"Hello world\")"
        );
        assert!(parse("Text", &[]).is_err());
        assert!(parse("Text", &["x".repeat(81).as_str()]).is_err());
        assert!(parse("Text", &["é".repeat(80).as_str()]).is_ok());
    }

    /// Each standard view reads left to right and upward.
    #[test]
    fn the_text_reads_from_every_standard_view() {
        let x = Vector::x_axis();
        let y = Vector::y_axis();
        let z = Vector::z_axis();

        for (view, a, b, right, up) in [
            (View::Top, &x, &y, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            (View::Front, &x, &z, [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            (View::Right, &y, &z, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
            (View::Back, &x, &z, [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
            (View::Bottom, &x, &y, [1.0, 0.0, 0.0], [0.0, -1.0, 0.0]),
        ] {
            let mut camera = Camera::new();
            camera.set_view(view);
            let turn = &camera.orientation;
            let found = reading_axes(
                a,
                b,
                &turn.rotate_vector(Vector::x_axis()),
                &turn.rotate_vector(Vector::z_axis()),
            );
            let close = |p: [f64; 3], q: [f64; 3]| (0..3).all(|i| (p[i] - q[i]).abs() < 1e-9);
            assert!(
                close(found.0, right) && close(found.1, up),
                "{found:?} should be {right:?} {up:?}"
            );
        }
    }

    /// The line box stands on the point, and the label shapes.
    #[test]
    fn the_line_box_stands_on_the_insertion_point() {
        let label = label(
            "Hello",
            &Point::new(1.0, 2.0, 3.0),
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            18.0,
        );
        let TextPlacement::WorldPlane { world, .. } = label.placement else {
            panic!("a plane label");
        };
        assert_eq!(world, [1.0, 28.0, 3.0]);
        assert!(
            crate::engine::text::TextDocument::new()
                .set_labels(vec![label])
                .is_ok()
        );
    }

    /// Heights round down to 1, 2 or 5 times a power of ten.
    #[test]
    fn nice_heights_round_down() {
        assert_eq!(nice(137.0), 100.0);
        assert!((nice(0.37) - 0.2).abs() < 1e-12);
        assert_eq!(nice(5.0), 5.0);
        assert_eq!(nice(0.0), 1.0);
        assert_eq!(nice(f64::NAN), 1.0);
    }
}
// --8<-- [end:text-tests]
