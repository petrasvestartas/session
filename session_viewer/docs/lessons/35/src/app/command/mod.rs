// --8<-- [start:command-action]
pub mod tool;
pub mod verbs;

pub use verbs::REGISTRY; // `pub use` re-exports it: callers write `command::REGISTRY` and never name the verbs module

use crate::State;
use crate::app::coords;

// `: Debug` asks every Action to print with {:?} too, so a test can compare `Create(Point, ..)` as text.
/// What a parsed line does. A trait is a promise of methods; every verb's parsed line is some type that keeps it.
pub trait Action: std::fmt::Debug {
    /// Carry it out; the answer is what to show the person.
    fn run(&self, state: &mut State) -> Result<String, String>;

    /// A method with a body is a default: a verb overrides only what differs, e.g. Delete says true here.
    fn needs_selection(&self) -> bool {
        false
    }

    /// True when a shape being drawn survives it.
    fn keeps_draft(&self) -> bool {
        false
    }

    /// True when a split waiting for cutters survives it.
    fn keeps_split(&self) -> bool {
        false
    }
}

/// One verb as the command line sees it: the names it answers to, its help line, its options and its parser.
pub struct Spec {
    pub names: &'static [&'static str], // `'static`: text stored in the binary for the whole run, so a const can hold it; the first name is the one shown
    pub aliases: &'static [&'static str], // accepted when typed, never shown
    pub hint: &'static str,             // help line, empty for the generic one
    pub options: &'static [&'static str], // clickable choices
    pub arity: Option<usize>, // exact argument count, when fixed: Delete takes Some(0)
    pub wait_for_option: bool,          // completing the bare verb waits for an option
    pub wait_after_option: bool,        // completing verb and option waits for more
    // A function pointer: each verb file names its own parser here. `Box<dyn Action>` is any Action on the heap; the caller never learns which type.
    pub parse: fn(verb: &str, rest: &[&str]) -> Result<Box<dyn Action>, String>,
}

impl Spec {
    /// A verb without aliases, options or a fixed argument count.
    pub const fn new(
        names: &'static [&'static str],
        hint: &'static str,
        parse: fn(&str, &[&str]) -> Result<Box<dyn Action>, String>,
    ) -> Self {
        Self {
            names,
            aliases: &[],
            hint,
            options: &[],
            arity: None,
            wait_for_option: false,
            wait_after_option: false,
            parse,
        }
    }
}

/// A REGISTRY entry. Two kinds share one list: a plain Spec, and a Draw that also turns points into geometry.
pub trait Verb {
    /// How it is typed, completed and parsed.
    fn spec(&self) -> &Spec;

    /// What it draws from points, for a drawing verb.
    fn draw(&self) -> Option<&verbs::geometry::Draw> {
        None
    }
}

/// A Spec is itself a Verb, so a plain verb puts `&SPEC` straight into the list.
impl Verb for Spec {
    fn spec(&self) -> &Spec {
        self
    }
}
// --8<-- [end:command-action]

// --8<-- [start:command-lookup]
/// A name lowercased without its spaces, so `clippingplane` spells `Clipping Plane`.
fn compact(name: &str) -> String {
    name.split_whitespace()
        .collect::<String>()
        .to_ascii_lowercase()
}

/// How many leading words spell `name`, ignoring case and the spaces inside it.
fn spells(name: &str, words: &[&str]) -> Option<usize> {
    let mut rest = compact(name);

    for (index, word) in words.iter().enumerate() {
        let word = word.to_ascii_lowercase();
        // `?` on an Option returns None from the whole function the moment a word does not continue the name.
        let tail = rest.strip_prefix(word.as_str())?.to_owned();

        if tail.is_empty() {
            return Some(index + 1);
        }

        rest = tail;
    }

    None
}

/// The verb the first words spell and how many words it took; when several names fit, the longest wins.
fn verb_words(words: &[&str]) -> Option<(&'static dyn Verb, usize)> {
    REGISTRY
        .iter()
        .flat_map(|verb| {
            let spec = verb.spec();
            spec.names.iter().chain(spec.aliases).filter_map(|name| {
                spells(name, words).map(|count| (*verb, count, compact(name).len()))
            })
        })
        // `Nurbs Surface Loft` beats `Loft` typed alone because it spells more letters
        .max_by_key(|(_, _, letters)| *letters)
        .map(|(verb, count, _)| (verb, count))
}

/// The Spec of the longest command name and how many words spell it.
fn command_words(words: &[&str]) -> Option<(&'static Spec, usize)> {
    verb_words(words).map(|(verb, count)| (verb.spec(), count))
}

