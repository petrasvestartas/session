//! Known retained source payloads, not RSS, allocator usage or total heap ownership.
//! Vec/String capacities are exact payload capacities. Slice and map-entry figures use
//! exposed occupied lengths; private spare capacity and allocator metadata are excluded.
use super::super::scene::Doc;
use session_rust::{
    BRep, Element, Geometry, Line, Mesh, NurbsCurve, NurbsSurface, NurbsSurfaceTrimmed, OBB, Plane,
    Point, PointCloud, Polyline, Session,
};
use std::collections::{HashMap, HashSet};
use std::mem::{size_of, size_of_val};
use std::rc::{Rc, Weak};

/// Disjoint known payload categories; their sum is a lower bound on retained source memory.
#[derive(Clone, Copy, Default, serde::Serialize)]
pub(super) struct Payload {
    pub vector_capacity_bytes: usize,
    pub string_capacity_bytes: usize,
    pub exposed_slice_bytes: usize,
    pub occupied_map_entry_bytes: usize,
    pub shared_value_bytes: usize,
    pub unique_sessions: usize,
    pub unique_geometry_values: usize,
    pub scans: u64,
}

impl Payload {
    /// Sum the disjoint known categories without implying complete heap accounting.
    pub fn known_bytes(&self) -> usize {
        self.vector_capacity_bytes
            + self.string_capacity_bytes
            + self.exposed_slice_bytes
            + self.occupied_map_entry_bytes
            + self.shared_value_bytes
    }
    /// Count allocated Vec elements without traversing them or counting their children twice.
    fn vector<T>(&mut self, value: &Vec<T>) {
        self.vector_capacity_bytes += value.capacity() * size_of::<T>();
    }
    /// Count occupied data where the source exposes a slice but keeps capacity private.
    fn slice<T>(&mut self, value: &[T]) {
        self.exposed_slice_bytes += size_of_val(value);
    }
    /// Count key/value payload only; hash buckets, control bytes and spare slots are excluded.
    fn map<K, V>(&mut self, value: &HashMap<K, V>) {
        self.occupied_map_entry_bytes += value.len() * size_of::<(K, V)>();
    }
    /// Account child string allocation separately from its already-counted header.
    fn string(&mut self, value: &String) {
        self.string_capacity_bytes += value.capacity();
    }
}

/// Weak identities keep the inspection cache from retaining source objects after replacement.
#[derive(Default)]
pub(super) struct SourceCache {
    documents: Vec<Weak<Session>>,
    payload: Payload,
}

impl SourceCache {
    /// Current viewer sources are immutable Rc sessions: replacement/append changes identity.
    /// Future in-place source editing must invalidate this cache or replace the Rc session.
    pub fn snapshot(&mut self, docs: &[Doc]) -> Payload {
        let mut matches = docs.len() == self.documents.len();
        if matches {
            for (old, doc) in self.documents.iter().zip(docs) {
                if old.as_ptr() != Rc::as_ptr(&doc.session) {
                    matches = false;
                    break;
                }
            }
        }
        if matches {
            return self.payload;
        }
        let mut payload = Payload {
            scans: self.payload.scans + 1,
            ..Payload::default()
        };
        let mut seen_sessions = HashSet::new();
        let mut seen_geometry = HashSet::new();
        self.documents.clear();
        for doc in docs {
            self.documents.push(Rc::downgrade(&doc.session));
            if seen_sessions.insert(Rc::as_ptr(&doc.session) as usize) {
                payload.unique_sessions += 1;
                payload.shared_value_bytes += size_of::<Session>();
                session_payload(&doc.session, &mut payload, &mut seen_geometry);
            }
        }
        self.payload = payload;
        payload
    }
}

/// Count each shared value once across typed collections, lookup and repeated documents.
/// A typed collection: its vector, then every shared value it holds.
fn shared_all<T>(
    values: &Vec<Rc<T>>,
    payload: &mut Payload,
    seen: &mut HashSet<usize>,
    children: fn(&T, &mut Payload),
) {
    payload.vector(values);
    for value in values {
        shared(value, payload, seen, children);
    }
}

fn shared<T>(
    value: &Rc<T>,
    payload: &mut Payload,
    seen: &mut HashSet<usize>,
    children: fn(&T, &mut Payload),
) {
    if !seen.insert(Rc::as_ptr(value) as usize) {
        return;
    }
    payload.unique_geometry_values += 1;
    payload.shared_value_bytes += size_of::<T>();
    children(value, payload);
}

