use session_rust::{BRep, Geometry, NurbsCurve, NurbsSurface};

pub const CURVE_SEGMENTS: usize = 4096;
pub const SURFACE_STEPS: usize = 256;
pub const OBJECT_BYTES: u64 = 16 * 1024 * 1024;

const MAX_ORDER: usize = 16;

fn curve_admitted(curve: &NurbsCurve) -> bool {
    (2..=MAX_ORDER).contains(&curve.m_order)
        && curve.m_cv_count >= curve.m_order
        && (0..curve.m_cv_count).all(|i| {
            curve.get_cv_4d(i).is_some_and(|(x, y, z, w)| {
                [x, y, z, w].into_iter().all(f64::is_finite) && w > 1e-14
            })
        })
}

fn surface_admitted(surface: &NurbsSurface) -> bool {
    (0..2).all(|axis| {
        (2..=MAX_ORDER).contains(&surface.m_order[axis])
            && surface.m_cv_count[axis] >= surface.m_order[axis]
    }) && (0..surface.cv_count(0)).all(|i| {
        (0..surface.cv_count(1)).all(|j| {
            surface.get_cv_4d(i, j).is_some_and(|(x, y, z, w)| {
                [x, y, z, w].into_iter().all(f64::is_finite) && w > 1e-14
            })
        })
    })
}

fn control(curve: &NurbsCurve, index: usize) -> Option<[f64; 3]> {
    let (x, y, z, w) = curve.get_cv_4d(index)?;
    let point = [x / w, y / w, z / w];
    (w.is_finite() && w > 0.0 && point.into_iter().all(f64::is_finite)).then_some(point)
}

fn color_bytes(color: &session_rust::Color) -> usize {
    color
        .name
        .capacity()
        .saturating_add(256)
        .saturating_add(if color.has_guid() {
            color.guid().len()
        } else {
            0
        })
}

fn curve_bytes(curve: &NurbsCurve) -> Option<usize> {
    if !curve_admitted(curve) {
        return None;
    }
    let colors = curve
        .pointcolors
        .capacity()
        .checked_add(curve.linecolors.capacity())?
        .checked_mul(std::mem::size_of::<session_rust::Color>())?
        .checked_add(
            curve
                .pointcolors
                .iter()
                .chain(&curve.linecolors)
                .map(color_bytes)
                .sum::<usize>(),
        )?;
    curve
        .m_cv
        .capacity()
        .checked_add(curve.m_nurbsknot.capacity())?
        .checked_mul(8)?
        .checked_add(colors)?
        .checked_add(curve.name.capacity())?
        .checked_add(if curve.has_guid() {
            curve.guid().len()
        } else {
            0
        })?
        .checked_add(std::mem::size_of::<NurbsCurve>() + 64)
}

fn surface_bytes(surface: &NurbsSurface) -> Option<usize> {
    // Cached physical meshes have private caches of their own; never retain them to refine a picture.
    if surface.m_mesh.is_some() || !surface_admitted(surface) {
        return None;
    }
    let colors = surface
        .pointcolors
        .capacity()
        .checked_add(surface.linecolors.capacity())?
        .checked_add(surface.facecolors.capacity())?
        .checked_mul(std::mem::size_of::<session_rust::Color>())?
        .checked_add(
            surface
                .pointcolors
                .iter()
                .chain(&surface.linecolors)
                .chain(&surface.facecolors)
                .map(color_bytes)
                .sum::<usize>(),
        )?;
    surface
        .m_cv
        .capacity()
        .checked_add(surface.m_nurbsknot[0].capacity())?
        .checked_add(surface.m_nurbsknot[1].capacity())?
        .checked_mul(8)?
        .checked_add(colors)?
        .checked_add(surface.name.capacity())?
        .checked_add(if surface.has_guid() {
            surface.guid().len()
        } else {
            0
        })?
        .checked_add(std::mem::size_of::<NurbsSurface>() + 64)
}

