# 11 · Separate face and edge colors and keep large-object dragging live

[Previous](current-10.md) · [Sequence](extend-integrated-tutorial.md) · [Next](command-line-walkthrough.md)

Continue in the same checkpoint workspace. Complete the edits below before compiling.

![Ownership and data flow](illustrations/extend-ui.svg)

Store face and edge display overrides separately from authored source colors. Use the spare instance word for packed edge RGB without growing the GPU object stride. A per-channel Original button removes the override; Save/Open retains both channels and migrates older single-color archives. Mesh gestures cache source-to-render addresses and patch only the selected neighborhood during pointer movement, committing source geometry once on release. Include object placement revisions in the cloud raster cache key so a cloud follows its gumball while the camera stays still.

### `../session_rust/src/mesh.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub fn invalidate_triangle_bvh(&mut self) {
```

**ADD ABOVE**

```rust
    /// Maintained name for dropping geometry-derived caches.
    pub fn clear_triangle_bvh(&mut self) {
        self.invalidate_triangle_bvh();
    }
```

### `src/app/deform.rs`

**TYPE THIS**

**CURRENT**

```rust

fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<usize>, String> {
    match target {
```

**REPLACE WITH**

```rust

pub(crate) fn mesh_keys(mesh: &Mesh, target: Target) -> Result<Vec<usize>, String> {
    match target {
```

**TYPE THIS**

**CURRENT**

```rust
            }
            mesh.triangulation.clear();
            // The identity transform invalidates kernel render/BVH caches in both the frozen
            // course kernel and the maintained kernel without changing the edited positions.
            mesh.transform(&Xform::identity());
            Geometry::Mesh(Rc::new(mesh))
```

**REPLACE WITH**

```rust
            }
            // A subobject move preserves connectivity, including authored hole triangulations.
            mesh.clear_triangle_bvh();
            Geometry::Mesh(Rc::new(mesh))
```

### `src/app/feedback.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub color: Option<[u8; 3]>,
```

**ADD BELOW**

```rust
    pub edge_color: Option<[u8; 3]>,
    pub has_faces: bool,
```

### `src/app/inspection.rs`

**TYPE THIS**

**CURRENT**

```rust
    snapshot["color_count"] = serde_json::json!(state.scene.colors.len());
```

**ADD BELOW**

```rust
    snapshot["edge_color_count"] = serde_json::json!(state.scene.edge_colors.len());
```

### `src/app/mesh_preview.rs`

**NEW FILE · TYPE THIS**

```rust
//! Sparse render-only mesh gestures. Source replacement happens once on release.
use super::deform::{Target, mesh_keys};
use crate::engine::gpu::glyphs::GlyphPoint;
use crate::engine::gpu::patch::{Counts, Span};
use crate::engine::gpu::segments::{CylinderSegment, SegRows};
use crate::engine::gpu::{Gpu, Upload};
use session_rust::{Geometry, Mesh, RenderVertex, Xform};
use std::collections::{HashMap, HashSet};

pub struct MeshPreview {
    vertices: Vec<(usize, RenderVertex)>,
    pipes: Vec<([usize; 2], CylinderSegment)>,
    spheres: Vec<(usize, GlyphPoint)>,
    dots: Vec<(usize, GlyphPoint)>,
    span: Span,
}
pub struct Gesture {
    vertices: Vec<(u32, RenderVertex, bool)>,
    pipes: Vec<(u32, CylinderSegment, [bool; 2])>,
    spheres: Vec<(u32, GlyphPoint, bool)>,
    dots: Vec<(u32, GlyphPoint, bool)>,
}
pub fn mesh(geometry: &Geometry) -> Option<&Mesh> {
    match geometry {
        Geometry::Mesh(mesh) => Some(mesh),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::Mesh(mesh) => Some(mesh),
            _ => None,
        },
        _ => None,
    }
}
impl MeshPreview {
    pub(crate) fn capture(
        up: &Upload,
        span: Span,
        local: Counts,
        geometry: &Geometry,
    ) -> Option<Self> {
        let mesh = mesh(geometry)?;
        if span.count.ribbons != 0 {
            return None;
        }
        let mut keys = mesh.vertices();
        if mesh.color_mode == session_rust::mesh::ColorMode::FACECOLORS
            && mesh.get_facecolors().len() == mesh.face.len()
        {
            keys.clear();
            for face in mesh.faces() {
                let ring = &mesh.face[&face];
                let triangles = mesh
                    .triangulation
                    .get(&face)
                    .filter(|t| !t.is_empty())
                    .cloned()
                    .unwrap_or_else(|| {
                        (1..ring.len().saturating_sub(1))
                            .map(|i| [ring[0], ring[i], ring[i + 1]])
                            .collect()
                    });
                for triangle in triangles {
                    if triangle.iter().all(|key| mesh.vertex.contains_key(key)) {
                        keys.extend(triangle);
                    }
                }
            }
        }
        if keys.len() != span.count.verts as usize {
            return None;
        }
        let vertices = keys
            .into_iter()
            .zip(
                up.arena.verts[local.verts as usize..(local.verts + span.count.verts) as usize]
                    .iter()
                    .copied(),
            )
            .collect();
        let mut seen = HashSet::new();
        let mut edges = Vec::new();
        for face in mesh.faces() {
            let ring = &mesh.face[&face];
            for i in 0..ring.len() {
                let pair = [
                    ring[i].min(ring[(i + 1) % ring.len()]),
                    ring[i].max(ring[(i + 1) % ring.len()]),
                ];
                if seen.insert(pair) {
                    edges.push(pair);
                }
            }
        }
        let pipes = (local.pipes..local.pipes + span.count.pipes)
            .map(|i| {
                Some((
                    *edges.get(*up.seg.pipe_ids.get(i as usize)? as usize)?,
                    up.seg.pipes[i as usize],
                ))
            })
            .collect::<Option<Vec<_>>>()?;
        // Marker provenance is accepted only when source positions are unambiguous.
        let mut positions = HashMap::new();
        for (&key, v) in &mesh.vertex {
            let p = [v.x as f32, v.y as f32, v.z as f32].map(f32::to_bits);
            positions
                .entry(p)
                .and_modify(|value| *value = None)
                .or_insert(Some(key));
        }
        let markers = |items: &[GlyphPoint]| {
            items
                .iter()
                .map(|g| Some(((*positions.get(&g.center.map(f32::to_bits))?)?, *g)))
                .collect::<Option<Vec<_>>>()
        };
        Some(Self {
            vertices,
            pipes,
            spheres: markers(
                &up.glyph.spheres
                    [local.spheres as usize..(local.spheres + span.count.spheres) as usize],
            )?,
            dots: markers(
                &up.glyph.dots[local.dots as usize..(local.dots + span.count.dots) as usize],
            )?,
            span,
        })
    }
    pub fn begin(&self, geometry: &Geometry, target: Target) -> Option<Gesture> {
        let mesh = mesh(geometry)?;
        let selected: HashSet<_> = mesh_keys(mesh, target).ok()?.into_iter().collect();
        let mut affected = selected.clone();
        for ring in mesh.face.values() {
            if ring.iter().any(|k| selected.contains(k)) {
                affected.extend(ring);
            }
        }
        let vertices = self
            .vertices
            .iter()
            .enumerate()
            .filter(|(_, (k, _))| affected.contains(k))
            .map(|(i, (k, v))| (self.span.start.verts + i as u32, *v, selected.contains(k)))
            .collect();
        let pipes = self
            .pipes
            .iter()
            .enumerate()
            .filter(|(_, (keys, _))| keys.iter().any(|k| affected.contains(k)))
            .map(|(i, (keys, p))| {
                (
                    self.span.start.pipes + i as u32,
                    *p,
                    keys.map(|k| selected.contains(&k)),
                )
            })
            .collect();
        let markers = |items: &[(usize, GlyphPoint)], start| {
            items
                .iter()
                .enumerate()
                .filter(|(_, (k, _))| affected.contains(k))
                .map(|(i, (k, g))| (start + i as u32, *g, selected.contains(k)))
                .collect()
        };
        Some(Gesture {
            vertices,
            pipes,
            spheres: markers(&self.spheres, self.span.start.spheres),
            dots: markers(&self.dots, self.span.start.dots),
        })
    }
    pub fn allocated_bytes(&self) -> usize {
        self.vertices.capacity() * std::mem::size_of::<(usize, RenderVertex)>()
            + self.pipes.capacity() * std::mem::size_of::<([usize; 2], CylinderSegment)>()
            + (self.spheres.capacity() + self.dots.capacity())
                * std::mem::size_of::<(usize, GlyphPoint)>()
    }
}
impl Gesture {
    pub fn apply(&self, gpu: &mut Gpu, delta: &Xform, restore: bool) {
        let position = |p: [f32; 3]| {
            let m = &delta.m;
            std::array::from_fn(|r| {
                (m[r] * p[0] as f64 + m[4 + r] * p[1] as f64 + m[8 + r] * p[2] as f64 + m[12 + r])
                    as f32
            })
        };
        for &(index, mut v, moved) in &self.vertices {
            if !restore {
                if moved {
                    v.position = position(v.position);
                }
                v.normal = [0.; 3];
            }
            gpu.arena.patch_vertices(&gpu.ctx, index, &[v]);
        }
        for &(index, mut p, moved) in &self.pipes {
            if !restore {
                if moved[0] {
                    p.p0 = position(p.p0);
                }
                if moved[1] {
                    p.p1 = position(p.p1);
                }
                // During a gesture, finite triangle visibility supplies the occlusion test.
                p.facing = u32::MAX;
            }
            gpu.segments.patch_pipes(
                &gpu.ctx,
                index,
                &SegRows {
                    pipes: vec![p],
                    ..Default::default()
                },
            );
        }
        for (items, sphere) in [(&self.spheres, true), (&self.dots, false)] {
            for &(index, mut g, moved) in items {
                if !restore {
                    if moved {
                        g.center = position(g.center);
                    }
                    g.facing = u32::MAX;
                    g.facing_ext = [u32::MAX; 2];
                }
                gpu.glyphs.patch_marker(&gpu.ctx, index, sphere, g);
            }
        }
        gpu.objects.geometry_changed();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::{Walk, WalkCx, walk_geometry};
    use session_rust::Point;
    use std::rc::Rc;
    #[test]
    fn large_mesh_gesture_only_patches_local_neighborhood() {
        let mut mesh = Mesh::new();
        for y in 0..101 {
            for x in 0..101 {
                mesh.add_vertex(Point::new(x as f64, y as f64, 0.), Some(y * 101 + x));
            }
        }
        for y in 0..100 {
            for x in 0..100 {
                let a = y * 101 + x;
                mesh.add_face(vec![a, a + 1, a + 102], None);
                mesh.add_face(vec![a, a + 102, a + 101], None);
            }
        }
        let geometry = Geometry::Mesh(Rc::new(mesh));
        let mut up = Upload::default();
        walk_geometry(
            &mut Walk::of(&mut up),
            &WalkCx {
                vert_base: 0,
                cloud_px: 0.,
                row: 0,
            },
            &geometry,
        );
        let span = Span {
            start: Counts::default(),
            count: Counts::of(&up),
        };
        let preview = MeshPreview::capture(&up, span, Counts::default(), &geometry).unwrap();
        let gesture = preview.begin(&geometry, Target::Face(10000)).unwrap();
        assert!(
            gesture.vertices.len() < 30,
            "drag cost follows adjacency, not the 10,201-vertex mesh"
        );
        assert_eq!(gesture.vertices.iter().filter(|v| v.2).count(), 3);
        assert!(gesture.pipes.len() < 100);
        assert_eq!(
            super::mesh(&geometry).unwrap().vertex[&0].z,
            0.,
            "preview does not mutate source"
        );
    }
}
```

### `src/app/mod.rs`

**TYPE THIS**

**CURRENT**

```rust
pub mod manifest;
```

**ADD BELOW**

```rust
pub mod mesh_preview;
```

### `src/app/scene.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub colors: HashMap<(usize, Rc<str>), [u8; 3]>,
```

**ADD BELOW**

```rust
    pub edge_colors: HashMap<(usize, Rc<str>), [u8; 3]>,
```

**TYPE THIS**

**CURRENT**

```rust
    surface_previews: Vec<Option<crate::app::surface_preview::SurfacePreview>>,
```

**ADD BELOW**

```rust
    pub(crate) mesh_previews: Vec<Option<crate::app::mesh_preview::MeshPreview>>,
```

**TYPE THIS**

**CURRENT**

```rust
            colors: HashMap::new(),
```

**ADD BELOW**

```rust
            edge_colors: HashMap::new(),
```

**TYPE THIS**

**CURRENT**

```rust
            surface_previews: Vec::new(),
```

**ADD BELOW**

```rust
            mesh_previews: Vec::new(),
```

**TYPE THIS**

**CURRENT**

```rust
        self.colors.clear();
```

**ADD BELOW**

```rust
        self.edge_colors.clear();
```

**TYPE THIS**

**CURRENT**

```rust
        self.surface_previews.clear();
```

**ADD BELOW**

```rust
        self.mesh_previews.clear();
```

**TYPE THIS**

**CURRENT**

```rust
        let guid: Rc<str> = Rc::from(guid);
```

**ADD ABOVE**

```rust
        if let Some(color) = self.edge_colors.get(&(owner, Rc::from(guid))) {
            let object = self.tables.obj.rows.last_mut().expect("row just appended");
            object.edge_color = u32::from_le_bytes([color[0], color[1], color[2], 255]);
            object.flags |= Instance::FLAG_EDGE_COLOR;
        }
```

**TYPE THIS**

**CURRENT**

```rust
        self.surface_previews.push(None);
```

**ADD BELOW**

```rust
        self.mesh_previews.push(None);
```

**TYPE THIS**

**CURRENT**

```rust
                );
            let o = self.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;
            let ribbon_end = self.tables.seg.ribbons.len();
```

**REPLACE WITH**

```rust
                );
            self.mesh_previews[row as usize] = crate::app::mesh_preview::MeshPreview::capture(
                &self.tables,
                span,
                start.minus(self.uploaded),
                geom,
            );
            let o = self.tables.obj.rows.last_mut().unwrap();
            o.flags |= r.flags;
            o.bounds = r.bounds;
            o.spacing = r.spacing;
            o.faces = r.faces;
            if r.faces {
                o.flags |= Instance::FLAG_HAS_FACES;
            }
            let ribbon_end = self.tables.seg.ribbons.len();
