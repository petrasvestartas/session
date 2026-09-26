use super::rows::{FREE, Footprint, GEOMETRY, SINK};
use super::sync::{Work, baked};
use super::{Scene, placement};
use crate::app::walk::{Row, Walk, WalkCx, is_drawable, walk_features, walk_geometry};
use crate::engine::gpu::glyphs::GlyphPoint;
use crate::engine::gpu::hull::{Hull, hull_of};
use crate::engine::gpu::instanced::Definition;
use crate::engine::gpu::objects::world_box;
use crate::engine::gpu::patch::Counts;
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
use session_rust::{AABB, Geometry, InstanceRef, Session, Xform};
use std::collections::HashMap;
use std::rc::{Rc, Weak};

/// What each instance row takes from its definition's one walk.
#[derive(Clone)]
struct Walked {
    bounds: AABB,             // box in the definition frame
    spacing: f32,             // vertex spacing
    flags: u32,               // row flag bits of the walk
    faces: bool,              // the walk drew faces
    spheres: Vec<GlyphPoint>, // markers, drawn per instance: spheres index themselves by instance
    dots: Vec<GlyphPoint>,    // dots, drawn per instance with the markers
    hull: Option<Hull>,       // extreme points of the walk, for exact turned boxes
}

/// One definition of one document: walked once under a hidden row, drawn once per instance.
struct Batch {
    key: u32,            // names it on the GPU for as long as it lives
    doc: usize,          // its document
    definition: Rc<str>, // the definition's guid
    row: u32,            // the hidden row its shared rows are walked under
    walked: Walked,      // what its instances take
    members: Vec<u32>,   // the instance rows it draws
}

/// The definitions drawn once and the instance rows that place them.
#[derive(Default)]
pub(crate) struct Instancing {
    batches: Vec<Option<Batch>>, // a slot per batch; None once dropped
    by_definition: HashMap<(usize, Rc<str>), Option<usize>>, // batch of each walked definition; None: walked per instance
    of_row: HashMap<u32, Option<usize>>, // batch of each instance row; None: walked per instance
    member_at: HashMap<u32, usize>,      // index of each shared instance row in its batch's members
    dirty: bool,                         // the GPU draw list is stale
    full: bool,                          // every batch was walked again: all slots are stale
    touched: Vec<(u32, u32)>,            // (batch key, member position) written since the upload
    next_key: u32,                       // key of the next batch
}

impl Instancing {
    /// Forget every batch; keys stay unique, so the GPU never takes a new batch for one it holds.
    pub(crate) fn clear(&mut self) {
        *self = Self {
            next_key: self.next_key,
            full: true,
            ..Self::default()
        };
    }

    /// Live batches.
    fn live(&self) -> impl Iterator<Item = &Batch> {
        self.batches.iter().flatten()
    }

    /// Hidden rows that hold definitions.
    pub(crate) fn batch_rows(&self) -> usize {
        self.live().count()
    }

    /// Instance rows drawn from a shared definition.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn shared_rows(&self) -> usize {
        self.of_row.values().filter(|batch| batch.is_some()).count()
    }

    /// True for a row that draws an instance.
    pub(crate) fn is_instance(&self, row: u32) -> bool {
        self.of_row.contains_key(&row)
    }

    /// Row `row` draws an instance of batch `batch`, or of none when walked per instance.
    fn join(&mut self, row: u32, batch: Option<usize>) {
        self.of_row.insert(row, batch);

        if let Some(batch) = batch.and_then(|b| self.batches[b].as_mut()) {
            self.member_at.insert(row, batch.members.len());
            self.touched.push((batch.key, batch.members.len() as u32));
            batch.members.push(row);
            self.dirty = true;
        }
    }

    /// Row `row` draws no instance any more, in O(1); its batch and the members it has left.
    fn leave(&mut self, row: u32) -> Option<(usize, usize)> {
        let index = self.of_row.remove(&row)??;
        let at = self.member_at.remove(&row)?;
        let batch = self.batches[index].as_mut()?;
        let members = &mut batch.members;
        members.swap_remove(at);

        // the last member moved into the gap
        if let Some(&moved) = members.get(at) {
            self.member_at.insert(moved, at);
            self.touched.push((batch.key, at as u32));
        }

        self.dirty = true;
        Some((index, members.len()))
    }

    /// True for a hidden row that holds a definition.
    #[cfg(test)]
    pub(crate) fn is_batch(&self, row: u32) -> bool {
        self.live().any(|batch| batch.row == row)
    }
}

