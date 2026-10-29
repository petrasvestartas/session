use session_rust::{Polyline, element::ElementFeature};

const FEATURES: usize = 256;
const OUTLINES: usize = 256;
const POINTS: usize = 4096;
const CONTACT_POINTS: usize = 64;
const CONTACT_WORK: usize = 65_536;
const SOURCE_BYTES: usize = 512 * 1024;

pub struct Cost {
    pub source: usize,
    pub transient: usize,
}

// Counts the fresh display snapshot, never a Polyline's cached plane or spare capacity.
pub fn cost(features: &[ElementFeature]) -> Option<Cost> {
    if features.len() > FEATURES {
        return None;
    }
    let mut bytes = features
        .len()
        .checked_mul(std::mem::size_of::<ElementFeature>())?;
    let (mut outlines, mut points, mut contacts) = (0usize, 0usize, 0usize);
    for feature in features {
        bytes = bytes
            .checked_add(feature.name.len())?
            .checked_add(feature.feature_type.len())?
            .checked_add(if feature.has_guid() {
                feature.guid().len()
            } else {
                0
            })?;
        outlines = outlines.checked_add(feature.outlines.len())?;
        if outlines > OUTLINES {
            return None;
        }
        for outline in &feature.outlines {
            let n = outline.coords.len() / 3;
            if outline.coords.len() % 3 != 0 || n > POINTS {
                return None;
            }
            points = points.checked_add(n)?;
            if points > POINTS || !outline.coords.iter().all(|x| x.is_finite()) {
                return None;
            }
            if feature.feature_type == "contact" {
                if n > CONTACT_POINTS {
                    return None;
                }
                contacts = contacts.checked_add(n.checked_mul(n)?)?;
                if contacts > CONTACT_WORK {
                    return None;
                }
            }
            bytes = bytes
                .checked_add(std::mem::size_of::<Polyline>())?
                .checked_add(outline.coords.len().checked_mul(8)?)?
                .checked_add(outline.name.len())?
                .checked_add(if outline.has_guid() {
                    outline.guid().len()
                } else {
                    0
                })?
                .checked_add(outline.linecolor.name.len())?
                .checked_add(if outline.linecolor.has_guid() {
                    outline.linecolor.guid().len()
                } else {
                    0
                })?
                .checked_add(2048)?;
            if bytes > SOURCE_BYTES {
                return None;
            }
        }
    }
    if bytes > SOURCE_BYTES {
        return None;
    }
    // Includes simultaneous snapshots, line/glyph output, and contact CDT plus render copies.
    let transient = bytes
        .checked_mul(4)?
        .checked_add(points.checked_mul(2048)?)?
        .checked_add(contacts.checked_mul(128)?)?;
    Some(Cost {
        source: bytes,
        transient,
    })
}

