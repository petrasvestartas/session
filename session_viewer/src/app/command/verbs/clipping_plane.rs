use crate::State;
use crate::app::clipping::{Mode, plane_from};
use crate::app::command::{Action, Spec};
use crate::app::coords;

pub const SPEC: Spec = Spec {
    names: &["clipping_plane"],
    aliases: &["ClippingPlane"],
    hint: "clipping_plane: an origin, then a point on the side to cut away · 3Point · XY / YZ / ZX through a point · On / Off every cut · Flip the selected plane · Fill Hatch / Solid (default) · Example: clipping_plane XY 0,0,1200",
    options: &[
        "clipping_plane Normal",
        "clipping_plane 3Point",
        "clipping_plane XY",
        "clipping_plane YZ",
        "clipping_plane ZX",
        "clipping_plane Flip",
        "clipping_plane On",
        "clipping_plane Off",
        "clipping_plane Fill", // before its values: the space after Fill must not take Hatch
        "clipping_plane Fill Hatch",
        "clipping_plane Fill Solid",
    ],
    arity: None,
    wait_for_option: false,
    wait_after_option: false,
    parse,
};

/// A mode to pick points in, typed points, a switch, a flip or a fill.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let usage =
        "try clipping_plane, clipping_plane XY 0,0,1200, clipping_plane Off, Flip or Fill Solid";
    let word = rest.first().map(|word| word.to_ascii_lowercase());

    match (word.as_deref(), rest.len()) {
        (None, _) => Ok(Box::new(Pick(Mode::Normal))),
        (Some("on"), 1) => Ok(Box::new(Switch(true))),
        (Some("off"), 1) => Ok(Box::new(Switch(false))),
        (Some("flip"), 1) => Ok(Box::new(Flip)),
        (Some("hatch"), 1) => Ok(Box::new(Fill(Some(false)))),
        (Some("solid"), 1) => Ok(Box::new(Fill(Some(true)))),
        (Some("fill"), 1) => Ok(Box::new(Fill(None))),
        (Some("fill"), 2) => match rest[1].to_ascii_lowercase().as_str() {
            "hatch" => Ok(Box::new(Fill(Some(false)))),
            "solid" => Ok(Box::new(Fill(Some(true)))),
            _ => Err("clipping_plane Fill takes Hatch or Solid".into()),
        },
        _ => {
            let mode = Mode::parse(rest[0]).ok_or(usage)?;

            if rest.len() == 1 {
                return Ok(Box::new(Pick(mode)));
            }

            let points = rest[1..]
                .iter()
                .map(|word| world_point(word))
                .collect::<Result<Vec<_>, _>>()?;

            if points.len() != mode.points() {
                return Err(format!(
                    "clipping_plane {} takes {} point{}",
                    mode.word(),
                    mode.points(),
                    if mode.points() == 1 { "" } else { "s" }
                ));
            }

            Ok(Box::new(Create { mode, points }))
        }
    }
}

/// One typed world point, finite and within ±1e12.
fn world_point(word: &str) -> Result<[f64; 3], String> {
    let Some(coords::Typed::Absolute { x, y, z }) = coords::parse(word) else {
        return Err(format!("`{word}` is not a world point; use x,y,z"));
    };
    let point = [x, y, z.unwrap_or(0.0)];

    if point.iter().any(|v| !v.is_finite() || v.abs() > 1e12) {
        return Err("coordinates must be finite and within ±1e12".into());
    }

    Ok(point)
}

/// Pick the plane's points with clicks, snaps or typed coordinates.
#[derive(Debug)]
struct Pick(Mode);

impl Action for Pick {
    /// Ask for the first point.
    fn run(&self, state: &mut State) -> Result<String, String> {
        Ok(state.pick_clipping_plane(self.0))
    }
}

/// A clipping plane through typed points.
#[derive(Debug)]
struct Create {
    mode: Mode,            // how the points place it
    points: Vec<[f64; 3]>, // world points
}

