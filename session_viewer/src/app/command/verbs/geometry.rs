use crate::State;
use crate::app::command::Action;
use crate::app::modeling::Modeling;

/// Geometry created or edited by one of the modeling verbs.
#[derive(Debug)]
pub struct Model(pub Modeling);

impl Action for Model {
    /// Build or edit the geometry, then select what was created.
    fn run(&self, state: &mut State) -> Result<String, String> {
        // a new object, as opposed to an edit
        let created = matches!(
            self.0,
            Modeling::Point(_) | Modeling::Line(..) | Modeling::Polyline(_) | Modeling::Curve(_)
        );
        state.scene.model(&self.0)?;
        state.after_history();

        if !created {
            return Ok("geometry updated".into());
        }

        // select the new object: the last row of its document
        if let Some(doc) = state.scene.created_doc {
            let row = (0..state.gpu.objects.len())
                .rev()
                .find(|&row| state.scene.identity_of(row).is_some_and(|id| id.0 == doc));
            state.select(row);
        }

        let name = match self.0 {
            Modeling::Point(_) => "point",
            Modeling::Line(..) => "line",
            Modeling::Curve(_) => "NURBS curve",
            _ => "polyline",
        };
        Ok(format!(
            "Created and selected {name}. Type Fit to locate it; Undo to remove it."
        ))
    }
}
