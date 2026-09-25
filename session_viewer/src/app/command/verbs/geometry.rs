use crate::State;
use crate::app::command::Action;
use crate::app::modeling::Modeling;

/// Geometry created or edited by one of the modeling verbs.
#[derive(Debug)]
pub struct Model(pub Modeling);

impl Action for Model {
    /// Build or edit the geometry, then select what was created.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let made = state.scene.model(&self.0)?;
        state.after_history();

        // an edit, as opposed to a new object
        let Some((doc, guid)) = made else {
            return Ok("geometry updated".into());
        };

        let name = match self.0 {
            Modeling::Point(_) => "point",
            Modeling::Line(..) => "line",
            Modeling::Curve(_) => "NURBS curve",
            _ => "polyline",
        };
        Ok(created(state, doc, &guid, name))
    }
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

/// Select the new object, on the layer it went to, and say so.
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
