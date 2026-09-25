use crate::State;
use crate::app::command::tool::surfacing::{self, Picked};
use crate::app::command::tool::{Next, Overlay, Stroke, Tool, typed_number};
use crate::app::command::{Action, verbs};
use crate::app::coords;
use crate::app::cplane::CPlane;
use session_rust::{Geometry, Plane, Point, Vector};

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
    fn prompt(&self) -> &'static str {
        match self {
            Step::Curves { prompt, .. } | Step::Number { prompt, .. } => prompt,
            Step::Point(prompt) | Step::Distance(prompt) => prompt,
        }
    }

    /// True for a step answered by picking curves.
    fn curves(&self) -> bool {
        matches!(self, Step::Curves { .. })
    }

    /// A typed word as this step's answer.
    fn read(&self, word: &str) -> Result<Answer, String> {
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
            Step::Point(_) => match coords::parse(word) {
                Some(coords::Typed::Absolute { x, y, z }) => {
                    Ok(Answer::Point(Point::new(x, y, z.unwrap_or(0.0))))
                }
                _ => Err(format!("`{word}` is not a point x,y,z")),
            },
            Step::Distance(_) => match (typed_number(word), coords::parse(word)) {
                (Some(value), _) => Ok(Answer::Number(value)),
                (None, Some(coords::Typed::Absolute { x, y, z })) => {
                    Ok(Answer::Vector(Vector::new(x, y, z.unwrap_or(0.0))))
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
    fn choices(&self) -> impl Iterator<Item = &'static str> {
        self.chips
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

    /// The steps that are not curve picks.
    fn others(&self) -> impl Iterator<Item = &'static Step> {
        self.steps.iter().filter(|step| !step.curves())
    }
}

/// Lowercase without spaces.
fn compact(text: &str) -> String {
    text.split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase()
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

/// Start `recipe`; typed words pick options and answer its questions.
pub fn start(recipe: &'static Recipe, words: &[&str]) -> Result<Box<dyn Action>, String> {
    let (choice, answers) = read(recipe, words)?;
    Ok(Box::new(Gather {
        recipe,
        choice,
        answers,
    }))
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
            Some(Step::Curves { min, .. }) if self.rows[self.step].len() < *min => Err(format!(
                "Select at least {}",
                surfacing::count(*min, "curve")
            )),
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

/// A gathering command about to start.
#[derive(Debug)]
pub struct Gather {
    recipe: &'static Recipe, // what it builds
    choice: usize,           // the chosen option
    answers: Vec<Answer>,    // values typed with the command
}

impl Action for Gather {
    /// Selected curves answer first; a typed command that fails leaves them selected.
    fn run(&self, state: &mut State) -> Result<String, String> {
        let rows = state.ordered_rows();
        let tool = Gathering::new(self.recipe, self.choice, self.answers.clone());
        let answer = state.open_tool(Box::new(tool));

        if answer.is_err() && state.tool_running() {
            state.cancel_drawing();
            state.select_rows(rows, false);
        }

        answer
    }
}

/// A gathering command running.
pub struct Gathering {
    recipe: &'static Recipe,       // what it builds
    choice: usize,                 // index into the options
    progress: Progress,            // where it is
    answers: Vec<Answer>,          // typed or clicked values
    curves: Vec<(u32, Picked)>,    // every row picked, in the world
    typed: String,                 // option words typed so far, e.g. `cap` before `on`
    normal: Vector,                // the construction plane the camera faces
    axis: Option<(Point, Vector)>, // where a Distance step slides
    lifted: Option<Point>,         // the cursor on that axis
    outlines: Vec<Vec<Point>>,     // the picked curves, for the Distance preview
    before: Vec<u32>,              // the selection it began with, back on Esc
}

impl std::fmt::Debug for Gathering {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Gathering({})", self.recipe.name)
    }
}

impl Gathering {
    /// Nothing picked yet; typed answers wait for their steps.
    pub fn new(recipe: &'static Recipe, choice: usize, answers: Vec<Answer>) -> Self {
        Self {
            recipe,
            choice,
            progress: Progress::new(recipe.steps, answers.len()),
            answers,
            curves: Vec::new(),
            typed: String::new(),
            normal: Vector::new(0.0, 0.0, 1.0),
            axis: None,
            lifted: None,
            outlines: Vec::new(),
            before: Vec::new(),
        }
    }

    /// The chosen option's label, empty when there is none.
    fn choice(&self) -> &'static str {
        self.recipe.choices().nth(self.choice).unwrap_or("")
    }

    /// The step waiting.
    fn current(&self) -> Option<&'static Step> {
        self.recipe.steps.get(self.progress.step)
    }

    /// The world curve of `row`, reading it on first use; None when it is no curve.
    fn world(&mut self, state: &State, row: u32) -> Option<Picked> {
        if let Some((_, picked)) = self.curves.iter().find(|(r, _)| *r == row) {
            return Some(picked.clone());
        }

        let geometry = state.scene.geometry(row)?;
        let picked = surfacing::picked(geometry, &state.scene.placement_of(row)?)?;
        self.curves.push((row, picked.clone()));
        Some(picked)
    }

    /// The gathered input for `progress`.
    fn input(&self, progress: &Progress) -> Input {
        let lookup = |row: &u32| {
            self.curves
                .iter()
                .find(|(r, _)| r == row)
                .map(|(_, picked)| picked.clone())
        };
        Input {
            curves: progress
                .rows
                .iter()
                .map(|rows| rows.iter().filter_map(lookup).collect())
                .collect(),
            answers: self.answers[..progress.answered.min(self.answers.len())].to_vec(),
            choice: self.choice(),
            normal: self.normal.clone(),
        }
    }

    /// Highlight the picked rows, or take the highlight away.
    fn light(&self, state: &mut State, on: bool) {
        for row in self.progress.picked() {
            state.gpu.set_selected(row, on);
        }
    }

    /// Carry on with `progress`, or build once it is done; a refused build keeps the old progress.
    fn settle(
        &mut self,
        state: &mut State,
        progress: Progress,
        answers: Vec<Answer>,
    ) -> Result<Next, String> {
        let (before, kept) = (
            std::mem::replace(&mut self.progress, progress),
            std::mem::replace(&mut self.answers, answers),
        );

        if !self.progress.done(self.recipe.steps) {
            self.prepare(state);
            return Ok(Next::More);
        }

        self.normal = facing(state);
        let made = (self.recipe.build)(&self.input(&self.progress));
        let made = match made {
            Ok(made) => made,
            Err(error) => {
                self.progress = before;
                self.answers = kept;
                return Err(error);
            }
        };
        self.light(state, false);
        verbs::geometry::create_all(state, made.geometries, &made.message).map(Next::Done)
    }

    /// Entering a Distance step: its axis and the outlines that follow the cursor.
    fn prepare(&mut self, state: &State) {
        self.lifted = None;

        if !matches!(self.current(), Some(Step::Distance(_))) || self.axis.is_some() {
            return;
        }

        self.normal = facing(state);
        let input = self.input(&self.progress);
        self.axis = Some((self.recipe.axis)(&input));
        self.outlines = input.curves.iter().flatten().map(Picked::outline).collect();
    }

    /// The cursor on the Distance axis, None when the view looks along it.
    fn on_axis(&self, state: &State, at: (f64, f64)) -> Option<Point> {
        let (origin, direction) = state.camera.ray(at, state.viewport())?;
        let (base, axis) = self.axis.as_ref()?;
        let (a, b) = (direction.dot(&direction), direction.dot(axis));
        let denominator = a - b * b; // |axis| = 1

        if denominator <= 0.01 * a {
            return None;
        }

        let offset = &origin - base;
        let t = (a * offset.dot(axis) - b * offset.dot(&direction)) / denominator;
        Some(base + &(axis * t))
    }

    /// The Distance vector from the axis base to `p`.
    fn offset(&self, p: &Point) -> Option<Vector> {
        let (base, _) = self.axis.as_ref()?;
        Some(p - base)
    }
}

