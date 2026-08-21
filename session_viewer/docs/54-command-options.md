# 54 Command options — modal, multi-step, Rhino-style

> **Big picture.** *Phase 8.* Real commands are conversations: `Line` asks *from*, then *to*; along
> the way you can flip `Snap=On`, type a number, or step back one prompt. Rhino renders that whole
> conversation in one prompt line — `Line (From, Snap=On):` — and that's the shape every drawing tool
> (62–63) will reuse. This lesson teaches the Get-loop that grammar with a deliberately geometry-free
> dummy command, so 62 can focus on geometry, not plumbing.

<svg viewBox="0 0 680 110" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="a running command chains prompts: first point, second point, done; Esc cancels, Enter accepts default, back steps to the previous prompt; options render in the prompt line" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="180" height="28" fill="none" stroke="#6fb3ff"/><text x="100" y="48" fill="#d7dae0" text-anchor="middle">probe (Rounded=On): pick A</text>
  <rect x="240" y="30" width="140" height="28" fill="none" stroke="#6fb3ff"/><text x="310" y="48" fill="#d7dae0" text-anchor="middle">pick B</text>
  <rect x="430" y="30" width="150" height="28" fill="none" stroke="#3a3a3a"/><text x="505" y="48" fill="#888" text-anchor="middle">Done: "dist = 812.5"</text>
  <line x1="190" y1="44" x2="238" y2="44" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah49)"/>
  <line x1="380" y1="44" x2="428" y2="44" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah49)"/>
  <path d="M310,58 Q 200,90 105,60" fill="none" stroke="#666" stroke-dasharray="3 3" marker-end="url(#ah49g)"/>
  <text x="210" y="92" fill="#666" font-size="10">'back' → previous prompt</text>
  <text x="505" y="80" fill="#666" font-size="10">Esc anywhere → Cancel</text>
  <text x="100" y="18" fill="#888" font-size="10">typing 'rounded=off' mid-command flips the option</text>
  <defs>
    <marker id="ah49" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker>
    <marker id="ah49g" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#666"/></marker>
  </defs>
</svg>

## Files we touch

```
src/app/getloop.rs    # CmdOption (toggle/number/list) + prompt_line() rendering
src/app/commands.rs   # the dummy: ProbeCmd — two points, one option, back/cancel/default
src/state.rs          # feed_text parses options + 'back' BEFORE reaching the command
```

## Step 1 — options as data: `src/app/getloop.rs`

Options render in the prompt and parse from typed text — one type serves both:

```rust
/// An option a running command exposes mid-conversation. Rendered in the prompt line;
/// set by typing `name=value` (case-insensitive) while the command runs.
pub enum CmdOption {
    Toggle { name: &'static str, value: bool },
    Number { name: &'static str, value: f64 },
    List   { name: &'static str, choices: Vec<&'static str>, current: usize },
}

impl CmdOption {
    pub fn label(&self) -> String {
        match self {
            CmdOption::Toggle { name, value } =>
                format!("{name}={}", if *value { "On" } else { "Off" }),
            CmdOption::Number { name, value } => format!("{name}={value}"),
            CmdOption::List { name, choices, current } => format!("{name}={}", choices[*current]),
        }
    }
    /// `probe (Rounded=On, Digits=1):` — the Rhino prompt shape.
    pub fn prompt_line(verb: &str, opts: &[CmdOption], ask: &str) -> String {
        if opts.is_empty() { return format!("{verb}: {ask}"); }
        let list: Vec<String> = opts.iter().map(|o| o.label()).collect();
        format!("{verb} ({}): {ask}", list.join(", "))
    }
}
```

Extend the trait with an options hook (default: none) — the Get-loop reads it to build the prompt:

```rust
pub trait ActiveCommand {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep;
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep;
    fn options(&mut self) -> &mut [CmdOption] { &mut [] }      // ← ADD
    /// One prompt backwards. The default re-asks the CURRENT prompt — the right answer for a
    /// stateless command; commands with a stage (ProbeCmd, 62's tools) override it to step back.
    fn back(&mut self) -> CmdStep { CmdStep::Prompt(self.prompt()) }   // ← ADD
    fn prompt(&self) -> GetState;                             // ← ADD — the current prompt, so
                                                              //   state.rs can re-render after an
                                                              //   option flip without advancing
}
```

