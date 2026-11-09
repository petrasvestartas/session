use super::State;
use crate::app::presentation::{Effect, EffectKind, Presentation, Target};
use crate::app::scene::FileDoc;
use crate::engine::gpu::{Instance, Pick};
use crate::engine::text::{TextLabel, TextPlacement};
use session_rust::{AABB, Point};
use std::collections::{HashMap, HashSet};

/// Geometry and text rows loaded for one step of the current topic.
pub struct LoadedStep {
    topic: usize,
    index: usize,
    rows: Vec<u32>,
    texts: Vec<u32>,
    complete: bool,
    played: bool,
}

/// A name and its resolved matrix span, if it has one.
#[derive(Clone, Copy)]
struct Resolved {
    row: u32,
    cells: Option<(u32, u32)>,
}

/// One effect whose targets were resolved after its session file loaded.
struct ActiveEffect {
    effect: Effect,
    targets: Vec<Resolved>,
    source: Vec<Resolved>,
    start: Vec<(u32, [f64; 3])>,
    morph_start: Vec<(u32, TextLabel)>,
    geometry_morph: Vec<GeometryMorph>,
    placements: Vec<(u32, session_rust::Xform)>,
    delta: [f64; 3],
    started_ms: Option<f64>,
}

struct GeometryMorph {
    source: u32,
    target: u32,
    from: [[f64; 3]; 2],
    to: [[f64; 3]; 2],
    local: [[f64; 3]; 2],
    place: session_rust::Xform,
}

struct MorphFrame {
    labels: Vec<(u32, TextLabel)>,
    covered: Vec<bool>,
}

fn smooth(t: f64) -> f64 {
    t * t * t * (10.0 + t * (-15.0 + 6.0 * t))
}

fn label_position(label: &TextLabel) -> [f64; 3] {
    match label.placement {
        TextPlacement::Screen { left, top } => [f64::from(left), f64::from(top), 0.0],
        TextPlacement::Anchor { world, .. }
        | TextPlacement::Nameplate { world, .. }
        | TextPlacement::WorldPlane { world, .. }
        | TextPlacement::WorldBillboard { world, .. } => world,
    }
}

fn label_height(label: &TextLabel) -> f64 {
    match label.placement {
        TextPlacement::WorldPlane { world_height, .. }
        | TextPlacement::WorldBillboard { world_height, .. } => world_height,
        _ => f64::from(label.font_size),
    }
}

fn move_label(label: &mut TextLabel, at: [f64; 3]) {
    match &mut label.placement {
        TextPlacement::Screen { left, top } => {
            *left = at[0] as f32;
            *top = at[1] as f32;
        }
        TextPlacement::Anchor { world, .. }
        | TextPlacement::Nameplate { world, .. }
        | TextPlacement::WorldPlane { world, .. }
        | TextPlacement::WorldBillboard { world, .. } => *world = at,
    }
}

fn size_label(label: &mut TextLabel, height: f64) {
    let height = height.max(0.0001);
    match &mut label.placement {
        TextPlacement::WorldPlane { world_height, .. }
        | TextPlacement::WorldBillboard { world_height, .. } => *world_height = height,
        _ => {
            let ratio = height as f32 / label.font_size.max(0.0001);
            label.font_size = (label.font_size * ratio).clamp(1.0, 256.0);
            label.line_height = (label.line_height * ratio).clamp(label.font_size, 1024.0);
        }
    }
}

fn mix_color(a: [u8; 4], b: [u8; 4], t: f64) -> [u8; 4] {
    std::array::from_fn(|i| {
        (f64::from(a[i]) + (f64::from(b[i]) - f64::from(a[i])) * t)
            .round()
            .clamp(0.0, 255.0) as u8
    })
}

fn interpolate_label(source: &TextLabel, target: &TextLabel, t: f64, text: String) -> TextLabel {
    let mut label = source.clone();
    label.text = text;
    let a = label_position(source);
    let b = label_position(target);
    move_label(
        &mut label,
        std::array::from_fn(|axis| a[axis] + (b[axis] - a[axis]) * t),
    );
    size_label(&mut label, label_height(target));
    if let (TextPlacement::Anchor { offset: a, .. }, TextPlacement::Anchor { offset: b, .. }) =
        (&source.placement, &target.placement)
    {
        let TextPlacement::Anchor { offset, .. } = &mut label.placement else {
            unreachable!()
        };
        *offset = [
            a[0] + (b[0] - a[0]) * t as f32,
            a[1] + (b[1] - a[1]) * t as f32,
        ];
    }
    label.color = mix_color(source.color, target.color, t);
    label.outline = source.outline + (target.outline - source.outline) * t as f32;
    label
}

/// Match repeated glyphs by occurrence; surplus glyphs exchange, fade or appear in place.
fn morph_labels(source: &[(u32, TextLabel)], target: &[(u32, TextLabel)], t: f64) -> MorphFrame {
    let mut matched = vec![None; source.len()];
    let mut covered = vec![false; target.len()];
    for (index, (_, label)) in source.iter().enumerate() {
        if let Some(other) = target
            .iter()
            .enumerate()
            .position(|(index, candidate)| !covered[index] && candidate.1.text == label.text)
        {
            matched[index] = Some(other);
            covered[other] = true;
        }
    }
    let center = if target.is_empty() {
        [0.0; 3]
    } else {
        let mut center = [0.0; 3];
        for (_, label) in target {
            let at = label_position(label);
            for axis in 0..3 {
                center[axis] += at[axis] / target.len() as f64;
            }
        }
        center
    };
    let mut labels = Vec::with_capacity(source.len());
    for (index, (row, source)) in source.iter().enumerate() {
        let label = if let Some(target_index) = matched[index] {
            let target = &target[target_index].1;
            interpolate_label(source, target, t, target.text.clone())
        } else {
            let mut label = source.clone();
            let at = label_position(source);
            move_label(
                &mut label,
                std::array::from_fn(|axis| at[axis] + (center[axis] - at[axis]) * t * 0.15),
            );
            label.color[3] = (f64::from(label.color[3]) * (1.0 - t)).round() as u8;
            label
        };
        labels.push((*row, label));
    }
    MorphFrame { labels, covered }
}

type CellSpans = Option<((u32, u32), Option<(u32, u32)>)>;

/// The deck, its cursor and everything loaded for the current topic.
pub struct PresentationState {
    spec: Presentation,
    base: String,
    index: usize,
    topic: usize,
    generation: u64,
    loading: Option<(usize, usize)>,
    loaded: Vec<LoadedStep>,
    initial: HashMap<u32, (session_rust::Xform, [f64; 3])>,
    effects: Vec<ActiveEffect>,
    transient: Vec<Target>,
    hover: Option<Pick>,
    pinned: Option<Pick>,
    pick_pin: bool,
    cursor: (f64, f64),
    lit: HashMap<u32, CellSpans>,
    gradients: HashSet<u32>,
    overlaps: usize,
    error: Option<String>,
    spatial_rows: HashSet<u32>,
    spatial_bounds: AABB,
    spatial_camera: crate::camera::Camera,
    spatial_fit: bool,
    spatial_items: Vec<(String, crate::app::manifest::TextItem)>,
    spatial_overlaps: usize,
    spatial_reflow_pending: bool,
    clear_on_part: bool,
}

impl PresentationState {
    /// A deck at its first step, before its first file arrives.
    fn new(spec: Presentation, base: String) -> Self {
        Self {
            spec,
            base,
            index: 0,
            topic: 0,
            generation: 0,
            loading: None,
            loaded: Vec::new(),
            initial: HashMap::new(),
            effects: Vec::new(),
            transient: Vec::new(),
            hover: None,
            pinned: None,
            pick_pin: false,
            cursor: (0.0, 0.0),
            lit: HashMap::new(),
            gradients: HashSet::new(),
            overlaps: 0,
            error: None,
            spatial_rows: HashSet::new(),
            spatial_bounds: AABB::empty(),
            spatial_camera: crate::camera::Camera::new(),
            spatial_fit: false,
            spatial_items: Vec::new(),
            spatial_overlaps: 0,
            spatial_reflow_pending: false,
            clear_on_part: false,
        }
    }

    /// Resolve a name against the currently loaded objects.
    fn resolve(scene: &crate::app::scene::Scene, target: &Target) -> Vec<Resolved> {
        scene
            .rows_named(&target.name)
            .into_iter()
            .filter_map(|row| {
                let cells = match target.cells {
                    Some(_) => scene
                        .matrix_shape(row)
                        .and_then(|shape| target.packed_range(shape)),
                    None => None,
                };
                if target.cells.is_some() && cells.is_none() {
                    None
                } else {
                    Some(Resolved { row, cells })
                }
            })
            .collect()
    }