pub fn brep_source_bytes(brep: &BRep) -> Option<usize> {
    let mut bytes = brep.name.capacity().checked_add(4096)?;
    bytes = bytes
        .checked_add(if brep.has_guid() {
            brep.guid().len()
        } else {
            0
        })?
        .checked_add(color_bytes(&brep.surfacecolor))?;
    bytes = bytes
        .checked_add(
            brep.m_surfaces
                .capacity()
                .saturating_sub(brep.m_surfaces.len())
                .checked_mul(std::mem::size_of::<NurbsSurface>())?,
        )?
        .checked_add(
            brep.m_curves_3d
                .capacity()
                .saturating_sub(brep.m_curves_3d.len())
                .checked_add(
                    brep.m_curves_2d
                        .capacity()
                        .saturating_sub(brep.m_curves_2d.len()),
                )?
                .checked_mul(std::mem::size_of::<NurbsCurve>())?,
        )?;
    for surface in &brep.m_surfaces {
        bytes = bytes.checked_add(surface_bytes(surface)?)?;
    }
    for curve in brep.m_curves_3d.iter().chain(&brep.m_curves_2d) {
        bytes = bytes.checked_add(curve_bytes(curve)?)?;
    }
    fn table_bytes<T>(table: &Vec<T>) -> Option<usize> {
        table
            .capacity()
            .checked_mul(std::mem::size_of::<T>().checked_add(64)?)
    }
    // Nested buffers and metadata are charged below, separately from each table's stored fields.
    for table in [
        table_bytes(&brep.m_vertices)?,
        table_bytes(&brep.m_edges)?,
        table_bytes(&brep.m_wires)?,
        table_bytes(&brep.m_faces)?,
        table_bytes(&brep.m_shells)?,
        table_bytes(&brep.m_solids)?,
    ] {
        bytes = bytes.checked_add(table)?;
    }
    for vertex in &brep.m_vertices {
        let point = &vertex.point;
        bytes = bytes
            .checked_add(point.name.capacity())?
            .checked_add(if point.has_guid() {
                point.guid().len()
            } else {
                0
            })?
            .checked_add(color_bytes(&point.pointcolor))?;
    }
    for edge in &brep.m_edges {
        bytes = bytes.checked_add(edge.pcurves.capacity().checked_mul(64)?)?;
    }
    for wire in &brep.m_wires {
        bytes = bytes.checked_add(wire.edges.capacity().checked_mul(32)?)?;
    }
    for face in &brep.m_faces {
        bytes = bytes.checked_add(face.wires.capacity().checked_mul(32)?)?;
        if let Some(color) = &face.facecolor {
            bytes = bytes.checked_add(color_bytes(color))?;
        }
    }
    for shell in &brep.m_shells {
        bytes = bytes.checked_add(shell.faces.capacity().checked_mul(32)?)?;
    }
    for solid in &brep.m_solids {
        bytes = bytes.checked_add(solid.shells.capacity().checked_mul(32)?)?;
    }
    Some(bytes)
}

/// Admission accounting for retained exact sources; unknown private mesh caches decline retention.
pub fn source_bytes(geometry: &Geometry) -> Option<usize> {
    match geometry {
        Geometry::NurbsCurve(curve) => curve_bytes(curve),
        Geometry::NurbsSurface(surface) => surface_bytes(surface),
        Geometry::BRep(brep) => brep_source_bytes(brep),
        _ => None,
    }
}

pub fn source_bounds(geometry: &Geometry) -> Option<session_rust::AABB> {
    fn curve(bounds: &mut session_rust::AABB, curve: &NurbsCurve) -> Option<()> {
        if !curve_admitted(curve) {
            return None;
        }
        for i in 0..curve.m_cv_count {
            let (_, _, _, weight) = curve.get_cv_4d(i)?;
            let p = control(curve, i)?;
            if !weight.is_finite() || weight <= 0.0 || !(0..3).all(|k| p[k].is_finite()) {
                return None;
            }
            bounds.union_with_point(p[0], p[1], p[2]);
        }
        Some(())
    }
    fn surface(bounds: &mut session_rust::AABB, surface: &NurbsSurface) -> Option<()> {
        if !surface_admitted(surface) {
            return None;
        }
        for i in 0..surface.cv_count(0) {
            for j in 0..surface.cv_count(1) {
                let (x, y, z, weight) = surface.get_cv_4d(i, j)?;
                let p = [x / weight, y / weight, z / weight];
                if !weight.is_finite() || weight <= 0.0 || !(0..3).all(|k| p[k].is_finite()) {
                    return None;
                }
                bounds.union_with_point(p[0], p[1], p[2]);
            }
        }
        Some(())
    }
    fn brep(bounds: &mut session_rust::AABB, brep: &BRep) -> Option<()> {
        for item in &brep.m_surfaces {
            surface(bounds, item)?;
        }
        for item in &brep.m_curves_3d {
            curve(bounds, item)?;
        }
        Some(())
    }
    let mut bounds = session_rust::AABB::empty();
    match geometry {
        Geometry::NurbsCurve(item) => curve(&mut bounds, item)?,
        Geometry::NurbsSurface(item) => surface(&mut bounds, item)?,
        Geometry::BRep(item) => brep(&mut bounds, item)?,
        Geometry::Element(item) => match item.geometry() {
            session_rust::element::ElementGeometry::BRep(item) => brep(&mut bounds, item)?,
            _ => return None,
        },
        _ => return None,
    }
    bounds.is_valid().then_some(bounds)
}

