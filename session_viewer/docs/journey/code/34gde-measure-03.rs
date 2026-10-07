use crate::load_measure::{now, run};

#[test]
fn completed_work_keeps_its_result_and_exact_clock_boundaries() {
    for result in [Ok(42), Err("rejected")] {
        let mut samples = [12.5, 17.0].into_iter();
        let mut observed = Vec::new();
        let actual = run("decode", 512, &mut || samples.next().unwrap(), &mut |phase| {
            observed.push((phase.name.to_owned(), phase.duration_ms, phase.elapsed_ms, phase.bytes, phase.succeeded));
        }, || result);
        assert_eq!(actual, result);
        assert_eq!(observed, vec![("decode".into(), 4.5, 17.0, 512, result.is_ok())]);
        assert!(samples.next().is_none());
    }
}

#[test]
fn invalid_clocks_cannot_change_or_retain_an_operation() {
    for samples in [[2.0, 1.0], [f64::NAN, 3.0], [0.0, f64::INFINITY], [-1.0, 2.0]] {
        let mut samples = samples.into_iter();
        let mut calls = 0;
        let result = run("validation", 8, &mut || samples.next().unwrap(), &mut |_| calls += 1,
            || Err::<(), _>("invalid geometry"));
        assert_eq!(result, Err("invalid geometry"));
        assert_eq!(calls, 0);
    }
}

#[test]
fn native_clock_is_finite_and_monotonic() {
    let first = now(); let second = now();
    assert!(first.is_finite() && first >= 0.0 && second >= first);
}
