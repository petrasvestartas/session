//! What the Element commands share: the picked curves and points in world space, and adding a wood
//! element with its hidden base plane under `attributes`, as wood's sync_attributes writes it.

use crate::State;
use crate::app::command::tool::{Next, Tool, typed_number};
use session_rust::{Color, Geometry, Plane, Point, Polyline};
use std::rc::Rc;
use wood::WoodElement;

/// The picked polylines and lines, in pick order and world space; anything else is skipped.
pub fn picked_polylines(state: &State) -> Vec<Polyline> {
    let mut out = Vec::new();

    for row in state.ordered_rows() {
        let (Some(geometry), Some(place)) =
            (state.scene.geometry(row), state.scene.placement_of(row))
        else {
            continue;
        };

        match geometry {
            Geometry::Polyline(polyline) => out.push(polyline.transformed(&place)),
            Geometry::Line(line) => {
                let line = line.transformed(&place);
                out.push(Polyline::new(vec![line.start(), line.end()]));
            }
            _ => {}
        }
    }

    out
}

/// The picked points and planes as world planes, in pick order: a point gives the world xy plane there.
pub fn picked_planes(state: &State) -> Vec<Plane> {
    let mut out = Vec::new();

    for row in state.ordered_rows() {
        let (Some(geometry), Some(place)) =
            (state.scene.geometry(row), state.scene.placement_of(row))
        else {
            continue;
        };

        match geometry {
            Geometry::Point(point) => {
                let at: Point = point.transformed(&place);
                out.push(Plane::new(
                    at,
                    session_rust::Vector::x_axis(),
                    session_rust::Vector::y_axis(),
                ));
            }
            Geometry::Plane(plane) => out.push(plane.transformed(&place)),
            _ => {}
        }
    }

    out
}

/// The closed polylines among the picked curves, in pick order.
pub fn picked_loops(state: &State) -> Vec<Polyline> {
    picked_polylines(state)
        .into_iter()
        .filter(|polyline| polyline.is_closed() && polyline.point_count() >= 4)
        .collect()
}

/// The positive lengths typed after the verb, the defaults for the ones left out.
pub fn lengths<const N: usize>(
    rest: &[&str],
    defaults: [f64; N],
    usage: &str,
) -> Result<[f64; N], String> {
    if rest.len() > N {
        return Err(format!("Too many numbers · {usage}"));
    }

    let mut values = defaults;

    for (value, word) in values.iter_mut().zip(rest) {
        *value = typed_number(word)
            .filter(|value| *value > 0.0)
            .ok_or_else(|| format!("`{word}` is not a positive length · {usage}"))?;
    }

    Ok(values)
}

/// The element's base plane as wood writes it: hidden, black, named base_plane.
fn base_plane<T: WoodElement>(element: &T) -> Vec<Geometry> {
    let Some(mut plane) = element.base_plane() else {
        return Vec::new();
    };

    plane.name = "base_plane".into();
    plane.is_visible = false;
    plane.linecolor = Color::black();

    vec![Geometry::Plane(Rc::new(plane))]
}

/// Each element as a kernel Element with its base plane, ready for Scene::create_with_attributes.
pub fn items<T: WoodElement>(elements: &[T]) -> Vec<(Geometry, Vec<Geometry>)> {
    elements
        .iter()
        .map(|element| {
            (
                Geometry::Element(Rc::new(element.to_element())),
                base_plane(element),
            )
        })
        .collect()
}

/// Add the elements to the current layer as one undo step, each with its base plane and features, the picked curves and points becoming them; select them and say so.
pub fn create<T: WoodElement>(
    state: &mut State,
    elements: Vec<T>,
    what: &str,
) -> Result<String, String> {
    let consumed: Vec<u32> = state
        .ordered_rows()
        .into_iter()
        .filter(|&row| {
            state
                .scene
                .geometry(row)
                .is_some_and(|g| is_curve(g) || is_station(g))
        })
        .collect();
    state.select(None);
    let made = state.scene.create_with_attributes(
        items(&elements),
        &format!("Element {what}"),
        &consumed,
    )?;
    state.after_history();
    let rows = made
        .iter()
        .filter_map(|(doc, guid)| state.scene.row_of(*doc, guid))
        .collect();
    state.select_rows(rows, false);
    let noun = match elements.len() {
        1 => what.to_ascii_lowercase(),
        count => format!("{count} {}s", what.to_ascii_lowercase()),
    };

    Ok(format!(
        "Created {noun} from its curves · Undo brings them back"
    ))
}

