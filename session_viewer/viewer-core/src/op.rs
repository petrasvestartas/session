//! Typed, serde-serializable operation layer shared by the command line, native CLI,
//! browser `execute_op`, and the Python FastMCP server.
//!
//! Every operation executes against `&mut dyn CommandHost`, keeping `session_rust` as the
//! single geometry kernel and `Scene` as the single scene model.

use crate::app::clipping;
use crate::app::command::tool::shape::{Answer, Frame, Shape};
use crate::app::command::verbs;
use crate::app::command::{CommandHost, ViewFlag};
use crate::app::cplane::CPlane;
#[cfg(not(target_arch = "wasm32"))]
use crate::app::manifest::Manifest;
use crate::app::modeling::{Interval, check_point};
use crate::app::scene::{FileDoc, READ_ONLY, Scene};
#[cfg(not(target_arch = "wasm32"))]
use crate::app::session_io::open_with_state;
use crate::app::session_io::{SavedCamera, save_with_state};
use crate::camera::View;
use crate::rows::{Instance, TextLabel, TextPlacement};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use session_rust::{AABB, Arrowhead, Geometry, Plane, Point, Polyline, Session, Vector, Xform};
use std::rc::Rc;

/// Envelope for one JSON operation request (e.g. over JSONL stdio or `postMessage`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(flatten)]
    pub op: Op,
}

/// Envelope for one JSON operation reply or structured error.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub ok: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: Option<Reply>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OpError>,
}

/// Structured success payload returned by `execute`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reply {
    pub message: String,
    #[serde(default)]
    pub created: Vec<ObjectRef>,
    #[serde(default)]
    pub changed: Vec<ObjectRef>,
    #[serde(default)]
    pub value: Value,
}

impl Reply {
    fn msg(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            created: Vec::new(),
            changed: Vec::new(),
            value: Value::Null,
        }
    }

    fn val(message: impl Into<String>, value: Value) -> Self {
        Self {
            message: message.into(),
            created: Vec::new(),
            changed: Vec::new(),
            value,
        }
    }
}

/// Machine-readable error categories returned by `execute`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    InvalidArgument,
    NotFound,
    Locked,
    NoSelection,
    NotRetained,
    Busy,
    TooLarge,
    Unsupported,
}

/// Structured error returned by `execute`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpError {
    pub code: ErrorCode,
    pub message: String,
}

impl OpError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn classify(message: impl Into<String>) -> Self {
        let message = message.into();
        let lower = message.to_ascii_lowercase();
        let code = if lower.contains("nothing is selected")
            || lower.contains("select objects first")
            || lower.contains("select an object")
            || lower.contains("select one object")
            || lower.contains("none is selected")
        {
            ErrorCode::NoSelection
        } else if lower.contains("locked") {
            ErrorCode::Locked
        } else if lower.contains("display only") || lower.contains("not retained") {
            ErrorCode::NotRetained
        } else if lower.contains("try again") || lower.contains("loading") {
            ErrorCode::Busy
        } else if lower.contains("512 mib")
            || lower.contains("too many")
            || lower.contains("at most")
        {
            ErrorCode::TooLarge
        } else if lower.contains("no longer exists")
            || lower.contains("is gone")
            || lower.contains("not found")
            || lower.contains("nothing matches")
        {
            ErrorCode::NotFound
        } else if lower.contains("native-only") {
            ErrorCode::Unsupported
        } else {
            ErrorCode::InvalidArgument
        };
        Self { code, message }
    }
}

/// Stable document reference by index or display name.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocRef {
    Index(usize),
    Name(String),
}

impl DocRef {
    pub fn resolve(&self, scene: &Scene) -> Result<usize, OpError> {
        match self {
            DocRef::Index(i) => (*i < scene.docs.len()).then_some(*i).ok_or_else(|| {
                OpError::new(
                    ErrorCode::NotFound,
                    format!(
                        "document index {i} out of range ({} open)",
                        scene.docs.len()
                    ),
                )
            }),
            DocRef::Name(name) => {
                scene
                    .docs
                    .iter()
                    .position(|d| d.name == *name)
                    .ok_or_else(|| {
                        OpError::new(ErrorCode::NotFound, format!("document `{name}` not found"))
                    })
            }
        }
    }
}

/// Stable object address `(doc, guid)` — never a volatile row or tree index.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectRef {
    pub doc: DocRef,
    pub guid: String,
}

impl ObjectRef {
    pub fn of(doc: usize, guid: impl Into<String>) -> Self {
        Self {
            doc: DocRef::Index(doc),
            guid: guid.into(),
        }
    }
}

/// Stable layer address `(doc, name)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<DocRef>,
    pub name: String,
}

/// Target objects for an operation: current selection or explicit `(doc, guid)` references.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Targets {
    #[default]
    Selection,
    Refs(Vec<ObjectRef>),
}

/// Geometry creation variants for `Op::ObjectCreate`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateKind {
    Point {
        point: [f64; 3],
    },
    Line {
        start: [f64; 3],
        end: [f64; 3],
    },
    Arrow {
        start: [f64; 3],
        end: [f64; 3],
    },
    Polyline {
        points: Vec<[f64; 3]>,
        #[serde(default)]
        closed: bool,
    },
    Curve {
        points: Vec<[f64; 3]>,
        #[serde(default)]
        closed: bool,
    },
    Circle {
        center: [f64; 3],
        radius: f64,
    },
    Box {
        origin: [f64; 3],
        dx: f64,
        dy: f64,
        dz: f64,
        #[serde(default)]
        mesh: bool,
    },
    Sphere {
        center: [f64; 3],
        radius: f64,
        #[serde(default)]
        mesh: bool,
    },
    Cylinder {
        origin: [f64; 3],
        radius: f64,
        height: f64,
        #[serde(default)]
        mesh: bool,
    },
    Cone {
        origin: [f64; 3],
        radius: f64,
        height: f64,
        #[serde(default)]
        mesh: bool,
    },
    Torus {
        center: [f64; 3],
        major_radius: f64,
        minor_radius: f64,
        #[serde(default)]
        mesh: bool,
    },
    Text {
        text: String,
        origin: [f64; 3],
        #[serde(default)]
        height: Option<f64>,
    },
    BoundingBox,
}

/// World-axis or arbitrary-vector axis specification.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisSpec {
    X,
    Y,
    Z,
    Vector([f64; 3]),
}

impl AxisSpec {
    fn to_unit_vector(&self) -> Result<Vector, OpError> {
        let v = match self {
            AxisSpec::X => Vector::x_axis(),
            AxisSpec::Y => Vector::y_axis(),
            AxisSpec::Z => Vector::z_axis(),
            AxisSpec::Vector([x, y, z]) => {
                if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "axis components must be finite",
                    ));
                }
                let mut v = Vector::new(*x, *y, *z);
                if v.magnitude() <= 1e-12 || !v.normalize_self() {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "axis vector must be non-zero",
                    ));
                }
                v
            }
        };
        Ok(v)
    }
}

/// Explicit world-space transformation specification (no implicit UI pivot).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum XformSpec {
    Translate {
        delta: [f64; 3],
    },
    Rotate {
        axis: AxisSpec,
        degrees: f64,
        #[serde(default)]
        pivot: Option<[f64; 3]>,
    },
    Scale {
        factor: [f64; 3],
        #[serde(default)]
        pivot: Option<[f64; 3]>,
    },
    Orient3 {
        from: [[f64; 3]; 3],
        to: [[f64; 3]; 3],
    },
    Matrix {
        m: [f64; 16],
    },
}

impl XformSpec {
    pub fn to_xform(&self) -> Result<(Xform, &'static str), OpError> {
        match self {
            XformSpec::Translate {
                delta: [dx, dy, dz],
            } => {
                check_point([*dx, *dy, *dz]).map_err(OpError::classify)?;
                Ok((Xform::translation(*dx, *dy, *dz), "move"))
            }
            XformSpec::Rotate {
                axis,
                degrees,
                pivot,
            } => {
                if !degrees.is_finite() {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "rotation degrees must be finite",
                    ));
                }
                let dir = axis.to_unit_vector()?;
                let rot = Xform::rotation(&dir, *degrees, true);
                let xf = if let Some(p) = pivot {
                    let pt = check_point(*p).map_err(OpError::classify)?;
                    &(&Xform::translation(pt[0], pt[1], pt[2]) * &rot)
                        * &Xform::translation(-pt[0], -pt[1], -pt[2])
                } else {
                    rot
                };
                Ok((xf, "rotate"))
            }
            XformSpec::Scale { factor, pivot } => {
                if factor.iter().any(|v| !v.is_finite() || *v <= 0.0) {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "scale factors must be positive and finite",
                    ));
                }
                let sc = Xform::scale_xyz(factor[0], factor[1], factor[2]);
                let xf = if let Some(p) = pivot {
                    let pt = check_point(*p).map_err(OpError::classify)?;
                    &(&Xform::translation(pt[0], pt[1], pt[2]) * &sc)
                        * &Xform::translation(-pt[0], -pt[1], -pt[2])
                } else {
                    sc
                };
                Ok((xf, "scale"))
            }
            XformSpec::Orient3 { from, to } => {
                let frame = |pts: &[[f64; 3]; 3]| -> Result<Plane, OpError> {
                    let a = check_point(pts[0]).map_err(OpError::classify)?;
                    let b = check_point(pts[1]).map_err(OpError::classify)?;
                    let c = check_point(pts[2]).map_err(OpError::classify)?;
                    let x = &b - &a;
                    let y = &c - &a;
                    if x.magnitude() <= 1e-12
                        || x.cross(&y).magnitude() <= 1e-9 * x.magnitude() * y.magnitude()
                    {
                        return Err(OpError::new(
                            ErrorCode::InvalidArgument,
                            "orient3 points must not be collinear",
                        ));
                    }
                    Ok(Plane::from_points(vec![a, b, c]))
                };
                let src = frame(from)?;
                let dst = frame(to)?;
                Ok((Xform::plane_to_plane(&src, &dst), "orient"))
            }
            XformSpec::Matrix { m } => {
                if m.iter().any(|v| !v.is_finite()) {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "matrix entries must be finite",
                    ));
                }
                Ok((Xform::from_matrix(*m), "transform"))
            }
        }
    }
}

/// Plane specification for `ObjectProject`, `ConstructionPlaneSet`, and `ClippingPlaneAdd`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlaneSpec {
    Xy {
        #[serde(default)]
        z: f64,
    },
    Yz {
        #[serde(default)]
        x: f64,
    },
    Zx {
        #[serde(default)]
        y: f64,
    },
    CPlane,
    Points {
        a: [f64; 3],
        b: [f64; 3],
        c: [f64; 3],
    },
    Normal {
        origin: [f64; 3],
        normal: [f64; 3],
    },
}

/// Curve arrowhead mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadSpec {
    None,
    Start,
    End,
    Both,
}

impl HeadSpec {
    fn as_kernel(self) -> (Arrowhead, &'static str) {
        match self {
            HeadSpec::None => (Arrowhead::NONE, "None"),
            HeadSpec::Start => (Arrowhead::START, "Start"),
            HeadSpec::End => (Arrowhead::END, "End"),
            HeadSpec::Both => (Arrowhead::BOTH, "Both"),
        }
    }
}

/// Selection update mode.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetMode {
    #[default]
    Replace,
    Add,
    Remove,
}

/// Object query filter for `SelectionQuery` and `ObjectList`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Filter {
    All,
    Name { substring: String },
    SmallerThan { max_diagonal: f64 },
    InBox { min: [f64; 3], max: [f64; 3] },
    Kind { kind: String },
}

/// Measurement mode for `Op::Measure`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MeasureKind {
    Length,
    Area,
    Volume,
    BoundsExtent,
    Distance { a: [f64; 3], b: [f64; 3] },
}

/// Partial update to viewport display settings.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ViewPatch {
    #[serde(default)]
    pub arctic: Option<bool>,
    #[serde(default)]
    pub wireframe: Option<bool>,
    #[serde(default)]
    pub outlines: Option<bool>,
    #[serde(default)]
    pub points: Option<bool>,
    #[serde(default)]
    pub lines: Option<bool>,
    #[serde(default)]
    pub edges: Option<bool>,
    #[serde(default)]
    pub lighting: Option<bool>,
    #[serde(default)]
    pub backfaces: Option<bool>,
    #[serde(default)]
    pub cull_backfaces: Option<bool>,
    #[serde(default)]
    pub opacity: Option<f32>,
    #[serde(default)]
    pub cloud_size: Option<f32>,
    #[serde(default)]
    pub plane_size: Option<f64>,
    #[serde(default)]
    pub attributes: Option<bool>,
    #[serde(default)]
    pub interactions: Option<bool>,
}