/// The drawing verb the words start with, and how many words spell it.
pub fn drawing(words: &[&str]) -> Option<(&'static verbs::geometry::Draw, usize)> {
    let (verb, count) = verb_words(words)?;
    Some((verb.draw()?, count))
}
// --8<-- [end:command-lookup]

// --8<-- [start:command-text]
/// The line with its command spelled as shown, e.g. `clippingplane xy` becomes `Clipping Plane XY`.
pub fn canonical(line: &str) -> String {
    let words: Vec<_> = line.split_whitespace().collect();
    let Some((spec, count)) = command_words(&words) else {
        return line.trim().to_owned();
    };
    let name = spec.names[0];
    let mut text = name.to_owned();
    let mut rest = &words[count..];
    // the longest option the next words spell takes its shown case
    let option = spec
        .options
        .iter()
        .filter_map(|option| {
            let tail: Vec<_> = option
                .split_whitespace()
                .skip(name.split_whitespace().count())
                .collect();
            (!tail.is_empty()
                && rest.len() >= tail.len()
                && tail
                    .iter()
                    .zip(rest)
                    .all(|(a, b)| a.eq_ignore_ascii_case(b)))
            .then_some(tail)
        })
        .max_by_key(|tail| tail.len());

    if let Some(tail) = option {
        text.push(' ');
        text.push_str(&tail.join(" "));
        rest = &rest[tail.len()..];
    }

    for word in rest {
        text.push(' ');
        text.push_str(word);
    }

    text
}

/// True once the command name has been followed by a space or an option.
pub fn choosing_option(line: &str) -> bool {
    let words: Vec<_> = line.split_whitespace().collect();
    command_words(&words).is_some_and(|(_, count)| words.len() > count || line.ends_with(' '))
}

/// True when the line starts with a drawing verb.
pub fn draws(line: &str) -> bool {
    drawing(&line.split_whitespace().collect::<Vec<_>>()).is_some()
}

/// The option without its command name, for the inline buttons.
pub fn option_label(line: &str) -> &str {
    let words: Vec<_> = line.split_whitespace().collect();
    let count = command_words(&words).map_or(1, |(_, count)| count);
    line.splitn(count + 1, ' ').last().unwrap_or(line)
}

/// The shown name of the line's command, e.g. `Move` for `m 10 0 0`.
pub fn name_of(line: &str) -> &'static str {
    command_words(&line.split_whitespace().collect::<Vec<_>>())
        .map_or("", |(spec, _)| spec.names[0])
}

/// Help text for the verb being typed.
pub fn hint(line: &str) -> &'static str {
    match command_words(&line.split_whitespace().collect::<Vec<_>>()) {
        Some((spec, _)) if !spec.hint.is_empty() => spec.hint,
        _ => "Type a command · Up/Down browse · Tab completes · Enter executes · Esc cancels",
    }
}
// --8<-- [end:command-text]

// --8<-- [start:command-parse]
/// Parse one line; the error is the message to show.
pub fn parse(line: &str) -> Result<Box<dyn Action>, String> {
    if line.len() > 65536 {
        return Err("command exceeds 64 KiB".into());
    }

    let line = line.trim();

    if line.is_empty() {
        return Err("nothing typed".into());
    }

    let words: Vec<_> = line.split_whitespace().collect();
    let Some((spec, count)) = command_words(&words) else {
        return Err(format!("no command `{}`", words[0].to_ascii_lowercase()));
    };
    let verb = spec.names[0];
    let rest = &words[count..];

    if spec.arity.is_some_and(|count| rest.len() != count) {
        return Err(format!("wrong number of arguments for `{verb}`"));
    }

    (spec.parse)(verb, rest)
}

/// Clickable choices for the verb being typed.
pub fn options(line: &str) -> &'static [&'static str] {
    command_words(&line.split_whitespace().collect::<Vec<_>>())
        .map_or(&[], |(spec, _)| spec.options)
}

/// Commands or options starting with the typed text.
pub fn completions(line: &str) -> Vec<&'static str> {
    // after a space, complete the option instead
    if choosing_option(line) {
        let mut typed = canonical(line).to_ascii_lowercase();

        if line.ends_with(' ') {
            typed.push(' ');
        }

        return options(line)
            .iter()
            .copied()
            .filter(|name| name.to_ascii_lowercase().starts_with(&typed))
            .collect();
    }

    let typed = compact(line);
    let mut names: Vec<&'static str> = REGISTRY
        .iter()
        .flat_map(|verb| verb.spec().names)
        .copied()
        .collect();
    names.sort_unstable_by_key(|name| name.to_ascii_lowercase());
    names.retain(|name| compact(name).starts_with(&typed));

    // a whole alias puts its command first, so `m` stays Move beside Measure Distance
    if let Some((spec, _)) = command_words(&line.split_whitespace().collect::<Vec<_>>())
        && let Some(index) = names.iter().position(|name| *name == spec.names[0])
    {
        let name = names.remove(index);
        names.insert(0, name);
    }

    names
}