/// Include typed collection holders and both stores, which may own different geometry values.
fn session_payload(session: &Session, p: &mut Payload, seen: &mut HashSet<usize>) {
    p.string(&session.name);
    p.string(&session.objects.name);
    let objects = &session.objects;
    shared_all(&objects.points, p, seen, point_payload);
    shared_all(&objects.lines, p, seen, line_payload);
    shared_all(&objects.planes, p, seen, plane_payload);
    shared_all(&objects.bboxes, p, seen, box_payload);
    shared_all(&objects.polylines, p, seen, polyline_payload);
    shared_all(&objects.pointclouds, p, seen, cloud_payload);
    shared_all(&objects.meshes, p, seen, mesh_payload);
    shared_all(&objects.nurbscurves, p, seen, curve_payload);
    shared_all(&objects.nurbssurfaces, p, seen, surface_payload);
    shared_all(&objects.nurbssurfacetrimmeds, p, seen, trimmed_payload);
    shared_all(&objects.breps, p, seen, brep_payload);
    shared_all(&objects.elements, p, seen, element_payload);
    p.vector(&objects.components);
    for component in &objects.components {
        p.string(&component.name);
        p.string(&component.type_name);
        p.string(&component.guid);
    }
    p.map(&session.lookup);
    for (name, geometry) in &session.lookup {
        p.string(name);
        match geometry {
            Geometry::Point(value) => shared(value, p, seen, point_payload),
            Geometry::Line(value) => shared(value, p, seen, line_payload),
            Geometry::Plane(value) => shared(value, p, seen, plane_payload),
            Geometry::OBB(value) => shared(value, p, seen, box_payload),
            Geometry::Polyline(value) => shared(value, p, seen, polyline_payload),
            Geometry::PointCloud(value) => shared(value, p, seen, cloud_payload),
            Geometry::Mesh(value) => shared(value, p, seen, mesh_payload),
            Geometry::NurbsCurve(value) => shared(value, p, seen, curve_payload),
            Geometry::NurbsSurface(value) => shared(value, p, seen, surface_payload),
            Geometry::BRep(value) => shared(value, p, seen, brep_payload),
            Geometry::Element(value) => shared(value, p, seen, element_payload),
        }
    }
    p.map(&session.xforms);
    for name in session.xforms.keys() {
        p.string(name);
    }
    p.vector(&session.cached_guids);
    for name in &session.cached_guids {
        p.string(name);
    }
    p.vector(&session.cached_boxes);
    for value in &session.cached_boxes {
        box_payload(value, p);
    }
}