/// Every typed operation supported by `viewer-core::op::execute`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Op {
    // Create & edit
    ObjectCreate {
        kind: CreateKind,
        #[serde(default)]
        layer: Option<LayerRef>,
    },
    ObjectDelete {
        #[serde(default)]
        targets: Targets,
    },
    ObjectTransform {
        #[serde(default)]
        targets: Targets,
        xform: XformSpec,
    },
    ObjectCopy {
        #[serde(default)]
        targets: Targets,
        xform: XformSpec,
    },
    ObjectExplode {
        #[serde(default)]
        targets: Targets,
    },
    ObjectProject {
        #[serde(default)]
        targets: Targets,
        plane: PlaneSpec,
    },
    CurveInterval {
        target: ObjectRef,
        t0: f64,
        t1: f64,
        #[serde(default)]
        extend: bool,
    },
    CurveArrowhead {
        #[serde(default)]
        targets: Targets,
        head: HeadSpec,
    },
    // Selection
    SelectionSet {
        targets: Targets,
        #[serde(default)]
        mode: SetMode,
    },
    SelectionQuery {
        by: Filter,
    },
    SelectionClear,
    // Layers, groups, graph & appearance
    LayerList,
    LayerCreate {
        #[serde(default)]
        parent: Option<LayerRef>,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        sublayer: bool,
    },
    LayerRename {
        layer: LayerRef,
        name: String,
    },
    LayerDelete {
        layer: LayerRef,
    },
    LayerSetCurrent {
        layer: LayerRef,
    },
    LayerAssign {
        #[serde(default)]
        targets: Targets,
        layer: LayerRef,
    },
    LayerSet {
        layer: LayerRef,
        #[serde(default)]
        visible: Option<bool>,
        #[serde(default)]
        locked: Option<bool>,
        #[serde(default)]
        color: Option<Option<[u8; 3]>>,
    },
    GroupAdd {
        #[serde(default)]
        targets: Targets,
        #[serde(default)]
        name: String,
    },
    GraphAddEdge {
        a: ObjectRef,
        b: ObjectRef,
    },
    VisibilitySet {
        #[serde(default)]
        targets: Targets,
        visible: bool,
    },
    ShowAll,
    LockSet {
        #[serde(default)]
        targets: Targets,
        locked: bool,
    },
    UnlockAll,
    ColorSet {
        #[serde(default)]
        targets: Targets,
        #[serde(default)]
        edge: bool,
        color: Option<[u8; 3]>,
    },
    AlphaSet {
        #[serde(default)]
        targets: Targets,
        alpha: Option<f32>,
    },
    // Query & measure
    SceneInspect,
    ObjectList {
        #[serde(default)]
        filter: Option<Filter>,
    },
    ObjectGet {
        target: ObjectRef,
    },
    Measure {
        what: MeasureKind,
        #[serde(default)]
        targets: Targets,
    },
    // Document & history I/O
    SceneOpen {
        path: String,
    },
    SceneSave {
        #[serde(default)]
        path: Option<String>,
    },
    SceneLoadManifest {
        path: String,
    },
    SceneImport {
        path: String,
    },
    SceneExportObj {
        #[serde(default)]
        path: Option<String>,
    },
    HistoryUndo,
    HistoryRedo,
    // Camera, view & clipping
    CameraGet,
    CameraSetView {
        view: String,
    },
    CameraSetPose {
        target: [f64; 3],
        position: [f64; 3],
        #[serde(default)]
        perspective: Option<bool>,
    },
    CameraFit {
        #[serde(default)]
        targets: Option<Targets>,
    },
    ViewGet,
    ViewSet {
        patch: ViewPatch,
    },
    ConstructionPlaneSet {
        plane: Option<PlaneSpec>,
    },
    ClippingPlaneAdd {
        plane: PlaneSpec,
    },
    ClippingSet {
        #[serde(default)]
        enabled: Option<bool>,
        #[serde(default)]
        solid_fill: Option<bool>,
        #[serde(default)]
        flip: bool,
    },
    RenderFrame {
        width: u32,
        height: u32,
        path: String,
    },
    PerfStats,
    // Escape hatch so every command-line verb is reachable
    CommandRun {
        line: String,
    },
}

/// Execute one `Request` and wrap the outcome in a `Response` carrying the request `id`.
pub fn execute_request(host: &mut dyn CommandHost, req: Request) -> Response {
    match execute(host, req.op) {
        Ok(reply) => Response {
            id: req.id,
            ok: true,
            reply: Some(reply),
            error: None,
        },
        Err(error) => Response {
            id: req.id,
            ok: false,
            reply: None,
            error: Some(error),
        },
    }
}

/// Parse a JSON `Request`, execute it against `host`, and serialize the `Response` as one JSON line.
pub fn execute_json(host: &mut dyn CommandHost, json_line: &str) -> String {
    let parsed: Result<Request, _> = serde_json::from_str(json_line);
    let resp = match parsed {
        Ok(req) => execute_request(host, req),
        Err(err) => Response {
            id: None,
            ok: false,
            reply: None,
            error: Some(OpError::new(
                ErrorCode::InvalidArgument,
                format!("invalid JSON operation request: {err}"),
            )),
        },
    };
    serde_json::to_string(&resp).unwrap_or_else(|e| {
        format!(
            r#"{{"ok":false,"error":{{"code":"invalid_argument","message":"serialization error: {e}"}}}}"#
        )
    })
}

fn resolve_rows(host: &dyn CommandHost, targets: &Targets) -> Result<Vec<u32>, OpError> {
    match targets {
        Targets::Selection => {
            let rows = host.selected_rows();
            if rows.is_empty() {
                return Err(OpError::new(ErrorCode::NoSelection, "nothing is selected"));
            }
            Ok(rows)
        }
        Targets::Refs(refs) => {
            if refs.is_empty() {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "empty target list",
                ));
            }
            let scene = host.scene();
            let mut rows = Vec::with_capacity(refs.len());
            for r in refs {
                let doc = r.doc.resolve(scene)?;
                let row = scene.row_of(doc, &r.guid).ok_or_else(|| {
                    OpError::new(
                        ErrorCode::NotFound,
                        format!("object `{}` not found in document {doc}", r.guid),
                    )
                })?;
                rows.push(row);
            }
            Ok(rows)
        }
    }
}

fn refs_of_rows(scene: &Scene, rows: &[u32]) -> Vec<ObjectRef> {
    rows.iter()
        .filter_map(|&row| {
            let (doc, guid) = scene.identity_of(row)?;
            Some(ObjectRef::of(doc, guid.as_ref()))
        })
        .collect()
}

fn resolve_layer_doc(scene: &Scene, layer: &LayerRef) -> Result<usize, OpError> {
    if let Some(doc_ref) = &layer.doc {
        return doc_ref.resolve(scene);
    }
    if let Some((doc, _)) = scene.current_layer() {
        return Ok(doc);
    }
    if !scene.docs.is_empty() {
        return Ok(0);
    }
    Err(OpError::new(
        ErrorCode::NotFound,
        "no open document holds layers",
    ))
}

fn build_shape_geometry(
    shape: &Shape,
    answers: &[Answer],
    mesh: bool,
) -> Result<Geometry, OpError> {
    let Some(Answer::Point(origin)) = answers.first() else {
        return Err(OpError::new(
            ErrorCode::InvalidArgument,
            format!("{} requires an origin point", shape.name),
        ));
    };
    let xy = Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), Vector::z_axis(), None);
    let frame = Frame::new(&xy, origin.clone());
    let opt = if mesh { "Mesh" } else { "Brep" };
    let part = (shape.read)(&frame, answers, opt).map_err(OpError::classify)?;
    (shape.build)(&part, opt).map_err(OpError::classify)
}

fn match_filter(host: &dyn CommandHost, filter: &Filter) -> Result<Vec<u32>, OpError> {
    let scene = host.scene();
    let mut out = Vec::new();
    match filter {
        Filter::All => {
            for row in 0..scene.row_count() as u32 {
                if scene.identity_of(row).is_some()
                    && scene.selectable(row)
                    && host
                        .row_flags(row)
                        .is_none_or(|f| f & (Instance::FLAG_HIDDEN | Instance::FLAG_DEAD) == 0)
                {
                    out.push(row);
                }
            }
        }
        Filter::Name { substring } => {
            let needle = substring.to_ascii_lowercase();
            for row in 0..scene.row_count() as u32 {
                let Some((doc, guid)) = scene.identity_of(row) else {
                    continue;
                };
                if !scene.selectable(row) {
                    continue;
                }
                let name = scene.object_name(row).to_ascii_lowercase();
                let doc_name = scene
                    .docs
                    .get(doc)
                    .map(|d| d.name.to_ascii_lowercase())
                    .unwrap_or_default();
                if name.contains(&needle)
                    || guid.to_ascii_lowercase().contains(&needle)
                    || doc_name.contains(&needle)
                {
                    out.push(row);
                }
            }
        }
        Filter::SmallerThan { max_diagonal } => {
            if !max_diagonal.is_finite() || *max_diagonal <= 0.0 {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "max_diagonal must be positive and finite",
                ));
            }
            for row in 0..scene.row_count() as u32 {
                if scene.identity_of(row).is_none() || !scene.selectable(row) {
                    continue;
                }
                if let Some(b) = host.row_bounds(row) {
                    let diag = 2.0 * b.hx.hypot(b.hy).hypot(b.hz);
                    if diag <= *max_diagonal {
                        out.push(row);
                    }
                }
            }
        }
        Filter::InBox { min, max } => {
            let lo = check_point(*min).map_err(OpError::classify)?;
            let hi = check_point(*max).map_err(OpError::classify)?;
            for row in 0..scene.row_count() as u32 {
                if scene.identity_of(row).is_none() || !scene.selectable(row) {
                    continue;
                }
                if let Some(b) = host.row_bounds(row) {
                    let bmin = b.min_point();
                    let bmax = b.max_point();
                    let overlaps = bmin[0] <= hi[0]
                        && bmax[0] >= lo[0]
                        && bmin[1] <= hi[1]
                        && bmax[1] >= lo[1]
                        && bmin[2] <= hi[2]
                        && bmax[2] >= lo[2];
                    if overlaps {
                        out.push(row);
                    }
                }
            }
        }
        Filter::Kind { kind } => {
            let needle = kind.trim().to_ascii_lowercase();
            for row in 0..scene.row_count() as u32 {
                if scene.identity_of(row).is_none() {
                    continue;
                }
                let gk = geometry_kind_name(scene.geometry(row));
                if gk.eq_ignore_ascii_case(&needle) {
                    out.push(row);
                }
            }
        }
    }
    Ok(out)
}

fn geometry_kind_name(geom: Option<&Geometry>) -> &'static str {
    match geom {
        Some(Geometry::Point(_)) => "point",
        Some(Geometry::Line(_)) => "line",
        Some(Geometry::Polyline(_)) => "polyline",
        Some(Geometry::NurbsCurve(_)) => "nurbs_curve",
        Some(Geometry::NurbsSurface(_)) => "nurbs_surface",
        Some(Geometry::Mesh(_)) => "mesh",
        Some(Geometry::BRep(_)) => "brep",
        Some(Geometry::Element(_)) => "element",
        Some(Geometry::PointCloud(_)) => "point_cloud",
        Some(Geometry::Plane(_)) => "plane",
        Some(Geometry::OBB(_)) => "obb",
        None => "text_or_shell",
    }
}

fn object_summary(host: &dyn CommandHost, row: u32) -> Option<Value> {
    let scene = host.scene();
    let (doc, guid) = scene.identity_of(row)?;
    let key = (doc, guid.clone());
    let bounds = host.row_bounds(row).map(|b| {
        json!({
            "center": [b.cx, b.cy, b.cz],
            "half": [b.hx, b.hy, b.hz],
        })
    });
    let layer = scene
        .node_of(row)
        .and_then(|(n, _)| n.borrow().parent())
        .map(|p| p.borrow().name.clone());
    Some(json!({
        "doc": doc,
        "doc_name": scene.docs.get(doc).map(|d| d.name.as_str()),
        "guid": guid.as_ref(),
        "name": scene.object_name(row),
        "kind": geometry_kind_name(scene.geometry(row)),
        "layer": layer,
        "hidden": scene.hidden.contains(&key),
        "locked": scene.locked.contains(&key),
        "color": scene.colors.get(&key),
        "edge_color": scene.edge_colors.get(&key),
        "alpha": scene.alphas.get(&key),
        "bounds": bounds,
    }))
}

fn plane_from_spec(host: &dyn CommandHost, spec: &PlaneSpec) -> Result<Plane, OpError> {
    match spec {
        PlaneSpec::Xy { z } => Ok(Plane::from_point_normal(
            Point::new(0.0, 0.0, *z),
            Vector::z_axis(),
            None,
        )),
        PlaneSpec::Yz { x } => Ok(Plane::from_point_normal(
            Point::new(*x, 0.0, 0.0),
            Vector::x_axis(),
            None,
        )),
        PlaneSpec::Zx { y } => Ok(Plane::from_point_normal(
            Point::new(0.0, *y, 0.0),
            Vector::y_axis(),
            None,
        )),
        PlaneSpec::CPlane => {
            let n =
                CPlane::facing(&host.camera().orientation.rotate_vector(Vector::y_axis())).normal();
            Ok(Plane::from_point_normal(Point::new(0.0, 0.0, 0.0), n, None))
        }
        PlaneSpec::Points { a, b, c } => {
            let pa = check_point(*a).map_err(OpError::classify)?;
            let pb = check_point(*b).map_err(OpError::classify)?;
            let pc = check_point(*c).map_err(OpError::classify)?;
            let (x, y) = (&pb - &pa, &pc - &pa);
            if x.magnitude() <= 1e-12
                || x.cross(&y).magnitude() <= 1e-9 * x.magnitude() * y.magnitude()
            {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "plane points must not be collinear",
                ));
            }
            Ok(Plane::from_points(vec![pa, pb, pc]))
        }
        PlaneSpec::Normal { origin, normal } => {
            let o = check_point(*origin).map_err(OpError::classify)?;
            let n = AxisSpec::Vector(*normal).to_unit_vector()?;
            Ok(Plane::from_point_normal(o, n, None))
        }
    }
}

