use crate::State;
use crate::app::command::tool::{Next, Overlay, Stroke, Tool, typed_number};
use crate::app::command::{Action, verbs};
use crate::app::coords;
use session_rust::{BRep, Geometry, Mesh, Plane, Point, Polyline, Vector, Xform};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::OnceLock;

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
    fn choices(&self) -> impl Iterator<Item = &'static str> {
        self.options
            .iter()
            .filter(|(_, line)| !line.is_empty() && *line != "Escape")
            .map(|(label, _)| *label)
    }

    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        self.choices().enumerate().find_map(|(index, label)| {
            let target = compact(label);
            let mut typed = String::new();

            for (count, word) in words.iter().enumerate() {
                typed.push_str(&word.to_ascii_lowercase());

                if typed == target {
                    return Some((index, count + 1));
                }

                if !target.starts_with(&typed) {
                    return None;
                }
            }

            None
        })
    }
}

/// Lowercase without spaces.
fn compact(text: &str) -> String {
    text.split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase()
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

/// Start drawing `shape`: a leading word picks an option, the rest answer its questions.
pub fn start(shape: &'static Shape, words: &[&str]) -> Result<Box<dyn Action>, String> {
    let (option, rest) = match shape.option_in(words) {
        Some((index, count)) => (index, &words[count..]),
        None => (0, words),
    };

    if let Some(word) = rest.iter().find(|word| coords::parse(word).is_none()) {
        let options: Vec<_> = shape.choices().collect();
        let listed = if options.is_empty() {
            String::new()
        } else {
            format!("; its options are {}", options.join(", "))
        };
        return Err(format!(
            "`{word}` is not a point or a number of {}{listed}",
            shape.name
        ));
    }

    Ok(Box::new(Start {
        shape,
        option,
        words: rest.iter().map(|word| word.to_string()).collect(),
    }))
}

/// A creation command about to ask its questions.
#[derive(Debug)]
pub struct Start {
    shape: &'static Shape, // what it makes
    option: usize,         // the chosen option
    words: Vec<String>,    // answers typed with the command
}

impl Action for Start {
    /// Open the questions; typed answers go in at once, and a refused one cancels.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let prompt = state.open_tool(Box::new(Shaping::new(self.shape, self.option)))?;

        if self.words.is_empty() {
            return Ok(prompt);
        }

        let answer = state.run_command(&self.words.join(" "));

        if answer.is_err() {
            state.cancel_drawing();
        }

        answer
    }
}

/// A shape being drawn: its option and the answers so far.
pub struct Shaping {
    shape: &'static Shape, // what it makes
    option: usize,         // index into the choices
    answers: Vec<Answer>,  // in question order
    part: Part,            // what the answers say
    plane: Plane,          // the drawing plane, from the draft
    lifted: Option<Point>, // the cursor on the normal, while a height is asked
}

impl std::fmt::Debug for Shaping {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Shaping({} {})", self.shape.name, self.choice())
    }
}

impl Shaping {
    /// Nothing answered yet.
    pub fn new(shape: &'static Shape, option: usize) -> Self {
        let plane = Plane::default();
        let part = Part::new(frame_of(&plane, &[]));
        Self {
            shape,
            option,
            answers: Vec::new(),
            part,
            plane,
            lifted: None,
        }
    }

    /// The chosen option's label, empty when there is none.
    fn choice(&self) -> &'static str {
        self.shape.choices().nth(self.option).unwrap_or("")
    }

    /// The question waiting for an answer.
    fn next(&self) -> Option<Ask> {
        (self.shape.ask)(&self.answers, &self.part, self.choice())
    }

    /// The answers plus `answer`, and what they say; an error refuses it.
    fn with(&self, answer: Answer) -> Result<(Vec<Answer>, Part), String> {
        let mut answers = self.answers.clone();
        answers.push(answer);
        let frame = frame_of(&self.plane, &answers);
        let part = (self.shape.read)(&frame, &answers, self.choice())?;
        Ok((answers, part))
    }

    /// Take one answer; the last one builds the object into the document.
    fn take(&mut self, state: &mut State, answer: Answer) -> Result<Next, String> {
        let (answers, part) = self.with(answer)?;

        if (self.shape.ask)(&answers, &part, self.choice()).is_some() {
            self.answers = answers;
            self.part = part;
            return Ok(Next::More);
        }

        let geometry = (self.shape.build)(&part, self.choice())?;
        let what = match self.choice() {
            "" => self.shape.name.to_ascii_lowercase(),
            choice => format!("{} ({choice})", self.shape.name.to_ascii_lowercase()),
        };
        verbs::geometry::create(state, geometry, &what).map(Next::Done)
    }

    /// The cursor on the normal through the first point, None when the view looks along it.
    fn on_axis(&self, state: &State, at: (f64, f64)) -> Option<Point> {
        let (origin, direction) = state.camera.ray(at, state.viewport())?;
        let axis = &self.part.frame;
        let (a, b) = (direction.dot(&direction), direction.dot(&axis.z));
        let denominator = a - b * b; // |z| = 1

        if denominator <= 0.01 * a {
            return None;
        }

        let offset = &origin - &axis.origin;
        let t = (a * offset.dot(&axis.z) - b * offset.dot(&direction)) / denominator;
        Some(axis.at(0.0, 0.0, t))
    }

    /// The part with the cursor as the pending answer, else as answered.
    fn pending(&self, cursor: Option<&Point>) -> Part {
        let height = self.next().is_some_and(|ask| ask.kind == Kind::Height);
        let cursor = if height { self.lifted.as_ref() } else { cursor };
        cursor
            .and_then(|cursor| self.with(Answer::Point(cursor.clone())).ok())
            .map_or_else(|| self.part.clone(), |(_, part)| part)
    }
}