/// Points keep coordinates in their counted struct; only public name allocation is additional.
fn point_payload(value: &Point, p: &mut Payload) {
    p.string(&value.name);
    p.string(&value.pointcolor.name);
}
/// Lines expose their dash pattern and name; coordinate storage is inline.
fn line_payload(value: &Line, p: &mut Payload) {
    p.string(&value.name);
    p.vector(&value.dash);
    p.string(&value.linecolor.name);
}
/// Plane internals are private; no cloned origin or axes are manufactured for inspection.
fn plane_payload(value: &Plane, p: &mut Payload) {
    p.string(&value.name);
    p.string(&value.linecolor.name);
}
/// Public box center metadata is additional to the enclosing box value.
fn box_payload(value: &OBB, p: &mut Payload) {
    p.string(&value.name);
    point_payload(&value.center, p);
    p.string(&value.x_axis.name);
    p.string(&value.y_axis.name);
    p.string(&value.z_axis.name);
    p.string(&value.half_size.name);
}
/// Polyline coordinates and dash storage are exposed Vec capacities.
fn polyline_payload(value: &Polyline, p: &mut Payload) {
    p.string(&value.name);
    p.vector(&value.coords);
    p.vector(&value.dash);
    plane_payload(&value.plane, p);
    p.string(&value.linecolor.name);
}
/// Cloud accessors borrow packed data without allocating Point/Color objects or serialization.
fn cloud_payload(value: &PointCloud, p: &mut Payload) {
    p.string(&value.name);
    p.slice(value.coords());
    p.slice(value.colors());
    p.slice(value.normals());
    p.slice(value.point_ids());
}
/// Curves expose control, knot and color vector capacities directly.
fn curve_payload(value: &NurbsCurve, p: &mut Payload) {
    p.string(&value.name);
    p.vector(&value.m_cv);
    p.vector(&value.m_nurbsknot);
    p.vector(&value.pointcolors);
    p.vector(&value.linecolors);
    color_names(&value.pointcolors, p);
    color_names(&value.linecolors, p);
}
/// Include an actual retained surface mesh cache, without creating one for accounting.
fn surface_payload(value: &NurbsSurface, p: &mut Payload) {
    p.string(&value.name);
    p.vector(&value.m_cv);
    p.vector(&value.m_nurbsknot[0]);
    p.vector(&value.m_nurbsknot[1]);
    p.vector(&value.pointcolors);
    p.vector(&value.linecolors);
    p.vector(&value.facecolors);
    color_names(&value.pointcolors, p);
    color_names(&value.linecolors, p);
    color_names(&value.facecolors, p);
    if let Some(mesh) = &value.m_mesh {
        mesh_payload(mesh, p);
    }
}
/// Trim loops and plane-cut arrays belong to the retained trimmed source representation.
fn trimmed_payload(value: &NurbsSurfaceTrimmed, p: &mut Payload) {
    p.string(&value.name);
    p.string(&value.surfacecolor.name);
    surface_payload(&value.m_surface, p);
    p.vector(&value.m_inner_loops);
    p.vector(&value.cut_planes);
    if let Some(curve) = &value.m_outer_loop {
        curve_payload(curve, p);
    }
    for curve in &value.m_inner_loops {
        curve_payload(curve, p);
    }
    if let Some(point) = &value.cut_q0 {
        point_payload(point, p);
    }
    if let Some(normal) = &value.cut_n {
        p.string(&normal.name);
    }
    for (point, normal) in &value.cut_planes {
        point_payload(point, p);
        p.string(&normal.name);
    }
}
/// Account nested topology vectors and their owned curve/surface children exactly once.
fn brep_payload(value: &BRep, p: &mut Payload) {
    p.string(&value.name);
    p.string(&value.surfacecolor.name);
    p.vector(&value.m_surfaces);
    p.vector(&value.m_curves_3d);
    p.vector(&value.m_curves_2d);
    p.vector(&value.m_vertices);
    p.vector(&value.m_edges);
    p.vector(&value.m_wires);
    p.vector(&value.m_faces);
    p.vector(&value.m_shells);
    p.vector(&value.m_solids);
    for surface in &value.m_surfaces {
        surface_payload(surface, p);
    }
    for curve in &value.m_curves_3d {
        curve_payload(curve, p);
    }
    for curve in &value.m_curves_2d {
        curve_payload(curve, p);
    }
    for vertex in &value.m_vertices {
        point_payload(&vertex.point, p);
    }
    for edge in &value.m_edges {
        p.vector(&edge.pcurves);
    }
    for wire in &value.m_wires {
        p.vector(&wire.edges);
    }
    for face in &value.m_faces {
        p.vector(&face.wires);
        if let Some(color) = &face.facecolor {
            p.string(&color.name);
        }
    }
    for shell in &value.m_shells {
        p.vector(&shell.faces);
    }
    for solid in &value.m_solids {
        p.vector(&solid.shells);
    }
}
/// Color arrays own public names in addition to their inline RGBA values and headers.
fn color_names(values: &[session_rust::Color], p: &mut Payload) {
    for color in values {
        p.string(&color.name);
    }
}

