use super::Scene;
use crate::app::walk::display::{
    OBJECT_BYTES, Quality, brep_source_bytes, refinement_bytes, source_bounds, source_bytes,
};
use crate::app::walk::display_features;
use crate::camera::{Camera, FOVY_DEG, NEAR_FRACTION};
use crate::engine::gpu::patch::Counts;
use crate::engine::gpu::{Gpu, Instance};
use session_rust::element::ElementFeature;
use session_rust::{AABB, Geometry, Xform};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

const SOURCE_BYTES: usize = 64 * 1024 * 1024;
const REFINEMENT_BYTES: u64 = 64 * 1024 * 1024;
const TRANSIENT_BYTES: usize = 32 * 1024 * 1024;
const SCAN_ROWS: usize = 128;
const QUALITY_ATTEMPTS: usize = 4;

fn retry_qualities(requested: Quality, current: Quality) -> impl Iterator<Item = Quality> {
    (0..QUALITY_ATTEMPTS)
        .map(move |step| Quality {
            tolerance: requested.tolerance * 4.0_f64.powi(step as i32),
        })
        .take_while(move |quality| {
            quality.tolerance.is_finite()
                && quality.tolerance > 0.0
                && quality.tolerance < current.tolerance
        })
}

fn initial_brep_chord(geometry: &Geometry, current: Quality) -> Option<f64> {
    if current.tolerance.is_finite() {
        return None;
    }
    let brep = match geometry {
        Geometry::BRep(brep) => brep.as_ref(),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => brep,
            _ => return None,
        },
        _ => return None,
    };
    Some(crate::app::walk::display::brep_diagonal(brep) * crate::app::walk::brep::QUALITY.1)
}

#[derive(Default)]
pub(super) struct Display {
    quality: HashMap<u32, Quality>,
    sources: HashMap<u32, (Geometry, usize)>,
    instance_features: HashMap<u32, (Rc<[ElementFeature]>, usize)>,
    source_bytes: usize,
    allocated: u64,
    signature: u64,
    cursor: usize,
    remaining: usize,
    limited: HashMap<u32, u64>,
    limit_reasons: HashMap<u32, &'static str>,
}

impl Display {
    pub(super) fn quality(&self, row: u32) -> Quality {
        self.quality.get(&row).copied().unwrap_or_default()
    }

    pub(super) fn forget(&mut self, row: u32) {
        self.quality.remove(&row);
        self.limited.remove(&row);
        self.limit_reasons.remove(&row);
        if let Some((_, bytes)) = self.instance_features.remove(&row) {
            self.source_bytes -= bytes;
        }
        if let Some((_, bytes)) = self.sources.remove(&row) {
            self.source_bytes -= bytes;
        }
        self.signature = 0;
    }

    pub(super) fn reset_allocations(&mut self) {
        self.quality.clear();
        self.limited.clear();
        self.limit_reasons.clear();
        self.allocated = 0;
        self.signature = 0;
    }

    fn limit(&mut self, row: u32, requested: Option<Quality>, reason: &'static str) {
        self.limited
            .insert(row, requested.map_or(u64::MAX, |q| q.tolerance.to_bits()));
        self.limit_reasons.insert(row, reason);
    }

    fn accepted(&mut self, row: u32, actual: Quality, requested: Quality, bytes: u64) {
        self.quality.insert(row, actual);
        self.allocated += bytes;
        if actual.tolerance > requested.tolerance {
            self.limit(row, Some(requested), "requested quality not admitted");
        } else {
            self.limited.remove(&row);
            self.limit_reasons.remove(&row);
        }
    }

    fn retain_instance_features(&mut self, row: u32, features: &[ElementFeature]) {
        if self.instance_features.contains_key(&row) {
            return;
        }
        let Some(cost) = display_features::cost(features) else {
            return;
        };
        let Some(bytes) = cost.source.checked_add(128) else {
            return;
        };
        if bytes > SOURCE_BYTES.saturating_sub(self.source_bytes) {
            return;
        }
        let Some(snapshot) = display_features::snapshot(features) else {
            return;
        };
        self.instance_features
            .insert(row, (Rc::from(snapshot), bytes));
        self.source_bytes += bytes;
    }