```

**TYPE THIS**

**CURRENT**

```rust
        gpu.arena.patch(&gpu.ctx, span.start, &up.arena);
```

**ADD ABOVE**

```rust
        self.mesh_previews[row as usize] =
            crate::app::mesh_preview::MeshPreview::capture(&up, span, Counts::default(), geometry);
```

**TYPE THIS**

**CURRENT**

```rust
            .sum::<usize>()
```

**ADD BELOW**

```rust
            + self
                .mesh_previews
                .iter()
                .flatten()
                .map(|p| p.allocated_bytes())
                .sum::<usize>()
```

### `src/app/session_io.rs`

**TYPE THIS**

**CURRENT**

```rust
    colors: Vec<(usize, String, [u8; 3])>,
```

**ADD BELOW**

```rust
    #[serde(default)]
    edge_colors: Option<Vec<(usize, String, [u8; 3])>>,
```

**TYPE THIS**

**CURRENT**

```rust
        colors,
```

**ADD BELOW**

```rust
        edge_colors: Some({
            let mut colors: Vec<_> = scene
                .edge_colors
                .iter()
                .map(|((doc, id), color)| (*doc, id.to_string(), *color))
                .collect();
            colors.sort();
            colors
        }),
```

**TYPE THIS**

**CURRENT**

```rust
        .locked
        .into_iter()
        .map(|(doc, id)| (doc, Rc::from(id)))