fn project_geometry_to_plane(geom: &Geometry, place: &Xform, plane: &Plane) -> Option<Geometry> {
    let inv = place.inverse()?;
    let o = plane.origin();
    let mut n = plane.z_axis();
    let _ = n.normalize_self();
    let proj = |p: &Point| -> Point {
        let w = p.transformed(place);
        let d = (&w - &o).dot(&n);
        let flat = Point::new(w[0] - d * n[0], w[1] - d * n[1], w[2] - d * n[2]);
        flat.transformed(&inv)
    };
    match geom {
        Geometry::Point(p) => {
            let mut next = proj(p);
            next.name = p.name.clone();
            Some(Geometry::Point(Rc::new(next)))
        }
        Geometry::Line(l) => {
            let a = proj(&l.start());
            let b = proj(&l.end());
            let mut next = session_rust::Line::from_points(&a, &b);
            next.name = l.name.clone();
            next.arrowhead = l.arrowhead;
            next.width = l.width;
            next.linecolor = l.linecolor.clone();
            Some(Geometry::Line(Rc::new(next)))
        }
        Geometry::Polyline(pl) => {
            let pts: Vec<Point> = (0..pl.point_count())
                .filter_map(|i| pl.get_point(i).map(|p| proj(&p)))
                .collect();
            let mut next = Polyline::new(pts);
            next.name = pl.name.clone();
            next.arrowhead = pl.arrowhead;
            next.width = pl.width;
            next.linecolor = pl.linecolor.clone();
            Some(Geometry::Polyline(Rc::new(next)))
        }
        Geometry::NurbsCurve(c) => {
            let mut next = (**c).clone();
            for i in 0..next.cv_count() {
                if let Some(cv) = next.get_cv(i) {
                    let _ = next.set_cv_point(i, &proj(&cv));
                }
            }
            Some(Geometry::NurbsCurve(Rc::new(next)))
        }
        Geometry::Mesh(m) => {
            let mut next = (**m).clone();
            let keys: Vec<usize> = next.vertex.keys().copied().collect();
            for k in keys {
                if let Some(p) = next.vertex_point(k) {
                    let q = proj(&p);
                    if let Some(v) = next.vertex.get_mut(&k) {
                        v.x = q[0];
                        v.y = q[1];
                        v.z = q[2];
                    }
                }
            }
            Some(Geometry::Mesh(Rc::new(next)))
        }
        _ => None,
    }
}

/// Execute one typed `Op` against any `CommandHost` (`State` or `HeadlessHost`).
pub fn execute(host: &mut dyn CommandHost, op: Op) -> Result<Reply, OpError> {
    let result = execute_inner(host, op);
    if let Ok(reply) = &result {
        host.note_status(&reply.message);
        host.touch();
    }
    result
}

