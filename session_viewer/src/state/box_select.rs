use super::State;
use crate::app::command::tool::{Overlay, Stroke};
use crate::app::command::verbs::selecting::{self, Reach, Region};

const GREY: [u8; 3] = [110, 110, 110]; // the thin outline
const FILL: [u8; 4] = [160, 160, 160, 40]; // a translucent light grey inside, the scene shows through

/// A selection rectangle being dragged, device pixels.
pub struct BoxSelect {
    down: (f64, f64),   // the corner pressed
    cursor: (f64, f64), // the other corner, under the pointer
}

impl BoxSelect {
    /// Dragged left to right it takes what lies wholly inside; right to left whatever it touches.
    fn reach(&self) -> Reach {
        match self.cursor.0 >= self.down.0 {
            true => Reach::Window,
            false => Reach::Crossing,
        }
    }
}

impl State {
    /// Start a rectangle pressed at `down`, its other corner at `at`; never while drawing or splitting.
    pub(crate) fn start_box(&mut self, down: (f64, f64), at: (f64, f64)) -> bool {
        let mut busy = false;
        busy |= self.drafting(); // register:commands
        busy |= self.features.pending_split.is_some(); // register:split

        if busy {
            return false;
        }

        self.features.box_select = Some(BoxSelect { down, cursor: at });
        true
    }

    /// The other corner follows the pointer.
    pub(crate) fn drag_box(&mut self, at: (f64, f64)) -> bool {
        let Some(box_) = self.features.box_select.as_mut() else {
            return false;
        };
        box_.cursor = at;
        true
    }

    /// Let go: select what the rectangle takes; Shift adds to the selection, Ctrl takes out of it.
    pub(crate) fn end_box(&mut self) -> bool {
        self.finish_box(self.shift_held, self.ctrl_held)
    }

    /// Select what the rectangle takes with these keys, e.g. those held when a flick was let go.
    pub(crate) fn finish_box(&mut self, shift: bool, ctrl: bool) -> bool {
        let Some(box_) = self.features.box_select.take() else {
            return false;
        };
        let region = Region::rectangle(box_.down, box_.cursor);
        let found = selecting::found(self, &region, box_.reach());
        let count = found.len();
        let remove = ctrl;
        let total = selecting::apply(self, found, shift && !remove, remove);
        let how = match box_.reach() {
            Reach::Window => "inside",
            Reach::Crossing => "touched",
        };
        self.status(&format!("{count} {how} · {total} selected"));
        true
    }

    /// The rectangle while it is dragged: solid for a window, dashed for a crossing.
    pub fn box_overlay(&self) -> Option<Overlay> {
        let box_ = self.features.box_select.as_ref()?;
        let (a, b) = (box_.down, box_.cursor);
        Some(Overlay {
            strokes: vec![Stroke {
                points: vec![a, (b.0, a.1), b, (a.0, b.1), a],
                color: GREY,
                width: 1.0,
                dashed: box_.reach() == Reach::Crossing,
            }],
            fill: Some(FILL),
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Left to right is a window, right to left a crossing.
    #[test]
    fn the_drag_direction_picks_window_or_crossing() {
        let to_the_right = BoxSelect {
            down: (10.0, 10.0),
            cursor: (90.0, 5.0),
        };
        let to_the_left = BoxSelect {
            down: (90.0, 10.0),
            cursor: (10.0, 50.0),
        };
        assert_eq!(to_the_right.reach(), Reach::Window);
        assert_eq!(to_the_left.reach(), Reach::Crossing);
    }
}