```

**ADD BELOW**

```rust
        .collect();
    // Older archives applied their single color to faces and edges alike.
    scene.edge_colors = metadata
        .edge_colors
        .unwrap_or_else(|| metadata.colors.clone())
        .into_iter()
        .map(|(doc, id, color)| ((doc, Rc::from(id)), color))
```

**TYPE THIS**

**CURRENT**

```rust
            .insert(scene.identity_of(1).unwrap(), [240, 80, 30]);
```

**ADD BELOW**

```rust
        scene
            .edge_colors
            .insert(scene.identity_of(1).unwrap(), [30, 80, 240]);
```

**TYPE THIS**

**CURRENT**

```rust
        assert_eq!(restored.colors, scene.colors);
```

**ADD BELOW**

```rust
        assert_eq!(restored.edge_colors, scene.edge_colors);
        let mut archive = Archive::decode(&bytes[MAGIC.len()..]).unwrap();
        let mut old: serde_json::Value = serde_json::from_slice(&archive.metadata).unwrap();
        old.as_object_mut().unwrap().remove("edge_colors");
        archive.metadata = serde_json::to_vec(&old).unwrap();
        let mut legacy = MAGIC.to_vec();
        archive.encode(&mut legacy).unwrap();
        let legacy = open(&legacy).unwrap();
        assert_eq!(
            legacy.edge_colors, legacy.colors,
            "legacy overrides still color both channels"
        );