pub fn snapshot(features: &[ElementFeature]) -> Option<Vec<ElementFeature>> {
    cost(features)?;
    Some(
        features
            .iter()
            .map(|feature| {
                let outlines = feature
                    .outlines
                    .iter()
                    .map(|outline| {
                        let mut copy = Polyline::default();
                        copy.coords = outline.coords.clone();
                        copy.name = outline.name.clone();
                        copy.is_visible = outline.is_visible;
                        copy.is_locked = outline.is_locked;
                        copy.width = outline.width;
                        copy.arrowhead = outline.arrowhead;
                        copy.linecolor = outline.linecolor.clone();
                        if outline.has_guid() {
                            copy.set_guid(outline.guid().to_owned());
                        }
                        copy
                    })
                    .collect();
                let mut copy = ElementFeature::new(
                    &feature.feature_type,
                    feature.face_index,
                    outlines,
                    &feature.name,
                );
                copy.visible = feature.visible;
                if feature.has_guid() {
                    copy.set_guid(feature.guid().to_owned());
                }
                copy
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::walk::{Walk, WalkCx, display::Quality, walk_features};
    use crate::engine::gpu::Upload;

    fn feature() -> ElementFeature {
        let mut outline = Polyline::default();
        outline.coords = vec![0., 0., 0., 1., 0., 0., 1., 1., 0., 0., 1., 0., 0., 0., 0.];
        outline.set_guid("outline".into());
        outline.linecolor.set_guid("color".into());
        let mut feature = ElementFeature::new("contact", 4, vec![outline], "contact");
        feature.set_guid("feature".into());
        feature
    }

    #[test]
    fn snapshot_preserves_feature_identity_without_retaining_plane_or_dash() {
        let mut feature = feature();
        feature.outlines[0].plane.name = "cached".repeat(200_000);
        feature.outlines[0].dash = vec![1.; 200_000];
        let source = [feature];
        let bytes = cost(&source).unwrap().source;
        assert!(bytes < 16 * 1024);
        let copy = snapshot(&source).unwrap();
        assert_eq!(copy[0].guid(), "feature");
        assert_eq!(copy[0].face_index, 4);
        assert_eq!(copy[0].outlines[0].guid(), "outline");
        assert_eq!(copy[0].outlines[0].linecolor.guid(), "color");
        assert!(copy[0].outlines[0].dash.is_empty());
        assert!(copy[0].outlines[0].plane.name.len() < 100);
        assert_eq!(cost(&copy).unwrap().source, bytes);
    }

    #[test]
    fn snapshot_walk_keeps_contact_pick_parent_and_glyphs() {
        let mut dot = Polyline::default();
        dot.coords = vec![2., 3., 4.];
        let mut hidden = feature();
        hidden.visible = false;
        let source = [
            feature(),
            ElementFeature::new("joint", 2, vec![dot], "dot"),
            hidden,
        ];
        let copy = snapshot(&source).unwrap();
        let walk = |features: &[ElementFeature], interactions| {
            let mut up = Upload::default();
            let cx = WalkCx {
                quality: Quality::default(),
                vert_base: 0,
                cloud_px: 0.,
                row: 37,
                interactions,
                attributes: true,
            };
            walk_features(
                &mut Walk::of(&mut up),
                &cx,
                features,
                &mut session_rust::AABB::empty(),
            );
            up
        };
        let a = walk(&source, true);
        let b = walk(&copy, true);
        assert_eq!(a.arena.idx, b.arena.idx);
        assert_eq!(a.arena.face_ids, b.arena.face_ids);
        assert_eq!(a.seg.ribbon_ids, b.seg.ribbon_ids);
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&a.arena.verts),
            bytemuck::cast_slice::<_, u8>(&b.arena.verts)
        );
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&a.seg.ribbons),
            bytemuck::cast_slice::<_, u8>(&b.seg.ribbons)
        );
        assert_eq!(
            bytemuck::cast_slice::<_, u8>(&a.glyph.dots),
            bytemuck::cast_slice::<_, u8>(&b.glyph.dots)
        );
        assert!(!b.arena.face_sources.is_empty());
        assert!(
            b.arena
                .face_sources
                .iter()
                .all(|s| s.parent == 37 && s.face == 0)
        );
        assert!(!b.glyph.dots.is_empty());
        assert!(b.glyph.dots.iter().all(|s| s.instance_id == 37));
        let off = walk(&copy, false);
        assert!(
            off.arena.idx.is_empty() && off.seg.ribbons.is_empty() && off.glyph.dots.is_empty()
        );
    }

    #[test]
    fn unsupported_feature_rejects_the_whole_finite_candidate() {
        let mut oversized = feature();
        oversized.outlines[0].coords = vec![0.; (CONTACT_POINTS + 1) * 3];
        assert!(snapshot(&[feature(), oversized.clone()]).is_none());
        let result = crate::app::walk::display::candidate(|| {
            let mut up = Upload::default();
            let cx = WalkCx {
                quality: Quality { tolerance: 0.01 },
                vert_base: 0,
                cloud_px: 0.,
                row: 1,
                interactions: true,
                attributes: true,
            };
            walk_features(
                &mut Walk::of(&mut up),
                &cx,
                &[feature(), oversized],
                &mut session_rust::AABB::empty(),
            );
            up
        });
        assert!(result.is_none());
    }
}