    /// Source and target runs in authored order, not GPU row allocation order.
    fn ordered_labels(
        scene: &crate::app::scene::Scene,
        targets: &[Resolved],
    ) -> Vec<(u32, TextLabel)> {
        let mut labels: Vec<_> = scene
            .texts
            .iter()
            .filter(|text| targets.iter().any(|target| target.row == text.row))
            .map(|text| {
                (
                    text.name.clone(),
                    text.key.clone(),
                    text.row,
                    text.label.clone(),
                )
            })
            .collect();
        labels.sort_by(|a, b| {
            let order = |text: &String| {
                text.rsplit('/')
                    .next()
                    .and_then(|index| index.parse::<usize>().ok())
                    .unwrap_or(usize::MAX)
            };
            a.0.cmp(&b.0)
                .then_with(|| order(&a.1).cmp(&order(&b.1)))
                .then_with(|| a.2.cmp(&b.2))
        });
        labels
            .into_iter()
            .map(|(_, _, row, label)| (row, label))
            .collect()
    }

    /// The target under a pick, including an individual matrix entry.
    fn picked_target(scene: &crate::app::scene::Scene, pick: Pick) -> Option<Target> {
        let name = scene.row_link_name(pick.row)?.to_string();
        if pick.sub > 0
            && let Some(shape) = scene.matrix_shape(pick.row)
        {
            let at = pick.sub - 1;
            let row = at >> 16;
            let col = at & 0xffff;
            if row < shape.0 && col < shape.1 {
                return Target::parse(&format!("{name}[{row},{col}]")).ok();
            }
        }
        if let Ok(target) = Target::parse(&name)
            && target.cells.is_some()
            && scene.matrix_shape(pick.row).is_none()
            && scene
                .rows_named(&target.name)
                .into_iter()
                .any(|row| scene.matrix_shape(row).is_some())
        {
            return Some(target);
        }
        Some(Target { name, cells: None })
    }

    /// The pick and all targets linked to it.
    fn linked_to(&self, target: &Target) -> Vec<Target> {
        let mut out = Vec::new();
        for group in &self.spec.links {
            let parsed: Vec<Target> = group
                .iter()
                .filter_map(|name| Target::parse(name).ok())
                .collect();
            let hit = parsed.iter().any(|candidate| {
                candidate.name == target.name
                    && match (&candidate.cells, &target.cells) {
                        (None, _) | (_, None) => true,
                        (Some((a, _)), Some((b, _))) => a[0] == b[0],
                    }
            });
            if hit {
                out.extend(parsed);
            }
        }
        if out.is_empty() {
            out.push(target.clone());
        }
        out
    }
}

impl State {
    /// True while the ordinary panels and camera gestures should stay out of the way.
    pub fn presenting(&self) -> bool {
        self.presentation.is_some()
    }

    /// True when the current deck uses a black background and a light palette.
    pub fn presentation_dark(&self) -> bool {
        self.presentation
            .as_ref()
            .is_some_and(|deck| deck.spec.dark)
    }

    /// True while an effect needs another frame.
    pub fn presentation_animating(&self) -> bool {
        self.presentation
            .as_ref()
            .is_some_and(|deck| !deck.effects.is_empty())
    }

    /// Install a parsed deck and start loading its first step.
    pub fn begin_presentation(&mut self, spec: Presentation, base: String) {
        self.presentation = Some(PresentationState::new(spec, base));
        self.camera.set_view(crate::camera::View::Top);
        self.camera.perspective = false;
        self.view_frame = [0.0, 0.0, 1.0, 1.0];
        self.camera.frame = self.view_frame;
        self.gpu.view.show_grid = false;
        self.gpu.view.dark = self.presentation_dark();
        self.gpu.view.thickness_px = crate::engine::gpu::View::from_env().thickness_px;
        self.load_presentation(true);
    }

    /// Advance one step, completing an in-flight effect first.
    pub fn presentation_next(&mut self) {
        self.presentation_move(1);
    }

    /// Return one step, restoring the earlier topic when necessary.
    pub fn presentation_previous(&mut self) {
        self.presentation_move(-1);
    }

    /// Clear a pinned highlight and the current hover.
    pub fn presentation_escape(&mut self) {
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        deck.hover = None;
        deck.pinned = None;
        self.refresh_presentation_highlights();
        self.update_label();
        self.sync_presentation_dom();
        self.touch();
    }

    /// Move within a topic without clearing; cross a topic by clearing and reloading it.
    fn presentation_move(&mut self, delta: i32) {
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        let (index, topic) = deck.spec.navigate(deck.index, deck.topic, delta);
        if index == deck.index {
            return;
        }
        self.finish_presentation_effects();
        let reset = {
            let deck = self
                .presentation
                .as_mut()
                .expect("presentation remains installed");
            deck.hover = None;
            deck.pinned = None;
            deck.index = index;
            deck.topic = topic;
            topic != deck.loaded.first().map_or(topic, |step| step.topic)
        };
        self.load_presentation(reset);
    }

    fn clear_presentation_topic(&mut self, deck: &mut PresentationState) {
        self.clear();
        deck.loading = None;
        deck.loaded.clear();
        deck.initial.clear();
        deck.gradients.clear();
        deck.lit.clear();
        deck.spatial_rows.clear();
        deck.spatial_bounds = AABB::empty();
        deck.spatial_camera = crate::camera::Camera::new();
        deck.spatial_fit = false;
        deck.spatial_items.clear();
        deck.spatial_overlaps = 0;
        deck.spatial_reflow_pending = false;
        deck.clear_on_part = false;
        self.gpu.set_spatial_view(None);
        self.scene.linked.clear();
    }

    /// Load the current step and prefetch the remaining reveals in this topic.
    fn load_presentation(&mut self, reset: bool) {
        self.finish_presentation_effects();
        let Some(mut deck) = self.presentation.take() else {
            return;
        };
        if reset {
            deck.clear_on_part = true;
            deck.loading = None;
        }
        if !reset
            && !deck.clear_on_part
            && deck.loading.is_none()
            && deck
                .loaded
                .iter()
                .any(|step| step.index == deck.index && step.complete)
        {
            self.presentation = Some(deck);
            self.sync_presentation_steps();
            self.fit_presentation();
            self.touch();
            return;
        }
        if deck
            .loading
            .is_some_and(|(topic, last)| topic == deck.topic && last >= deck.index)
        {
            self.presentation = Some(deck);
            self.sync_presentation_dom();
            self.touch();
            return;
        }
        deck.generation = deck.generation.wrapping_add(1);
        let from = if reset || deck.loaded.iter().any(|step| !step.complete) {
            0
        } else {
            deck.spec.locate(deck.index).map_or(0, |(_, step)| step)
        };
        if from == 0 && !reset && !deck.loaded.is_empty() {
            self.clear_presentation_topic(&mut deck);
        }
        let local = deck.spec.locate(deck.index).map_or(0, |(_, step)| step);
        let through = deck
            .spec
            .topics
            .get(deck.topic)
            .map_or(local, |group| group.steps.len().saturating_sub(1));
        deck.loading = Some((deck.topic, deck.spec.topic_start(deck.topic) + through));
        #[cfg(target_arch = "wasm32")]
        crate::app::loader::spawn_presentation(
            deck.spec.clone(),
            deck.base.clone(),
            deck.topic,
            from,
            through,
            deck.generation,
        );
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (from, through);
        self.presentation = Some(deck);
        self.fit_presentation();
        self.sync_presentation_dom();
        self.touch();
    }

