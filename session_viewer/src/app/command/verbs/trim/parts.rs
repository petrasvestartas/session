use crate::app::command::tool::cut::{Cutter, planar, samples};
use crate::app::scene::{Scene, sync};
use session_rust::{
    BRep, BRepRef, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, Plane, Point, Polyline, Vector,
    intersection, simple_split,
};
use std::collections::HashMap;
use std::rc::Rc;

pub const TOLERANCE: f64 = 1e-6; // local units, as Split

/// What a cut leaves of one object: the parts a click can remove.
pub enum Parts {
    Curve {
        curve: NurbsCurve, // the object as a curve
        cuts: Vec<f64>,    // sorted cut parameters inside its domain
    },
    Faces {
        brep: BRep,               // the object with every cut face split
        groups: Vec<Vec<usize>>,  // faces of each part
        meshes: Vec<Mesh>,        // one mesh per face, for clicks
        section: Vec<Vec<Point>>, // the cut edges
    },
    Mesh {
        sides: Vec<Mesh>,         // above and below the plane
        section: Vec<[Point; 2]>, // the cut segments
    },
}

/// One cutter and, for a straight line, the plane it stands for off the object.
pub struct Blade {
    pub cutter: Cutter,       // in the target's frame
    pub fence: Option<Plane>, // a line's vertical plane, in the target's frame
}

impl Parts {
    /// How many parts there are.
    pub fn count(&self) -> usize {
        match self {
            Parts::Curve { curve, cuts } if curve.is_closed() => cuts.len(),
            Parts::Curve { cuts, .. } => cuts.len() + 1,
            Parts::Faces { groups, .. } => groups.len(),
            Parts::Mesh { sides, .. } => sides.len(),
        }
    }

    /// Part `index` of a curve as points, for drawing and clicking.
    pub fn outline(&self, index: usize) -> Vec<Point> {
        let Parts::Curve { curve, cuts } = self else {
            return Vec::new();
        };
        let (a, b) = interval(curve, cuts, index);
        piece(curve, a, b).map_or_else(Vec::new, |part| samples(&part, 48))
    }

    /// The points where a curve was cut.
    pub fn cut_points(&self) -> Vec<Point> {
        match self {
            Parts::Curve { curve, cuts } => cuts.iter().map(|t| curve.point_at(*t)).collect(),
            _ => Vec::new(),
        }
    }

    /// The cut edges as polylines.
    pub fn section(&self) -> Vec<Vec<Point>> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { section, .. } => section.clone(),
            Parts::Mesh { section, .. } => section.iter().map(|pair| pair.to_vec()).collect(),
        }
    }

    /// The meshes a click can hit, with the part each belongs to.
    pub fn hit_meshes(&self) -> Vec<(usize, &Mesh)> {
        match self {
            Parts::Curve { .. } => Vec::new(),
            Parts::Faces { groups, meshes, .. } => groups
                .iter()
                .enumerate()
                .flat_map(|(part, faces)| {
                    faces
                        .iter()
                        .filter_map(move |f| Some((part, meshes.get(*f)?)))
                })
                .collect(),
            Parts::Mesh { sides, .. } => sides.iter().enumerate().collect(),
        }
    }
}

/// Cut `target` (in its own frame) by `blades`; an error when nothing is crossed.
pub fn cut(target: &Geometry, blades: &[Blade]) -> Result<Parts, String> {
    match target {
        Geometry::Line(_) | Geometry::Polyline(_) | Geometry::NurbsCurve(_) => {
            let curve = crate::app::command::tool::cut::as_curve(target)
                .ok_or("This curve has no points")?;
            let cuts = curve_cuts(&curve, blades)?;
            let parts = Parts::Curve { curve, cuts };

            if parts.count() < 2 {
                return Err("not crossed".into());
            }

            Ok(parts)
        }
        Geometry::BRep(brep) => faces(brep, blades),
        Geometry::NurbsSurface(surface) => {
            let brep = face_of(surface)?;
            faces(&brep, blades)
        }
        Geometry::Mesh(mesh) => {
            let mut planes = blades
                .iter()
                .filter_map(|blade| match &blade.cutter {
                    Cutter::Plane(plane) => Some(plane),
                    Cutter::Curve(_) => blade.fence.as_ref(),
                })
                .peekable();

            if planes.peek().is_none() {
                return Err(
                    "A mesh is cut by a plane: a Plane, a planar surface or a straight line".into(),
                );
            }

            // the first plane that passes through it
            let mut answer = Err("not crossed".to_string());

            for plane in planes {
                answer = mesh_sides(mesh, plane);

                if answer.as_ref().is_ok()
                    || answer.as_ref().is_err_and(|error| error != "not crossed")
                {
                    break;
                }
            }

            let (sides, section) = answer?;
            Ok(Parts::Mesh { sides, section })
        }
        _ => Err("Trim cuts lines, polylines, curves, surfaces, BReps and meshes".into()),
    }
}

/// The cut parameters of a curve: crossings with the curves, passes through the planes.
fn curve_cuts(curve: &NurbsCurve, blades: &[Blade]) -> Result<Vec<f64>, String> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let closed = curve.is_closed();
    let curves: Vec<NurbsCurve> = blades
        .iter()
        .filter_map(|blade| match &blade.cutter {
            Cutter::Curve(curve) => Some(curve.clone()),
            Cutter::Plane(_) => None,
        })
        .collect();
    let mut cuts = Vec::new();

    // every piece end is a crossing
    if !curves.is_empty() {
        let pieces = simple_split::split_curve_by_curves(curve, &curves, TOLERANCE)?;

        if pieces.len() > 1 {
            for piece in &pieces {
                for end in [piece.point_at_start(), piece.point_at_end()] {
                    cuts.push(intersection::curve_closest_point(curve, &end, d0, d1).0);
                }
            }
        }
    }

    for blade in blades {
        if let Cutter::Plane(plane) = &blade.cutter {
            cuts.extend(intersection::curve_plane(curve, plane, Some(TOLERANCE)));
        }
    }

    // a closed curve's seam is one point; an open curve's ends cut nothing
    let mut cuts: Vec<f64> = cuts
        .into_iter()
        .map(|t| if closed && t >= d1 - near { d0 } else { t })
        .filter(|t| closed || (*t > d0 + near && *t < d1 - near))
        .collect();
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() <= near.max(1e-12));
    Ok(cuts)
}

