use crate::primitive_options as options;
use session_rust::{BRep,Geometry,Mesh,Plane,Point,Polyline,Vector,Xform};
use std::{collections::HashMap,rc::Rc,sync::OnceLock};

pub const MESH_QUALITY: (f64, f64) = (10.0, 0.002); // tessellation of a Mesh option: degrees, chord factor
pub const RING: usize = 64; // preview samples of a full circle

pub const BREP_MESH: &[(&str, &str)] = &[("Brep", "Brep"), ("Mesh", "Mesh"), ("Cancel", "Escape")];
pub const MESH_ONLY: &[(&str, &str)] = &[("Mesh", "Mesh"), ("Cancel", "Escape")];
pub const CURVE: &[(&str, &str)] = &[("Cancel", "Escape")];

/// One answer to a shape's question.
#[derive(Clone, Debug)]
pub enum Answer {
    Point(Point), // clicked or typed coordinates
    Number(f64),  // a typed value
}

/// How a question reads a bare number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Point,  // a distance along the rubber band
    Size,   // the value itself
    Height, // the value itself; clicks follow the normal through the first point
}

/// One question a shape asks.
#[derive(Clone, Copy, Debug)]
pub struct Ask {
    pub prompt: &'static str, // what the answer is for
    pub kind: Kind,           // how a number and a click count
    pub default: Option<f64>, // what Enter takes
}

impl Ask {
    /// A point question.
    pub fn point(prompt: &'static str) -> Self {
        Self {
            prompt,
            kind: Kind::Point,
            default: None,
        }
    }

    /// A click or a number.
    pub fn size(prompt: &'static str) -> Self {
        Self {
            prompt,
            kind: Kind::Size,
            default: None,
        }
    }

    /// A click along the normal or a number.
    pub fn height(prompt: &'static str) -> Self {
        Self {
            prompt,
            kind: Kind::Height,
            default: None,
        }
    }

    /// Enter takes `value`.
    pub fn or(self, value: f64) -> Self {
        Self {
            default: Some(value),
            ..self
        }
    }
}

/// A right-handed frame: in-plane x and y, normal z.
#[derive(Clone, Debug)]
pub struct Frame {
    pub origin: Point, // the first point answered
    pub x: Vector,     // in the plane
    pub y: Vector,     // in the plane
    pub z: Vector,     // the plane normal
}

impl Frame {
    /// The plane's axes at `origin`; z = x × y.
    pub fn new(plane: &Plane, origin: Point) -> Self {
        let (x, y) = (plane.x_axis(), plane.y_axis());
        let z = x.cross(&y);
        Self { origin, x, y, z }
    }

    /// The point at local coordinates.
    pub fn at(&self, u: f64, v: f64, w: f64) -> Point {
        &self.origin + (&self.x * u + &self.y * v + &self.z * w)
    }

    /// Local coordinates of `p`.
    pub fn local(&self, p: &Point) -> [f64; 3] {
        let d = p - &self.origin;
        [d.dot(&self.x), d.dot(&self.y), d.dot(&self.z)]
    }

    /// Turned about z so x points along in-plane (u, v); unchanged for a zero direction.
    pub fn turned(&self, u: f64, v: f64) -> Self {
        let length = u.hypot(v);

        if length <= 1e-300 {
            return self.clone();
        }

        let x = &self.x * (u / length) + &self.y * (v / length);
        let y = self.z.cross(&x);
        Self {
            origin: self.origin.clone(),
            x,
            y,
            z: self.z.clone(),
        }
    }