/// One length an Element command asks for: its name on the command line and its value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Param {
    pub name: &'static str, // e.g. Thickness
    pub value: f64,         // mm
}

/// What an Element command picks and how it makes its elements.
#[derive(Debug)]
pub struct Recipe {
    pub name: &'static str,          // the command, e.g. Element Plate
    pub picks: &'static str,         // what to click, e.g. closed polylines
    pub fits: fn(&Geometry) -> bool, // what a click may pick
    pub options: &'static [(&'static str, &'static str)], // the buttons under the command line
    pub make: fn(&mut State, &[f64]) -> Result<String, String>, // the elements from the selection, in pick order
}

/// A line or polyline, an axis or an outline.
pub fn is_curve(geometry: &Geometry) -> bool {
    matches!(geometry, Geometry::Polyline(_) | Geometry::Line(_))
}

/// A point or a plane, where a support stands.
pub fn is_station(geometry: &Geometry) -> bool {
    matches!(geometry, Geometry::Point(_) | Geometry::Plane(_))
}

/// Run an Element command: typed values with a fitting selection make the elements at once, else the command asks for picks and values.
pub fn start(
    state: &mut State,
    recipe: &'static Recipe,
    params: Vec<Param>,
    typed: bool,
) -> Result<String, String> {
    let fitting = state
        .ordered_rows()
        .iter()
        .any(|&row| state.scene.geometry(row).is_some_and(recipe.fits));

    if typed && fitting {
        let values: Vec<f64> = params.iter().map(|param| param.value).collect();
        return (recipe.make)(state, &values);
    }

    state.open_tool(Box::new(ElementTool::new(recipe, params)))
}

/// An Element command running: the picked rows, the values, the one being typed.
#[derive(Debug)]
pub struct ElementTool {
    recipe: &'static Recipe,
    params: Vec<Param>,
    picked: Vec<u32>,      // in click order
    asking: Option<usize>, // the value a name word asked for
    next: usize,           // the value a bare number sets
}

impl ElementTool {
    /// A command with nothing picked yet.
    pub fn new(recipe: &'static Recipe, params: Vec<Param>) -> Self {
        Self {
            recipe,
            params,
            picked: Vec::new(),
            asking: None,
            next: 0,
        }
    }

    /// Pick a row, or drop it when picked.
    fn toggle(&mut self, state: &mut State, row: u32) -> Result<(), String> {
        if let Some(at) = self.picked.iter().position(|picked| *picked == row) {
            self.picked.remove(at);
            state.gpu.set_selected(row, false);
            return Ok(());
        }

        if !state.scene.selectable(row) || !state.scene.geometry(row).is_some_and(self.recipe.fits)
        {
            return Err(format!("{}: pick {}", self.recipe.name, self.recipe.picks));
        }

        self.picked.push(row);
        state.gpu.set_selected(row, true);
        Ok(())
    }

    /// Clear the pick highlights.
    fn unmark(&self, state: &mut State) {
        for &row in &self.picked {
            state.gpu.set_selected(row, false);
        }
    }

    /// The values as the prompt shows them, e.g. `Width 120 · Height 200`.
    fn values(&self) -> String {
        self.params
            .iter()
            .map(|param| format!("{} {}", param.name, param.value))
            .collect::<Vec<_>>()
            .join(" · ")
    }

    /// Take one typed word: a value name asks for it, `Name=60` sets it, a number sets the asked or the next value.
    fn take(&mut self, word: &str) -> Result<(), String> {
        let (name, number) = match word.split_once('=') {
            Some((name, number)) => (Some(name), Some(number)),
            None if typed_number(word).is_some() => (None, Some(word)),
            None => (Some(word), None),
        };

        if let Some(name) = name {
            let at = self
                .params
                .iter()
                .position(|param| param.name.eq_ignore_ascii_case(name))
                .ok_or_else(|| {
                    format!(
                        "`{word}` is not a value of {} · {}",
                        self.recipe.name,
                        self.values()
                    )
                })?;
            self.asking = Some(at);
        }

        let Some(number) = number else {
            return Ok(());
        };
        let value = typed_number(number)
            .filter(|value| *value > 0.0)
            .ok_or_else(|| format!("`{number}` is not a positive length"))?;

        if self.params.is_empty() {
            return Err(format!(
                "{} takes no values · pick {}",
                self.recipe.name, self.recipe.picks
            ));
        }

        let at = self.asking.take().unwrap_or(self.next % self.params.len());
        self.params[at].value = value;
        self.next = at + 1;
        Ok(())
    }
}

