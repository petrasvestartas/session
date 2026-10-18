#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind { End, Vertex, Mid, Center, Perp, Near, Grid }

pub const END: u8 = 1;
pub const NEAR: u8 = 2;
pub const MID: u8 = 4;
pub const CENTER: u8 = 8;
pub const PERP: u8 = 16;
pub const MODES: [(&str, u8); 5] = [("End", END), ("Near", NEAR), ("Mid", MID), ("Center", CENTER), ("Perp", PERP)];

impl Kind {
    pub fn mode(self) -> u8 { match self { Self::End | Self::Vertex => END, Self::Near => NEAR, Self::Mid => MID, Self::Center => CENTER, Self::Perp => PERP, Self::Grid => 0 } }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hit { pub point: [f64; 3], pub kind: Kind, pub owner: Option<crate::scene::ObjectId> }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings { pub objects: bool, pub modes: u8, pub grid: bool, pub step: f64 }

impl Default for Settings {
    fn default() -> Self { Self { objects: true, modes: END | NEAR | MID | CENTER, grid: true, step: 1000. } }
}

#[derive(Clone, Copy)]
pub enum Setting { Objects(Option<bool>), Mode(u8), Grid(Option<bool>), Spacing(f64) }

impl Settings {
    pub fn apply(&mut self, setting: Setting) -> Result<String, &'static str> {
        Ok(match setting {
            Setting::Objects(on) => { self.objects = on.unwrap_or(!self.objects); format!("Snap {}", if self.objects { "On" } else { "Off" }) }
            Setting::Grid(on) => { self.grid = on.unwrap_or(!self.grid); self.grid_status() }
            Setting::Spacing(step) => {
                if !step.is_finite() || step <= 0. { return Err("The grid spacing must be above zero · Example: Grid Snap 50"); }
                self.step = step; self.grid = true; self.grid_status()
            }
            Setting::Mode(bit) => {
                let (name, _) = MODES.iter().find(|(_, value)| *value == bit).ok_or("Unknown snap mode")?;
                self.modes ^= bit; format!("Snap {name} {}", if self.modes & bit != 0 { "On" } else { "Off" })
            }
        })
    }
    fn grid_status(&self) -> String { if self.grid { format!("Grid Snap On · {} mm", self.step) } else { "Grid Snap Off".into() } }
}
