// --8<-- [start:draw-verb]
use crate::State;
use crate::app::command::{Action, Spec, Verb};
use crate::app::coords;
use crate::app::modeling::{Interval, MAX_POINTS};
use session_rust::{Geometry, Point};
use std::ops::RangeInclusive;

/// A verb that makes one geometry from points, clicked or typed: Point, Line, Polyline and Curve are all a Draw.
pub struct Draw {
    pub spec: Spec,                                       // how it is typed
    pub points: RangeInclusive<usize>, // `1..=1` is exactly one point and finishes by itself; `2..=4096` takes two or more until Enter
    pub what: &'static str,            // named in the answer, e.g. `NURBS curve`
    pub buttons: &'static [(&'static str, &'static str)], // under the command line while drawing
    pub build: fn(&[Point]) -> Result<Geometry, String>, // the verb's own function: checked points in, geometry out
}

impl Verb for Draw {
    fn spec(&self) -> &Spec {
        &self.spec
    }

    /// Every entry is asked for a Draw; only this impl answers Some, which is how the drawing code finds its verbs.
    fn draw(&self) -> Option<&Draw> {
        Some(self)
    }
}

impl Draw {
    /// True when Enter finishes it, e.g. Polyline.
    pub fn open(&self) -> bool {
        self.points.start() != self.points.end()
    }

    /// The typed points as a creation. `&'static self`: the Draw lives in a const, so the Create below may keep a pointer to it.
    pub fn parse(&'static self, words: &[&str]) -> Result<Box<dyn Action>, String> {
        if words.len() > MAX_POINTS {
            return Err("too many points".into());
        }

        let mut points = Vec::new();

        for word in words {
            let Some(coords::Typed::Absolute { x, y, z }) = coords::parse(word) else {
                return Err("use world coordinates x,y,z separated by spaces".into());
            };
            points.push([x, y, z.unwrap_or(0.0)]);
        }

        if !self.points.contains(&points.len()) {
            return Err(self.count());
        }

        Ok(Box::new(Create { draw: self, points }))
    }

    /// The geometry these points make, after checking them.
    pub fn geometry(&self, points: &[[f64; 3]]) -> Result<Geometry, String> {
        if !self.points.contains(&points.len()) {
            return Err(self.count());
        }

        let points = points
            .iter()
            .map(|p| point(*p))
            .collect::<Result<Vec<_>, _>>()?;
        (self.build)(&points)
    }

    /// How many points it needs, as a message.
    fn count(&self) -> String {
        let name = self.spec.names[0];

        match (*self.points.start(), self.open()) {
            (1, false) => format!("{name} needs one point"),
            (count, false) => format!("{name} needs {count} points"),
            (count, true) => format!("{name} needs at least {count} points"),
        }
    }
}

/// A point from finite, reasonable coordinates.
fn point(p: [f64; 3]) -> Result<Point, String> {
    if p.iter().any(|v| !v.is_finite() || v.abs() > 1e12) {
        return Err("coordinates must be finite and within ±1e12".into());
    }

    Ok(Point::new(p[0], p[1], p[2]))
}
// --8<-- [end:draw-verb]

// --8<-- [start:draw-create]
/// One geometry from a drawing verb and its points.
pub struct Create {
    draw: &'static Draw,   // the verb
    points: Vec<[f64; 3]>, // its checked count of world points
}

/// Debug written by hand, printing `Create(Line, [[0.0, 0.0, 0.0], ..])` with the verb's shown name.
impl std::fmt::Debug for Create {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Create({}, {:?})", self.draw.spec.names[0], self.points)
    }
}

impl Action for Create {
    /// Build the geometry, then select it.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let (doc, guid) = state.scene.model(self.draw, &self.points)?;
        state.after_history();
        Ok(created(state, doc, &guid, self.draw.what))
    }
}

/// The selected curve trimmed or extended over a part of its length.
#[derive(Debug)]
pub struct Edit(pub Interval); // a tuple struct: one unnamed field, read as `self.0`

