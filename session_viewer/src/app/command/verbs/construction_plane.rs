use crate::State;
use crate::app::command::tool::{Next, Tool};
use crate::app::command::{Action, Spec};
use crate::app::coords;
use crate::app::cplane::CPlane;
use session_rust::{Plane, Point};

pub const NAME: &str = "Construction Plane";

pub const SPEC: Spec = Spec {
    options: &[
        "Construction Plane XY",
        "Construction Plane XZ",
        "Construction Plane YZ",
        "Construction Plane 3 Point",
        "Construction Plane View",
    ],
    wait_for_option: true,
    ..Spec::new(
        &[NAME],
        "Construction Plane XY / XZ / YZ fixes the plane points land on and the grid lies on · 3 Point picks its origin, a point on its x axis and one on its y side · View follows the view again · Example: Construction Plane 3 Point 0,0,0 1000,0,1000 0,1000,0",
        parse,
    )
};

/// A preset, View, or 3 Point with or without its three typed points.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let usage = "Construction Plane XY, XZ, YZ, View or 3 Point";
    let word = rest
        .first()
        .map(|word| word.to_ascii_lowercase())
        .unwrap_or_default();
    let plane = match word.as_str() {
        "xy" if rest.len() == 1 => Some(CPlane::Xy),
        "xz" if rest.len() == 1 => Some(CPlane::Xz),
        "yz" if rest.len() == 1 => Some(CPlane::Yz),
        "view" if rest.len() == 1 => None,
        "3" if rest.get(1).is_some_and(|w| w.eq_ignore_ascii_case("point")) => {
            return three_points(&rest[2..]);
        }
        "3point" => return three_points(&rest[1..]),
        "" => return Ok(Box::new(ChoosePlane)),
        _ => return Err(format!("Choose a plane · {usage}")),
    };
    Ok(Box::new(SetPlane(plane)))
}

/// Pick the three points, or take them typed.
fn three_points(rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Ok(Box::new(PickPlane));
    }

    let points = rest
        .iter()
        .map(|word| match coords::absolute(word) {
            Some([x, y, z]) => Ok(Point::new(x, y, z)),
            None => Err("Construction Plane 3 Point takes three x,y,z points".to_string()),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let [origin, on_x, in_plane] = points.as_slice() else {
        return Err("Construction Plane 3 Point takes three x,y,z points".into());
    };
    let plane =
        CPlane::from_3_points(origin, on_x, in_plane).ok_or("The three points are in line")?;
    Ok(Box::new(SetPlane(Some(plane))))
}

#[derive(Debug)]
struct SetPlane(Option<CPlane>); // None follows the view

impl Action for SetPlane {
    /// Fix the plane, or let it follow the view again.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.set_construction_plane(self.0);
        Ok(match self.0 {
            None => "Construction Plane View · points land on the plane the view faces".into(),
            Some(_) => format!(
                "{NAME} {}",
                state.construction_plane_status()["mode"]
                    .as_str()
                    .unwrap_or("")
            ),
        })
    }

    /// The plane changes mid-draw, so the draft stays.
    fn keeps_draft(&self) -> bool {
        true
    }
}

/// Picks the plane's origin, a point on its x axis and one on its y side.
#[derive(Clone, Debug)]
struct PickPlane;

impl Action for PickPlane {
    /// Start asking for the points; nothing needs to be selected.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.open_tool(Box::new(self.clone()))
    }
}

impl Tool for PickPlane {
    fn name(&self) -> &'static str {
        NAME
    }

    fn prompt(&self, points: &[Point]) -> String {
        match points.len() {
            0 => "Plane origin".into(),
            1 => "Point on the plane's x axis".into(),
            _ => "Point on the plane's y side".into(),
        }
    }

    /// The third point sets the plane.
    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        let [origin, on_x, in_plane] = points else {
            return Ok(Next::More);
        };
        let plane = CPlane::from_3_points(origin, on_x, in_plane)
            .ok_or("The three points are in line; pick the last one again")?;
        state.set_construction_plane(Some(plane));
        Ok(Next::Done(format!("{NAME} 3 Point · the grid lies on it")))
    }
}

/// The plane chosen from the options in the prompt.
#[derive(Debug, Clone)]
struct ChoosePlane;

impl Action for ChoosePlane {
    /// Show the choices.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.open_tool(Box::new(self.clone()))
    }
}

impl Tool for ChoosePlane {
    fn name(&self) -> &'static str {
        NAME
    }

    fn prompt(&self, _points: &[Point]) -> String {
        "choose the plane the grid lies on and points land on".into()
    }

    /// The planes as words to click; 3 Point runs its own command.
    fn buttons(&self) -> Vec<(String, String)> {
        [
            ("XY", "XY"),
            ("XZ", "XZ"),
            ("YZ", "YZ"),
            ("3 Point", "Construction Plane 3 Point"),
            ("View", "View"),
            ("Cancel", "Escape"),
        ]
        .into_iter()
        .map(|(label, line)| (label.to_string(), line.to_string()))
        .collect()
    }

    fn asks_points(&self) -> bool {
        false
    }

    /// A preset or View sets the plane; any other word is left to the commands.
    fn word(
        &mut self,
        state: &mut State,
        word: &str,
        _points: &[Point],
        _plane: &Plane,
    ) -> Option<Result<Next, String>> {
        let plane = match word.to_ascii_lowercase().as_str() {
            "xy" => Some(CPlane::Xy),
            "xz" => Some(CPlane::Xz),
            "yz" => Some(CPlane::Yz),
            "view" => None,
            _ => return None,
        };
        Some(SetPlane(plane).run(state).map(Next::Done))
    }

    fn placed(
        &mut self,
        _state: &mut State,
        _points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        Err("Choose XY, XZ, YZ, 3 Point or View".into())
    }

    fn enter(&mut self, _state: &mut State, _points: &[Point]) -> Result<Next, String> {
        Ok(Next::Done(format!("{NAME} unchanged")))
    }
}

#[cfg(test)]
mod tests {
    use crate::app::command::parse;

    #[test]
    fn construction_plane_parses_its_planes() {
        for line in [
            "Construction Plane XY",
            "construction plane xz",
            "Construction Plane YZ",
            "Construction Plane View",
            "Construction Plane 3 Point",
            "Construction Plane 3 Point 0,0,0 1000,0,1000 0,1000,0",
            "Construction Plane",
        ] {
            assert!(parse(line).is_ok(), "{line}");
        }
        for line in [
            "Construction Plane ZZ",
            "Construction Plane 3 Point 0,0,0 1,1,1 2,2,2",
            "Construction Plane 3 Point 0,0,0 1,0,0",
        ] {
            assert!(parse(line).is_err(), "{line}");
        }
    }
}