fn execute_inner(host: &mut dyn CommandHost, op: Op) -> Result<Reply, OpError> {
    match op {
        Op::ObjectCreate { kind, layer } => {
            let prev_layer = host.scene().current_layer.clone();
            if let Some(layer_ref) = &layer {
                let doc = resolve_layer_doc(host.scene(), layer_ref)?;
                host.scene_mut()
                    .set_current_layer(doc, &layer_ref.name)
                    .map_err(OpError::classify)?;
            }
            let out = create_object(host, kind);
            if layer.is_some() {
                host.scene_mut().current_layer = prev_layer;
            }
            out
        }
        Op::ObjectDelete { targets } => match targets {
            Targets::Selection => {
                let deleted_refs = refs_of_rows(host.scene(), &host.selected_rows());
                let msg = host.delete_selection().map_err(OpError::classify)?;
                Ok(Reply {
                    message: msg,
                    created: Vec::new(),
                    changed: deleted_refs,
                    value: Value::Null,
                })
            }
            Targets::Refs(_) => {
                let rows = resolve_rows(host, &targets)?;
                if let Some(reason) = host.locked_reason(&rows) {
                    return Err(OpError::classify(reason));
                }
                let deleted_refs = refs_of_rows(host.scene(), &rows);
                let n = host.scene_mut().delete_rows(&rows);
                if n == 0 {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "object cannot be deleted",
                    ));
                }
                host.after_history();
                Ok(Reply {
                    message: if n == 1 {
                        "deleted".into()
                    } else {
                        format!("deleted {n} objects")
                    },
                    created: Vec::new(),
                    changed: deleted_refs,
                    value: json!({ "deleted": n }),
                })
            }
        },
        Op::ObjectTransform { targets, xform } => {
            let (delta, label) = xform.to_xform()?;
            match targets {
                Targets::Selection => {
                    let changed = refs_of_rows(host.scene(), &host.selected_rows());
                    let msg = host.apply(delta, label).map_err(OpError::classify)?;
                    Ok(Reply {
                        message: msg,
                        created: Vec::new(),
                        changed,
                        value: Value::Null,
                    })
                }
                Targets::Refs(_) => {
                    let rows = resolve_rows(host, &targets)?;
                    if let Some(reason) = host.locked_reason(&rows) {
                        return Err(OpError::classify(reason));
                    }
                    let changed = refs_of_rows(host.scene(), &rows);
                    host.scene_mut()
                        .transform_rows(&rows, &delta, label)
                        .ok_or_else(|| {
                            OpError::new(
                                ErrorCode::NotRetained,
                                "this target selection cannot be edited",
                            )
                        })?;
                    host.commit_rows();
                    Ok(Reply {
                        message: label.into(),
                        created: Vec::new(),
                        changed,
                        value: Value::Null,
                    })
                }
            }
        }
        Op::ObjectCopy { targets, xform } => {
            let (delta, _) = xform.to_xform()?;
            let rows = resolve_rows(host, &targets)?;
            if let Some(reason) = host.locked_reason(&rows) {
                return Err(OpError::classify(reason));
            }
            if matches!(targets, Targets::Selection)
                && crate::app::deform::Target::selected(host.selection()).is_some()
            {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "Copy works on whole objects; press Esc to leave the face, edge or control point",
                ));
            }
            let copies = host
                .scene_mut()
                .copy_rows(&rows, &delta)
                .map_err(OpError::classify)?;
            host.commit_rows();
            let created: Vec<ObjectRef> = copies
                .iter()
                .map(|(doc, guid)| ObjectRef::of(*doc, guid.as_ref()))
                .collect();
            Ok(Reply {
                message: format!("{} copied", created.len()),
                created,
                changed: Vec::new(),
                value: Value::Null,
            })
        }
        Op::ObjectExplode { targets } => {
            let rows = resolve_rows(host, &targets)?;
            if let Some(reason) = host.locked_reason(&rows) {
                return Err(OpError::classify(reason));
            }
            let changed = refs_of_rows(host.scene(), &rows);
            let (pieces, msg) = host
                .scene_mut()
                .explode_rows(&rows)
                .map_err(OpError::classify)?;
            host.commit_rows();
            let piece_rows: Vec<u32> = pieces
                .iter()
                .filter_map(|(doc, guid)| host.scene().row_of(*doc, guid))
                .collect();
            host.select_rows(piece_rows, false);
            let created = pieces
                .into_iter()
                .map(|(doc, guid)| ObjectRef::of(doc, guid.as_ref()))
                .collect();
            Ok(Reply {
                message: msg,
                created,
                changed,
                value: Value::Null,
            })
        }
        Op::ObjectProject { targets, plane } => {
            let rows = resolve_rows(host, &targets)?;
            if let Some(reason) = host.locked_reason(&rows) {
                return Err(OpError::classify(reason));
            }
            let target_plane = plane_from_spec(host, &plane)?;
            let mut edits = Vec::new();
            for &row in &rows {
                if let (Some(geom), Some(place)) =
                    (host.scene().geometry(row), host.scene().placement_of(row))
                    && let Some(projected) = project_geometry_to_plane(geom, &place, &target_plane)
                {
                    edits.push((row, projected));
                }
            }
            if edits.is_empty() {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "no projectable objects in targets",
                ));
            }
            let changed: Vec<ObjectRef> = edits
                .iter()
                .filter_map(|(r, _)| {
                    let (d, g) = host.scene().identity_of(*r)?;
                    Some(ObjectRef::of(d, g.as_ref()))
                })
                .collect();
            let n = host
                .scene_mut()
                .replace_rows(edits, "project")
                .map_err(OpError::classify)?;
            host.commit_rows();
            Ok(Reply {
                message: format!("Projected {n} object(s)"),
                created: Vec::new(),
                changed,
                value: json!({ "projected": n }),
            })
        }
        Op::CurveInterval {
            target,
            t0,
            t1,
            extend,
        } => {
            let doc = target.doc.resolve(host.scene())?;
            let row = host.scene().row_of(doc, &target.guid).ok_or_else(|| {
                OpError::new(
                    ErrorCode::NotFound,
                    format!("curve `{}` not found", target.guid),
                )
            })?;
            if let Some(reason) = host.locked_reason(&[row]) {
                return Err(OpError::classify(reason));
            }
            let prev = host.selected_rows();
            host.select(Some(row));
            let interval = if extend {
                Interval::Extend(t0, t1)
            } else {
                Interval::Trim(t0, t1)
            };
            let res = host.scene_mut().edit_interval(interval);
            if let Err(e) = res {
                host.select_rows(prev, false);
                return Err(OpError::classify(e));
            }
            host.after_history();
            Ok(Reply {
                message: "geometry updated".into(),
                created: Vec::new(),
                changed: vec![ObjectRef::of(doc, target.guid)],
                value: Value::Null,
            })
        }
        Op::CurveArrowhead { targets, head } => {
            let rows = resolve_rows(host, &targets)?;
            if let Some(reason) = host.locked_reason(&rows) {
                return Err(OpError::classify(reason));
            }
            let (arrow, label) = head.as_kernel();
            let mut edits = Vec::new();
            let mut curves = 0usize;
            for &row in &rows {
                let Some(geom) = host.scene().geometry(row) else {
                    continue;
                };
                match geom {
                    Geometry::Line(l) => {
                        curves += 1;
                        if l.arrowhead != arrow {
                            let mut next = (**l).clone();
                            next.arrowhead = arrow;
                            edits.push((row, Geometry::Line(Rc::new(next))));
                        }
                    }
                    Geometry::Polyline(pl) => {
                        curves += 1;
                        if pl.arrowhead != arrow {
                            let mut next = (**pl).clone();
                            next.arrowhead = arrow;
                            edits.push((row, Geometry::Polyline(Rc::new(next))));
                        }
                    }
                    Geometry::NurbsCurve(c) => {
                        curves += 1;
                        if c.arrowhead != arrow {
                            let mut next = (**c).clone();
                            next.arrowhead = arrow;
                            edits.push((row, Geometry::NurbsCurve(Rc::new(next))));
                        }
                    }
                    _ => {}
                }
            }
            if curves == 0 {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "Arrowhead needs a line, polyline or curve",
                ));
            }
            let changed: Vec<ObjectRef> = edits
                .iter()
                .filter_map(|(r, _)| {
                    let (d, g) = host.scene().identity_of(*r)?;
                    Some(ObjectRef::of(d, g.as_ref()))
                })
                .collect();
            let count = if edits.is_empty() {
                0
            } else {
                host.scene_mut()
                    .replace_rows(edits, "arrowhead")
                    .map_err(OpError::classify)?
            };
            host.commit_rows();
            Ok(Reply {
                message: format!("Arrowhead {label} on {curves} curve(s), {count} changed"),
                created: Vec::new(),
                changed,
                value: json!({ "curves": curves, "changed": count }),
            })
        }
        Op::SelectionSet { targets, mode } => {
            let incoming = resolve_rows(host, &targets)?;
            let next: Vec<u32> = match mode {
                SetMode::Replace => incoming,
                SetMode::Add => {
                    let mut cur = host.selected_rows();
                    cur.extend(incoming);
                    cur.sort_unstable();
                    cur.dedup();
                    cur
                }
                SetMode::Remove => {
                    let drop: std::collections::HashSet<u32> = incoming.into_iter().collect();
                    host.selected_rows()
                        .into_iter()
                        .filter(|r| !drop.contains(r))
                        .collect()
                }
            };
            let count = next.len();
            host.select_rows(next.clone(), false);
            let refs = refs_of_rows(host.scene(), &host.selected_rows());
            Ok(Reply::val(
                format!("{count} selected"),
                json!({ "selected": refs }),
            ))
        }
        Op::SelectionQuery { by } => {
            let matched = match_filter(host, &by)?;
            if matched.is_empty() && !matches!(by, Filter::All) {
                return Err(OpError::new(
                    ErrorCode::NotFound,
                    "nothing matches selection query",
                ));
            }
            let count = matched.len();
            host.select_rows(matched, false);
            let refs = refs_of_rows(host.scene(), &host.selected_rows());
            Ok(Reply::val(
                format!("{count} selected"),
                json!({ "selected": refs }),
            ))
        }
        Op::SelectionClear => {
            host.select(None);
            Ok(Reply::msg("selection cleared"))
        }
        Op::LayerList => {
            let scene = host.scene();
            let current = scene.current_layer();
            let mut layers_json = Vec::new();
            for (doc_idx, file) in scene.docs.iter().enumerate() {
                for node in file.session.tree.nodes() {
                    let borrowed = node.borrow();
                    if file.session.lookup.contains_key(&borrowed.name) {
                        continue;
                    }
                    let is_object_child = borrowed
                        .ancestors()
                        .iter()
                        .any(|a| file.session.lookup.contains_key(&a.borrow().name));
                    if is_object_child {
                        continue;
                    }
                    let parent = borrowed.parent().map(|p| p.borrow().name.clone());
                    layers_json.push(json!({
                        "doc": doc_idx,
                        "doc_name": file.name,
                        "name": borrowed.name,
                        "parent": parent,
                        "current": current.as_ref().is_some_and(|(d, n)| *d == doc_idx && *n == borrowed.name),
                    }));
                }
            }
            Ok(Reply::val(
                format!("{} layer(s)", layers_json.len()),
                json!({ "layers": layers_json }),
            ))
        }
        Op::LayerCreate {
            parent,
            name,
            sublayer,
        } => {
            if host.scene().docs.is_empty() {
                // Ensure a `Created` document exists first.
                let session = Rc::new(Session::new("Created"));
                let idx = host.scene().docs.len();
                host.append_file(FileDoc {
                    name: "Created".into(),
                    place: Xform::identity(),
                    session,
                    point_px: 0.0,
                    display_only: false,
                });
                host.scene_mut().created_doc = Some(idx);
            }
            let (doc, at_name) = if let Some(p) = &parent {
                (resolve_layer_doc(host.scene(), p)?, p.name.clone())
            } else if let Some((doc, cur)) = host.scene().current_layer() {
                (doc, cur)
            } else {
                let root = host.scene().docs[0]
                    .session
                    .tree
                    .root()
                    .ok_or_else(|| OpError::new(ErrorCode::NotFound, "document has no tree"))?
                    .borrow()
                    .name
                    .clone();
                (0, root)
            };
            let created_name = host
                .scene_mut()
                .new_layer(doc, &at_name, sublayer || parent.is_some())
                .map_err(OpError::classify)?;
            let final_name = if let Some(desired) = name.filter(|s| !s.trim().is_empty()) {
                host.scene_mut()
                    .rename_layer(doc, &created_name, &desired)
                    .map_err(OpError::classify)?;
                desired.trim().to_string()
            } else {
                created_name
            };
            host.commit_rows();
            Ok(Reply::val(
                format!("Created layer {final_name}"),
                json!({ "doc": doc, "name": final_name }),
            ))
        }
        Op::LayerRename { layer, name } => {
            let doc = resolve_layer_doc(host.scene(), &layer)?;
            host.scene_mut()
                .rename_layer(doc, &layer.name, &name)
                .map_err(OpError::classify)?;
            host.commit_rows();
            Ok(Reply::val(
                format!("Renamed layer {} to {}", layer.name, name.trim()),
                json!({ "doc": doc, "name": name.trim() }),
            ))
        }
        Op::LayerDelete { layer } => {
            let doc = resolve_layer_doc(host.scene(), &layer)?;
            let removed = host
                .scene_mut()
                .delete_layer(doc, &layer.name)
                .map_err(OpError::classify)?;
            host.after_history();
            Ok(Reply::val(
                format!("Deleted layer {} ({removed} object(s))", layer.name),
                json!({ "doc": doc, "removed_objects": removed }),
            ))
        }
        Op::LayerSetCurrent { layer } => {
            let doc = resolve_layer_doc(host.scene(), &layer)?;
            host.scene_mut()
                .set_current_layer(doc, &layer.name)
                .map_err(OpError::classify)?;
            host.refresh_layers();
            Ok(Reply::val(
                format!("Current layer set to {}", layer.name),
                json!({ "doc": doc, "name": layer.name }),
            ))
        }
        Op::LayerAssign { targets, layer } => {
            let rows = resolve_rows(host, &targets)?;
            let doc = resolve_layer_doc(host.scene(), &layer)?;
            let moved = host
                .scene_mut()
                .change_object_layer(&rows, doc, &layer.name)
                .map_err(OpError::classify)?;
            host.commit_rows();
            let changed: Vec<ObjectRef> = moved
                .into_iter()
                .map(|(d, g)| ObjectRef::of(d, g.as_ref()))
                .collect();
            Ok(Reply {
                message: format!("Moved {} object(s) to {}", changed.len(), layer.name),
                created: Vec::new(),
                changed,
                value: Value::Null,
            })
        }
        Op::LayerSet {
            layer,
            visible,
            locked,
            color,
        } => {
            let doc = resolve_layer_doc(host.scene(), &layer)?;
            let rows = layer_member_rows(host.scene(), doc, &layer.name)?;
            if let Some(vis) = visible {
                for r in host
                    .scene_mut()
                    .set_rows_hidden(&rows, !vis, "layer visibility")
                {
                    host.set_row_hidden(r, !vis);
                }
            }
            if let Some(lck) = locked {
                host.scene_mut().set_rows_locked(&rows, lck);
            }
            if let Some(col) = color {
                host.scene_mut().set_rows_color(&rows, false, col);
            }
            host.commit_rows();
            let changed = refs_of_rows(host.scene(), &rows);
            Ok(Reply {
                message: format!("Updated layer {}", layer.name),
                created: Vec::new(),
                changed,
                value: Value::Null,
            })
        }
        Op::GroupAdd { targets, name } => {
            if name.contains('/') {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "A group name must be without /",
                ));
            }
            let rows = resolve_rows(host, &targets)?;
            if let Some(reason) = host.locked_reason(&rows) {
                return Err(OpError::classify(reason));
            }
            let (doc, group_name, count) = host
                .scene_mut()
                .add_group(&rows, &name)
                .map_err(OpError::classify)?;
            host.commit_rows();
            host.reveal_layer(doc, &group_name);
            host.refresh_layers();
            let changed = refs_of_rows(host.scene(), &rows);
            Ok(Reply {
                message: format!("Grouped {count} object(s) as {group_name}"),
                created: Vec::new(),
                changed,
                value: json!({ "doc": doc, "group": group_name, "count": count }),
            })
        }
        Op::GraphAddEdge { a, b } => {
            let da = a.doc.resolve(host.scene())?;
            let db = b.doc.resolve(host.scene())?;
            let ra = host.scene().row_of(da, &a.guid).ok_or_else(|| {
                OpError::new(
                    ErrorCode::NotFound,
                    format!("object `{}` not found", a.guid),
                )
            })?;
            let rb = host.scene().row_of(db, &b.guid).ok_or_else(|| {
                OpError::new(
                    ErrorCode::NotFound,
                    format!("object `{}` not found", b.guid),
                )
            })?;
            if let Some(reason) = host.locked_reason(&[ra, rb]) {
                return Err(OpError::classify(reason));
            }
            let msg = host
                .scene_mut()
                .connect(&[ra, rb])
                .map_err(OpError::classify)?;
            host.commit_rows();
            host.ensure_graph_open();
            host.refresh_layers();
            Ok(Reply {
                message: msg,
                created: Vec::new(),
                changed: vec![a, b],
                value: Value::Null,
            })
        }
        Op::VisibilitySet { targets, visible } => {
            if matches!(targets, Targets::Selection) && !visible {
                let changed = refs_of_rows(host.scene(), &host.selected_rows());
                if changed.is_empty() {
                    return Err(OpError::new(ErrorCode::NoSelection, "nothing is selected"));
                }
                host.hide_selected();
                return Ok(Reply {
                    message: "hidden".into(),
                    created: Vec::new(),
                    changed,
                    value: Value::Null,
                });
            }
            let rows = resolve_rows(host, &targets)?;
            let changed_rows = host.scene_mut().set_rows_hidden(
                &rows,
                !visible,
                if visible { "show" } else { "hide" },
            );
            for &r in &changed_rows {
                host.set_row_hidden(r, !visible);
            }
            host.commit_rows();
            let changed = refs_of_rows(host.scene(), &changed_rows);
            Ok(Reply {
                message: if visible { "shown" } else { "hidden" }.into(),
                created: Vec::new(),
                changed,
                value: Value::Null,
            })
        }
        Op::ShowAll => {
            host.show_all();
            Ok(Reply::msg("everything shown"))
        }
        Op::LockSet { targets, locked } => {
            let rows = resolve_rows(host, &targets)?;
            let changed = refs_of_rows(host.scene(), &rows);
            if locked && matches!(targets, Targets::Selection) {
                host.select(None);
            }
            let n = host.scene_mut().set_rows_locked(&rows, locked);
            host.refresh_layers();
            Ok(Reply {
                message: format!(
                    "{} {n} object(s)",
                    if locked { "Locked" } else { "Unlocked" }
                ),
                created: Vec::new(),
                changed,
                value: json!({ "count": n }),
            })
        }
        Op::UnlockAll => {
            let n = host.scene_mut().unlock_all_identities();
            host.refresh_layers();
            Ok(Reply::val(
                format!("Unlocked {n} object(s)"),
                json!({ "count": n }),
            ))
        }
        Op::ColorSet {
            targets,
            edge,
            color,
        } => {
            let rows = resolve_rows(host, &targets)?;
            let changed_rows = host.scene_mut().set_rows_color(&rows, edge, color);
            host.commit_rows();
            let changed = refs_of_rows(host.scene(), &changed_rows);
            Ok(Reply {
                message: format!("Updated colour on {} object(s)", changed.len()),
                created: Vec::new(),
                changed,
                value: Value::Null,
            })
        }
        Op::AlphaSet { targets, alpha } => {
            if let Some(a) = alpha
                && (!a.is_finite() || !(0.0..=1.0).contains(&a))
            {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "alpha must be in 0.0..=1.0",
                ));
            }
            if matches!(targets, Targets::Selection) {
                let changed = refs_of_rows(host.scene(), &host.selected_rows());
                let msg = host.set_object_alpha(alpha).map_err(OpError::classify)?;
                return Ok(Reply {
                    message: msg,
                    created: Vec::new(),
                    changed,
                    value: Value::Null,
                });
            }
            let rows = resolve_rows(host, &targets)?;
            let changed_rows = host.scene_mut().set_rows_alpha(&rows, alpha);
            host.commit_rows();
            let changed = refs_of_rows(host.scene(), &changed_rows);
            Ok(Reply {
                message: match alpha {
                    Some(a) => format!("Alpha {a:.2} on {} object(s)", changed.len()),
                    None => "Alpha reset".into(),
                },
                created: Vec::new(),
                changed,
                value: Value::Null,
            })
        }
        Op::SceneInspect => {
            host.refresh_bounds();
            let scene = host.scene();
            let bounds = host.scene_bounds();
            let docs: Vec<Value> = scene
                .docs
                .iter()
                .enumerate()
                .map(|(i, d)| {
                    json!({
                        "index": i,
                        "name": d.name,
                        "objects": d.session.lookup.len(),
                        "display_only": d.display_only,
                    })
                })
                .collect();
            let selected = refs_of_rows(scene, &host.selected_rows());
            let val = json!({
                "documents": docs,
                "object_count": scene.object_count(),
                "row_count": scene.row_count(),
                "hidden_count": scene.hidden.len(),
                "locked_count": scene.locked.len(),
                "undo_depth": scene.undo_steps.len(),
                "redo_depth": scene.redo_steps.len(),
                "selected": selected,
                "bounds": bounds.is_valid().then(|| json!({
                    "center": [bounds.cx, bounds.cy, bounds.cz],
                    "half": [bounds.hx, bounds.hy, bounds.hz],
                })),
                "camera": camera_json(host),
                "view": view_json(host),
            });
            Ok(Reply::val(
                format!(
                    "{} document(s), {} object(s)",
                    scene.docs.len(),
                    scene.object_count()
                ),
                val,
            ))
        }
        Op::ObjectList { filter } => {
            let rows = match filter {
                Some(f) => match_filter(host, &f)?,
                None => (0..host.scene().row_count() as u32)
                    .filter(|&r| host.scene().identity_of(r).is_some())
                    .collect(),
            };
            let items: Vec<Value> = rows
                .into_iter()
                .filter_map(|r| object_summary(host, r))
                .collect();
            Ok(Reply::val(
                format!("{} object(s)", items.len()),
                json!({ "objects": items }),
            ))
        }
        Op::ObjectGet { target } => {
            let doc = target.doc.resolve(host.scene())?;
            let row = host.scene().row_of(doc, &target.guid).ok_or_else(|| {
                OpError::new(
                    ErrorCode::NotFound,
                    format!("object `{}` not found in document {doc}", target.guid),
                )
            })?;
            let info = object_summary(host, row)
                .ok_or_else(|| OpError::new(ErrorCode::NotFound, "object row is retired"))?;
            Ok(Reply::val(host.scene().object_name(row), info))
        }
        Op::Measure { what, targets } => measure_op(host, what, &targets),
        Op::SceneOpen { path } => open_session_file(host, &path),
        Op::SceneSave { path } => save_session_file(host, path.as_deref()),
        Op::SceneLoadManifest { path } => load_manifest_file(host, &path),
        Op::SceneImport { path } => {
            let before = host.scene().docs.len();
            let msg = host.import_file(Some(&path)).map_err(OpError::classify)?;
            let mut created = Vec::new();
            for doc in before..host.scene().docs.len() {
                for guid in host.scene().docs[doc].session.order() {
                    if host.scene().row_of(doc, &guid).is_some() {
                        created.push(ObjectRef::of(doc, guid));
                    }
                }
            }
            Ok(Reply {
                message: msg,
                created,
                changed: Vec::new(),
                value: Value::Null,
            })
        }
        Op::SceneExportObj { path } => {
            let msg = host
                .export_obj(path.as_deref())
                .map_err(OpError::classify)?;
            Ok(Reply::val(msg, json!({ "path": path })))
        }
        Op::HistoryUndo => {
            if !host.scene_mut().undo() {
                return Err(OpError::new(ErrorCode::InvalidArgument, "nothing to undo"));
            }
            host.after_history();
            Ok(Reply::msg("undone"))
        }
        Op::HistoryRedo => {
            if !host.scene_mut().redo() {
                return Err(OpError::new(ErrorCode::InvalidArgument, "nothing to redo"));
            }
            host.after_history();
            Ok(Reply::msg("redone"))
        }
        Op::CameraGet => {
            let val = camera_json(host);
            Ok(Reply::val("camera state", val))
        }
        Op::CameraSetView { view } => {
            let v = match view.trim().to_ascii_lowercase().as_str() {
                "top" => View::Top,
                "bottom" => View::Bottom,
                "front" => View::Front,
                "back" => View::Back,
                "right" | "side" => View::Right,
                "left" => View::Left,
                "iso" | "isometric" => View::Iso,
                "isoback" | "isometric back" | "isometric_back" => View::IsoBack,
                _ => {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        format!("unknown camera view `{view}`"),
                    ));
                }
            };
            host.camera_mut().set_view(v);
            Ok(Reply::val(format!("View {view}"), camera_json(host)))
        }
        Op::CameraSetPose {
            target,
            position,
            perspective,
        } => {
            check_point(target).map_err(OpError::classify)?;
            check_point(position).map_err(OpError::classify)?;
            let units = host.camera().unit.to_meters();
            let t_m = target.map(|v| v * units);
            let p_m = position.map(|v| v * units);
            let dx = t_m[0] - p_m[0];
            let dy = t_m[1] - p_m[1];
            let dz = t_m[2] - p_m[2];
            let dist = (dx * dx + dy * dy + dz * dz).sqrt();
            if dist <= 1e-9 {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "camera position and target must differ",
                ));
            }
            let cam = host.camera_mut();
            cam.target = t_m;
            cam.position = p_m;
            cam.distance = dist;
            if let Some(p) = perspective {
                cam.perspective = p;
            }
            Ok(Reply::val("camera pose updated", camera_json(host)))
        }
        Op::CameraFit { targets } => {
            match targets {
                None | Some(Targets::Selection) => host.fit_selected_or_all(),
                Some(t @ Targets::Refs(_)) => {
                    let rows = resolve_rows(host, &t)?;
                    let mut b = AABB::empty();
                    for r in rows {
                        if let Some(rb) = host.row_bounds(r) {
                            b.union_with(&rb);
                        }
                    }
                    if b.is_valid() {
                        let aspect = host.aspect();
                        host.camera_mut().fit_in(&b, aspect, [0.0, 0.0, 1.0, 1.0]);
                    }
                }
            }
            Ok(Reply::val("fitted", camera_json(host)))
        }
        Op::ViewGet => Ok(Reply::val("view settings", view_json(host))),
        Op::ViewSet { patch } => {
            if let Some(v) = patch.arctic {
                host.set_arctic(v);
            }
            if let Some(v) = patch.wireframe {
                host.set_view_flag(ViewFlag::Wireframe, v);
            }
            if let Some(v) = patch.outlines {
                host.set_outlines(v);
            }
            if let Some(v) = patch.points {
                host.set_view_flag(ViewFlag::Points, v);
            }
            if let Some(v) = patch.lines {
                host.set_view_flag(ViewFlag::Lines, v);
            }
            if let Some(v) = patch.edges {
                host.set_view_flag(ViewFlag::Edges, v);
            }
            if let Some(v) = patch.lighting {
                host.set_view_flag(ViewFlag::Lighting, v);
            }
            if let Some(v) = patch.backfaces {
                host.set_view_flag(ViewFlag::Backfaces, v);
            }
            if let Some(v) = patch.cull_backfaces {
                host.set_view_flag(ViewFlag::CullBackfaces, v);
            }
            if let Some(v) = patch.opacity {
                if !v.is_finite() || !(0.0..=1.0).contains(&v) {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "opacity must be in 0.0..=1.0",
                    ));
                }
                host.set_opacity(v);
            }
            if let Some(v) = patch.cloud_size {
                if !v.is_finite() || v <= 0.0 {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "cloud_size must be positive",
                    ));
                }
                host.set_cloud_size(v);
            }
            if let Some(v) = patch.plane_size {
                if !v.is_finite() || v <= 0.0 {
                    return Err(OpError::new(
                        ErrorCode::InvalidArgument,
                        "plane_size must be positive",
                    ));
                }
                host.set_plane_size(v);
            }
            if let Some(v) = patch.attributes {
                host.show_attributes(Some(v));
            }
            if let Some(v) = patch.interactions {
                host.show_interactions(Some(v));
            }
            Ok(Reply::val("view updated", view_json(host)))
        }
        Op::ConstructionPlaneSet { plane } => {
            let cp = match plane {
                None | Some(PlaneSpec::CPlane) => None,
                Some(PlaneSpec::Xy { .. }) => Some(CPlane::Xy),
                Some(PlaneSpec::Yz { .. }) => Some(CPlane::Yz),
                Some(PlaneSpec::Zx { .. }) => Some(CPlane::Xz),
                Some(PlaneSpec::Points { a, b, c }) => {
                    let p = plane_from_spec(host, &PlaneSpec::Points { a, b, c })?;
                    let (o, vx, vy) = (p.origin(), p.x_axis(), p.y_axis());
                    Some(CPlane::Frame {
                        origin: [o[0], o[1], o[2]],
                        x: [vx[0], vx[1], vx[2]],
                        y: [vy[0], vy[1], vy[2]],
                    })
                }
                Some(PlaneSpec::Normal { origin, normal }) => {
                    let p = plane_from_spec(host, &PlaneSpec::Normal { origin, normal })?;
                    let (o, vx, vy) = (p.origin(), p.x_axis(), p.y_axis());
                    Some(CPlane::Frame {
                        origin: [o[0], o[1], o[2]],
                        x: [vx[0], vx[1], vx[2]],
                        y: [vy[0], vy[1], vy[2]],
                    })
                }
            };
            host.set_construction_plane(cp);
            Ok(Reply::val(
                "construction plane updated",
                host.construction_plane_status(),
            ))
        }
        Op::ClippingPlaneAdd { plane } => {
            let half = host.clipping_half_size();
            let clip_plane = match plane {
                PlaneSpec::Xy { z } => {
                    clipping::plane_from(clipping::Mode::Xy, &[[0.0, 0.0, z]], half)
                }
                PlaneSpec::Yz { x } => {
                    clipping::plane_from(clipping::Mode::Yz, &[[x, 0.0, 0.0]], half)
                }
                PlaneSpec::Zx { y } => {
                    clipping::plane_from(clipping::Mode::Zx, &[[0.0, y, 0.0]], half)
                }
                PlaneSpec::Points { a, b, c } => {
                    clipping::plane_from(clipping::Mode::Points, &[a, b, c], half)
                }
                PlaneSpec::Normal { origin, normal } => clipping::plane_from(
                    clipping::Mode::Normal,
                    &[
                        origin,
                        [
                            origin[0] + normal[0],
                            origin[1] + normal[1],
                            origin[2] + normal[2],
                        ],
                    ],
                    half,
                ),
                PlaneSpec::CPlane => {
                    let n =
                        CPlane::facing(&host.camera().orientation.rotate_vector(Vector::y_axis()))
                            .normal();
                    clipping::plane_from(
                        clipping::Mode::Normal,
                        &[[0.0, 0.0, 0.0], [n[0], n[1], n[2]]],
                        half,
                    )
                }
            }
            .map_err(OpError::classify)?;
            let msg = host
                .create_clipping_plane(clip_plane)
                .map_err(OpError::classify)?;
            let created = refs_of_rows(host.scene(), &host.selected_rows());
            Ok(Reply {
                message: msg,
                created,
                changed: Vec::new(),
                value: Value::Null,
            })
        }
        Op::ClippingSet {
            enabled,
            solid_fill,
            flip,
        } => {
            let mut msgs = Vec::new();
            if let Some(on) = enabled {
                msgs.push(host.set_clipping(on));
            }
            if let Some(solid) = solid_fill {
                msgs.push(host.set_clipping_fill(Some(solid)));
            }
            if flip {
                msgs.push(host.flip_clipping_planes().map_err(OpError::classify)?);
            }
            let msg = if msgs.is_empty() {
                "clipping unchanged".into()
            } else {
                msgs.join(" · ")
            };
            Ok(Reply::msg(msg))
        }
        Op::RenderFrame { .. } => Err(OpError::new(
            ErrorCode::Unsupported,
            "render_frame requires a GPU render host (use `session-viewer-cli exec --render <out.ppm>` or `serve --stdio`)",
        )),
        Op::PerfStats => Ok(Reply::val("performance stats", host.perf_stats())),
        Op::CommandRun { line } => {
            let msg = host.run_command(&line).map_err(OpError::classify)?;
            let selected = refs_of_rows(host.scene(), &host.selected_rows());
            Ok(Reply::val(msg, json!({ "selected": selected })))
        }
    }
}

