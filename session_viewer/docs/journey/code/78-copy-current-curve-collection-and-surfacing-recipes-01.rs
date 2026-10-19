use crate::{primitive_options as options,drawing_coords as coords};
use session_rust::{Geometry,NurbsCurve,Point,Vector,Xform};
fn typed_number(word:&str)->Option<f64>{word.parse::<f64>().ok().filter(|v|v.is_finite())}
/// One question a gathering command asks; Curves steps come first.
#[derive(Clone, Copy, Debug)]
pub enum Step {
    Curves {
        prompt: &'static str,
        min: usize,
        max: usize,
    }, // curves clicked in order; reaching max or Enter moves on
    Point(&'static str), // a click, snap or typed x,y,z
    Number {
        prompt: &'static str,
        default: f64,
        low: f64,
        high: f64,
    }, // low < value <= high; Enter takes the default
    Distance(&'static str), // along the recipe's axis: a click, a signed number or a typed vector
}

impl Step {
    /// What the step asks for.
    pub fn prompt(&self) -> &'static str {
        match self {
            Step::Curves { prompt, .. } | Step::Number { prompt, .. } => prompt,
            Step::Point(prompt) | Step::Distance(prompt) => prompt,
        }
    }

    /// True for a step answered by picking curves.
    pub fn curves(&self) -> bool {
        matches!(self, Step::Curves { .. })
    }

    /// A typed word as this step's answer.
    pub fn read(&self, word: &str) -> Result<Answer, String> {
        match self {
            Step::Number {
                prompt, low, high, ..
            } => match typed_number(word) {
                Some(value) if value > *low && value <= *high => Ok(Answer::Number(value)),
                Some(_) => Err(format!(
                    "The {prompt} must be above {low} and at most {high}"
                )),
                None => Err(format!("`{word}` is not a number")),
            },
            Step::Point(_) => match coords::absolute(word) {
                Some([x, y, z]) => Ok(Answer::Point(Point::new(x, y, z))),
                None => Err(format!("`{word}` is not a point x,y,z")),
            },
            Step::Distance(_) => match (typed_number(word), coords::parse_world(word)) {
                (Some(value), _) => Ok(Answer::Number(value)),
                (None, Some((coords::Typed::Absolute { x, y, z }, world))) => {
                    let [x, y, z] = coords::offset([x, y, z.unwrap_or(0.0)], world);
                    Ok(Answer::Vector(Vector::new(x, y, z)))
                }
                _ => Err(format!("`{word}` is not a distance or a vector x,y,z")),
            },
            Step::Curves { .. } => Err("Click the curves".into()),
        }
    }
}

/// An answer to a Point, Number or Distance step.
#[derive(Clone, Debug)]
pub enum Answer {
    Point(Point),   // a location
    Number(f64),    // a value, or a distance along the axis
    Vector(Vector), // a Distance given as a vector
}

/// What a command gathered, in the world.
#[derive(Clone)]
pub struct Input {
    pub curves: Vec<Vec<Picked>>, // per Curves step, in pick order
    pub answers: Vec<Answer>,     // per other step
    pub choice: &'static str,     // the chosen option, empty when none
    pub normal: Vector,           // the construction plane the camera faces
}

impl Input {
    /// The Distance answer as a vector along `axis`.
    pub fn along(&self, index: usize, axis: &Vector) -> Option<Vector> {
        match self.answers.get(index)? {
            Answer::Number(value) => Some(axis * *value),
            Answer::Vector(vector) => Some(vector.clone()),
            Answer::Point(_) => None,
        }
    }

    /// The Point answer at `index`.
    pub fn point(&self, index: usize) -> Option<Point> {
        match self.answers.get(index)? {
            Answer::Point(p) => Some(p.clone()),
            _ => None,
        }
    }

    /// The Number answer at `index`.
    pub fn number(&self, index: usize) -> Option<f64> {
        match self.answers.get(index)? {
            Answer::Number(value) => Some(*value),
            _ => None,
        }
    }
}

/// What a build made and what to say about it.
pub struct Made {
    pub geometries: Vec<Geometry>, // added as one undo step
    pub message: String,           // e.g. `Lofted 3 curves into a NURBS surface`
}

/// A command that gathers curves and answers, then builds.
pub struct Recipe {
    pub name: &'static str,                             // shown name
    pub chips: &'static [(&'static str, &'static str)], // buttons: options, then Finish and Cancel
    pub steps: &'static [Step],                         // what it asks, in order
    pub axis: fn(&Input) -> (Point, Vector), // where a Distance step slides, unit direction
    pub build: fn(&Input) -> Result<Made, String>, // the objects from the answers
}

impl std::fmt::Debug for Recipe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.name)
    }
}