pub const BREP_WORK: usize = 16_384;
pub const OBJECT_CURVE_SEGMENTS: usize = 32_768;

fn brep_refinement_bytes(brep: &BRep) -> Option<usize> {
    let source = brep_source_bytes(brep)?;
    if source > 512 * 1024 || brep.m_faces.len() > 128 || brep.m_edges.len() > 512 {
        return None;
    }
    (BREP_WORK * 1024 + OBJECT_CURVE_SEGMENTS * 128).checked_add(source.checked_mul(20)?)
}

thread_local! {
    static FAILED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ACTIVE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn candidate_active() -> bool {
    ACTIVE.get()
}

pub fn failed() {
    if ACTIVE.get() {
        FAILED.set(true);
    }
}

pub fn candidate<T>(walk: impl FnOnce() -> T) -> Option<T> {
    struct Restore(bool, bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            FAILED.set(self.0);
            ACTIVE.set(self.1);
        }
    }
    let _restore = Restore(FAILED.replace(false), ACTIVE.replace(true));
    let result = walk();
    (!FAILED.get()).then_some(result)
}

/// Conservative preallocation admission for supported display families. None retains the existing LOD.
pub fn refinement_bytes(geometry: &Geometry, _quality: Quality) -> Option<usize> {
    match geometry {
        Geometry::NurbsCurve(curve) => curve_bytes(curve)?
            .checked_mul(20)?
            .checked_add(CURVE_SEGMENTS * 128),
        Geometry::NurbsSurface(surface) => surface_bytes(surface)?
            .checked_mul(20)?
            .checked_add((SURFACE_STEPS + 1).pow(2) * 256),
        Geometry::BRep(brep) => brep_refinement_bytes(brep),
        Geometry::Element(element) => match element.geometry() {
            session_rust::element::ElementGeometry::BRep(brep) => brep_refinement_bytes(brep)?
                .checked_add(super::display_features::cost(element.features())?.transient),
            _ => None,
        },
        _ => None,
    }
}

