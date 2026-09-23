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

        // select the new object, on the layer it went to
        let row = state.scene.row_of(doc, &guid);
        state.select(row);
        let layer = row
            .and_then(|row| state.scene.node_of(row))
            .and_then(|(node, _)| node.borrow().parent())
            .map(|parent| parent.borrow().name.clone())
            .unwrap_or_default();
        let name = match self.0 {
            Modeling::Point(_) => "point",
            Modeling::Line(..) => "line",
            Modeling::Curve(_) => "NURBS curve",
            _ => "polyline",
        };
        Ok(format!(
            "Created and selected {name} on {layer}. Type Fit to locate it; Undo to remove it."
        ))
    }
}
