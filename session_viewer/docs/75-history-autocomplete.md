# 75 History & autocomplete — the CLI grows muscle memory

> **Big picture.** *Phase 8.* A command line lives or dies on typing speed: ↑ to repeat, Tab to
> finish a verb, one-letter aliases. Rhino users run `l ⏎` a hundred times a day without thinking.
> This is a small lesson by design — three quality-of-life features on 61's CLI, no new architecture
> — but it's the difference between an interface you *demo* and one you *use*.

## Files we touch

```
src/app/commands.rs   # VERBS + ALIASES tables — one source of truth for dispatch AND completion
src/ui/cli.rs         # ↑/↓ history walk, Tab completion (keys intercepted before the TextEdit)
src/state.rs          # cli_history: Vec<String> pushed by run_command
```

## Step 1 — the verb tables: `src/app/commands.rs`

61's `match` arms embedded the aliases in patterns; completion needs them *as data*. One pair of
tables feeds both — the `match` now consults `ALIASES` first, so a name exists in exactly one place:

```rust
/// Every canonical verb, for Tab-completion and `help`. Grows with each new command lesson.
pub const VERBS: &[&str] = &["hide", "show", "zoom", "probe", "help"];

/// alias → canonical. Dispatch resolves through this BEFORE matching, so `match` arms only
/// ever name canonical verbs (drop the `| "h"`-style patterns from 61).
pub const ALIASES: &[(&str, &str)] = &[("h", "hide"), ("z", "zoom"), ("fit", "zoom"), ("f", "zoom"), ("?", "help")];

pub fn resolve(verb: &str) -> &str {
    ALIASES.iter().find(|(a, _)| *a == verb).map(|(_, c)| *c).unwrap_or(verb)
}
```

and at the top of `dispatch`, replace 61's `let verb = parts.next().unwrap_or("");` with:

```rust
    let verb = resolve(parts.next().unwrap_or(""));   // ← aliases resolved once, here
```

Because `verb` is now always canonical, strip the alias patterns from every arm — find each
match arm in `dispatch` and drop the `| "…"` alternatives, e.g.:

```rust
    // 61:                          →  now:
    "hide" | "h" => …                  "hide" => …
    "zoom" | "z" | "fit" | "f" => …    "zoom" => …
    "help" | "?" => …                  "help" => …
```

## Step 2 — history: `src/state.rs`

`run_command` records every non-empty line; the CLI walks the record. Two guards keep the record
honest: consecutive duplicates collapse (running `zoom` ten times is one entry), and the vec is
capped — a long session must not grow it without bound:

```rust
    pub cli_history: Vec<String>,        // ← ADD to State (init empty in State::new)
                                         //   NB: named cli_history, not history — lesson 76 adds
                                         //   `pub history: History` (undo stack) to this same struct.

    // first line of run_command:
    const CLI_HISTORY_MAX: usize = 500;   // cap: oldest fall off (one O(n) drain per 500 commands)
    if !line.trim().is_empty() && self.cli_history.last().map_or(true, |l| l.as_str() != line) {
        self.cli_history.push(line.to_string());
        if self.cli_history.len() > CLI_HISTORY_MAX {
            self.cli_history.drain(..self.cli_history.len() - CLI_HISTORY_MAX);
        }
    }
```

## Step 3 — ↑ / ↓ / Tab in the panel: `src/ui/cli.rs`

egui's `TextEdit` consumes arrow keys for cursor movement, so intercept **before** building the
widget. The panel gains a cursor into history (`None` = live line) and a stash of what was typed
before the walk began:

First, two new fields on `UiState` (find `pub input: String,` in `struct UiState` and add these
after it; then add `hist_cursor: None, stash: String::new(),` to the `UiState { … }` literal in
`State::new`, 60):

```rust
    pub hist_cursor: Option<usize>,   // index into cli_history while walking; None = editing a fresh line
    pub stash: String,                // the fresh line saved when ↑ starts, restored past the end
```

Now replace 61's `cli_panel` wholesale with the version below. Note the `history: &[String]`
param is just a borrow of the caller's `cli_history` — the field rename from Step 2 only lives on
`State`; inside this function it reads as `history`:

