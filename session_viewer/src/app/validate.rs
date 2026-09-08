//! Viewer trust-boundary validation before kernel constructors allocate from serialized counts.
use prost::Message;
use session_rust::proto;

/// JSON constructors allocate from declared NURBS counts; check those against actual storage first.
pub fn json(text: &str) -> Result<(), String> {
    let source: serde_json::Value = serde_json::from_str(text).map_err(json_error)?;
    let mut pending = vec![(&source, 0usize)];
    while let Some((value, depth)) = pending.pop() {
        if depth > 64 {
            return Err("session JSON exceeds 64 nested levels".into());
        }
        match value {
            serde_json::Value::Object(fields) => {
                if fields.contains_key("control_points")
                    && (fields.contains_key("cv_count") || fields.contains_key("cv_count_u"))
                {
                    json_controls(value)?;
                }
                for child in fields.values() {
                    pending.push((child, depth + 1));
                }
            }
            serde_json::Value::Array(values) => {
                for child in values {
                    pending.push((child, depth + 1));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Keep parsing errors distinct from geometry validation errors in page feedback.
fn json_error(error: serde_json::Error) -> String {
    format!("invalid session JSON: {error}")
}

/// Read a bounded unsigned source count, with defaults matching the shared JSON constructors.
fn json_count(
    source: &serde_json::Value,
    key: &str,
    default: u64,
    maximum: u64,
) -> Result<usize, String> {
    let count = match source.get(key) {
        Some(value) => match value.as_u64() {
            Some(value) => value,
            None => return Err(format!("invalid JSON {key}")),
        },
        None => default,
    };
    if count > maximum {
        return Err(format!("JSON {key} exceeds its supported count"));
    }
    Ok(count as usize)
}

/// Compute the last addressed control component without overflowing malformed strides.
fn addressed_surface_controls(
    rows: usize,
    columns: usize,
    stride_u: usize,
    stride_v: usize,
    size: usize,
) -> Option<usize> {
    (rows - 1)
        .checked_mul(stride_u)?
        .checked_add((columns - 1).checked_mul(stride_v)?)?
        .checked_add(size)
}

/// JSON curves store nested controls; surfaces store a flat row-major array.
fn json_controls(source: &serde_json::Value) -> Result<(), String> {
    let controls = source["control_points"]
        .as_array()
        .ok_or("JSON controls must be an array")?;
    let dimension = json_count(source, "dimension", 3, 3)?;
    if dimension < 2 {
        return Err("invalid JSON control dimension".into());
    }
    let size = dimension + usize::from(source["is_rational"].as_bool().unwrap_or(false));
    if source.get("cv_count_u").is_some() {
        let rows = json_count(source, "cv_count_u", 0, 1_000_000)?;
        let columns = json_count(source, "cv_count_v", 0, 1_000_000)?;
        let total = rows as u64 * columns as u64;
        if total > 4_000_000 || controls.len() as u64 != total * size as u64 {
            return Err("JSON surface controls disagree with their declared counts".into());
        }
        json_axis(source, "order_u", rows, "nurbsknots_u")?;
        json_axis(source, "order_v", columns, "nurbsknots_v")?;
    } else {
        let count = json_count(source, "cv_count", 0, 1_000_000)?;
        let stride = json_count(source, "cv_stride", size as u64, 4)?;
        if count != controls.len() || stride != size {
            return Err("JSON curve controls disagree with their declared count or stride".into());
        }
        for control in controls {
            if control.as_array().map(Vec::len) != Some(size) {
                return Err("incomplete JSON curve control".into());
            }
        }
        json_axis(source, "order", count, "nurbsknots")?;
    }
    Ok(())
}

/// Apply the same knot-domain convention to JSON and protobuf records.
fn json_axis(
    source: &serde_json::Value,
    order_key: &str,
    count: usize,
    knots_key: &str,
) -> Result<(), String> {
    let order = json_count(source, order_key, 4, 64)?;
    let values = source[knots_key]
        .as_array()
        .ok_or("missing JSON NURBS knots")?;
    let mut knots = Vec::with_capacity(values.len());
    for value in values {
        knots.push(value.as_f64().ok_or("invalid JSON NURBS knot")?);
    }
    axis(order as i32, count as i32, &knots)?;
    Ok(())
}

/// JSON's serde path has no protobuf constructors; validate its prepared records before drawing.
pub fn retained(source: &session_rust::Session) -> Result<(), String> {
    for item in &source.objects.meshes {
        mesh(&item.to_proto())?;
    }
    for item in &source.objects.pointclouds {
        cloud(&item.to_proto())?;
    }
    for item in &source.objects.nurbscurves {
        curve(&item.to_proto())?;
    }
    for item in &source.objects.nurbssurfaces {
        surface(&item.to_proto())?;
    }
    for item in &source.objects.breps {
        brep(&item.to_proto())?;
    }
    Ok(())
}

/// Reject malformed serialized records before they can reach geometry preparation or the GPU.
pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
            + objects.meshes.len()
            + objects.pointclouds.len()
            + objects.nurbscurves.len()
            + objects.nurbssurfaces.len()
            + objects.breps.len()
            + objects.elements.len();
        if count > 2_000_000 {
            return Err("scene exceeds two million source objects".into());
        }
        for point in &objects.points {
            finite(&[point.x, point.y, point.z, point.width], "point")?;
        }
        for line in &objects.lines {
            if line.coords.len() != 6 {
                return Err("line needs exactly two endpoint triples".into());
            }
            finite(&line.coords, "line coordinates")?;
            finite(&line.dash, "line dashes")?;
        }
        for polyline in &objects.polylines {
            triples(&polyline.coords, "polyline coordinates")?;
        }
        for item in &objects.meshes {
            mesh(item)?;
        }
        for item in &objects.pointclouds {
            cloud(item)?;
        }
        for item in &objects.nurbscurves {
            curve(item)?;
        }
        for item in &objects.nurbssurfaces {
            surface(item)?;
        }
        for item in &objects.breps {
            brep(item)?;
        }
        for item in &objects.elements {
            match item.geometry_type.as_str() {
                "Mesh" => mesh(
                    &proto::Mesh::decode(item.geometry_data.as_slice()).map_err(decode_error)?,
                )?,
                "BRep" => brep(
                    &proto::BRep::decode(item.geometry_data.as_slice()).map_err(decode_error)?,
                )?,
                _ => {}
            }
        }
    }
    for entry in &source.xforms {
        let Some(transform) = &entry.xform else {
            return Err("missing object transform".into());
        };
        finite(&transform.matrix, "object transform")?;
        if transform.matrix.len() != 16
            || transform.matrix[3] != 0.0
            || transform.matrix[7] != 0.0
            || transform.matrix[11] != 0.0
            || transform.matrix[15] != 1.0
        {
            return Err("object placement must be a sixteen-value affine matrix".into());
        }
    }
    if let Some(tree) = &source.tree
        && let Some(root) = &tree.root
    {
        let mut pending = vec![(root, 0usize)];
        while let Some((node, depth)) = pending.pop() {
            if depth > 64 {
                return Err("scene hierarchy exceeds 64 levels".into());
            }
            for child in &node.children {
                pending.push((child, depth + 1));
            }
        }
    }
    Ok(())
}

/// Nested element protobuf errors keep their context at the browser boundary.
fn decode_error(error: prost::DecodeError) -> String {
    format!("invalid element geometry: {error}")
}

/// All stored scalars must be finite; valid degenerate geometry is handled by its producer.
fn finite(values: &[f64], label: &str) -> Result<(), String> {
    for value in values {
        if !value.is_finite() {
            return Err(format!("{label}: non-finite value"));
        }
    }
    Ok(())
}

/// Packed source positions have complete triples, including a valid empty array.
fn triples(values: &[f64], label: &str) -> Result<(), String> {
    if !values.len().is_multiple_of(3) {
        return Err(format!("{label}: incomplete coordinate triple"));
    }
    finite(values, label)
}

/// NURBS counts are checked against actual knot storage before allocation and evaluation.
fn axis(order: i32, count: i32, knots: &[f64]) -> Result<usize, String> {
    if !(2..=64).contains(&order) || count < order || count > 1_000_000 {
        return Err("invalid or oversized NURBS order/control count".into());
    }
    if knots.len() != (order + count - 2) as usize {
        return Err("NURBS knot count does not match its order and controls".into());
    }
    finite(knots, "NURBS knots")?;
    for pair in knots.windows(2) {
        if pair[0] > pair[1] {
            return Err("NURBS knots are not monotonic".into());
        }
    }
    if knots[(order - 2) as usize] >= knots[(count - 1) as usize] {
        return Err("NURBS parameter domain is empty".into());
    }
    Ok(count as usize)
}

/// Curves use tightly packed Euclidean or homogeneous source controls.
fn curve(source: &proto::NurbsCurve) -> Result<(), String> {
    if !(2..=3).contains(&source.dimension) {
        return Err("NURBS curve dimension must be 2 or 3".into());
    }
    let count = axis(source.order, source.cv_count, &source.nurbsknots)?;
    let size = source.dimension as usize + usize::from(source.is_rational);
    if source.cvs.len() != count * size {
        return Err("NURBS curve control storage is incomplete".into());
    }
    finite(&source.cvs, "NURBS curve controls")
}

/// Validate both surface strides and the final addressed control before allocation.
fn surface(source: &proto::NurbsSurface) -> Result<(), String> {
    if source.dimension != 3 {
        return Err("NURBS surface dimension must be 3".into());
    }
    let rows = axis(source.order_u, source.cv_count_u, &source.nurbsknots_u)?;
    let columns = axis(source.order_v, source.cv_count_v, &source.nurbsknots_v)?;
    if rows.saturating_mul(columns) > 4_000_000 {
        return Err("surface exceeds four million controls".into());
    }
    let size = 3 + usize::from(source.is_rational);
    let stride_u = if source.cv_stride_u > 0 {
        source.cv_stride_u as usize
    } else {
        columns * size
    };
    let stride_v = if source.cv_stride_v > 0 {
        source.cv_stride_v as usize
    } else {
        size
    };
    let addressed = addressed_surface_controls(rows, columns, stride_u, stride_v, size);
    if !matches!(addressed, Some(length) if length <= source.cvs.len()) {
        return Err("NURBS surface control storage is incomplete".into());
    }
    finite(&source.cvs, "NURBS surface controls")?;
    if let Some(cached) = &source.cached_mesh {
        mesh(cached)?;
    }
    Ok(())
}

/// Mesh faces and cached triangles must address real source vertices, never absent keys.
fn mesh(source: &proto::Mesh) -> Result<(), String> {
    for (key, vertex) in &source.vertices {
        if *key > u32::MAX as u64 {
            return Err("mesh vertex key exceeds the browser index range".into());
        }
        finite(&[vertex.x, vertex.y, vertex.z], "mesh vertex")?;
        for value in vertex.attributes.values() {
            if !value.is_finite() {
                return Err("mesh vertex has a non-finite attribute".into());
            }
        }
    }
    for face in source.faces.values() {
        if face.vertices.len() < 3 {
            return Err("mesh face has fewer than three vertices".into());
        }
        mesh_indices(source, &face.vertices)?;
        for hole in &face.holes {
            mesh_indices(source, &hole.vertices)?;
        }
    }
    for (face, triangles) in &source.triangulation {
        if !source.faces.contains_key(face) || !triangles.vertices.len().is_multiple_of(3) {
            return Err("invalid mesh triangulation record".into());
        }
        mesh_indices(source, &triangles.vertices)?;
    }
    Ok(())
}

/// Every referenced index must exist before the shared mesher can dereference it.
fn mesh_indices(source: &proto::Mesh, indices: &[u64]) -> Result<(), String> {
    for index in indices {
        if !source.vertices.contains_key(index) {
            return Err(format!("mesh references missing vertex {index}"));
        }
    }
    Ok(())
}

/// Point attributes and octree arrays must describe the same source ordering.
fn cloud(source: &proto::PointCloud) -> Result<(), String> {
    triples(&source.coords, "point cloud")?;
    finite(&source.normals, "point normals")?;
    let points = source.coords.len() / 3;
    if (!source.normals.is_empty() && source.normals.len() != source.coords.len())
        || (!source.colors.is_empty() && source.colors.len() != points * 4)
        || (!source.point_ids.is_empty() && source.point_ids.len() != points)
    {
        return Err("point-cloud attribute counts disagree".into());
    }
    let nodes = source.lod_size.len();
    if nodes > 0 {
        if source.lod_min.len() != nodes * 3
            || source.lod_spacing.len() != nodes
            || source.lod_level.len() != nodes
            || source.lod_first.len() != nodes
            || source.lod_count.len() != nodes
            || source.lod_children.len() != nodes * 8
        {
            return Err("point-cloud octree arrays disagree".into());
        }
        finite(&source.lod_min, "octree bounds")?;
        finite(&source.lod_size, "octree sizes")?;
        for node in 0..nodes {
            if source.lod_first[node] < 0
                || source.lod_count[node] < 0
                || source.lod_first[node] as u64 + source.lod_count[node] as u64 > points as u64
            {
                return Err("octree range exceeds its source points".into());
            }
        }
        for child in &source.lod_children {
            if *child < -1 || *child >= nodes as i32 {
                return Err("octree references an absent child".into());
            }
        }
    }
    Ok(())
}

/// BRep constructors receive structurally valid underlying analytic geometry.
fn brep(source: &proto::BRep) -> Result<(), String> {
    for item in &source.curves_2d {
        curve(item)?;
    }
    for item in &source.curves_3d {
        curve(item)?;
    }
    for item in &source.surfaces {
        surface(item)?;
    }
    for vertex in &source.vertices {
        if let Some(point) = &vertex.point {
            finite(&[point.x, point.y, point.z], "BRep vertex")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hostile_counts_do_not_reach_kernel_allocations() {
        let source = proto::NurbsCurve {
            dimension: 3,
            order: 2,
            cv_count: i32::MAX,
            ..Default::default()
        };
        assert!(curve(&source).is_err());
        let source = proto::NurbsSurface {
            dimension: 3,
            order_u: 2,
            order_v: 2,
            cv_count_u: -1,
            cv_count_v: 2,
            ..Default::default()
        };
        assert!(surface(&source).is_err());
    }
    #[test]
    fn missing_indices_and_partial_cloud_attributes_are_errors() {
        let mut source = proto::Mesh::default();
        source.faces.insert(
            0,
            proto::FaceData {
                vertices: vec![0, 1, 2],
                ..Default::default()
            },
        );
        assert!(mesh(&source).is_err());
        let cloud_source = proto::PointCloud {
            coords: vec![0., 0., 0.],
            point_ids: vec![7, 8],
            ..Default::default()
        };
        assert!(cloud(&cloud_source).is_err());
    }

    #[test]
    fn json_declared_controls_are_checked_before_constructor_allocation() {
        assert!(
            json(r#"{"control_points":[],"cv_count":18446744073709551615,"dimension":3}"#).is_err()
        );
        assert!(
            json(r#"{"control_points":[],"cv_count_u":1000000,"cv_count_v":1000000}"#).is_err()
        );
        assert!(json(r#"{"control_points":[[0,0,0],[1,0,0]],"cv_count":2,"dimension":3,"order":2,"nurbsknots":[0,1]}"#).is_ok());
    }
}
