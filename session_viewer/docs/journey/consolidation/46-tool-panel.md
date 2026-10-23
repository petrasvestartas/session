# 7.1 · Active tools own their command input

**Section 5: Interactive Drawing, Snapping, Gumball Handles, and Scene Hierarchy** · **Lesson 7.1** (Redraft `5.1`) · **1 checkpoint (`46-tool-panel`)** · **Study time: 0.5–1 h**

> **What You Will Learn & Build:** Keep active values and tool controls inside the existing GPU command panel.

## 1. Visual WebGPU Explanation & Architecture (Black & White Schematics)

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="256" viewBox="0 0 840 256" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="248" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Chapter 7 Visual Explanation: Construction-Plane Ray Intersection, Snap Priority & 3D Gumball Gizmo</text>
  <text x="20" y="45" font-size="11" fill="#111111">Left: Intersecting cursor ray with construction plane (P0, N). Center: Snap priority radius. Right: Perspective-scaled 3D Gumball.</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    <!-- Panel A: Construction Plane Ray Hit -->
    <rect x="18" y="64" width="265" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="150" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">A. Ray–Plane Hit (Construction Grid)</text>
    <!-- Tilted 3D plane parallelogram -->
    <polygon points="40,165 180,125 260,155 120,195" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <line x1="75" y1="155" x2="155" y2="185" stroke="#111111" stroke-width="0.75" stroke-dasharray="2,2"/>
    <line x1="110" y1="145" x2="190" y2="175" stroke="#111111" stroke-width="0.75" stroke-dasharray="2,2"/>
    <line x1="145" y1="135" x2="225" y2="165" stroke="#111111" stroke-width="0.75" stroke-dasharray="2,2"/>
    <!-- Plane normal N -->
    <line x1="150" y1="160" x2="150" y2="104" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>
    <text x="164" y="112" font-size="9.5" font-weight="700" fill="#111111">N</text>
    <!-- Ray from top-left hitting plane at (150, 160) -->
    <line x1="55" y1="96" x2="146" y2="157" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <circle cx="150" cy="160" r="4.5" fill="#111111"/>
    <text x="90" y="112" font-size="9.5" font-weight="700" fill="#111111">Ray O+t·D</text>
    <text x="150" y="214" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">t = dot(P0 - O, N) / dot(D, N)</text>
    <text x="150" y="229" text-anchor="middle" font-size="9" fill="#111111">Preview rubber-band updates before commit</text>

    <line x1="283" y1="154" x2="305" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel B: Snap Priority Radius -->
    <rect x="307" y="64" width="240" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="427" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">B. Snap Priority Within Radius</text>
    <!-- Snap radius circle -->
    <circle cx="427" cy="144" r="44" fill="#ffffff" stroke="#111111" stroke-width="1.2" stroke-dasharray="4,3"/>
    <!-- Geometry segment passing through snap circle -->
    <line x1="365" y1="168" x2="465" y2="120" stroke="#111111" stroke-width="2"/>
    <!-- Endpoint square (winner #1) -->
    <rect x="459" y="114" width="12" height="12" fill="url(#hatch)" stroke="#111111" stroke-width="1.75"/>
    <!-- Midpoint triangle (#2) -->
    <polygon points="415,137 409,149 421,149" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="480" y="123" font-size="9" font-weight="700" fill="#111111">1. End</text>
    <text x="382" y="146" font-size="9" font-weight="700" fill="#111111">2. Mid</text>
    <text x="427" y="208" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Priority: Endpoint &gt; Midpoint</text>
    <text x="427" y="223" text-anchor="middle" font-size="9" fill="#111111">&gt; Curve Nearest &gt; Grid Step</text>

    <line x1="547" y1="154" x2="569" y2="154" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <!-- Panel C: 3D Gumball Gizmo (widget.wgsl) -->
    <rect x="571" y="64" width="251" height="180" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="696" y="82" text-anchor="middle" font-size="11.5" font-weight="700" fill="#111111">C. 3D Gumball Gizmo (widget.wgsl)</text>
    <!-- Center origin at (685, 160) -->
    <circle cx="685" cy="160" r="4.5" fill="#111111"/>
    <!-- Z Axis (Up) -->
    <line x1="685" y1="160" x2="685" y2="98" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <text x="685" y="92" text-anchor="middle" font-size="9.5" font-weight="700" fill="#111111">Z</text>
    <!-- X Axis (Down-Right) + Scale Box -->
    <line x1="685" y1="160" x2="758" y2="182" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <rect x="730" y="170" width="8" height="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="770" y="186" font-size="9.5" font-weight="700" fill="#111111">X</text>
    <!-- Y Axis (Up-Right) -->
    <line x1="685" y1="160" x2="742" y2="122" stroke="#111111" stroke-width="2" marker-end="url(#arrow)"/>
    <text x="752" y="122" font-size="9.5" font-weight="700" fill="#111111">Y</text>
    <!-- Rotation Arc between Z and X -->
    <path d="M 685 122 A 38 38 0 0 1 722 171" fill="none" stroke="#111111" stroke-width="1.5" stroke-dasharray="3,2"/>
    <text x="696" y="210" text-anchor="middle" font-size="9.5" font-weight="600" fill="#111111">Scaled by clip.w for constant pixel size</text>
    <text x="696" y="225" text-anchor="middle" font-size="9" fill="#111111">Arrows=Move · Arcs=Rotate · Boxes=Scale</text>
    
</svg>
</div>

<div class="webgpu-schematic" style="margin:1.1rem 0;padding:0.75rem;border:1.5px solid #111111;border-radius:10px;background:#ffffff;">
<svg width="840" height="214" viewBox="0 0 840 214" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrow-hollow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    </marker>
    <pattern id="hatch" width="6" height="6" patternTransform="rotate(45 0 0)" patternUnits="userSpaceOnUse">
      <line x1="0" y1="0" x2="0" y2="6" stroke="#111111" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="4" y="4" width="832" height="206" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text x="20" y="27" font-size="13.5" font-weight="700" fill="#111111">Lesson 7.1 Pipeline Schematic: Active tools own their command input</text>
  <text x="20" y="45" font-size="11" fill="#111111">Step-by-step data flow you implement in this lesson (Checkpoint: 46-tool-panel)</text>
  <line x1="16" y1="53" x2="824" y2="53" stroke="#111111" stroke-width="1"/>
  
    
    <rect x="18" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="110" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 1</text>
    <line x1="28" y1="90" x2="192" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="110" y="108" text-anchor="middle" font-size="11" fill="#111111">active tool prompt and</text>
    <text x="110" y="124" text-anchor="middle" font-size="11" fill="#111111">values</text>
    <line x1="202" y1="108" x2="222" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="224" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 2</text>
    <line x1="234" y1="90" x2="398" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="316" y="108" text-anchor="middle" font-size="11" fill="#111111">shared egui command dock</text>
    <line x1="408" y1="108" x2="428" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="430" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="522" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 3</text>
    <line x1="440" y1="90" x2="604" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="522" y="108" text-anchor="middle" font-size="11" fill="#111111">owned pointer and</text>
    <text x="522" y="124" text-anchor="middle" font-size="11" fill="#111111">keyboard</text>
    <line x1="614" y1="108" x2="634" y2="108" stroke="#111111" stroke-width="1.75" marker-end="url(#arrow)"/>

    <rect x="636" y="66" width="184" height="82" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="728" y="84" text-anchor="middle" font-size="11" font-weight="700" fill="#111111">Stage 4</text>
    <line x1="646" y1="90" x2="810" y2="90" stroke="#111111" stroke-width="0.75"/>
    <text x="728" y="108" text-anchor="middle" font-size="11" fill="#111111">unfiltered tool input</text>
    
    <rect x="18" y="158" width="804" height="44" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="32" y="177" font-size="11" font-weight="700" fill="#111111">Files Modified in Lesson 7.1:</text>
    <text x="235" y="177" font-size="11" fill="#111111">src/panel.rs</text>
    <text x="32" y="194" font-size="10.5" fill="#111111">Checkpoint: 46-tool-panel — Verify at each checkpoint with: npm --prefix ../session_tests run course -- check &lt;ID&gt;</text>
  
</svg>
</div>

## 2. Language & Shader Bridge (`Rust` ↔ `WGSL` ↔ `WebGPU`)

Before writing code, review how the three layers in this chapter talk to each other:

| Layer | Construct | Plain-English Explanation for Students |
| :--- | :--- | :--- |
| **Rust (CAD Math)** | Ray–Plane Intersection `t = dot(P0 - O, N) / dot(D, N)` | Projects a 2D mouse ray onto the active 3D construction plane (`Plane`), with priority snapping (`Endpoint > Midpoint > Curve > Grid`). |
| **Rust (Editor)** | Preview Overlay vs. Atomic Undo Commit | Rubber-band preview geometry updates on every `pointermove` without touching the Undo stack; `Enter` or final click commits one atomic kernel transaction. |
| **WGSL (`widget.wgsl`)** | Perspective-Compensated 3D Gumball Handles | Scales axis arrows, rotation arcs, and scale boxes by camera depth `w` so the 3D Gumball gizmo stays a constant clickable pixel size at any zoom. |

**Authoritative production source (`session_viewer/src`):** [app/command/mod.rs](../../../src/app/command/mod.rs), [app/command/tool.rs](../../../src/app/command/tool.rs), [app/cplane.rs](../../../src/app/cplane.rs), [app/snap.rs](../../../src/app/snap.rs), [app/gizmo.rs](../../../src/app/gizmo.rs), [engine/gpu/widget.rs](../../../src/engine/gpu/widget.rs), [app/deform.rs](../../../src/app/deform.rs)

## 3. Checkpoint Roadmap (Jump to Any Notebook Step)

| Step | Checkpoint | What You Build | Files Modified | Pipeline Trace |
| ---: | :--- | :--- | :--- | :--- |
| **1** | [`46-tool-panel`](#checkpoint-46-tool-panel) | Keep active values and tool controls inside the existing GPU command panel. | `src/panel.rs` | `active tool prompt and values → shared egui command dock → owned pointer and keyboard → unfiltered tool input` |

---

## 4. Step-by-Step Coding Notebook (Concept + Implementation)

Work through each notebook cell below in order inside `workspace/journey`. Study the **Visual Step Trace**, read the **Concept Walkthrough** and **Self-Check Q&A**, then expand the **Implementation** block to type and verify the code.

<a id="checkpoint-46-tool-panel"></a>

### Step 1 of 1 · Give active tools ownership of the command panel (`46-tool-panel`)

<div class="webgpu-schematic" style="margin:0.75rem 0;padding:0.5rem 0.65rem;border:1px solid #111111;border-radius:8px;background:#ffffff;">
<svg width="840" height="104" viewBox="0 0 840 104" xmlns="http://www.w3.org/2000/svg" style="width:100%;max-width:840px;height:auto;display:block;margin:0 auto;font-family:system-ui,-apple-system,Segoe UI,Roboto,sans-serif;">
  <defs>
    <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
      <path d="M 0 1 L 10 5 L 0 9 z" fill="#111111"/>
    </marker>
  </defs>
  <text x="18" y="24" font-size="11.5" font-weight="700" fill="#111111">Step 1/1 Visual Trace (46-tool-panel): Give active tools ownership of the command panel</text>
  <text x="822" y="24" text-anchor="end" font-size="10.5" fill="#111111">Files: src/panel.rs</text>
  <line x1="18" y1="31" x2="822" y2="31" stroke="#111111" stroke-width="0.75"/>
  
    <rect x="18" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="111" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">active tool prompt and</text>
    <text x="111" y="81" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">values</text>
    <line x1="205" y1="68" x2="221" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="223" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="316" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">shared egui command dock</text>
    <line x1="410" y1="68" x2="426" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="428" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="521" y="66" text-anchor="middle" font-size="10.5" font-weight="500" fill="#111111">owned pointer and keyboard</text>
    <line x1="615" y1="68" x2="631" y2="68" stroke="#111111" stroke-width="1.5" marker-end="url(#arrow)"/>

    <rect x="633" y="42" width="187" height="52" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
    <text x="726" y="66" text-anchor="middle" font-size="10.5" font-weight="700" fill="#111111">unfiltered tool input</text>
    
</svg>
</div>

- **Goal:** Keep active values and tool controls inside the existing GPU command panel.
- **Data Flow Trace:** `active tool prompt and values → shared egui command dock → owned pointer and keyboard → unfiltered tool input`
- **Files You Touch:** `src/panel.rs` (*Supply active prompts, own option input and forward values to their tool.*)

The panel already shares the viewer device and draws its font atlas after the scene. Add the tool handoff: its prompt and options own value input, while field typing, button clicks and scrolling remain inside the dock.

> **Self-Check Question:** *Why return a tool value before checking ordinary command names?*  
> **Answer:** A value such as60 and an empty Enter belong to the running tool. The tool decides whether to keep a value, advance a point or commit.

- **What You Should See:** Inspect the copied tool_buttons_tests, then repeat the two-box selection and Undo exercise. This preparation adds the panel handoff; the point tool connects it at48.
- **Hands-On Lab Experiment:** Inspect the supplied widget fixture: click Thickness40, type60 and press Enter. Explain why the owner must receive60 without a Help completion or a second submission.

<details>
<summary>Implementation 46-tool-panel: Give active tools ownership of the command panel</summary>

Keep active values and tool controls inside the existing GPU command panel.

Data flow: active tool prompt and values → shared egui command dock → owned pointer and keyboard → unfiltered tool input.

### 1. `src/panel.rs`

Supply active prompts, own option input and forward values to their tool.

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

<details>
<summary>Locate this block</summary>

```rust
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else { return Ok(None); };
        if line == "Escape" { return Ok(None); }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
```

</details>

Replace it with:

```rust
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else { return Ok(None); };
        if !self.model.options.is_empty() { return Ok(Some(line)); }
        if line == "Escape" { return Ok(None); }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
```

<details>
<summary>Locate this block</summary>

```rust
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
```

</details>

Replace it with:

```rust
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
            "options": self.model.options, "options_rect": self.model.options_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
```

<details>
<summary>Locate this block</summary>

```rust
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        if over { event.prevent_default(); }
        over
    }
```

</details>

Replace it with:

```rust
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || (self.model.completion_rect.is_some_and(|r| r.contains(pos)) || self.model.options_rect.is_some_and(|r| r.contains(pos)));
        if over { event.prevent_default(); }
        over
    }
```

<details>
<summary>Locate this block</summary>

```rust
    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || self.model.completion_rect.is_some_and(|r| r.contains(pos));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
```

</details>

Replace it with:

```rust
    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || (self.model.completion_rect.is_some_and(|r| r.contains(pos)) || self.model.options_rect.is_some_and(|r| r.contains(pos)));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
```

<details>
<summary>Locate this block</summary>

```rust
        } }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }
```

</details>

Replace it with:

```rust
        } }
    }

    pub fn tool(&mut self, prompt: String, options: Vec<(String, String)>) {
        self.model.drawing_prompt = prompt; self.model.options = options;
        if !self.model.options.is_empty() { self.model.completion_visible = false; self.model.inline_suffix = false; }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }
```

<details>
<summary>Complete updated block reference</summary>

```rust
use crate::command_dock::{self, CommandLine, Commands as _, theme};
use crate::renderer::Renderer;

struct Commands(&'static [&'static str]);

impl command_dock::Commands for Commands {
    fn canonical(&self, line: &str) -> String {
        let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
        self.0
            .iter()
            .find(|name| name.eq_ignore_ascii_case(&line))
            .map_or(line.clone(), |name| (*name).into())
    }
    fn choosing_option(&self, _: &str) -> bool {
        false
    }
    fn accept(&self, line: &str) -> (String, bool) {
        (self.canonical(line), true)
    }
    fn completions(&self, line: &str) -> Vec<&'static str> {
        crate::command_vocabulary::ranked(self.0.as_ref(), line)
    }
    fn browse(&self, line: &str) -> Vec<&'static str> {
        self.completions(line)
    }
}

pub struct Panel {
    commands: Commands,
    pub consumed: bool,
    pointer_owned: bool,
    top: f32,
    controls: Option<Vec<command_dock::Control>>,
    context: egui::Context,
    painter: egui_wgpu::Renderer,
    model: CommandLine,
    events: Vec<egui::Event>,
    output: Option<egui::FullOutput>,
    screen: egui_wgpu::ScreenDescriptor,
}

impl Panel {
    pub fn new(renderer: &Renderer, format: wgpu::TextureFormat, commands: &'static [&'static str]) -> Self {
        let context = egui::Context::default();
        context.set_fonts(theme::fonts([
            include_bytes!("../assets/text/NotoSans-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols2-Regular.subset.ttf"),
            include_bytes!("../assets/text/NotoSansSymbols-Regular.subset.ttf"),
        ]));
        context.set_theme(egui::Theme::Light);
        context.set_visuals(theme::visuals());
        // One layout pass prevents a future text event from being replayed.
        context.options_mut(|options| options.max_passes = 1.try_into().unwrap());
        let painter = egui_wgpu::Renderer::new(&renderer.device, format, Default::default());
        Self { commands: Commands(commands), consumed: false, pointer_owned: false, top: f32::INFINITY, controls: None, context, painter, events: Vec::new(), output: None,
            screen: egui_wgpu::ScreenDescriptor { size_in_pixels: [640, 480], pixels_per_point: 1.0 }, model: CommandLine {
            focus_command: true,
            ..Default::default()
        } }
    }

    pub fn tool(&mut self, prompt: String, options: Vec<(String, String)>) {
        self.model.drawing_prompt = prompt; self.model.options = options;
        if !self.model.options.is_empty() { self.model.completion_visible = false; self.model.inline_suffix = false; }
    }

    pub fn command(&self) -> &str { &self.model.command }

    pub fn history(&self) -> &std::collections::VecDeque<String> { &self.model.history }

    pub fn shortcut(&self, event: &web_sys::KeyboardEvent) -> Option<crate::shortcuts::Shortcut> {
        if event.type_() != "keydown" || event.repeat() { return None; }
        crate::shortcuts::key(&event.key(), event.ctrl_key() || event.meta_key(), event.shift_key(),
            event.alt_key(), event.is_composing(), self.context.egui_wants_keyboard_input(), self.model.command.is_empty())
    }

    pub fn key(&mut self, event: &web_sys::KeyboardEvent) -> bool {

        if event.is_composing() { return false; }
        let modifiers = egui::Modifiers {
            alt: event.alt_key(), ctrl: event.ctrl_key(), shift: event.shift_key(),
            mac_cmd: event.meta_key(), command: event.ctrl_key() || event.meta_key(),
        };
        let text = event.type_() == "keydown" && event.key().chars().count() == 1
            && !modifiers.command && !modifiers.alt;
        if text {
            if !self.context.egui_wants_keyboard_input() { self.model.focus_command = true; }
            self.context.memory_mut(|memory| memory.request_focus(egui::Id::new("command-input")));
            self.model.command_open = true;
        }
        if !self.context.egui_wants_keyboard_input() { return false; }
        if let Some(key) = egui::Key::from_name(&event.key()) {
            self.events.push(egui::Event::Key {
                key, physical_key: None, pressed: event.type_() == "keydown",
                repeat: event.repeat(), modifiers,
            });
        }
        if text { self.events.push(egui::Event::Text(event.key())); }
        event.prevent_default();
        true
    }

    pub fn pointer(&mut self, event: &web_sys::PointerEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        let over = pos.y >= self.top || (self.model.completion_rect.is_some_and(|r| r.contains(pos)) || self.model.options_rect.is_some_and(|r| r.contains(pos)));
        let owned = over || self.pointer_owned;
        if event.type_() == "pointerdown" {
            self.pointer_owned = over;
            if !over {
                self.model.command_open = false;
                self.model.focus_command = false;
                self.context.memory_mut(|memory| memory.surrender_focus(egui::Id::new("command-input")));
            }
        }
        self.events.push(egui::Event::PointerMoved(pos));
        if matches!(event.type_().as_str(), "pointercancel" | "lostpointercapture") {
            for button in [egui::PointerButton::Primary, egui::PointerButton::Middle, egui::PointerButton::Secondary] {
                self.events.push(egui::Event::PointerButton { pos, button, pressed: false, modifiers: Default::default() });
            }
            self.events.push(egui::Event::PointerGone);
        } else if matches!(event.type_().as_str(), "pointerdown" | "pointerup") {
            let button = match event.button() { 0 => egui::PointerButton::Primary, 1 => egui::PointerButton::Middle, _ => egui::PointerButton::Secondary };
            self.events.push(egui::Event::PointerButton { pos, button, pressed: event.type_() == "pointerdown", modifiers: Default::default() });
        }
        if matches!(event.type_().as_str(), "pointerup" | "pointercancel" | "lostpointercapture") { self.pointer_owned = false; }
        if owned { event.prevent_default(); }
        owned
    }

    pub fn wheel(&mut self, event: &web_sys::WheelEvent, canvas: &web_sys::HtmlCanvasElement) -> bool {
        let rect = canvas.get_bounding_client_rect();
        let pos = egui::pos2((event.client_x() as f64 - rect.left()) as f32, (event.client_y() as f64 - rect.top()) as f32);
        self.events.push(egui::Event::PointerMoved(pos));
        self.events.push(egui::Event::MouseWheel {
            unit: match event.delta_mode() { 1 => egui::MouseWheelUnit::Line, 2 => egui::MouseWheelUnit::Page, _ => egui::MouseWheelUnit::Point },
            phase: egui::TouchPhase::Move, delta: egui::vec2(-event.delta_x() as f32, -event.delta_y() as f32), modifiers: Default::default(),
        });
        let over = pos.y >= self.top || (self.model.completion_rect.is_some_and(|r| r.contains(pos)) || self.model.options_rect.is_some_and(|r| r.contains(pos)));
        if over { event.prevent_default(); }
        over
    }

    pub fn inspect(&self) -> String {
        serde_json::json!({"controls": self.controls, "command": self.model.command, "history": self.model.history, "status": self.model.status,
            "pointer_owned": self.pointer_owned,
            "top": self.top, "focused": self.context.egui_wants_keyboard_input(),
            "completion_rect": self.model.completion_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y]),
            "options": self.model.options, "options_rect": self.model.options_rect.map(|r| [r.min.x, r.min.y, r.max.x, r.max.y])}).to_string()
    }

    pub fn answer(&mut self, line: &str, message: &str) {
        self.model.remember(format!("> {line}\n{message}"));
        self.model.status = message.into();
    }

    pub fn result(&mut self, message: &str) {
        self.model.status = message.into();
        if let Some(last) = self.model.history.back_mut()
            && let Some(at) = last.find('\n')
        {
            last.truncate(at + 1);
            last.push_str(message);
        }
    }

    pub fn update(
        &mut self,
        event: Option<&web_sys::Event>,
        canvas: &web_sys::HtmlCanvasElement,
    ) -> Result<Option<String>, wasm_bindgen::JsValue> {
        use wasm_bindgen::JsCast;
        let rect = canvas.get_bounding_client_rect();
        self.screen.size_in_pixels = [canvas.width(), canvas.height()];
        self.screen.pixels_per_point = canvas.width() as f32 / rect.width() as f32;
        self.consumed = false;
        if let Some(event) = event {
            if let Some(key) = event.dyn_ref::<web_sys::KeyboardEvent>() { self.consumed = self.key(key); }
            if let Some(pointer) = event.dyn_ref::<web_sys::PointerEvent>() {
                self.consumed = self.pointer(pointer, canvas);
                if event.type_() == "pointerdown" {
                    let options = web_sys::FocusOptions::new(); options.set_prevent_scroll(true);
                    canvas.focus_with_options(&options)?;
                    event.prevent_default();
                }
            }
            if let Some(wheel) = event.dyn_ref::<web_sys::WheelEvent>() { self.consumed = self.wheel(wheel, canvas); }
        }
        let line = self.prepare();
        canvas.set_attribute("data-command-ui", &self.inspect())?;
        let Some(line) = line else { return Ok(None); };
        if !self.model.options.is_empty() { return Ok(Some(line)); }
        if line == "Escape" { return Ok(None); }
        if line.eq_ignore_ascii_case("Help") {
            self.answer(&line, &self.commands.0.join(" · "));
            return Ok(None);
        }
        if !self.commands.0.iter()
            .any(|name| name.eq_ignore_ascii_case(&line)
                || (*name == "Move" && line.to_ascii_lowercase().starts_with("move ")))
        {
            self.answer(&line, "Unknown command. Type Help.");
            return Ok(None);
        }
        self.answer(&line, "Command submitted.");
        Ok(Some(line.to_ascii_lowercase()))
    }

    fn prepare(&mut self) -> Option<String> {
        let size = self.screen.size_in_pixels;
        let mut input = egui::RawInput {
            focused: true,
            events: std::mem::take(&mut self.events),
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO, egui::vec2(size[0] as f32, size[1] as f32) / self.screen.pixels_per_point,
            )),
            ..Default::default()
        };
        input.viewports.get_mut(&egui::ViewportId::ROOT).unwrap().native_pixels_per_point = Some(self.screen.pixels_per_point);
        self.controls = Some(Vec::new());
        let mut line = None;
        let output = self.context.run_ui(input, |root| {
            command_dock::draw(root, &mut self.model, &mut self.controls, &mut line, &self.commands, true);
        });
        if let Some(control) = self.controls.as_ref().and_then(|items| items.iter().find(|item| item.key == "command/resize")) {
            self.top = control.rect[1];
        }
        if let Some(previous) = self.output.as_mut() { previous.append(output); }
        else { self.output = Some(output); }
        line
    }

    pub fn draw(&mut self, renderer: &Renderer, target: &wgpu::TextureView) {
        let Some(output) = self.output.take() else { return; };
        // The font atlas is a texture: upload its changed pixels before drawing the letters.
        for (id, delta) in &output.textures_delta.set {
            self.painter.update_texture(&renderer.device, &renderer.queue, *id, delta);
        }
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let mut encoder = renderer.device.create_command_encoder(&Default::default());
        let mut buffers = self.painter.update_buffers(
            &renderer.device, &renderer.queue, &mut encoder, &jobs, &self.screen,
        );
        let pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("command panel"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target, resolve_target: None, depth_slice: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
            })],
            ..Default::default()
        });
        self.painter.render(&mut pass.forget_lifetime(), &jobs, &self.screen);
        buffers.push(encoder.finish());
        renderer.queue.submit(buffers);
        for id in output.textures_delta.free { self.painter.free_texture(&id); }
    }
}
```

</details>

<details>
<summary>Optional patch references</summary>

Use these original edit fragments when comparing a saved checkpoint. Apply the complete blocks above.

```text
--8<-- "journey/code/46-tool-panel-01.rs"
```

```text
--8<-- "journey/code/46-tool-panel-02.rs"
```

```text
--8<-- "journey/code/46-tool-panel-03.rs"
```

```text
--8<-- "journey/code/46-tool-panel-04.rs"
```

```text
--8<-- "journey/code/46-tool-panel-05.rs"
```

</details>

</details>

---

## 5. Build, Verify, and Save Your Checkpoint

Run these commands from `session_viewer` to install dependencies, verify your typed code against the checkpoint, and save your progress:

```sh
npm --prefix ../session_tests run course -- dependencies 46-tool-panel
npm --prefix ../session_tests run course -- check 46-tool-panel
npm --prefix ../session_tests run course -- save 46-tool-panel
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
```

Open `http://127.0.0.1:8780/` in your browser. Use [checkpoint recovery](../recovery.md) to save or restore any checkpoint ID.

## 6. Lesson Summary & Expected Visual Result

![Recorded visual output for Lesson 7.1 (46-tool-panel)](../../screenshots/journey/46-tool-panel.png)

Inspect the copied tool_buttons_tests, then repeat the two-box selection and Undo exercise. This preparation adds the panel handoff; the point tool connects it at48.

- **Final Lesson Experiment:** Inspect the supplied widget fixture: click Thickness40, type60 and press Enter. Explain why the owner must receive60 without a Help completion or a second submission.

Continue with [Typed commands and host requests](47-registry.md).