/// The normal of the construction plane the camera faces.
fn facing(state: &State) -> Vector {
    let forward = state.camera.orientation.rotate_vector(Vector::y_axis());
    CPlane::facing(&forward).normal()
}

impl Tool for Gathering {
    fn name(&self) -> &'static str {
        self.recipe.name
    }

    fn prompt(&self, _points: &[Point]) -> String {
        let Some(step) = self.current() else {
            return String::new();
        };
        let choice = match self.choice() {
            "" => String::new(),
            choice => format!(" · {choice}"),
        };
        let more = match step {
            Step::Curves { min, max, .. } => {
                let count = self.progress.rows[self.progress.step].len();
                match (*max, count >= *min) {
                    (1, _) => format!(" ({count})"),
                    (_, true) => format!(" ({count}) · Enter continues"),
                    _ => format!(" ({count})"),
                }
            }
            Step::Number { default, .. } => format!(" <{default}> · Enter keeps it"),
            Step::Distance(_) => " · move the cursor, or type a number or x,y,z".into(),
            Step::Point(_) => String::new(),
        };
        format!("{}{more}{choice}", step.prompt())
    }

    fn options(&self) -> &'static [(&'static str, &'static str)] {
        self.recipe.chips
    }

    fn chosen(&self) -> Option<&'static str> {
        Some(self.choice()).filter(|choice| !choice.is_empty())
    }

    fn asks_points(&self) -> bool {
        matches!(self.current(), Some(Step::Point(_)))
    }

    /// Selected curves answer the curve steps; everything answered builds at once.
    fn begin(&mut self, state: &mut State) -> Result<Next, String> {
        self.before = state.ordered_rows();
        let rows: Vec<u32> = self
            .before
            .iter()
            .copied()
            .filter(|&row| state.scene.selectable(row))
            .collect();
        state.select(None);
        let mut progress = self.progress.clone();

        if self.recipe.steps.iter().any(Step::curves) {
            let curves: Vec<u32> = rows
                .into_iter()
                .filter(|&row| self.world(state, row).is_some())
                .collect();
            progress.preselect(self.recipe.steps, &curves);
        }

        let answers = self.answers.clone();
        let answer = self.settle(state, progress.clone(), answers);

        // a refused build still shows what was picked
        if answer.is_err() {
            self.progress = progress;
        }

        if !matches!(answer, Ok(Next::Done(_))) {
            self.light(state, true);
        }

        answer
    }

    fn picks(&self) -> bool {
        matches!(self.current(), Some(Step::Curves { .. }))
    }

    fn picked(&mut self, state: &mut State, row: Option<u32>) -> Result<Next, String> {
        let Some(row) = row else {
            return Err(format!("Nothing there · {}", self.prompt(&[])));
        };

        if let Some(reason) = state.locked_reason(&[row]) {
            return Err(reason);
        }

        if !state.scene.selectable(row) || self.world(state, row).is_none() {
            return Err("Pick a line, polyline or curve".into());
        }

        let mut progress = self.progress.clone();
        progress.pick(self.recipe.steps, row)?;
        let on = progress.picked().contains(&row);
        state.gpu.set_selected(row, on);
        let answers = self.answers.clone();
        let answer = self.settle(state, progress, answers);

        if answer.is_err() {
            state.gpu.set_selected(row, !on);
        }

        answer
    }

    /// An option word switches the option; a number answers a Number or Distance step.
    fn word(
        &mut self,
        state: &mut State,
        word: &str,
        _points: &[Point],
        _plane: &Plane,
    ) -> Option<Result<Next, String>> {
        let typed = format!("{}{}", self.typed, word.to_ascii_lowercase());
        let whole = self
            .recipe
            .choices()
            .position(|label| compact(label) == typed);

        if let Some(index) = whole {
            self.choice = index;
            self.typed.clear();
            return Some(Ok(Next::More));
        }

        if self
            .recipe
            .choices()
            .any(|label| compact(label).starts_with(&typed))
        {
            self.typed = typed;
            return Some(Ok(Next::More));
        }

        self.typed.clear();
        let step = self.current()?;

        if !matches!(step, Step::Number { .. } | Step::Distance(_)) {
            return None;
        }

        let answer = match step.read(word) {
            Ok(answer) => answer,
            Err(error) => return Some(Err(error)),
        };
        let mut progress = self.progress.clone();
        let mut answers = self.answers.clone();
        progress.answer(self.recipe.steps, &mut answers, answer);
        Some(self.settle(state, progress, answers))
    }

    fn guide(&self, points: &[Point], cursor: Option<&Point>) -> Vec<Point> {
        match self.current() {
            Some(Step::Point(_)) => points.iter().chain(cursor).cloned().collect(),
            Some(Step::Distance(_)) => self
                .axis
                .iter()
                .map(|(base, _)| base.clone())
                .chain(self.lifted.clone())
                .collect(),
            _ => Vec::new(),
        }
    }

    fn readout(&self, _points: &[Point], _cursor: &Point, _plane: &Plane) -> String {
        let (Some((_, axis)), Some(lifted)) = (&self.axis, &self.lifted) else {
            return String::new();
        };
        self.offset(lifted)
            .map(|offset| format!("{:.3}", offset.dot(axis)))
            .unwrap_or_default()
    }

    fn placed(
        &mut self,
        state: &mut State,
        points: &[Point],
        _plane: &Plane,
    ) -> Result<Next, String> {
        if !matches!(self.current(), Some(Step::Point(_))) {
            return Err(format!("{}: {}", self.recipe.name, self.prompt(&[])));
        }

        let point = points.last().ok_or("No point was placed")?.clone();
        let mut progress = self.progress.clone();
        let mut answers = self.answers.clone();
        progress.answer(self.recipe.steps, &mut answers, Answer::Point(point));
        self.settle(state, progress, answers)
    }

    fn enter(&mut self, state: &mut State, _points: &[Point]) -> Result<Next, String> {
        let mut progress = self.progress.clone();
        let mut answers = self.answers.clone();
        progress.next(self.recipe.steps, &mut answers)?;
        self.settle(state, progress, answers)
    }

    /// Curve and number steps ignore the cursor; a Distance slides along its axis.
    fn hovered(&mut self, state: &mut State, at: (f64, f64)) -> Option<bool> {
        match self.current()? {
            Step::Point(_) => None,
            Step::Distance(_) => {
                self.lifted = self.on_axis(state, at);
                Some(true)
            }
            _ => Some(false),
        }
    }

    /// A click at a Distance step answers with the point on the axis.
    fn clicked(&mut self, state: &mut State, at: (f64, f64)) -> Option<Result<Next, String>> {
        if !matches!(self.current()?, Step::Distance(_)) {
            return None;
        }

        let Some(offset) = self.on_axis(state, at).and_then(|p| self.offset(&p)) else {
            return Some(Err("Type the distance: the view looks along it".into()));
        };
        let mut progress = self.progress.clone();
        let mut answers = self.answers.clone();
        progress.answer(self.recipe.steps, &mut answers, Answer::Vector(offset));
        Some(self.settle(state, progress, answers))
    }

    /// At a Distance step the picked curves follow the cursor.
    fn marks(&self, state: &State) -> Option<Overlay> {
        let offset = self.lifted.as_ref().and_then(|p| self.offset(p))?;
        let screen = state.view_screen();
        let strokes = self
            .outlines
            .iter()
            .map(|outline| Stroke {
                points: outline
                    .iter()
                    .filter_map(|p| screen.point(&(p + &offset)))
                    .collect(),
                color: [30, 110, 170],
                width: 1.5,
                dashed: false,
            })
            .filter(|stroke| stroke.points.len() >= 2)
            .collect();
        Some(Overlay {
            strokes,
            ..Default::default()
        })
    }

    /// Esc: the picks lose their highlight and the selection it began with comes back.
    fn cancel(&mut self, state: &mut State) {
        self.light(state, false);

        if !self.before.is_empty() {
            state.select_rows(std::mem::take(&mut self.before), false);
        }
    }

    fn status(&self) -> serde_json::Value {
        let step = self.current();
        serde_json::json!({
            "command": self.recipe.name,
            "option": self.choice(),
            "step": self.progress.step,
            "kind": step.map(|step| match step {
                Step::Curves { .. } => "curves",
                Step::Point(_) => "point",
                Step::Number { .. } => "number",
                Step::Distance(_) => "distance",
            }),
            "prompt": self.prompt(&[]),
            "rows": self.progress.picked(),
            "count": step
                .filter(|step| step.curves())
                .map_or(0, |_| self.progress.rows[self.progress.step].len()),
            "answers": self.progress.answered,
            "lifted": self.lifted.as_ref().map(|p| [p[0], p[1], p[2]]),
        })
    }
}

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