    fn retain(&mut self, row: u32, geometry: &Geometry) {
        if let Geometry::Element(element) = geometry {
            if self.sources.contains_key(&row) {
                return;
            }
            let session_rust::element::ElementGeometry::BRep(brep) = element.geometry() else {
                return;
            };
            let Some(features) = display_features::cost(element.features()) else {
                return;
            };
            let Some(bytes) = brep_source_bytes(brep)
                .and_then(|bytes| bytes.checked_add(features.source))
                .and_then(|bytes| bytes.checked_add(element.name.len()))
                .and_then(|bytes| {
                    bytes.checked_add(if element.has_guid() {
                        element.guid().len()
                    } else {
                        0
                    })
                })
                .and_then(|bytes| bytes.checked_add(2048))
            else {
                return;
            };
            if bytes > SOURCE_BYTES.saturating_sub(self.source_bytes) || bytes > TRANSIENT_BYTES {
                return;
            }
            let Some(features) = display_features::snapshot(element.features()) else {
                return;
            };
            let mut snapshot = session_rust::Element::from_brep(brep.clone(), &element.name);
            snapshot.set_features(features);
            if element.has_guid() {
                snapshot.set_guid(element.guid().to_owned());
            }
            snapshot.is_visible = element.is_visible;
            snapshot.is_locked = element.is_locked;
            self.sources
                .insert(row, (Geometry::Element(std::rc::Rc::new(snapshot)), bytes));
            self.source_bytes += bytes;
            return;
        }
        let Some(bytes) = source_bytes(geometry) else {
            return;
        };
        if self.sources.contains_key(&row)
            || bytes > SOURCE_BYTES.saturating_sub(self.source_bytes)
            || bytes > TRANSIENT_BYTES
        {
            return;
        }
        let snapshot = match geometry {
            Geometry::NurbsCurve(curve) => {
                Geometry::NurbsCurve(std::rc::Rc::new((**curve).clone()))
            }
            Geometry::NurbsSurface(surface) => {
                Geometry::NurbsSurface(std::rc::Rc::new((**surface).clone()))
            }
            Geometry::BRep(brep) => Geometry::BRep(std::rc::Rc::new((**brep).clone())),
            _ => return,
        };
        self.sources.insert(row, (snapshot, bytes));
        self.source_bytes += bytes;
    }
}

impl Scene {
    pub(super) fn retain_display_sources(&mut self, doc: usize, first: u32) {
        let session = std::rc::Rc::clone(&self.docs[doc].session);
        for row in first as usize..self.order.len() {
            if self.owners[row] != doc {
                continue;
            }
            let guid = self.order[row].as_ref();
            let geometry = session.lookup.get(guid).or_else(|| {
                let reference = session.instance_lookup.get(guid)?;
                session.definition_lookup.get(&reference.definition_guid)
            });
            if let Some(geometry) = geometry {
                let target = self.instance_batch_row(row as u32).unwrap_or(row as u32);
                self.display.retain(target, geometry);
                if target == row as u32 && self.display.sources.contains_key(&target) {
                    if let Some(reference) = session.instance_lookup.get(guid) {
                        self.display
                            .retain_instance_features(row as u32, &reference.features);
                    }
                }
            }
        }
    }

    pub(super) fn forget_display_sources(&mut self, doc: usize) {
        for row in 0..self.order.len() {
            if self.owners[row] == doc {
                if let Some(batch) = self.instance_batch_row(row as u32) {
                    self.display.forget(batch);
                }
                self.display.forget(row as u32);
            }
        }
    }

    fn display_source(&self, row: u32) -> Option<Geometry> {
        if let Some(geometry) = self.shape_of(row) {
            return Some(geometry.clone());
        }
        // A live edit or undo must never fall back to a released revision.
        self.released_doc(row)?;
        let row = self.instance_batch_row(row).unwrap_or(row);
        self.display
            .sources
            .get(&row)
            .map(|(geometry, _)| geometry.clone())
    }

    fn display_instance_features(&self, row: u32) -> Option<Rc<[ElementFeature]>> {
        if !self.instancing.is_instance(row) || self.instance_batch_row(row).is_some() {
            return Some(Rc::from([]));
        }
        let (doc, guid) = self.identity_of(row)?;
        if let Some(reference) = self.docs[doc].session.instance_lookup.get(guid.as_ref()) {
            return display_features::snapshot(&reference.features).map(Rc::from);
        }
        // Match the retained definition only while this exact document revision is released.
        self.released_doc(row)?;
        self.display
            .instance_features
            .get(&row)
            .map(|(features, _)| Rc::clone(features))
    }

