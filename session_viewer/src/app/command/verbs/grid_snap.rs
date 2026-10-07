use crate::State;
use crate::app::command::tool::typed_number;
use crate::app::command::{Action, Spec, on_off};

pub const SPEC: Spec = Spec {
    options: &["Grid Snap On", "Grid Snap Off"],
    ..Spec::new(
        &["Grid Snap"],
        "Grid Snap (On Off) or a spacing: a point off every object lands on the grid of the construction plane, 100 mm apart at start · Example: Grid Snap 50",
        parse,
    )
};

/// On, Off, a spacing, or nothing to flip.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if let [word] = rest
        && let Some(step) = typed_number(word)
    {
        if step <= 0.0 {
            return Err("The grid spacing must be above zero · Example: Grid Snap 50".into());
        }

        return Ok(Box::new(GridSnap {
            on: Some(true),
            step: Some(step),
        }));
    }

    Ok(Box::new(GridSnap {
        on: on_off(rest, "Grid Snap (On Off) or a spacing")?,
        step: None,
    }))
}

#[derive(Debug)]
struct GridSnap {
    on: Option<bool>,  // None flips
    step: Option<f64>, // a new spacing, mm
}

impl Action for GridSnap {
    /// Set the switch and the spacing the drawing code reads.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let snap = &mut state.features.snap;
        snap.grid = self.on.unwrap_or(!snap.grid);

        if let Some(step) = self.step {
            snap.grid_step = step;
        }

        Ok(match snap.grid {
            true => format!("Grid Snap On · {} mm", snap.grid_step),
            false => "Grid Snap Off".into(),
        })
    }

    /// Grid Snap is changed mid-draw, so the draft stays.
    fn keeps_draft(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::app::command::parse;
    use crate::app::snap::on_grid;
    use session_rust::{Point, Vector};

    /// The plane coordinates round to the step, the height stays.
    #[test]
    fn grid_snap_rounds_along_the_plane() {
        assert!(
            parse("Grid Snap").is_ok()
                && parse("grid snap off").is_ok()
                && parse("Grid Snap 25").is_ok()
        );
        assert!(parse("Grid Snap -1").is_err() && parse("Grid Snap maybe").is_err());
        let p = on_grid(
            &Point::new(149.0, 51.0, 33.3),
            &Vector::new(0.0, 0.0, 1.0),
            100.0,
        );
        assert_eq!([p[0], p[1], p[2]], [100.0, 100.0, 33.3]);
        let side = on_grid(
            &Point::new(149.0, 51.0, 33.3),
            &Vector::new(1.0, 0.0, 0.0),
            10.0,
        );
        assert_eq!([side[0], side[1], side[2]], [149.0, 50.0, 30.0]);
    }
}
