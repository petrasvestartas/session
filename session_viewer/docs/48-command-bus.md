# 48 Command bus + Get-loop — THE interface arrives

> **Big picture.** *Phase 8.* The locked interface decision is **commands-only**: like Rhino, every
> action is a typed verb, and buttons/keys are just shortcuts that type verbs for you. That has a
> deep consequence: if every mutation is born as a command, then undo (51), macros, and scripting
> fall out of one pipeline instead of being retrofitted per feature. This lesson builds the pipeline:
> a **registry** (verb → command), a **CLI line** in the 47 overlay, and the **Get-loop** — the small
> state machine that lets a running command ask "pick a point or type a value", Rhino's signature
> interaction.

<svg viewBox="0 0 680 150" xmlns="http://www.w3.org/2000/svg" role="img" aria-label="typed text or a shortcut enters the bus, the registry resolves the verb, the command either finishes or asks the get-loop for input which can come from a click or typed text" style="max-width:100%;height:auto;font:11px ui-monospace,monospace">
  <rect x="10" y="30" width="110" height="28" fill="none" stroke="#6fb3ff"/><text x="65" y="48" fill="#d7dae0" text-anchor="middle">CLI / shortcut</text>
  <rect x="160" y="30" width="120" height="28" fill="none" stroke="#6fb3ff"/><text x="220" y="48" fill="#d7dae0" text-anchor="middle">registry: verb→cmd</text>
  <rect x="320" y="12" width="140" height="24" fill="none" stroke="#3a3a3a"/><text x="390" y="28" fill="#888" text-anchor="middle">instant → done, log</text>
  <rect x="320" y="46" width="140" height="24" fill="none" stroke="#6fb3ff"/><text x="390" y="62" fill="#d7dae0" text-anchor="middle">needs input → Get-loop</text>
  <line x1="120" y1="44" x2="158" y2="44" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah48)"/>
  <line x1="280" y1="38" x2="318" y2="26" stroke="#3a3a3a" stroke-width="1.1" marker-end="url(#ah48)"/>
  <line x1="280" y1="50" x2="318" y2="58" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah48)"/>
  <rect x="500" y="34" width="170" height="48" fill="none" stroke="#6fb3ff"/>
  <text x="585" y="52" fill="#d7dae0" text-anchor="middle">prompt: "Pick point:"</text>
  <text x="585" y="70" fill="#666" text-anchor="middle" font-size="10">fed by CLICK or TYPED text</text>
  <line x1="460" y1="58" x2="498" y2="58" stroke="#6fb3ff" stroke-width="1.3" marker-end="url(#ah48)"/>
  <text x="340" y="120" fill="#888" text-anchor="middle">every mutation from now on is born on this bus → undo/macros/scripting share one pipeline</text>
  <defs><marker id="ah48" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto"><path d="M0,0 L6,3 L0,6 Z" fill="#6fb3ff"/></marker></defs>
</svg>

## Files we touch

```
src/app/getloop.rs    # NEW — GetState + CmdStep: what a running command is waiting for
src/app/commands.rs   # NEW — the registry: verb (+ aliases) → handler; first verbs hide/show/zoom
src/ui/cli.rs         # NEW — bottom-docked input line + response log (egui)
src/state.rs          # run_command(); clicks feed the Get-loop BEFORE picking; Esc cancels
```

## Step 1 — what a command can be waiting for: `src/app/getloop.rs` (NEW)

```rust
use session_rust::Point;

/// What the interface is currently waiting for. Idle = clicks mean selection (45);
/// anything else = clicks (and typed text) are FED TO THE RUNNING COMMAND instead.
pub enum GetState {
    Idle,
    WaitingPoint { prompt: String },
    WaitingOption { prompt: String, options: Vec<String> },
}

/// What a command hands back after each feed.
pub enum CmdStep {
    Prompt(GetState),   // ask for the next input (stays active)
    Done(String),       // finished — log this message
    Cancel,             // aborted — clean up, back to Idle
}

/// A command that outlives one call: it consumes inputs until it reports Done/Cancel.
/// Instant verbs never construct one — they act in `run` and return. Multi-step commands
/// (line, move, … from 49 on) live here.
pub trait ActiveCommand {
    fn feed_point(&mut self, state: &mut crate::state::State, p: Point) -> CmdStep;
    fn feed_text(&mut self, state: &mut crate::state::State, s: &str) -> CmdStep;
}
```