impl Action for Create {
    /// Add it, sized to the scene, and select it.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let half = state.clipping_half_size();
        let plane = plane_from(self.mode, &self.points, half)?;
        state.create_clipping_plane(plane)
    }
}

/// Every cut on or off.
#[derive(Debug)]
struct Switch(bool);

impl Action for Switch {
    /// Flip the switch the frame reads.
    fn run(&self, state: &mut State) -> Result<String, String> {
        Ok(state.set_clipping(self.0))
    }

    /// Switching mid-pick keeps the points.
    fn keeps_draft(&self) -> bool {
        true
    }
}

/// Cut the other side away.
#[derive(Debug)]
struct Flip;

impl Action for Flip {
    /// Reverse the selected clipping planes.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.flip_clipping_planes()
    }

    fn needs_selection(&self) -> bool {
        true
    }
}

/// Black hatch or solid light grey caps; `None` switches.
#[derive(Debug)]
struct Fill(Option<bool>);

impl Action for Fill {
    /// Set the cap fill the frame reads.
    fn run(&self, state: &mut State) -> Result<String, String> {
        Ok(state.set_clipping_fill(self.0))
    }

    /// A fill change mid-pick keeps the points.
    fn keeps_draft(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::app::command::{accept, completions, parse};

    /// What a parsed line becomes.
    fn parsed(line: &str) -> Result<String, String> {
        parse(line).map(|action| format!("{action:?}"))
    }

    /// Modes, switches, flips, fills and typed points parse; nonsense is refused.
    #[test]
    fn clipping_plane_lines_parse() {
        assert_eq!(parsed("clipping_plane"), Ok("Pick(Normal)".into()));
        assert_eq!(parsed("clipping_plane xy"), Ok("Pick(Xy)".into()));
        assert_eq!(parsed("clipping_plane 3point"), Ok("Pick(Points)".into()));
        assert_eq!(parsed("clipping_plane Off"), Ok("Switch(false)".into()));
        assert_eq!(parsed("clipping_plane on"), Ok("Switch(true)".into()));
        assert_eq!(parsed("clipping_plane Flip"), Ok("Flip".into()));
        assert_eq!(
            parsed("clipping_plane Fill Solid"),
            Ok("Fill(Some(true))".into())
        );
        assert_eq!(
            parsed("clipping_plane hatch"),
            Ok("Fill(Some(false))".into())
        );
        assert_eq!(
            parsed("clipping_plane XY 0,0,50"),
            Ok("Create { mode: Xy, points: [[0.0, 0.0, 50.0]] }".into())
        );
        assert!(parsed("clipping_plane Normal 0,0,0 0,0,1").is_ok());
        assert!(parsed("clipping_plane 3Point 0,0,0 1,0,0 0,1,0").is_ok());

        for line in [
            "clipping_plane Normal 0,0,0",
            "clipping_plane XY @1,2",
            "clipping_plane XY nan,0,0",
            "clipping_plane XY 1e13,0,0",
            "clipping_plane sideways",
            "clipping_plane Fill blue",
        ] {
            assert!(parsed(line).is_err(), "{line}");
        }
    }

    /// Typing part of the name completes it; one Enter runs it.
    #[test]
    fn clipping_plane_completes() {
        assert_eq!(accept("clip"), ("clipping_plane".into(), true));
        assert_eq!(
            accept("clipping_plane x"),
            ("clipping_plane XY".into(), true)
        );
        assert_eq!(completions("clipping_plane ").len(), 11);
        // typed Fill completes to itself, so the space after it keeps the value open
        assert_eq!(completions("clipping_plane Fill")[0], "clipping_plane Fill");
        assert_eq!(
            accept("clipping_plane Fill"),
            ("clipping_plane Fill".into(), true)
        );
        assert_eq!(parsed("clipping_plane Fill"), Ok("Fill(None)".into()));
        assert_eq!(
            completions("clipping_plane Fill S"),
            ["clipping_plane Fill Solid"]
        );
    }
}
