# 2.5 · Document history and perspective rays

**Steps:** 4 · **Source:** `src/camera.rs` · `src/engine/gpu/render.rs` · `src/shaders/scene.wgsl`

## <a id="checkpoint-13a-history"></a>1. Retain reversible scene snapshots

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-6ee78b50" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-6ee78b50" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-6ee78b50" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-6ee78b50" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-6ee78b50" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">13A..14 · UNDO / REDO</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Reversible History Stack (past, present, future) &amp; Branch Clearing on New Commit</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">commit(S') -&gt; past.push(S); future.clear()</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-6ee78b50)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Three-Part State Partition</text>
  <rect x="44" y="82" width="84" height="60" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="86" y="108" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">past</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="86" y="126" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">[S₀, S₁]</text>
  <line x1="132" y1="112" x2="148" y2="112" stroke="#111111" stroke-width="1.75" marker-end="url(#arr-6ee78b50)"/>
  <rect x="152" y="76" width="84" height="72" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="194" y="108" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">present</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="194" y="126" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">S₂</text>
  <line x1="240" y1="112" x2="256" y2="112" stroke="#111111" stroke-width="1.75" marker-end="url(#arr-6ee78b50)"/>
  <rect x="260" y="82" width="76" height="60" rx="4" fill="#fef9c3" stroke="#d97706" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="298" y="108" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">future</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="298" y="126" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">[S₃]</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-6ee78b50)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-6ee78b50)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Undo / Redo / New Edit Transitions</text>
  <rect x="414" y="74" width="292" height="108" rx="4" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="98" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">Undo: future.push(present); present = past.pop()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="124" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Redo: past.push(present); present = future.pop()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="150" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#dc2626">Edit: past.push(present); future.clear()</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="172" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#111111">No-op changes skipped to prevent duplicate frames</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-6ee78b50)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-6ee78b50)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Command Dock Undo / Redo</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="826" y="84" width="96" height="32" rx="4" fill="#dbeafe" stroke="#2563eb" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="874" y="104" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#2563eb">Undo (Ctrl+Z)</text>
  <rect x="942" y="84" width="96" height="32" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="990" y="104" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#16a34a">Redo (Ctrl+Y)</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="156" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">Exact Scene Restored ✓</text>
</svg>
</div>

### 1. `src/scene.rs`

**Locate this existing block:**

```rust
use crate::mesh::Mesh;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectId(u32);

pub struct Object {
    pub id: ObjectId,
    pub mesh: Mesh,
}

pub struct Scene {
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn demo() -> Self {
        let near = Mesh::new(
            vec![
                [-0.7, -0.6, 0.25, 0.9, 0.25, 0.45],
                [0.5, -0.6, 0.25, 0.9, 0.25, 0.45],
                [-0.1, 0.6, 0.25, 0.9, 0.25, 0.45],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid near triangle");
        let far = Mesh::new(
            vec![
                [-0.4, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.8, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.2, 0.8, 0.75, 0.05, 0.7, 0.7],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid far triangle");
        let mut scene = Self {
            objects: Vec::new(),
            next_id: 1,
            extra: None,
        };
        scene.insert(near).expect("ID available");
        scene.insert(far).expect("ID available");
        scene
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn insert(&mut self, mesh: Mesh) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.objects.push(Object { id, mesh });
        Ok(id)
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use crate::mesh::Mesh;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectId(u32);

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub mesh: Rc<Mesh>,
}

#[derive(Clone)]
pub struct Scene {
    objects: Vec<Object>,
    next_id: u32,
    extra: Option<ObjectId>,
}

impl Scene {
    pub fn demo() -> Self {
        let near = Mesh::new(
            vec![
                [-0.7, -0.6, 0.25, 0.9, 0.25, 0.45],
                [0.5, -0.6, 0.25, 0.9, 0.25, 0.45],
                [-0.1, 0.6, 0.25, 0.9, 0.25, 0.45],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid near triangle");
        let far = Mesh::new(
            vec![
                [-0.4, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.8, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.2, 0.8, 0.75, 0.05, 0.7, 0.7],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid far triangle");
        let mut scene = Self {
            objects: Vec::new(),
            next_id: 1,
            extra: None,
        };
        scene.insert(near).expect("ID available");
        scene.insert(far).expect("ID available");
        scene
    }

    pub fn restore(&mut self, mut saved: Self) {
        saved.next_id = saved.next_id.max(self.next_id);
        *self = saved;
    }

    pub fn objects(&self) -> &[Object] {
        &self.objects
    }

    pub fn insert(&mut self, mesh: Mesh) -> Result<ObjectId, &'static str> {
        let next = self
            .next_id
            .checked_add(1)
            .ok_or("Object IDs exhausted")?;
        let id = ObjectId(self.next_id);
        self.next_id = next;
        self.objects.push(Object {
            id,
            mesh: Rc::new(mesh),
        });
        Ok(id)
```