pub fn brep_diagonal(brep: &session_rust::BRep) -> f64 {
    let mut bounds = session_rust::AABB::empty();
    for surface in &brep.m_surfaces {
        if !surface_admitted(surface) {
            return 0.0;
        }
        for i in 0..surface.cv_count(0) {
            for j in 0..surface.cv_count(1) {
                if let Some(p) = surface.get_cv(i, j) {
                    bounds.union_with_point(p[0], p[1], p[2]);
                }
            }
        }
    }
    bounds.diagonal()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quality {
    pub tolerance: f64,
}

impl Default for Quality {
    fn default() -> Self {
        Self {
            tolerance: f64::INFINITY,
        }
    }
}

impl Quality {
    /// Quantized half-pixel error in local units, with a dead band on zoom-out.
    pub fn for_error(error: f64, previous: Self) -> Self {
        if !error.is_finite() || error <= 0.0 {
            return previous;
        }
        if previous.tolerance <= error && error < previous.tolerance * 2.0 {
            return previous;
        }
        let exponent = (error.log2() * 2.0).floor().clamp(-80.0, 80.0);
        Self {
            tolerance: 2.0_f64.powf(exponent / 2.0),
        }
    }

    pub fn mesh(self, diagonal: f64) -> (f64, f64) {
        if !self.tolerance.is_finite() || diagonal <= 0.0 {
            return super::brep::QUALITY;
        }
        if self.tolerance / diagonal < 1e-6 {
            failed();
        }
        // The kernel also bounds subdivisions. Reaching either limit is not a pixel-error guarantee.
        // Chord error follows the camera, including cheaper retries; normal turn is a safety guard.
        (20.0, (self.tolerance / diagonal).max(1e-6))
    }
}

pub struct Samples {
    pub parameters: Vec<f64>,
    pub limited: bool,
}

fn point(curve: &NurbsCurve, t: f64) -> [f64; 3] {
    let p = curve.point_at(t);
    [p[0], p[1], p[2]]
}

fn distance(p: [f64; 3], a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = std::array::from_fn::<_, 3, _>(|i| b[i] - a[i]);
    let q = std::array::from_fn::<_, 3, _>(|i| p[i] - a[i]);
    let length = d.iter().map(|v| v * v).sum::<f64>();
    let t = ((0..3).map(|i| q[i] * d[i]).sum::<f64>() / length.max(1e-300)).clamp(0.0, 1.0);
    (0..3)
        .map(|i| (q[i] - t * d[i]).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Positive-weight control polygons bound the whole curve, including inflections and loops.
fn flat(curve: &NurbsCurve, a: [f64; 3], b: [f64; 3], tolerance: f64) -> bool {
    if !a.into_iter().chain(b).all(f64::is_finite) {
        return false;
    }
    let mut previous: Option<[f64; 3]> = None;
    for i in 0..curve.m_cv_count {
        if curve
            .get_cv_4d(i)
            .is_none_or(|cv| !cv.3.is_finite() || cv.3 <= 0.0)
        {
            return false;
        }
        let Some(p) = control(curve, i) else {
            return false;
        };
        let deviation = distance([p[0], p[1], p[2]], a, b);
        if ![p[0], p[1], p[2], deviation]
            .into_iter()
            .all(f64::is_finite)
            || deviation > tolerance
        {
            return false;
        }
        if i > 0 {
            let Some(q) = control(curve, i - 1) else {
                return false;
            };
            let d = [p[0] - q[0], p[1] - q[1], p[2] - q[2]];
            let length = d.iter().map(|v| v * v).sum::<f64>().sqrt();
            if length > 1e-14 {
                let unit = d.map(|v| v / length);
                if let Some(before) = previous
                    && (0..3).map(|k| before[k] * unit[k]).sum::<f64>() < 5.0_f64.to_radians().cos()
                {
                    return false;
                }
                previous = Some(unit);
            }
        }
    }
    true
}

/// Exact subcurves are split at knots first; one finite stack and one output replace the previous LOD.
pub fn sample(curve: &NurbsCurve, quality: Quality, budget: usize) -> Samples {
    let budget = budget.clamp(1, CURVE_SEGMENTS);
    let (start, end) = curve.domain();
    if !curve_admitted(curve) {
        failed();
        let parameters = if !quality.tolerance.is_finite() && curve.is_valid() {
            let count = ((super::curves::turning_degrees(curve) / 5.0).ceil() as usize)
                .max(curve.span_count())
                .clamp(1, budget);
            (0..=count)
                .map(|i| start + (end - start) * i as f64 / count as f64)
                .collect()
        } else {
            Vec::new()
        };
        return Samples {
            parameters,
            limited: true,
        };
    }
    if curve.m_cv_count < 2 || !start.is_finite() || !end.is_finite() || start >= end {
        return Samples {
            parameters: Vec::new(),
            limited: false,
        };
    }
    let tolerance = if quality.tolerance.is_finite() {
        quality.tolerance
    } else {
        let a = point(curve, start);
        (0..curve.m_cv_count)
            .filter_map(|i| curve.get_cv(i))
            .map(|p| (0..3).map(|k| (p[k] - a[k]).powi(2)).sum::<f64>().sqrt())
            .fold(0.0, f64::max)
            * 0.001
    }
    .max(1e-14);
    if flat(curve, point(curve, start), point(curve, end), tolerance) {
        return Samples {
            parameters: vec![start, end],
            limited: false,
        };
    }
    let spans = curve.get_span_vector();
    let mut limited = spans.len().saturating_sub(1) > budget;
    let mut parameters = vec![start];
    let mut stack = Vec::new();
    if limited {
        failed();
        for i in 1..=budget {
            parameters.push(start + (end - start) * i as f64 / budget as f64);
        }
        return Samples {
            parameters,
            limited,
        };
    }
    for (index, span) in spans.windows(2).enumerate() {
        let remaining_spans = spans.len() - index - 2;
        let mut piece = curve.clone();
        if piece.trim(span[0], span[1]) {
            stack.push((piece, span[0], span[1], 0));
        } else {
            limited = true;
            parameters.push(span[1]);
            continue;
        }
        while let Some((piece, a, b, depth)) = stack.pop() {
            if flat(&piece, point(curve, a), point(curve, b), tolerance) {
                parameters.push(b);
                continue;
            }
            if depth >= 16 || parameters.len() + stack.len() + remaining_spans >= budget {
                limited = true;
                parameters.push(b);
                continue;
            }
            let mid = a + (b - a) * 0.5;
            let (left, right) = piece.split(mid);
            let epsilon = (end - start).abs() * 1e-10;
            if (left.domain().0 - a).abs() > epsilon
                || (left.domain().1 - mid).abs() > epsilon
                || (right.domain().0 - mid).abs() > epsilon
                || (right.domain().1 - b).abs() > epsilon
            {
                limited = true;
                parameters.push(b);
                continue;
            }
            stack.push((right, mid, b, depth + 1));
            stack.push((left, a, mid, depth + 1));
        }
    }
    if parameters.last() != Some(&end) {
        parameters.push(end);
        limited = true;
    }
    if limited {
        failed();
    }
    Samples {
        parameters,
        limited,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Point;

    fn inflection() -> NurbsCurve {
        NurbsCurve::create(
            false,
            3,
            &[
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 3.0, 0.0),
                Point::new(2.0, -3.0, 0.0),
                Point::new(3.0, 0.0, 0.0),
            ],
        )
    }

    #[test]
    fn inflections_are_bounded_even_when_the_midpoint_is_on_the_chord() {
        let curve = inflection();
        let coarse = sample(&curve, Quality { tolerance: 0.02 }, CURVE_SEGMENTS);
        let fine = sample(&curve, Quality { tolerance: 0.0002 }, CURVE_SEGMENTS);
        assert!(!coarse.limited && !fine.limited);
        assert!(coarse.parameters.len() > 2);
        assert!(fine.parameters.len() > coarse.parameters.len());
        for pair in fine.parameters.windows(2) {
            let a = point(&curve, pair[0]);
            let b = point(&curve, pair[1]);
            for k in 1..16 {
                let t = pair[0] + (pair[1] - pair[0]) * k as f64 / 16.0;
                assert!(distance(point(&curve, t), a, b) <= 0.0002);
            }
        }
    }

    #[test]
    fn finite_budget_reports_unmet_quality() {
        let curve = inflection();
        let samples = sample(&curve, Quality { tolerance: 1e-14 }, 8);
        assert!(samples.limited);
        assert!(samples.parameters.len() <= 9);
        assert_eq!(samples.parameters.first(), Some(&curve.domain().0));
        assert_eq!(samples.parameters.last(), Some(&curve.domain().1));
    }

    #[test]
    fn straight_curves_need_only_two_endpoints() {
        let curve = NurbsCurve::create(
            false,
            3,
            &[
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
                Point::new(2.0, 0.0, 0.0),
                Point::new(3.0, 0.0, 0.0),
            ],
        );
        assert_eq!(
            sample(&curve, Quality { tolerance: 1e-10 }, CURVE_SEGMENTS).parameters,
            vec![curve.domain().0, curve.domain().1]
        );
    }

    #[test]
    fn knot_breaks_survive_adaptive_sampling() {
        let curve = NurbsCurve::create(
            false,
            1,
            &[
                Point::new(0.0, 0.0, 0.0),
                Point::new(1.0, 1.0, 0.0),
                Point::new(2.0, 0.0, 0.0),
                Point::new(3.0, 1.0, 0.0),
            ],
        );
        let sampled = sample(&curve, Quality { tolerance: 0.01 }, CURVE_SEGMENTS);
        for knot in curve.get_span_vector() {
            assert!(sampled.parameters.contains(&knot));
        }
    }

    #[test]
    fn zoom_buckets_refine_and_coarsen_with_hysteresis() {
        let quality = Quality::for_error(0.9, Default::default());
        assert!(quality.tolerance <= 0.9);
        assert_eq!(Quality::for_error(1.1, quality), quality);
        let finer = Quality::for_error(0.2, quality);
        assert!(finer.tolerance < quality.tolerance);
        assert!(Quality::for_error(3.0, finer).tolerance > finer.tolerance);
    }

    #[test]
    fn finite_mesh_retries_change_chord_tolerance_past_the_baseline_floor() {
        assert_eq!(Quality::default().mesh(100.0), super::super::brep::QUALITY);
        let mut last = 0.0;
        for tolerance in [0.1, 0.4, 1.6, 6.4] {
            let (angle, chord) = Quality { tolerance }.mesh(100.0);
            assert_eq!(angle, 20.0);
            assert!(chord > last);
            assert!((chord * 100.0 - tolerance).abs() < 1e-12);
            last = chord;
        }
    }

    #[test]
    fn nonfinite_control_data_is_never_certified_flat() {
        let mut curve = inflection();
        curve.m_cv[3] = f64::NAN;
        assert!(!flat(&curve, [0.0; 3], [3.0, 0.0, 0.0], 1.0));
    }

    #[test]
    fn a_capped_candidate_is_rejected_and_the_next_walk_is_independent() {
        assert!(candidate(|| sample(&inflection(), Quality { tolerance: 1e-14 }, 8)).is_none());
        assert!(
            candidate(|| sample(&inflection(), Quality { tolerance: 0.01 }, CURVE_SEGMENTS))
                .is_some()
        );
    }

    #[test]
    fn display_budget_preserves_holes_and_the_default_physical_mesh() {
        let source = BRep::create_block_with_hole(50.0, 30.0, 20.0, 8.0);
        let before = source.to_proto();
        let ordinary = source.face_meshes_q(Some((5.0, 0.001)));
        assert!(source.face_meshes_q_bounded((5.0, 0.001), 1).is_none());
        let bounded = source
            .face_meshes_q_bounded((5.0, 0.001), 1_000_000)
            .unwrap();
        assert_eq!(bounded.len(), ordinary.len());
        for (a, b) in bounded.iter().zip(&ordinary) {
            let (a, b) = (a.to_render(), b.to_render());
            assert_eq!(a.indices, b.indices);
            assert_eq!(
                bytemuck::cast_slice::<_, u8>(&a.vertices),
                bytemuck::cast_slice::<_, u8>(&b.vertices)
            );
        }
        assert_eq!(source.to_proto(), before);
        let repeated = source.face_meshes_q(Some((5.0, 0.001)));
        for (a, b) in repeated.iter().zip(&ordinary) {
            assert_eq!(a.to_render().indices, b.to_render().indices);
        }
    }

    #[test]
    fn retained_source_accounting_includes_spare_colors_and_nested_metadata() {
        let mut curve = inflection();
        let initial = curve_bytes(&curve).unwrap();
        curve.linecolors.reserve(4096);
        assert!(
            curve_bytes(&curve).unwrap()
                >= initial + 4096 * std::mem::size_of::<session_rust::Color>()
        );
        let mut brep = BRep::create_box(1.0, 2.0, 3.0);
        let initial = brep_source_bytes(&brep).unwrap();
        brep.m_vertices[0].point.set_guid("x".repeat(8192));
        assert!(brep_source_bytes(&brep).unwrap() >= initial + 8192);
    }

    #[test]
    fn repeated_endpoint_metadata_cannot_expand_a_display_candidate() {
        let mut source = BRep::create_box(1.0, 2.0, 3.0);
        let ordinary = source.face_meshes_q(Some((5.0, 0.001)));
        source.m_vertices[0].point.name = "x".repeat(257);
        assert!(
            source
                .face_meshes_q_bounded((5.0, 0.001), 1_000_000)
                .is_none()
        );
        let repeated = source.face_meshes_q(Some((5.0, 0.001)));
        assert_eq!(ordinary.len(), repeated.len());
        for (a, b) in ordinary.iter().zip(&repeated) {
            let (a, b) = (a.to_render(), b.to_render());
            assert_eq!(a.indices, b.indices);
            assert_eq!(
                bytemuck::cast_slice::<_, u8>(&a.vertices),
                bytemuck::cast_slice::<_, u8>(&b.vertices)
            );
        }
    }
    #[test]
    fn tiny_rational_weights_do_not_certify_an_inflection_as_a_line() {
        let original = inflection();
        let mut curve = original.clone();
        curve.m_is_rat = true;
        curve.m_cv_stride = 4;
        curve.m_cv = original
            .m_cv
            .chunks_exact(3)
            .flat_map(|p| [p[0] * 1e-16, p[1] * 1e-16, p[2] * 1e-16, 1e-16])
            .collect();
        let (a, b) = curve.domain();
        assert!(!flat(&curve, point(&curve, a), point(&curve, b), 0.001));
        let source = Geometry::NurbsCurve(std::rc::Rc::new(curve.clone()));
        assert!(source_bounds(&source).is_none());
        assert!(refinement_bytes(&source, Quality { tolerance: 0.001 }).is_none());
        assert!(
            candidate(|| sample(&curve, Quality { tolerance: 0.001 }, CURVE_SEGMENTS)).is_none()
        );
        assert!(
            !sample(&curve, Default::default(), CURVE_SEGMENTS)
                .parameters
                .is_empty()
        );
    }

    #[test]
    fn high_order_refinement_is_declined_before_sampling() {
        let points: Vec<_> = (0..18)
            .map(|i| Point::new(i as f64, (i as f64).sin(), 0.0))
            .collect();
        let curve = NurbsCurve::create(false, 17, &points);
        let source = Geometry::NurbsCurve(std::rc::Rc::new(curve.clone()));
        assert!(source_bounds(&source).is_none());
        assert!(source_bytes(&source).is_none());
        assert!(refinement_bytes(&source, Quality { tolerance: 0.001 }).is_none());
        assert!(
            candidate(|| sample(&curve, Quality { tolerance: 0.001 }, CURVE_SEGMENTS)).is_none()
        );
    }

    #[test]
    fn ordinary_spheres_and_cylinders_refine_within_the_display_work_budget() {
        // The 1600x1200 close-up camera shows about 4.6 pixels per scene unit.
        let pixels_per_unit = 4.6;
        let quality = Quality {
            tolerance: 0.45 / pixels_per_unit,
        };
        for brep in [
            BRep::create_sphere(160.0),
            BRep::create_cylinder(150.0, 240.0),
        ] {
            let ordinary = brep.face_meshes_q(Some(super::super::brep::QUALITY));
            let refined = brep
                .face_meshes_q_bounded(quality.mesh(brep_diagonal(&brep)), BREP_WORK)
                .expect("ordinary close-up stays inside the display work budget");
            let counts = |meshes: &[session_rust::Mesh]| {
                meshes.iter().map(|mesh| mesh.face.len()).sum::<usize>()
            };
            assert!(
                counts(&refined) > counts(&ordinary),
                "{} must refine its shaded faces",
                brep.name
            );
            let chains = super::super::brep_edges::edge_chains(&brep, &refined);
            for (index, edge) in brep.m_edges.iter().enumerate() {
                if !edge.degenerated && !brep.edge_faces(index).is_empty() {
                    assert!(
                        chains[index].is_some(),
                        "refined face retains boundary edge {index}"
                    );
                }
            }
        }
    }

    #[test]
    fn tracked_teapot_reaches_the_bounded_refinement_candidate() {
        let scene = session_rust::Session::pb_load(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/pb/view_mixed_teapot.pb"
        ))
        .unwrap();
        let brep = &scene.objects.breps[0];
        assert_eq!(brep.m_faces.len(), 32);
        let bytes = brep_source_bytes(brep).unwrap();
        assert!(bytes <= 512 * 1024, "teapot source reservation: {bytes}");
        let geometry = Geometry::BRep(brep.clone());
        let reservation = refinement_bytes(&geometry, Quality { tolerance: 0.2 }).unwrap();
        assert_eq!(
            reservation,
            BREP_WORK * 1024 + OBJECT_CURVE_SEGMENTS * 128 + bytes * 20
        );
        assert!(reservation <= 32 * 1024 * 1024);
    }
}