/// Every command, matching ones first.
pub fn browse(line: &str) -> Vec<&'static str> {
    if choosing_option(line) {
        return options(line).to_vec();
    }

    let typed = compact(line);
    let (matching, rest): (Vec<_>, Vec<_>) = completions("")
        .into_iter()
        .partition(|name| compact(name).starts_with(&typed));
    matching.into_iter().chain(rest).collect()
}

/// Tab or Enter on a partial line: the completed text, and whether it can run now; false leaves `Rotate x ` open for more.
pub fn accept(line: &str) -> (String, bool) {
    let line = line.trim_start(); // a stray leading space must not hide the verb

    if line.is_empty() {
        return (String::new(), true);
    }

    let choices = completions(line);
    let text = choices.first().copied().unwrap_or(line).trim();
    let words: Vec<_> = text.split_whitespace().collect();
    let typed = command_words(&words);
    let bare_verb_waits =
        typed.is_some_and(|(spec, count)| words.len() == count && spec.wait_for_option);
    let option_waits = typed.is_some_and(|(spec, count)| {
        words.len() == count + 1
            && spec.wait_after_option
            && spec
                .options
                .iter()
                .any(|option| option.eq_ignore_ascii_case(text))
    });

    if bare_verb_waits || option_waits {
        (format!("{text} "), false)
    } else {
        (text.to_owned(), true)
    }
}
// --8<-- [end:command-parse]

// --8<-- [start:command-arguments]
/// A move offset from `10 0 0`, `@10,0` or `10<45`.
pub fn offset(words: &[&str]) -> Result<[f64; 3], String> {
    let joined = words.join(" ");
    // spaces between numbers become commas
    let text = if joined.contains(',') || joined.contains('<') {
        joined.replace(' ', "")
    } else {
        words.join(",").replacen("@,", "@", 1)
    };
    let Some(typed) = coords::parse(&text) else {
        return Err(format!("`{joined}` is not an offset; try `Move 10 0 0`"));
    };

    match typed {
        coords::Typed::Absolute { x, y, z } | coords::Typed::Relative { x, y, z } => {
            Ok([x, y, z.unwrap_or(0.0)])
        }
        coords::Typed::Polar { distance, degrees } => {
            // polar in the world XY plane
            let r = degrees.to_radians();
            Ok([distance * r.cos(), distance * r.sin(), 0.0])
        }
        coords::Typed::Distance(d) => Ok([d, 0.0, 0.0]), // a bare number moves along x
    }
}

/// An axis letter and a number.
pub fn axis_and_number(
    words: &[&str],
    example: &str,
) -> Result<(crate::app::gizmo::Axis, f64), String> {
    use crate::app::gizmo::Axis;
    let axis = match words.first().map(|w| w.to_ascii_lowercase()) {
        Some(a) if a == "x" => Axis::X,
        Some(a) if a == "y" => Axis::Y,
        Some(a) if a == "z" => Axis::Z,
        _ => return Err(format!("which axis? try `{example}`")),
    };
    Ok((axis, number(words.get(1).copied(), example)?))
}

/// A finite number, or a message with the example.
pub fn number(word: Option<&str>, example: &str) -> Result<f64, String> {
    let Some(word) = word else {
        return Err(format!("missing a number; try `{example}`"));
    };

    match word.parse::<f64>() {
        Ok(v) if v.is_finite() => Ok(v),
        _ => Err(format!("`{word}` is not a number")),
    }
}

/// `On`, `Off`, or nothing for a toggle.
pub fn on_off(words: &[&str], usage: &str) -> Result<Option<bool>, String> {
    match words {
        [] => Ok(None),
        [value] if value.eq_ignore_ascii_case("on") => Ok(Some(true)),
        [value] if value.eq_ignore_ascii_case("off") => Ok(Some(false)),
        _ => Err(usage.into()),
    }
}
// --8<-- [end:command-arguments]

// --8<-- [start:command-tests]
#[cfg(test)]
mod tests;
// --8<-- [end:command-tests]