    /// Receive one file or the text of a step. Stale topic loads are ignored.
    pub fn presentation_part(
        &mut self,
        generation: u64,
        topic: usize,
        index: usize,
        doc: Option<FileDoc>,
        texts: Vec<crate::app::manifest::TextItem>,
        last: bool,
    ) {
        if self
            .presentation
            .as_ref()
            .is_none_or(|deck| deck.generation != generation)
        {
            return;
        }
        if self
            .presentation
            .as_ref()
            .is_some_and(|deck| deck.clear_on_part && deck.topic == topic)
        {
            let mut deck = self
                .presentation
                .take()
                .expect("presentation remains installed");
            self.clear_presentation_topic(&mut deck);
            self.presentation = Some(deck);
        }
        let first_text = self.scene.texts.len();
        let first_row = self.scene.row_count();
        if let Some(doc) = doc {
            crate::app::fonts::need_names(&doc.name, &doc.session);
            self.scene.add_file(doc);
            for row in &mut self.scene.tables.obj.rows {
                row.flags |= Instance::FLAG_MATH;
            }
            self.scene.upload_to(&mut self.gpu);
            let head_scale = self
                .presentation
                .as_ref()
                .map_or(1.0, |deck| deck.spec.head_scale / deck.spec.stroke_scale)
                as f32;
            for row in first_row..self.scene.row_count() {
                if self.scene.matrix_shape(row as u32).is_none() {
                    self.gpu.set_head_scale(row as u32, head_scale);
                }
            }
            let spatial = self
                .presentation
                .as_ref()
                .map(|deck| deck.spec.spatial.clone())
                .unwrap_or_default();
            let non_pickable = self
                .presentation
                .as_ref()
                .map(|deck| deck.spec.non_pickable.clone())
                .unwrap_or_default();
            for row in first_row..self.scene.row_count() {
                let row = row as u32;
                if self
                    .scene
                    .row_link_name(row)
                    .is_some_and(|name| spatial.iter().any(|other| other == name))
                {
                    if self
                        .scene
                        .row_link_name(row)
                        .is_some_and(|name| non_pickable.iter().any(|other| other == name))
                    {
                        self.gpu.set_non_pickable(row, true);
                    }
                    self.gpu.set_spatial(row, true);
                    let deck = self
                        .presentation
                        .as_mut()
                        .expect("presentation remains installed");
                    deck.spatial_rows.insert(row);
                    if !self
                        .scene
                        .row_link_name(row)
                        .is_some_and(|name| name.starts_with("grid/"))
                        && let Some(bounds) = self.gpu.objects.row_bounds(row)
                    {
                        deck.spatial_bounds.union_with(&bounds);
                    }
                }
            }
            let colored = self
                .presentation
                .as_ref()
                .map(|deck| deck.spec.colored.clone())
                .unwrap_or_default();
            for (name, cells) in colored {
                let mut mask = [0u32; 2];
                for cell in cells {
                    if cell < 64 {
                        mask[(cell / 32) as usize] |= 1 << (cell % 32);
                    }
                }
                for row in self.scene.rows_named(&name) {
                    if self.scene.matrix_shape(row).is_some() {
                        self.gpu.set_colored_cells(row, mask);
                    }
                }
            }
        }
        if !texts.is_empty() {
            let mut texts = texts;
            if let Some(deck) = self.presentation.as_ref() {
                let right = deck
                    .spatial_camera
                    .orientation
                    .rotate_vector(session_rust::Vector::x_axis());
                let up = deck.spatial_camera.up;
                for item in texts
                    .iter_mut()
                    .filter(|item| item.spatial && item.camera_facing)
                {
                    item.right = [right[0], right[1], right[2]];
                    item.up = up;
                }
            }
            if let Some(deck) = self.presentation.as_mut() {
                let index = deck.spec.topic_start(topic) + index;
                let morph_targets: HashSet<&str> = deck
                    .spec
                    .step(index)
                    .into_iter()
                    .flat_map(|step| &step.effects)
                    .filter(|effect| effect.effect == EffectKind::Morph)
                    .map(|effect| effect.name.as_str())
                    .collect();
                let visible = |item: &crate::app::manifest::TextItem| {
                    !item.spatial && !morph_targets.contains(item.name.as_str())
                };
                let mut flat: Vec<_> = texts.iter().filter(|item| visible(item)).cloned().collect();
                for item in &mut flat {
                    if item.name.contains('[') {
                        item.fixed = true;
                    }
                }
                deck.overlaps =
                    crate::app::math::optimize_layout(&mut flat, deck.spec.width, deck.spec.height);
                let mut flat = flat.into_iter();
                for item in texts.iter_mut().filter(|item| visible(item)) {
                    if let Some(positioned) = flat.next() {
                        *item = positioned;
                    }
                }
            }
            let expanded = crate::app::math::typeset(&texts);
            let namespace = format!("presentation-text/{topic}/{index}");
            let deck = self
                .presentation
                .as_mut()
                .expect("presentation remains installed");
            deck.spatial_items.retain(|(key, _)| key != &namespace);
            deck.spatial_items.extend(
                texts
                    .iter()
                    .filter(|item| item.spatial)
                    .map(|item| (namespace.clone(), item.clone())),
            );
            for doc in expanded.documents {
                self.scene.add_file(doc);
                self.scene.upload_to(&mut self.gpu);
            }
            if !expanded.texts.is_empty() {
                self.scene.append_texts(
                    expanded.texts,
                    &format!("presentation-text/{topic}/{index}"),
                    &mut self.gpu,
                );
            }
        }
        let rows: Vec<u32> = (first_row..self.scene.row_count())
            .map(|row| row as u32)
            .filter(|row| !self.scene.is_text(*row))
            .collect();
        let texts: Vec<u32> = self.scene.texts[first_text..]
            .iter()
            .map(|text| text.row)
            .collect();
        let mut deck = self
            .presentation
            .take()
            .expect("presentation remains installed");
        let flat = deck.spec.topic_start(topic) + index;
        if let Some(step) = deck.loaded.iter_mut().find(|step| step.index == flat) {
            step.rows.extend(rows);
            step.texts.extend(texts);
            step.complete |= last;
        } else {
            deck.loaded.push(LoadedStep {
                topic,
                index: flat,
                rows,
                texts,
                complete: last,
                played: false,
            });
        }
        deck.error = None;
        if last && deck.loading == Some((topic, flat)) {
            deck.loading = None;
        }
        if last && !deck.spatial_fit && deck.spatial_bounds.is_valid() {
            let aspect = self.aspect();
            let bounds = deck.spatial_bounds;
            if bounds.max_point()[2] - bounds.min_point()[2] < 1.0e-9 {
                deck.spatial_camera.set_view(crate::camera::View::Top);
            }
            deck.spatial_camera
                .fit_in(&bounds, aspect, [0.0, 0.0, deck.spec.spatial_split, 1.0]);
            deck.spatial_camera.perspective = false;
            deck.spatial_fit = true;
        }
        for row in first_row..self.scene.row_count() {
            let row = row as u32;
            let at = if let Some(text) = self.scene.texts.iter().find(|text| text.row == row) {
                label_position(&text.label)
            } else if let Some(bounds) = self.gpu.objects.row_bounds(row) {
                let center = bounds.center();
                [center[0], center[1], center[2]]
            } else {
                continue;
            };
            let place = self.gpu.objects.placement(row).unwrap_or_default();
            deck.initial.entry(row).or_insert((place, at));
        }
        self.presentation = Some(deck);
        self.fit_presentation();
        self.reflow_spatial_labels();
        self.sync_presentation_steps();
        self.touch();
    }

    /// Show a recoverable fetch/decode error in the caption, keeping the current picture.
    pub fn presentation_error(&mut self, generation: u64, message: String) {
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        if deck.generation != generation {
            return;
        }
        deck.loading = None;
        deck.error = Some(message);
        self.sync_presentation_dom();
    }

    fn slide_endpoint(
        &self,
        deck: &PresentationState,
        row: u32,
        local: session_rust::Point,
    ) -> Option<[f64; 3]> {
        let place = self.gpu.objects.placement(row)?;
        let world = place.transform_point(&local);
        let spatial = self.gpu.objects.row(row)?.flags & Instance::FLAG_SPATIAL != 0;
        if !spatial {
            return Some([world[0], world[1], world[2]]);
        }
        let anchor = self.camera.origin();
        let projection = deck
            .spatial_camera
            .view_proj_anchored(self.aspect(), &anchor);
        let at = crate::app::math::screen_projection(
            &projection,
            &anchor,
            [world[0], world[1], world[2]],
            [
                f64::from(self.gpu.config.width),
                f64::from(self.gpu.config.height),
            ],
        )?
        .0;
        let (origin, direction) = self.camera.ray(
            at.into(),
            (
                f64::from(self.gpu.config.width),
                f64::from(self.gpu.config.height),
            ),
        )?;
        if direction[2].abs() < 1.0e-12 {
            return None;
        }
        let t = -origin[2] / direction[2];
        Some(std::array::from_fn(|axis| {
            origin[axis] + direction[axis] * t
        }))
    }

    fn geometry_morph(
        &self,
        deck: &PresentationState,
        source: u32,
        target: u32,
    ) -> Option<GeometryMorph> {
        let (session_rust::Geometry::Line(source_line), session_rust::Geometry::Line(target_line)) =
            (self.scene.geometry(source)?, self.scene.geometry(target)?)
        else {
            return None;
        };
        let from = [
            self.slide_endpoint(deck, source, source_line.start())?,
            self.slide_endpoint(deck, source, source_line.end())?,
        ];
        let local =
            [target_line.start(), target_line.end()].map(|point| [point[0], point[1], point[2]]);
        let place = self.gpu.objects.placement(target)?;
        let to = local.map(|point| {
            let point =
                place.transform_point(&session_rust::Point::new(point[0], point[1], point[2]));
            [point[0], point[1], point[2]]
        });
        Some(GeometryMorph {
            source,
            target,
            from,
            to,
            local,
            place,
        })
    }

