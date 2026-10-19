# 18.4.8 · Teach named and bare Element values

**Chapter 18 · Lighting, render quality and comparison · Section 18.4 · Wood Elements and archives** · [Contents](contents.md)

**Typing: 15–30 minutes.** [Estimate](typing-load.md).

Teach named and bare Element values.

## Type

Continue from [Capture original picks while the tool runs](92e-capture-original-picks-while-the-tool-runs.md). [Save or recover your work](recovery.md).

### 1. `src/element_draft.rs`

Adapt the maintained Element parameter word recipe without changing its semantics.

<details>
<summary>Locate the existing block</summary>

```rust
 pub fn prompt(&self)->String{if let Some(i)=self.asking{format!("{} <{}>: type a number · Enter keeps it",self.params[i].name,self.params[i].value)}else{format!("{} · pick original sources ({} picked) · Enter creates · Esc cancels",self.kind.name(),self.picked.len())}}
 pub fn buttons(&self)->Vec<(String,String)>{self.params.iter().map(|p|(format!("{}={}",p.name,p.value),p.name.into())).chain([("Create".into(),"".into()),("Cancel".into(),"Escape".into())]).collect()}

}
```

</details>

Edit the existing block to match this code; keep unchanged lines:

```rust
--8<-- "journey/code/92f-teach-named-and-bare-element-values-01.rs"
```

## Run and check

Run this from `session_viewer` to select the fixed dependency versions and install supplied binary assets. It updates Cargo.lock and preserves the previous lock. It does not write implementation code:

```sh
npm --prefix ../session_tests run course -- dependencies 18.4.8
```

In your project:

```sh
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/`. Keep an existing Trunk server running; saving rebuilds it.



### Recorded visual result

**Previous native render — current verification pending.**

Actual original curved and flat surfaces use the same black outline width.

![Native renderer output for teach named and bare element values.](../screenshots/journey/91i-prove-actual-outlines-and-original-ownership.png)

*This recorded image predates a source or checker change. Rebuild this checkpoint before using it as current evidence; browser controls require separate acceptance.*

This reference image is shared with [18.3.10 · Prove actual outlines and original ownership](91i-prove-actual-outlines-and-original-ownership.md). It does not demonstrate a new feature by itself.

<details>
<summary>Code explanation and diagram</summary>



original ordered picks → current Wood builders → typed Element and attributes → one history edit.

![Teach named and bare Element values.](../illustrations/92f-teach-named-and-bare-element-values.svg)

What does Height without a number do?

It asks for that value; Enter keeps the current number. Name=value sets it directly, and bare numbers fill parameters cyclically.

Study estimate, including typing and experiments: 0.5–1 hours.

</details>

<details>
<summary>Optional experiment</summary>

Try Width=100 Height150, then ask for Height and keep it with Enter.

</details>

<details>
<summary>Check and save your work</summary>

Restore experimental edits, then run from `session_viewer`:

```sh
npm --prefix ../session_tests run course -- check 18.4.8
npm --prefix ../session_tests run course -- save 18.4.8
```

Source comparison leaves your project untouched. [Save and recovery instructions](recovery.md).

</details>

<details>
<summary>Viewer coverage and verification</summary>



The supplied browser contract uses the command dock to create a beam from original lines, adjust named values, show features, measure bounds and Undo/Redo. It is syntaxchecked and requires an actual enabled browser. Native fixtures do not constitute browser acceptance.

[Full validation scope](release.md).

To reproduce the scripted acceptance of the reference checkpoint, run from `session_viewer` with the [course bundle server](release.md#reproduce) running on port 8781:

```sh
npm --prefix ../session_tests run course -- capture 18.4.8
```

This uses the verified reference bundle; it does not check or change your typed project.

</details>