## Step 2 — the registry + first verbs: `src/app/commands.rs` (NEW)

A verb resolves to a plain function; multi-step commands return an `ActiveCommand` to install. The
first three verbs are instant — they exercise the bus end to end without needing the Get-loop yet:

```rust
use crate::state::State;

/// A verb either acts immediately (returning a log line) or starts an interactive command.
pub enum Dispatch {
    Instant(String),
    Start(Box<dyn super::getloop::ActiveCommand>, super::getloop::GetState),
}

/// The whole interface, one match. Aliases live in the pattern — `h` IS `hide`.
/// (49 upgrades this to per-command options; 50 adds history + Tab-completion over these names.)
pub fn dispatch(state: &mut State, line: &str) -> Dispatch {
    let mut parts = line.trim().split_whitespace();
    let verb = parts.next().unwrap_or("");
    match verb {
        "hide" | "h" => {
            let n = state.scene.selected.len();
            state.scene.hide_selected(&mut state.gpu);              // 46's verb, now bus-born
            Dispatch::Instant(format!("hidden {n} object(s)"))
        }
        "show" => {
            state.scene.show_all(&mut state.gpu);
            Dispatch::Instant("all objects shown".into())
        }
        "zoom" | "z" | "fit" | "f" => {
            let aspect = state.gpu.config.width as f64 / state.gpu.config.height as f64;
            state.camera.fit(state.gpu.scene_min, state.gpu.scene_max, aspect);   // 15/34b
            Dispatch::Instant("zoom extents".into())
        }
        "help" | "?" => Dispatch::Instant("verbs: hide show zoom help".into()),
        "" => Dispatch::Instant(String::new()),
        other => Dispatch::Instant(format!("unknown: '{other}'  (type 'help')")),
    }
}
```

> **Why a `match`, not a `HashMap<&str, fn>`?** With commands borrowing `&mut State`, function
> pointers force every handler into the same rigid signature and the map buys nothing — the `match`
> *is* the registry, aliases are patterns, and the compiler checks exhaustiveness of nothing you
> forgot. The archive shipped exactly this shape. When commands become objects with options (49),
> the arms construct them — the dispatch point doesn't move.

## Step 3 — the CLI line: `src/ui/cli.rs` (NEW)

A bottom panel: a one-line log above an input field. It only *collects* the submitted string — 47's
rule — `State` runs it after the closure:

```rust
/// Returns Some(line) when the user pressed Enter. `log` is the last response to show.
pub fn cli_panel(ctx: &egui::Context, input: &mut String, log: &str,
                 prompt: &str) -> Option<String> {
    let mut submitted = None;
    egui::TopBottomPanel::bottom("cli").show(ctx, |ui| {
        if !log.is_empty() { ui.label(log); }
        ui.horizontal(|ui| {
            // Get-loop prompt replaces '>'
            ui.label(if prompt.is_empty() { ">" } else { prompt });
            let r = ui.add(egui::TextEdit::singleline(input).desired_width(f32::INFINITY));
            if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submitted = Some(std::mem::take(input));
                r.request_focus();                                   // stay in the CLI after Enter
            }
        });
    });
    submitted
}
```

Call it inside 47's `build_ui` closure (add `input: String`, `log: String`, `prompt: String` to
`UiState`, and a `pending_command: Option<String>` the panel fills); after the closure, `State`
drains `pending_command` into `run_command`.

## Step 4 — the bus + the feed rule: `src/state.rs`

`State` gains the loop state and one entry point:

```rust
    pub get: crate::app::getloop::GetState,                    // ← ADD, init Idle
    pub active: Option<Box<dyn crate::app::getloop::ActiveCommand>>,   // ← ADD, init None

    pub fn run_command(&mut self, line: &str) {
        use crate::app::{commands::Dispatch, getloop::GetState};
        // If a command is running, typed text FEEDS it instead of dispatching a new verb:
        if let Some(mut cmd) = self.active.take() {
            // two statements — feed borrows self, then step does
            let r = cmd.feed_text(self, line);
            self.step(r, cmd);
            return;
        }
        match crate::app::commands::dispatch(self, line) {
            Dispatch::Instant(msg) => self.ui.log = msg,
            Dispatch::Start(cmd, get) => { self.active = Some(cmd); self.set_prompt(get); }
        }
    }

    /// One place decides what a CmdStep means. set_prompt writes GetState + the CLI prompt text.
    fn step(&mut self, s: crate::app::getloop::CmdStep,
            cmd: Box<dyn crate::app::getloop::ActiveCommand>) {
        use crate::app::getloop::{CmdStep, GetState};
        match s {
            CmdStep::Prompt(get) => { self.active = Some(cmd); self.set_prompt(get); }
            CmdStep::Done(msg)   => { self.ui.log = msg; self.set_prompt(GetState::Idle); }
            CmdStep::Cancel      => {
                self.ui.log = "cancelled".into(); self.set_prompt(GetState::Idle);
            }
        }
    }
```

And the two input reroutes — this is the Get-loop actually looping:

```rust
    // in the left-click handler, BEFORE 45's selection logic:
    if matches!(self.get, GetState::WaitingPoint { .. }) {
        if let Some(hit) = self.scene.pick_ray(&ray, tol) {        // a click IS a point
            if let Some(mut cmd) = self.active.take() {
                let r = cmd.feed_point(self, hit.point);
                self.step(r, cmd);
            }
        }
        // never falls through to selection
        return;
    }

    // in the key handler:
    Key::Named(NamedKey::Escape) => {
        self.active = None; self.set_prompt(GetState::Idle); self.ui.log = "cancelled".into();
    }
```

(When nothing is hit, intersect the ray with the work plane `z = 0` — 41 Step 3's formula — so
clicking empty grid still yields a point. That's what makes drawing on the grid possible in 57.)

## Step 5 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Select two boxes, type **`hide`** ⏎ → they vanish, the log reads `hidden 2 object(s)`. Type
  **`show`** ⏎ → back. **`h`** does the same as `hide` — aliases are patterns.
- **`zoom`** ⏎ → same as pressing F. **`frobnicate`** ⏎ → `unknown: 'frobnicate'  (type 'help')` —
  friendly, no panic.
- Typing in the CLI must **not** orbit the camera or trigger H/F shortcuts — that's 47's `consumed`
  gate protecting the whole interface.
- The Get-loop plumbing (prompt display, click-feeds-point, Esc-cancels) is wired but no verb uses it
  yet — 49's multi-step command is its first customer. The `return` before selection logic is the
  contract to remember: **while a command runs, clicks belong to the command.**

## Recap

```
Ch 47: egui overlay — panels, input contract.
Ch 48: THE BUS. commands::dispatch(state, line) — a match IS the registry, aliases are patterns;
       instant verbs act and log (hide/show/zoom — 46's Scene verbs, now bus-born); multi-step verbs
       return an ActiveCommand + a GetState prompt. getloop::GetState { Idle / WaitingPoint /
       WaitingOption } is what the interface is waiting for; CmdStep { Prompt / Done / Cancel } is
       what a command says back. INPUT RULES: typed text feeds the active command before
       dispatching; a click while WaitingPoint feeds the command (never selection); empty-space
       clicks intersect z=0 so the grid is clickable; Esc cancels. ui/cli.rs = bottom panel,
       collect-then-apply.
       From here on, EVERY mutation is born as a command — undo (51) rides this for free.
```

Edited: `app/getloop.rs` (NEW — `GetState`, `CmdStep`, `ActiveCommand`), `app/commands.rs` (NEW —
`dispatch` + hide/show/zoom/help), `ui/cli.rs` (NEW — bottom panel), `state.rs` (`get`/`active`,
`run_command`/`step`, click + Esc rerouting).

## Next

`49-command-options.md` — commands grow **options** rendered in the prompt line, Rhino-style:
`Line (From, Snap=On):` — toggles, numbers, and lists clickable or typeable mid-command; chained
prompts (from → to) with Esc = cancel, Enter = accept default, and one-step-back.