fn create_object(host: &mut dyn CommandHost, kind: CreateKind) -> Result<Reply, OpError> {
    match kind {
        CreateKind::Point { point } => {
            let geom = verbs::point::SPEC
                .geometry(&[point])
                .map_err(OpError::classify)?;
            commit_created_geometry(host, geom, "point")
        }
        CreateKind::Line { start, end } => {
            let geom = verbs::line::SPEC
                .geometry(&[start, end])
                .map_err(OpError::classify)?;
            commit_created_geometry(host, geom, "line")
        }
        CreateKind::Arrow { start, end } => {
            let geom = verbs::arrow::SPEC
                .geometry(&[start, end])
                .map_err(OpError::classify)?;
            commit_created_geometry(host, geom, "arrow")
        }
        CreateKind::Polyline { mut points, closed } => {
            if closed && points.len() >= 3 && points.first() != points.last() {
                points.push(points[0]);
            }
            let geom = verbs::polyline::SPEC
                .geometry(&points)
                .map_err(OpError::classify)?;
            commit_created_geometry(host, geom, "polyline")
        }
        CreateKind::Curve { mut points, closed } => {
            if closed && points.len() >= 3 && points.first() != points.last() {
                points.push(points[0]);
            }
            let geom = verbs::curve::SPEC
                .geometry(&points)
                .map_err(OpError::classify)?;
            commit_created_geometry(host, geom, "NURBS curve")
        }
        CreateKind::Circle { center, radius } => {
            let pt = check_point(center).map_err(OpError::classify)?;
            let geom = build_shape_geometry(
                &verbs::nurbs_curve_circle::SHAPE,
                &[Answer::Point(pt), Answer::Number(radius)],
                false,
            )?;
            commit_created_geometry(host, geom, "circle")
        }
        CreateKind::Box {
            origin,
            dx,
            dy,
            dz,
            mesh,
        } => {
            let pt = check_point(origin).map_err(OpError::classify)?;
            let geom = build_shape_geometry(
                &verbs::r#box::SHAPE,
                &[
                    Answer::Point(pt),
                    Answer::Number(dx),
                    Answer::Number(dy),
                    Answer::Number(dz),
                ],
                mesh,
            )?;
            commit_created_geometry(host, geom, "box")
        }
        CreateKind::Sphere {
            center,
            radius,
            mesh,
        } => {
            let pt = check_point(center).map_err(OpError::classify)?;
            let geom = build_shape_geometry(
                &verbs::sphere::SHAPE,
                &[Answer::Point(pt), Answer::Number(radius)],
                mesh,
            )?;
            commit_created_geometry(host, geom, "sphere")
        }
        CreateKind::Cylinder {
            origin,
            radius,
            height,
            mesh,
        } => {
            let pt = check_point(origin).map_err(OpError::classify)?;
            let geom = build_shape_geometry(
                &verbs::cylinder::SHAPE,
                &[
                    Answer::Point(pt),
                    Answer::Number(radius),
                    Answer::Number(height),
                ],
                mesh,
            )?;
            commit_created_geometry(host, geom, "cylinder")
        }
        CreateKind::Cone {
            origin,
            radius,
            height,
            mesh,
        } => {
            let pt = check_point(origin).map_err(OpError::classify)?;
            let geom = build_shape_geometry(
                &verbs::cone::SHAPE,
                &[
                    Answer::Point(pt),
                    Answer::Number(radius),
                    Answer::Number(height),
                ],
                mesh,
            )?;
            commit_created_geometry(host, geom, "cone")
        }
        CreateKind::Torus {
            center,
            major_radius,
            minor_radius,
            mesh,
        } => {
            let pt = check_point(center).map_err(OpError::classify)?;
            let geom = build_shape_geometry(
                &verbs::torus::SHAPE,
                &[
                    Answer::Point(pt),
                    Answer::Number(major_radius),
                    Answer::Number(minor_radius),
                ],
                mesh,
            )?;
            commit_created_geometry(host, geom, "torus")
        }
        CreateKind::Text {
            text,
            origin,
            height,
        } => {
            let trimmed = text.trim();
            if trimmed.is_empty() || trimmed.chars().count() > 80 {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "text must be 1..=80 characters",
                ));
            }
            let pt = check_point(origin).map_err(OpError::classify)?;
            let h = height.unwrap_or(20.0);
            if !h.is_finite() || h <= 0.0 {
                return Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "text height must be positive",
                ));
            }
            host.add_text(TextLabel {
                id: 0,
                object: None,
                text: trimmed.to_string(),
                font_size: 18.0,
                line_height: 26.0,
                color: [30, 30, 30, 255],
                placement: TextPlacement::WorldPlane {
                    world: [pt[0], pt[1], pt[2]],
                    right: [1.0, 0.0, 0.0],
                    up: [0.0, 1.0, 0.0],
                    world_height: h,
                    plate: false,
                },
                clip: None,
            });
            let key = host
                .scene()
                .texts
                .last()
                .map(|t| t.key.clone())
                .unwrap_or_default();
            Ok(Reply::val(
                format!("Created text `{trimmed}`"),
                json!({ "text_key": key }),
            ))
        }
        CreateKind::BoundingBox => {
            let msg = host.run_command("BoundingBox").map_err(OpError::classify)?;
            let created = refs_of_rows(host.scene(), &host.selected_rows());
            Ok(Reply {
                message: msg,
                created,
                changed: Vec::new(),
                value: Value::Null,
            })
        }
    }
}