    /// One bounded idle refresh. A declined row keeps its current buffers and source identity.
    pub(crate) fn refine_display(&mut self, gpu: &mut Gpu, camera: &Camera) -> (bool, bool) {
        if self.preview.is_some() || self.has_pending() || self.loaded {
            return (false, false);
        }
        let signature = view_signature(camera, gpu, self.row_revision);
        if signature != self.display.signature {
            self.display.signature = signature;
            self.display.remaining = self.order.len();
            self.display.cursor = 0;
        }
        let size = [gpu.config.width, gpu.config.height];
        for _ in 0..SCAN_ROWS.min(self.display.remaining) {
            let row = self.display.cursor as u32;
            self.display.cursor += 1;
            self.display.remaining -= 1;
            let Some((doc, guid)) = self.identity_of(row) else {
                continue;
            };
            if !matches!(
                self.shape(row),
                Some(
                    super::Shape::BRep
                        | super::Shape::Element
                        | super::Shape::Curve
                        | super::Shape::Surface
                )
            ) {
                continue;
            }
            if gpu
                .objects
                .row(row)
                .is_none_or(|object| object.flags & Instance::FLAG_HIDDEN != 0)
            {
                continue;
            }
            let (Some(mut bounds), Some(place)) =
                (gpu.objects.row_bounds(row), self.placement_of(row))
            else {
                continue;
            };
            let target = self.instance_batch_row(row).unwrap_or(row);
            let Some(geometry) = self.display_source(row) else {
                self.display.limit(target, None, "source unavailable");
                continue;
            };
            // Admission precedes control-hull traversal and any candidate tessellation.
            let Some(geometry_bytes) = refinement_bytes(&geometry, Quality::default()) else {
                self.display.limit(target, None, "source admission");
                continue;
            };
            let Some(features) = self.display_instance_features(row) else {
                self.display
                    .limit(target, None, "instance features unavailable");
                continue;
            };
            let estimate = display_features::cost(&features)
                .and_then(|cost| geometry_bytes.checked_add(cost.transient));
            if estimate.is_none_or(|bytes| bytes > TRANSIENT_BYTES) {
                self.display.limit(target, None, "transient admission");
                continue;
            }
            let Some(source_box) = source_bounds(&geometry) else {
                self.display.limit(target, None, "invalid source hull");
                continue;
            };
            bounds.union_with(&source_box.transformed(&place));
            let Some(error) = local_error(camera, size, &bounds, &place) else {
                continue;
            };
            let old = self.display.quality(target);
            let requested = Quality::for_error(error, old);
            // Keep a finer existing display when zooming out; no allocation churn.
            if requested.tolerance >= old.tolerance {
                continue;
            }
            if self.display.limited.get(&target) == Some(&requested.tolerance.to_bits()) {
                continue;
            }
            if self.display.allocated >= REFINEMENT_BYTES {
                self.display
                    .limit(target, Some(requested), "scene allocation budget");
                continue;
            }
            let mut accepted = None;
            let initial_chord = initial_brep_chord(&geometry, old);
            let mut limit_reason = "candidate or row budget";
            for quality in retry_qualities(requested, old) {
                if initial_chord
                    .is_some_and(|chord| !chord.is_finite() || quality.tolerance > chord)
                {
                    limit_reason = "initial BRep chord limit";
                    break;
                }
                self.display.quality.insert(target, quality);
                let bytes =
                    if target != row {
                        self.refine_instance_display(
                            row,
                            &geometry,
                            REFINEMENT_BYTES - self.display.allocated,
                            gpu,
                        )
                    } else {
                        crate::app::walk::display::candidate(|| {
                            self.walk_one_with_features(doc, row, &geometry, &place, &features)
                        })
                        .and_then(|(up, walked)| {
                            let bytes = Counts::of(&up).bytes();
                            if bytes == 0
                                || (gpu.objects.row(row).is_some_and(|object| {
                                    object.flags & Instance::FLAG_HAS_FACES != 0
                                }) && !walked.faces)
                                || bytes > OBJECT_BYTES as u64
                                || bytes > REFINEMENT_BYTES - self.display.allocated
                            {
                                None
                            } else {
                                self.replace_walk(row, doc, &guid, &geometry, up, walked, false);
                                Some(bytes)
                            }
                        })
                    };
                if let Some(bytes) = bytes {
                    accepted = Some((quality, bytes));
                    break;
                }
            }
            let changed = accepted.is_some();
            if let Some((actual, bytes)) = accepted {
                // Charge the full replacement, including allocations later retired by another zoom.
                self.display.accepted(target, actual, requested, bytes);
                self.upload_to(gpu);
                self.display.signature = view_signature(camera, gpu, self.row_revision);
            } else {
                if old.tolerance.is_finite() {
                    self.display.quality.insert(target, old);
                } else {
                    self.display.quality.remove(&target);
                }
                self.display.limit(target, Some(requested), limit_reason);
            }
            return (self.display.remaining > 0, changed);
        }
        (self.display.remaining > 0, false)
    }