/// The parameter range of curve part `index`; a closed curve's last part runs over the seam.
fn interval(curve: &NurbsCurve, cuts: &[f64], index: usize) -> (f64, f64) {
    let (d0, d1) = curve.domain();

    if curve.is_closed() {
        return (cuts[index], cuts[(index + 1) % cuts.len()]);
    }

    let a = if index == 0 { d0 } else { cuts[index - 1] };
    (a, cuts.get(index).copied().unwrap_or(d1))
}

/// The piece of `curve` from `a` to `b`, over the seam when `b <= a`.
fn piece(curve: &NurbsCurve, a: f64, b: f64) -> Option<NurbsCurve> {
    let (d0, d1) = curve.domain();
    let near = (d1 - d0) * 1e-9;
    let trimmed = |a: f64, b: f64| {
        let mut part = curve.clone();
        part.trim(a, b).then_some(part)
    };

    if a < b {
        return trimmed(a, b);
    }

    // over the seam: the tail and the head joined
    let tail = (a < d1 - near).then(|| trimmed(a, d1)).flatten();
    let head = (b > d0 + near).then(|| trimmed(d0, b)).flatten();

    match (tail, head) {
        (Some(tail), Some(head)) if curve.degree() == 1 => {
            let mut points = samples(&tail, 1);
            points.extend(samples(&head, 1).into_iter().skip(1));
            Some(NurbsCurve::create(false, 1, &points))
        }
        (Some(tail), Some(head)) => NurbsCurve::join(&[tail, head], Some(TOLERANCE * 10.0))
            .into_iter()
            .next(),
        (tail, head) => tail.or(head),
    }
}

/// What is left of `source` once `removed` parts are gone: one geometry per unbroken run.
pub fn kept(source: &Geometry, parts: &Parts, removed: &[usize]) -> Result<Vec<Geometry>, String> {
    let count = parts.count();

    if removed.len() >= count {
        return Err("The last part stays; use Delete".into());
    }

    match parts {
        Parts::Curve { curve, cuts } => {
            let closed = curve.is_closed();
            // start after a removed part, so a run never breaks at a closed curve's seam
            let first = if closed {
                (0..count)
                    .find(|i| removed.contains(i))
                    .map_or(0, |i| (i + 1) % count)
            } else {
                0
            };
            let mut runs: Vec<(f64, f64)> = Vec::new();
            let mut open = false; // the last run can still grow

            for step in 0..count {
                let index = (first + step) % count;

                if removed.contains(&index) {
                    open = false;
                    continue;
                }

                let (a, b) = interval(curve, cuts, index);

                match runs.last_mut() {
                    Some(run) if open => run.1 = b,
                    _ => runs.push((a, b)),
                }

                open = true;
            }

            runs.iter()
                .map(|(a, b)| {
                    rebuilt(source, curve, *a, *b)
                        .ok_or_else(|| "Cannot cut this curve".to_string())
                })
                .collect()
        }
        Parts::Faces { brep, groups, .. } => {
            let faces: Vec<usize> = removed
                .iter()
                .flat_map(|part| groups[*part].iter().copied())
                .collect();
            Ok(vec![Geometry::BRep(Rc::new(without_faces(brep, &faces)?))])
        }
        Parts::Mesh { sides, .. } => {
            let side = (0..sides.len())
                .find(|i| !removed.contains(i))
                .ok_or("The last part stays")?;
            Ok(vec![Geometry::Mesh(Rc::new(sides[side].clone()))])
        }
    }
}

/// The run `a..b` of `curve` as the source's own type, with its name and pen.
fn rebuilt(source: &Geometry, curve: &NurbsCurve, a: f64, b: f64) -> Option<Geometry> {
    let part = piece(curve, a, b)?;

    match source {
        Geometry::Line(line) => {
            let mut next = Line::from_points(&part.point_at_start(), &part.point_at_end());
            next.name = line.name.clone();
            next.width = line.width;
            next.dash = line.dash.clone();
            next.linecolor = line.linecolor.clone();
            Some(Geometry::Line(Rc::new(next)))
        }
        Geometry::Polyline(polyline) => {
            let mut next = Polyline::new(samples(&part, 1));
            next.name = polyline.name.clone();
            next.width = polyline.width;
            next.dash = polyline.dash.clone();
            next.linecolor = polyline.linecolor.clone();
            Some(Geometry::Polyline(Rc::new(next)))
        }
        Geometry::NurbsCurve(original) => {
            let mut next = part;
            next.refresh_guid();
            next.name = original.name.clone();
            next.width = original.width;

            // per-control colours only fit the control count they were made for
            if next.pointcolors.len() != next.m_cv_count {
                next.pointcolors.clear();
            }

            if next.linecolors.len() + 1 != next.m_cv_count {
                next.linecolors.clear();
            }

            Some(Geometry::NurbsCurve(Rc::new(next)))
        }
        _ => None,
    }
}

/// A surface as a one-face BRep, cut by nothing yet.
fn face_of(surface: &NurbsSurface) -> Result<BRep, String> {
    let corner = surface
        .get_cv(0, 0)
        .ok_or("Surface has no control points")?;
    let size = planar_size(surface);
    // a cutter far off the surface cuts nothing, so the kernel only wraps it
    let far = &corner + &Vector::new(size * 10.0, size * 10.0, size * 10.0);
    let away = NurbsCurve::create(
        false,
        1,
        &[far.clone(), &far + &Vector::new(size, 0.0, 0.0)],
    );
    let mut brep = simple_split::split_surface_by_curves(surface, &[away], TOLERANCE)?;
    brep.name = surface.name.clone();
    Ok(brep)
}

/// The largest control point coordinate of a surface, at least 1.
fn planar_size(surface: &NurbsSurface) -> f64 {
    let mut size: f64 = 1.0;

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                size = size.max(p[0].abs()).max(p[1].abs()).max(p[2].abs());
            }
        }
    }

    size
}

