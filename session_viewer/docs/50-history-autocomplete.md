# 50 History & autocomplete — the CLI grows muscle memory

> **Big picture.** *Phase 8.* A command line lives or dies on typing speed: ↑ to repeat, Tab to
> finish a verb, one-letter aliases. Rhino users run `l ⏎` a hundred times a day without thinking.
> This is a small lesson by design — three quality-of-life features on 48's CLI, no new architecture
> — but it's the difference between an interface you *demo* and one you *use*.

## Files we touch

```
src/app/commands.rs   # VERBS + ALIASES tables — one source of truth for dispatch AND completion
src/ui/cli.rs         # ↑/↓ history walk, Tab completion (keys intercepted before the TextEdit)
src/state.rs          # history: Vec<String> pushed by run_command
```

## Step 1 — the verb tables: `src/app/commands.rs`

48's `match` arms embedded the aliases in patterns; completion needs them *as data*. One pair of
tables feeds both — the `match` now consults `ALIASES` first, so a name exists in exactly one place:

```rust
/// Every canonical verb, for Tab-completion and `help`. Grows with each new command lesson.
pub const VERBS: &[&str] = &["hide", "show", "zoom", "probe", "help"];

/// alias → canonical. Dispatch resolves through this BEFORE matching, so `match` arms only
/// ever name canonical verbs (drop the `| "h"`-style patterns from 48).
pub const ALIASES: &[(&str, &str)] = &[("h", "hide"), ("z", "zoom"), ("f", "zoom"), ("?", "help")];

pub fn resolve(verb: &str) -> &str {
    ALIASES.iter().find(|(a, _)| *a == verb).map(|(_, c)| *c).unwrap_or(verb)
}
```

and at the top of `dispatch`:

```rust
    let verb = resolve(parts.next().unwrap_or(""));   // ← aliases resolved once, here
```

## Step 2 — history: `src/state.rs`

`run_command` records every non-empty line; the CLI walks the record:

```rust
    pub history: Vec<String>,            // ← ADD to State (init empty)

    // first line of run_command:
    if !line.trim().is_empty() { self.history.push(line.to_string()); }
```

## Step 3 — ↑ / ↓ / Tab in the panel: `src/ui/cli.rs`

egui's `TextEdit` consumes arrow keys for cursor movement, so intercept **before** building the
widget. The panel gains a cursor into history (`None` = live line) and a stash of what was typed
before the walk began:

```rust
/// Extra CLI state, lives beside `input` in UiState:
///   hist_cursor: Option<usize>   — index into history while walking; None = editing a fresh line
///   stash: String                — the fresh line saved when ↑ starts, restored past the end

pub fn cli_panel(ctx: &egui::Context, input: &mut String, hist_cursor: &mut Option<usize>,
                 stash: &mut String, history: &[String], log: &str, prompt: &str) -> Option<String> {
    let mut submitted = None;
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
        }
        if down {
            match *hist_cursor {
                Some(i) if i + 1 < history.len() => { *hist_cursor = Some(i + 1); *input = history[i + 1].clone(); }
                Some(_) => { *hist_cursor = None; *input = std::mem::take(stash); }   // past the end → fresh line
                None => {}
            }
        }
        if tab && !input.is_empty() {
            // prefix-complete over canonical verbs; unique match wins, ambiguity shows candidates
            let hits: Vec<&str> = crate::app::commands::VERBS.iter().copied()
                .filter(|v| v.starts_with(input.as_str())).collect();
            match hits.len() {
                1 => { *input = hits[0].to_string(); input.push(' '); }
                n if n > 1 => { /* show candidates in the log line via return channel, or ui.label */ }
                _ => {}
            }
        }

        ui.horizontal(|ui| {
            ui.label(if prompt.is_empty() { ">" } else { prompt });
            let r = ui.add(egui::TextEdit::singleline(input).desired_width(f32::INFINITY));
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

> **Two egui realities worth knowing.** (1) `consume_key` *removes* the key event, so the TextEdit
> built afterwards never sees it — that's the whole interception trick, and it's frame-order
> dependent: intercept first, build second. (2) After programmatically replacing `input`, egui keeps
> the old cursor position (start of line). The polish — jumping the cursor to the end via
> `egui::text_edit::TextEditState::load / set_ccursor_range` — is real but noisy; add it once the
> basics feel right. Recalled lines run fine either way.

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
Ch 49: options + multi-step — the conversation grammar.
Ch 50: MUSCLE MEMORY. VERBS + ALIASES as const tables — one source of truth for dispatch (resolve()
       before the match), Tab-completion, and help. History: Vec<String> on State, pushed by
       run_command; the CLI walks it with ↑/↓ (cursor Option<usize>, live line stashed and restored
       past the end). Tab prefix-completes canonical verbs, unique-match-only. egui gotchas: keys
       must be consume_key'd BEFORE the TextEdit is built (frame order), and programmatic input
       replacement leaves the cursor at line start (TextEditState polish, optional).
```

Edited: `app/commands.rs` (`VERBS`, `ALIASES`, `resolve`), `ui/cli.rs` (↑/↓ walk + stash, Tab
completion), `state.rs` (`history`).

## Next

`51-delete-undo.md` — the first *destructive* mutation, and with it the pattern the whole editor
stands on: `trait Command { apply / revert }` + done/undone stacks. Delete an object, Ctrl+Z brings
it back byte-identical — and the trap the archive fell into (an `UndoAction` enum instead of a trait)
is called out so it's never repeated.
