
/// A press with Ctrl held, and Shift held, not held or either.
const fn ctrl(chars: &'static [&'static str], shift: Option<bool>, run: fn(&mut State)) -> Binding {
    Binding {
        trigger: Trigger::Chars(chars),
        ctrl: true,
        shift,
        run,
    }
}