impl Action for Edit {
    /// Replace the curve as one undo step.
    fn run(&self, state: &mut State) -> Result<String, String> {
        state.scene.edit_interval(self.0)?;
        state.after_history();
        Ok("geometry updated".into())
    }
}

/// Two numbers `a b` for Trim or Extend.
pub fn range(words: &[&str]) -> Result<(f64, f64), String> {
    let [a, b] = words else {
        return Err("try Trim 0.2 0.8 or Extend -0.2 1.2".into());
    };
    let a = crate::app::command::number(Some(a), "Trim 0.2 0.8")?;
    let b = crate::app::command::number(Some(b), "Trim 0.2 0.8")?;
    Ok((a, b))
}

/// Add `geometry` to the current layer as one undo step and select it; `what` names it.
pub(crate) fn create(
    state: &mut State,
    geometry: session_rust::Geometry,
    what: &str,
) -> Result<String, String> {
    let (doc, guid) = state.scene.create_geometry(geometry)?;
    state.after_history();
    Ok(created(state, doc, &guid, what))
}

/// Add `geometries` to the current layer as one undo step, select them and say `message`.
pub(crate) fn create_all(
    state: &mut State,
    geometries: Vec<session_rust::Geometry>,
    message: &str,
) -> Result<String, String> {
    let made = state.scene.create_many(geometries, message)?;
    state.after_history();
    let rows = made
        .iter()
        .filter_map(|(doc, guid)| state.scene.row_of(*doc, guid))
        .collect();
    state.select_rows(rows, false);
    Ok(format!("{message} · Undo removes it"))
}

/// Select the new object and name the layer it went to, read from its node's parent in the document tree.
fn created(state: &mut State, doc: usize, guid: &str, what: &str) -> String {
    let row = state.scene.row_of(doc, guid);
    state.select(row);
    let layer = row
        .and_then(|row| state.scene.node_of(row))
        .and_then(|(node, _)| node.borrow().parent())
        .map(|parent| parent.borrow().name.clone())
        .unwrap_or_default();
    format!("Created and selected {what} on {layer}. Type Fit to locate it; Undo to remove it.")
}
// --8<-- [end:draw-create]

// --8<-- [start:draw-tests]
#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::app::command::{Spec, completions, drawing, parse};
    use crate::app::scene::Scene;
    use session_rust::Polyline;
    use std::rc::Rc;

    /// A drawing verb that exists only here and in one `#[cfg(test)]` registry line.
    pub const SPEC: Draw = Draw {
        spec: Spec {
            names: &["Wedge"],
            aliases: &[],
            hint: "Wedge · three corners",
            options: &[],
            arity: None,
            wait_for_option: false,
            wait_after_option: false,
            parse: |_, rest| Draw::parse(&SPEC, rest),
        },
        points: 3..=3,
        what: "wedge",
        buttons: &[("Cancel", "Escape")],
        build: |points| {
            let mut corners = points.to_vec();
            corners.push(points[0].clone());
            Ok(Geometry::Polyline(Rc::new(Polyline::new(corners))))
        },
    };

    /// A new drawing verb is found, parsed, drafted and built from its own file alone.
    #[test]
    fn a_drawing_verb_registers_in_one_file() {
        assert!(completions("we").contains(&"Wedge"));
        let (draw, count) = drawing(&["wedge", "0,0,0"]).unwrap();
        assert_eq!(
            (draw.spec.names[0], count, draw.open()),
            ("Wedge", 1, false)
        );
        assert_eq!(
            format!("{:?}", parse("wedge 0,0,0 1,0,0 0,1,0").unwrap()),
            "Create(Wedge, [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])"
        );
        assert_eq!(
            parse("wedge 0,0,0 1,0,0").unwrap_err(),
            "Wedge needs 3 points"
        );
        let mut scene = Scene::new();
        scene
            .model(&SPEC, &[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]])
            .unwrap();
        assert_eq!(scene.docs[0].session.lookup.len(), 1);
        assert!(scene.model(&SPEC, &[[0.0, 0.0, 0.0]]).is_err());
    }
}
// --8<-- [end:draw-tests]