### 2. `src/history.rs`

Create the file and type:

```rust
use crate::scene::Scene;

const LIMIT: usize = 64;

#[derive(Default)]
pub struct History {
    undo: Vec<Scene>,
    redo: Vec<Scene>,
}

impl History {
    pub fn edit(&mut self, scene: &mut Scene, action: impl FnOnce(&mut Scene)) {
        let before = scene.clone();
        action(scene);
        if self.undo.len() == LIMIT {
            self.undo.remove(0);
        }
        self.undo.push(before);
        self.redo.clear();
    }

    pub fn undo(&mut self, scene: &mut Scene) -> bool {
        Self::travel(&mut self.undo, &mut self.redo, scene)
    }

    pub fn redo(&mut self, scene: &mut Scene) -> bool {
        Self::travel(&mut self.redo, &mut self.undo, scene)
    }

    fn travel(from: &mut Vec<Scene>, to: &mut Vec<Scene>, scene: &mut Scene) -> bool {
        let Some(saved) = from.pop() else {
            return false;
        };
        to.push(scene.clone());
        scene.restore(saved);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn undo_restores_identity_and_shares_the_original_mesh() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        let id = scene.objects()[0].id;
        let mesh = Rc::clone(&scene.objects()[0].mesh);
        history.edit(&mut scene, |scene| {
            scene.remove(id);
        });
        assert!(!scene.contains(id));
        assert!(history.undo(&mut scene));
        assert_eq!(scene.objects()[0].id, id);
        assert!(Rc::ptr_eq(&mesh, &scene.objects()[0].mesh));
        assert!(history.redo(&mut scene));
        assert!(!scene.contains(id));
    }

    #[test]
    fn a_new_branch_discards_redo_without_reusing_an_old_id() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        history.edit(&mut scene, Scene::toggle_extra);
        let old_id = scene.objects()[2].id;
        history.undo(&mut scene);
        history.edit(&mut scene, Scene::toggle_extra);
        assert_ne!(scene.objects()[2].id, old_id);
        assert!(!history.redo(&mut scene));
    }

    #[test]
    fn retained_history_has_a_fixed_count_limit() {
        let mut scene = Scene::demo();
        let mut history = History::default();
        for _ in 0..LIMIT + 1 {
            history.edit(&mut scene, Scene::toggle_extra);
        }
        let mut count = 0;
        while history.undo(&mut scene) {
            count += 1;
        }
        assert_eq!(count, LIMIT);
    }
}
```