/// Split the faces of `brep` by the blades: curves on its faces make regions, planes make sides.
fn faces(brep: &BRep, blades: &[Blade]) -> Result<Parts, String> {
    let mut brep = brep.clone();
    let mut regions: Vec<Vec<usize>> = Vec::new(); // faces a curve split, one region each
    let mut planes: Vec<Plane> = Vec::new();
    let mut section: Vec<Vec<Point>> = Vec::new();
    let mut refused = None;

    for blade in blades {
        let curve = match &blade.cutter {
            Cutter::Plane(plane) => {
                planes.push(plane.clone());
                continue;
            }
            Cutter::Curve(curve) => curve,
        };
        let mut hit = false;

        for face in 0..brep.face_count() {
            let before = brep.face_count();

            match simple_split::split_brep_face_by_curves(
                &brep,
                face,
                std::slice::from_ref(curve),
                TOLERANCE,
            ) {
                Ok(next) if next.face_count() > before => {
                    regions.push(
                        [face]
                            .into_iter()
                            .chain(before..next.face_count())
                            .collect(),
                    );
                    brep = next;
                    hit = true;
                }
                Ok(_) => {}
                Err(error) => refused = Some(error),
            }
        }

        match (&blade.fence, hit) {
            (_, true) => section.push(samples(curve, 48)),
            (Some(fence), false) => planes.push(fence.clone()),
            (None, false) => {}
        }
    }

    // a plane that misses this object is some other target's
    let mut crossing = Vec::new();

    for plane in planes {
        match cut_by_plane(&mut brep, &plane) {
            Ok(split) => {
                section.extend(split);
                crossing.push(plane);
            }
            Err(error) if error == "not crossed" => {}
            Err(error) => return Err(error),
        }
    }

    let planes = crossing;

    if regions.is_empty() && section.is_empty() {
        return Err(refused.unwrap_or_else(|| {
            "Curves cut a BRep only where they lie on its faces; use a line, a Plane or a planar surface to cut through it".into()
        }));
    }

    let meshes = brep.face_meshes_q(Some(crate::app::walk::brep::QUALITY));
    let groups = if planes.is_empty() {
        // each region of a split face is a part; unsplit faces are no part
        regions
            .into_iter()
            .flatten()
            .map(|face| vec![face])
            .collect()
    } else {
        sides(&brep, &meshes, &planes, &regions)
    };

    if groups.len() < 2 {
        return Err("not crossed".into());
    }

    Ok(Parts::Faces {
        brep,
        groups,
        meshes,
        section,
    })
}

/// Faces grouped by the side of every plane they lie on; a curve-cut region stays its own part.
fn sides(
    brep: &BRep,
    meshes: &[Mesh],
    planes: &[Plane],
    regions: &[Vec<usize>],
) -> Vec<Vec<usize>> {
    let mut groups: Vec<(Vec<bool>, Option<usize>, Vec<usize>)> = Vec::new();

    for face in 0..brep.face_count() {
        let center = meshes
            .get(face)
            .map_or_else(|| brep.point_at(face, 0.5, 0.5), |mesh| mesh.centroid());
        let key: Vec<bool> = planes
            .iter()
            .map(|plane| (&center - &plane.origin()).dot(&plane.z_axis()) >= 0.0)
            .collect();
        let region = regions
            .iter()
            .any(|faces| faces.contains(&face))
            .then_some(face);

        match groups
            .iter_mut()
            .find(|(signs, own, _)| *signs == key && *own == region && region.is_none())
        {
            Some((_, _, faces)) => faces.push(face),
            None => groups.push((key, region, vec![face])),
        }
    }

    groups.into_iter().map(|(_, _, faces)| faces).collect()
}

/// Split every face `plane` passes through; returns the new edges on the plane.
fn cut_by_plane(brep: &mut BRep, plane: &Plane) -> Result<Vec<Vec<Point>>, String> {
    let normal = plane.z_axis();
    let origin = plane.origin();
    let side = |p: &Point| (p - &origin).dot(&normal);
    let size = brep
        .m_vertices
        .iter()
        .map(|v| v.point[0].abs().max(v.point[1].abs()).max(v.point[2].abs()))
        .fold(1.0, f64::max);
    let near = TOLERANCE * size;
    let mut crossed = false;

    for face in 0..brep.face_count() {
        let surface = &brep.m_surfaces[brep.m_faces[face].surface_index as usize];
        let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

        for i in 0..surface.m_cv_count[0] {
            for j in 0..surface.m_cv_count[1] {
                let d = surface.get_cv(i, j).map_or(0.0, |p| side(&p));
                low = low.min(d);
                high = high.max(d);
            }
        }

        // all on one side: nothing to cut
        if low > -near || high < near {
            continue;
        }

        // curved sections are left out: the kernel's surface-plane tracer costs 15 KB of wasm
        let own = planar(surface).ok_or(
            "A plane cuts flat faces only; cut a curved face with a curve that lies on it",
        )?;
        let Some(line) = intersection::plane_plane(&own, plane) else {
            continue;
        };
        let before = brep.face_count();
        let next = simple_split::split_brep_face_by_curves(
            brep,
            face,
            &[across(&line, surface)],
            TOLERANCE,
        )?;
        crossed |= next.face_count() > before;
        *brep = next;
    }

    if !crossed {
        return Err("not crossed".into());
    }

    // edges lying in the plane are the cut
    let mut section = Vec::new();

    for edge in &brep.m_edges {
        let Some(curve) = brep
            .m_curves_3d
            .get(edge.curve_3d_index.max(0) as usize)
            .filter(|_| edge.curve_3d_index >= 0)
        else {
            continue;
        };
        let points = samples(curve, 8);

        if points.iter().all(|p| side(p).abs() <= near * 10.0) {
            section.push(points);
        }
    }

    Ok(section)
}

