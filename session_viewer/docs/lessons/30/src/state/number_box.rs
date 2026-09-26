// --8<-- [start:number-prompt]
use super::State;
use crate::app::gizmo::{Handle, typed_value};

// The number box is the text field a clicked handle opens: type 250 on Move X to move the selection 250 mm.
/// The number box of a clicked gumball handle, as the panel draws it.
pub struct NumberPrompt {
    pub handle: Handle,     // the handle clicked
    pub title: String,      // e.g. Move X
    pub unit: &'static str, // mm, deg or factor
    pub hint: &'static str, // the value that changes nothing
    pub at: [f32; 2],       // the handle on screen, CSS pixels
}

impl State {
    /// True while a number box waits for its value.
    pub fn number_box_open(&self) -> bool {
        self.features
            .gizmo
            .as_ref()
            .is_some_and(|gizmo| gizmo.typing.is_some())
    }

    /// The open number box, pinned to its handle wherever the view puts it.
    pub fn number_prompt(&self) -> Option<NumberPrompt> {
        let gizmo = self.features.gizmo.as_ref()?;
        let handle = gizmo.typing?;
        let point = gizmo.handle_point(handle, self.world_per_px());
        let (x, y) = self.project([point[0], point[1], point[2]])?;
        let scale = self.pixel_scale(); // device pixels per CSS pixel
        let (_, _, unit) = handle.labels();
        Some(NumberPrompt {
            handle,
            title: handle.title(),
            unit,
            hint: if unit == "factor" { "1" } else { "0" },
            at: [(x / scale) as f32, (y / scale) as f32],
        })
    }
// --8<-- [end:number-prompt]

// --8<-- [start:number-typed]
    /// Close the number box; true when one was open.
    pub fn close_number_box(&mut self) -> bool {
        let Some(gizmo) = self
            .features
            .gizmo
            .as_mut()
            .filter(|gizmo| gizmo.typing.is_some())
        else {
            return false;
        };
        gizmo.typing = None;
        self.upload_gizmo(); // register:gumball
        self.touch();
        true
    }

    /// Enter in the number box: the value becomes one undo step; `Ok(None)` when nothing changed.
    pub fn type_number(&mut self, text: &str) -> Result<Option<String>, String> {
        let Some(gizmo) = self.features.gizmo.as_ref() else {
            return Ok(None);
        };
        let Some(handle) = gizmo.typing else {
            return Ok(None);
        };
        // `?` hands a bad number back as Err, which keeps the box open; Ok(None) means nothing to change
        let Some(value) = typed_value(handle, text)? else {
            self.close_number_box();
            return Ok(None);
        };
        let delta = gizmo.typed(handle, value);
        let label = match handle {
            Handle::Translate(_) => "move",
            Handle::Rotate(_) => "rotate",
            Handle::Scale(_) | Handle::ScaleUniform => "scale",
        };
        self.close_number_box();

        if let Err(error) = self.apply(delta, label) {
            self.status(&error);
            return Ok(None);
        }

        let (_, _, unit) = handle.labels();
        Ok(Some(format!("{} {value} {unit}", handle.title())))
    }
}
// --8<-- [end:number-typed]

// --8<-- [start:number-tap]
impl State {
    /// A tap that opened a number box raises the phone keyboard.
    pub(crate) fn number_box_tapped(&self, tap: bool) {
        if tap && self.number_box_open() {
            crate::app::feedback::raise_keyboard(); // register:phone
        }
    }
}
// --8<-- [end:number-tap]