impl Recipe {
    /// The option labels, e.g. Open and Closed.
    pub fn choices(&self) -> impl Iterator<Item = &'static str> {
        options::choices(self.chips)
    }

    /// The option `words` start with and how many words spell it, ignoring case and spaces.
    pub fn option_in(&self, words: &[&str]) -> Option<(usize, usize)> {
        options::find(self.chips, words)
    }

    /// The steps that are not curve picks.
    pub fn others(&self) -> impl Iterator<Item = &'static Step> {
        self.steps.iter().filter(|step| !step.curves())
    }
}

/// Read typed words: options anywhere, the last one winning, and answers in step order.
pub fn read(recipe: &Recipe, words: &[&str]) -> Result<(usize, Vec<Answer>), String> {
    let others: Vec<&Step> = recipe.others().collect();
    let mut choice = 0;
    let mut answers = Vec::new();
    let mut rest = words;

    while !rest.is_empty() {
        if let Some((index, count)) = recipe.option_in(rest) {
            choice = index;
            rest = &rest[count..];
            continue;
        }

        let Some(step) = others.get(answers.len()) else {
            let options: Vec<_> = recipe.choices().collect();
            let listed = if options.is_empty() {
                String::new()
            } else {
                format!("; its options are {}", options.join(", "))
            };
            return Err(format!(
                "`{}` is not a value of {}{listed}",
                rest[0], recipe.name
            ));
        };
        answers.push(step.read(rest[0])?);
        rest = &rest[1..];
    }

    // typed values take the defaults of the numbers left, e.g. a full turn
    if !answers.is_empty()
        && others[answers.len()..]
            .iter()
            .all(|step| matches!(step, Step::Number { .. }))
    {
        for step in &others[answers.len()..] {
            if let Step::Number { default, .. } = step {
                answers.push(Answer::Number(*default));
            }
        }
    }

    Ok((choice, answers))
}

/// Where a gathering command is: rows per Curves step, answers per other step.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Progress {
    pub rows: Vec<Vec<u32>>, // picked rows per Curves step, in pick order
    pub answered: usize,     // other steps answered, in order
    pub step: usize,         // the step waiting; steps.len() when done
}

impl Progress {
    /// Nothing picked; `answered` other steps already have their answers.
    pub fn new(steps: &[Step], answered: usize) -> Self {
        let mut progress = Self {
            rows: vec![Vec::new(); steps.iter().filter(|step| step.curves()).count()],
            answered,
            step: 0,
        };
        progress.skip(steps);
        progress
    }

    /// Selected rows answer the Curves steps in order, up to each step's max; stop at the first short one.
    pub fn preselect(&mut self, steps: &[Step], rows: &[u32]) {
        let mut rest = rows;

        while let Some(Step::Curves { min, max, .. }) = steps.get(self.step) {
            let taken = rest.len().min(*max);
            self.rows[self.step].extend_from_slice(&rest[..taken]);
            rest = &rest[taken..];

            if taken < *min {
                return;
            }

            self.step += 1;
            self.skip(steps);
        }
    }

    /// Step over other steps whose answers were typed already.
    fn skip(&mut self, steps: &[Step]) {
        while let Some(step) = steps.get(self.step)
            && !step.curves()
            && self.other(steps) < self.answered
        {
            self.step += 1;
        }
    }

    /// The index of the current step among the other steps.
    fn other(&self, steps: &[Step]) -> usize {
        steps[..self.step]
            .iter()
            .filter(|step| !step.curves())
            .count()
    }

    /// Every row picked so far, in step and pick order.
    pub fn picked(&self) -> Vec<u32> {
        self.rows.iter().flatten().copied().collect()
    }

    /// Pick `row` for the current Curves step; picking it again takes it back.
    pub fn pick(&mut self, steps: &[Step], row: u32) -> Result<(), String> {
        let Some(Step::Curves { max, .. }) = steps.get(self.step) else {
            return Err("Not picking curves now".into());
        };
        let rows = &mut self.rows[self.step];

        if let Some(at) = rows.iter().position(|r| *r == row) {
            rows.remove(at);
            return Ok(());
        }

        if self.rows.iter().flatten().any(|r| *r == row) {
            return Err("That curve is picked already".into());
        }

        self.rows[self.step].push(row);

        if self.rows[self.step].len() >= *max {
            self.step += 1;
            self.skip(steps);
        }

        Ok(())
    }