### 3. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod gpu_mesh;
pub mod mesh;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod gpu_mesh;
pub mod history;
pub mod mesh;
```

## <a id="checkpoint-14-history"></a>2. Run Undo and Redo from the command line

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="188" viewBox="0 0 1120 188" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arrR" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arrG" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arrB" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
  </defs>
  <rect x="2" y="2" width="1116" height="184" rx="10" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="14" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="186" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Cursor (x_px, y_px) -&gt; NDC (-1..+1)</text>
  <rect x="68" y="48" width="236" height="110" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="68" y1="103" x2="304" y2="103" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3"/>
  <line x1="186" y1="48" x2="186" y2="158" stroke="#94a3b8" stroke-width="1" stroke-dasharray="3 3"/>
  <circle cx="222" cy="82" r="6" fill="#facc15" stroke="#111111" stroke-width="1.75"/>
  <line x1="212" y1="82" x2="232" y2="82" stroke="#111111" stroke-width="1.5"/>
  <line x1="222" y1="72" x2="222" y2="92" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="222" y="66" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">(ndc_x, ndc_y)</text>
  <line x1="358" y1="94" x2="384" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="388" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="560" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">inv(M_vp): Near-&gt;Far Ray ∩ Triangle</text>
  <circle cx="432" cy="128" r="4.5" fill="#111111"/><text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="432" y="146" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">p_near</text>
  <line x1="432" y1="128" x2="684" y2="66" stroke="#dc2626" stroke-width="2" marker-end="url(#arrR)"/>
  <polygon points="548,52 606,146 508,136" fill="#dbeafe" stroke="#111111" stroke-width="1.75"/>
  <circle cx="556" cy="97" r="5.5" fill="#facc15" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="556" y="84" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">hit (u,v,t_min)</text>
  <line x1="732" y1="94" x2="758" y2="94" stroke="#111111" stroke-width="2" marker-end="url(#arr)"/>
  <rect x="762" y="12" width="344" height="164" rx="8" fill="#ffffff" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="5" stroke-linejoin="round" x="934" y="30" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Selected Object Highlight in Viewer</text>
  <rect x="814" y="44" width="240" height="118" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="888,68 948,144 844,144" fill="#cbd5e1" stroke="#111111" stroke-width="1.5"/>
  <polygon points="956,56 1022,144 908,144" fill="#fef08a" stroke="#111111" stroke-width="2.5"/>
  <circle cx="964" cy="108" r="4.5" fill="#dc2626"/>
</svg>
</div>

### 1. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Click a triangle; the visible object is selected.");
    Ok(())
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Undo restores the document while the view stays put.");
    Ok(())
```

**Locate this block:**

```rust
                    }
                    "delete" => {
                        if let Some(id) = selected.take() {
                            scene.remove(id);
                            renderer.set_scene(&scene, selected);
                        }
                    }
                    "background" => background.toggle(),
                    "zoom in" => camera.zoom(2.0),
                    "zoom out" => camera.zoom(0.5),
```

Replace it with:

```rust
                    }
                    "delete" => {
                        if let Some(id) = selected.take() {
                            history.edit(&mut scene, |scene| {
                                scene.remove(id);
                            });
                            renderer.set_scene(&scene, selected);
                        }
                    }
                    "undo" | "redo" => {
                        if line == "undo" {
                            history.undo(&mut scene);
                        } else {
                            history.redo(&mut scene);
                        }
                        selected = selected.filter(|id| scene.contains(*id));
                        renderer.set_scene(&scene, selected);
                    }
                    "background" => background.toggle(),
                    "zoom in" => camera.zoom(2.0),
                    "zoom out" => camera.zoom(0.5),
```

**Locate this block:**

```rust
                        renderer.set_scene(&scene, selected);
                    }
                    "example triangle" => {
                        scene.toggle_extra();
                        selected = selected.filter(|id| scene.contains(*id));
                        renderer.set_scene(&scene, selected);
                    }
```

Replace it with:

```rust
                        renderer.set_scene(&scene, selected);
                    }
                    "example triangle" => {
                        history.edit(&mut scene, Scene::toggle_extra);
                        selected = selected.filter(|id| scene.contains(*id));
                        renderer.set_scene(&scene, selected);
                    }
```

**Locate this block:**