impl Scene {
    /// One row per drawable instance of document `doc`, placed like any object; `placed` gets
    /// each for the node cache. Instance flags seed hidden and locked; their colour is not used.
    pub(in crate::app::scene) fn add_instances<'s>(
        &mut self,
        doc: usize,
        session: &'s Session,
        place: &Xform,
        world: &HashMap<String, Xform>,
        placed: &mut HashMap<&'s str, u32>,
    ) {
        for instance in &session.objects.instances {
            let Some(definition) = session.definition_lookup.get(&instance.definition_guid) else {
                continue;
            };

            if !is_drawable(definition) {
                continue;
            }

            let guid = instance.guid();
            let key = (doc, Rc::from(guid));

            if instance.flags & InstanceRef::FLAG_HIDDEN != 0 {
                self.hidden.insert(key.clone());
            }

            if instance.flags & InstanceRef::FLAG_LOCKED != 0 {
                self.locked.insert(key.clone());
            }

            let flags = if self.hidden.contains(&key) {
                Instance::FLAG_HIDDEN
            } else {
                0
            };
            let at = placement(world, place, guid);
            let row = self.push_row(doc, guid, at.clone(), flags);
            placed.insert(guid, row);
            let (foot, walked, hull) = self.draw_instance(doc, row, instance, definition, &at);
            self.feet[row as usize] = foot;
            let at = (row - self.object_rows) as usize;
            take_walk(&mut self.tables.obj.rows[at], &walked, hull);
        }
    }

    /// Walk the rows an instance owns alone, appended after every row: the markers of its
    /// definition's shared walk and its own features, or, for a definition drawn per instance,
    /// the whole definition. Returns its footprint and what its object row takes.
    fn draw_instance(
        &mut self,
        doc: usize,
        row: u32,
        instance: &InstanceRef,
        definition: &Geometry,
        place: &Xform,
    ) -> (Footprint, Row, Option<Hull>) {
        let batch = self.batch_for(doc, &instance.definition_guid, definition, place);
        let (foot, walked, hull) = self.walk_instance(doc, row, instance, definition, batch);
        self.instancing.join(row, batch);
        (foot, walked, hull)
    }

    /// Walk what an instance row owns alone, drawing a shared definition from `batch`; its
    /// footprint, what its object row takes and its extreme points.
    fn walk_instance(
        &mut self,
        doc: usize,
        row: u32,
        instance: &InstanceRef,
        definition: &Geometry,
        batch: Option<usize>,
    ) -> (Footprint, Row, Option<Hull>) {
        let mut up = Upload::default();
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: self.docs.get(doc).map_or(0.0, |file| file.point_px),
            row,
            attributes: self.attributes,
        };
        let mut walked = match batch {
            Some(index) => {
                let batch = self.instancing.batches[index].as_ref().unwrap();
                let own = |glyph: &GlyphPoint| GlyphPoint {
                    instance_id: row,
                    ..*glyph
                };
                up.glyph.spheres = batch.walked.spheres.iter().map(own).collect();
                up.glyph.dots = batch.walked.dots.iter().map(own).collect();
                Row {
                    bounds: batch.walked.bounds,
                    spacing: batch.walked.spacing,
                    flags: batch.walked.flags,
                    faces: batch.walked.faces,
                }
            }
            None => walk_geometry(&mut Walk::of(&mut up), &cx, definition),
        };
        let features = self.attributes && !instance.features.is_empty();

        if features {
            walk_features(
                &mut Walk::of(&mut up),
                &cx,
                &instance.features,
                &mut walked.bounds,
            );
        }

        let shared = batch.and_then(|index| self.instancing.batches[index].as_ref());
        let hull = match shared.map(|b| b.walked.hull.clone()) {
            // the definition's points, and the instance's own features when it draws any
            Some(Some(hull)) if features => hull_of(&up, &hull, &walked.bounds),
            Some(hull) => hull,
            None => hull_of(&up, &[], &walked.bounds),
        };
        let cloud = matches!(definition, Geometry::PointCloud(_));
        (self.append(up, cloud), walked, hull)
    }

    /// The batch of a definition of `doc`, walked now if it is new, its hidden row placed like the
    /// instance at `place`; None when it is drawn per instance: a type without faces, or a walk
    /// with rows no instanced draw reads.
    fn batch_for(
        &mut self,
        doc: usize,
        guid: &str,
        definition: &Geometry,
        place: &Xform,
    ) -> Option<usize> {
        let key = (doc, Rc::from(guid));

        if let Some(&batch) = self.instancing.by_definition.get(&key) {
            return batch;
        }

        let batch = self.walk_batch(doc, &key.1, definition, place);
        self.instancing.by_definition.insert(key, batch);
        batch
    }

    /// Walk `definition` once under a hidden row placed at `place`; None when instances of it are
    /// drawn one by one.
    fn walk_batch(
        &mut self,
        doc: usize,
        guid: &Rc<str>,
        definition: &Geometry,
        place: &Xform,
    ) -> Option<usize> {
        let shared = matches!(
            definition,
            Geometry::Mesh(_)
                | Geometry::BRep(_)
                | Geometry::NurbsSurface(_)
                | Geometry::Element(_)
        );

        if !shared {
            return None;
        }

        let row = self.hidden_row();
        let (mut up, walk) = self.shared_walk(row, definition);
        let counts = Counts::of(&up);

        // sheet fills, lettering and registered lanes have no instanced draw
        if counts.print + counts.text > 0 || counts.lanes.iter().any(|rows| *rows > 0) {
            self.free_hidden_row(row);
            return None;
        }

        let hull = hull_of(&up, &[], &walk.bounds);
        let spheres = std::mem::take(&mut up.glyph.spheres);
        let dots = std::mem::take(&mut up.glyph.dots);

        self.feet[row as usize] = self.append(up, false);
        // hidden and dead: never drawn, boxed or cut; ambient occlusion reads its box
        let mut object = ObjectRow::new(place.clone(), Instance::FLAG_HIDDEN | Instance::FLAG_DEAD);
        object.bounds = walk.bounds;
        self.set_object_row(row, object);
        let key = self.instancing.next_key;
        self.instancing.next_key += 1;
        let batch = Batch {
            key,
            doc,
            definition: Rc::clone(guid),
            row,
            walked: Walked {
                bounds: walk.bounds,
                spacing: walk.spacing,
                flags: walk.flags,
                faces: walk.faces,
                spheres,
                dots,
                hull,
            },
            members: Vec::new(),
        };
        self.instancing.dirty = true;
        let slot = self.instancing.batches.iter().position(Option::is_none);

        Some(match slot {
            Some(slot) => {
                self.instancing.batches[slot] = Some(batch);
                slot
            }
            None => {
                self.instancing.batches.push(Some(batch));
                self.instancing.batches.len() - 1
            }
        })
    }

    /// A definition's shared rows walked under hidden row `row`.
    fn shared_walk(&mut self, row: u32, definition: &Geometry) -> (Upload, Row) {
        let mut up = Upload::default();
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row,
            attributes: self.attributes,
        };
        let walk = walk_geometry(&mut Walk::of(&mut up), &cx, definition);
        (up, walk)
    }

    /// A row with no identity, for a definition's shared rows: a freed id, else one after every row.
    fn hidden_row(&mut self) -> u32 {
        if let Some(row) = self.ids.take() {
            let i = row as usize;
            self.order[i] = Rc::clone(&self.empty);
            self.owners[i] = SINK;
            return row;
        }

        let row = self.object_rows + self.tables.obj.rows.len() as u32;
        self.tables.obj.rows.push(ObjectRow::new(
            Xform::identity(),
            Instance::FLAG_HIDDEN | Instance::FLAG_DEAD,
        ));
        self.order.push(Rc::clone(&self.empty));
        self.owners.push(SINK);
        self.feet.push(Footprint::None);
        self.nodes.push(Weak::new());
        row
    }

    /// Give a hidden row back; its lane rows go to the sink.
    fn free_hidden_row(&mut self, row: u32) {
        let i = row as usize;
        let foot = std::mem::take(&mut self.feet[i]);
        let span = self.spans.span(foot);
        self.retire(span, false, &(FREE, Rc::clone(&self.empty)), foot);

        // a row still in the tables was made hidden and dead; one on the GPU is retired there
        if row < self.object_rows {
            self.staged.retire.push(row);
        }

        self.owners[i] = FREE;
        self.ids.give(row);
    }

    /// Write a whole object row, on the GPU or still in the tables.
    fn set_object_row(&mut self, row: u32, object: ObjectRow) {
        if row < self.object_rows {
            self.staged.rows.push((row, object));
        } else {
            self.tables.obj.rows[(row - self.object_rows) as usize] = object;
        }
    }

    /// Drop a batch: its hidden row and shared rows go; its members must be gone or moved.
    fn drop_batch(&mut self, index: usize) {
        let Some(batch) = self.instancing.batches[index].take() else {
            return;
        };
        self.instancing
            .by_definition
            .remove(&(batch.doc, batch.definition));
        self.free_hidden_row(batch.row);
        self.instancing.dirty = true;
    }

    /// Kill an instance row; its batch goes with its last member.
    fn kill_instance(&mut self, row: u32) {
        if let Some((index, 0)) = self.instancing.leave(row) {
            self.drop_batch(index);
        }

        self.kill(row);
    }

    /// Kill, create, redraw or move the row of an instance, or walk a changed definition again;
    /// None when `item` names neither.
    pub(super) fn reconcile_instance(&mut self, item: &Work) -> Option<bool> {
        let doc = item.doc;
        let session = Rc::clone(&self.docs[doc].session);
        let key = (doc, Rc::clone(&item.guid));

        if session.definition_lookup.contains_key(item.guid.as_ref())
            || self.instancing.by_definition.contains_key(&key)
        {
            self.redefine(doc, &item.guid);
            return Some(true);
        }

        let instance = session.instance_lookup.get(item.guid.as_ref());
        let row = self.guid_to_row.get(&key).copied();
        let ours = row.is_some_and(|row| self.instancing.is_instance(row));

        if instance.is_none() && !ours {
            return None;
        }

        let under = item.in_tree && item.node.as_ref().is_some_and(baked);
        let definition = instance.and_then(|i| session.definition_lookup.get(&i.definition_guid));
        let wanted = definition.is_some_and(is_drawable) && !under;

        // the same guid as another kind of object: to_instance or explode swapped it
        if let Some(row) = row
            && !ours
        {
            self.kill(row);
        }

        let row = row.filter(|_| ours);

        match (row, instance, definition) {
            (Some(row), Some(instance), Some(definition)) if wanted => {
                let place = self.world_place(doc, item.node.as_ref(), item.in_tree, &item.guid);

                if let Some(node) = &item.node {
                    self.nodes[row as usize] = Rc::downgrade(node);
                }

                if item.what & GEOMETRY != 0 {
                    self.redraw_instance(row, instance, definition, place);
                } else {
                    self.staged.places.push((row, place));
                }

                Some(false)
            }
            (None, Some(instance), Some(definition)) if wanted => {
                self.create_instance(item, instance, definition);
                Some(true)
            }
            (Some(row), _, _) => {
                self.kill_instance(row);

                // an exploded instance comes back as its own object, which the plain path draws
                if session.lookup.contains_key(item.guid.as_ref()) {
                    return None;
                }

                Some(true)
            }
            _ => Some(false),
        }
    }

    /// Give an instance a row: a freed id, else one after every row.
    fn create_instance(&mut self, item: &Work, instance: &InstanceRef, definition: &Geometry) {
        let doc = item.doc;
        let place = self.world_place(doc, item.node.as_ref(), item.in_tree, &item.guid);
        let row = match self.ids.take() {
            Some(row) => {
                let i = row as usize;
                self.order[i] = Rc::clone(&item.guid);
                self.owners[i] = doc;
                row
            }
            None => {
                let row = self.object_rows + self.tables.obj.rows.len() as u32;
                self.tables
                    .obj
                    .rows
                    .push(ObjectRow::new(Xform::identity(), 0));
                self.order.push(Rc::clone(&item.guid));
                self.owners.push(doc);
                self.feet.push(Footprint::None);
                self.nodes.push(Weak::new());
                row
            }
        };
        let i = row as usize;
        self.nodes[i] = item.node.as_ref().map(Rc::downgrade).unwrap_or_default();
        self.guid_to_row.insert((doc, Rc::clone(&item.guid)), row);
        let (foot, walked, hull) = self.draw_instance(doc, row, instance, definition, &place);
        self.feet[i] = foot;
        let hidden = if self.hidden.contains(&(doc, Rc::clone(&item.guid))) {
            Instance::FLAG_HIDDEN
        } else {
            0
        };
        let mut object = self.object_row(doc, &item.guid, place, hidden);
        take_walk(&mut object, &walked, hull);

        if row >= self.object_rows {
            let world = world_box(&object);

            if world.is_valid() {
                self.tables.bounds.union_with(&world);
            }
        }

        self.set_object_row(row, object);
    }

    /// Walk an instance's own rows again, after every row, and update its box and flags.
    fn redraw_instance(
        &mut self,
        row: u32,
        instance: &InstanceRef,
        definition: &Geometry,
        place: Xform,
    ) {
        let i = row as usize;
        let foot = std::mem::take(&mut self.feet[i]);

        if foot == Footprint::Cloud {
            self.staged.clouds.push(row);
        } else {
            let span = self.spans.span(foot);
            self.retire(span, false, &(FREE, Rc::clone(&self.empty)), foot);
        }

        self.instancing.leave(row);
        let doc = self.owners[i];
        let (foot, walked, hull) = self.draw_instance(doc, row, instance, definition, &place);
        self.feet[i] = foot;
        let mut object = ObjectRow::new(place, 0);
        take_walk(&mut object, &walked, hull);
        self.staged.geometry.push((row, object));
        self.bounds_stale = true;
    }

    /// A definition of `doc` changed: its batch walks again and every instance of it is redrawn.
    fn redefine(&mut self, doc: usize, guid: &Rc<str>) {
        let session = Rc::clone(&self.docs[doc].session);
        let key = (doc, Rc::clone(guid));

        if let Some(Some(index)) = self.instancing.by_definition.get(&key).copied() {
            let members = self.instancing.batches[index]
                .as_mut()
                .map(|batch| std::mem::take(&mut batch.members))
                .unwrap_or_default();

            for row in members {
                self.instancing.of_row.remove(&row);
                self.instancing.member_at.remove(&row);
            }

            self.drop_batch(index);
        }

        self.instancing.by_definition.remove(&key);

        for instance in session.instances_of(guid.as_ref()) {
            let Some(row) = self.row_of(doc, &instance) else {
                continue;
            };
            let (Some(reference), Some(definition)) = (
                session.instance_lookup.get(&instance),
                session.definition_lookup.get(guid.as_ref()),
            ) else {
                continue;
            };
            let Some(place) = self.placement_of(row) else {
                continue;
            };
            self.redraw_instance(row, reference, definition, place);
        }
    }

    /// Walk the batches and instance rows of `doc` again into fresh lanes, after `rewalk_doc`.
    pub(super) fn rewalk_instances(&mut self, doc: usize) {
        let session = Rc::clone(&self.docs[doc].session);
        let batches: Vec<usize> = (0..self.instancing.batches.len())
            .filter(|&i| {
                self.instancing.batches[i]
                    .as_ref()
                    .is_some_and(|b| b.doc == doc)
            })
            .collect();

        // the shared rows moved: the draws are stale only when this document has any
        self.instancing.dirty |= !batches.is_empty();
        self.instancing.full |= !batches.is_empty();

        for index in batches {
            let (row, guid) = {
                let batch = self.instancing.batches[index].as_ref().unwrap();
                (batch.row, Rc::clone(&batch.definition))
            };
            let Some(definition) = session.definition_lookup.get(guid.as_ref()) else {
                continue;
            };
            let (mut up, walk) = self.shared_walk(row, definition);
            // the attributes may have been switched: what instances take is walked again too
            let hull = hull_of(&up, &[], &walk.bounds);
            let walked = Walked {
                bounds: walk.bounds,
                spacing: walk.spacing,
                flags: walk.flags,
                faces: walk.faces,
                spheres: std::mem::take(&mut up.glyph.spheres),
                dots: std::mem::take(&mut up.glyph.dots),
                hull,
            };
            self.feet[row as usize] = self.append(up, false);

            if let Some(batch) = self.instancing.batches[index].as_mut() {
                batch.walked = walked;
            }
        }

        for instance in &session.objects.instances {
            let Some(row) = self.row_of(doc, instance.guid()) else {
                continue;
            };
            let Some(definition) = session.definition_lookup.get(&instance.definition_guid) else {
                continue;
            };

            if self.feet[row as usize] == Footprint::Cloud {
                continue;
            }

            let batch = self.instancing.of_row.get(&row).copied().flatten();
            let (foot, walked, hull) = self.walk_instance(doc, row, instance, definition, batch);
            self.feet[row as usize] = foot;
            let place = self.placement_of(row).unwrap_or_else(Xform::identity);
            let mut object = ObjectRow::new(place, 0);
            take_walk(&mut object, &walked, hull);
            self.staged.geometry.push((row, object));
        }
    }

    /// The instance rows in slot order and one draw per batch, from where the shared rows sit now.
    #[cfg(test)]
    pub(crate) fn instanced_draws(&self) -> (Vec<u32>, Vec<crate::engine::gpu::slots::Draw>) {
        let mut rows = Vec::new();
        let mut draws = Vec::new();

        for batch in self.instancing.live() {
            if batch.members.is_empty() {
                continue;
            }

            let span = self.spans.span(self.feet[batch.row as usize]);
            let first = 1 + rows.len() as u32; // slot 0 keeps plain rows their own
            let (s, c) = (span.start, span.count);
            rows.extend_from_slice(&batch.members);
            draws.push(crate::engine::gpu::slots::Draw {
                faces: s.faces..s.faces + c.faces,
                pipes: s.pipes..s.pipes + c.pipes,
                ribbons: s.ribbons..s.ribbons + c.ribbons,
                slots: first..first + batch.members.len() as u32,
            });
        }

        (rows, draws)
    }

    /// Send the instance slots and draws to the GPU when they changed.
    pub(in crate::app::scene) fn upload_instances(&mut self, gpu: &mut Gpu) {
        if !std::mem::take(&mut self.instancing.dirty) {
            return;
        }

        let touched = std::mem::take(&mut self.instancing.touched);
        let full = std::mem::take(&mut self.instancing.full);
        let definitions: Vec<Definition> = self
            .instancing
            .live()
            .map(|batch| {
                let span = self.spans.span(self.feet[batch.row as usize]);
                let (s, c) = (span.start, span.count);
                Definition {
                    key: batch.key,
                    rows: &batch.members,
                    faces: s.faces..s.faces + c.faces,
                    pipes: s.pipes..s.pipes + c.pipes,
                    ribbons: s.ribbons..s.ribbons + c.ribbons,
                }
            })
            .collect();
        gpu.set_instanced(&definitions, &touched, full);
    }

    /// The definition an instance row draws, in its own frame; the row's placement places it.
    pub fn instance_definition(&self, row: u32) -> Option<&Geometry> {
        if !self.instancing.is_instance(row) {
            return None;
        }

        let (doc, guid) = self.identity_of(row)?;
        let session = &self.docs.get(doc)?.session;
        let instance = session.instance_lookup.get(guid.as_ref())?;
        session.definition_lookup.get(&instance.definition_guid)
    }

    /// The shared solid face indices an instance row draws, in its definition's hidden row.
    pub fn instance_faces(&self, row: u32) -> Option<std::ops::Range<u32>> {
        let batch = (*self.instancing.of_row.get(&row)?)?;
        self.face_range(self.instancing.batches.get(batch)?.as_ref()?.row)
    }

    /// The hidden row holding the definition an instance row draws.
    pub fn instance_batch_row(&self, row: u32) -> Option<u32> {
        let batch = (*self.instancing.of_row.get(&row)?)?;
        Some(self.instancing.batches.get(batch)?.as_ref()?.row)
    }

    /// The geometry of a row: its own, or an instance's definition in the definition's frame.
    pub fn shape_of(&self, row: u32) -> Option<&Geometry> {
        self.geometry(row).or_else(|| self.instance_definition(row))
    }

    /// The name of an instance row, or "Instance" when it has none.
    pub fn instance_name(&self, row: u32) -> Option<&str> {
        if !self.instancing.is_instance(row) {
            return None;
        }

        let (doc, guid) = self.identity_of(row)?;
        let name = self
            .docs
            .get(doc)?
            .session
            .instance_lookup
            .get(guid.as_ref())?
            .name
            .as_str();
        Some(if name.trim().is_empty() {
            "Instance"
        } else {
            name
        })
    }
}