/// A plane-plane line as a segment reaching past every control point of `surface`.
fn across(line: &Line, surface: &NurbsSurface) -> NurbsCurve {
    let start = line.start();
    let mut direction = line.to_vector();
    direction.normalize_self();
    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);

    for i in 0..surface.m_cv_count[0] {
        for j in 0..surface.m_cv_count[1] {
            if let Some(p) = surface.get_cv(i, j) {
                let t = (&p - &start).dot(&direction);
                low = low.min(t);
                high = high.max(t);
            }
        }
    }

    let margin = (high - low).max(1.0) * 0.1;
    NurbsCurve::create(
        false,
        1,
        &[
            &start + &(&direction * (low - margin)),
            &start + &(&direction * (high + margin)),
        ],
    )
}

/// New indices for the used entries, -1 for the rest.
fn remap(used: &[bool]) -> Vec<i32> {
    let mut next = 0;
    used.iter()
        .map(|&kept| {
            if !kept {
                return -1;
            }

            next += 1;
            next - 1
        })
        .collect()
}

/// `brep` without `removed` faces; every table is compacted and a solid left open is dropped.
pub fn without_faces(brep: &BRep, removed: &[usize]) -> Result<BRep, String> {
    if removed.is_empty() {
        return Ok(brep.clone());
    }

    let keep_face: Vec<bool> = (0..brep.m_faces.len())
        .map(|face| !removed.contains(&face))
        .collect();

    if !keep_face.contains(&true) {
        return Err("The last part stays; use Delete".into());
    }

    let mut keep_wire = vec![false; brep.m_wires.len()];
    let mut keep_surface = vec![false; brep.m_surfaces.len()];

    for (face, kept) in brep.m_faces.iter().zip(&keep_face) {
        if !kept {
            continue;
        }

        keep_surface[face.surface_index as usize] = true;

        for wire in &face.wires {
            keep_wire[wire.index as usize] = true;
        }
    }

    let mut keep_edge = vec![false; brep.m_edges.len()];

    for (wire, kept) in brep.m_wires.iter().zip(&keep_wire) {
        if !kept {
            continue;
        }

        for edge in &wire.edges {
            keep_edge[edge.index as usize] = true;
        }
    }

    let mut keep_vertex = vec![false; brep.m_vertices.len()];
    let mut keep_curve = vec![false; brep.m_curves_3d.len()];
    let mut keep_pcurve = vec![false; brep.m_curves_2d.len()];

    for (edge, kept) in brep.m_edges.iter().zip(&keep_edge) {
        if !kept {
            continue;
        }

        for vertex in [edge.start_vertex, edge.end_vertex] {
            if vertex >= 0 {
                keep_vertex[vertex as usize] = true;
            }
        }

        if edge.curve_3d_index >= 0 {
            keep_curve[edge.curve_3d_index as usize] = true;
        }

        for pcurve in edge
            .pcurves
            .iter()
            .filter(|p| keep_surface[p.surface_index as usize])
        {
            keep_pcurve[pcurve.curve_2d_index as usize] = true;

            if pcurve.curve_2d_index_2 >= 0 {
                keep_pcurve[pcurve.curve_2d_index_2 as usize] = true;
            }
        }
    }

    let [faces, wires, edges, vertices, curves, pcurves, surfaces] = [
        &keep_face,
        &keep_wire,
        &keep_edge,
        &keep_vertex,
        &keep_curve,
        &keep_pcurve,
        &keep_surface,
    ]
    .map(|used| remap(used));
    let at = |map: &[i32], index: i32| {
        if index < 0 {
            index
        } else {
            map[index as usize]
        }
    };
    let mut out = brep.clone();
    out.m_surfaces = kept_of(&brep.m_surfaces, &keep_surface);
    out.m_curves_3d = kept_of(&brep.m_curves_3d, &keep_curve);
    out.m_curves_2d = kept_of(&brep.m_curves_2d, &keep_pcurve);
    out.m_vertices = kept_of(&brep.m_vertices, &keep_vertex);
    out.m_edges = kept_of(&brep.m_edges, &keep_edge);

    for edge in &mut out.m_edges {
        edge.curve_3d_index = at(&curves, edge.curve_3d_index);
        edge.start_vertex = at(&vertices, edge.start_vertex);
        edge.end_vertex = at(&vertices, edge.end_vertex);
        edge.pcurves
            .retain(|p| keep_surface[p.surface_index as usize]);

        for pcurve in &mut edge.pcurves {
            pcurve.surface_index = at(&surfaces, pcurve.surface_index);
            pcurve.curve_2d_index = at(&pcurves, pcurve.curve_2d_index);
            pcurve.curve_2d_index_2 = at(&pcurves, pcurve.curve_2d_index_2);
        }
    }

    let refs = |list: &[BRepRef], map: &[i32]| -> Vec<BRepRef> {
        list.iter()
            .filter(|r| map[r.index as usize] >= 0)
            .map(|r| BRepRef::new(map[r.index as usize], r.orientation))
            .collect()
    };
    out.m_wires = kept_of(&brep.m_wires, &keep_wire);

    for wire in &mut out.m_wires {
        wire.edges = refs(&wire.edges, &edges);
    }

    out.m_faces = kept_of(&brep.m_faces, &keep_face);

    for face in &mut out.m_faces {
        face.surface_index = at(&surfaces, face.surface_index);
        face.wires = refs(&face.wires, &wires);
    }

    let mut shells = Vec::new(); // new index of each shell, -1 when it lost every face
    out.m_shells.clear();

    for shell in &brep.m_shells {
        let mut shell = shell.clone();
        shell.faces = refs(&shell.faces, &faces);
        shells.push(if shell.faces.is_empty() {
            -1
        } else {
            out.m_shells.len() as i32
        });

        if !shell.faces.is_empty() {
            out.m_shells.push(shell);
        }
    }

    out.m_solids.clear();

    for solid in &brep.m_solids {
        let mut solid = solid.clone();
        solid.shells = refs(&solid.shells, &shells);

        // a solid is only a solid while its shells are closed
        if !solid.shells.is_empty() && solid.shells.iter().all(|r| out.is_closed(r.index as usize))
        {
            out.m_solids.push(solid);
        }
    }

    if !out.is_valid() {
        return Err("Removing these faces leaves an invalid BRep".into());
    }

    Ok(out)
}

