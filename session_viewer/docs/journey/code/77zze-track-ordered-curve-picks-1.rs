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

}

/// A picked curve in world coordinates.
#[derive(Clone, Debug)]
pub struct Picked {
    pub curve: NurbsCurve,           // clamped, domain 0..1
    pub corners: Option<Vec<Point>>, // a line's or polyline's points
}

/// "1 curve" or "3 curves".
pub fn count(count: usize, noun: &str) -> String {
    match count {
        1 => format!("1 {noun}"),
        _ => format!("{count} {noun}s"),
    }