    /// Enter: a Curves step with enough picks or a Number step's default moves on.
    pub fn next(&mut self, steps: &[Step], answers: &mut Vec<Answer>) -> Result<(), String> {
        match steps.get(self.step) {
            Some(Step::Curves { min, .. }) if self.rows[self.step].len() < *min => {
                Err(format!("Select at least {}", count(*min, "curve")))
            }
            Some(Step::Curves { .. }) => {
                self.step += 1;
                self.skip(steps);
                Ok(())
            }
            Some(Step::Number { default, .. }) => {
                self.answer(steps, answers, Answer::Number(*default));
                Ok(())
            }
            Some(step) => Err(format!("Give the {}", step.prompt())),
            None => Ok(()),
        }
    }

    /// Answer the current step.
    pub fn answer(&mut self, steps: &[Step], answers: &mut Vec<Answer>, answer: Answer) {
        answers.truncate(self.answered);
        answers.push(answer);
        self.answered += 1;
        self.step += 1;
        self.skip(steps);
    }

    /// True once every step is answered.
    pub fn done(&self, steps: &[Step]) -> bool {
        self.step >= steps.len()
    }
}

/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
}

/// A line, polyline or NURBS curve moved into the world by `place`; None for anything else.
pub fn picked(geometry: &Geometry, place: &Xform) -> Option<Picked> {
    let corners = match geometry {
        Geometry::Line(line) => Some(vec![line.start(), line.end()]),
        Geometry::Polyline(polyline) => {
            let mut polyline = (**polyline).clone();
            polyline.remove_consecutive_duplicates(1e-12);
            Some(polyline.get_points())
        }
        Geometry::NurbsCurve(_) => None,
        _ => return None,
    };
    let corners: Option<Vec<Point>> =
        corners.map(|points| points.iter().map(|p| p.transformed(place)).collect());
    let mut curve = match (&corners, geometry) {
        (Some(points), _) if points.len() >= 2 => NurbsCurve::create(false, 1, points),
        (None, Geometry::NurbsCurve(curve)) => {
            let mut curve = (**curve).clone();

            // loft and sweep cannot mix periodic and clamped knots
            if curve.is_periodic() {
                curve.clamp_end(2);
            }

            curve.transform(place);
            curve
        }
        _ => return None,
    };

    if !curve.is_valid() || !curve.set_domain(0.0, 1.0) {
        return None;
    }

    Some(Picked { curve, corners })
}

/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
}

impl Picked {
    /// Points along the curve: its corners, else even samples.
    pub fn outline(&self) -> Vec<Point> {
        match &self.corners {
            Some(corners) => corners.clone(),
            None => self.curve.divide_by_count(SAMPLES, true).0,
        }
    }
}

pub const SAMPLES: usize = 48; // points per curve for normals and previews
#[cfg(test)]
mod tests {
    use super::*;

    const LOFT: &[Step] = &[Step::Curves {
        prompt: "select curves in order",
        min: 2,
        max: usize::MAX,
    }];
    const SWEEP: &[Step] = &[
        Step::Curves {
            prompt: "select the rail",
            min: 1,
            max: 1,
        },
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
    ];
    const REVOLVE: &[Step] = &[
        Step::Curves {
            prompt: "select profile curves",
            min: 1,
            max: usize::MAX,
        },
        Step::Point("axis start"),
        Step::Point("axis end"),
        Step::Number {
            prompt: "angle",
            default: 360.0,
            low: 0.0,
            high: 360.0,
        },
    ];

    /// Build nothing; tests read the recipe only.
    fn nothing(_: &Input) -> Result<Made, String> {
        Err("no build".into())
    }