/// Box, extreme points, spacing, flags and faces of a walk onto an object row.
fn take_walk(object: &mut ObjectRow, walked: &Row, hull: Option<Hull>) {
    object.flags |= walked.flags;
    object.bounds = walked.bounds;
    object.hull = hull;
    object.spacing = walked.spacing;
    object.faces = walked.faces;

    if walked.faces {
        object.flags |= Instance::FLAG_HAS_FACES;
    }
}

#[cfg(test)]
mod tests {
    use super::super::FileDoc;
    use super::*;
    use session_rust::element::ElementFeature;
    use session_rust::{Element, Mesh, Point, Polyline};

    /// A document at the origin.
    fn file(session: Session) -> FileDoc {
        FileDoc {
            name: "placed".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        }
    }

    /// Sync, upload without a GPU and compare with a scene loaded fresh.
    fn check(scene: &mut Scene) {
        scene.sync();
        scene.settle();
        scene.verify();
    }

    /// A box element with an axis feature, defined once and placed `n` times along x, the last
    /// under a group moved up by 10; the first instance carries a feature of its own.
    fn placed(n: usize) -> (Session, String) {
        placed_with(n, 0)
    }

    /// `placed` with `flags` on the second instance.
    fn placed_with(n: usize, flags: u32) -> (Session, String) {
        let mut s = Session::new("placed");
        let mut element = Element::new("box");
        element.set_geometry(Mesh::create_box(1.0, 1.0, 1.0));
        let axis = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(0.0, 0.0, 2.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let definition = s.add_definition(Geometry::Element(Rc::new(element)));
        let group = s.add_group("row");
        s.set_xform("row", Xform::translation(0.0, 0.0, 10.0));

        for i in 0..n {
            let mut instance =
                InstanceRef::with_name(&format!("box_{i}"), &definition, Xform::identity());

            if i == 1 {
                instance.flags = flags;
            }

            if i == 0 {
                let dot = Polyline::new(vec![Point::new(0.5, 0.5, 0.5)]);
                instance
                    .features
                    .push(ElementFeature::new("contact", 0, vec![dot], "dot"));
            }

            let parent = (i + 1 == n).then_some(&group);
            s.add_instance(
                instance,
                Xform::translation(i as f64 * 3.0, 0.0, 0.0),
                parent,
            );
        }

        (s, definition)
    }

    /// A 4 by 3 grid of one beam element with an axis poking out of both ends, every instance
    /// turned its own way about z and tipped about x, on a floor: beams hide each other's edges
    /// and axes without touching, and stand close enough to the floor for contact shadows.
    fn turned_grid() -> Session {
        let mut s = Session::new("turned");
        let mut element = Element::new("beam");
        element.set_geometry(Mesh::create_box(3.0, 1.0, 1.0));
        let axis = Polyline::new(vec![Point::new(-2.0, 0.0, 0.0), Point::new(2.0, 0.0, 0.0)]);
        element.add_feature(ElementFeature::new("axis", -1, vec![axis], "axis"));
        let beam = s.add_definition(Geometry::Element(Rc::new(element)));
        let floor = s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(18.0, 13.0, 0.2))));

        for i in 0..12 {
            let turn = &Xform::rotation_z(i as f64 * 37.0, true)
                * &Xform::rotation_x((i % 3) as f64 * 20.0, true);
            let at = Xform::translation((i % 4) as f64 * 3.5, (i / 4) as f64 * 3.5, 0.0);
            let instance = InstanceRef::with_name(&format!("beam_{i}"), &beam, Xform::identity());
            s.add_instance(instance, &at * &turn, None);
        }

        let under = Xform::translation(5.25, 3.5, -0.9);
        s.add_instance(InstanceRef::new(&floor, Xform::identity()), under, None);
        s
    }

    /// The same objects with every instance baked into its own element or mesh.
    fn bake(source: &Session) -> Session {
        let mut baked = Session::new("baked");
        let geometry = source.get_geometry();

        for element in &geometry.elements {
            baked.add_element((**element).clone(), None);
        }

        for mesh in &geometry.meshes {
            baked.add_mesh((**mesh).clone(), None);
        }

        baked
    }

    /// Instance rows of a scene, by guid.
    fn rows(scene: &Scene) -> Vec<(String, u32)> {
        let mut out: Vec<_> = (0..scene.row_count() as u32)
            .filter(|row| scene.instancing.is_instance(*row))
            .map(|row| (scene.identity_of(row).unwrap().1.to_string(), row))
            .collect();
        out.sort();
        out
    }

    /// One walk of the definition serves every instance: its vertices and edges once, one draw
    /// with a slot per instance, each row placed by its own world transform.
    #[test]
    fn a_definition_is_walked_once_for_every_instance() {
        let (source, _) = placed(3);
        let mut one = Scene::new();
        let (single, _) = placed(1);
        one.add_file(file(single));
        let mut scene = Scene::new();
        scene.add_file(file(source.clone()));
        scene.settle();
        scene.verify();

        assert_eq!(scene.object_count(), 3);
        assert_eq!(scene.instancing.batch_rows(), 1);
        assert_eq!(
            scene.uploaded.verts,
            one.tables.arena.verts.len() as u32,
            "vertices once"
        );
        assert_eq!(
            scene.uploaded.pipes,
            one.tables.seg.pipes.len() as u32,
            "edges once"
        );
        let (slots, draws) = scene.instanced_draws();
        assert_eq!(slots.len(), 3);
        assert_eq!(draws.len(), 1);
        assert_eq!(draws[0].slots, 1..4);
        assert!(!draws[0].faces.is_empty() && !draws[0].pipes.is_empty());

        for (guid, row) in rows(&scene) {
            let world = source.world_xform(&guid);
            assert_eq!(scene.placement_of(row).unwrap().m, world.m, "{guid}");
            assert!(scene.face_range(row).is_none(), "{guid} owns no faces");
        }

        // markers and the instance's own dot are drawn per instance, in its row
        let first = rows(&scene)
            .into_iter()
            .find(|(guid, _)| source.instance_lookup[guid].name == "box_0")
            .unwrap()
            .1;
        let span = scene.spans.span(scene.feet[first as usize]);
        assert!(span.count.dots > 0 && span.count.verts == 0);
        assert_eq!(scene.object_name(first), "box_0");
    }

    /// A pick on an instance's edge names the definition's edge on that instance, never on
    /// another row; the instance offers its definition's corners placed where it stands.
    #[test]
    fn instance_edges_pick_and_corners_snap() {
        use crate::app::snap::{SnapKind, of_geometry};
        use crate::engine::gpu::pick::Pick;

        let (source, _) = placed(3);
        let mut scene = Scene::new();
        scene.add_file(file(source.clone()));
        scene.settle();
        let instances = rows(&scene);
        let batch = scene.instance_batch_row(instances[0].1).unwrap();
        let (pipe, edge) = scene
            .edge_sources
            .iter()
            .enumerate()
            .find_map(|(i, &(owner, edge))| {
                (owner == batch && edge != u32::MAX).then_some((i, edge))
            })
            .expect("the definition's edges");
        let sub = 0x8000_0000 | pipe as u32;

        for (guid, row) in &instances {
            assert_eq!(scene.edge_at(Pick { row: *row, sub }), Some(edge), "{guid}");
        }

        let other = Pick {
            row: batch + 100,
            sub,
        };
        assert_eq!(scene.edge_at(other), None, "not another row's");

        for (guid, row) in &instances {
            let shape = scene.shape_of(*row).expect("the definition");
            let place = scene.placement_of(*row).unwrap();
            let (mut snaps, mut wires) = (Vec::new(), Vec::new());
            of_geometry(shape, &place, *row, 64, &mut snaps, &mut wires);
            let world = source.world_xform(guid);
            let corner = session_rust::Point::new(0.5, 0.5, 0.5).transformed(&world);
            let found = snaps.iter().any(|snap| {
                snap.kind == SnapKind::Vertex
                    && snap.owner == *row
                    && (0..3).all(|k| (snap.point[k] - corner[k]).abs() < 1e-9)
            });
            assert!(found, "{guid} snaps to its corner at {corner:?}");
        }

        assert!(
            scene.shape_of(batch).is_none(),
            "no snaps on the definition"
        );
    }

    /// Hidden and locked instances come from their flags, per instance.
    #[test]
    fn instance_flags_hide_and_lock_one_instance() {
        let (source, _) = placed_with(3, InstanceRef::FLAG_HIDDEN | InstanceRef::FLAG_LOCKED);
        let guid = source.objects.instances.get(1).unwrap().guid().to_string();
        let mut scene = Scene::new();
        scene.add_file(file(source));
        scene.settle();
        let row = scene.row_of(0, &guid).unwrap();
        assert!(scene.hidden.contains(&(0, Rc::from(guid.as_str()))));
        assert!(!scene.selectable(row));
        assert_ne!(scene.ledger[&row].flags & Instance::FLAG_HIDDEN, 0);
        assert_eq!(scene.hidden.len(), 1);
    }

    /// Deleting, moving and undoing instances keeps the rows a fresh load would make; the batch
    /// goes with its last instance and comes back on undo.
    #[test]
    fn undo_and_redo_of_instance_edits_sync() {
        let (source, _) = placed(3);
        let mut scene = Scene::new();
        scene.add_file(file(source));
        scene.settle();
        let before = rows(&scene);
        let moved = before[0].1;
        assert!(
            scene
                .transform_rows(&[moved], &Xform::translation(0.0, 5.0, 0.0), "Move")
                .is_some()
        );
        check(&mut scene);
        assert_eq!(scene.placement_of(moved).unwrap().m[13], 5.0);

        for (_, row) in &before {
            assert!(scene.delete_row(*row));
            check(&mut scene);
        }

        assert_eq!(
            scene.instancing.batch_rows(),
            0,
            "the batch went with the last instance"
        );
        assert!(scene.instanced_draws().1.is_empty());

        for _ in 0..3 {
            assert!(scene.undo());
            check(&mut scene);
        }

        assert_eq!(rows(&scene).len(), 3);
        assert_eq!(scene.instancing.batch_rows(), 1);
        assert_eq!(scene.instanced_draws().0.len(), 3);
        assert!(scene.undo());
        check(&mut scene);
        assert_eq!(scene.placement_of(moved).unwrap().m[13], 0.0, "move undone");
        assert!(scene.redo());
        check(&mut scene);
        assert_eq!(scene.placement_of(moved).unwrap().m[13], 5.0);
    }

    /// Switching the attributes walks shared definitions again: the instances lose the axis.
    #[test]
    fn attributes_switch_reaches_every_instance() {
        let (source, _) = placed(2);
        let mut scene = Scene::new();
        scene.add_file(file(source));
        scene.settle();
        let ribbons = scene.uploaded.ribbons;
        scene.attributes = false;
        scene.rewalk_cpu();
        scene.verify();
        assert!(scene.uploaded.ribbons < ribbons, "no feature ribbons");
        assert_eq!(
            scene
                .spans
                .span(scene.feet[rows(&scene)[0].1 as usize])
                .count
                .dots,
            0
        );
    }

    /// A released document keeps each instance row's definition shape and instance name.
    #[test]
    fn released_instances_keep_shape_and_name() {
        let (source, _) = placed(2);
        let mut scene = Scene::new();
        scene.add_file(file(source));
        scene.settle();
        let row = rows(&scene)
            .into_iter()
            .find(|(guid, _)| scene.docs[0].session.instance_lookup[guid].name == "box_0")
            .unwrap()
            .1;
        scene.release(0, 0, "placed.pb".into());
        assert!(scene.docs[0].session.instance_lookup.is_empty());
        assert_eq!(scene.object_name(row), "box_0");
        assert_eq!(scene.shape(row), Some(super::super::Shape::Element));
    }

    /// Exploding an instance into its own element and undoing it swap the row's kind, nothing else.
    #[test]
    fn explode_and_undo_swap_the_row() {
        let (source, _) = placed(2);
        let mut scene = Scene::new();
        scene.add_file(file(source));
        scene.settle();
        let (guid, _) = rows(&scene)[0].clone();
        let session = Rc::make_mut(&mut scene.docs[0].session);
        session.begin("Explode");
        assert!(session.explode(&guid));
        let notes = super::super::sync::commit(session);
        scene.noted(0, notes);
        scene.edited(&[0]);
        check(&mut scene);
        let row = scene.row_of(0, &guid).unwrap();
        assert!(!scene.instancing.is_instance(row));
        assert!(matches!(scene.geometry(row), Some(Geometry::Element(_))));
        assert_eq!(scene.instanced_draws().0.len(), 1);

        assert!(scene.undo());
        check(&mut scene);
        let row = scene.row_of(0, &guid).unwrap();
        assert!(scene.instancing.is_instance(row));
        assert_eq!(scene.instanced_draws().0.len(), 2);
    }

    /// A replaced definition redraws every instance of it; compaction walks batches again.
    #[test]
    fn a_replaced_definition_and_a_compaction_redraw_the_instances() {
        let (source, definition) = placed(2);
        let mut scene = Scene::new();
        scene.add_file(file(source));
        scene.settle();
        let session = Rc::make_mut(&mut scene.docs[0].session);
        session.begin("Replace");
        let bigger = Mesh::create_box(2.0, 2.0, 2.0);
        assert!(session.replace_definition(&definition, Geometry::Mesh(Rc::new(bigger))));
        let notes = super::super::sync::commit(session);
        scene.noted(0, notes);
        scene.edited(&[0]);
        check(&mut scene);
        let row = rows(&scene)[0].1;
        assert_eq!(scene.ledger[&row].bounds.hx, 1.0, "the bigger box");
        assert_eq!(scene.instancing.batch_rows(), 1);

        scene.rewalk_cpu();
        scene.verify();
        assert_eq!(scene.instanced_draws().0.len(), 2);
    }

    /// Frames drawn by a headless GPU; every test here needs a native adapter.
    #[cfg(not(target_arch = "wasm32"))]
    mod gpu {
        use super::*;
        use crate::camera::Camera;
        use crate::engine::gpu::FrameInput;

        /// A 400 by 300 frame as `<dir>/<name>.ppm`, for looking at a failure.
        fn write_ppm(dir: &std::ffi::OsStr, name: &str, rgba: &[u8]) {
            let mut ppm = b"P6 400 300 255\n".to_vec();
            ppm.extend(rgba.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]));
            std::fs::write(std::path::Path::new(dir).join(format!("{name}.ppm")), ppm).unwrap();
        }

        /// `session` on a headless GPU, with ambient occlusion when `ssao`, and a camera fitted
        /// to it, orbited when `rotate`.
        fn loaded(session: Session, rotate: bool, ssao: bool) -> (Gpu, Scene, Camera) {
            let mut gpu =
                pollster::block_on(Gpu::new_headless(400, 300)).expect("a native adapter");
            gpu.view.show_grid = false;
            gpu.view.opacity = 0.7;
            gpu.view.set_arctic(ssao);
            let mut scene = Scene::new();
            scene.add_file(file(session));
            scene.upload_to(&mut gpu);
            let mut camera = Camera::new();
            camera.fit(&gpu.bounds, 4.0 / 3.0);

            if rotate {
                camera.orbit(0.6, 0.4);
            }

            (gpu, scene, camera)
        }

        /// Colors and object ids of one frame of `session`, fitted to its box, with ambient
        /// occlusion when `ssao`.
        fn shot_with(
            session: Session,
            rotate: bool,
            ssao: bool,
        ) -> (Vec<u8>, Vec<Option<String>>, Gpu, Scene) {
            let (mut gpu, scene, camera) = loaded(session, rotate, ssao);
            let (color, ids) = frame(&mut gpu, &scene, &camera);
            (color, ids, gpu, scene)
        }

        /// Colors and object ids of one frame seen by `camera`.
        fn frame(gpu: &mut Gpu, scene: &Scene, camera: &Camera) -> (Vec<u8>, Vec<Option<String>>) {
            let anchor = gpu
                .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
                .anchor;
            let input = FrameInput {
                view_proj: camera.view_proj_anchored(4.0 / 3.0, &anchor),
                clear: wgpu::Color::WHITE,
                now_ms: 0.0,
            };
            let color = gpu.render_offscreen(&input);
            let ids = gpu
                .render_ids_offscreen(&input)
                .iter()
                .map(|&[object, _]| {
                    (object != 0).then(|| scene.identity_of(object - 1).unwrap().1.to_string())
                })
                .collect();
            (color, ids)
        }

        /// Instances drawn from one upload look and pick exactly like the same objects baked:
        /// every pixel's object is the instance's guid, and faces, hidden edges and axes seen
        /// through the glass, with and without ambient occlusion, match to the pixel.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn instanced_frames_match_the_baked_scene() {
            let scenes = [(placed(3).0, 1, 3), (turned_grid(), 2, 13)];

            for (source, batches, instances) in scenes {
                let baked = bake(&source);

                for (rotate, ssao) in [(false, false), (true, false), (true, true)] {
                    let (color, ids, gpu, scene) = shot_with(source.clone(), rotate, ssao);
                    let (want_color, want_ids, want_gpu, _) =
                        shot_with(baked.clone(), rotate, ssao);
                    // the fitted box is exact over turned instances, as over their baked twins
                    let ends = |b: &AABB| [b.min_point(), b.max_point()];

                    for (got, want) in ends(&gpu.bounds).iter().zip(ends(&want_gpu.bounds)) {
                        assert!((0..3).all(|k| (got[k] - want[k]).abs() < 1e-3), "fit box");
                    }

                    assert_eq!(scene.instancing.batch_rows(), batches);
                    assert_eq!(gpu.arena.source_faces.slots.instances(), instances);
                    let drawn = ids.iter().filter(|id| id.is_some()).count();
                    let same = ids.iter().zip(&want_ids).filter(|(a, b)| a == b).count();
                    let pairs = color.chunks_exact(4).zip(want_color.chunks_exact(4));
                    let apart = |levels: u8| {
                        pairs
                            .clone()
                            .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > levels))
                            .count()
                    };
                    let (differ, far) = (apart(2), apart(8));

                    if let Some(dir) = std::env::var_os("INSTANCING_SHOTS") {
                        let name = format!("{}_{rotate}_{ssao}", source.name);
                        write_ppm(&dir, &format!("{name}_instanced"), &color);
                        write_ppm(&dir, &format!("{name}_baked"), &want_color);
                    }
                    eprintln!(
                        "{} rotate {rotate} ssao {ssao}: {drawn} object pixels, {} id mismatches, {} exact color mismatches, {differ} by more than 2 levels, {far} by more than 8",
                        source.name,
                        ids.len() - same,
                        apart(0)
                    );
                    assert!(drawn > 1000, "the instances are drawn");
                    // turned instances are placed in f32 on the GPU: a few silhouette pixels flip
                    assert!(
                        (ids.len() - same) * 200 <= drawn,
                        "each pixel picks the same instance"
                    );
                    // a turned normal shades a level or two apart, and f32 placement on the GPU
                    // against f64 baking flips a rare edge pixel
                    let pixels = color.len() / 4;
                    assert!(differ * 2_000 <= drawn, "{differ} pixels differ in color");
                    assert!(far * 20_000 <= pixels, "{far} pixels differ visibly");
                }
            }
        }

        /// Deleting, moving, exploding and undoing turned instances on a live GPU: a delete writes
        /// only its own slots, and after every edit the frame and its ids match the same objects
        /// loaded fresh.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn instance_edits_write_their_own_slots() {
            let (mut gpu, mut scene, camera) = loaded(turned_grid(), true, true);
            let beam = |name: &str| {
                rows(&scene)
                    .into_iter()
                    .find(|(guid, _)| scene.docs[0].session.instance_lookup[guid].name == name)
                    .unwrap()
            };
            let (deleted, moved, exploded) = (beam("beam_2").1, beam("beam_5").1, beam("beam_7").0);
            let mut written = Vec::new();

            for step in 0..6 {
                match step {
                    0 => assert!(scene.delete_row(deleted)),
                    1 => {
                        let turn = Xform::rotation_z(25.0, true);
                        assert!(scene.transform_rows(&[moved], &turn, "Move").is_some());
                    }
                    2 => {
                        let session = Rc::make_mut(&mut scene.docs[0].session);
                        session.begin("Explode");
                        assert!(session.explode(&exploded));
                        let notes = super::super::super::sync::commit(session);
                        scene.noted(0, notes);
                        scene.edited(&[0]);
                    }
                    _ => assert!(scene.undo()),
                }

                gpu.arena.space.written = 0;
                scene.sync();
                scene.upload_to(&mut gpu);
                written.push(gpu.arena.space.written);
                let (color, ids) = frame(&mut gpu, &scene, &camera);
                let fresh = (*scene.docs[0].session).clone();
                let (mut want_gpu, want_scene, _) = loaded(fresh, true, true);
                let (want, want_ids) = frame(&mut want_gpu, &want_scene, &camera);
                let drawn = ids.iter().filter(|id| id.is_some()).count();
                let same = ids.iter().zip(&want_ids).filter(|(a, b)| a == b).count();
                let differ = color
                    .chunks_exact(4)
                    .zip(want.chunks_exact(4))
                    .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > 2))
                    .count();
                eprintln!(
                    "step {step}: {} slots written, {drawn} object pixels, {} ids and {differ} colors apart",
                    written[step],
                    ids.len() - same
                );
                assert!(drawn > 1000);
                // a leaver's slot takes the last member: where two beams meet, a pixel may flip
                assert!((ids.len() - same) * 1000 <= drawn, "step {step}: ids");
                assert!(differ * 1000 <= drawn, "step {step}: colors");
            }

            // head, the slot the last beam moved into, the slot it left
            assert!(written[0] <= 3, "a delete writes {}", written[0]);
            assert_eq!(written[1], 0, "a move writes no slot");
        }

        /// Walls standing on a floor, one of them turned a quarter: with ambient occlusion the
        /// instanced frame matches the baked one, the turned wall included, since each instance
        /// triangle names its own row, matrix and contact radius.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn ambient_occlusion_of_turned_instances() {
            let mut s = Session::new("walls");
            let wall = s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(8.0, 0.4, 3.0))));
            let floor =
                s.add_definition(Geometry::Mesh(Rc::new(Mesh::create_box(30.0, 30.0, 0.2))));
            let turn = Xform::rotation_z(90.0, true);
            let places = [
                (&wall, Xform::translation(0.0, 0.0, 1.6)),
                (&wall, &Xform::translation(6.0, 4.0, 1.6) * &turn),
                (&floor, Xform::identity()),
            ];

            for (definition, place) in places {
                s.add_instance(InstanceRef::new(definition, Xform::identity()), place, None);
            }

            let mut baked = Session::new("baked");

            for mesh in &s.get_geometry().meshes {
                baked.add_mesh((**mesh).clone(), None);
            }

            let (color, _, _, _) = shot_with(s, true, true);
            let (want, _, _, _) = shot_with(baked, true, true);
            let far = color
                .chunks_exact(4)
                .zip(want.chunks_exact(4))
                .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > 24))
                .count();

            if let Some(dir) = std::env::var_os("INSTANCING_SHOTS") {
                write_ppm(&dir, "walls_instanced", &color);
                write_ppm(&dir, "walls_baked", &want);
            }

            eprintln!("ambient occlusion: {far} pixels differ by more than 24 levels");
            assert!(far * 20_000 <= 400 * 300, "{far} pixels differ");
        }

        /// INSTANCING_PAIR=<instanced.pb>,<baked.pb>: pixels of the two files' frames apart by more
        /// than 2 and 8 levels, with ambient occlusion, and how far their fitted boxes are apart.
        #[test]
        #[ignore = "measurement: INSTANCING_PAIR=<instanced.pb>,<baked.pb>"]
        fn instanced_file_against_its_baked_twin() {
            let pair = std::env::var("INSTANCING_PAIR").expect("INSTANCING_PAIR=<a.pb>,<b.pb>");
            let load = |path: &str| Session::pb_loads(&std::fs::read(path).unwrap()).unwrap();
            let (on, off) = pair.split_once(',').expect("two paths");

            for ssao in [false, true] {
                let (color, _, gpu, _) = shot_with(load(on), true, ssao);
                let (want, _, want_gpu, _) = shot_with(load(off), true, ssao);
                let pairs = color.chunks_exact(4).zip(want.chunks_exact(4));
                let apart = |levels: u8| {
                    pairs
                        .clone()
                        .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > levels))
                        .count()
                };
                let (a, b) = (&gpu.bounds, &want_gpu.bounds);
                let fit = [a.min_point(), a.max_point()]
                    .iter()
                    .zip([b.min_point(), b.max_point()])
                    .flat_map(|(p, q)| (0..3).map(move |k| (p[k] - q[k]).abs()))
                    .fold(0.0, f64::max);
                println!(
                    "ssao {ssao}: {} of {} pixels apart by more than 2 levels, {} by more than 8; fit boxes {fit:.6} apart ({} and {})",
                    apart(2),
                    color.len() / 4,
                    apart(8),
                    a.str(),
                    b.str()
                );
            }
        }

        /// Median wall time of an offscreen 1920x1080 frame of INSTANCING_BENCH (a .pb), orbiting,
        /// with and without ambient occlusion; frames are read back, so the GPU work is inside.
        #[test]
        #[ignore = "benchmark: INSTANCING_BENCH=<scene.pb>, run with buildslot --exclusive"]
        fn bench_frames() {
            let path = std::env::var("INSTANCING_BENCH").expect("INSTANCING_BENCH protobuf path");
            let source = Session::pb_loads(&std::fs::read(&path).unwrap()).unwrap();
            let mut gpu =
                pollster::block_on(Gpu::new_headless(1920, 1080)).expect("a native adapter");
            gpu.view.msaa_forced = Some(4);
            gpu.resize(1920, 1080);
            gpu.view.show_grid = false;
            gpu.view.opacity = 0.7;
            let mut scene = Scene::new();
            scene.add_file(file(source));
            scene.upload_to(&mut gpu);
            let mut camera = Camera::new();
            camera.fit(&gpu.bounds, 1920.0 / 1080.0);

            for ssao in [false, true] {
                gpu.view.set_arctic(ssao);
                let mut times = Vec::new();

                for i in 0..64 {
                    camera.orbit(0.05, 0.0);
                    let anchor = gpu
                        .rebase_anchor(&camera.origin(), camera.distance_world(), 0.0)
                        .anchor;
                    let input = FrameInput {
                        view_proj: camera.view_proj_anchored(1920.0 / 1080.0, &anchor),
                        clear: wgpu::Color::WHITE,
                        now_ms: 0.0,
                    };
                    let start = std::time::Instant::now();
                    gpu.render_offscreen(&input);

                    // the first frames compile pipelines
                    if i >= 8 {
                        times.push(start.elapsed().as_secs_f64() * 1000.0);
                    }
                }

                times.sort_by(f64::total_cmp);
                println!(
                    "ssao {ssao}: median {:.2} ms, p90 {:.2} ms, {} draws",
                    times[times.len() / 2],
                    times[times.len() * 9 / 10],
                    gpu.performance.draws
                );
            }
        }
    }
}