    /// Upside down: y and z reversed, still a rotation.
    pub fn flipped(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            x: self.x.clone(),
            y: -&self.y,
            z: -&self.z,
        }
    }

    /// The frame and height with the height made positive.
    pub fn upright(&self, height: f64) -> (Self, f64) {
        if height < 0.0 {
            (self.flipped(), -height)
        } else {
            (self.clone(), height)
        }
    }

    /// The axes turned one step: x, y, z become y, z, x.
    pub fn rolled(&self) -> Self {
        Self {
            origin: self.origin.clone(),
            x: self.y.clone(),
            y: self.z.clone(),
            z: self.x.clone(),
        }
    }

    /// Local to world.
    pub fn to_xform(&self) -> Xform {
        Xform::frame_to_world(&self.origin, &self.x, &self.y, &self.z)
    }

    /// A closed ellipse of radii `rx`, `ry` at height `w`.
    pub fn oval(&self, rx: f64, ry: f64, w: f64) -> Vec<Point> {
        (0..=RING)
            .map(|i| {
                let angle = std::f64::consts::TAU * i as f64 / RING as f64;
                self.at(rx * angle.cos(), ry * angle.sin(), w)
            })
            .collect()
    }

    /// A closed circle at height `w`.
    pub fn ring(&self, radius: f64, w: f64) -> Vec<Point> {
        self.oval(radius, radius, w)
    }

    /// A closed rectangle centered on the axis at height `w`.
    pub fn rectangle(&self, sx: f64, sy: f64, w: f64) -> Vec<Point> {
        let corner = self.at(-sx * 0.5, -sy * 0.5, w);
        Polyline::rectangle(&corner, &self.x, &self.y, sx, sy, true).get_points()
    }

    /// A box standing on the plane, `h` along the normal: two rectangles and four edges.
    pub fn cuboid(&self, sx: f64, sy: f64, h: f64) -> Vec<Vec<Point>> {
        let mut wires = vec![self.rectangle(sx, sy, 0.0), self.rectangle(sx, sy, h)];

        for (u, v) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            wires.push(vec![
                self.at(u * sx * 0.5, v * sy * 0.5, 0.0),
                self.at(u * sx * 0.5, v * sy * 0.5, h),
            ]);
        }

        wires
    }

    /// A number, or a click's distance from the origin in the plane.
    pub fn size(&self, answer: &Answer) -> f64 {
        match answer {
            Answer::Number(value) => *value,
            Answer::Point(p) => {
                let [u, v, _] = self.local(p);
                u.hypot(v)
            }
        }
    }

    /// A number, or a click's distance from the origin.
    pub fn distance(&self, answer: &Answer) -> f64 {
        match answer {
            Answer::Number(value) => *value,
            Answer::Point(p) => p.distance(&self.origin, None),
        }
    }

    /// A number, or a click's height along the normal.
    pub fn height(&self, answer: &Answer) -> f64 {
        match answer {
            Answer::Number(value) => *value,
            Answer::Point(p) => self.local(p)[2],
        }
    }
}

/// What the answers say so far: the placed frame and the sizes known.
#[derive(Clone, Debug)]
pub struct Part {
    pub frame: Frame,    // where the shape sits
    pub sizes: Vec<f64>, // its dimensions in answer order
}

impl Part {
    /// No sizes yet.
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            sizes: Vec::new(),
        }
    }
}

/// A creation command: its questions, preview and kernel object.
pub struct Shape {
    pub name: &'static str,                               // shown name, e.g. `Box`
    pub options: &'static [(&'static str, &'static str)], // buttons: (label, line)
    pub upfront: bool, // the option changes the questions: chosen before any answer
    pub ask: fn(&[Answer], &Part, &str) -> Option<Ask>, // the next question, None when all are answered
    pub read: fn(&Frame, &[Answer], &str) -> Result<Part, String>, // what the answers say; an error refuses the last one
    pub outline: fn(&Part) -> Vec<Vec<Point>>,                     // preview wires, cheap
    pub build: fn(&Part, &str) -> Result<Geometry, String>, // the object for the chosen option
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Shape {
    /// The option labels a word can pick, without Cancel.
    pub(crate) fn choices(&self) -> impl Iterator<Item = &'static str> {
        options::choices(self.options)
    }

    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub(crate) fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.options, words)
    }
}

/// A positive, finite size, or a message naming it.
pub fn positive(value: f64, what: &str) -> Result<f64, String> {
    if !(value.is_finite() && value > 1e-9 && value <= 1e9) {
        return Err(format!("{what} must be above zero"));
    }

    Ok(value)
}

/// A nonzero, finite height, or a message.
pub fn nonzero(value: f64, what: &str) -> Result<f64, String> {
    if !(value.is_finite() && value.abs() > 1e-9 && value.abs() <= 1e9) {
        return Err(format!("{what} must not be zero"));
    }

    Ok(value)
}

/// The frame at the first answered point, else at the plane's origin.
pub fn frame_of(plane: &Plane, answers: &[Answer]) -> Frame {
    let origin = match answers.first() {
        Some(Answer::Point(p)) => p.clone(),
        _ => plane.origin(),
    };
    Frame::new(plane, origin)
}

/// Center, then radius: the questions of a sphere-like shape.
pub fn center_radius(answers: &[Answer], _part: &Part, _option: &str) -> Option<Ask> {
    match answers.len() {
        0 => Some(Ask::point("Center")),
        1 => Some(Ask::size("Radius")),
        _ => None,
    }
}

/// The radius as the distance from the center.
pub fn read_radius(frame: &Frame, answers: &[Answer], _option: &str) -> Result<Part, String> {
    let mut part = Part::new(frame.clone());

    if let Some(radius) = answers.get(1) {
        part.sizes
            .push(positive(frame.distance(radius), "The radius")?);
    }

    Ok(part)
}