    /// No axis.
    fn up(_: &Input) -> (Point, Vector) {
        (Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0))
    }

    static REVOLVING: Recipe = Recipe {
        name: "Revolve",
        chips: &[("Finish", ""), ("Cancel", "Escape")],
        steps: REVOLVE,
        axis: up,
        build: nothing,
    };
    static EXTRUDING: Recipe = Recipe {
        name: "Extrude",
        chips: &[
            ("Cap On", "Cap On"),
            ("Cap Off", "Cap Off"),
            ("Finish", ""),
            ("Cancel", "Escape"),
        ],
        steps: &[
            Step::Curves {
                prompt: "select curves",
                min: 1,
                max: usize::MAX,
            },
            Step::Distance("distance"),
        ],
        axis: up,
        build: nothing,
    };

    /// Clicks keep their order; a second click takes a row back.
    #[test]
    fn a_curves_step_keeps_click_order_and_toggles() {
        let mut progress = Progress::new(LOFT, 0);
        progress.pick(LOFT, 7).unwrap();
        progress.pick(LOFT, 3).unwrap();
        progress.pick(LOFT, 5).unwrap();
        assert_eq!(progress.rows[0], vec![7, 3, 5]);
        progress.pick(LOFT, 3).unwrap();
        assert_eq!(progress.rows[0], vec![7, 5]);
        assert!(!progress.done(LOFT));
    }

    /// Enter with too few curves is refused.
    #[test]
    fn enter_below_min_is_refused() {
        let mut progress = Progress::new(LOFT, 0);
        progress.pick(LOFT, 1).unwrap();
        let mut answers = Vec::new();
        assert_eq!(
            progress.next(LOFT, &mut answers),
            Err("Select at least 2 curves".into())
        );
        progress.pick(LOFT, 2).unwrap();
        progress.next(LOFT, &mut answers).unwrap();
        assert!(progress.done(LOFT));
    }

    /// A rail step of one curve moves on by itself; the rail cannot be a profile too.
    #[test]
    fn a_full_single_curve_step_advances_on_its_own() {
        let mut progress = Progress::new(SWEEP, 0);
        progress.pick(SWEEP, 4).unwrap();
        assert_eq!(progress.step, 1);
        assert!(progress.pick(SWEEP, 4).is_err());
        progress.pick(SWEEP, 9).unwrap();
        assert_eq!(progress.rows, vec![vec![4], vec![9]]);
    }

    /// Selected rows fill the curve steps in order and stop at the first short step.
    #[test]
    fn preselection_answers_curve_steps_in_order_and_stops_at_the_first_open_one() {
        let mut progress = Progress::new(SWEEP, 0);
        progress.preselect(SWEEP, &[3, 1, 2]);
        assert_eq!(progress.rows, vec![vec![3], vec![1, 2]]);
        assert!(progress.done(SWEEP));
        let mut short = Progress::new(LOFT, 0);
        short.preselect(LOFT, &[8]);
        assert_eq!((short.step, short.rows[0].clone()), (0, vec![8]));
        let mut revolve = Progress::new(REVOLVE, 0);
        revolve.preselect(REVOLVE, &[6]);
        assert_eq!(revolve.step, 1, "axis start waits");
    }

    /// Typed values answer the steps in order; the angle takes its default.
    #[test]
    fn answers_are_appended_in_step_order() {
        let (_, answers) = read(&REVOLVING, &["0,0,0", "0,0,10", "90"]).unwrap();
        assert!(matches!(answers[2], Answer::Number(value) if value == 90.0));
        let (_, answers) = read(&REVOLVING, &["0,0,0", "0,0,10"]).unwrap();
        assert!(matches!(answers[2], Answer::Number(value) if value == 360.0));
        assert!(read(&REVOLVING, &["0,0,0", "0,0,10", "400"]).is_err());
        assert!(read(&REVOLVING, &["0,0,0", "0,0,10", "90", "5"]).is_err());
        let mut progress = Progress::new(REVOLVE, answers.len());
        progress.preselect(REVOLVE, &[2]);
        assert!(progress.done(REVOLVE));
    }

    /// Enter at a Number step keeps the default.
    #[test]
    fn a_number_step_keeps_the_default_on_enter() {
        let mut progress = Progress::new(REVOLVE, 0);
        let mut answers = Vec::new();
        progress.preselect(REVOLVE, &[1]);
        progress.answer(REVOLVE, &mut answers, Answer::Point(Point::new(0., 0., 0.)));
        progress.answer(REVOLVE, &mut answers, Answer::Point(Point::new(0., 0., 1.)));
        let mut again = progress.clone();
        again.next(REVOLVE, &mut answers).unwrap();
        assert!(again.done(REVOLVE));
        assert!(matches!(answers[2], Answer::Number(value) if value == 360.0));
    }

    /// Options take several words, the last one winning; unknown words are refused.
    #[test]
    fn options_are_words_and_the_last_wins() {
        assert_eq!(EXTRUDING.option_in(&["cap", "off", "5"]), Some((1, 2)));
        let (choice, answers) = read(&EXTRUDING, &["Cap", "Off", "Cap", "On", "10"]).unwrap();
        assert_eq!((choice, answers.len()), (0, 1));
        let (_, answers) = read(&EXTRUDING, &["0,0,5"]).unwrap();
        assert!(matches!(&answers[0], Answer::Vector(v) if v[2] == 5.0));
        assert!(read(&EXTRUDING, &["sideways"]).is_err());
    }
}


pub fn no_axis(_input:&Input)->(Point,Vector){(Point::new(0.,0.,0.),Vector::new(0.,0.,1.))}