impl Tool for Shaping {
    fn name(&self) -> &'static str {
        self.shape.name
    }

    fn prompt(&self, _points: &[Point]) -> String {
        let Some(ask) = self.next() else {
            return String::new();
        };
        // a prompt naming its typed form already says so
        let number = match ask.kind {
            Kind::Size | Kind::Height if !ask.prompt.contains(" or ") => " or a number",
            _ => "",
        };
        let default = ask
            .default
            .map(|value| format!(" · Enter uses {}", (value * 1000.0).round() / 1000.0))
            .unwrap_or_default();
        format!("{}{number}{default}", ask.prompt)
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        self.shape.options
    }

    fn chosen(&self) -> Option<&'static str> {
        Some(self.choice()).filter(|choice| !choice.is_empty())
    }

    /// An option name switches the option; a number answers a size or height.
    fn word(
        &mut self,
        state: &mut State,
        word: &str,
        _points: &[Point],
        plane: &Plane,
    ) -> Option<Result<Next, String>> {
        self.plane = plane.clone();

        if let Some((index, _)) = self.shape.option_in(&[word]) {
            if self.shape.upfront && !self.answers.is_empty() && index != self.option {
                return Some(Err(format!("Choose {word} before the first point")));
            }

            self.option = index;
            return Some(Ok(Next::More));
        }

        let ask = self.next()?;

        if ask.kind == Kind::Point {
            return None;
        }

        let value = typed_number(word)?;
        Some(self.take(state, Answer::Number(value)))
    }

    fn guide(&self, points: &[Point], cursor: Option<&Point>) -> Vec<Point> {
        if self.next().is_some_and(|ask| ask.kind == Kind::Height) {
            let base = self.part.frame.origin.clone();
            return self.lifted.iter().cloned().chain([base]).collect();
        }

        points.last().into_iter().chain(cursor).cloned().collect()
    }

    fn readout(&self, _points: &[Point], cursor: &Point, _plane: &Plane) -> String {
        let cursor = match self.next().map(|ask| ask.kind) {
            Some(Kind::Size) => Some(cursor.clone()),
            Some(Kind::Height) => self.lifted.clone(),
            _ => None,
        };
        cursor
            .and_then(|cursor| self.with(Answer::Point(cursor)).ok())
            .and_then(|(_, part)| part.sizes.last().map(|size| format!("{size:.3}")))
            .unwrap_or_default()
    }

    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        plane: &Plane,
    ) -> Result<Next, String> {
        self.plane = plane.clone();
        let point = points.last().ok_or("No point was placed")?.clone();
        self.take(state, Answer::Point(point))
    }

    /// Enter takes the question's default.
    fn enter(&mut self, state: &mut State, _points: &[Point]) -> Result<Next, String> {
        let ask = self.next().ok_or("Nothing is asked")?;

        match ask.default {
            Some(value) => self.take(state, Answer::Number(value)),
            None => Err(format!(
                "{} needs the {}",
                self.shape.name,
                ask.prompt.to_ascii_lowercase()
            )),
        }
    }

    /// A height follows the cursor along the normal instead of the plane.
    fn hovered(&mut self, state: &mut State, at: (f64, f64)) -> Option<bool> {
        self.lifted = None;

        if self.next()?.kind != Kind::Height {
            return None;
        }

        self.lifted = self.on_axis(state, at);
        Some(true)
    }

    /// A click at a height question answers with the point on the normal.
    fn clicked(&mut self, state: &mut State, at: (f64, f64)) -> Option<Result<Next, String>> {
        if self.next()?.kind != Kind::Height {
            return None;
        }

        Some(match self.on_axis(state, at) {
            Some(point) => self.take(state, Answer::Point(point)),
            None => Err("Type the height: the view looks along it".into()),
        })
    }

    /// The shape as blue wires, the cursor answering the waiting question.
    fn marks(&self, state: &State) -> Option<Overlay> {
        let part = self.pending(state.drawing_cursor());
        let screen = state.view_screen();
        let mut strokes = Vec::new();

        for wire in (self.shape.outline)(&part) {
            let mut points = Vec::with_capacity(wire.len());

            for p in &wire {
                match screen.point(p) {
                    Some(at) => points.push(at),
                    None if points.len() >= 2 => strokes.push(stroke(std::mem::take(&mut points))),
                    None => points.clear(),
                }
            }

            if points.len() >= 2 {
                strokes.push(stroke(points));
            }
        }

        Some(Overlay {
            strokes,
            ..Default::default()
        })
    }

    fn status(&self) -> serde_json::Value {
        serde_json::json!({
            "command": self.shape.name,
            "option": self.choice(),
            "ask": self.next().map(|ask| ask.prompt),
            "answers": self.answers.len(),
            "sizes": self.part.sizes,
            "lifted": self.lifted.as_ref().map(|p| [p[0], p[1], p[2]]),
        })
    }
}

