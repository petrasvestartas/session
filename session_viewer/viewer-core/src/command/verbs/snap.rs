use crate::app::command::{Action, CommandHost, Spec, on_off};
use crate::app::snap;

pub const SPEC: Spec = Spec {
    options: &[
        "Snap On",
        "Snap Off",
        "Snap End",
        "Snap Near",
        "Snap Mid",
        "Snap Center",
        "Snap Perp",
    ],
    wait_for_option: true,
    ..Spec::new(
        &["Snap"],
        "Snap (On Off): snapping · Snap End / Near / Mid / Center / Perp toggles one kind",
        parse,
    )
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

#[derive(Debug)]
struct Snap {
    on: Option<bool>, // snapping and its toolbar, None flips
    mode: u8,         // one kind to toggle, 0 for none
}

impl Action for Snap {
    /// Flip the flags the drawing code and the toolbar read.
    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        if self.mode != 0 {
            let snap = state.snap_mut();
            snap.modes ^= self.mode;
            let (label, _) = snap::MODES
                .iter()
                .find(|(_, bit)| *bit == self.mode)
                .unwrap();
            let on = snap.modes & self.mode != 0;
            return Ok(format!("Snap {label} {}", if on { "On" } else { "Off" }));
        }

        let snap = state.snap_mut();
        snap.enabled = self.on.unwrap_or(!snap.enabled);
        snap.bar = snap.enabled;
        Ok(format!("Snap {}", if snap.enabled { "On" } else { "Off" }))
    }

    /// Snapping is changed mid-draw, so the draft stays.
    fn keeps_draft(&self) -> bool {
        true
    }
}