/// The entries of `list` marked in `used`.
fn kept_of<T: Clone>(list: &[T], used: &[bool]) -> Vec<T> {
    list.iter()
        .zip(used)
        .filter(|(_, kept)| **kept)
        .map(|(item, _)| item.clone())
        .collect()
}

/// A mesh cut by `plane` into its two sides, sharing the cut vertices; the cut as segments.
pub fn mesh_sides(mesh: &Mesh, plane: &Plane) -> Result<(Vec<Mesh>, Vec<[Point; 2]>), String> {
    if !mesh.get_face_holes().is_empty() {
        return Err("Trim cannot cut a mesh with holed faces".into());
    }

    let (mut points, faces) = mesh.to_vertices_and_faces();
    let normal = plane.z_axis();
    let origin = plane.origin();
    let size = points
        .iter()
        .map(|p| p[0].abs().max(p[1].abs()).max(p[2].abs()))
        .fold(1.0, f64::max);
    let near = 1e-9 * size;
    let side: Vec<f64> = points
        .iter()
        .map(|p| {
            let d = (p - &origin).dot(&normal);
            if d.abs() <= near { 0.0 } else { d }
        })
        .collect();

    if side.iter().all(|d| *d >= 0.0) || side.iter().all(|d| *d <= 0.0) {
        return Err("not crossed".into());
    }

    let mut crossings: HashMap<(usize, usize), usize> = HashMap::new(); // edge to its cut vertex
    let mut halves: [Vec<Vec<usize>>; 2] = [Vec::new(), Vec::new()];
    let mut section = Vec::new();

    for face in &faces {
        if face.iter().all(|v| side[*v] >= 0.0) {
            halves[0].push(face.clone());
            continue;
        }

        if face.iter().all(|v| side[*v] <= 0.0) {
            halves[1].push(face.clone());
            continue;
        }

        let mut above = Vec::new();
        let mut below = Vec::new();
        let mut on = Vec::new(); // this face's points on the plane

        for (i, &a) in face.iter().enumerate() {
            let b = face[(i + 1) % face.len()];

            if side[a] >= 0.0 {
                above.push(a);
            }

            if side[a] <= 0.0 {
                below.push(a);
            }

            if side[a] == 0.0 {
                on.push(a);
            }

            if side[a] * side[b] < 0.0 {
                let key = (a.min(b), a.max(b));
                let at = *crossings.entry(key).or_insert_with(|| {
                    let t = side[a] / (side[a] - side[b]);
                    let (p, q) = (&points[a], &points[b]);
                    points.push(Point::new(
                        p[0] + t * (q[0] - p[0]),
                        p[1] + t * (q[1] - p[1]),
                        p[2] + t * (q[2] - p[2]),
                    ));
                    points.len() - 1
                });
                above.push(at);
                below.push(at);
                on.push(at);
            }
        }

        if let [a, b] = on.as_slice() {
            section.push([points[*a].clone(), points[*b].clone()]);
        }

        for (half, ring) in [(0, above), (1, below)] {
            if ring.len() >= 3 {
                halves[half].push(ring);
            }
        }
    }

    let sides = halves
        .into_iter()
        .map(|faces| {
            // only the vertices this side uses
            let mut index = vec![usize::MAX; points.len()];
            let mut used = Vec::new();

            for face in &faces {
                for &v in face {
                    if index[v] == usize::MAX {
                        index[v] = used.len();
                        used.push(points[v].clone());
                    }
                }
            }

            let faces = faces
                .iter()
                .map(|face| face.iter().map(|v| index[*v]).collect())
                .collect();
            let mut side = Mesh::from_vertices_and_faces(used, faces);
            side.name = mesh.name.clone();
            side.set_objectcolor(mesh.get_objectcolor().clone());
            side
        })
        .collect();
    Ok((sides, section))
}