fn commit_created_geometry(
    host: &mut dyn CommandHost,
    geom: Geometry,
    what: &str,
) -> Result<Reply, OpError> {
    let (doc, guid) = host
        .scene_mut()
        .create_geometry(geom)
        .map_err(OpError::classify)?;
    host.after_history();
    let row = host.scene().row_of(doc, &guid);
    host.select(row);
    let obj_ref = ObjectRef::of(doc, guid);
    Ok(Reply {
        message: format!("Created {what}"),
        created: vec![obj_ref],
        changed: Vec::new(),
        value: Value::Null,
    })
}

fn layer_member_rows(scene: &Scene, doc: usize, name: &str) -> Result<Vec<u32>, OpError> {
    let file = scene
        .docs
        .get(doc)
        .ok_or_else(|| OpError::new(ErrorCode::NotFound, "document is gone"))?;
    if file.display_only {
        return Err(OpError::new(ErrorCode::NotRetained, READ_ONLY));
    }
    let nodes = file.session.tree.get_nodes_by_name(name);
    let Some(node) = nodes.first() else {
        return Err(OpError::new(
            ErrorCode::NotFound,
            format!("layer `{name}` not found"),
        ));
    };
    let mut rows: Vec<u32> = node
        .borrow()
        .descendants()
        .iter()
        .filter_map(|m| scene.row_of(doc, &m.borrow().name))
        .collect();
    rows.sort_unstable();
    rows.dedup();
    Ok(rows)
}

fn measure_op(
    host: &mut dyn CommandHost,
    what: MeasureKind,
    targets: &Targets,
) -> Result<Reply, OpError> {
    if let MeasureKind::Distance { a, b } = what {
        let pa = check_point(a).map_err(OpError::classify)?;
        let pb = check_point(b).map_err(OpError::classify)?;
        let dist = pa.distance(&pb, None);
        let unit = verbs::measure::unit_suffix(host, 1);
        let msg = format!("Distance {} {unit}", verbs::measure::to_text(dist));
        host.set_mark(
            vec![pa, pb],
            format!("{} {unit}", verbs::measure::to_text(dist)),
        );
        return Ok(Reply::val(msg, json!({ "distance": dist, "unit": unit })));
    }

    let prev = host.selected_rows();
    let restore = !matches!(targets, Targets::Selection);
    if restore {
        let rows = resolve_rows(host, targets)?;
        host.select_rows(rows, false);
    }

    let rows = host.selected_rows();
    if rows.is_empty() {
        if restore {
            host.select_rows(prev, false);
        }
        return Err(OpError::new(ErrorCode::NoSelection, "nothing is selected"));
    }

    let result = match what {
        MeasureKind::Length => {
            let mut sum = 0.0;
            let mut count = 0usize;
            for &r in &rows {
                if let (Some(g), Some(pl)) =
                    (host.scene().geometry(r), host.scene().placement_of(r))
                {
                    let len = match g {
                        Geometry::Line(l) => Some(l.transformed(&pl).length()),
                        Geometry::Polyline(p) => Some(p.transformed(&pl).length()),
                        Geometry::NurbsCurve(c) => Some(c.transformed(&pl).length(None)),
                        _ => None,
                    };
                    if let Some(v) = len {
                        sum += v;
                        count += 1;
                    }
                }
            }
            if count == 0 {
                Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "Length measures lines, polylines and NURBS curves; none in targets",
                ))
            } else {
                let unit = verbs::measure::unit_suffix(host, 1);
                Ok(Reply::val(
                    format!(
                        "Length {} {unit} · {count} curve(s)",
                        verbs::measure::to_text(sum)
                    ),
                    json!({ "length": sum, "count": count, "unit": unit }),
                ))
            }
        }
        MeasureKind::Area => {
            let mut sum = 0.0;
            let mut count = 0usize;
            for &r in &rows {
                if let (Some(g), Some(pl)) =
                    (host.scene().geometry(r), host.scene().placement_of(r))
                {
                    let area = match g {
                        Geometry::Mesh(m) => Some(verbs::measure::compute_mesh_area(m, &pl, None)),
                        Geometry::BRep(b) => Some(
                            b.face_meshes_q(Some(crate::app::walk::brep::QUALITY))
                                .iter()
                                .map(|m| verbs::measure::compute_mesh_area(m, &pl, None))
                                .sum(),
                        ),
                        Geometry::NurbsSurface(s) => {
                            let m = s.mesh();
                            Some(verbs::measure::compute_mesh_area(&m, &pl, None))
                        }
                        Geometry::Element(e) => match e.geometry() {
                            session_rust::element::ElementGeometry::Mesh(m) => {
                                Some(verbs::measure::compute_mesh_area(m, &pl, None))
                            }
                            session_rust::element::ElementGeometry::BRep(b) => Some(
                                b.face_meshes_q(Some(crate::app::walk::brep::QUALITY))
                                    .iter()
                                    .map(|m| verbs::measure::compute_mesh_area(m, &pl, None))
                                    .sum(),
                            ),
                            session_rust::element::ElementGeometry::None => None,
                        },
                        _ => None,
                    };
                    if let Some(a) = area {
                        sum += a;
                        count += 1;
                    }
                }
            }
            if count == 0 {
                Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "Area measures meshes, BReps, surfaces and elements; none in targets",
                ))
            } else {
                let unit = verbs::measure::unit_suffix(host, 2);
                Ok(Reply::val(
                    format!(
                        "Area {} {unit} · {count} object(s)",
                        verbs::measure::to_text(sum)
                    ),
                    json!({ "area": sum, "count": count, "unit": unit }),
                ))
            }
        }
        MeasureKind::Volume => match host.run_command("Volume") {
            Ok(msg) => {
                // Parse numeric prefix after "Volume "
                let num = msg
                    .strip_prefix("Volume ")
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse::<f64>().ok());
                let unit = verbs::measure::unit_suffix(host, 3);
                Ok(Reply::val(
                    msg,
                    json!({ "volume_approx": num, "unit": unit }),
                ))
            }
            Err(e) => Err(OpError::classify(e)),
        },
        MeasureKind::BoundsExtent => {
            let mut b = AABB::empty();
            for &r in &rows {
                if let Some(rb) = host.row_bounds(r) {
                    b.union_with(&rb);
                }
            }
            if !b.is_valid() {
                Err(OpError::new(
                    ErrorCode::InvalidArgument,
                    "targets have empty bounding box",
                ))
            } else {
                let extent = [2.0 * b.hx, 2.0 * b.hy, 2.0 * b.hz];
                let unit = verbs::measure::unit_suffix(host, 1);
                Ok(Reply::val(
                    format!(
                        "X {} · Y {} · Z {} {unit}",
                        verbs::measure::to_text(extent[0]),
                        verbs::measure::to_text(extent[1]),
                        verbs::measure::to_text(extent[2]),
                    ),
                    json!({
                        "extent": extent,
                        "center": [b.cx, b.cy, b.cz],
                        "min": [b.cx - b.hx, b.cy - b.hy, b.cz - b.hz],
                        "max": [b.cx + b.hx, b.cy + b.hy, b.cz + b.hz],
                        "unit": unit,
                    }),
                ))
            }
        }
        MeasureKind::Distance { .. } => unreachable!(),
    };

    if restore {
        host.select_rows(prev, false);
    }
    result
}

fn open_session_file(host: &mut dyn CommandHost, path: &str) -> Result<Reply, OpError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let bytes = std::fs::read(path)
            .map_err(|e| OpError::new(ErrorCode::NotFound, format!("cannot read {path}: {e}")))?;
        let opened = open_with_state(&bytes).map_err(OpError::classify)?;
        host.install_opened_session(opened);
        let docs = host.scene().docs.len();
        let objects = host.scene().object_count();
        Ok(Reply::val(
            format!("Opened {path} ({docs} document(s), {objects} object(s))"),
            json!({ "path": path, "documents": docs, "objects": objects }),
        ))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (host, path);
        Err(OpError::new(
            ErrorCode::Unsupported,
            "scene_open by filesystem path is native-only; use Open in the browser",
        ))
    }
}

fn save_session_file(host: &mut dyn CommandHost, path: Option<&str>) -> Result<Reply, OpError> {
    let cam = SavedCamera::from_camera(host.camera());
    let view = host.saved_view();
    let bytes = save_with_state(host.scene(), Some(cam), Some(view)).map_err(OpError::classify)?;
    let size = bytes.len();
    if let Some(p) = path {
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::fs::write(p, &bytes).map_err(|e| {
                OpError::new(ErrorCode::InvalidArgument, format!("cannot write {p}: {e}"))
            })?;
            return Ok(Reply::val(
                format!("Saved session to {p} ({size} bytes)"),
                json!({ "path": p, "bytes": size }),
            ));
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = p;
        }
    }
    let msg = host.save_session().map_err(OpError::classify)?;
    Ok(Reply::val(msg, json!({ "bytes": size })))
}

fn load_manifest_file(host: &mut dyn CommandHost, path: &str) -> Result<Reply, OpError> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let lower = path.to_ascii_lowercase();
        if !(lower.ends_with(".yaml")
            || lower.ends_with(".yml")
            || lower.ends_with(".json")
            || lower.ends_with(".toml"))
        {
            // Single `.pb` or `.obj` scene file: clear and import.
            host.clear_scene();
            let msg = host.import_file(Some(path)).map_err(OpError::classify)?;
            host.fit_selected_or_all();
            return Ok(Reply::val(
                msg,
                json!({ "path": path, "documents": host.scene().docs.len(), "objects": host.scene().object_count() }),
            ));
        }
        let raw = std::fs::read(path)
            .map_err(|e| OpError::new(ErrorCode::NotFound, format!("cannot read {path}: {e}")))?;
        let manifest = Manifest::parse(&raw).map_err(|e| {
            OpError::new(
                ErrorCode::InvalidArgument,
                format!("invalid manifest {path}: {e}"),
            )
        })?;
        let here = std::path::Path::new(path)
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .to_path_buf();
        let first = manifest
            .items
            .first()
            .map(|i| i.file.clone())
            .unwrap_or_default();
        let root = if here.join(&first).exists() {
            here
        } else {
            here.join("..")
        };
        host.clear_scene();
        for (i, item) in manifest.items.iter().enumerate() {
            let fp = root.join(&item.file);
            let bytes = std::fs::read(&fp).map_err(|e| {
                OpError::new(
                    ErrorCode::NotFound,
                    format!("cannot read {}: {e}", fp.display()),
                )
            })?;
            let session = Session::pb_loads(&bytes).map_err(|e| {
                OpError::new(
                    ErrorCode::InvalidArgument,
                    format!("invalid session {}: {e}", fp.display()),
                )
            })?;
            let doc_name = if item.name.is_empty() {
                item.file.clone()
            } else {
                item.name.clone()
            };
            host.append_file(FileDoc {
                name: doc_name,
                place: manifest.place(i, [3000.0, 3000.0]),
                session: Rc::new(session),
                point_px: item.point_size as f32,
                display_only: item.display_only,
            });
        }
        if let Some(v) = manifest.view {
            host.camera_mut().set_view(v);
        }
        host.fit_selected_or_all();
        let docs = host.scene().docs.len();
        let objects = host.scene().object_count();
        Ok(Reply::val(
            format!("Loaded manifest {path} ({docs} document(s), {objects} object(s))"),
            json!({ "path": path, "documents": docs, "objects": objects }),
        ))
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (host, path);
        Err(OpError::new(
            ErrorCode::Unsupported,
            "scene_load_manifest from filesystem path is native-only",
        ))
    }
}