impl Tool for ElementTool {
    fn name(&self) -> &'static str {
        self.recipe.name
    }

    fn prompt(&self, _points: &[Point]) -> String {
        if let Some(at) = self.asking {
            let param = self.params[at];
            return format!(
                "{} <{}>: type a number · Enter keeps it",
                param.name, param.value
            );
        }

        let values = match self.params.is_empty() {
            true => String::new(),
            false => format!(" · {} · type a value to change it", self.values()),
        };

        format!(
            "pick {} ({} picked){values} · Enter creates",
            self.recipe.picks,
            self.picked.len()
        )
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        self.recipe.options
    }

    /// One button per value showing it, e.g. `Thickness 40`; a click asks for that value alone.
    fn buttons(&self) -> Vec<(String, String)> {
        let values = self.params.iter().map(|param| {
            (
                format!("{} {}", param.name, param.value),
                param.name.to_string(),
            )
        });
        let rest = [("Create", ""), ("Cancel", "Escape")]
            .into_iter()
            .map(|(label, line)| (label.to_string(), line.to_string()));
        values.chain(rest).collect()
    }

    fn asks_points(&self) -> bool {
        false
    }

    /// The fitting selection is picked already.
    fn begin(&mut self, state: &mut State) -> Result<Next, String> {
        let selected = state.ordered_rows();
        state.select(None);
        state.place_gizmo(None);

        for row in selected {
            let _ = self.toggle(state, row);
        }

        Ok(Next::More)
    }

    fn picks(&self) -> bool {
        true
    }

    fn picked(&mut self, state: &mut State, row: Option<u32>) -> Result<Next, String> {
        let row = row.ok_or_else(|| format!("Nothing there: pick {}", self.recipe.picks))?;
        self.toggle(state, row)?;
        Ok(Next::More)
    }

    fn word(
        &mut self,
        _state: &mut State,
        word: &str,
        _points: &[Point],
        _plane: &Plane,
    ) -> Option<Result<Next, String>> {
        Some(self.take(word).map(|()| Next::More))
    }

    fn placed(
        &mut self,
        _state: &mut State,
        _points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        Err(format!("{}: pick {}", self.recipe.name, self.recipe.picks))
    }

    /// Enter makes the elements from the picks, in click order.
    fn enter(&mut self, state: &mut State, _points: &[Point]) -> Result<Next, String> {
        // Enter on an asked value keeps it
        if self.asking.take().is_some() {
            return Ok(Next::More);
        }

        if self.picked.is_empty() {
            return Err(format!("Pick {} first, then Enter", self.recipe.picks));
        }

        self.unmark(state);
        state.select_rows(self.picked.clone(), false);
        let values: Vec<f64> = self.params.iter().map(|param| param.value).collect();

        match (self.recipe.make)(state, &values) {
            Ok(message) => Ok(Next::Done(message)),
            Err(error) => {
                // keep the picks for another try
                state.select(None);
                for &row in &self.picked {
                    state.gpu.set_selected(row, true);
                }
                Err(error)
            }
        }
    }

    fn cancel(&mut self, state: &mut State) {
        self.unmark(state);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::app::scene::Scene;
    use session_rust::Point;
    use wood::Plate;

    /// A closed square of side at z.
    pub fn square(side: f64, z: f64) -> Polyline {
        let corners = [
            (0.0, 0.0),
            (side, 0.0),
            (side, side),
            (0.0, side),
            (0.0, 0.0),
        ];
        Polyline::new(corners.iter().map(|&(x, y)| Point::new(x, y, z)).collect())
    }

    /// Lengths default when left out and refuse anything but a positive number.
    fn no_make(_state: &mut State, _values: &[f64]) -> Result<String, String> {
        Ok(String::new())
    }

    static BEAM: Recipe = Recipe {
        name: "Element Beam",
        picks: "lines",
        fits: is_curve,
        options: &[],
        make: no_make,
    };

    /// A name asks for its value, Name=value sets it, bare numbers fill the values in turn.
    #[test]
    fn typed_words_change_the_values() {
        let params = vec![
            Param {
                name: "Width",
                value: 120.0,
            },
            Param {
                name: "Height",
                value: 200.0,
            },
        ];
        let mut tool = ElementTool::new(&BEAM, params);
        assert!(tool.take("height").is_ok() && tool.prompt(&[]).starts_with("Height <200>"));
        assert!(tool.take("250").is_ok());
        assert!(tool.take("Width=150").is_ok());
        assert_eq!((tool.params[0].value, tool.params[1].value), (150.0, 250.0));
        assert!(tool.take("100").is_ok() && tool.take("300").is_ok());
        assert_eq!(
            (tool.params[0].value, tool.params[1].value),
            (300.0, 100.0),
            "after Width the next number is Height"
        );
        assert!(tool.prompt(&[]).contains("Width 300 · Height 100"));
        assert!(tool.take("Depth").is_err() && tool.take("-5").is_err());
        let labels: Vec<String> = tool.buttons().into_iter().map(|(label, _)| label).collect();
        assert_eq!(
            labels,
            ["Width 300", "Height 100", "Create", "Cancel"],
            "a button per value"
        );
        assert_eq!(tool.buttons()[1].1, "Height", "a click asks for that value");
        let mut bare = ElementTool::new(&BEAM, Vec::new());
        assert!(bare.take("40").is_err(), "no values to take");
    }

    #[test]
    fn lengths_default_and_refuse() {
        assert_eq!(lengths(&[], [40.0], "usage"), Ok([40.0]));
        assert_eq!(lengths(&["12"], [1.0, 2.0], "usage"), Ok([12.0, 2.0]));
        assert!(lengths(&["0"], [1.0], "usage").is_err());
        assert!(lengths(&["-3"], [1.0], "usage").is_err());
        assert!(lengths(&["x"], [1.0], "usage").is_err());
        assert!(lengths(&["1", "2"], [1.0], "usage").is_err());
    }

    /// A beam made from a picked line takes the line's place and carries it as its axis feature; one Undo brings the line back.
    #[test]
    fn a_beam_takes_its_axis_curve_and_undo_returns_it() {
        use crate::app::scene::FileDoc;
        use session_rust::{Session, Xform};
        use wood::Beam;
        use wood::geometry::profile_rectangle;

        let axis = Polyline::new(vec![
            Point::new(0.0, 0.0, 0.0),
            Point::new(3000.0, 0.0, 0.0),
        ]);
        let mut source = Session::new("curves");
        source.add_polyline(axis.clone(), None);
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "curves".into(),
            session: Rc::new(source),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        let line = (0..scene.row_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Polyline(_))))
            .unwrap();
        let picked = scene.identity_of(line).unwrap();
        let beam = Beam::from_profile(&axis, profile_rectangle(120.0, 200.0), "beam");
        let made = scene
            .create_with_attributes(items(&[beam]), "Element Beam", &[line])
            .unwrap();
        scene.sync();

        assert!(scene.row_of(picked.0, &picked.1).is_none(), "no loose line");
        let element = scene.row_of(made[0].0, &made[0].1).unwrap();
        let Some(Geometry::Element(beam)) = scene.geometry(element) else {
            panic!("a beam")
        };
        assert!(
            beam.features()
                .iter()
                .any(|f| f.feature_type == "axis" && f.outlines[0].point_count() == 2)
        );

        assert!(scene.undo(), "one step");
        scene.sync();
        assert!(
            scene.row_of(picked.0, &picked.1).is_some(),
            "the line is back"
        );
        assert!(
            scene.row_of(made[0].0, &made[0].1).is_none(),
            "the beam is gone"
        );
    }

    /// An element lands on the layer with its hidden base plane under `attributes`, one undo step for both.
    #[test]
    fn an_element_comes_with_its_hidden_base_plane() {
        let mut scene = Scene::new();
        let made = scene
            .create_with_attributes(
                items(&[Plate::from_outline(&square(600.0, 0.0), 40.0, "plate")]),
                "Element Plate",
                &[],
            )
            .unwrap();
        assert_eq!(made.len(), 1);
        scene.sync();
        let element = scene.row_of(made[0].0, &made[0].1).unwrap();
        let plane = (0..scene.row_count() as u32)
            .find(|&row| matches!(scene.geometry(row), Some(Geometry::Plane(_))))
            .unwrap();
        let (doc, guid) = scene.identity_of(plane).unwrap();

        assert!(
            matches!(scene.geometry(element), Some(Geometry::Element(e)) if e.element_type == "Plate")
        );
        assert_eq!(
            scene.parent_of(doc, &guid).unwrap().borrow().name,
            "attributes"
        );
        assert!(
            scene.hidden_rows().contains(&plane),
            "the base plane starts hidden"
        );
        assert!(crate::app::layers::under_element(&scene, plane) && !scene.selectable(plane));
        assert!(scene.undo());
        scene.sync();
        assert_eq!(
            scene.object_count(),
            0,
            "one undo takes the element and its plane"
        );
    }
}
