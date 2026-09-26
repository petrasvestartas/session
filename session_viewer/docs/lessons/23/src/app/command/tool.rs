use crate::State;
use session_rust::{Plane, Point, Xform};

/// What a tool wants after a point or a word.
#[derive(Debug, PartialEq)]
pub enum Next {
    More,           // ask for the next point
    Repeat(String), // acted once; keep the first point and ask again
    Done(String),   // finished, with the message to show
}

/// A command that asks for points; `state.features.draft` is empty while its methods run.
pub trait Tool: std::fmt::Debug {
    /// The command shown while it runs, e.g. `Move`.
    fn name(&self) -> &'static str;

    /// What the next point is for.
    fn prompt(&self, points: &[Point]) -> String;

    /// Buttons under the command line: (label, line it runs); "" is Enter, Cancel is a phone's Esc.
    fn options(&self) -> &'static [(&'static str, &'static str)] {
        &[("Cancel", "Escape")]
    }

    /// The option button shown as chosen, if any.
    fn chosen(&self) -> Option<&'static str> {
        None
    }

    /// A typed word the tool takes itself, e.g. an angle; None leaves it to coordinates.
    fn word(
        &mut self,
        _state: &mut State,
        _word: &str,
        _points: &[Point],
        _plane: &Plane,
    ) -> Option<Result<Next, String>> {
        None
    }

    /// The transform the selection shows with the cursor at `cursor`.
    fn preview(&self, _points: &[Point], _cursor: &Point, _plane: &Plane) -> Option<Xform> {
        None
    }

    /// The rubber band: one polyline in the scene.
    fn guide(&self, points: &[Point], cursor: Option<&Point>) -> Vec<Point> {
        points.iter().chain(cursor).cloned().collect()
    }

    /// A value shown beside the cursor, e.g. a distance.
    fn readout(&self, _points: &[Point], _cursor: &Point, _plane: &Plane) -> String {
        String::new()
    }

    /// A point was placed; an error refuses it.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        plane: &Plane,
    ) -> Result<Next, String>;

    /// Enter with no text.
    fn enter(&mut self, _state: &mut State, _points: &[Point]) -> Result<Next, String> {
        Ok(Next::Done(format!("{} cancelled", self.name())))
    }

    /// False when clicks never place points, so the prompt offers no coordinates.
    fn asks_points(&self) -> bool {
        true
    }

    /// Set up highlights once the tool is in charge.
    fn begin(&mut self, _state: &mut State) -> Result<Next, String> {
        Ok(Next::More)
    }

    /// True while a click picks an object; the answer goes to `picked`.
    fn picks(&self) -> bool {
        false
    }

    /// The object a click picked, if any.
    fn picked(&mut self, _state: &mut State, _row: Option<u32>) -> Result<Next, String> {
        Ok(Next::More)
    }

    /// A click at device pixels `at` the tool takes itself; None places a point.
    fn clicked(&mut self, _state: &mut State, _at: (f64, f64)) -> Option<Result<Next, String>> {
        None
    }

    /// The cursor moved; None snaps a point as usual, Some(changed) when the tool follows it.
    fn hovered(&mut self, _state: &mut State, _at: (f64, f64)) -> Option<bool> {
        None
    }

    /// The left button went down at device pixels `at`; true when the tool takes the drag that follows.
    fn pressed(&mut self, _state: &mut State, _at: (f64, f64)) -> bool {
        false
    }

    /// The pointer moved with the button held after `pressed`; true redraws.
    fn dragged(&mut self, _state: &mut State, _at: (f64, f64)) -> bool {
        false
    }

    /// The button came up after `pressed`; Shift adds, Ctrl removes.
    fn released(&mut self, _state: &mut State, _add: bool, _remove: bool) -> Result<Next, String> {
        Ok(Next::More)
    }

    /// A panel took the release or the pointer was lost: forget the drag, keep running.
    fn abandoned(&mut self) {}

    /// Strokes, squares and a label drawn over the scene.
    fn marks(&self, _state: &State) -> Option<Overlay> {
        None
    }

    /// Esc or another command: undo what only the GPU shows.
    fn cancel(&mut self, _state: &mut State) {}

    /// The tool as JSON, for the inspection tests.
    fn status(&self) -> serde_json::Value {
        serde_json::json!({ "command": self.name() })
    }
}

/// One polyline on screen, in device pixels.
pub struct Stroke {
    pub points: Vec<(f64, f64)>, // device pixels
    pub color: [u8; 3],          // RGB
    pub width: f32,              // CSS pixels
    pub dashed: bool,            // dashes instead of a solid line
}

/// What a tool draws over the scene.
#[derive(Default)]
pub struct Overlay {
    pub strokes: Vec<Stroke>,                // lines
    pub marks: Vec<(f64, f64)>,              // small squares, device pixels
    pub label: Option<((f64, f64), String)>, // text beside a point
}

/// The move that takes `from` to `to`.
pub fn translation(from: &Point, to: &Point) -> Xform {
    Xform::translation(to[0] - from[0], to[1] - from[1], to[2] - from[2])
}

/// A finite number typed as one word.
pub fn typed_number(word: &str) -> Option<f64> {
    word.parse::<f64>().ok().filter(|value| value.is_finite())
}