## Step 2 — universal text handling: `src/state.rs`

`name=value` and `back` are *grammar*, not per-command logic — intercept them in `run_command`
before the text ever reaches the command. First widen 53's `use` line so `CmdOption` is in scope:

```rust
        use crate::app::{commands::Dispatch, getloop::{GetState, CmdOption}};   // ← add CmdOption
```

Then **find the `if let Some(mut cmd) = self.active.take()` block you wrote in 53** (the two-line
`feed_text` + `step` version) and replace its body with the grammar-aware one:

```rust
        if let Some(mut cmd) = self.active.take() {
            let t = line.trim().to_ascii_lowercase();
            // grammar first: back / name=value are handled for EVERY command identically
            if t == "back" {
                let r = cmd.back();
                self.step(r, cmd);
                return;
            }
            if let Some((k, v)) = t.split_once('=') {
                let mut known = false;
                let mut err: Option<String> = None;
                for o in cmd.options() {
                    let name = match o {
                        CmdOption::Toggle { name, .. } |
                        CmdOption::Number { name, .. } |
                        CmdOption::List { name, .. } => *name,
                    };
                    if !name.eq_ignore_ascii_case(k) { continue; }
                    known = true;
                    match o {
                        CmdOption::Toggle { value, .. } =>
                            *value = matches!(v, "on" | "true" | "1" | "yes"),
                        CmdOption::Number { value, .. } => match v.parse() {
                            Ok(n) => *value = n,
                            Err(_) => err = Some(format!("{k}: '{v}' is not a number")),
                        },
                        CmdOption::List { choices, current, .. } =>
                            match choices.iter().position(|c| c.eq_ignore_ascii_case(v)) {
                                Some(i) => *current = i,
                                None =>
                                    err = Some(format!("{k}: expected one of {}", choices.join("|"))),
                            },
                    }
                }
                // NEVER silent: a typo'd option that vanishes reads as "accepted" — the worst lie
                self.ui.log = err.unwrap_or_else(|| if known { format!("{k}={v}") }
                    else { format!("unknown option '{k}'") });
                // option set; SAME prompt — the command didn't advance
                self.active = Some(cmd);
                self.refresh_prompt();
                return;
            }
            let r = cmd.feed_text(self, line);    // everything else: the command's own business
            self.step(r, cmd);
            return;
        }
```

