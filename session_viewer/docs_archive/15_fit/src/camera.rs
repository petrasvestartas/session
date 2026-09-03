use session_rust::{Point, Vector, Xform};

#[derive(Clone, Copy)]
pub enum View {Front, Back, Left, Right, Top, Bottom, Iso}

pub struct Camera{
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub target: [f32; 3],
    pub perspective: bool,
}

impl Camera {
    
    pub fn new() -> Self {
        Self { yaw: 0.6, pitch: 0.5, distance: 3.0, target: [0.0, 0.0, 0.0], perspective: true}
    }

    pub fn orbit(&mut self, dx: f32, dy: f32){
        self.yaw -= dx * 0.005;
        self.pitch = (self.pitch - dy * 0.005).clamp(-1.5, 1.5);
    }

    pub fn pan(&mut self, dx: f32, dy: f32){
        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(), self.yaw.sin());
        let right = [cy, 0.0, -sy];
        let up = [-sp*sy, cp, -sp*cy];
        let k = self.distance*0.0015;
        for i in 0..3{
            self.target[i] += (-dx * right[i] + dy * up[i]) * k;
        }
    }

    pub fn zoom(&mut self, amount: f32){
        self.distance = (self.distance*(1.0-amount*0.1)).clamp(0.2, 100.0);
    }

    pub fn toggle_projection(&mut self){
        self.perspective = !self.perspective;
    }

    pub fn view_proj(&self, aspect: f64) -> Xform {
        let projection = if self.perspective {
            Xform::perspective(f64::to_radians(60.0), aspect, 0.1, 100.0)
        } else {
            let h = 1.0;
            Xform::orthographic(-aspect*h, aspect*h, -h, h, 0.1, 100.0)
        };

        let (cp, sp) = (self.pitch.cos(), self.pitch.sin());
        let (cy, sy) = (self.yaw.cos(), self.yaw.sin());
        let eye = Point::new(
            self.target[0] as f64 + (self.distance * cp * sy) as f64,
            self.target[1] as f64 + (self.distance * sp) as f64,
            self.target[2] as f64 + (self.distance * cp * cy) as f64,
        );
        let target = Point::new(self.target[0] as f64, self.target[1] as f64, self.target[2] as f64);
        let up = Vector::new(0.0, 1.0, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        projection*view
    }

    pub fn set_view(&mut self, view: View){
        use std::f32::consts::{PI, FRAC_PI_2, FRAC_PI_4};
        let top = FRAC_PI_2 - 0.001;
        let (yaw, pitch) = match view {
            View::Front => (0.0, 0.0),
            View::Back => (PI, 0.0),
            View::Left => (-FRAC_PI_2, 0.0),
            View::Right => (FRAC_PI_2, 0.0),
            View::Top => (0.0, top),
            View::Bottom => (0.0, -top),
            View::Iso => (FRAC_PI_4, 0.6155), // classic 35.26° isometric
        };
        self.yaw = yaw;
        self.pitch = pitch;
        self.perspective = false;
    }

    pub fn reset(&mut self){
        *self = Camera::new();
    }

    pub fn fit(&mut self, min: [f32; 3], max: [f32; 3], aspect: f64){

        // target + box center
        self.target = [
            (min[0]+max[0])*0.5,
            (min[1]+max[1])*0.5,
            (min[2]+max[2])*0.5,
        ];

        // bounding-box-sphere radius = half  the diagonal
        let dx = (max[0]-min[0])*0.5;
        let dy = (max[1]-min[1])*0.5;
        let dz = (max[2]-min[2])*0.5;
        let radius = (dx*dx + dy*dy + dz*dz).sqrt();
        if radius <= 0.0 { return; }

        // limiting half-FOV 60 deg vertical, narrower horizontal on a tall convas
        let half_fov_y = f64::to_radians(60.0)*0.5;
        let half_fov_x = (aspect*half_fov_y.tan()).atan();
        let half_fov = half_fov_y.min(half_fov_x) as f32;

        // distance so the shere fills that half-angle, plus 10% room
        self.distance = (radius / half_fov.sin() * 1.1).clamp(0.2, 100.0);
    }

}