fn camera_json(host: &dyn CommandHost) -> Value {
    let cam = host.camera();
    let u = cam.unit.to_meters();
    json!({
        "position": cam.position.map(|v| v / u),
        "target": cam.target.map(|v| v / u),
        "distance": cam.distance / u,
        "orientation": cam.orientation_quat(),
        "perspective": cam.perspective,
    })
}

fn view_json(host: &dyn CommandHost) -> Value {
    let sv = host.saved_view();
    json!({
        "arctic": host.arctic(),
        "wireframe": host.view_flag(ViewFlag::Wireframe),
        "outlines": host.outlines(),
        "points": host.view_flag(ViewFlag::Points),
        "lines": host.view_flag(ViewFlag::Lines),
        "edges": host.view_flag(ViewFlag::Edges),
        "lighting": host.view_flag(ViewFlag::Lighting),
        "backfaces": host.view_flag(ViewFlag::Backfaces),
        "cull_backfaces": host.view_flag(ViewFlag::CullBackfaces),
        "opacity": host.opacity(),
        "grid": sv.show_grid,
        "cloud_size": sv.cloud_size,
        "attributes": host.scene().attributes,
        "interactions": host.scene().interactions,
    })
}

/// Metadata entry describing one `Op` variant for generated docs and MCP schemas.
#[derive(Clone, Debug, Serialize)]
pub struct OpDescriptor {
    pub op: &'static str,
    pub category: &'static str,
    pub summary: &'static str,
    pub example_json: &'static str,
}

/// Complete catalog of all `Op` variants supported by `viewer-core::op`.
pub fn op_catalog() -> Vec<OpDescriptor> {
    vec![
        OpDescriptor {
            op: "object_create",
            category: "Create & Edit",
            summary: "Create a kernel geometry or scene text object on the current or specified layer.",
            example_json: r#"{"op":"object_create","kind":{"type":"box","origin":[0,0,0],"dx":100,"dy":50,"dz":30}}"#,
        },
        OpDescriptor {
            op: "object_delete",
            category: "Create & Edit",
            summary: "Delete the selected objects or explicit (doc, guid) targets in one undo step.",
            example_json: r#"{"op":"object_delete","targets":"selection"}"#,
        },
        OpDescriptor {
            op: "object_transform",
            category: "Create & Edit",
            summary: "Translate, rotate (with explicit pivot), scale (with explicit pivot), orient3, or apply a 4x4 matrix.",
            example_json: r#"{"op":"object_transform","xform":{"type":"rotate","axis":"z","degrees":45.0,"pivot":[0,0,0]}}"#,
        },
        OpDescriptor {
            op: "object_copy",
            category: "Create & Edit",
            summary: "Duplicate target objects transformed by `xform` in one undo step.",
            example_json: r#"{"op":"object_copy","xform":{"type":"translate","delta":[200,0,0]}}"#,
        },
        OpDescriptor {
            op: "object_explode",
            category: "Create & Edit",
            summary: "Explode polylines into lines, BReps/meshes into faces, or point clouds into points.",
            example_json: r#"{"op":"object_explode","targets":"selection"}"#,
        },
        OpDescriptor {
            op: "object_project",
            category: "Create & Edit",
            summary: "Project target objects onto XY, YZ, ZX, CPlane, 3-point, or normal plane.",
            example_json: r#"{"op":"object_project","plane":{"type":"xy","z":0.0}}"#,
        },
        OpDescriptor {
            op: "curve_interval",
            category: "Create & Edit",
            summary: "Trim (`0 <= t0 < t1 <= 1`) or extend (`extend: true`, `t0 <= 0, t1 >= 1`) a curve.",
            example_json: r#"{"op":"curve_interval","target":{"doc":0,"guid":"..."},"t0":0.2,"t1":0.8}"#,
        },
        OpDescriptor {
            op: "curve_arrowhead",
            category: "Create & Edit",
            summary: "Set curve arrowheads (`none`, `start`, `end`, `both`) on target curves.",
            example_json: r#"{"op":"curve_arrowhead","head":"end"}"#,
        },
        OpDescriptor {
            op: "selection_set",
            category: "Selection",
            summary: "Replace, add to, or remove from the current selection by `(doc, guid)` refs.",
            example_json: r#"{"op":"selection_set","targets":{"refs":[{"doc":0,"guid":"..."}]},"mode":"replace"}"#,
        },
        OpDescriptor {
            op: "selection_query",
            category: "Selection",
            summary: "Select objects matching `all`, `name`, `smaller_than`, `in_box`, or `kind`.",
            example_json: r#"{"op":"selection_query","by":{"type":"all"}}"#,
        },
        OpDescriptor {
            op: "selection_clear",
            category: "Selection",
            summary: "Clear the current selection.",
            example_json: r#"{"op":"selection_clear"}"#,
        },
        OpDescriptor {
            op: "layer_list",
            category: "Layers & Appearance",
            summary: "List all document layers and indicate which layer is current.",
            example_json: r#"{"op":"layer_list"}"#,
        },
        OpDescriptor {
            op: "layer_create",
            category: "Layers & Appearance",
            summary: "Create a new layer or sublayer in a document.",
            example_json: r#"{"op":"layer_create","name":"Beams"}"#,
        },
        OpDescriptor {
            op: "layer_rename",
            category: "Layers & Appearance",
            summary: "Rename an existing layer addressed by `(doc, name)`.",
            example_json: r#"{"op":"layer_rename","layer":{"doc":0,"name":"Beams"},"name":"Columns"}"#,
        },
        OpDescriptor {
            op: "layer_delete",
            category: "Layers & Appearance",
            summary: "Delete a non-root, non-current layer and its objects.",
            example_json: r#"{"op":"layer_delete","layer":{"doc":0,"name":"Columns"}}"#,
        },
        OpDescriptor {
            op: "layer_set_current",
            category: "Layers & Appearance",
            summary: "Set the active layer where newly created objects are placed.",
            example_json: r#"{"op":"layer_set_current","layer":{"doc":0,"name":"Beams"}}"#,
        },
        OpDescriptor {
            op: "layer_assign",
            category: "Layers & Appearance",
            summary: "Move target objects onto a destination layer, preserving world placement.",
            example_json: r#"{"op":"layer_assign","layer":{"doc":0,"name":"Beams"}}"#,
        },
        OpDescriptor {
            op: "layer_set",
            category: "Layers & Appearance",
            summary: "Batch-update visibility, lock state, or face colour for all members of a layer.",
            example_json: r#"{"op":"layer_set","layer":{"doc":0,"name":"Beams"},"visible":true,"color":[220,80,40]}"#,
        },
        OpDescriptor {
            op: "group_add",
            category: "Layers & Appearance",
            summary: "Group target objects of one document under a new named group node.",
            example_json: r#"{"op":"group_add","name":"Truss_01"}"#,
        },
        OpDescriptor {
            op: "graph_add_edge",
            category: "Layers & Appearance",
            summary: "Add an undirected edge between two objects in their document's session graph.",
            example_json: r#"{"op":"graph_add_edge","a":{"doc":0,"guid":"..."},"b":{"doc":0,"guid":"..."}}"#,
        },
        OpDescriptor {
            op: "visibility_set",
            category: "Layers & Appearance",
            summary: "Hide (`visible: false`) or show (`visible: true`) target objects (undoable).",
            example_json: r#"{"op":"visibility_set","visible":false}"#,
        },
        OpDescriptor {
            op: "show_all",
            category: "Layers & Appearance",
            summary: "Unhide every hidden object across all open documents (undoable).",
            example_json: r#"{"op":"show_all"}"#,
        },
        OpDescriptor {
            op: "lock_set",
            category: "Layers & Appearance",
            summary: "Lock or unlock target objects (undoable).",
            example_json: r#"{"op":"lock_set","locked":true}"#,
        },
        OpDescriptor {
            op: "unlock_all",
            category: "Layers & Appearance",
            summary: "Unlock every locked object across all open documents (undoable).",
            example_json: r#"{"op":"unlock_all"}"#,
        },
        OpDescriptor {
            op: "color_set",
            category: "Layers & Appearance",
            summary: "Set or clear face (`edge: false`) or edge (`edge: true`) RGB override (undoable).",
            example_json: r#"{"op":"color_set","edge":false,"color":[240,90,40]}"#,
        },
        OpDescriptor {
            op: "alpha_set",
            category: "Layers & Appearance",
            summary: "Set (`0.0..=1.0`) or clear (`null`) per-object alpha transparency override (undoable).",
            example_json: r#"{"op":"alpha_set","alpha":0.4}"#,
        },
        OpDescriptor {
            op: "scene_inspect",
            category: "Query & Measure",
            summary: "Return document counts, object/row counts, undo depth, bounds, camera, and view state.",
            example_json: r#"{"op":"scene_inspect"}"#,
        },
        OpDescriptor {
            op: "object_list",
            category: "Query & Measure",
            summary: "List scene objects with stable `(doc, guid)` refs, kinds, layers, colours, and bounds.",
            example_json: r#"{"op":"object_list"}"#,
        },
        OpDescriptor {
            op: "object_get",
            category: "Query & Measure",
            summary: "Inspect a single object by `(doc, guid)`.",
            example_json: r#"{"op":"object_get","target":{"doc":0,"guid":"..."}}"#,
        },
        OpDescriptor {
            op: "measure",
            category: "Query & Measure",
            summary: "Measure `length`, `area`, `volume`, `bounds_extent`, or point-to-point `distance`.",
            example_json: r#"{"op":"measure","what":{"type":"volume"}}"#,
        },
        OpDescriptor {
            op: "scene_open",
            category: "Document & History",
            summary: "Open a `.session` archive from disk, restoring documents, camera, and display settings.",
            example_json: r#"{"op":"scene_open","path":"model.session"}"#,
        },
        OpDescriptor {
            op: "scene_save",
            category: "Document & History",
            summary: "Save all documents, appearance overrides, camera, and view state to a `.session` archive.",
            example_json: r#"{"op":"scene_save","path":"model.session"}"#,
        },
        OpDescriptor {
            op: "scene_load_manifest",
            category: "Document & History",
            summary: "Clear the scene and load a `.yaml`/`.json`/`.pb` scene manifest or protobuf file.",
            example_json: r#"{"op":"scene_load_manifest","path":"assets/view_local.yaml"}"#,
        },
        OpDescriptor {
            op: "scene_import",
            category: "Document & History",
            summary: "Import a `.pb`, `.obj`, or `.session` file into the scene.",
            example_json: r#"{"op":"scene_import","path":"mesh.obj"}"#,
        },
        OpDescriptor {
            op: "scene_export_obj",
            category: "Document & History",
            summary: "Export selected or all mesh/BRep/surface/element geometry as Wavefront `.obj`.",
            example_json: r#"{"op":"scene_export_obj","path":"out.obj"}"#,
        },
        OpDescriptor {
            op: "history_undo",
            category: "Document & History",
            summary: "Undo the most recent geometry, layer, text, or appearance transaction.",
            example_json: r#"{"op":"history_undo"}"#,
        },
        OpDescriptor {
            op: "history_redo",
            category: "Document & History",
            summary: "Redo the most recently undone transaction.",
            example_json: r#"{"op":"history_redo"}"#,
        },
        OpDescriptor {
            op: "camera_get",
            category: "Camera, View & Clipping",
            summary: "Return camera position, target, distance, quaternion orientation, and projection.",
            example_json: r#"{"op":"camera_get"}"#,
        },
        OpDescriptor {
            op: "camera_set_view",
            category: "Camera, View & Clipping",
            summary: "Snap camera orientation to `top`, `bottom`, `front`, `back`, `left`, `right`, `iso`, or `isoback`.",
            example_json: r#"{"op":"camera_set_view","view":"iso"}"#,
        },
        OpDescriptor {
            op: "camera_set_pose",
            category: "Camera, View & Clipping",
            summary: "Set explicit camera `target`, `position` (in scene units), and optional `perspective`.",
            example_json: r#"{"op":"camera_set_pose","target":[0,0,0],"position":[500,-500,400],"perspective":true}"#,
        },
        OpDescriptor {
            op: "camera_fit",
            category: "Camera, View & Clipping",
            summary: "Fit camera framing to target objects, current selection, or the whole scene.",
            example_json: r#"{"op":"camera_fit"}"#,
        },
        OpDescriptor {
            op: "view_get",
            category: "Camera, View & Clipping",
            summary: "Return all active display flags (`arctic`, `wireframe`, `opacity`, `cull_backfaces`, etc.).",
            example_json: r#"{"op":"view_get"}"#,
        },
        OpDescriptor {
            op: "view_set",
            category: "Camera, View & Clipping",
            summary: "Patch one or more viewport display flags or sizes.",
            example_json: r#"{"op":"view_set","patch":{"arctic":true,"opacity":0.95}}"#,
        },
        OpDescriptor {
            op: "construction_plane_set",
            category: "Camera, View & Clipping",
            summary: "Set the construction plane (`xy`, `yz`, `zx`, `points`, `normal`, or `null` for view-facing).",
            example_json: r#"{"op":"construction_plane_set","plane":{"type":"xy","z":0.0}}"#,
        },
        OpDescriptor {
            op: "clipping_plane_add",
            category: "Camera, View & Clipping",
            summary: "Create a section clipping plane on `xy`, `yz`, `zx`, `points`, or `normal`.",
            example_json: r#"{"op":"clipping_plane_add","plane":{"type":"xy","z":120.0}}"#,
        },
        OpDescriptor {
            op: "clipping_set",
            category: "Camera, View & Clipping",
            summary: "Enable/disable clipping planes, switch cap fill (`solid_fill`), or flip selected planes.",
            example_json: r#"{"op":"clipping_set","enabled":true,"solid_fill":false}"#,
        },
        OpDescriptor {
            op: "render_frame",
            category: "Camera, View & Clipping",
            summary: "Render the current scene, camera, and view state to a `.ppm` image on native GPU hosts.",
            example_json: r#"{"op":"render_frame","width":900,"height":700,"path":"frame.ppm"}"#,
        },
        OpDescriptor {
            op: "perf_stats",
            category: "Diagnostics & Escape Hatch",
            summary: "Return rolling frame timing percentiles, CPU split, redraw causes, or headless counts.",
            example_json: r#"{"op":"perf_stats"}"#,
        },
        OpDescriptor {
            op: "command_run",
            category: "Diagnostics & Escape Hatch",
            summary: "Run any command-dock text line directly (`Box 0,0,0 10 20 30`, `View Wireframe`, etc.).",
            example_json: r#"{"op":"command_run","line":"Select All"}"#,
        },
    ]
}