`refresh_prompt` re-renders `prompt_line` from the command's current options — the prompt is always
a pure function of the command's state. It can't build the prompt itself (the verb and the "which
point" text live in `ProbeCmd`'s private `ask()`), so it asks the command via the new `prompt()`
trait method. Add it to `impl State` next to `set_prompt`:

```rust
    fn refresh_prompt(&mut self) {
        // borrow ends after prompt(); then set_prompt takes &mut self freely
        let get = match &self.active {
            Some(cmd) => cmd.prompt(),
            None => return,
        };
        self.set_prompt(get);
    }
```

## Step 3 — the dummy command: `src/app/commands.rs`

`probe`: pick two points, print their distance. No geometry created, no Session touched — pure
Get-loop exercise. Its structure — a `stage` field, prompts derived from it — is the template every
real tool copies:

```rust
use session_rust::Point;
use super::getloop::{ActiveCommand, CmdOption, CmdStep, GetState};

pub struct ProbeCmd {
    a: Option<Point>,                 // stage 0: None → asking for A; stage 1: Some → asking for B
    opts: [CmdOption; 1],
}

impl ProbeCmd {
    pub fn start() -> (Box<dyn ActiveCommand>, GetState) {
        let cmd = ProbeCmd { a: None, opts: [CmdOption::Toggle { name: "Rounded", value: true }] };
        (Box::new(cmd), GetState::WaitingPoint {
            prompt: "probe (Rounded=On): pick first point".into() })
    }
    fn ask(&self) -> CmdStep { CmdStep::Prompt(self.prompt()) }
}

impl ActiveCommand for ProbeCmd {
    fn feed_point(&mut self, _state: &mut crate::state::State, p: Point) -> CmdStep {
        match self.a.take() {
            None => { self.a = Some(p); self.ask() }                       // A stored → ask for B
            Some(a) => {
                // kernel signature: distance(&Point, Option<f64> min-clamp)
                let d = a.distance(&p, None);
                let rounded = matches!(self.opts[0], CmdOption::Toggle { value: true, .. });
                CmdStep::Done(
                    if rounded { format!("dist = {d:.1}") } else { format!("dist = {d}") })
            }
        }
    }
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep {
        // Enter on an empty line = accept default. probe's default second point: the origin.
        if s.trim().is_empty() {
            if self.a.is_some() { return self.feed_point(state, Point::new(0.0, 0.0, 0.0)); }
            return self.ask();
        }
        // 'x,y,z' typed instead of clicked — a point is a point, whichever way it arrives:
        let nums: Vec<f64> = s.split(',').filter_map(|t| t.trim().parse().ok()).collect();
        if nums.len() == 3 { return self.feed_point(state, Point::new(nums[0], nums[1], nums[2])); }
        self.ask()
    }
    fn options(&mut self) -> &mut [CmdOption] { &mut self.opts }
    fn prompt(&self) -> GetState {
        let what = if self.a.is_none() { "pick first point" } else { "pick second point" };
        GetState::WaitingPoint { prompt: CmdOption::prompt_line("probe", &self.opts, what) }
    }
    fn back(&mut self) -> CmdStep {
        // forget A → back to prompt 1
        self.a = None;
        self.ask()
    }
}
```

Register it — one new arm in `dispatch`:

```rust
        "probe" => { let (cmd, get) = ProbeCmd::start(); Dispatch::Start(cmd, get) }
```

(`Point::distance(&other, None)` is a kernel method — the second arg is an optional min-clamp, same as
Step 3's call; typed `x,y,z` points and clicked points converge on the
same `feed_point` — that equivalence *is* the Get-loop's promise.)

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- **`probe`** ⏎ → prompt reads `probe (Rounded=On): pick first point`. Click the grid → prompt
  advances to `pick second point`. Click again → `dist = 812.5`, prompt back to `>`.
- Mid-command type **`rounded=off`** ⏎ → prompt re-renders `(Rounded=Off)`, still asking the same
  point; finish → full-precision distance. Type **`120,45,0`** ⏎ instead of clicking → same as a
  click at that point.
- **`back`** ⏎ after the first pick → asking for the *first* point again. **Esc** anywhere →
  `cancelled`. **Enter** on the empty line at the second prompt → accepts the default (origin).
- Type **`ronded=off`** ⏎ (a typo) → the log answers `unknown option 'ronded'` instead of eating
  it — silence reads as "accepted", which is the bug the feedback line exists to kill.
- Every behaviour above came from `state.rs`'s grammar layer or the `stage` pattern — the command
  itself is ~40 lines. That ratio is the point: tools stay small because the loop owns the grammar.

## Recap

```
Ch 53: the bus — verbs dispatch, instant commands work end to end.
Ch 54: THE CONVERSATION. CmdOption { Toggle / Number / List } renders into the Rhino prompt shape —
       `probe (Rounded=On): pick first point` — via prompt_line; typed `name=value` is parsed by the
       GRAMMAR layer in run_command (every command gets options for free), as is `back` (one prompt
       backwards, default = re-ask the current prompt, staged commands override). Unknown names and
       unparseable values land in the log — never eaten. ProbeCmd is the canonical multi-step template: a
       stage field (Option<Point>), ask() derives the prompt from state, feed_point advances, typed
       `x,y,z` converges on feed_point, empty Enter = default, Esc = cancel (53). Geometry-free on
       purpose — 62's real tools copy the shape and only swap what Done does.
```

Edited: `app/getloop.rs` (`CmdOption`, `prompt_line`, trait gains `options()`/`back()`/`prompt()`),
`app/commands.rs` (`ProbeCmd` + registry arm), `state.rs` (grammar layer: `back`, `name=value`,
`refresh_prompt`).

## Next

`55-history-autocomplete.md` — the CLI grows muscle memory: ↑/↓ recall previous commands, Tab
prefix-completes verbs, and an alias table (`l` → `line`) formalizes what 53's match patterns started.