/// String-keyed attribute entry payload excludes the map implementation's bucket metadata.
fn attributes_payload(value: &HashMap<String, f64>, p: &mut Payload) {
    p.map(value);
    for key in value.keys() {
        p.string(key);
    }
}
/// Mesh maps expose occupied entries; vector child capacities remain separately accountable.
fn mesh_payload(value: &Mesh, p: &mut Payload) {
    p.string(&value.name);
    p.map(&value.vertex);
    p.map(&value.face);
    p.map(&value.halfedge);
    for vertex in value.vertex.values() {
        p.occupied_map_entry_bytes += vertex.attributes.len() * size_of::<(String, f64)>();
        for key in vertex.attributes.keys() {
            p.string(key);
        }
    }
    for face in value.face.values() {
        p.vector(face);
    }
    for edges in value.halfedge.values() {
        p.map(edges);
    }
    p.map(&value.facedata);
    for attributes in value.facedata.values() {
        attributes_payload(attributes, p);
    }
    p.map(&value.edgedata);
    for attributes in value.edgedata.values() {
        attributes_payload(attributes, p);
    }
    attributes_payload(&value.default_vertex_attributes, p);
    attributes_payload(&value.default_face_attributes, p);
    attributes_payload(&value.default_edge_attributes, p);
    p.map(&value.triangulation);
    for triangles in value.triangulation.values() {
        p.vector(triangles);
    }
    p.map(&value.face_holes);
    for holes in value.face_holes.values() {
        p.vector(holes);
        for hole in holes {
            p.vector(hole);
        }
    }
    p.vector(&value.tri_tris);
    p.vector(&value.tri_vertices);
    for point in &value.tri_vertices {
        point_payload(point, p);
    }
    p.slice(value.get_pointcolors());
    p.slice(value.get_linecolors());
    p.slice(value.get_facecolors());
    p.slice(value.widths());
    color_names(value.get_pointcolors(), p);
    color_names(value.get_linecolors(), p);
    color_names(value.get_facecolors(), p);
    p.string(&value.objectcolor().name);
}
/// Count existing element geometry and public feature payloads without invoking lazy caches.
fn element_payload(value: &Element, p: &mut Payload) {
    use session_rust::element::ElementGeometry;
    p.string(&value.name);
    p.string(&value.element_type);
    p.vector(&value.element_data);
    p.vector(&value.features);
    p.vector(&value.insertion_vectors);
    for vector in &value.insertion_vectors {
        p.string(&vector.name);
    }
    if let Some(vector) = &value.dimensions {
        p.string(&vector.name);
    }
    for feature in &value.features {
        p.string(&feature.name);
        p.string(&feature.feature_type);
        p.vector(&feature.outlines);
        for outline in &feature.outlines {
            polyline_payload(outline, p);
        }
    }
    match value.geometry() {
        ElementGeometry::Mesh(mesh) => mesh_payload(mesh, p),
        ElementGeometry::BRep(brep) => brep_payload(brep, p),
        ElementGeometry::None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use session_rust::Xform;

    /// A document shares the source exactly as the loader and scene coordinator do.
    fn document(session: Rc<Session>) -> Doc {
        Doc {
            name: "memory fixture".into(),
            place: Xform::identity(),
            session,
            point_px: 0.0,
            display_only: false,
        }
    }

    /// Reserve spare coordinates so capacity accounting differs meaningfully from length.
    fn source() -> Rc<Session> {
        let mut line = Polyline::new(vec![Point::new(0.0, 0.0, 0.0), Point::new(1.0, 0.0, 0.0)]);
        line.coords.reserve(128);
        let mut session = Session::new("memory fixture");
        session.add_polyline(line, None);
        Rc::new(session)
    }

    /// Repeated document and lookup references count one owned geometry allocation.
    #[test]
    fn shared_documents_and_lookup_do_not_duplicate_geometry_payload() {
        let source = source();
        let mut cache = SourceCache::default();
        let mut docs = vec![document(Rc::clone(&source))];
        let once = cache.snapshot(&docs);
        assert!(once.known_bytes() >= once.vector_capacity_bytes);
        assert_eq!(once.unique_sessions, 1);
        assert_eq!(once.unique_geometry_values, 1);
        assert!(once.vector_capacity_bytes >= 128 * size_of::<f64>());
        assert_eq!(cache.snapshot(&docs).scans, once.scans);
        docs.push(document(Rc::clone(&source)));
        let twice = cache.snapshot(&docs);
        assert_eq!(twice.unique_sessions, 1);
        assert_eq!(twice.unique_geometry_values, 1);
        assert_eq!(twice.vector_capacity_bytes, once.vector_capacity_bytes);
        assert_eq!(twice.string_capacity_bytes, once.string_capacity_bytes);
        assert_eq!(twice.shared_value_bytes, once.shared_value_bytes);
    }

    /// Cache identities expire on replacement and never keep geometry alive.
    #[test]
    fn replacement_invalidates_without_retaining_the_old_source() {
        let mut docs = vec![document(source())];
        let old = Rc::downgrade(&docs[0].session);
        let mut cache = SourceCache::default();
        let before = cache.snapshot(&docs);
        assert_eq!(Rc::strong_count(&docs[0].session), 1);
        docs[0] = document(source());
        assert!(old.upgrade().is_none());
        let after = cache.snapshot(&docs);
        assert_eq!(after.scans, before.scans + 1);
        docs.clear();
        let empty = cache.snapshot(&docs);
        assert_eq!(empty.vector_capacity_bytes, 0);
        assert_eq!(empty.shared_value_bytes, 0);
        assert_eq!(empty.unique_sessions, 0);
    }

    /// Private cloud capacity must not be inferred from exposed occupied slices.
    #[test]
    fn cloud_borrowed_payload_is_separate_from_known_vector_capacity() {
        let cloud = PointCloud::from_coords(vec![0.0; 30], vec![255; 40], vec![0.0; 30]);
        let mut payload = Payload::default();
        cloud_payload(&cloud, &mut payload);
        assert_eq!(
            payload.exposed_slice_bytes,
            60 * size_of::<f64>() + 40 * size_of::<i32>()
        );
        assert_eq!(payload.vector_capacity_bytes, 0);
    }
}