/// Three great circles.
pub fn sphere_outline(part: &Part) -> Vec<Vec<Point>> {
    let [radius] = part.sizes[..] else {
        return Vec::new();
    };
    let side = part.frame.rolled();
    vec![
        part.frame.ring(radius, 0.0),
        side.ring(radius, 0.0),
        side.rolled().ring(radius, 0.0),
    ]
}

/// One mesh of every part's faces; corners within a billionth of the size share a vertex.
pub fn to_mesh(parts: &[Mesh]) -> Mesh {
    let polygons: Vec<Vec<Point>> = parts
        .iter()
        .flat_map(|part| part.face_outlines())
        .map(|outline| {
            let mut points = outline.get_points();
            points.pop(); // the closing point
            points
        })
        .collect();
    let size = polygons.iter().flatten().fold(0.0_f64, |size, p| {
        size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs())
    });
    let step = (size * 1e-9).max(1e-12);
    let mut cells: HashMap<[i64; 3], Point> = HashMap::new();
    let welded = polygons
        .into_iter()
        .map(|polygon| {
            polygon
                .into_iter()
                .map(|p| weld(&mut cells, p, step))
                .collect()
        })
        .collect();
    Mesh::from_polylines(welded, None)
}

/// The point already stored within `step` of `p` in this or a neighbouring cell, else `p`.
fn weld(cells: &mut HashMap<[i64; 3], Point>, p: Point, step: f64) -> Point {
    let key = [0, 1, 2].map(|i| (p[i] / step).round() as i64);

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if let Some(q) = cells.get(&[key[0] + dx, key[1] + dy, key[2] + dz])
                    && q.distance(&p, None) <= step
                {
                    return q.clone();
                }
            }
        }
    }

    cells.insert(key, p.clone());
    p
}

/// The BRep placed by `place`, or its tessellation for the Mesh option.
pub fn solid(mut brep: BRep, option: &str, place: &Xform, name: &str) -> Geometry {
    brep.transform(place);

    if option == "Mesh" {
        let mut mesh = to_mesh(&brep.face_meshes_q(Some(MESH_QUALITY)));
        mesh.name = name.into();
        return Geometry::Mesh(Rc::new(mesh));
    }

    brep.name = name.into();
    Geometry::BRep(Rc::new(brep))
}

/// The mesh placed by `place`, or a BRep of its planar faces for the Brep option.
pub fn polyhedron(mut mesh: Mesh, option: &str, place: &Xform, name: &str) -> Geometry {
    mesh.transform(place);

    if option == "Brep" {
        let mut brep = BRep::from_polylines(&mesh.face_outlines(), &[]);
        brep.name = name.into();
        return Geometry::BRep(Rc::new(brep));
    }

    mesh.name = name.into();
    Geometry::Mesh(Rc::new(mesh))
}

/// The largest distance of a vertex from the origin.
pub fn circumradius(mesh: &Mesh) -> f64 {
    mesh.vertex
        .values()
        .map(|vertex| vertex.position().distance(&Point::new(0.0, 0.0, 0.0), None))
        .fold(0.0, f64::max)
}

/// A polyhedron of circumradius 1 as face outlines, built on first use.
pub fn unit_faces(
    cache: &'static OnceLock<Vec<Vec<[f64; 3]>>>,
    make: fn(f64) -> Mesh,
) -> &'static [Vec<[f64; 3]>] {
    cache.get_or_init(|| {
        let mesh = make(1.0);
        let scale = 1.0 / circumradius(&mesh);
        mesh.face_outlines()
            .iter()
            .map(|outline| {
                outline
                    .get_points()
                    .iter()
                    .map(|p| [p[0] * scale, p[1] * scale, p[2] * scale])
                    .collect()
            })
            .collect()
    })
}

/// The unit faces at the part's radius.
pub fn polyhedron_outline(part: &Part, faces: &[Vec<[f64; 3]>]) -> Vec<Vec<Point>> {
    let [radius] = part.sizes[..] else {
        return Vec::new();
    };
    faces
        .iter()
        .map(|face| {
            face.iter()
                .map(|q| part.frame.at(q[0] * radius, q[1] * radius, q[2] * radius))
                .collect()
        })
        .collect()
}