    pub(crate) fn display_counters(&self) -> (usize, u64, usize, usize) {
        (
            self.display.source_bytes,
            self.display.allocated,
            self.display.quality.len(),
            self.display.limited.len(),
        )
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn display_quality_sample(
        &self,
        limit: usize,
    ) -> Vec<(u32, Option<f64>, Option<f64>, &'static str)> {
        (0..self.order.len() as u32)
            .filter_map(|row| {
                let actual = self.display.quality.get(&row).map(|q| q.tolerance);
                let requested = self.display.limited.get(&row).copied();
                if actual.is_none() && requested.is_none() {
                    return None;
                }
                Some((
                    row,
                    actual,
                    requested
                        .filter(|bits| *bits != u64::MAX)
                        .map(f64::from_bits),
                    self.display
                        .limit_reasons
                        .get(&row)
                        .copied()
                        .unwrap_or("accepted"),
                ))
            })
            .take(limit)
            .collect()
    }
}

fn view_signature(camera: &Camera, gpu: &Gpu, revision: u64) -> u64 {
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    for value in camera
        .position
        .into_iter()
        .chain(camera.target)
        .chain(camera.up)
        .chain(camera.frame)
        .chain([camera.distance, camera.unit.to_meters()])
    {
        value.to_bits().hash(&mut hash);
    }
    camera.perspective.hash(&mut hash);
    (
        gpu.config.width,
        gpu.config.height,
        gpu.objects.geometry_revision(),
        revision,
    )
        .hash(&mut hash);
    hash.finish()
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x * y).sum()
}
fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn unit(a: [f64; 3]) -> Option<[f64; 3]> {
    let length = dot(a, a).sqrt();
    (length.is_finite() && length > 0.0).then(|| a.map(|v| v / length))
}