```

### `src/app/splitting.rs`

**TYPE THIS**

**CURRENT**

```rust
        let color = self.colors.get(&(doc, Rc::clone(&guid))).copied();
```

**ADD BELOW**

```rust
        let edge_color = self.edge_colors.get(&(doc, Rc::clone(&guid))).copied();
```

**TYPE THIS**

**CURRENT**

```rust
            if let Some(color) = color {
                self.colors.insert((doc, Rc::from(id)), color);
            }
```

**REPLACE WITH**

```rust
            if let Some(color) = color {
                self.colors.insert((doc, Rc::from(id.as_str())), color);
            }
            if let Some(color) = edge_color {
                self.edge_colors.insert((doc, Rc::from(id.as_str())), color);
            }
```

### `src/app/ui.rs`

**TYPE THIS**

**CURRENT**

```rust
    let mut color = row.color.unwrap_or([180, 180, 180]);
    let response = ui
        .menu_button(
            egui::RichText::new("■").color(egui::Color32::from_rgb(color[0], color[1], color[2])),
            |ui| {
                ui.label("Object and child colors");
                for colors in [
                    [
                        ("Red", [230, 65, 55]),
                        ("Orange", [240, 145, 45]),
                        ("Yellow", [240, 210, 60]),
                    ],
                    [
                        ("Green", [60, 170, 100]),
                        ("Blue", [65, 130, 225]),
                        ("Violet", [160, 85, 210]),
                    ],
                    [
                        ("White", [245, 245, 245]),
                        ("Gray", [150, 150, 150]),
                        ("Black", [35, 35, 35]),
                    ],
                ] {
                    ui.horizontal(|ui| {
                        for (name, rgb) in colors {
                            let response = ui
                                .add_sized(
                                    [48., 28.],
                                    egui::Button::new(
                                        egui::RichText::new("■")
                                            .color(egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2])),
                                    ),
                                )
                                .on_hover_text(name);
                            let key =
                                format!("color/{index}/{:02x}{:02x}{:02x}", rgb[0], rgb[1], rgb[2]);
                            record(controls, &key, name, &response);
                            if response.clicked() {
                                *action = Some(key);
                                ui.close();
                            }
                        }
                    });
                }
                ui.separator();
                let mut changed = false;
                for (channel, value) in ["R", "G", "B"].into_iter().zip(color.iter_mut()) {
                    changed |= ui
                        .add(egui::Slider::new(value, 0..=255).text(channel))
                        .changed();
                }
                if changed {
                    *action = Some(format!(
                        "color/{index}/{:02x}{:02x}{:02x}",
                        color[0], color[1], color[2]
                    ));
                }
            },
        )
        .response
        .on_hover_text("Change object and child colors");
    record(
```

**REPLACE WITH**

```rust
    let mut color = row.color.unwrap_or([180, 180, 180]);
    let response = egui::containers::menu::MenuButton::new(
        egui::RichText::new("■").color(egui::Color32::from_rgb(color[0], color[1], color[2])),
    )
    .config(
        egui::containers::menu::MenuConfig::default()
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside),
    )
    .ui(ui, |ui| {
        let channel_id = egui::Id::new(("layer-color-channel", index));
        let mut edge = ui
            .ctx()
            .data_mut(|data| data.get_temp::<bool>(channel_id).unwrap_or(false))
            && row.has_faces;
        if row.has_faces {
            ui.horizontal(|ui| {
                for (label, value) in [("Faces", false), ("Edges", true)] {
                    let response = ui.selectable_label(edge == value, label);
                    record(
                        controls,
                        &format!("color-channel/{index}/{label}"),
                        label,
                        &response,
                    );
                    if response.clicked() {
                        edge = value;
                    }
                }
            });
        } else {
            ui.label("Object and child colors");
        }
        ui.ctx().data_mut(|data| data.insert_temp(channel_id, edge));
        let channel = if edge { "edge" } else { "face" };
        color = if edge { row.edge_color } else { row.color }.unwrap_or([180; 3]);
        let response = ui
            .button("Original")
            .on_hover_text("Restore the source colors for this channel and its children");
        let key = format!("color/{index}/{channel}/original");
        record(controls, &key, "Original", &response);
        if response.clicked() {
            *action = Some(key);
            ui.close();
        }
        ui.separator();
        for colors in [
            [
                ("Red", [230, 65, 55]),
                ("Orange", [240, 145, 45]),
                ("Yellow", [240, 210, 60]),
            ],
            [
                ("Green", [60, 170, 100]),
                ("Blue", [65, 130, 225]),
                ("Violet", [160, 85, 210]),
            ],
            [
                ("White", [245, 245, 245]),
                ("Gray", [150, 150, 150]),
                ("Black", [35, 35, 35]),
            ],
        ] {
            ui.horizontal(|ui| {
                for (name, rgb) in colors {
                    let response = ui
                        .add_sized(
                            [48., 28.],
                            egui::Button::new(
                                egui::RichText::new("■")
                                    .color(egui::Color32::from_rgb(rgb[0], rgb[1], rgb[2])),
                            ),
                        )
                        .on_hover_text(name);
                    let key = format!(
                        "color/{index}/{channel}/{:02x}{:02x}{:02x}",
                        rgb[0], rgb[1], rgb[2]
                    );
                    record(controls, &key, name, &response);
                    if response.clicked() {
                        *action = Some(key);
                        ui.close();
                    }
                }
            });
        }
        ui.separator();
        let mut changed = false;
        for (channel, value) in ["R", "G", "B"].into_iter().zip(color.iter_mut()) {
            changed |= ui
                .add(egui::Slider::new(value, 0..=255).text(channel))
                .changed();
        }
        if changed {
            *action = Some(format!(
                "color/{index}/{channel}/{:02x}{:02x}{:02x}",
                color[0], color[1], color[2]
            ));
        }
    })
    .0
    .on_hover_text("Change object and child colors");
    record(
```

### `src/engine/gpu/glyphs.rs`

**TYPE THIS**

**CURRENT**

```rust
impl GlyphLane {
```

**ADD BELOW**

```rust
    pub(crate) fn patch_marker(
        &mut self,
        ctx: &GpuCtx,
        index: u32,
        sphere: bool,
        glyph: GlyphPoint,
    ) {
        let table = if sphere {
            &mut self.spheres
        } else {
            &mut self.dots
        };
        table.buf.write_at(ctx, index, &[glyph]);
    }
```

### `src/engine/gpu/instance.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub const FLAG_COLOR: u32 = 1 << 8;
```

**ADD BELOW**

```rust
    pub const FLAG_EDGE_COLOR: u32 = 1 << 9;
    pub const FLAG_HAS_FACES: u32 = 1 << 10;
```

**TYPE THIS**

**CURRENT**

```rust
    fn instance_mirror() {
        let rust = ["model", "color", "flags", "_pad0", "spacing"];
        assert_eq!(wgsl_fields(SCENE, "Instance"), rust, "Instance fields");
```

**REPLACE WITH**

```rust
    fn instance_mirror() {
        let rust = ["model", "color", "flags", "_pad0", "spacing", "edge_color"];
        assert_eq!(wgsl_fields(SCENE, "Instance"), rust, "Instance fields");
```

### `src/engine/gpu/mod.rs`

**TYPE THIS**

**CURRENT**

```rust

    pub fn set_object_color(&mut self, row: u32, color: [u8; 3]) {
        self.objects.set_color(&self.ctx, row, color);
        self.splat.invalidate();
```

**REPLACE WITH**

```rust

    pub fn set_object_color(&mut self, row: u32, edge: bool, color: Option<[u8; 3]>) {
        self.objects.set_color(&self.ctx, row, edge, color);
        self.splat.invalidate();
```

### `src/engine/gpu/objects.rs`

**TYPE THIS**

**CURRENT**

```rust
    pub color: [f32; 4],
```

**ADD BELOW**

```rust
    pub edge_color: u32,
```

**TYPE THIS**

**CURRENT**

```rust
            flags,
```

**ADD ABOVE**

```rust
            edge_color: 0,
```

**TYPE THIS**

**CURRENT**

```rust
                spacing: r.spacing,
                _pad: 0,
            });
```

**REPLACE WITH**

```rust
                spacing: r.spacing,
                _pad: r.edge_color,
            });
```

**TYPE THIS**

**CURRENT**

```rust
    /// Change display color without rebuilding geometry or placement.
    pub fn set_color(&mut self, ctx: &GpuCtx, row: u32, color: [u8; 3]) {
        if let Some(r) = self.rows.get_mut(row as usize) {
            r.color = [
                color[0] as f32 / 255.,
                color[1] as f32 / 255.,
                color[2] as f32 / 255.,
                1.,
            ];
            r.flags |= Instance::FLAG_COLOR;
            self.buffer.write_at(ctx, row, std::slice::from_ref(r));
```

**REPLACE WITH**

```rust
    /// Change display color without rebuilding geometry or placement.
    pub fn set_color(&mut self, ctx: &GpuCtx, row: u32, edge: bool, color: Option<[u8; 3]>) {
        if let Some(r) = self.rows.get_mut(row as usize) {
            let flag = if edge {
                Instance::FLAG_EDGE_COLOR
            } else {
                Instance::FLAG_COLOR
            };
            r.flags &= !flag;
            if color.is_some() {
                r.flags |= flag;
            }
            if edge {
                r._pad = color
                    .map(|c| u32::from_le_bytes([c[0], c[1], c[2], 255]))
                    .unwrap_or(0);
            } else {
                r.color = color
                    .map(|c| {
                        [
                            c[0] as f32 / 255.,
                            c[1] as f32 / 255.,
                            c[2] as f32 / 255.,
                            1.,
                        ]
                    })
                    .unwrap_or([1.; 4]);
            }
            self.buffer.write_at(ctx, row, std::slice::from_ref(r));
```

**TYPE THIS**

**CURRENT**

```rust
        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }
```

**REPLACE WITH**

```rust
        self.buffer.write_at(ctx, row, std::slice::from_ref(r));
    }

    pub(crate) fn geometry_changed(&mut self) {
        self.geometry_revision = self.geometry_revision.wrapping_add(1);
    }
```

### `src/engine/gpu/splat.rs`

**TYPE THIS**

**CURRENT**

```rust
    point_count: u32,
```

**ADD BELOW**

```rust
    geometry: u64,
```

**TYPE THIS**

**CURRENT**

```rust
            point_count,
```

**ADD BELOW**

```rust
            geometry: cx.objects.geometry_revision(),
```

### `src/shaders/glyph.wgsl`

**TYPE THIS**

**CURRENT**

```wgsl
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    var color = object_color(g.color, inst);
    if ((inst.flags & FLAG_SELECTED) != 0u) {
```

**REPLACE WITH**

```wgsl
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    var color = edge_color(g.color, inst);
    if ((inst.flags & FLAG_SELECTED) != 0u) {
```

### `src/shaders/ribbon.wgsl`

**TYPE THIS**

**CURRENT**

```wgsl
    o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
    var color = object_color(unpack4x8unorm(seg.color), inst);
    if (selected) {
```

**REPLACE WITH**

```wgsl
    o.pos = vec4<f32>(ndc * clip.w, clip.z, clip.w);
    var color = edge_color(unpack4x8unorm(seg.color), inst);
    if (selected) {
```

### `src/shaders/scene.wgsl`

**TYPE THIS**

**CURRENT**

```wgsl
    spacing: f32,
```

**ADD BELOW**

```wgsl
    edge_color: u32,
```

**TYPE THIS**

**CURRENT**

```wgsl
const FACING_UNKNOWN: u32 = 0xffffffffu;
```

**ADD ABOVE**

```wgsl
fn edge_color(authored: vec4<f32>, inst: Instance) -> vec4<f32> {
    if ((inst.flags & 1024u) == 0u) { return object_color(authored, inst); }
    if ((inst.flags & 512u) != 0u) {
        return vec4<f32>(unpack4x8unorm(inst.edge_color).rgb, authored.a);
    }
    return authored;
}
```

### `src/shaders/sphere.wgsl`

**TYPE THIS**

**CURRENT**

```wgsl
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    var color = object_color(g.color, inst);
    if ((inst.flags & FLAG_SELECTED) != 0u) {
```

**REPLACE WITH**

```wgsl
    o.pos = vec4<f32>(clip.xy + off, clip.z, clip.w);
    var color = edge_color(g.color, inst);
    if ((inst.flags & FLAG_SELECTED) != 0u) {
```

### `src/state/edit.rs`

**TYPE THIS**

**CURRENT**

```rust
    source: Option<session_rust::Geometry>,
    origin: Point,
```

**ADD BELOW**

```rust
    mesh_preview: Option<crate::app::mesh_preview::Gesture>,
```

**TYPE THIS**

**CURRENT**

```rust
        self.dragging = Some(GizmoDrag {
```

**ADD ABOVE**

```rust
        let target = crate::app::deform::Target::selected(&self.selection);
        let mesh_preview = target.and_then(|target| {
            self.scene
                .mesh_previews
                .get(row as usize)?
                .as_ref()?
                .begin(self.scene.geometry(row)?, target)
        });
```

**TYPE THIS**

**CURRENT**

```rust
            origin: gizmo.origin.clone(),
```

**ADD BELOW**

```rust
            mesh_preview,
```

**TYPE THIS**

**CURRENT**

```rust
            let local = &(&back * &Xform::from_matrix(delta)) * &place;
```

**ADD BELOW**

```rust
            if let Some(preview) = active.mesh_preview.as_ref() {
                preview.apply(&mut self.gpu, &local, false);
                let origin = active.origin.transformed(&Xform::from_matrix(delta));
                self.gpu.bounds.grow(origin.to_f32());
                if let Some(gizmo) = self.gizmo.as_mut() {
                    gizmo.origin = origin;
                }
                self.upload_gizmo();
                self.touch();
                return true;
            }
```

**TYPE THIS**

**CURRENT**

```rust
        if let Some(active) = self.dragging.take() {
            if active.target.is_some() {
                self.restore_source_render(active.row);
```

**REPLACE WITH**

```rust
        if let Some(active) = self.dragging.take() {
            if let Some(preview) = active.mesh_preview.as_ref() {
                preview.apply(&mut self.gpu, &Xform::identity(), true);
                self.restore_edit_selection(active.row);
            } else if active.target.is_some() {
                self.restore_source_render(active.row);
```

### `src/state/panel.rs`

**TYPE THIS**

**CURRENT**

```rust
        if let Some(value) = key.strip_prefix("color/") {
            if let Some((index, hex)) = value.split_once('/')
                && let (Ok(index), Ok(rgb)) = (index.parse::<usize>(), u32::from_str_radix(hex, 16))
                && index < self.hierarchy.nodes.len()
            {
                let color = [(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8];
                for row in self.hierarchy.targets(index) {
                    if let Some(id) = self.scene.identity_of(row) {
                        self.scene.colors.insert(id, color);
                        self.gpu.set_object_color(row, color);
                    }
```

**REPLACE WITH**

```rust
        if let Some(value) = key.strip_prefix("color/") {
            let parts: Vec<_> = value.split('/').collect();
            if parts.len() == 3
                && let Ok(index) = parts[0].parse::<usize>()
                && index < self.hierarchy.nodes.len()
                && matches!(parts[1], "face" | "edge")
            {
                let edge = parts[1] == "edge";
                let color = if parts[2] == "original" {
                    None
                } else {
                    let Ok(rgb) = u32::from_str_radix(parts[2], 16) else {
                        return;
                    };
                    Some([(rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8])
                };
                for row in self.hierarchy.targets(index) {
                    if edge
                        && !self.gpu.objects.row(row).is_some_and(|r| {
                            r.flags & crate::engine::gpu::Instance::FLAG_HAS_FACES != 0
                        })
                    {
                        continue;
                    }
                    if let Some(id) = self.scene.identity_of(row) {
                        let colors = if edge {
                            &mut self.scene.edge_colors
                        } else {
                            &mut self.scene.colors
                        };
                        if let Some(color) = color {
                            colors.insert(id, color);
                        } else {
                            colors.remove(&id);
                        }
                        self.gpu.set_object_color(row, edge, color);
                    }
```

**TYPE THIS**

**CURRENT**

```rust
                color,
```

**ADD BELOW**

```rust
                edge_color: targets
                    .first()
                    .and_then(|row| self.scene.identity_of(*row))
                    .and_then(|id| self.scene.edge_colors.get(&id).copied())
                    .filter(|first| {
                        targets.iter().all(|row| {
                            self.scene
                                .identity_of(*row)
                                .is_some_and(|id| self.scene.edge_colors.get(&id) == Some(first))
                        })
                    }),
                has_faces: targets.iter().any(|row| {
                    self.gpu.objects.row(*row).is_some_and(|r| {
                        r.flags & crate::engine::gpu::Instance::FLAG_HAS_FACES != 0
                    })
                }),
```

### Check step 11

**Verification pending:** run the check below before continuing.

```bash
cargo check -j4 --lib
```

## Check

```bash
cargo xtest -j4 --lib
trunk serve --port 8780
```

Open <http://localhost:8780/?data=off&inspect=1>. Stop the server with **Ctrl+C**.

### Reproduce the screenshots

The screenshots use the small [nested fixture](extensions/nested.pb) and [manifest](extensions/nested.yaml), not private project files. Save both into your workspace:

```bash
cp "$COURSE_REPO/docs/extensions/nested.pb" assets/extension-nested.pb
cp "$COURSE_REPO/docs/extensions/nested.yaml" assets/extension-nested.yaml
```

Open <http://localhost:8780/?scene=extension-nested.yaml&data=off&inspect=1>.

## What changed

The docked workspace supports source mesh, curve, surface and compatible BRep subobject edits. Unsupported BRep trim reconstructions and incomplete streamed-source exports fail without discarding the retained scene.

## Try

Use the command walkthrough, tree selection and gumball controls. Every feature is present.

## Questions and answers

**What goes to the GPU?** Modeling rebuilds existing geometry lanes; panels change object flags; controls upload a small preview. The solid gumball owns a fixed mesh, an unlit shader and a bounded antialiasing tile.

**Why clear row selection after rebuilding?** Row numbers are upload addresses, not permanent identities. A rebuild can assign the same number to a different object.

**Where is the exact patch?** [step 1](extensions/integrated-1.patch), [step 2](extensions/integrated-2.patch), [step 3](extensions/integrated-3.patch), [step 4](extensions/integrated-4.patch), [step 5](extensions/integrated-5.patch), [step 6](extensions/integrated-6.patch), [step 7](extensions/integrated-7.patch), [step 8](extensions/integrated-8.patch), [step 9](extensions/integrated-9.patch), [step 10](extensions/integrated-10.patch), [step 11](extensions/integrated-11.patch). The patch and these visible instructions are generated from the same changes.

## Answers and next action

**How do I restore the source colors?** Click the item’s layer swatch, choose Faces or Edges when the item has surfaces, then click Original. Each channel resets independently. Points and curves use one Color picker. Parent controls apply to their children.

**Will a BVH make dragging faster?** A spatial index helps find a selection. Here the expensive mesh work happened after selection: full geometry copies and rendering conversions on every pointer event. The preview now patches the selected vertices and neighboring render data. The source is replaced once on release. Point-cloud placement changes also invalidate the cached raster image.

**Run now**, in the same learning workspace:

```bash
cargo check -j4 --lib
trunk serve --port 8780
```

Expected compiler result: `Finished` with no errors. Open <http://localhost:8780/?data=off&inspect=1>. Use the downloadable split.pb fixture from step 10. Expand the layer tree and click the Joined shell color swatch. Set Faces to Red and Edges to Blue. The shell should show red surfaces and blue boundary edges. Click Original for Faces: the authored gray surface returns while edges stay blue. Reset Edges to restore its authored edge colors. Save/Open retains independent overrides. Drag a mesh face and a cloud with a stationary camera; the geometry should follow throughout the gesture.

Stop the server with **Ctrl+C** before editing the next checkpoint. Then follow [Use the command line](command-line-walkthrough.md) to exercise the finished interface.

[Previous](current-10.md) · [Sequence](extend-integrated-tutorial.md) · [Next](command-line-walkthrough.md)

## Expected viewer result

The running viewer shows the shell with red faces and blue edges. Its layer color popup has separate Faces and Edges controls and an Original button that restores the authored colors for the active channel. The full command dock, toolbar and layer tree remain visible.

[![Full viewer result for current 11](screenshots/extensions-color-channels.png)](screenshots/extensions-color-channels.png)