```rust
        ],
    );
    panel.update(None, &canvas)?;
    let mut selected = None;
    let mut background = Background::default();
    let mut camera = Camera::default();
```

Replace it with:

```rust
        ],
    );
    panel.update(None, &canvas)?;
    let mut history = History::default();
    let mut selected = None;
    let mut background = Background::default();
    let mut camera = Camera::default();
```

**Locate this block:**

```rust
            "Example Triangle",
            "Select Next",
            "Delete",
            "Background",
            "Zoom In",
            "Zoom Out",
```

Replace it with:

```rust
            "Example Triangle",
            "Select Next",
            "Delete",
            "Undo",
            "Redo",
            "Background",
            "Zoom In",
            "Zoom Out",
```

**Locate this block:**

```rust
use crate::camera::Camera;
use crate::renderer::Renderer;
use crate::scene::Scene;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

Replace it with:

```rust
use crate::camera::Camera;
use crate::history::History;
use crate::renderer::Renderer;
use crate::scene::Scene;
use wasm_bindgen::{JsCast, JsValue, closure::Closure};
```

## <a id="checkpoint-14a-perspective"></a>3. Draw through a perspective camera

<div class="webgpu-schematic" style="width:100%;margin:6px auto;">
<svg xmlns="http://www.w3.org/2000/svg" width="1120" height="214" viewBox="0 0 1120 214" style="width:100%;max-width:100%;height:auto;display:block;margin:0 auto;" role="img">
  <defs>
    <marker id="arr-10de0199" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#111111"/>
    </marker>
    <marker id="arr-r-10de0199" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#dc2626"/>
    </marker>
    <marker id="arr-g-10de0199" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#16a34a"/>
    </marker>
    <marker id="arr-b-10de0199" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto">
      <path d="M 0 1 L 9 5 L 0 9 z" fill="#2563eb"/>
    </marker>
    <pattern id="grid-10de0199" width="20" height="20" patternUnits="userSpaceOnUse">
      <path d="M 20 0 L 0 0 0 20" fill="none" stroke="#f1f5f9" stroke-width="1"/>
    </pattern>
  </defs>
  <rect x="2" y="2" width="1116" height="210" rx="8" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <rect x="2" y="2" width="1116" height="28" rx="6" fill="#f8fafc" stroke="#111111" stroke-width="1.5"/>
  <line x1="2" y1="30" x2="1118" y2="30" stroke="#111111" stroke-width="1.5"/>
  <rect x="12" y="7" width="176" height="18" rx="3" fill="#111111"/>
  <text x="100" y="19.5" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#ffffff" paint-order="normal" stroke="none">14A..14B · FRUSTUM &amp; RAY</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="518" y="20" text-anchor="middle" font-family="monospace" font-size="10.5" font-weight="700" fill="#111111">Right-Handed Perspective Frustum (Z ∈ [0, 1]), Near/Far Unprojection &amp; Undo/Redo Stack</text>
  <rect x="838" y="7" width="268" height="18" rx="3" fill="#ffffff" stroke="#111111" stroke-width="1.2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="972" y="19.5" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">R(t) = P_near + t*(P_far - P_near)</text>
  <rect x="14" y="38" width="344" height="164" rx="6" fill="url(#grid-10de0199)" stroke="#111111" stroke-width="1.5"/>
  <line x1="14" y1="60" x2="358" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">1 · Reversible History Stack (Undo / Redo)</text>
  <rect x="44" y="80" width="84" height="54" rx="4" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="86" y="104" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">past</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="86" y="120" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#2563eb">Vec&lt;Scene&gt;</text>
  <line x1="132" y1="107" x2="148" y2="107" stroke="#2563eb" stroke-width="1.75" marker-end="url(#arr-b-10de0199)"/>
  <rect x="152" y="76" width="84" height="62" rx="4" fill="#dcfce7" stroke="#16a34a" stroke-width="2"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="194" y="104" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">present</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="194" y="120" text-anchor="middle" font-family="monospace" font-size="8.5" fill="#16a34a">Active</text>
  <line x1="240" y1="107" x2="256" y2="107" stroke="#dc2626" stroke-width="1.75" marker-end="url(#arr-r-10de0199)"/>
  <rect x="260" y="80" width="76" height="54" rx="4" fill="#f1f5f9" stroke="#111111" stroke-width="1.5"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="298" y="104" text-anchor="middle" font-family="monospace" font-size="9.5" font-weight="700" fill="#111111">future</text>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="186" y="166" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Undo pops past · Redo pops future</text>
  <line x1="358" y1="120" x2="384" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-10de0199)"/>
  <rect x="388" y="38" width="344" height="164" rx="6" fill="url(#grid-10de0199)" stroke="#111111" stroke-width="1.5"/>
  <line x1="388" y1="60" x2="732" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="560" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">2 · Frustum Unprojection (z_ndc = 0.0 -&gt; 1.0)</text>
  <circle cx="424" cy="128" r="4.5" fill="#111111"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="424" y="148" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">Eye</text>
  <polygon points="484,96 688,68 688,188 484,160" fill="#dbeafe" fill-opacity="0.35" stroke="#111111" stroke-width="1.5"/>
  <line x1="484" y1="96" x2="484" y2="160" stroke="#2563eb" stroke-width="2.5"/>
  <line x1="688" y1="68" x2="688" y2="188" stroke="#16a34a" stroke-width="2"/>
  <line x1="424" y1="128" x2="704" y2="106" stroke="#dc2626" stroke-width="2.2" marker-end="url(#arr-r-10de0199)"/>
  <circle cx="484" cy="123" r="4" fill="#2563eb"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="484" y="88" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#2563eb">P_near(z=0)</text>
  <circle cx="688" cy="107" r="4" fill="#16a34a"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="668" y="64" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#16a34a">P_far(z=1)</text>
  <line x1="732" y1="120" x2="758" y2="120" stroke="#111111" stroke-width="2" marker-end="url(#arr-10de0199)"/>
  <rect x="762" y="38" width="344" height="164" rx="6" fill="url(#grid-10de0199)" stroke="#111111" stroke-width="1.75"/>
  <line x1="762" y1="60" x2="1106" y2="60" stroke="#cbd5e1" stroke-width="1"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="53" text-anchor="middle" font-family="monospace" font-size="10" font-weight="700" fill="#111111">3 · Perspective Foreshortening</text>
  <rect x="806" y="68" width="256" height="122" rx="4" fill="#ffffff" stroke="#111111" stroke-width="2"/>
  <polygon points="852,168 892,84 996,96 964,174" fill="#93c5fd" stroke="#111111" stroke-width="1.75"/>
  <text paint-order="stroke fill" stroke="#ffffff" stroke-width="4.5" stroke-linejoin="round" x="934" y="184" text-anchor="middle" font-family="monospace" font-size="9" font-weight="700" fill="#111111">clip.xyz / clip.w</text>
</svg>
</div>

### 1. `Cargo.toml`

**Locate this existing block:**

```toml
wgpu = "=29.0.4"

