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


