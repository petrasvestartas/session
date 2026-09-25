use super::super::rows::{FREE, Footprint, GEOMETRY, SINK};
use super::super::{Scene, placement};
use super::{Work, baked};
use crate::app::walk::{Row, Walk, WalkCx, is_drawable, walk_features, walk_geometry};
use crate::engine::gpu::glyphs::GlyphPoint;
use crate::engine::gpu::instanced::{Draw, Slot};
use crate::engine::gpu::patch::Counts;
use crate::engine::gpu::{Gpu, Instance, ObjectRow, Upload};
use session_rust::{AABB, Geometry, InstanceRef, RenderVertex, Session, Xform};
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
}

/// One definition of one document: walked once under a hidden row, drawn once per instance.
struct Batch {
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
}

impl Instancing {
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

        if let Some(members) = batch
            .and_then(|b| self.batches[b].as_mut())
            .map(|b| &mut b.members)
        {
            self.member_at.insert(row, members.len());
            members.push(row);
            self.dirty = true;
        }
    }

    /// Row `row` draws no instance any more, in O(1); its batch and the members it has left.
    fn leave(&mut self, row: u32) -> Option<(usize, usize)> {
        let index = self.of_row.remove(&row)??;
        let at = self.member_at.remove(&row)?;
        let members = &mut self.batches[index].as_mut()?.members;
        members.swap_remove(at);

        // the last member moved into the gap
        if let Some(&moved) = members.get(at) {
            self.member_at.insert(moved, at);
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
            let (foot, walked) = self.draw_instance(doc, row, instance, definition, &at);
            self.feet[row as usize] = foot;
            let at = (row - self.object_rows) as usize;
            take_walk(&mut self.tables.obj.rows[at], &walked);
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
    ) -> (Footprint, Row) {
        let batch = self.batch_for(doc, &instance.definition_guid, definition, place);
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
        self.instancing.join(row, batch);

        if self.attributes {
            walk_features(
                &mut Walk::of(&mut up),
                &cx,
                &instance.features,
                &mut walked.bounds,
            );
        }

        let cloud = matches!(definition, Geometry::PointCloud(_));
        (self.append(up, cloud), walked)
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
        let spheres = std::mem::take(&mut up.glyph.spheres);
        let dots = std::mem::take(&mut up.glyph.dots);

        // sheet fills, lettering and registered lanes have no instanced draw
        if counts.print + counts.text > 0 || counts.lanes.iter().any(|rows| *rows > 0) {
            self.free_hidden_row(row);
            return None;
        }

        self.feet[row as usize] = self.append(up, false);
        // hidden and dead: never drawn, boxed or cut; ambient occlusion reads its box and, for the
        // instances turned like its first, its matrix
        let mut object = ObjectRow::new(place.clone(), Instance::FLAG_HIDDEN | Instance::FLAG_DEAD);
        object.bounds = walk.bounds;
        self.set_object_row(row, object);
        let batch = Batch {
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

    /// A definition's shared rows walked under hidden row `row`, then one empty triangle on the
    /// sink: the id a turned instance reports, so readers of triangle ids find no plane, no
    /// matrix of another turn and no contact radius.
    fn shared_walk(&mut self, row: u32, definition: &Geometry) -> (Upload, Row) {
        let mut up = Upload::default();
        let cx = WalkCx {
            vert_base: 0,
            cloud_px: 0.0,
            row,
            attributes: self.attributes,
        };
        let walk = walk_geometry(&mut Walk::of(&mut up), &cx, definition);
        let sink = self.sink_row();
        let vertex = up.arena.verts.len() as u32;
        up.arena.verts.push(RenderVertex {
            position: [0.0; 3],
            normal: [0.0; 3],
            color: [0.0; 4],
        });
        up.arena.vids.push(sink);
        up.arena.idx.extend_from_slice(&[vertex; 3]);
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
        let (foot, walked) = self.draw_instance(doc, row, instance, definition, &place);
        self.feet[i] = foot;
        let hidden = if self.hidden.contains(&(doc, Rc::clone(&item.guid))) {
            Instance::FLAG_HIDDEN
        } else {
            0
        };
        let mut object = self.object_row(doc, &item.guid, place, hidden);
        take_walk(&mut object, &walked);

        if row >= self.object_rows {
            let world = object.bounds.transformed(&object.place);

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
        let (foot, walked) = self.draw_instance(doc, row, instance, definition, &place);
        self.feet[i] = foot;
        let mut object = ObjectRow::new(place, 0);
        take_walk(&mut object, &walked);
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
            let walked = Walked {
                bounds: walk.bounds,
                spacing: walk.spacing,
                flags: walk.flags,
                faces: walk.faces,
                spheres: std::mem::take(&mut up.glyph.spheres),
                dots: std::mem::take(&mut up.glyph.dots),
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
            let mut up = Upload::default();
            let cx = WalkCx {
                vert_base: 0,
                cloud_px: self.docs[doc].point_px,
                row,
                attributes: self.attributes,
            };
            let mut walked = match batch.and_then(|index| self.instancing.batches[index].as_ref()) {
                Some(batch) => {
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

            if self.attributes {
                walk_features(
                    &mut Walk::of(&mut up),
                    &cx,
                    &instance.features,
                    &mut walked.bounds,
                );
            }

            self.feet[row as usize] = self.append(up, false);
            let place = self.placement_of(row).unwrap_or_else(Xform::identity);
            let mut object = ObjectRow::new(place, 0);
            take_walk(&mut object, &walked);
            self.staged.geometry.push((row, object));
        }
    }

    /// The instance slots and one draw per batch, from where the shared rows sit now.
    pub(crate) fn instanced_draws(&self) -> (Vec<Slot>, Vec<Draw>) {
        let mut rows = Vec::new();
        let mut draws = Vec::new();

        for batch in self.instancing.live() {
            if batch.members.is_empty() {
                continue;
            }

            let span = self.spans.span(self.feet[batch.row as usize]);
            let first = 1 + rows.len() as u32; // slot 0 keeps plain rows their own
            let (s, c) = (span.start, span.count);
            // the empty triangle is the last of the shared ones
            let empty = (s.faces + c.faces) / 3;
            rows.extend(batch.members.iter().map(|&row| [row, empty]));
            draws.push(Draw {
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

        let (rows, draws) = self.instanced_draws();
        gpu.set_instanced(&rows, &draws);
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

/// Box, spacing, flags and faces of a walk onto an object row.
fn take_walk(object: &mut ObjectRow, walked: &Row) {
    object.flags |= walked.flags;
    object.bounds = walked.bounds;
    object.spacing = walked.spacing;
    object.faces = walked.faces;

    if walked.faces {
        object.flags |= Instance::FLAG_HAS_FACES;
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::FileDoc;
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
        assert_eq!(scene.shape(row), Some(super::super::super::Shape::Element));
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
        let notes = super::super::commit(session);
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
        let notes = super::super::commit(session);
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

        /// Colors and object ids of one frame of `session`, fitted to its box.
        fn shot(session: Session, rotate: bool) -> (Vec<u8>, Vec<Option<String>>, Gpu, Scene) {
            shot_with(session, rotate, false)
        }

        /// `shot`, with ambient occlusion when `ssao`.
        fn shot_with(
            session: Session,
            rotate: bool,
            ssao: bool,
        ) -> (Vec<u8>, Vec<Option<String>>, Gpu, Scene) {
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
            (color, ids, gpu, scene)
        }

        /// Instances drawn from one upload look and pick exactly like the same objects baked:
        /// every pixel's object is the instance's guid.
        #[test]
        #[ignore = "requires a native GPU adapter"]
        fn instanced_frames_match_the_baked_scene() {
            let (source, _) = placed(3);
            let mut baked = Session::new("baked");

            for element in &source.get_geometry().elements {
                baked.add_element((**element).clone(), None);
            }

            for rotate in [false, true] {
                let (color, ids, gpu, scene) = shot(source.clone(), rotate);
                let (want_color, want_ids, _, _) = shot(baked.clone(), rotate);
                assert_eq!(scene.instancing.batch_rows(), 1);
                assert_eq!(gpu.arena.source_faces.slots.instances(), 3);
                let drawn = ids.iter().filter(|id| id.is_some()).count();
                let same = ids.iter().zip(&want_ids).filter(|(a, b)| a == b).count();
                let differ = color
                    .chunks_exact(4)
                    .zip(want_color.chunks_exact(4))
                    .filter(|(a, b)| a != b)
                    .count();
                eprintln!(
                    "rotate {rotate}: {drawn} object pixels, {} id mismatches, {differ} color mismatches",
                    ids.len() - same
                );
                assert!(drawn > 1000, "the instances are drawn");
                assert_eq!(same, ids.len(), "each pixel picks the same instance");
                // faces, edges and features alike: only a few edge pixels differ, tested against depth alone
                assert!(
                    differ * 500 < color.len() / 4,
                    "{differ} pixels differ in color"
                );
            }
        }

        /// Walls standing on a floor, one of them turned a quarter: with ambient occlusion the
        /// instanced frame stays close to the baked one, the turned wall never darker; its
        /// triangles carry no shared id, so it takes depth normals and no contact radius.
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
                for (name, pixels) in [("instanced", &color), ("baked", &want)] {
                    let mut ppm = b"P6 400 300 255\n".to_vec();
                    ppm.extend(pixels.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]));
                    std::fs::write(std::path::Path::new(&dir).join(format!("{name}.ppm")), ppm)
                        .unwrap();
                }
            }

            // what differs are edge pixels: until instanced triangles join the tile lists, ink tests the fitted planes
            eprintln!("ambient occlusion: {far} pixels differ by more than 24 levels");
            assert!(far < 400 * 300 / 100, "{far} pixels differ");
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