egui = { version = "=0.34.3", default-features = false }
```

Edit the existing block to match this code; keep unchanged lines:

```toml
wgpu = "=29.0.4"
session_rust = { path = "../../../session_rust", default-features = false }
getrandom = { version = "=0.2.17", features = ["js"] }

egui = { version = "=0.34.3", default-features = false }
```

### 2. `src/camera.rs`

**Locate this existing block:**

```rust
pub struct Camera {
    pub center: [f32; 2],
    pub scale: f32,
    pub angle: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            scale: 1.0,
            angle: 0.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        self.center[0] += dx;
        self.center[1] += dy;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.scale = (self.scale * factor).clamp(0.1, 10.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians;
    }

    pub fn world_from_screen(&self, point: [f32; 2]) -> [f32; 2] {
        let x = point[0] / self.scale;
        let y = point[1] / self.scale;
        let (sin, cos) = self.angle.sin_cos();
        [
            self.center[0] + x * cos - y * sin,
            self.center[1] + x * sin + y * cos,
        ]
    }

    pub fn uniform(&self) -> [f32; 16] {
        let a = self.scale * self.angle.cos();
        let b = self.scale * self.angle.sin();
        let [x, y] = self.center;
        // WGSL matrices are uploaded column by column.
        [
            a,
            -b,
            0.0,
            0.0,
            b,
            a,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            -a * x - b * y,
            b * x - a * y,
            0.0,
            1.0,
        ]
    }
}