/// The polyhedron `make` with its corners at the part's radius.
pub fn polyhedron_of(
    part: &Part,
    option: &str,
    make: fn(f64) -> Mesh,
    name: &str,
) -> Result<Geometry, String> {
    let [radius] = part.sizes[..] else {
        return Err(format!("The {name} needs a radius"));
    };

    // the Brep's faces join within 1e-6
    if option == "Brep" && radius < 1e-3 {
        return Err("The radius must be at least 0.001 for a Brep".into());
    }

    let mesh = make(radius / circumradius(&make(1.0)));
    Ok(polyhedron(mesh, option, &part.frame.to_xform(), name))
}


#[cfg(test)]
pub mod tests {
    use super::*;

    /// A point from coordinates.
    pub fn p(x: f64, y: f64, z: f64) -> Point {
        Point::new(x, y, z)
    }

    /// A drawing plane through the origin with axes `x` and `y`.
    pub fn plane(x: [f64; 3], y: [f64; 3]) -> Plane {
        Plane::new(
            p(0.0, 0.0, 0.0),
            Vector::new(x[0], x[1], x[2]),
            Vector::new(y[0], y[1], y[2]),
        )
    }

    pub const XY: ([f64; 3], [f64; 3]) = ([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]); // Top
    pub const XZ: ([f64; 3], [f64; 3]) = ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0]); // Front
    pub const YZ: ([f64; 3], [f64; 3]) = ([0.0, 1.0, 0.0], [0.0, 0.0, 1.0]); // Right

    /// Answer every question of `shape` in turn, then build it.
    pub fn build(
        shape: &Shape,
        plane: &Plane,
        answers: &[Answer],
        option: &str,
    ) -> Result<Geometry, String> {
        let frame = frame_of(plane, answers);

        for count in 1..=answers.len() {
            let part = (shape.read)(&frame, &answers[..count], option)?;
            let asked = (shape.ask)(&answers[..count], &part, option).is_some();
            assert_eq!(
                asked,
                count < answers.len(),
                "{shape:?} after {count} answers"
            );
        }

        let part = (shape.read)(&frame, answers, option)?;
        (shape.build)(&part, option)
    }

    /// Every point of a geometry: BRep vertices, mesh vertices or curve samples.
    pub fn corners(geometry: &Geometry) -> Vec<Point> {
        match geometry {
            Geometry::BRep(brep) => brep.vertex_points(),
            Geometry::Mesh(mesh) => mesh
                .vertex
                .values()
                .map(|vertex| vertex.position())
                .collect(),
            Geometry::NurbsCurve(curve) => curve.divide_by_count(RING, true).0,
            _ => Vec::new(),
        }
    }

    /// Low and high corners of the points.
    pub fn bounds(points: &[Point]) -> [[f64; 3]; 2] {
        let mut low = [f64::MAX; 3];
        let mut high = [f64::MIN; 3];

        for q in points {
            for i in 0..3 {
                low[i] = low[i].min(q[i]);
                high[i] = high[i].max(q[i]);
            }
        }

        [low, high]
    }

    /// Close within 1e-9 of the size.
    pub fn near(a: [[f64; 3]; 2], b: [[f64; 3]; 2]) -> bool {
        let scale = b.iter().flatten().fold(1.0_f64, |m, v| m.max(v.abs()));
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .all(|(x, y)| (x - y).abs() <= 1e-9 * scale)
    }

    /// Xy gives +Z, Xz gives −Y, Yz gives +X; the flipped frame is still a rotation.
    #[test]
    fn frames_are_right_handed_on_every_plane() {
        for ((x, y), z) in [
            (XY, [0.0, 0.0, 1.0]),
            (XZ, [0.0, -1.0, 0.0]),
            (YZ, [1.0, 0.0, 0.0]),
        ] {
            let frame = Frame::new(&plane(x, y), p(1.0, 2.0, 3.0));
            assert_eq!([frame.z[0], frame.z[1], frame.z[2]], z);
            let flipped = frame.flipped();
            let turn = flipped.x.cross(&flipped.y);
            assert!((turn.dot(&flipped.z) - 1.0).abs() < 1e-12);
            let rolled = frame.rolled();
            assert!((rolled.x.cross(&rolled.y).dot(&rolled.z) - 1.0).abs() < 1e-12);
            let moved = frame.to_xform().transform_point(&p(1.0, 0.0, 0.0));
            assert!(moved.distance(&frame.at(1.0, 0.0, 0.0), None) < 1e-12);
        }

        let turned = Frame::new(&plane(XY.0, XY.1), p(0.0, 0.0, 0.0)).turned(0.0, 2.0);
        assert!(turned.at(1.0, 0.0, 0.0).distance(&p(0.0, 1.0, 0.0), None) < 1e-12);
        assert!(turned.at(0.0, 1.0, 0.0).distance(&p(-1.0, 0.0, 0.0), None) < 1e-12);
    }

}