/// A preview wire.
fn stroke(points: Vec<(f64, f64)>) -> Stroke {
    Stroke {
        points,
        color: [30, 110, 170],
        width: 1.5,
        dashed: false,
    }
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

    /// A leading option word picks it; every other word must be a point or a number.
    #[test]
    fn typed_words_pick_an_option_then_answer() {
        let shape = &verbs::r#box::SHAPE;
        assert_eq!(shape.option_in(&["mesh", "0,0,0"]), Some((1, 1)));
        assert_eq!(shape.option_in(&["0,0,0"]), None);
        assert!(start(shape, &["Mesh", "0,0,0", "@50,25", "30"]).is_ok());
        assert!(start(shape, &["wide"]).is_err());
        let arc = &verbs::nurbs_curve_arc::SHAPE;
        assert_eq!(arc.option_in(&["3", "Points"]), Some((1, 2)));
        assert_eq!(arc.option_in(&["3points"]), Some((1, 1)));
        assert_eq!(arc.option_in(&["3"]), None);
    }

    /// Enter takes the default the question offers.
    #[test]
    fn enter_takes_the_default_or_refuses() {
        let shape = &verbs::r#box::SHAPE;
        let mut tool = Shaping::new(shape, 0);
        tool.plane = plane(XY.0, XY.1);
        assert!(tool.next().unwrap().default.is_none(), "base center");
        let (answers, part) = tool.with(Answer::Point(p(0.0, 0.0, 0.0))).unwrap();
        (tool.answers, tool.part) = (answers, part);
        let (answers, part) = tool.with(Answer::Number(400.0)).unwrap();
        (tool.answers, tool.part) = (answers, part);
        assert_eq!(tool.next().unwrap().default, Some(400.0));
        assert!(tool.prompt(&[]).contains("Enter uses 400"));
        let (answers, part) = tool.with(Answer::Number(300.0)).unwrap();
        (tool.answers, tool.part) = (answers, part);
        assert_eq!(tool.next().unwrap().prompt, "Height");
        assert_eq!(tool.next().unwrap().default, Some(300.0));
    }

    /// Tessellated primitives weld into closed meshes on their surfaces.
    #[test]
    fn brep_meshes_are_closed_and_welded() {
        let place = Xform::identity();

        for (brep, radius) in [
            (BRep::create_box(10.0, 20.0, 30.0), None),
            (BRep::create_sphere(10.0), Some(10.0)),
            (BRep::create_cylinder(5.0, 20.0), None),
            (BRep::create_torus(20.0, 5.0), None),
        ] {
            let Geometry::Mesh(mesh) = solid(brep, "Mesh", &place, "part") else {
                panic!()
            };
            assert!(mesh.is_closed(), "{}", mesh.number_of_faces());

            if let Some(radius) = radius {
                for q in corners(&Geometry::Mesh(mesh.clone())) {
                    assert!((q.distance(&p(0.0, 0.0, 0.0), None) - radius).abs() < 0.01 * radius);
                }
            }
        }
    }

    /// Polyhedra become closed solids of planar faces.
    #[test]
    fn polyhedra_become_solid_breps() {
        let place = Xform::identity();

        for (mesh, faces) in [
            (Mesh::create_dodecahedron(10.0), 12),
            (session_rust::Primitives::tetrahedron(10.0), 4),
        ] {
            let Geometry::BRep(brep) = polyhedron(mesh, "Brep", &place, "part") else {
                panic!()
            };
            assert_eq!(brep.face_count(), faces);
            assert!(brep.is_solid());
        }
    }
}