#[cfg(test)]
mod coordinate_tests {
    use super::Camera;

    #[test]
    fn moving_the_view_preserves_the_coordinate_round_trip() {
        let mut camera = Camera::default();
        camera.pan(0.1, -0.2);
        camera.zoom(0.5);
        camera.rotate(0.7);
        let world = [0.4, 0.0];
        let matrix = camera.uniform();
        let screen = [
            matrix[0] * world[0] + matrix[4] * world[1] + matrix[12],
            matrix[1] * world[0] + matrix[5] * world[1] + matrix[13],
        ];
        let restored = camera.world_from_screen(screen);
        assert!((restored[0] - world[0]).abs() < 1.0e-6);
        assert!((restored[1] - world[1]).abs() < 1.0e-6);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panned_target_stays_at_the_screen_centre_when_zooming() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        camera.rotate(std::f32::consts::FRAC_PI_4);
        let m = camera.uniform();
        let [x, y] = camera.center;
        assert!((m[0] * x + m[4] * y + m[12]).abs() < 1.0e-6);
        assert!((m[1] * x + m[5] * y + m[13]).abs() < 1.0e-6);
    }

    #[test]
    fn zoom_stays_positive_and_bounded() {
        let mut camera = Camera::default();
        camera.zoom(0.0001);
        assert_eq!(camera.scale, 0.1);
        camera.zoom(1_000.0);
        assert_eq!(camera.scale, 10.0);
        camera.zoom(f32::NAN);
        camera.zoom(-1.0);
        assert_eq!(camera.scale, 10.0);
    }
```

Edit the existing block to match this code; keep unchanged lines:

```rust
use session_rust::{Point, Vector, Xform};

pub struct Camera {
    pub center: [f64; 2],
    pub distance: f64,
    pub angle: f64,
    pub aspect: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            center: [0.0, 0.0],
            distance: 3.0,
            angle: 0.0,
            aspect: 640.0 / 480.0,
        }
    }
}

impl Camera {
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let (sin, cos) = self.angle.sin_cos();
        self.center[0] += dx as f64 * cos - dy as f64 * sin;
        self.center[1] += dx as f64 * sin + dy as f64 * cos;
    }

    pub fn zoom(&mut self, factor: f32) {
        if factor.is_finite() && factor > 0.0 {
            self.distance = (self.distance / factor as f64).clamp(0.2, 50.0);
        }
    }

    pub fn rotate(&mut self, radians: f32) {
        self.angle += radians as f64;
    }

    pub fn view_projection(&self) -> Xform {
        let eye = Point::new(self.center[0], self.center[1], self.distance);
        let target = Point::new(self.center[0], self.center[1], 0.0);
        let (sin, cos) = self.angle.sin_cos();
        let up = Vector::new(-sin, cos, 0.0);
        let view = Xform::look_at_right_handed(&eye, &target, &up);
        let projection = Xform::perspective(
            60.0_f64.to_radians(),
            self.aspect,
            0.01,
            100.0,
        );
        projection * view
    }

    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_target_stays_centred_after_pan_zoom_and_turn() {
        let mut camera = Camera::default();
        camera.pan(0.25, -0.5);
        camera.zoom(2.0);
        camera.rotate(0.7);
        let target = Point::new(camera.center[0], camera.center[1], 0.0);
        let screen = camera
            .view_projection()
            .transform_point(&target);
        assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
        assert!((0.0..1.0).contains(&screen[2]));
    }