impl Scene {
    /// Replace each target by what it keeps, in one undo step of its document; returns how many changed.
    pub fn commit_trim(&mut self, edits: &[(u32, Vec<Geometry>)]) -> Result<usize, String> {
        let mut doc = None;
        let mut plans = Vec::new(); // (guid, parent name, local transform, colours, pieces)

        for (row, pieces) in edits {
            let (at, guid) = self.identity_of(*row).ok_or("A target no longer exists")?;

            if doc.is_some_and(|doc| doc != at) {
                return Err("Trim edits one document at a time".into());
            }

            doc = Some(at);
            let file = self.docs.get(at).ok_or("A target has no document")?;

            if file.display_only {
                return Err(crate::app::scene::READ_ONLY.into());
            }

            let parent = self
                .node_of(*row)
                .and_then(|(node, in_tree)| in_tree.then(|| node.borrow().parent()).flatten())
                .map(|node| node.borrow().name.clone());
            let place = file.session.xform(&guid);
            let color = self.colors.get(&(at, Rc::clone(&guid))).copied();
            let edge_color = self.edge_colors.get(&(at, Rc::clone(&guid))).copied();
            plans.push((guid, parent, place, color, edge_color, pieces.clone()));
        }

        let Some(doc) = doc else {
            return Ok(0);
        };
        let session = Rc::make_mut(&mut self.docs[doc].session);
        let mut made = Vec::new();
        session.begin("trim");
        let mut refused = None; // a piece the document would not take

        for (guid, parent, place, color, edge_color, mut pieces) in plans {
            let Some(source) = session.lookup.get(guid.as_ref()).cloned() else {
                refused = Some("A target no longer exists");
                break;
            };
            let name = source.name().to_owned();
            let parent = parent.and_then(|name| session.tree.get_node_by_name(&name));

            // several pieces are named `x (part 1)`, `x (part 2)`...
            if pieces.len() > 1 {
                for (index, piece) in pieces.iter_mut().enumerate() {
                    let name = format!("{name} (part {})", index + 1);

                    match piece {
                        Geometry::Line(p) => Rc::make_mut(p).name = name,
                        Geometry::Polyline(p) => Rc::make_mut(p).name = name,
                        Geometry::NurbsCurve(p) => Rc::make_mut(p).name = name,
                        _ => {}
                    }
                }
            }

            let first = pieces.remove(0);
            let same = std::mem::discriminant(&first) == std::mem::discriminant(&source);

            // the first piece keeps the object; a surface that became a BRep is swapped for it
            if same {
                session.replace(&guid, first);
            } else {
                session.remove_object(&guid);
                pieces.insert(0, first);
            }

            for piece in pieces {
                let node = match piece {
                    Geometry::Line(p) => Some(session.add_line((*p).clone(), parent.as_ref())),
                    Geometry::Polyline(p) => session.add_polyline((*p).clone(), parent.as_ref()),
                    Geometry::NurbsCurve(p) => {
                        session.add_nurbscurve((*p).clone(), parent.as_ref())
                    }
                    Geometry::BRep(p) => session.add_brep((*p).clone(), parent.as_ref()),
                    Geometry::Mesh(p) => session.add_mesh((*p).clone(), parent.as_ref()),
                    _ => None,
                };
                let Some(node) = node else {
                    refused = Some("Cannot add a trimmed piece");
                    break;
                };
                let id = node.borrow().name.clone();
                session.set_xform(&id, place.clone());

                if let Some(color) = color {
                    self.colors.insert((doc, Rc::from(id.as_str())), color);
                }

                if let Some(color) = edge_color {
                    self.edge_colors.insert((doc, Rc::from(id.as_str())), color);
                }

                made.push(node);
            }

            if refused.is_some() {
                break;
            }
        }

        // half a trim is taken back before anything is drawn
        if let Some(reason) = refused {
            let wrote = session
                .history
                .current
                .as_ref()
                .is_some_and(|transaction| !transaction.ops.is_empty());
            session.commit();

            if wrote {
                session.undo();
                session.history.redo_stack.pop();
            }

            return Err(reason.into());
        }

        let notes = sync::commit(session);
        self.noted(doc, notes);

        for node in &made {
            self.hint(doc, node);
        }

        self.edited(&[doc]);
        Ok(edits.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::scene::FileDoc;
    use session_rust::{Session, Xform};

    /// A curve cutter.
    fn blade(a: [f64; 3], b: [f64; 3]) -> Blade {
        let curve = NurbsCurve::create(
            false,
            1,
            &[Point::new(a[0], a[1], a[2]), Point::new(b[0], b[1], b[2])],
        );
        Blade {
            cutter: Cutter::Curve(curve),
            fence: None,
        }
    }

    /// A plane cutter.
    fn plane(origin: [f64; 3], normal: [f64; 3]) -> Blade {
        let plane = Plane::from_point_normal(
            Point::new(origin[0], origin[1], origin[2]),
            Vector::new(normal[0], normal[1], normal[2]),
            None,
        );
        Blade {
            cutter: Cutter::Plane(plane),
            fence: None,
        }
    }

    /// A line from its ends.
    fn line(a: [f64; 3], b: [f64; 3]) -> Geometry {
        Geometry::Line(Rc::new(Line::from_points(
            &Point::new(a[0], a[1], a[2]),
            &Point::new(b[0], b[1], b[2]),
        )))
    }

    /// The ends of a line geometry.
    fn ends(geometry: &Geometry) -> [[f64; 3]; 2] {
        let Geometry::Line(line) = geometry else {
            panic!("not a line")
        };
        let (a, b) = (line.start(), line.end());
        [[a[0], a[1], a[2]], [b[0], b[1], b[2]]]
    }

    /// Lines keep the part they were not clicked on; three parts keep the two outer ones.
    #[test]
    fn a_line_keeps_what_is_not_removed() {
        let mut source = Line::from_points(&Point::new(0., 0., 0.), &Point::new(100., 0., 0.));
        source.width = 3.0;
        source.name = "rail".into();
        let source = Geometry::Line(Rc::new(source));
        let parts = cut(&source, &[blade([50., -5., 0.], [50., 5., 0.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let kept_left = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(ends(&kept_left[0]), [[0., 0., 0.], [50., 0., 0.]]);
        let Geometry::Line(styled) = &kept_left[0] else {
            panic!()
        };
        assert_eq!((styled.width, styled.name.as_str()), (3.0, "rail"));
        let parts = cut(
            &source,
            &[
                blade([30., -5., 0.], [30., 5., 0.]),
                blade([70., -5., 0.], [70., 5., 0.]),
            ],
        )
        .unwrap();
        assert_eq!(parts.count(), 3);
        let outer = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(outer.len(), 2);
        assert!((ends(&outer[0])[1][0] - 30.0).abs() < 1e-9);
        assert!((ends(&outer[1])[0][0] - 70.0).abs() < 1e-9);
        let right = kept(&source, &parts, &[0]).unwrap();
        assert_eq!(right.len(), 1, "the two kept parts stay one line");
        assert!(
            (ends(&right[0])[0][0] - 30.0).abs() < 1e-9
                && (ends(&right[0])[1][0] - 100.0).abs() < 1e-9
        );
        assert!(kept(&source, &parts, &[0, 1, 2]).is_err());
        assert!(cut(&source, &[blade([50., 5., 0.], [50., 15., 0.])]).is_err());
    }

    /// A polyline keeps its corner; a NURBS curve ends at the cut; a closed square makes two parts.
    #[test]
    fn polylines_curves_and_closed_curves() {
        let corner = Polyline::new(vec![
            Point::new(0., 60., 0.),
            Point::new(40., 60., 0.),
            Point::new(40., 100., 0.),
        ]);
        let source = Geometry::Polyline(Rc::new(corner));
        let parts = cut(&source, &[blade([20., 50., 0.], [20., 70., 0.])]).unwrap();
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::Polyline(rest) = &rest[0] else {
            panic!()
        };
        let points = rest.get_points();
        assert_eq!(points.len(), 3);
        assert!(points[0].distance(&Point::new(20., 60., 0.), None) < 1e-9);
        assert!(points[1].distance(&Point::new(40., 60., 0.), None) < 1e-9);
        let curve = NurbsCurve::create(
            false,
            2,
            &[
                Point::new(0., -60., 0.),
                Point::new(50., 0., 0.),
                Point::new(100., -60., 0.),
            ],
        );
        let source = Geometry::NurbsCurve(Rc::new(curve));
        let parts = cut(&source, &[blade([50., -80., 0.], [50., 10., 0.])]).unwrap();
        let left = kept(&source, &parts, &[1]).unwrap();
        let Geometry::NurbsCurve(left) = &left[0] else {
            panic!()
        };
        assert!((left.point_at_end()[0] - 50.0).abs() < 1e-6);
        let square = Polyline::new(vec![
            Point::new(0., 0., 0.),
            Point::new(10., 0., 0.),
            Point::new(10., 10., 0.),
            Point::new(0., 10., 0.),
            Point::new(0., 0., 0.),
        ]);
        let source = Geometry::Polyline(Rc::new(square));
        let parts = cut(&source, &[blade([5., -5., 0.], [5., 15., 0.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let half = kept(&source, &parts, &[0]).unwrap();
        assert_eq!(half.len(), 1);
    }

    /// A plane cuts a vertical line; mixed cutters cut at both.
    #[test]
    fn planes_and_curves_cut_together() {
        let source = line([0., 0., -10.], [0., 0., 10.]);
        let parts = cut(&source, &[plane([0., 0., 0.], [0., 0., 1.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let across = line([0., 0., 0.], [100., 0., 0.]);
        let parts = cut(
            &across,
            &[
                plane([20., 0., 0.], [1., 0., 0.]),
                blade([60., -5., 0.], [60., 5., 0.]),
            ],
        )
        .unwrap();
        assert_eq!(parts.count(), 3);
        let points = parts.cut_points();
        assert!((points[0][0] - 20.0).abs() < 1e-6 && (points[1][0] - 60.0).abs() < 1e-6);
    }

    /// A box cut by a plane: removing the top leaves an open, valid five-face BRep.
    #[test]
    fn a_box_cut_by_a_plane_loses_its_top() {
        let source = BRep::create_box(10., 10., 10.);
        let low = source
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let parts = cut(
            &Geometry::BRep(Rc::new(source)),
            &[plane([0., 0., low + 2.], [0., 0., 1.])],
        )
        .unwrap();
        assert_eq!(parts.count(), 2);
        let Parts::Faces {
            brep,
            groups,
            section,
            ..
        } = &parts
        else {
            panic!()
        };
        assert_eq!(brep.face_count(), 10);
        assert_eq!(section.len(), 4, "four cut edges");
        let upper = groups
            .iter()
            .position(|faces| {
                faces
                    .iter()
                    .all(|f| brep.point_at(*f, 0.5, 0.5)[2] > low + 2.0 - 1e-9)
            })
            .unwrap();
        let rest = kept(&Geometry::BRep(Rc::new(BRep::new())), &parts, &[upper]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 5);
        assert!(rest.is_valid());
        assert!(!rest.is_solid());

        for edge in 0..rest.m_edges.len() {
            assert!(!rest.edge_faces(edge).is_empty(), "edge {edge} has a face");
        }

        let used: std::collections::HashSet<i32> = rest
            .m_edges
            .iter()
            .flat_map(|e| [e.start_vertex, e.end_vertex])
            .collect();
        assert_eq!(used.len(), rest.m_vertices.len(), "no unused vertex");
        assert!(
            cut(
                &Geometry::BRep(Rc::new(BRep::create_box(10., 10., 10.))),
                &[plane([0., 0., 100.], [0., 0., 1.])]
            )
            .is_err()
        );
        assert_eq!(without_faces(rest, &[]).unwrap().face_count(), 5);
    }

    /// A plane through a curved face is refused rather than cut half way.
    #[test]
    fn a_plane_refuses_curved_faces() {
        let source = Geometry::BRep(Rc::new(BRep::create_cylinder(5., 10.)));
        let Geometry::BRep(cylinder) = &source else {
            panic!()
        };
        let low = cylinder
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let answer = cut(&source, &[plane([0., 0., low + 5.], [0., 0., 1.])]);
        assert!(answer.is_err_and(|error| error.contains("flat faces")));
    }

    /// A line off the box cuts it as a vertical fence; a line on the top face splits that face only.
    #[test]
    fn lines_fence_or_split_a_box() {
        let source = Geometry::BRep(Rc::new(BRep::create_box(10., 10., 10.)));
        let Geometry::BRep(boxed) = &source else {
            panic!()
        };
        let low = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::INFINITY, f64::min);
        let high = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[2])
            .fold(f64::NEG_INFINITY, f64::max);
        let x = boxed
            .m_vertices
            .iter()
            .map(|v| v.point[0])
            .fold(f64::INFINITY, f64::min)
            + 3.0;
        let fence = crate::app::command::tool::cut::fence_plane(
            &Line::from_points(
                &Point::new(x, -50., low - 5.),
                &Point::new(x, 50., low - 5.),
            ),
            &Vector::new(0., 0., 1.),
        )
        .unwrap();
        let mut off = blade([x, -50., low - 5.], [x, 50., low - 5.]);
        off.fence = Some(fence);
        let parts = cut(&source, &[off]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 5);
        assert!(rest.is_valid());
        let on = blade([x, -50., high], [x, 50., high]);
        let parts = cut(&source, &[on]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[1]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 6);
        assert!(rest.is_valid());
    }

    /// A planar surface split by a line keeps the chosen region as a BRep.
    #[test]
    fn a_surface_becomes_the_kept_region() {
        let mut surface = NurbsSurface::new(3, false, 2, 2, 2, 2);
        surface.set_cv(0, 0, &Point::new(0., 0., 0.));
        surface.set_cv(1, 0, &Point::new(10., 0., 0.));
        surface.set_cv(0, 1, &Point::new(0., 10., 0.));
        surface.set_cv(1, 1, &Point::new(10., 10., 0.));
        let source = Geometry::NurbsSurface(Rc::new(surface));
        let parts = cut(&source, &[blade([4., -5., 0.], [4., 15., 0.])]).unwrap();
        assert_eq!(parts.count(), 2);
        let rest = kept(&source, &parts, &[0]).unwrap();
        let Geometry::BRep(rest) = &rest[0] else {
            panic!()
        };
        assert_eq!(rest.face_count(), 1);
    }

    /// A mesh box cut at z = 2: the areas add up, both sides are open and share the cut.
    #[test]
    fn a_mesh_splits_into_two_open_sides() {
        let mut source = Mesh::create_box(10., 10., 10.);
        source.name = "block".into();
        let area = source.area();
        let (points, _) = source.to_vertices_and_faces();
        let low = points.iter().map(|p| p[2]).fold(f64::INFINITY, f64::min);
        let cutter =
            Plane::from_point_normal(Point::new(0., 0., low + 2.), Vector::new(0., 0., 1.), None);
        let (sides, section) = mesh_sides(&source, &cutter).unwrap();
        assert!((sides[0].area() + sides[1].area() - area).abs() < 1e-9);
        assert!(!sides[0].is_closed() && !sides[1].is_closed());
        assert_eq!(sides[1].number_of_vertices(), 8);
        assert_eq!(sides[0].name, "block");
        assert!(!section.is_empty());
        let away =
            Plane::from_point_normal(Point::new(0., 0., 100.), Vector::new(0., 0., 1.), None);
        assert!(mesh_sides(&source, &away).is_err());
    }

    /// One trim is one undo step; the pieces keep the group, placement and colour.
    #[test]
    fn a_trim_is_one_undo_step() {
        let mut session = Session::new("doc");
        let group = session.add_group("parts");
        let target = session.add_line(
            Line::from_points(&Point::new(0., 0., 0.), &Point::new(100., 0., 0.)),
            Some(&group),
        );
        let id = target.borrow().name.clone();
        session.set_xform(&id, Xform::translation(0., 5., 0.));
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "doc".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.,
            display_only: false,
        });
        let row = scene.row_of(0, &id).unwrap();
        scene
            .colors
            .insert(scene.identity_of(row).unwrap(), [60, 170, 100]);
        let before = Rc::clone(&scene.docs[0].session);
        let source = scene.geometry(row).unwrap().clone();
        let parts = cut(
            &source,
            &[
                blade([30., -5., 0.], [30., 5., 0.]),
                blade([70., -5., 0.], [70., 5., 0.]),
            ],
        )
        .unwrap();
        assert!(
            Rc::ptr_eq(&before, &scene.docs[0].session),
            "cutting writes nothing"
        );
        let pieces = kept(&source, &parts, &[1]).unwrap();
        assert_eq!(scene.commit_trim(&[(row, pieces)]).unwrap(), 1);
        let session = &scene.docs[0].session;
        assert_eq!(session.objects.lines.len(), 2);
        let parent = session.tree.get_node_by_name("parts").unwrap();
        assert_eq!(parent.borrow().children().len(), 2);
        assert_eq!(scene.colors.len(), 2);

        for child in parent.borrow().children() {
            let place = session.xform(&child.borrow().name);
            assert_eq!(place.m[13], 5.0);
        }

        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 1);
        assert!(scene.redo());
        assert_eq!(scene.docs[0].session.objects.lines.len(), 2);
    }

    /// A piece the document refuses takes the whole trim back and keeps the earlier undo steps.
    #[test]
    fn a_refused_piece_changes_nothing() {
        let mut session = Session::new("doc");
        let target = session.add_line(
            Line::from_points(&Point::new(0., 0., 0.), &Point::new(100., 0., 0.)),
            None,
        );
        let id = target.borrow().name.clone();
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "doc".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.,
            display_only: false,
        });
        let row = scene.row_of(0, &id).unwrap();
        scene
            .commit_geometry(row, line([0., 0., 0.], [90., 0., 0.]), "edit")
            .unwrap();
        let depth = scene.docs[0].session.history.undo_stack.len();
        let lone = Geometry::Polyline(Rc::new(Polyline::new(vec![Point::new(1., 1., 1.)])));
        let pieces = vec![line([0., 0., 0.], [30., 0., 0.]), lone];
        assert!(scene.commit_trim(&[(row, pieces)]).is_err());
        let session = &scene.docs[0].session;
        assert!(session.history.current.is_none());
        assert_eq!(session.history.undo_stack.len(), depth);
        assert_eq!(ends(scene.geometry(row).unwrap())[1], [90., 0., 0.]);
        assert!(session.objects.polylines.is_empty());
    }

    /// A trimmed surface becomes a BRep that saving keeps.
    #[test]
    fn a_trimmed_surface_is_saved_as_a_brep() {
        let mut surface = NurbsSurface::new(3, false, 2, 2, 2, 2);
        surface.set_cv(0, 0, &Point::new(0., 0., 0.));
        surface.set_cv(1, 0, &Point::new(10., 0., 0.));
        surface.set_cv(0, 1, &Point::new(0., 10., 0.));
        surface.set_cv(1, 1, &Point::new(10., 10., 0.));
        let mut session = Session::new("doc");
        let node = session.add_nurbssurface(surface, None).unwrap();
        let id = node.borrow().name.clone();
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "doc".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.,
            display_only: false,
        });
        let row = scene.row_of(0, &id).unwrap();
        let source = scene.geometry(row).unwrap().clone();
        let parts = cut(&source, &[blade([4., -5., 0.], [4., 15., 0.])]).unwrap();
        let pieces = kept(&source, &parts, &[0]).unwrap();
        scene.commit_trim(&[(row, pieces)]).unwrap();
        let session = &scene.docs[0].session;
        assert_eq!(session.objects.breps.len(), 1);
        assert!(session.objects.nurbssurfaces.is_empty());
        let bytes = crate::app::session_io::save(&scene).unwrap();
        let restored = crate::app::session_io::open(&bytes).unwrap();
        assert_eq!(restored.docs[0].session.objects.breps.len(), 1);
        assert!(restored.docs[0].session.objects.nurbssurfaces.is_empty());
        assert!(scene.undo());
        assert_eq!(scene.docs[0].session.objects.nurbssurfaces.len(), 1);
    }
}