    /// Apply cumulative visibility, gradients and newly completed effects.
    fn sync_presentation_steps(&mut self) {
        let Some(mut deck) = self.presentation.take() else {
            return;
        };
        let mut play = false;
        for step in &mut deck.loaded {
            let preserve = deck
                .spec
                .topics
                .get(step.topic)
                .is_some_and(|group| group.preserve_spatial);
            let visible = step.topic == deck.topic
                && step.index <= deck.index
                && (!preserve || step.index == deck.index);
            let persistent = preserve
                && deck
                    .spec
                    .locate(step.index)
                    .is_some_and(|(_, local)| local == 0);
            for &row in &step.rows {
                let spatial = self
                    .gpu
                    .objects
                    .row(row)
                    .is_some_and(|instance| instance.flags & Instance::FLAG_SPATIAL != 0);
                let retained = visible || persistent && spatial;
                let authored_hidden = self
                    .scene
                    .identity_of(row)
                    .is_some_and(|id| self.scene.hidden.contains(&id));
                self.gpu.set_hidden(row, !retained || authored_hidden);
            }
            for &row in &step.texts {
                let mut shown = visible;
                if let Some(text) = self.scene.texts.iter_mut().find(|text| text.row == row) {
                    if persistent && text.label.id & crate::engine::text::SPATIAL_TEXT_ID != 0 {
                        shown = true;
                    }
                    if text.label.id & crate::engine::text::SPATIAL_TEXT_ID != 0 {
                        shown &= deck.spatial_fit;
                    }
                    text.active = shown;
                }
                self.gpu.set_hidden(row, !shown);
            }
            if visible && step.complete && step.index == deck.index && !step.played {
                step.played = true;
                play = true;
            } else if step.index != deck.index {
                step.played = false;
            }
        }
        for (name, gradient) in &deck.spec.gradients {
            for target in PresentationState::resolve(&self.scene, &Target::parse(name).unwrap()) {
                if deck.gradients.insert(target.row) {
                    self.gpu
                        .set_gradient(target.row, gradient.from, gradient.to);
                }
            }
        }
        if play {
            let effects = deck
                .spec
                .step(deck.index)
                .map(|step| step.effects.clone())
                .unwrap_or_default();
            for effect in effects {
                let named = |name: &str| {
                    Target::parse(name).unwrap_or_else(|_| Target::parse("missing").unwrap())
                };
                let mut targets = PresentationState::resolve(&self.scene, &named(&effect.name));
                if deck
                    .spec
                    .topics
                    .get(deck.topic)
                    .is_some_and(|group| group.preserve_spatial)
                {
                    let current_rows: HashSet<u32> = deck
                        .loaded
                        .iter()
                        .find(|step| step.index == deck.index)
                        .map(|step| step.rows.iter().chain(step.texts.iter()).copied().collect())
                        .unwrap_or_default();
                    targets.retain(|target| {
                        current_rows.contains(&target.row)
                            || self.gpu.objects.row(target.row).is_some_and(|instance| {
                                instance.flags & Instance::FLAG_SPATIAL != 0
                            })
                    });
                }
                if matches!(effect.effect, EffectKind::Move | EffectKind::Scale) {
                    for target in &targets {
                        let Some((place, at)) = deck.initial.get(&target.row).cloned() else {
                            continue;
                        };
                        self.gpu.set_placement(target.row, &place);
                        if let Some(text) = self
                            .scene
                            .texts
                            .iter_mut()
                            .find(|text| text.row == target.row)
                        {
                            move_label(&mut text.label, at);
                        }
                        if let Some((_, item)) = deck
                            .spatial_items
                            .iter_mut()
                            .find(|(_, item)| item.name == effect.name)
                        {
                            item.at = at;
                        }
                    }
                }
                let source = effect.from.as_ref().map_or_else(Vec::new, |name| {
                    PresentationState::resolve(&self.scene, &named(name))
                });
                let morph_start = if effect.effect == EffectKind::Morph {
                    PresentationState::ordered_labels(&self.scene, &source)
                } else {
                    Vec::new()
                };
                let start = if effect.effect == EffectKind::Move {
                    targets
                        .iter()
                        .filter_map(|target| {
                            self.presentation_anchor(target.row)
                                .map(|at| (target.row, at))
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                };
                let spatial_at = deck
                    .spatial_items
                    .iter()
                    .find(|(_, item)| item.name == effect.name)
                    .map(|(_, item)| item.at);
                let group_at = spatial_at.unwrap_or_else(|| {
                    let mut at = [0.0; 3];
                    for (_, position) in &start {
                        for axis in 0..3 {
                            at[axis] += position[axis] / start.len().max(1) as f64;
                        }
                    }
                    at
                });
                let delta = effect.to.map_or([0.0; 3], |to| {
                    std::array::from_fn(|axis| to[axis] - group_at[axis])
                });
                let placements = if effect.effect == EffectKind::Scale {
                    targets
                        .iter()
                        .filter_map(|target| {
                            self.gpu
                                .objects
                                .placement(target.row)
                                .map(|place| (target.row, place))
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                let geometry_morph = if effect.effect == EffectKind::Morph {
                    source
                        .iter()
                        .zip(&targets)
                        .filter_map(|(source, target)| {
                            self.geometry_morph(&deck, source.row, target.row)
                        })
                        .collect()
                } else {
                    Vec::new()
                };
                for target in &targets {
                    match effect.effect {
                        EffectKind::Grow => self.set_presentation_fade(target.row, 0.0),
                        EffectKind::Morph => {
                            self.gpu.set_hidden(target.row, true);
                            self.scene.set_text_scale(target.row, 0.0);
                        }
                        EffectKind::Pulse
                        | EffectKind::Highlight
                        | EffectKind::Cells
                        | EffectKind::Move
                        | EffectKind::Scale => self.set_presentation_reveal(target.row, 1.0, 1.0),
                    }
                }
                deck.effects.push(ActiveEffect {
                    effect,
                    targets,
                    source,
                    start,
                    morph_start,
                    geometry_morph,
                    placements,
                    delta,
                    started_ms: None,
                });
            }
        }
        self.presentation = Some(deck);
        self.refresh_presentation_highlights();
        self.update_label();
        self.sync_presentation_dom();
    }

    /// Refit the slide and its 3D region after the viewport changes.
    pub(crate) fn refit_presentation(&mut self) {
        let aspect = self.aspect();
        if let Some(deck) = self.presentation.as_mut()
            && deck.spatial_fit
        {
            let bounds = deck.spatial_bounds;
            if bounds.max_point()[2] - bounds.min_point()[2] < 1.0e-9 {
                deck.spatial_camera.set_view(crate::camera::View::Top);
            }
            deck.spatial_camera
                .fit_in(&bounds, aspect, [0.0, 0.0, deck.spec.spatial_split, 1.0]);
            deck.spatial_camera.perspective = false;
        }
        self.fit_presentation();
        self.reflow_spatial_labels();
    }

    /// Fit the fixed slide plane, not the subset loaded so far.
    fn fit_presentation(&mut self) {
        let Some(deck) = self.presentation.as_ref() else {
            return;
        };
        let mut bounds = AABB::empty();
        bounds.union_with(&AABB::from_points(
            &[
                Point::new(0.0, -90.0, 0.0),
                Point::new(deck.spec.width, deck.spec.height, 0.0),
            ],
            0.0,
        ));
        self.camera
            .fit_in(&bounds, self.aspect(), [0.0, 0.0, 1.0, 1.0]);
        self.camera.grow_extent(&bounds);
        self.sync_spatial_view();
    }

    fn sync_spatial_view(&mut self) {
        let Some(deck) = self.presentation.as_ref() else {
            self.gpu.set_spatial_view(None);
            return;
        };
        let view = if deck.spatial_fit && !deck.spatial_rows.is_empty() {
            let anchor = self.camera.origin();
            Some(crate::engine::gpu::SpatialView {
                view_proj: deck
                    .spatial_camera
                    .view_proj_anchored(self.aspect(), &anchor),
                split_x: self.gpu.config.width as f32 * deck.spec.spatial_split as f32,
            })
        } else {
            None
        };
        self.gpu.set_spatial_view(view);
        if let Some(deck) = self.presentation.as_mut() {
            deck.spatial_reflow_pending = true;
        }
    }

    fn reflow_spatial_labels(&mut self) {
        let aspect = self.aspect();
        let anchor = self.camera.origin();
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        if !deck.spatial_fit || deck.spatial_items.is_empty() {
            deck.spatial_reflow_pending = false;
            return;
        }
        let namespace: Vec<_> = deck
            .spatial_items
            .iter()
            .filter(|(namespace, item)| {
                let visible = deck.loaded.iter().any(|step| {
                    step.topic == deck.topic
                        && step.index <= deck.index
                        && deck.spec.locate(step.index).is_some_and(|(topic, index)| {
                            namespace == &format!("presentation-text/{topic}/{index}")
                        })
                });
                visible
                    && self.scene.texts.iter().any(|text| {
                        text.name == item.name
                            && text.key.starts_with(&format!("{namespace}/"))
                            && self
                                .scene
                                .text_scales
                                .get(&text.row)
                                .copied()
                                .unwrap_or(1.0)
                                > 0.0001
                    })
            })
            .cloned()
            .collect();
        if namespace.is_empty() {
            deck.spatial_overlaps = 0;
            deck.spatial_reflow_pending = false;
            return;
        }
        let mut items: Vec<_> = namespace.iter().map(|(_, item)| item.clone()).collect();
        let width = f64::from(self.gpu.config.width);
        let height = f64::from(self.gpu.config.height);
        let pixel_scale = width / self.gpu.logical_size[0].max(1.0);
        let projection = deck.spatial_camera.view_proj_anchored(aspect, &anchor);
        let mut obstacles = Vec::new();
        let mut rows: Vec<_> = deck.spatial_rows.iter().copied().collect();
        rows.sort_unstable();
        for row in rows {
            if self
                .scene
                .row_link_name(row)
                .is_some_and(|name| name.starts_with("grid/"))
                || self.gpu.objects.row(row).is_none_or(|instance| {
                    instance.flags & crate::engine::gpu::Instance::FLAG_HIDDEN != 0
                })
            {
                continue;
            }
            let Some(place) = self.gpu.objects.placement(row) else {
                continue;
            };
            let project = |point: session_rust::Point| {
                let point = place.transform_point(&point);
                crate::app::math::screen_projection(
                    &projection,
                    &anchor,
                    [point[0], point[1], point[2]],
                    [width, height],
                )
                .map(|(at, _)| at)
            };
            match self.scene.geometry(row) {
                Some(session_rust::Geometry::Point(point)) => {
                    if let Some(at) = project((**point).clone()) {
                        obstacles.push(crate::app::math::SpatialObstacle::Disc(
                            at,
                            (f64::from(point.width) * 0.5 + 8.0) * pixel_scale,
                        ));
                    }
                }
                Some(session_rust::Geometry::Line(line)) => {
                    if let (Some(a), Some(b)) = (project(line.start()), project(line.end())) {
                        obstacles.push(crate::app::math::SpatialObstacle::Segment(
                            a,
                            b,
                            (f64::from(line.width) * 0.5 + 8.0) * pixel_scale,
                        ));
                    }
                }
                Some(session_rust::Geometry::Polyline(line)) => {
                    let points = line.get_points();
                    for pair in points.windows(2) {
                        if let (Some(a), Some(b)) =
                            (project(pair[0].clone()), project(pair[1].clone()))
                        {
                            obstacles.push(crate::app::math::SpatialObstacle::Segment(
                                a,
                                b,
                                (f64::from(line.width) * 0.5 + 8.0) * pixel_scale,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
        deck.spatial_overlaps = crate::app::math::spatial_layout(
            &mut items,
            &obstacles,
            &deck.spatial_camera,
            aspect,
            deck.spec.spatial_split,
            anchor,
            width,
            height,
            pixel_scale,
        );
        let right = deck
            .spatial_camera
            .orientation
            .rotate_vector(session_rust::Vector::x_axis());
        let up = deck.spatial_camera.up;
        for ((namespace, original), mut item) in namespace.iter().zip(items) {
            if original.camera_facing {
                item.right = [right[0], right[1], right[2]];
                item.up = up;
            }
            let after = crate::app::math::typeset(&[item]);
            let mut group: Vec<_> = self
                .scene
                .texts
                .iter_mut()
                .filter(|text| {
                    text.key.starts_with(&format!("{namespace}/")) && text.name == original.name
                })
                .collect();
            group.sort_by_key(|text| {
                text.key
                    .rsplit('/')
                    .next()
                    .map_or(0, |index| index.parse::<usize>().unwrap_or(0))
            });
            for (index, text) in group.into_iter().enumerate() {
                let Some(run) = after.texts.get(index) else {
                    continue;
                };
                match &mut text.label.placement {
                    crate::engine::text::TextPlacement::WorldBillboard {
                        world,
                        world_height,
                    }
                    | crate::engine::text::TextPlacement::WorldPlane {
                        world,
                        world_height,
                        ..
                    } => {
                        *world = run.at;
                        *world_height = run.height;
                    }
                    crate::engine::text::TextPlacement::Anchor { world, offset } => {
                        *world = run.at;
                        *offset = run.offset;
                    }
                    _ => {}
                }
                if let Some(color) = run.color {
                    text.label.color = color;
                }
            }
        }
        self.update_label();
        if let Some(deck) = self.presentation.as_mut() {
            deck.spatial_reflow_pending = false;
        }
    }

    pub fn orbit_presentation_spatial(&mut self, dx: f32, dy: f32) -> bool {
        let Some(deck) = self.presentation.as_mut() else {
            return false;
        };
        if !deck.spatial_fit {
            return false;
        }
        deck.spatial_camera.orbit(dx, dy);
        self.sync_spatial_view();
        self.sync_presentation_dom();
        self.touch();
        true
    }

    pub fn zoom_presentation_spatial(
        &mut self,
        amount: f32,
        cursor: (f64, f64),
        viewport: (f64, f64),
    ) -> bool {
        let Some(deck) = self.presentation.as_mut() else {
            return false;
        };
        if !deck.spatial_fit {
            return false;
        }
        if cursor.0 > deck.spec.spatial_split * viewport.0 {
            return true;
        }
        deck.spatial_camera.zoom_at(amount, cursor, viewport);
        self.sync_spatial_view();
        self.sync_presentation_dom();
        self.touch();
        true
    }

    pub fn pan_presentation_spatial(&mut self, dx: f32, dy: f32) -> bool {
        let Some(deck) = self.presentation.as_mut() else {
            return false;
        };
        if !deck.spatial_fit {
            return false;
        }
        deck.spatial_camera.pan(dx, dy);
        self.sync_spatial_view();
        self.sync_presentation_dom();
        self.touch();
        true
    }

    /// The actual geometry center, not the often-identity document placement.
    fn presentation_anchor(&self, row: u32) -> Option<[f64; 3]> {
        if let Some(text) = self.scene.texts.iter().find(|text| text.row == row) {
            return Some(label_position(&text.label));
        }
        self.gpu.objects.row_bounds(row).map(|bounds| {
            let center = bounds.center();
            [center[0], center[1], center[2]]
        })
    }

    fn move_geometry_to(&mut self, row: u32, at: [f64; 3]) {
        let Some(anchor) = self.presentation_anchor(row) else {
            return;
        };
        let Some(place) = self.gpu.objects.placement(row) else {
            return;
        };
        self.gpu.set_translation(
            row,
            std::array::from_fn(|axis| place.m[12 + axis] + at[axis] - anchor[axis]),
        );
    }

    /// Set both the GPU row effect and the equivalent text reveal.
    fn set_presentation_reveal(&mut self, row: u32, trim: f32, scale: f32) {
        self.gpu.set_reveal(row, trim, scale);
        self.scene.set_text_scale(row, trim * scale);
    }

    /// Reveal geometry and fade text without changing its cached raster size.
    fn set_presentation_fade(&mut self, row: u32, reveal: f32) {
        self.gpu.set_reveal(row, reveal, 1.0);
        self.scene.set_text_opacity(row, reveal);
    }

    /// One absolute animation sample. At one, settle both the target and the morph source.
    fn play_presentation_effect(
        &mut self,
        deck: &mut PresentationState,
        active: &ActiveEffect,
        t: f64,
        transient: &mut Vec<Target>,
    ) -> bool {
        if t >= 1.0 {
            for &(row, start) in &active.start {
                let at = std::array::from_fn(|axis| start[axis] + active.delta[axis]);
                if self.scene.is_text(row) {
                    if let Some(text) = self.scene.texts.iter_mut().find(|text| text.row == row) {
                        move_label(&mut text.label, at);
                    }
                } else {
                    self.move_geometry_to(row, at);
                }
            }
            if let (Some(factor), Some(about)) = (active.effect.factor, active.effect.about) {
                for (row, place) in &active.placements {
                    self.gpu.set_placement(*row, place);
                    self.gpu.scale_about(*row, about, factor);
                }
            }
            for target in &active.targets {
                self.set_presentation_reveal(target.row, 1.0, 1.0);
                self.scene.set_text_opacity(target.row, 1.0);
                self.gpu.set_hidden(target.row, false);
                if let Some(text) = self
                    .scene
                    .texts
                    .iter_mut()
                    .find(|text| text.row == target.row)
                {
                    text.active = true;
                }
            }
            if active.effect.effect == EffectKind::Morph {
                for morph in &active.geometry_morph {
                    self.gpu.set_placement(morph.target, &morph.place);
                }
                for source in &active.source {
                    self.gpu
                        .set_hidden(source.row, !deck.spatial_rows.contains(&source.row));
                    self.scene.set_text_scale(source.row, 0.0);
                    if let Some(text) = self
                        .scene
                        .texts
                        .iter_mut()
                        .find(|text| text.row == source.row)
                    {
                        text.active = false;
                    }
                }
            }
            if active.effect.effect == EffectKind::Move
                && let Some(to) = active.effect.to
                && let Some((_, item)) = deck
                    .spatial_items
                    .iter_mut()
                    .find(|(_, item)| item.name == active.effect.name)
            {
                item.at = to;
            }
        } else {
            match active.effect.effect {
                EffectKind::Grow => {
                    let reveal = 1.0 - (1.0 - t).powi(3);
                    for target in &active.targets {
                        self.set_presentation_fade(target.row, reveal as f32);
                    }
                }
                EffectKind::Pulse => {
                    let scale = 1.0 + 0.35 * (std::f64::consts::PI * t).sin();
                    for target in &active.targets {
                        self.set_presentation_reveal(target.row, 1.0, scale as f32);
                    }
                }
                EffectKind::Highlight => {
                    if let Ok(target) = Target::parse(&active.effect.name) {
                        transient.push(target);
                    }
                }
                EffectKind::Cells => {
                    for target in &active.targets {
                        let Some((rows, cols)) = self.scene.matrix_shape(target.row) else {
                            continue;
                        };
                        let count = rows.saturating_mul(cols);
                        if count == 0 {
                            continue;
                        }
                        let cell = ((t * f64::from(count)) as u32).min(count - 1);
                        transient.push(Target {
                            name: active.effect.name.clone(),
                            cells: Some(([cell / cols, cell % cols], [cell / cols, cell % cols])),
                        });
                    }
                }
                EffectKind::Move => {
                    for &(row, start) in &active.start {
                        let at = std::array::from_fn(|axis| start[axis] + active.delta[axis] * t);
                        if self.scene.is_text(row) {
                            if let Some(text) =
                                self.scene.texts.iter_mut().find(|text| text.row == row)
                            {
                                move_label(&mut text.label, at);
                            }
                            if let Some((_, item)) = deck
                                .spatial_items
                                .iter_mut()
                                .find(|(_, item)| item.name == active.effect.name)
                            {
                                item.at = at;
                            }
                        } else {
                            self.move_geometry_to(row, at);
                        }
                    }
                }
                EffectKind::Scale => {
                    if let (Some(factor), Some(about)) = (active.effect.factor, active.effect.about)
                    {
                        let factor = 1.0 + (factor - 1.0) * t;
                        for (row, place) in &active.placements {
                            self.gpu.set_placement(*row, place);
                            self.gpu.scale_about(*row, about, factor);
                        }
                    }
                }
                EffectKind::Morph => {
                    for morph in &active.geometry_morph {
                        self.gpu
                            .set_hidden(morph.source, !deck.spatial_rows.contains(&morph.source));
                        self.gpu.set_hidden(morph.target, false);
                        self.gpu.set_reveal(morph.target, 1.0, 1.0);
                        let from = [
                            morph.from[1][0] - morph.from[0][0],
                            morph.from[1][1] - morph.from[0][1],
                        ];
                        let to = [
                            morph.to[1][0] - morph.to[0][0],
                            morph.to[1][1] - morph.to[0][1],
                        ];
                        let from_length = from[0].hypot(from[1]);
                        let to_length = to[0].hypot(to[1]);
                        if from_length < 1.0e-12 || to_length < 1.0e-12 {
                            continue;
                        }
                        let move_t = smooth((t / 0.6).clamp(0.0, 1.0));
                        let settle = smooth(((t - 0.6) / 0.4).clamp(0.0, 1.0));
                        let length = from_length + (to_length - from_length) * settle;
                        let from_angle = from[1].atan2(from[0]);
                        let turn = to[1].atan2(to[0]) - from_angle;
                        let turn = turn.sin().atan2(turn.cos());
                        let direction = from_angle + turn * settle;
                        let center: [f64; 3] = std::array::from_fn(|axis| {
                            let a = (morph.from[0][axis] + morph.from[1][axis]) * 0.5;
                            let b = (morph.to[0][axis] + morph.to[1][axis]) * 0.5;
                            a + (b - a) * move_t
                        });
                        let offset = [
                            direction.cos() * length * 0.5,
                            direction.sin() * length * 0.5,
                        ];
                        let mut center = center;
                        let margin = 24.0;
                        let radius = [
                            (offset[0].abs() + margin).min(deck.spec.width * 0.5),
                            (offset[1].abs() + margin).min(deck.spec.height * 0.5),
                        ];
                        center[0] = center[0].clamp(radius[0], deck.spec.width - radius[0]);
                        center[1] = center[1].clamp(radius[1], deck.spec.height - radius[1]);
                        let at = [center[0] - offset[0], center[1] - offset[1], center[2]];
                        let end = [center[0] + offset[0], center[1] + offset[1], center[2]];
                        let current_start = morph.local[0];
                        let current_end = morph.local[1];
                        let current = [
                            current_end[0] - current_start[0],
                            current_end[1] - current_start[1],
                        ];
                        let desired = [end[0] - at[0], end[1] - at[1]];
                        let current_length = current[0].hypot(current[1]);
                        let desired_length = desired[0].hypot(desired[1]);
                        if current_length < 1.0e-12 || desired_length < 1.0e-12 {
                            continue;
                        }
                        let angle = desired[1].atan2(desired[0]) - current[1].atan2(current[0]);
                        let scale = desired_length / current_length;
                        let cos = angle.cos() * scale;
                        let sin = angle.sin() * scale;
                        let mut transform = morph.place.clone();
                        transform.m[0] = cos;
                        transform.m[1] = sin;
                        transform.m[4] = -sin;
                        transform.m[5] = cos;
                        transform.m[10] = scale;
                        transform.m[12] = at[0] - cos * current_start[0] + sin * current_start[1];
                        transform.m[13] = at[1] - sin * current_start[0] - cos * current_start[1];
                        self.gpu.set_placement(morph.target, &transform);
                    }
                    let target = PresentationState::ordered_labels(&self.scene, &active.targets);
                    let frame = morph_labels(&active.morph_start, &target, t);
                    for (row, label) in frame.labels {
                        self.scene.update_text_label(row, label);
                    }
                    for row in &active.source {
                        self.gpu.set_hidden(
                            row.row,
                            !self.scene.is_text(row.row) && !deck.spatial_rows.contains(&row.row),
                        );
                    }
                    for resolved in &active.targets {
                        if self.scene.is_text(resolved.row) {
                            let covered = target
                                .iter()
                                .position(|(row, _)| *row == resolved.row)
                                .is_some_and(|index| frame.covered[index]);
                            self.scene
                                .set_text_scale(resolved.row, if covered { 0.0 } else { t as f32 });
                            self.gpu.set_hidden(resolved.row, false);
                        } else {
                            self.gpu.set_hidden(resolved.row, false);
                            self.gpu.set_reveal(resolved.row, 1.0, 1.0);
                        }
                    }
                }
            }
        }
        !active.morph_start.is_empty()
            || active
                .targets
                .iter()
                .any(|target| self.scene.is_text(target.row))
    }

    /// Finish every active effect at its resting state before a navigation.
    fn finish_presentation_effects(&mut self) {
        let Some(mut deck) = self.presentation.take() else {
            return;
        };
        let effects: Vec<_> = deck.effects.drain(..).collect();
        for active in &effects {
            self.play_presentation_effect(&mut deck, active, 1.0, &mut Vec::new());
        }
        deck.transient.clear();
        self.presentation = Some(deck);
        self.reflow_spatial_labels();
        self.refresh_presentation_highlights();
        self.update_label();
    }

    /// Advance the animation clock; redraw requests continue until the last effect ends.
    pub fn tick_presentation(&mut self) {
        let Some(mut deck) = self.presentation.take() else {
            return;
        };
        if deck.effects.is_empty() {
            let pending = deck.spatial_reflow_pending;
            self.presentation = Some(deck);
            if pending {
                self.reflow_spatial_labels();
                self.sync_presentation_dom();
            }
            return;
        }
        let pending = deck.spatial_reflow_pending;
        let spatial_motion = deck.effects.iter().any(|active| {
            matches!(
                active.effect.effect,
                EffectKind::Morph | EffectKind::Move | EffectKind::Scale
            ) && active
                .targets
                .iter()
                .chain(&active.source)
                .any(|target| deck.spatial_rows.contains(&target.row))
        });
        let highlight_animation = deck.effects.iter().any(|active| {
            matches!(
                active.effect.effect,
                EffectKind::Highlight | EffectKind::Cells
            )
        });
        let math_only = deck
            .loaded
            .iter()
            .filter(|step| step.topic == deck.topic && step.index <= deck.index)
            .all(|step| {
                step.complete
                    && step
                        .rows
                        .iter()
                        .all(|row| self.scene.matrix_shape(*row).is_some())
            });
        let now = crate::engine::performance::now_ms();
        let mut transient = Vec::new();
        let mut text_changed = false;
        let mut remaining = Vec::new();
        for mut active in std::mem::take(&mut deck.effects) {
            let started = if math_only || self.gpu.performance.geometry_shown() {
                *active.started_ms.get_or_insert(now)
            } else {
                now
            };
            let elapsed = (now - started - active.effect.delay_ms).max(0.0);
            let t = if active.effect.duration_ms <= 0.0 {
                1.0
            } else {
                (elapsed / active.effect.duration_ms).clamp(0.0, 1.0)
            };
            let eased = smooth(t);
            text_changed |=
                self.play_presentation_effect(&mut deck, &active, eased, &mut transient);
            if t < 1.0 {
                remaining.push(active);
            }
        }
        deck.effects = remaining;
        deck.transient = transient;
        let animating = !deck.effects.is_empty();
        self.presentation = Some(deck);
        if pending || spatial_motion {
            self.reflow_spatial_labels();
        }
        if highlight_animation {
            self.refresh_presentation_highlights();
        }
        if text_changed {
            self.update_label();
        }
        if pending || spatial_motion || highlight_animation || !animating {
            self.sync_presentation_dom();
        }
        self.touch();
    }

    /// Recompute hover, pinned, step and animated highlights as one set of row styles.
    fn refresh_presentation_highlights(&mut self) {
        let Some(mut deck) = self.presentation.take() else {
            return;
        };
        let mut targets = deck.transient.clone();
        for step in &deck.loaded {
            if step.topic == deck.topic && step.index <= deck.index {
                if let Some(spec) = deck.spec.step(step.index) {
                    targets.extend(
                        spec.highlight
                            .iter()
                            .filter_map(|name| Target::parse(name).ok()),
                    );
                }
            }
        }
        for pick in [deck.hover, deck.pinned].into_iter().flatten() {
            if let Some(target) = PresentationState::picked_target(&self.scene, pick) {
                targets.extend(deck.linked_to(&target));
            }
        }
        let mut wanted: HashMap<u32, CellSpans> = HashMap::new();
        for target in &targets {
            for resolved in PresentationState::resolve(&self.scene, target) {
                match (wanted.get(&resolved.row).copied(), resolved.cells) {
                    (Some(None), _) | (_, None) => {
                        wanted.insert(resolved.row, None);
                    }
                    (Some(Some((a, b))), Some((c, d))) => {
                        let spans = if b.is_none() && (a.1 + 1 == c || d + 1 == a.0) {
                            Some(((a.0.min(c), a.1.max(d)), None))
                        } else if b.is_none() {
                            Some((a, Some((c, d))))
                        } else {
                            Some(((a.0.min(c), a.1.max(d)), b))
                        };
                        wanted.insert(resolved.row, spans);
                    }
                    (None, cells) => {
                        wanted.insert(
                            resolved.row,
                            cells.map(|(first, last)| ((first, last), None)),
                        );
                    }
                }
            }
        }
        let previous = std::mem::take(&mut deck.lit);
        for (row, cells) in previous {
            if !wanted.contains_key(&row) {
                self.gpu.set_selected(row, false);
                self.gpu.set_cells(row, None);
                self.scene.linked.remove(&row);
            } else if wanted.get(&row) != Some(&cells) {
                self.gpu.set_selected(row, cells.is_none());
                self.gpu.set_cells(row, cells);
            }
        }
        for (row, cells) in wanted {
            deck.lit.insert(row, cells);
            self.gpu.set_selected(row, cells.is_none());
            self.gpu.set_cells(row, cells);
            self.scene.linked.insert(row);
        }
        self.presentation = Some(deck);
    }

    /// Presentation pointer input: hover highlights, click pins, no camera gestures.
    #[cfg(target_arch = "wasm32")]
    pub fn presentation_input(&mut self, event: &winit::event::WindowEvent) -> bool {
        use winit::event::{ElementState, MouseButton, WindowEvent};
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let scale = crate::engine::gpu::view::surface_per_physical();
                let at = (position.x * scale, position.y * scale);
                let changed = self.presentation.as_ref().is_some_and(|deck| {
                    (at.0 - deck.cursor.0).abs() + (at.1 - deck.cursor.1).abs() > 2.0
                });
                if let Some(deck) = self.presentation.as_mut() {
                    deck.cursor = at;
                }
                if changed && !self.gpu.pick.busy() {
                    self.request_presentation_pick(false);
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                self.request_presentation_pick(true);
            }
            _ => {}
        }
        false
    }

    /// Apply a TeX label's named target without waiting for a GPU pick.
    pub fn presentation_named(&mut self, name: Option<String>, pin: bool) {
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        deck.pick_pin = false;
        if let Some(name) = name {
            let pick = Target::parse(&name).ok().and_then(|target| {
                let rows = self.scene.rows_named(&target.name);
                let matrix = target.cells.and_then(|(first, _)| {
                    rows.iter().find_map(|row| {
                        let shape = self.scene.matrix_shape(*row)?;
                        target.packed_range(shape)?;
                        Some(Pick {
                            row: *row,
                            sub: crate::app::presentation::pack_cell(first) + 1,
                        })
                    })
                });
                matrix.or_else(|| rows.first().map(|row| Pick { row: *row, sub: 0 }))
            });
            deck.hover = pick;
            if pin {
                deck.pinned = match (deck.pinned, pick) {
                    (Some(old), Some(new)) if old == new => None,
                    _ => pick,
                };
            }
        } else {
            deck.hover = None;
        }
        self.refresh_presentation_highlights();
        self.update_label();
        self.sync_presentation_dom();
        self.touch();
    }

    /// Ask for the row under the last presentation cursor.
    #[cfg(target_arch = "wasm32")]
    fn request_presentation_pick(&mut self, pin: bool) {
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        let (x, y) = deck.cursor;
        deck.pick_pin |= pin;
        self.request_selection(x.max(0.0) as u32, y.max(0.0) as u32, false, false);
    }

    /// Apply a hover or pin answer without invoking ordinary object selection.
    fn apply_presentation_pick(&mut self, pick: Option<Pick>) {
        let Some(deck) = self.presentation.as_mut() else {
            return;
        };
        let pin = deck.pick_pin;
        deck.pick_pin = false;
        deck.hover = pick;
        if pin {
            deck.pinned = match (deck.pinned, pick) {
                (Some(old), Some(new)) if old == new => None,
                _ => pick,
            };
        }
        self.refresh_presentation_highlights();
        self.update_label();
        self.sync_presentation_dom();
        self.touch();
    }

    /// Update the accessible DOM controls and their browser-test state.
    #[cfg(target_arch = "wasm32")]
    fn sync_presentation_dom(&self) {
        if let Some(deck) = self.presentation.as_ref() {
            web::sync(deck, &self.scene);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn sync_presentation_dom(&self) {}

    /// Authored effect targets, actual GPU centers, and transformed line endpoints.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn presentation_effect_diagnostics(&self) -> serde_json::Value {
        let Some(deck) = self.presentation.as_ref() else {
            return serde_json::Value::Null;
        };
        let effects = deck.spec.step(deck.index).map(|step| {
            step.effects
                .iter()
                .map(|effect| {
                    let endpoints = |name: &str| {
                        self.scene
                            .rows_named(name)
                            .into_iter()
                            .filter_map(|row| {
                                let session_rust::Geometry::Line(line) =
                                    self.scene.geometry(row)?
                                else {
                                    return None;
                                };
                                let model = self.gpu.objects.anchored_model(row)?;
                                let place = session_rust::Xform::from_matrix(model.map(f64::from));
                                let at = |point| {
                                    let point = place.transform_point(&point);
                                    [point[0], point[1], point[2]]
                                };
                                Some([at(line.start()), at(line.end())])
                            })
                            .collect::<Vec<_>>()
                    };
                    let centers: Vec<[f64; 3]> = self
                        .scene
                        .rows_named(&effect.name)
                        .into_iter()
                        .filter_map(|row| self.gpu.objects.row_bounds(row))
                        .map(|bounds| {
                            let center = bounds.center();
                            [center[0], center[1], center[2]]
                        })
                        .collect();
                    serde_json::json!({
                        "kind": format!("{:?}", effect.effect),
                        "name": effect.name,
                        "source": effect.from,
                        "centers": centers,
                        "endpoints": endpoints(&effect.name),
                        "source_endpoints": effect.from.as_deref().map(&endpoints),
                    })
                })
                .collect::<Vec<_>>()
        });
        let spatial = serde_json::json!({
            "bounds": [deck.spatial_bounds.min_point(), deck.spatial_bounds.max_point()],
            "target": deck.spatial_camera.target,
            "distance": deck.spatial_camera.distance,
            "frame": deck.spatial_camera.frame,
            "rows": deck.spatial_rows.iter().filter_map(|row| {
                let b = self.gpu.objects.row_bounds(*row)?;
                Some((self.scene.row_link_name(*row)?, [b.cx, b.cy, b.cz, b.hx, b.hy, b.hz]))
            }).collect::<Vec<_>>(),
        });
        let loaded: Vec<_> = deck
            .loaded
            .iter()
            .map(|step| {
                serde_json::json!({"index": step.index, "complete": step.complete, "played": step.played})
            })
            .collect();
        let active: Vec<_> = deck
            .effects
            .iter()
            .map(|effect| {
                let morphs: Vec<_> = effect
                    .geometry_morph
                    .iter()
                    .map(|morph| {
                        let current = self.gpu.objects.placement(morph.target).map(|place| place.m);
                        serde_json::json!({"from": morph.from, "to": morph.to, "local": morph.local, "current": current})
                    })
                    .collect();
                serde_json::json!({"name": effect.effect.name, "started": effect.started_ms, "morphs": morphs})
            })
            .collect();
        serde_json::json!({"step": deck.index, "effects": effects, "spatial": spatial, "loaded": loaded, "active": active})
    }
}

impl State {
    /// Route a completed pick to the deck when one is active.
    pub(crate) fn apply_presentation_pick_if_active(&mut self, pick: Option<Pick>) -> bool {
        if !self.presenting() {
            return false;
        }
        self.apply_presentation_pick(pick);
        true
    }
}

#[cfg(target_arch = "wasm32")]
mod web {
    use super::PresentationState;

    /// Create the non-interactive title and caption once, then update their labels.
    pub fn sync(deck: &PresentationState, scene: &crate::app::scene::Scene) {
        let Some(document) = web_sys::window().and_then(|window| window.document()) else {
            return;
        };
        let Some(body) = document.body() else {
            return;
        };
        let (ink, paper, progress_color) = if deck.spec.dark {
            ("#eef2f7", "#101216f0", "#c4ccd6")
        } else {
            ("#1d2730", "#fffffff0", "#46515a")
        };
        if document.get_element_by_id("presentation-topic").is_none() {
            for (id, style) in [
                (
                    "presentation-topic",
                    "left:50%;top:14px;transform:translateX(-50%);text-align:center;font:600 26px/1.35 Georgia, 'Times New Roman', serif",
                ),
                (
                    "presentation-caption",
                    "left:50%;bottom:18px;transform:translateX(-50%);max-width:82%;text-align:center;font:18px/1.45 Georgia, 'Times New Roman', serif",
                ),
            ] {
                let Ok(element) = document.create_element("div") else {
                    continue;
                };
                element.set_id(id);
                element.set_attribute("style", &format!("position:fixed;{style};z-index:11;color:{ink};background:{paper};padding:5px 12px;border-radius:4px;pointer-events:none;white-space:pre-wrap")).ok();
                body.append_child(&element).ok();
            }
            if let Ok(progress) = document.create_element("span") {
                progress.set_id("presentation-progress");
                progress.set_attribute("style", &format!("position:fixed;right:18px;bottom:16px;z-index:11;color:{progress_color};font:13px system-ui;pointer-events:none")).ok();
                body.append_child(&progress).ok();
            }
        }
        let set = |id: &str, value: &str| {
            if let Some(element) = document.get_element_by_id(id) {
                element.set_text_content(Some(value));
            }
        };
        let topic = deck.spec.topics.get(deck.topic);
        set(
            "presentation-topic",
            topic.map_or("", |topic| topic.title.as_str()),
        );
        let caption = deck.error.clone().unwrap_or_else(|| {
            deck.spec
                .step(deck.index)
                .map_or_else(String::new, |step| step.caption.clone())
        });
        set("presentation-caption", &caption);
        set(
            "presentation-progress",
            &format!("{} / {}", deck.index + 1, deck.spec.step_count()),
        );
        if let Some(canvas) = document.get_element_by_id("canvas") {
            canvas
                .set_attribute("data-presentation-step", &deck.index.to_string())
                .ok();
            canvas
                .set_attribute("data-presentation-topic", &deck.topic.to_string())
                .ok();
            canvas
                .set_attribute(
                    "data-presentation-overlaps",
                    &(deck.overlaps + deck.spatial_overlaps).to_string(),
                )
                .ok();
            let ready = !deck.clear_on_part
                && deck
                    .loaded
                    .iter()
                    .any(|step| step.index == deck.index && step.complete);
            canvas
                .set_attribute(
                    "data-presentation-ready",
                    if ready { "true" } else { "false" },
                )
                .ok();
            canvas
                .set_attribute(
                    "data-presentation-animating",
                    if deck.effects.is_empty() {
                        "false"
                    } else {
                        "true"
                    },
                )
                .ok();
            let mut linked: Vec<String> = deck
                .lit
                .keys()
                .filter_map(|row| scene.row_link_name(*row).map(str::to_string))
                .collect();
            linked.sort();
            canvas
                .set_attribute("data-presentation-linked", &linked.join(","))
                .ok();
            let effects: Vec<String> = deck
                .effects
                .iter()
                .map(|active| {
                    format!(
                        "{:?}:{}:{}",
                        active.effect.effect,
                        active.effect.from.as_deref().unwrap_or(""),
                        active.effect.name
                    )
                })
                .collect();
            canvas
                .set_attribute("data-presentation-effects", &effects.join(","))
                .ok();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn glyph(id: u32, text: &str, x: f64, color: [u8; 4]) -> (u32, TextLabel) {
        (
            id,
            TextLabel {
                id: id + 1 | crate::engine::text::MATH_SERIF_ID,
                object: None,
                text: text.into(),
                font_size: 18.0,
                line_height: 26.0,
                color,
                outline: 0.0,
                placement: TextPlacement::WorldPlane {
                    world: [x, 0.0, 0.0],
                    right: [1.0, 0.0, 0.0],
                    up: [0.0, 1.0, 0.0],
                    world_height: 28.0,
                    plate: false,
                },
                clip: None,
            },
        )
    }

    #[test]
    fn morph_matches_repeated_glyphs_and_preserves_their_ids() {
        let source = vec![
            glyph(10, "x", 0.0, [255, 0, 0, 255]),
            glyph(11, "0", 10.0, [255, 0, 0, 255]),
            glyph(12, "x", 20.0, [255, 0, 0, 255]),
            glyph(13, "+", 30.0, [255, 0, 0, 255]),
        ];
        let target = vec![
            glyph(20, "x", 0.0, [0, 255, 0, 255]),
            glyph(21, "1", 10.0, [0, 255, 0, 255]),
            glyph(22, "x", 40.0, [0, 255, 0, 255]),
            glyph(23, "=", 50.0, [0, 255, 0, 255]),
        ];
        let frame = morph_labels(&source, &target, 0.5);
        assert_eq!(frame.labels[0].0, 10);
        assert_eq!(frame.labels[1].1.text, "0");
        assert_eq!(label_position(&frame.labels[2].1), [30.0, 0.0, 0.0]);
        assert_eq!(frame.labels[2].1.color, [128, 128, 0, 255]);
        assert_ne!(frame.labels[2].1.id & crate::engine::text::MATH_SERIF_ID, 0);
        assert_eq!(label_height(&frame.labels[2].1), 28.0);
        assert_eq!(frame.labels[3].1.text, "+");
        assert_eq!(label_height(&frame.labels[3].1), 28.0);
        assert_eq!(frame.covered, [true, false, true, false]);
        assert_eq!(smooth(0.5), 0.5);
        assert!(smooth(0.2) < 0.2);
        assert!(smooth(0.8) > 0.8);
    }
}
