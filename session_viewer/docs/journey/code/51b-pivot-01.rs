    }
}

#[derive(Clone, Debug)]
pub struct Drag {
    pub handle: Handle,
    grabbed: Point,
    angle: f64,
    reach: f64,
    plane: Vector,
}

pub struct Gizmo {
    pub origin: Point,
    pub hovered: Option<Handle>,
    pub drag: Option<Drag>,
    pub typing: Option<Handle>,
}

impl Gizmo {
    pub fn new(origin: Point) -> Self {
        Self {
            origin,
            hovered: None,
            drag: None,
            typing: None,
        }
    }

    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
        self.hovered = None;
        self.drag = None;
        self.typing = None;
    }

    pub fn handle_point(&self, handle: Handle, world_per_px: f64) -> Point {
        let s = world_per_px;

        match handle {
            Handle::Translate(axis) => &self.origin + &(&axis.unit() * (ARROW_AT * s)),
            Handle::Scale(axis) => &self.origin + &(&axis.unit() * (BALL_AT * s)),
            Handle::ScaleUniform => self.origin.clone(),
            Handle::Rotate(axis) => {
                let (u, v) = axis.others();
                &self.origin - &(&(&u + &v) * (ARM * s / 2.0_f64.sqrt()))
            }
        }
    }

}

fn typed_factor(value: f64) -> f64 {
    if value <= 0.0 {
        return MIN_SCALE;