```rust
pub fn cli_panel(ctx: &egui::Context, input: &mut String, hist_cursor: &mut Option<usize>,
                 stash: &mut String, history: &[String], log: &str,
                 prompt: &str) -> Option<String> {
    let mut submitted = None;
    // A recalled line must land with the cursor at its END — egui keeps the old position
    // (start of line) after a programmatic replace, so walk the TextEditState explicitly.
    let cursor_to_end = |ctx: &egui::Context, input: &String| {
        let id = egui::Id::new("cli_input");
        let mut st = egui::text_edit::TextEditState::load(ctx, id).unwrap_or_default();
        st.cursor.set_char_range(Some(egui::text::CCursorRange::one(
            egui::text::CCursor::new(input.chars().count()))));   // CHARS, not bytes
        st.store(ctx, id);
    };
    egui::TopBottomPanel::bottom("cli").show(ctx, |ui| {
        if !log.is_empty() { ui.label(log); }

        // Intercept BEFORE the TextEdit exists this frame — else it eats the keys.
        let (up, down, tab) = ui.input_mut(|i| (
            i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp),
            i.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown),
            i.consume_key(egui::Modifiers::NONE, egui::Key::Tab),
        ));
        if up && !history.is_empty() {
            let next = match *hist_cursor {
                None => { *stash = std::mem::take(input); history.len() - 1 }     // start walking
                Some(i) => i.saturating_sub(1),
            };
            *hist_cursor = Some(next);
            *input = history[next].clone();
            cursor_to_end(ctx, input);
        }
        if down {
            match *hist_cursor {
                Some(i) if i + 1 < history.len() => {
                    *hist_cursor = Some(i + 1);
                    *input = history[i + 1].clone();
                    cursor_to_end(ctx, input);
                }
                // past the end → fresh line
                Some(_) => { *hist_cursor = None; *input = std::mem::take(stash);
                             cursor_to_end(ctx, input); }
                None => {}
            }
        }
        if tab && !input.is_empty() {
            // prefix-complete over canonical verbs — a UNIQUE match wins, nothing else fires
            let hits: Vec<&str> = crate::app::commands::VERBS.iter().copied()
                .filter(|v| v.starts_with(input.as_str())).collect();
            match hits.len() {
                1 => { *input = hits[0].to_string(); input.push(' '); cursor_to_end(ctx, input); }
                // ambiguous ("h" → help/hide) or no match: leave the line as typed —
                // the user types one more letter (see Verify)
                _ => {}
            }
        }

        ui.horizontal(|ui| {
            ui.label(if prompt.is_empty() { ">" } else { prompt });
            let r = ui.add(egui::TextEdit::singleline(input).id(egui::Id::new("cli_input"))
                .desired_width(f32::INFINITY));
            if r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                submitted = Some(std::mem::take(input));
                *hist_cursor = None;                                   // a submit ends any walk
                r.request_focus();
            }
        });
    });
    submitted
}
```

Inside 61's `build_ui` closure (where `cli_panel` is already called and its return stashed in
`ui_state.pending_command` — the closure must not touch `State`), widen the arguments. `hist_cursor`/
`stash` come from `ui_state`; the history record lives on `State`, so add a `cli_history: &[String]`
parameter to `build_ui` **and** pass `&self.cli_history` at its call site in `render()` — the closure then
borrows it (miss the call site → E0061; note the type is `&[String]`, not `&History` which doesn't exist
until 56):

```rust
        // in build_ui's closure — same pending_command drain as 53, wider cli_panel args:
        let out = cli_panel(ctx, &mut ui_state.input, &mut ui_state.hist_cursor,
                            &mut ui_state.stash, cli_history, &ui_state.log, &ui_state.prompt);
        if let Some(line) = out { ui_state.pending_command = Some(line); }
```

> **Two egui realities worth knowing.** (1) `consume_key` *removes* the key event, so the TextEdit
> built afterwards never sees it — that's the whole interception trick, and it's frame-order
> dependent: intercept first, build second. (2) After programmatically replacing `input`, egui keeps
> the old cursor position (start of line) — which is why every recall above runs `cursor_to_end`:
> `TextEditState::load` → `set_char_range` → `store` against the TextEdit's explicit
> `Id::new("cli_input")`. The index is in CHARS (`chars().count()`), not bytes.

## Step 4 — verify

```bash
cd session_viewer && trunk serve   # http://localhost:8770
```

- Run `zoom`, `hide`, `probe` (Esc out). Press **↑ ↑ ↑** → `probe`, `hide`, `zoom` appear in the
  input, newest first. **↓** walks back down and past the end restores whatever you had half-typed
  (the stash).
- Type `pr` **Tab** → completes to `probe ` (trailing space, ready for options). Type `s` **Tab** →
  `show` (unique). Type `h` **Tab** → `help`/`hide` are both candidates — nothing completes
  (ambiguous); type one more letter.
- `h ⏎` still hides — aliases now resolve through the table, and `help` prints the canonical verb
  list straight from `VERBS` (update its arm to `format!("verbs: {}", VERBS.join(" "))`).

## Recap

```
Ch 62: options + multi-step — the conversation grammar.
Ch 63: MUSCLE MEMORY. VERBS + ALIASES as const tables — one source of truth for dispatch (resolve()
       before the match), Tab-completion, and help. History: cli_history: Vec<String> on State
       (named to dodge 64's undo `history`), pushed by run_command — capped at 500 entries,
       consecutive duplicates collapsed; the CLI walks it with ↑/↓ (cursor Option<usize>, live line stashed and restored
       past the end). Tab prefix-completes canonical verbs, unique-match-only. egui gotchas: keys
       must be consume_key'd BEFORE the TextEdit is built (frame order), and programmatic input
       replacement leaves the cursor at line start — every recall runs cursor_to_end
       (TextEditState + set_char_range, CHAR index) against the field's explicit id.
```

Edited: `app/commands.rs` (`VERBS`, `ALIASES`, `resolve`), `ui/cli.rs` (↑/↓ walk + stash, Tab
completion), `state.rs` (`cli_history`).

## Next

`76-delete-undo.md` — the first *destructive* mutation, and with it the pattern the whole editor
stands on: `trait Command { apply / revert }` + done/undone stacks. Delete an object, Ctrl+Z brings
it back byte-identical — and the trap the archive fell into (an `UndoAction` enum instead of a trait)
is called out so it's never repeated.
