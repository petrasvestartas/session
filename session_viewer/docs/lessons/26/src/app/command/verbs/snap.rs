// --8<-- [start:snap-spec]
use crate::State;
use crate::app::command::{Action, Spec, on_off};
use crate::app::snap;

pub const SPEC: Spec = Spec {
    names: &["Snap"],
    aliases: &[],
    hint: "Snap (On Off): snapping and its toolbar · Snap End / Near / Mid / Center / Perp toggles one kind",
    options: &[
        "Snap On",
        "Snap Off",
        "Snap End",
        "Snap Near",
        "Snap Mid",
        "Snap Center",
        "Snap Perp",
    ],
    arity: None,
    wait_for_option: true,
    wait_after_option: false,
    parse,
};

/// Turn snapping on or off, or toggle one snap kind.
fn parse(_verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let usage = "Snap (On Off End Near Mid Center Perp)";

    if let [word] = rest
        && let Some(mode) = snap::mode(word)
    {
        return Ok(Box::new(Snap { on: None, mode }));
    }

    Ok(Box::new(Snap {
        on: on_off(rest, usage)?,
        mode: 0,
    }))
}
// --8<-- [end:snap-spec]

// --8<-- [start:snap-action]
#[derive(Debug)]
struct Snap {
    on: Option<bool>, // snapping and its toolbar, None flips
    mode: u8,         // one kind to toggle, 0 for none
}

impl Action for Snap {
    /// Flip the flags the drawing code and the toolbar read.
    fn run(&self, state: &mut State) -> Result<String, String> {
        if self.mode != 0 {
            state.features.snap.modes ^= self.mode; // `^=` flips one bit: on becomes off, off becomes on
            let (label, _) = snap::MODES
                .iter()
                .find(|(_, bit)| *bit == self.mode)
                .unwrap(); // every mode came from MODES, so it is found
            let on = state.features.snap.modes & self.mode != 0;
            return Ok(format!("Snap {label} {}", if on { "On" } else { "Off" }));
        }

        state.features.snap.enabled = self.on.unwrap_or(!state.features.snap.enabled);
        state.features.snap.bar = state.features.snap.enabled;
        Ok(format!(
            "Snap {}",
            if state.features.snap.enabled {
                "On"
            } else {
                "Off"
            }
        ))
    }

    /// Snapping is changed mid-draw, so the draft stays.
    fn keeps_draft(&self) -> bool {
        true
    }
}
// --8<-- [end:snap-action]