/// Generate the Markdown reference documentation for `docs/operations.md` directly from `op_catalog()`.
pub fn reference_markdown() -> String {
    let mut out = String::from(
        "# Shared Operation API (`viewer-core::op`)\n\n\
         Every automation surface — the in-app command line, `session-viewer-cli`, browser `execute_op(json)`, \
         and the Python `session_mcp` FastMCP server — dispatches typed JSON operations through \
         `viewer_core::op::execute(&mut dyn CommandHost, Op) -> Result<Reply, OpError>`.\n\n\
         Objects are always addressed by stable `(doc, guid)` pairs (`ObjectRef`), never by volatile GPU row \
         or UI tree indices.\n\n\
         ## Request & Response Envelope\n\n\
         ```json\n\
         {\"id\": \"req-1\", \"op\": \"object_create\", \"kind\": {\"type\": \"box\", \"origin\": [0, 0, 0], \"dx\": 100, \"dy\": 50, \"dz\": 30}}\n\
         ```\n\n\
         ```json\n\
         {\"id\": \"req-1\", \"ok\": true, \"reply\": {\"message\": \"Created box\", \"created\": [{\"doc\": 0, \"guid\": \"...\"}], \"changed\": [], \"value\": null}}\n\
         ```\n\n\
         ## Operations\n\n\
         | `op` | Category | Summary | Example JSON |\n\
         |---|---|---|---|\n",
    );
    for d in op_catalog() {
        out.push_str(&format!(
            "| `{}` | {} | {} | `{}` |\n",
            d.op,
            d.category,
            d.summary,
            d.example_json.replace('|', "\\|")
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::command::HeadlessHost;

    #[test]
    fn every_catalog_example_deserializes_as_valid_op() {
        let catalog = op_catalog();
        assert!(catalog.len() >= 48);
        for item in &catalog {
            let req: Request = serde_json::from_str(item.example_json)
                .unwrap_or_else(|e| panic!("failed to parse example for `{}`: {e}", item.op));
            let round = serde_json::to_value(&req).unwrap();
            assert_eq!(round["op"], item.op);
        }
    }

    #[test]
    fn create_transform_measure_layer_appearance_and_undo_round_trip() {
        let mut host = HeadlessHost::new();

        // 1. Create a 10x20x30 BRep box and a line
        let box_reply = execute(
            &mut host,
            Op::ObjectCreate {
                kind: CreateKind::Box {
                    origin: [0.0, 0.0, 0.0],
                    dx: 10.0,
                    dy: 20.0,
                    dz: 30.0,
                    mesh: false,
                },
                layer: None,
            },
        )
        .unwrap();
        assert_eq!(box_reply.created.len(), 1);
        let box_ref = box_reply.created[0].clone();

        let line_reply = execute(
            &mut host,
            Op::ObjectCreate {
                kind: CreateKind::Line {
                    start: [0.0, 0.0, 0.0],
                    end: [30.0, 40.0, 0.0],
                },
                layer: None,
            },
        )
        .unwrap();
        let line_ref = line_reply.created[0].clone();

        // 2. Measure line length (= 50) and box volume (= 6000) by explicit ObjectRef
        let len = execute(
            &mut host,
            Op::Measure {
                what: MeasureKind::Length,
                targets: Targets::Refs(vec![line_ref.clone()]),
            },
        )
        .unwrap();
        assert!((len.value["length"].as_f64().unwrap() - 50.0).abs() < 1e-6);

        let vol = execute(
            &mut host,
            Op::Measure {
                what: MeasureKind::Volume,
                targets: Targets::Refs(vec![box_ref.clone()]),
            },
        )
        .unwrap();
        assert!((vol.value["volume_approx"].as_f64().unwrap() - 6000.0).abs() < 1e-3);

        // 3. Explicit-pivot 90-degree Z rotation + scale 2x on box
        execute(
            &mut host,
            Op::ObjectTransform {
                targets: Targets::Refs(vec![box_ref.clone()]),
                xform: XformSpec::Rotate {
                    axis: AxisSpec::Z,
                    degrees: 90.0,
                    pivot: Some([5.0, 10.0, 0.0]),
                },
            },
        )
        .unwrap();
        execute(
            &mut host,
            Op::ObjectTransform {
                targets: Targets::Refs(vec![box_ref.clone()]),
                xform: XformSpec::Scale {
                    factor: [2.0, 2.0, 2.0],
                    pivot: Some([0.0, 0.0, 0.0]),
                },
            },
        )
        .unwrap();
        let ext = execute(
            &mut host,
            Op::Measure {
                what: MeasureKind::BoundsExtent,
                targets: Targets::Refs(vec![box_ref.clone()]),
            },
        )
        .unwrap();
        let dims = ext.value["extent"].as_array().unwrap();
        assert!((dims[0].as_f64().unwrap() - 40.0).abs() < 1e-6);
        assert!((dims[1].as_f64().unwrap() - 20.0).abs() < 1e-6);
        assert!((dims[2].as_f64().unwrap() - 60.0).abs() < 1e-6);

        // 4. Layer CRUD by (doc, name) + assign + appearance + undo/redo
        execute(
            &mut host,
            Op::LayerCreate {
                parent: None,
                name: Some("Timber".into()),
                sublayer: false,
            },
        )
        .unwrap();
        execute(
            &mut host,
            Op::LayerAssign {
                targets: Targets::Refs(vec![box_ref.clone()]),
                layer: LayerRef {
                    doc: Some(DocRef::Index(0)),
                    name: "Timber".into(),
                },
            },
        )
        .unwrap();
        execute(
            &mut host,
            Op::AlphaSet {
                targets: Targets::Refs(vec![box_ref.clone()]),
                alpha: Some(0.5),
            },
        )
        .unwrap();
        let got = execute(
            &mut host,
            Op::ObjectGet {
                target: box_ref.clone(),
            },
        )
        .unwrap();
        assert_eq!(got.value["layer"], "Timber");
        assert!((got.value["alpha"].as_f64().unwrap() - 0.5).abs() < 1e-6);

        // Undo alpha, then redo alpha
        execute(&mut host, Op::HistoryUndo).unwrap();
        let got_undo = execute(
            &mut host,
            Op::ObjectGet {
                target: box_ref.clone(),
            },
        )
        .unwrap();
        assert!(got_undo.value["alpha"].is_null());
        execute(&mut host, Op::HistoryRedo).unwrap();
        let got_redo = execute(&mut host, Op::ObjectGet { target: box_ref }).unwrap();
        assert!((got_redo.value["alpha"].as_f64().unwrap() - 0.5).abs() < 1e-6);

        // 5. JSON envelope round-trip + structured error code check
        let raw_ok = execute_json(&mut host, r#"{"id":"42","op":"scene_inspect"}"#);
        let parsed_ok: Response = serde_json::from_str(&raw_ok).unwrap();
        assert_eq!(parsed_ok.id.as_deref(), Some("42"));
        assert!(parsed_ok.ok);
        assert_eq!(parsed_ok.reply.unwrap().value["object_count"], 2);

        let raw_err = execute_json(&mut host, r#"{"id":"43","op":"selection_clear"}"#);
        assert!(serde_json::from_str::<Response>(&raw_err).unwrap().ok);
        let del_err = execute(
            &mut host,
            Op::ObjectDelete {
                targets: Targets::Selection,
            },
        )
        .unwrap_err();
        assert_eq!(del_err.code, ErrorCode::NoSelection);
    }

    #[test]
    fn command_line_and_typed_ops_yield_identical_session_bytes() {
        let mut base = Session::new("fixture");
        let mesh = session_rust::Mesh::create_box(100.0, 200.0, 50.0);
        let guid = mesh.guid().to_string();
        base.add_mesh(mesh, None);
        let pb = base.pb_dumps();

        // Path A: command line verbs
        let mut host_cmd = HeadlessHost::new();
        host_cmd.add_file(FileDoc {
            name: "fixture.pb".into(),
            place: Xform::identity(),
            session: Rc::new(Session::pb_loads(&pb).unwrap()),
            point_px: 0.0,
            display_only: false,
        });
        host_cmd.run_command("Select All").unwrap();
        host_cmd.run_command("Move 25,50,10").unwrap();
        host_cmd.run_command("Alpha 0.6").unwrap();
        host_cmd.run_command("Arctic On").unwrap();
        host_cmd.run_command("View Isometric").unwrap();
        host_cmd.save_session().unwrap();
        let bytes_cmd = host_cmd.saved_bytes.clone().unwrap();

        // Path B: typed Op::* calls
        let mut host_op = HeadlessHost::new();
        host_op.add_file(FileDoc {
            name: "fixture.pb".into(),
            place: Xform::identity(),
            session: Rc::new(Session::pb_loads(&pb).unwrap()),
            point_px: 0.0,
            display_only: false,
        });
        let target = ObjectRef::of(0, guid);
        execute(
            &mut host_op,
            Op::ObjectTransform {
                targets: Targets::Refs(vec![target.clone()]),
                xform: XformSpec::Translate {
                    delta: [25.0, 50.0, 10.0],
                },
            },
        )
        .unwrap();
        execute(
            &mut host_op,
            Op::AlphaSet {
                targets: Targets::Refs(vec![target]),
                alpha: Some(0.6),
            },
        )
        .unwrap();
        execute(
            &mut host_op,
            Op::ViewSet {
                patch: ViewPatch {
                    arctic: Some(true),
                    ..Default::default()
                },
            },
        )
        .unwrap();
        execute(&mut host_op, Op::CameraSetView { view: "iso".into() }).unwrap();
        execute(&mut host_op, Op::SceneSave { path: None }).unwrap();
        let bytes_op = host_op.saved_bytes.clone().unwrap();

        assert_eq!(
            bytes_cmd, bytes_op,
            "command line and typed Op must produce byte-identical .session archives"
        );
    }

    #[test]
    fn headless_create_and_transform_throughput_benchmark() {
        let mut host = HeadlessHost::new();
        let start = std::time::Instant::now();
        let iters = 200usize;
        let mut refs = Vec::with_capacity(iters);
        for i in 0..iters {
            let reply = execute(
                &mut host,
                Op::ObjectCreate {
                    kind: CreateKind::Line {
                        start: [i as f64, 0.0, 0.0],
                        end: [i as f64, 100.0, 0.0],
                    },
                    layer: None,
                },
            )
            .unwrap();
            refs.push(reply.created[0].clone());
        }
        for r in &refs {
            execute(
                &mut host,
                Op::ObjectTransform {
                    targets: Targets::Refs(vec![r.clone()]),
                    xform: XformSpec::Translate {
                        delta: [10.0, 20.0, 5.0],
                    },
                },
            )
            .unwrap();
        }
        let elapsed = start.elapsed().as_secs_f64();
        let total_ops = (iters * 2) as f64;
        let ops_per_sec = total_ops / elapsed.max(1e-9);
        println!(
            "viewer-core::op headless throughput: {:.0} ops/s ({} ops in {:.2} ms)",
            ops_per_sec,
            iters * 2,
            elapsed * 1000.0
        );
        assert!(
            ops_per_sec > 200.0,
            "headless throughput too slow: {ops_per_sec:.1} ops/s"
        );
    }
}