```

### 3. `src/scene.rs`

**Locate this existing block:**

```rust
    pub fn demo() -> Self {
        let near = Mesh::new(
            vec![
                [-0.7, -0.6, 0.25, 0.9, 0.25, 0.45],
                [0.5, -0.6, 0.25, 0.9, 0.25, 0.45],
                [-0.1, 0.6, 0.25, 0.9, 0.25, 0.45],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid near triangle");
        let far = Mesh::new(
            vec![
                [-0.4, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.8, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.2, 0.8, 0.75, 0.05, 0.7, 0.7],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid far triangle");
        let mut scene = Self {
            objects: Vec::new(),
            next_id: 1,
            extra: None,
        };
        scene.insert(near).expect("ID available");
        scene.insert(far).expect("ID available");
        scene
```

Edit the existing block to match this code; keep unchanged lines:

```rust
    pub fn demo() -> Self {
        let pink = Mesh::new(
            vec![
                [-0.7, -0.6, 0.25, 0.9, 0.25, 0.45],
                [0.5, -0.6, 0.25, 0.9, 0.25, 0.45],
                [-0.1, 0.6, 0.25, 0.9, 0.25, 0.45],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid pink triangle");
        let turquoise = Mesh::new(
            vec![
                [-0.4, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.8, -0.2, 0.75, 0.05, 0.7, 0.7],
                [0.2, 0.8, 0.75, 0.05, 0.7, 0.7],
            ],
            vec![0, 1, 2],
        )
        .expect("Valid turquoise triangle");
        let mut scene = Self {
            objects: Vec::new(),
            next_id: 1,
            extra: None,
        };
        scene.insert(pink).expect("ID available");
        scene
            .insert(turquoise)
            .expect("ID available");
        scene
```

### 4. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                    "view reset" => camera = Camera::default(),
                    _ => return,
```

Replace it with:

```rust
                    "pan left" => camera.pan(-0.25, 0.0),
                    "pan right" => camera.pan(0.25, 0.0),
                    "orbit right" => camera.rotate(std::f32::consts::FRAC_PI_4),
                    "view reset" => {
                        camera = Camera {
                            aspect: camera.aspect,
                            ..Camera::default()
                        }
                    }
                    _ => return,
```

**Locate this block:**

```rust
    let mut background = Background::default();
    let mut camera = Camera::default();
    present(
        &surface,
        &renderer,
```

Replace it with:

```rust
    let mut background = Background::default();
    let mut camera = Camera {
        aspect: width as f64 / height as f64,
        ..Camera::default()
    };
    present(
        &surface,
        &renderer,
```

### 5. `src/lib.rs`

**Locate this existing block:**

```rust
pub mod mesh;
pub mod picking;
pub mod renderer;
```

Edit the existing block to match this code; keep unchanged lines:

```rust
pub mod mesh;
pub mod renderer;
```

### 6. `src/browser.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Undo restores the document while the view stays put.");
    Ok(())
```

Replace it with:

```rust
    )?;
    // The page has one listener for its lifetime; JavaScript must retain the Rust callback.
    click.forget();
    report("Perspective drawing uses the current camera.");
    Ok(())
```

**Locate this block:**

```rust
                match line.as_str() {
                    "canvas" => {
                        let Some(event) = event.dyn_ref::<web_sys::MouseEvent>() else {
                            return;
                        };
                        let rect = input_canvas.get_bounding_client_rect();
                        if rect.width() <= 0.0 || rect.height() <= 0.0 {
                            return;
                        }
                        let screen = [
                            (2.0 * (event.client_x() as f64 - rect.left())
                                / rect.width()
                                - 1.0) as f32,
                            (1.0 - 2.0 * (event.client_y() as f64 - rect.top())
                                / rect.height()) as f32,
                        ];
                        selected = crate::picking::pick(
                            &scene,
                            camera.world_from_screen(screen),
                        );
                        renderer.set_scene(&scene, selected);
                    }
                    "example triangle" => {
                        history.edit(&mut scene, Scene::toggle_extra);
                        selected = selected.filter(|id| scene.contains(*id));
```

Replace it with:

```rust
                match line.as_str() {
                    "canvas" => return,
                    "example triangle" => {
                        history.edit(&mut scene, Scene::toggle_extra);
                        selected = selected.filter(|id| scene.contains(*id));
```

## <a id="checkpoint-14b-ray"></a>4. Turn a screen point into a bounded ray

### 1. `src/camera.rs`

Edit these blocks from bottom to top. Keep the surrounding lines unchanged.

**Locate this block:**

```rust
        assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
        assert!((0.0..1.0).contains(&screen[2]));
    }
```

Replace it with:

```rust
        assert!(screen[0].abs() < 1.0e-9 && screen[1].abs() < 1.0e-9);
        assert!((0.0..1.0).contains(&screen[2]));
    }

    #[test]
    fn a_projected_point_lies_on_its_unprojected_ray() {
        let camera = Camera::default();
        let world = Point::new(0.4, 0.2, 0.75);
        let screen = camera
            .view_projection()
            .transform_point(&world);
        let ray = camera
            .ray([screen[0] as f32, screen[1] as f32])
            .unwrap();
        let offset = &world - &ray.origin;
        assert!(offset.cross(&ray.direction).magnitude() < 1.0e-6);
        assert!(offset.dot(&ray.direction) > 0.0);
        assert!(offset.magnitude() < ray.max_distance);
        assert!(camera.ray([f32::NAN, 0.0]).is_none());
    }
```

**Locate this block:**

```rust
    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }
}

#[cfg(test)]
```

Replace it with:

```rust
    pub fn uniform(&self) -> [f32; 16] {
        self.view_projection().to_f32()
    }

    pub fn ray(&self, screen: [f32; 2]) -> Option<Ray> {
        if screen
            .iter()
            .any(|value| !value.is_finite() || value.abs() > 1.0)
        {
            return None;
        }
        let inverse = self.view_projection().inverse()?;
        let origin = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            0.0,
        ));
        let end = inverse.transform_point(&Point::new(
            screen[0] as f64,
            screen[1] as f64,
            1.0,
        ));
        let delta = &end - &origin;
        let max_distance = delta.magnitude();
        if !max_distance.is_finite() || max_distance <= 0.0 {
            return None;
        }
        Some(Ray {
            origin,
            direction: delta.normalized(),
            max_distance,
        })
    }
}

#[cfg(test)]
```

**Locate this block:**

```rust
use session_rust::{Point, Vector, Xform};

pub struct Camera {
    pub center: [f64; 2],
    pub distance: f64,
```

Replace it with:

```rust
use session_rust::{Point, Vector, Xform};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
    pub max_distance: f64,
}

pub struct Camera {
    pub center: [f64; 2],
    pub distance: f64,
```

## Build & Run

![Recorded visual output for Lesson 2.5 (14b-ray)](../../screenshots/journey/14b-ray-browser.png)

```sh
npm --prefix ../session_tests run course -- dependencies 2.5
cd workspace/journey
REGEN_PROTO=0 cargo build --lib --locked --target wasm32-unknown-unknown -j1
REGEN_PROTO=0 CARGO_BUILD_JOBS=1 trunk serve --port 8780
cd ../..
npm --prefix ../session_tests run course -- check 2.5
npm --prefix ../session_tests run course -- save 2.5
```

Continue with [2.6 · Perspective picking and orbit navigation](15-perspective.md).