/// Conservative half-device-pixel tolerance, including off-axis perspective and affine scaling.
fn local_error(camera: &Camera, size: [u32; 2], bounds: &AABB, place: &Xform) -> Option<f64> {
    if size.contains(&0) || !bounds.is_valid() {
        return None;
    }
    let scale = camera.unit.to_meters();
    let forward = unit(std::array::from_fn(|i| {
        camera.target[i] - camera.position[i]
    }))?;
    let right = unit(cross(forward, camera.up))?;
    let up = cross(right, forward);
    let mut zmin = f64::INFINITY;
    let mut zmax = f64::NEG_INFINITY;
    let mut radius: f64 = 0.0;
    let mut outside = [true; 4];
    let tangent = (FOVY_DEG * 0.5).to_radians().tan();
    let aspect = size[0] as f64 / size[1] as f64;
    let shift = camera.shift();
    for point in bounds.corners() {
        let p = [
            point[0] * scale - camera.position[0],
            point[1] * scale - camera.position[1],
            point[2] * scale - camera.position[2],
        ];
        let (x, y, z) = (dot(p, right), dot(p, up), dot(p, forward));
        if ![x, y, z].iter().all(|v| v.is_finite()) {
            return None;
        }
        zmin = zmin.min(z);
        zmax = zmax.max(z);
        radius = radius.max(x.hypot(y));
        let h = if camera.perspective {
            z * tangent
        } else {
            camera.distance * tangent
        };
        outside[0] &= x < (-1.0 - shift[0]) * h * aspect;
        outside[1] &= x > (1.0 - shift[0]) * h * aspect;
        outside[2] &= y < (-1.0 - shift[1]) * h;
        outside[3] &= y > (1.0 - shift[1]) * h;
    }
    let near = camera.distance * NEAR_FRACTION;
    if (camera.perspective && zmax <= near) || outside.into_iter().any(|v| v) {
        return None;
    }
    let depth = if camera.perspective {
        zmin.max(near)
    } else {
        camera.distance
    };
    let perspective = if camera.perspective {
        1.0 + radius / depth
    } else {
        1.0
    };
    // sqrt(||A||1 ||A||inf) bounds the largest singular value, including shear and reflection.
    if place.m[3] != 0.0 || place.m[7] != 0.0 || place.m[11] != 0.0 || place.m[15] != 1.0 {
        return None;
    }
    let one = (0..3)
        .map(|c| (0..3).map(|r| place.m[4 * c + r].abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let inf = (0..3)
        .map(|r| (0..3).map(|c| place.m[4 * c + r].abs()).sum::<f64>())
        .fold(0.0, f64::max);
    let error = depth * tangent / (size[1] as f64 * perspective * scale * (one * inf).sqrt());
    (error.is_finite() && error > 0.0).then_some(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::{NurbsCurve, Point, Session};

    #[test]
    fn retries_are_finite_and_never_coarser_than_the_current_display() {
        let requested = Quality { tolerance: 0.25 };
        let attempts: Vec<f64> = retry_qualities(requested, Quality::default())
            .map(|quality| quality.tolerance)
            .collect();
        assert_eq!(attempts, [0.25, 1.0, 4.0, 16.0]);
        let current = Quality { tolerance: 4.0 };
        assert_eq!(retry_qualities(requested, current).count(), 2);
        assert!(retry_qualities(current, current).next().is_none());
        assert!(
            retry_qualities(Quality::default(), Quality::default())
                .next()
                .is_none()
        );
    }

    #[test]
    fn initial_brep_fallback_stays_within_the_existing_absolute_chord() {
        let brep = session_rust::BRep::create_box(2.0, 3.0, 4.0);
        let expected =
            crate::app::walk::display::brep_diagonal(&brep) * crate::app::walk::brep::QUALITY.1;
        let geometry = Geometry::BRep(Rc::new(brep));
        let ceiling = initial_brep_chord(&geometry, Quality::default()).unwrap();
        assert_eq!(ceiling, expected);
        let requested = Quality {
            tolerance: ceiling / 4.0,
        };
        let attempts: Vec<_> = retry_qualities(requested, Quality::default())
            .take_while(|quality| quality.tolerance <= ceiling)
            .map(|quality| quality.tolerance)
            .collect();
        assert_eq!(attempts, [ceiling / 4.0, ceiling]);
        assert!(initial_brep_chord(&geometry, requested).is_none());
        let curve = Geometry::NurbsCurve(Rc::new(NurbsCurve::default()));
        assert!(initial_brep_chord(&curve, Quality::default()).is_none());
    }

    #[test]
    fn element_initial_chord_uses_its_brep_not_feature_bounds() {
        let mut element =
            session_rust::Element::from_brep(session_rust::BRep::create_box(2.0, 3.0, 4.0), "box");
        let plain = Geometry::Element(Rc::new(element.clone()));
        let mut feature = display_feature("contact", false);
        feature.outlines[0].coords[0] = 1e6;
        element.set_features(vec![feature]);
        let featured = Geometry::Element(Rc::new(element));
        assert_eq!(
            initial_brep_chord(&featured, Quality::default()),
            initial_brep_chord(&plain, Quality::default())
        );
    }

    #[test]
    fn accepted_intermediate_quality_keeps_unmet_target_and_charges_retired_allocations() {
        let mut display = Display::default();
        let requested = Quality { tolerance: 0.25 };
        let fallback = Quality { tolerance: 1.0 };
        display.accepted(3, fallback, requested, 120);
        assert_eq!(display.quality(3), fallback);
        assert_eq!(
            display.limited.get(&3),
            Some(&requested.tolerance.to_bits())
        );
        assert_eq!(display.allocated, 120);
        // A later successful finer request replaces, but does not refund, the old buffers.
        display.accepted(3, requested, requested, 240);
        assert_eq!(display.quality(3), requested);
        assert!(!display.limited.contains_key(&3));
        assert!(!display.limit_reasons.contains_key(&3));
        assert_eq!(display.allocated, 360);
        display.forget(3);
        assert_eq!(display.allocated, 360);
        display.reset_allocations();
        assert_eq!(display.allocated, 0);
    }
    use std::rc::Rc;

    fn curve() -> NurbsCurve {
        NurbsCurve::create(
            false,
            2,
            &[
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 2.0, 0.0),
                Point::new(2.0, 0.0, 0.0),
            ],
        )
    }

    #[test]
    fn source_cap_and_forget_bound_retained_revisions() {
        let geometry = Geometry::NurbsCurve(Rc::new(curve()));
        let mut display = Display::default();
        display.retain(1, &geometry);
        let bytes = display.source_bytes;
        assert!(bytes > 0);
        display.retain(1, &geometry);
        assert_eq!(display.source_bytes, bytes);
        display.source_bytes = SOURCE_BYTES;
        display.retain(2, &geometry);
        assert!(!display.sources.contains_key(&2));
        display.source_bytes = bytes;
        display.forget(1);
        assert_eq!(display.source_bytes, 0);
        assert!(display.sources.is_empty());
    }

    #[test]
    fn released_element_snapshot_preserves_visible_brep_without_private_state() {
        let mut element =
            session_rust::Element::from_brep(session_rust::BRep::create_box(2.0, 3.0, 4.0), "box");
        element.set_guid("element-source".into());
        element.element_data = vec![7; 4096];
        element.is_locked = true;
        let mut display = Display::default();
        display.retain(3, &Geometry::Element(Rc::new(element)));
        let (Geometry::Element(snapshot), _) = &display.sources[&3] else {
            panic!("missing Element snapshot")
        };
        assert_eq!(snapshot.guid(), "element-source");
        assert_eq!(snapshot.name, "box");
        assert!(snapshot.is_locked);
        assert!(snapshot.element_data.is_empty());
        let session_rust::element::ElementGeometry::BRep(brep) = snapshot.geometry() else {
            panic!("lost BRep")
        };
        assert_eq!(brep.m_faces.len(), 6);
        assert!(snapshot.features.is_empty());
    }

    #[test]
    fn released_source_never_fetches_and_hydrated_revision_wins() {
        let mut session = Session::new("curves");
        let node = session.add_nurbscurve(curve(), None).unwrap();
        let guid = node.borrow().name.clone();
        let mut scene = Scene::new();
        scene.add_file(super::super::FileDoc {
            name: "curves".into(),
            place: Xform::identity(),
            session: Rc::new(session),
            point_px: 0.0,
            display_only: true,
        });
        let row = scene.row_of(0, &guid).unwrap();
        let original = scene.display_source(row).unwrap();
        scene.release(0, 0, "immutable-source.json".into());
        let retained = scene.display_source(row).unwrap();
        if let (Geometry::NurbsCurve(a), Geometry::NurbsCurve(b)) = (&original, &retained) {
            assert!(!Rc::ptr_eq(a, b));
            assert_eq!(a.m_cv, b.m_cv);
            assert_eq!(a.guid(), b.guid());
        } else {
            panic!("curve source changed type");
        }
        assert!(scene.take_wanted().is_empty());
        let token = scene.released[&0].token;
        let mut changed = curve();
        changed.set_guid(guid.clone());
        let mut session = Session::new("new revision");
        session.add_nurbscurve(changed, None).unwrap();
        scene.hydrate(0, token, session).unwrap();
        assert!(scene.display.sources.is_empty());
        if let (Geometry::NurbsCurve(a), Geometry::NurbsCurve(b)) =
            (original, scene.display_source(row).unwrap())
        {
            assert!(!Rc::ptr_eq(&a, &b));
        } else {
            panic!("hydrated curve changed type");
        }
        scene.reset_rows();
        assert!(scene.display.sources.is_empty());
        assert_eq!(scene.display.allocated, 0);
    }

    #[test]
    fn physical_pixels_placement_and_off_axis_depth_bound_error() {
        let mut camera = Camera::new();
        camera.position = [0.0, 0.0, 10.0];
        camera.target = [0.0; 3];
        camera.up = [0.0, 1.0, 0.0];
        camera.distance = 10.0;
        camera.unit = crate::camera::Unit::Meters;
        let bounds = AABB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let error = local_error(&camera, [1000, 1000], &bounds, &Xform::identity()).unwrap();
        let dpi = local_error(&camera, [2000, 2000], &bounds, &Xform::identity()).unwrap();
        assert_eq!(dpi, error / 2.0);
        let scaled = local_error(
            &camera,
            [1000, 1000],
            &bounds,
            &Xform::scale_xyz(-2.0, 1.0, 1.0),
        )
        .unwrap();
        assert_eq!(scaled, error / 2.0);
        let off = AABB::new(2.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        assert!(local_error(&camera, [1000, 1000], &off, &Xform::identity()).unwrap() < error);
        let behind = AABB::new(0.0, 0.0, 20.0, 1.0, 1.0, 1.0);
        assert!(local_error(&camera, [1000, 1000], &behind, &Xform::identity()).is_none());
        camera.perspective = false;
        assert!(local_error(&camera, [1000, 1000], &behind, &Xform::identity()).is_some());
    }
    fn display_feature(kind: &str, point: bool) -> ElementFeature {
        let mut outline = session_rust::Polyline::default();
        outline.coords = if point {
            vec![2., 3., 4.]
        } else {
            vec![0., 0., 0., 1., 0., 0., 1., 1., 0., 0., 1., 0., 0., 0., 0.]
        };
        outline.plane.name = "private cached plane".repeat(1000);
        let mut feature = ElementFeature::new(kind, 3, vec![outline], "visible feature");
        feature.set_guid(format!("{kind}-feature"));
        feature
    }

    #[test]
    fn featured_element_snapshot_keeps_brep_order_and_whole_feature_set() {
        let mut element = session_rust::Element::from_brep(
            session_rust::BRep::create_box(2., 3., 4.),
            "featured box",
        );
        element.set_guid("featured-element".into());
        element.element_data = vec![19; 4096];
        element.set_features(vec![
            display_feature("contact", false),
            display_feature("joint", true),
        ]);
        let original = Geometry::Element(Rc::new(element));
        let mut display = Display::default();
        display.retain(7, &original);
        let (Geometry::Element(snapshot), bytes) = &display.sources[&7] else {
            panic!("missing featured Element snapshot")
        };
        assert_eq!(snapshot.guid(), "featured-element");
        assert_eq!(snapshot.features().len(), 2);
        assert_eq!(snapshot.features()[0].guid(), "contact-feature");
        assert_eq!(snapshot.features()[1].guid(), "joint-feature");
        assert!(snapshot.element_data.is_empty());
        assert!(*bytes <= display.source_bytes);
        let geometry = Geometry::Element(Rc::clone(snapshot));
        assert!(refinement_bytes(&geometry, Quality { tolerance: 0.01 }).is_some());
        let mut session = Session::new("feature roundtrip");
        let Geometry::Element(element) = &original else {
            unreachable!()
        };
        session.add_element((**element).clone(), None);
        let mut scene = Scene::new();
        scene.interactions = true;
        scene.add_file(super::super::FileDoc {
            name: "features".into(),
            place: Xform::identity(),
            session: Rc::new(session),
            point_px: 0.,
            display_only: true,
        });
        let row = scene.row_of(0, "featured-element").unwrap();
        let a = scene.walk_one(0, row, &original, &Xform::identity()).0;
        let b = scene.walk_one(0, row, &geometry, &Xform::identity()).0;
        assert_eq!(a.arena.idx, b.arena.idx);
        assert_eq!(a.arena.face_ids, b.arena.face_ids);
        assert_eq!(a.seg.pipe_ids, b.seg.pipe_ids);
        assert_eq!(
            a.arena
                .face_sources
                .iter()
                .map(|s| (s.parent, s.face))
                .collect::<Vec<_>>(),
            b.arena
                .face_sources
                .iter()
                .map(|s| (s.parent, s.face))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&a.glyph.dots),
            bytemuck::cast_slice::<_, u8>(&b.glyph.dots)
        );
        display.forget(7);
        assert_eq!(display.source_bytes, 0);
    }

    #[test]
    fn released_nonshared_instance_replays_own_features_without_fetch() {
        let mut session = Session::new("featured curve instance");
        let mut definition_curve = curve();
        definition_curve.set_guid("feature-definition".into());
        let definition = session.add_definition(Geometry::NurbsCurve(Rc::new(definition_curve)));
        let mut instance = session_rust::InstanceRef::new(&definition, Xform::identity());
        instance.set_guid("featured-instance".into());
        instance.features = vec![
            display_feature("contact", false),
            display_feature("joint", true),
        ];
        let place = Xform::translation(5., 7., 9.);
        session.add_instance(instance, place.clone(), None);
        let mut scene = Scene::new();
        scene.interactions = true;
        scene.add_file(super::super::FileDoc {
            name: "instances".into(),
            place: Xform::identity(),
            session: Rc::new(session),
            point_px: 0.,
            display_only: true,
        });
        let row = scene.row_of(0, "featured-instance").unwrap();
        assert!(scene.instancing.is_instance(row));
        assert!(scene.instance_batch_row(row).is_none());
        let geometry = scene.display_source(row).unwrap();
        let features = scene.display_instance_features(row).unwrap();
        scene
            .display
            .quality
            .insert(row, Quality { tolerance: 0.01 });
        let a = scene
            .walk_one_with_features(0, row, &geometry, &place, &features)
            .0;
        scene.release(0, 0, "immutable-feature-instance.json".into());
        let retained = scene.display_source(row).unwrap();
        let features = scene.display_instance_features(row).unwrap();
        let b = scene
            .walk_one_with_features(0, row, &retained, &place, &features)
            .0;
        assert_eq!(features.len(), 2);
        assert_eq!(a.arena.idx, b.arena.idx);
        assert_eq!(a.arena.face_ids, b.arena.face_ids);
        assert_eq!(a.seg.ribbon_ids, b.seg.ribbon_ids);
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&a.seg.ribbons),
            bytemuck::cast_slice::<_, u8>(&b.seg.ribbons)
        );
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&a.glyph.dots),
            bytemuck::cast_slice::<_, u8>(&b.glyph.dots)
        );
        assert!(b.arena.face_sources.iter().all(|s| s.parent == row));
        assert!(!b.glyph.dots.is_empty());
        assert!(b.glyph.dots.iter().all(|s| s.instance_id == row));
        assert!(!b.arena.idx.is_empty());
        assert!(scene.take_wanted().is_empty());
        assert_eq!(scene.placement_of(row).unwrap().m, place.m);
        let token = scene.released[&0].token;
        let mut next = Session::new("edited instance");
        let mut definition_curve = curve();
        definition_curve.set_guid("feature-definition".into());
        let definition = next.add_definition(Geometry::NurbsCurve(Rc::new(definition_curve)));
        let mut edited = session_rust::InstanceRef::new(&definition, Xform::identity());
        edited.set_guid("featured-instance".into());
        edited.features = vec![display_feature("axis", true)];
        next.add_instance(edited, place.clone(), None);
        scene.hydrate(0, token, next).unwrap();
        assert!(scene.display.instance_features.is_empty());
        let current = scene.display_instance_features(row).unwrap();
        assert_eq!(current.len(), 1);
        assert_eq!(current[0].guid(), "axis-feature");
        // A live edit wins over any retained revision; forgetting cannot hide live sources.
        assert!(scene.display_source(row).is_some());
        scene.release(0, 0, "edited-feature-instance.json".into());
        scene.forget_display_sources(0);
        assert!(scene.display.instance_features.is_empty());
        assert!(scene.display_source(row).is_none());
        assert!(scene.display_instance_features(row).is_none());
        assert_eq!(scene.display.source_bytes, 0);
    }

    #[test]
    fn instance_feature_cap_never_becomes_an_empty_replay() {
        let mut display = Display::default();
        display.source_bytes = SOURCE_BYTES;
        display.retain_instance_features(9, &[display_feature("contact", false)]);
        assert!(!display.instance_features.contains_key(&9));
        assert_eq!(display.source_bytes, SOURCE_BYTES);
        display.source_bytes = 0;
        display.retain_instance_features(9, &[display_feature("contact", false)]);
        assert!(display.instance_features.contains_key(&9));
        display.forget(9);
        assert_eq!(display.source_bytes, 0);
    }
}
