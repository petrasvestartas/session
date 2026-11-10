use super::net::{Net, tex_name};
use super::tex::{Rgba, hex};
use serde_yaml_ng::{Mapping, Value};
use std::collections::HashMap;

/// The deck index: its title and chapter files.
#[derive(Clone, Debug)]
pub struct Index {
    pub title: String,
    pub chapters: Vec<String>,
}

/// One chapter file: a 3D scene shared by its slides.
#[derive(Clone, Debug)]
pub struct Chapter {
    pub title: String,
    pub palette: HashMap<String, Rgba>,
    pub points: Vec<(String, [f64; 3])>,
    pub camera: CameraSpec,
    pub split: f32,
    pub grid: Option<Grid>,
    pub axes: bool,
    pub slides: Vec<Slide>,
}

/// A square floor grid in the plane `z`.
#[derive(Clone, Debug)]
pub struct Grid {
    pub center: [f64; 2],
    pub size: f64,
    pub step: f64,
    pub z: f64,
}

/// An authored view: a named view, angles, zoom, centre or points to frame.
#[derive(Clone, Debug, Default)]
pub struct CameraSpec {
    pub yaw: Option<f64>,
    pub pitch: Option<f64>,
    pub zoom: Option<f64>,
    pub center: Option<Pos>,
    pub fit: Vec<Pos>,
    pub view: Option<String>,
}

/// One slide: a title and its steps; `keep` carries the panel over from the slide before.
#[derive(Clone, Debug)]
pub struct Slide {
    pub title: String,
    pub keep: bool,
    pub steps: Vec<Step>,
}

/// One click: the narration and the actions it plays.
#[derive(Clone, Debug)]
pub struct Step {
    pub say: Option<String>,
    pub actions: Vec<Action>,
}

/// One timed action of a step.
#[derive(Clone, Debug)]
pub struct Action {
    pub kind: Kind,
    pub dur: Option<f64>,
    pub with: bool,
    pub delay: f64,
}

/// What an action does.
#[derive(Clone, Debug)]
pub enum Kind {
    Add(Vec<ObjSpec>),
    Remove(Vec<String>),
    Morph {
        target: String,
        to: Content,
        map: Vec<(String, String)>,
    },
    Derive {
        from: String,
        obj: ObjSpec,
        map: Vec<(String, String)>,
    },
    Fly {
        from: String,
        to: String,
        keep: bool,
    },
    Indicate(Vec<String>, Option<String>),
    Box {
        id: String,
        target: String,
        color: Option<String>,
    },
    Color {
        target: String,
        color: String,
    },
    Move {
        point: String,
        to: Pos,
    },
    Camera(CameraSpec),
    Wait,
    Set {
        target: String,
        tex: String,
    },
    Dim(Vec<String>),
    Undim(Vec<String>),
    Clear,
    Show(Vec<String>),
    Hide(Vec<String>),
    Play(Vec<Action>),
}

/// The geometry or formula of an object.
#[derive(Clone, Debug)]
pub enum Content {
    Point(Pos),
    Line { a: Pos, b: Pos, arrow: bool },
    Polyline { pts: Vec<Pos>, closed: bool },
    Cuboid { a: Pos, b: Pos },
    Label { at: Pos, tex: String },
    Tex(String),
    Text(String),
    Code(String),
    Image { src: String, height: f32 },
    Matrix(MatrixSpec),
    Row(Vec<RowItem>),
}

/// One formula or matrix of a row, optionally named and coloured.
#[derive(Clone, Debug)]
pub struct RowItem {
    pub id: Option<String>,
    pub content: Content,
    pub color: Option<String>,
}

/// A matrix of rounded cells.
#[derive(Clone, Debug, Default)]
pub struct MatrixSpec {
    pub rows: Vec<Vec<String>>,           // Entries as TeX.
    pub name: Option<String>,             // TeX written under the matrix.
    pub rowlabels: Vec<String>,           // TeX left of each row.
    pub collabels: Vec<String>,           // TeX above each column.
    pub empty: bool,                      // Entries start hidden.
    pub signs: bool,                      // Cells are coloured by the sign of their entry.
    pub cell: Option<f64>,                // Cell size in em.
    pub colors: Vec<Vec<Option<String>>>, // Named cell colours.
    pub pattern: bool,                    // Only the nonzero cells, without entries.
}

/// An object to create.
#[derive(Clone, Debug)]
pub struct ObjSpec {
    pub id: String,
    pub content: Content,
    pub color: Option<String>,
    pub size: Option<f64>,
    pub width: Option<f64>,
    pub dash: bool,
    pub align: Option<String>,
    pub at: Option<[f64; 2]>,
    pub anim: Option<String>,
    pub label: Option<String>,
    pub label_color: Option<String>,
    pub dir: Option<[f64; 2]>,
    pub lag: Option<f64>,
    pub gap: Option<f64>,
    pub panel: Option<String>,
    pub opacity: Option<f64>,
}

/// A 3D position: a named point, coordinates, or a small expression.
#[derive(Clone, Debug)]
pub enum Pos {
    Name(String),
    Abs([Scalar; 3]),
    Expr(String),
}

/// A coordinate: a number or an expression.
#[derive(Clone, Debug)]
pub enum Scalar {
    N(f64),
    E(String),
}

/// An error at a place in the deck.
fn err<T>(at: &str, msg: impl std::fmt::Display) -> Result<T, String> {
    Err(format!("{at}: {msg}"))
}

/// A scalar value as text.
fn text(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(num_text(n.as_f64().unwrap_or(0.0))),
        Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// A number as a matrix entry: integers bare, otherwise at most three decimals.
pub fn num_text(x: f64) -> String {
    if (x - x.round()).abs() < 1e-9 {
        return format!("{}", x.round() as i64);
    }
    let mut s = format!("{x:.3}");
    while s.ends_with('0') {
        s.pop();
    }
    s
}

/// A number, also written as a string.
fn num(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

/// The value of `key` in a map.
fn get<'a>(m: &'a Mapping, key: &str) -> Option<&'a Value> {
    m.get(Value::String(key.into()))
}

/// A sequence, a single value as a one-item list, or nothing for null.
fn list(v: &Value) -> Vec<Value> {
    match v {
        Value::Sequence(s) => s.clone(),
        Value::Null => Vec::new(),
        other => vec![other.clone()],
    }
}

/// A list of scalars as text.
fn strings(v: &Value) -> Vec<String> {
    list(v).iter().filter_map(text).collect()
}

/// A position: a point name, `[x, y, z]` or an expression.
pub fn pos(v: &Value, at: &str) -> Result<Pos, String> {
    match v {
        Value::String(s) => {
            let s = s.trim();
            if s.chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
                && !s.is_empty()
            {
                Ok(Pos::Name(s.to_string()))
            } else {
                Ok(Pos::Expr(s.to_string()))
            }
        }
        Value::Sequence(s) if s.len() == 3 => {
            let mut out = [Scalar::N(0.0), Scalar::N(0.0), Scalar::N(0.0)];
            for (i, c) in s.iter().enumerate() {
                out[i] = match c {
                    Value::Number(n) => Scalar::N(n.as_f64().unwrap_or(0.0)),
                    Value::String(e) => Scalar::E(e.clone()),
                    _ => return err(at, "a coordinate must be a number or an expression"),
                };
            }
            Ok(Pos::Abs(out))
        }
        _ => err(at, "a position is a point name, [x, y, z] or an expression"),
    }
}

/// Two numbers.
fn pair(v: &Value) -> Option<[f64; 2]> {
    match v {
        Value::Sequence(s) if s.len() == 2 => Some([num(&s[0])?, num(&s[1])?]),
        _ => None,
    }
}

/// A camera: a view name or a map.
fn camera(v: &Value, at: &str) -> Result<CameraSpec, String> {
    let mut c = CameraSpec::default();
    let Value::Mapping(m) = v else {
        if let Some(view) = text(v) {
            c.view = Some(view);
            return Ok(c);
        }
        return err(at, "camera is a map or a view name");
    };
    c.yaw = get(m, "yaw").and_then(num);
    c.pitch = get(m, "pitch").and_then(num);
    c.zoom = get(m, "zoom").and_then(num);
    c.view = get(m, "view").and_then(text);
    if let Some(p) = get(m, "center") {
        c.center = Some(pos(p, at)?);
    }
    if let Some(f) = get(m, "fit") {
        for p in list(f) {
            c.fit.push(pos(&p, at)?);
        }
    }
    Ok(c)
}

/// A matrix: a net matrix by name, `of:` one with overrides, or rows written out.
fn matrix(v: &Value, at: &str, cx: &Cx) -> Result<MatrixSpec, String> {
    let named = |name: &str| {
        cx.net
            .as_ref()
            .and_then(|net| net.matrix(name))
            .ok_or_else(|| format!("{at}: the net has no matrix {name}"))
    };
    if let Value::String(name) = v {
        return named(name);
    }
    let mut spec = MatrixSpec {
        signs: true,
        ..MatrixSpec::default()
    };
    let rows_value = match v {
        Value::Mapping(m) => {
            if let Some(name) = get(m, "of").and_then(text) {
                spec = named(&name)?;
                spec.pattern = get(m, "pattern")
                    .and_then(Value::as_bool)
                    .unwrap_or(spec.pattern);
                spec.name = get(m, "name").and_then(text).or(spec.name);
                spec.empty = get(m, "empty").and_then(Value::as_bool).unwrap_or(false);
                spec.cell = get(m, "cell").and_then(num);
                return Ok(spec);
            }
            spec.pattern = get(m, "pattern").and_then(Value::as_bool).unwrap_or(false);
            spec.name = get(m, "name").and_then(text);
            spec.rowlabels = get(m, "rowlabels").map(strings).unwrap_or_default();
            spec.collabels = get(m, "collabels").map(strings).unwrap_or_default();
            spec.empty = get(m, "empty").and_then(Value::as_bool).unwrap_or(false);
            spec.signs = get(m, "signs").and_then(Value::as_bool).unwrap_or(true);
            spec.cell = get(m, "cell").and_then(num);
            if let Some(c) = get(m, "colors") {
                spec.colors = list(c)
                    .iter()
                    .map(|row| {
                        list(row)
                            .iter()
                            .map(|c| text(c).filter(|s| s != "~" && !s.is_empty()))
                            .collect()
                    })
                    .collect();
            }
            get(m, "rows").cloned().unwrap_or(Value::Null)
        }
        other => other.clone(),
    };
    for row in list(&rows_value) {
        let cells: Vec<String> = match &row {
            Value::Sequence(s) => s.iter().map(|c| text(c).unwrap_or_default()).collect(),
            other => vec![text(other).unwrap_or_default()],
        };
        spec.rows.push(cells);
    }
    if spec.rows.is_empty() {
        return err(at, "a matrix needs rows");
    }
    let cols = spec.rows[0].len();
    if spec.rows.iter().any(|r| r.len() != cols) || cols == 0 {
        return err(at, "every matrix row needs the same number of entries");
    }
    Ok(spec)
}

/// Keys that make an object, in the order they are looked for.
const TYPES: [&str; 13] = [
    "point", "line", "arrow", "polyline", "polygon", "cuboid", "label", "tex", "text", "code",
    "image", "matrix", "row",
];

/// The geometry or formula of an object map.
fn content(m: &Mapping, at: &str, cx: &Cx) -> Result<Content, String> {
    for key in TYPES {
        let Some(v) = get(m, key) else { continue };
        let at = &format!("{at} {key}");
        return Ok(match key {
            "point" => Content::Point(pos(v, at)?),
            "line" | "arrow" => {
                let pts = list(v);
                if pts.len() != 2 {
                    return err(at, "a line needs two positions");
                }
                Content::Line {
                    a: pos(&pts[0], at)?,
                    b: pos(&pts[1], at)?,
                    arrow: key == "arrow",
                }
            }
            "polyline" | "polygon" => Content::Polyline {
                pts: list(v)
                    .iter()
                    .map(|p| pos(p, at))
                    .collect::<Result<_, _>>()?,
                closed: key == "polygon",
            },
            "cuboid" => {
                let pts = list(v);
                if pts.len() != 2 {
                    return err(at, "a cuboid needs two opposite corners");
                }
                Content::Cuboid {
                    a: pos(&pts[0], at)?,
                    b: pos(&pts[1], at)?,
                }
            }
            "label" => {
                let tex = get(m, "tex")
                    .and_then(text)
                    .ok_or_else(|| format!("{at}: a label needs `tex`"))?;
                Content::Label {
                    at: pos(v, at)?,
                    tex,
                }
            }
            "tex" => Content::Tex(text(v).unwrap_or_default()),
            "text" => Content::Text(text(v).unwrap_or_default()),
            "code" => Content::Code(text(v).unwrap_or_default()),
            "image" => match v {
                Value::Mapping(im) => Content::Image {
                    src: get(im, "src").and_then(text).unwrap_or_default(),
                    height: get(im, "height").and_then(num).unwrap_or(7.0) as f32,
                },
                other => Content::Image {
                    src: text(other).unwrap_or_default(),
                    height: 7.0,
                },
            },
            "matrix" => Content::Matrix(matrix(v, at, cx)?),
            "row" => {
                let mut items = Vec::new();
                for (i, item) in list(v).iter().enumerate() {
                    let at = &format!("{at}[{i}]");
                    items.push(match item {
                        Value::Mapping(im) => RowItem {
                            id: get(im, "id").and_then(text),
                            content: content(im, at, cx)?,
                            color: get(im, "color").and_then(text),
                        },
                        other => RowItem {
                            id: None,
                            content: Content::Tex(text(other).unwrap_or_default()),
                            color: None,
                        },
                    });
                }
                Content::Row(items)
            }
            _ => unreachable!(),
        });
    }
    err(at, format!("an object needs one of {}", TYPES.join(", ")))
}

/// An object to add; plain text becomes a formula with an automatic id.
fn object(v: &Value, at: &str, cx: &mut Cx) -> Result<ObjSpec, String> {
    let Value::Mapping(m) = v else {
        if let Some(t) = text(v) {
            return Ok(plain(cx.auto(), Content::Tex(t)));
        }
        return err(at, "an object is a map");
    };
    // a `label` key next to `tex` makes a 3D label; next to point/line it names the attached label
    let is_label = get(m, "label").is_some()
        && get(m, "tex").is_some()
        && TYPES[..6].iter().all(|k| get(m, k).is_none());
    let content = if is_label {
        Content::Label {
            at: pos(get(m, "label").unwrap(), at)?,
            tex: get(m, "tex").and_then(text).unwrap_or_default(),
        }
    } else {
        content(m, at, cx)?
    };
    let id = match (get(m, "id").and_then(text), get(m, "matrix")) {
        (Some(id), _) => id,
        (None, Some(Value::String(name))) => name.clone(),
        (None, Some(Value::Mapping(mm))) => match get(mm, "id").or(get(mm, "of")).and_then(text) {
            Some(id) => id,
            None => cx.auto(),
        },
        _ => cx.auto(),
    };
    if let Content::Line { a, b, .. } = &content {
        cx.ends.insert(id.clone(), (written(a), written(b)));
    }
    if let Content::Matrix(spec) = &content {
        let number = |entry: &String| entry.replace('−', "-").trim().parse::<f64>().ok();
        let rows: Option<Vec<Vec<f64>>> =
            spec.rows.iter().map(|row| row.iter().map(number).collect()).collect();
        if let Some(rows) = rows {
            cx.tables.insert(id.clone(), rows);
        }
    }
    Ok(ObjSpec {
        id,
        content,
        color: get(m, "color").and_then(text),
        size: get(m, "size").and_then(num),
        width: get(m, "width").and_then(num),
        dash: get(m, "dash").and_then(Value::as_bool).unwrap_or(false),
        align: get(m, "align").and_then(text),
        at: get(m, "at").and_then(pair),
        anim: get(m, "anim").and_then(text),
        label: if is_label {
            None
        } else {
            get(m, "label").and_then(text)
        },
        label_color: get(m, "labelcolor").and_then(text),
        dir: get(m, "dir").and_then(pair),
        lag: get(m, "lag").and_then(num),
        gap: get(m, "gap").and_then(num),
        panel: get(m, "panel").and_then(text),
        opacity: get(m, "opacity").and_then(num),
    })
}

/// An object with every option left at its default.
pub fn plain(id: String, content: Content) -> ObjSpec {
    ObjSpec {
        id,
        content,
        color: None,
        size: None,
        width: None,
        dash: false,
        align: None,
        at: None,
        anim: None,
        label: None,
        label_color: None,
        dir: None,
        lag: None,
        gap: None,
        panel: None,
        opacity: None,
    }
}

/// A morph map as pairs of source and target references.
fn mapping(v: Option<&Value>) -> Vec<(String, String)> {
    match v {
        Some(Value::Mapping(m)) => m
            .iter()
            .filter_map(|(k, v)| Some((text(k)?, text(v)?)))
            .collect(),
        _ => Vec::new(),
    }
}

/// Keys that make an action.
const KINDS: [&str; 19] = [
    "add", "remove", "morph", "derive", "fly", "indicate", "box", "color", "move", "camera",
    "wait", "set", "dim", "undim", "clear", "show", "hide", "play", "write",
];

/// One action; `clear` and `wait` may be written bare.
fn action(v: &Value, at: &str, cx: &mut Cx) -> Result<Action, String> {
    let Value::Mapping(m) = v else {
        return match text(v).as_deref() {
            Some("clear") => Ok(act(Kind::Clear)),
            Some("wait") => Ok(Action {
                dur: Some(1.0),
                ..act(Kind::Wait)
            }),
            _ => err(at, "an action is a map such as `add: {...}`"),
        };
    };
    let key = KINDS
        .iter()
        .find(|k| get(m, k).is_some())
        .ok_or_else(|| format!("{at}: an action needs one of {}", KINDS.join(", ")))?;
    let v = get(m, key).unwrap();
    let at = &format!("{at} {key}");
    let kind = match *key {
        "add" | "write" => {
            let mut objs = Vec::new();
            for o in list(v) {
                objs.push(object(&o, at, cx)?);
            }
            Kind::Add(objs)
        }
        "remove" => Kind::Remove(strings(v)),
        "morph" => {
            let Value::Mapping(mm) = v else {
                return err(at, "morph: {target: id, tex: ...}");
            };
            let target = get(mm, "target")
                .and_then(text)
                .ok_or_else(|| format!("{at}: needs `target`"))?;
            Kind::Morph {
                target,
                to: content(mm, at, cx)?,
                map: mapping(get(mm, "map")),
            }
        }
        "derive" => {
            let Value::Mapping(mm) = v else {
                return err(at, "derive: {from: id, id: new, tex: ...}");
            };
            let from = get(mm, "from")
                .and_then(text)
                .ok_or_else(|| format!("{at}: needs `from`"))?;
            Kind::Derive {
                from,
                obj: object(v, at, cx)?,
                map: mapping(get(mm, "map")),
            }
        }
        "fly" => {
            let Value::Mapping(mm) = v else {
                return err(at, "fly: {from: ref, to: ref}");
            };
            Kind::Fly {
                from: get(mm, "from")
                    .and_then(text)
                    .ok_or_else(|| format!("{at}: needs `from`"))?,
                to: get(mm, "to")
                    .and_then(text)
                    .ok_or_else(|| format!("{at}: needs `to`"))?,
                keep: get(mm, "keep").and_then(Value::as_bool).unwrap_or(true),
            }
        }
        "indicate" => Kind::Indicate(strings(v), get(m, "color").and_then(text)),
        "box" => match v {
            Value::Mapping(mm) => {
                let target = get(mm, "target")
                    .and_then(text)
                    .ok_or_else(|| format!("{at}: needs `target`"))?;
                Kind::Box {
                    id: get(mm, "id")
                        .and_then(text)
                        .unwrap_or_else(|| format!("box:{target}")),
                    target,
                    color: get(mm, "color").and_then(text),
                }
            }
            other => {
                let target = text(other).unwrap_or_default();
                Kind::Box {
                    id: format!("box:{target}"),
                    target,
                    color: get(m, "color").and_then(text),
                }
            }
        },
        "color" => {
            let Value::Mapping(mm) = v else {
                return err(at, "color: {target: ref, color: name}");
            };
            Kind::Color {
                target: get(mm, "target").and_then(text).unwrap_or_default(),
                color: get(mm, "color").and_then(text).unwrap_or_default(),
            }
        }
        "move" => {
            let Value::Mapping(mm) = v else {
                return err(at, "move: {point: P0, to: [x, y, z]}");
            };
            Kind::Move {
                point: get(mm, "point")
                    .and_then(text)
                    .ok_or_else(|| format!("{at}: needs `point`"))?,
                to: pos(
                    get(mm, "to").ok_or_else(|| format!("{at}: needs `to`"))?,
                    at,
                )?,
            }
        }
        "camera" => Kind::Camera(camera(v, at)?),
        "wait" => Kind::Wait,
        "set" => {
            let Value::Mapping(mm) = v else {
                return err(at, "set: {target: M[0,0], tex: ...}");
            };
            Kind::Set {
                target: get(mm, "target").and_then(text).unwrap_or_default(),
                tex: get(mm, "tex").and_then(text).unwrap_or_default(),
            }
        }
        "dim" => Kind::Dim(strings(v)),
        "undim" => Kind::Undim(strings(v)),
        "clear" => Kind::Clear,
        "show" => Kind::Show(strings(v)),
        "hide" => Kind::Hide(strings(v)),
        "play" => {
            let mut acts = Vec::new();
            for (i, a) in list(v).iter().enumerate() {
                let mut a = action(a, &format!("{at}[{i}]"), cx)?;
                a.with = i > 0;
                acts.push(a);
            }
            Kind::Play(acts)
        }
        _ => unreachable!(),
    };
    // timing may sit beside the action key or inside its map
    let inner = |k: &str| {
        get(m, k).or(match v {
            Value::Mapping(vm) => get(vm, k),
            _ => None,
        })
    };
    let wait = if *key == "wait" {
        num(v).or(Some(1.0))
    } else {
        None
    };
    let dur = inner("dur").and_then(num).or(wait);
    Ok(Action {
        kind,
        dur,
        with: inner("with").and_then(Value::as_bool).unwrap_or(false),
        delay: inner("delay").and_then(num).unwrap_or(0.0),
    })
}

/// The default colors every chapter starts from.
pub fn default_palette() -> HashMap<String, Rgba> {
    let mut p = HashMap::new();
    for (k, v) in [
        ("ink", "#e8ecf2"),
        ("dim", "#7d8796"),
        ("faint", "#3a414c"),
        ("free", "#dc4b8d"),
        ("fixed", "#5b9cf6"),
        ("load", "#4fc775"),
        ("x", "#ef7770"),
        ("y", "#62cc7c"),
        ("z", "#72a4ef"),
        ("accent", "#f2c14e"),
        ("bar", "#c9ced6"),
        ("pos", "#dc4b8d"),
        ("neg", "#2b6fdb"),
        ("cell", "#252b36"),
        ("bg", "#0e1013"),
    ] {
        p.insert(k.to_string(), hex(v).unwrap());
    }
    p
}

/// Parse `index.yaml`.
pub fn index(src: &str) -> Result<Index, String> {
    let v: Value = serde_yaml_ng::from_str(src).map_err(|e| format!("index: {e}"))?;
    let Value::Mapping(m) = &v else {
        return err("index", "expected a map");
    };
    let chapters = get(m, "chapters").map(strings).unwrap_or_default();
    if chapters.is_empty() {
        return err("index", "lists no chapters");
    }
    Ok(Index {
        title: get(m, "title")
            .or(get(m, "name"))
            .and_then(text)
            .unwrap_or_default(),
        chapters,
    })
}

/// Parse one chapter file.
pub fn chapter(src: &str, name: &str) -> Result<Chapter, String> {
    let v: Value = serde_yaml_ng::from_str(src).map_err(|e| format!("{name}: {e}"))?;
    let Value::Mapping(m) = &v else {
        return err(name, "expected a map");
    };
    let mut palette = default_palette();
    if let Some(Value::Mapping(p)) = get(m, "palette") {
        for (k, v) in p {
            let (Some(k), Some(v)) = (text(k), text(v)) else {
                continue;
            };
            let rgba = palette
                .get(&v)
                .copied()
                .or_else(|| hex(&v))
                .ok_or_else(|| format!("{name}: bad color {v}"))?;
            palette.insert(k, rgba);
        }
    }
    let mut points = Vec::new();
    if let Some(Value::Mapping(p)) = get(m, "points") {
        for (k, v) in p {
            let k = text(k).unwrap_or_default();
            let c = list(v);
            if c.len() != 3 {
                return err(name, format!("point {k} needs [x, y, z]"));
            }
            points.push((
                k,
                [
                    num(&c[0]).unwrap_or(0.0),
                    num(&c[1]).unwrap_or(0.0),
                    num(&c[2]).unwrap_or(0.0),
                ],
            ));
        }
    }
    let grid = match get(m, "grid") {
        Some(Value::Mapping(g)) => Some(Grid {
            center: get(g, "center").and_then(pair).unwrap_or([0.0, 0.0]),
            size: get(g, "size").and_then(num).unwrap_or(10.0),
            step: get(g, "step").and_then(num).unwrap_or(1.0),
            z: get(g, "z").and_then(num).unwrap_or(0.0),
        }),
        Some(Value::Bool(true)) => Some(Grid {
            center: [0.0, 0.0],
            size: 10.0,
            step: 1.0,
            z: 0.0,
        }),
        _ => None,
    };
    let mut cx = Cx::default();
    cx.home = points.iter().cloned().collect();
    if let Some(Value::Mapping(blocks)) = get(m, "blocks") {
        for (k, v) in blocks {
            cx.blocks.insert(text(k).unwrap_or_default(), v.clone());
        }
    }
    if let Some(v) = get(m, "net") {
        let net = net(v, name)?;
        for (k, at) in net.names.iter().zip(&net.at) {
            points.push((k.clone(), *at));
        }
        // the camera frames every point, so the solved shape is in view before it is reached
        for (k, at) in net.names.iter().zip(net.solve()) {
            points.push((format!("{k}_rest"), at));
        }
        cx.net = Some(net);
        cx.home = points.iter().cloned().collect();
    }
    let mut slides = Vec::new();
    let slide_values = get(m, "slides").map(list).unwrap_or_default();
    for (si, s) in slide_values.iter().enumerate() {
        let Value::Mapping(sm) = s else {
            return err(name, format!("slide {} is not a map", si + 1));
        };
        let title = get(sm, "title").and_then(text).unwrap_or_default();
        let mut steps = Vec::new();
        let step_values = get(sm, "steps").map(list).unwrap_or_default();
        for (ti, t) in step_values.iter().enumerate() {
            let at = format!("{name} slide {} step {}", si + 1, ti + 1);
            let Value::Mapping(tm) = t else {
                return err(&at, "a step is a map with `say` and `do`");
            };
            steps.push(Step {
                say: get(tm, "say").and_then(text),
                actions: step(tm, &at, &mut cx)?,
            });
        }
        if steps.is_empty() {
            steps.push(Step {
                say: None,
                actions: Vec::new(),
            });
        }
        slides.push(Slide {
            title,
            keep: get(sm, "keep").and_then(Value::as_bool).unwrap_or(false),
            steps,
        });
    }
    if slides.is_empty() {
        return err(name, "has no slides");
    }
    Ok(Chapter {
        title: get(m, "title")
            .or(get(m, "name"))
            .and_then(text)
            .unwrap_or_default(),
        palette,
        points,
        camera: match get(m, "camera").or(get(m, "view")) {
            Some(c) => camera(c, name)?,
            None => CameraSpec::default(),
        },
        split: get(m, "split").and_then(num).unwrap_or(0.46) as f32,
        grid,
        axes: get(m, "axes").and_then(Value::as_bool).unwrap_or(false),
        slides,
    })
}

/// What a chapter's steps share while they are read.
#[derive(Default)]
struct Cx {
    n: usize,           // Automatic ids handed out.
    net: Option<Net>,   // The chapter's net.
    eq: Option<String>, // Id of the last equation.
    ends: HashMap<String, (String, String)>, // Tail and tip of each line or arrow, as expressions.
    home: HashMap<String, [f64; 3]>, // Where each named point starts.
    walked: Vec<String>,             // Points the last `walk` moved.
    tables: HashMap<String, Vec<Vec<f64>>>, // Entries of each numeric matrix, as the slides have left it.
    blocks: HashMap<String, Value>, // The chapter's own blocks: lists of step keys with `$name` holes.
}

impl Cx {
    /// A fresh automatic id.
    fn auto(&mut self) -> String {
        self.n += 1;
        format!("_auto{}", self.n)
    }
}

/// Three numbers.
fn triple(v: &Value) -> Option<[f64; 3]> {
    match v {
        Value::Sequence(s) if s.len() == 3 => Some([num(&s[0])?, num(&s[1])?, num(&s[2])?]),
        _ => None,
    }
}

/// Force densities, rim densities, lifted supports and loads written onto a net.
fn net_values(net: &mut Net, m: &Mapping, at: &str) -> Result<(), String> {
    let node = |net: &Net, k: &Value| {
        let name = text(k).unwrap_or_default();
        net.find(&name)
            .ok_or_else(|| format!("{at}: the net has no node {name}"))
    };
    match get(m, "q") {
        Some(Value::Mapping(q)) => {
            for (k, v) in q {
                let name = text(k).unwrap_or_default();
                let cable = net.cables.iter().position(|c| c.0 == name);
                let i = cable.ok_or_else(|| format!("{at}: the net has no cable {name}"))?;
                net.q[i] = num(v).unwrap_or(1.0);
            }
        }
        Some(v) => net.q = vec![num(v).unwrap_or(1.0); net.q.len()],
        None => {}
    }
    if let Some(rim) = get(m, "rim").and_then(num) {
        for k in 0..net.cables.len() {
            let (a, b) = (net.cables[k].1, net.cables[k].2);
            if net.degree(a) < 4 && net.degree(b) < 4 {
                net.q[k] = rim;
            }
        }
    }
    if let Some(Value::Mapping(lift)) = get(m, "lift") {
        for (k, v) in lift {
            let i = node(net, k)?;
            net.fixed[i] = true;
            net.at[i][2] = num(v).unwrap_or(0.0);
        }
    }
    let load = |v: &Value| triple(v).ok_or_else(|| format!("{at}: a load is [x, y, z]"));
    match get(m, "load") {
        Some(Value::Mapping(loads)) => {
            for (k, v) in loads {
                let i = node(net, k)?;
                net.load[i] = load(v)?;
            }
        }
        Some(v) => {
            for i in net.free() {
                net.load[i] = load(v)?;
            }
        }
        None => {}
    }
    Ok(())
}

/// The chapter's `net:`: named nodes and cables, or a grid.
fn net(v: &Value, at: &str) -> Result<Net, String> {
    let at = &format!("{at} net");
    let Value::Mapping(m) = v else {
        return err(at, "expected a map");
    };
    let mut net = Net::default();
    if let Some(size) = get(m, "grid").and_then(pair) {
        net = Net::grid(size[0] as usize, size[1] as usize);
        let fixed = get(m, "fixed").map(strings).unwrap_or_default();
        for i in 0..net.names.len() {
            net.fixed[i] = match fixed.first().map(String::as_str) {
                Some("corners") => net.degree(i) == 2,
                Some("boundary") => net.degree(i) < 4,
                _ => fixed.contains(&net.names[i]),
            };
        }
    } else {
        for (key, fixed) in [("free", false), ("fixed", true)] {
            let Some(Value::Mapping(nodes)) = get(m, key) else {
                continue;
            };
            for (k, v) in nodes {
                let name = text(k).unwrap_or_default();
                let p = triple(v).ok_or_else(|| format!("{at}: node {name} needs [x, y, z]"))?;
                net.node(&name, p, fixed);
            }
        }
        let Some(Value::Mapping(cables)) = get(m, "cables") else {
            return err(at, "needs `cables` or `grid`");
        };
        for (k, v) in cables {
            let name = text(k).unwrap_or_default();
            let ends: Vec<usize> = strings(v).iter().filter_map(|n| net.find(n)).collect();
            if ends.len() != 2 {
                return err(at, format!("cable {name} needs two node names"));
            }
            net.cable(&name, ends[0], ends[1], 1.0);
        }
    }
    net_values(&mut net, m, at)?;
    solvable(&net, at)?;
    Ok(net)
}

/// An error when a free node hangs on no cable, so the net has no equilibrium.
fn solvable(net: &Net, at: &str) -> Result<(), String> {
    let finite = |p: &[f64; 3]| p.iter().all(|c| c.is_finite());
    match net.solve().iter().position(|p| !finite(p)) {
        Some(i) => err(at, format!("node {} is not held by any cable with q above 0", net.names[i])),
        None => Ok(()),
    }
}

/// The objects that draw one part of the net: `fixed`, `free`, `cables`, `loads` or `net`.
fn net_objects(net: &Net, group: &str) -> Option<Vec<ObjSpec>> {
    let all = group == "net";
    if !all && !["fixed", "free", "cables", "loads"].contains(&group) {
        return None;
    }
    let small = net.names.len() <= 12;
    let mut out = Vec::new();
    if all || group == "cables" {
        for (name, a, b) in &net.cables {
            let line = Content::Line {
                a: Pos::Name(net.names[*a].clone()),
                b: Pos::Name(net.names[*b].clone()),
                arrow: false,
            };
            let mut o = plain(name.clone(), line);
            o.label = small.then(|| name.clone());
            o.width = (!small).then_some(1.6);
            out.push(o);
        }
    }
    for (key, fixed) in [("fixed", true), ("free", false)] {
        if !(all || group == key) {
            continue;
        }
        for (i, name) in net.names.iter().enumerate() {
            if net.fixed[i] != fixed || (!small && !fixed) {
                continue;
            }
            let mut o = plain(name.clone(), Content::Point(Pos::Name(name.clone())));
            o.color = Some(key.into());
            o.label = small.then(|| tex_name(name));
            out.push(o);
        }
    }
    if all || group == "loads" {
        let size = |p: &[f64; 3]| p.iter().map(|c| c * c).sum::<f64>().sqrt();
        let longest = net.load.iter().map(size).fold(1e-9, f64::max);
        let scale = if small { 1.0 } else { 0.5 } / longest;
        let free = net.free().into_iter();
        let loaded: Vec<usize> = free.filter(|&i| size(&net.load[i]) > 0.0).collect();
        for &i in &loaded {
            let (name, p) = (&net.names[i], net.load[i]);
            let tip = format!(
                "{name} + [{}, {}, {}]",
                p[0] * scale,
                p[1] * scale,
                p[2] * scale
            );
            let arrow = Content::Line {
                a: Pos::Name(name.clone()),
                b: Pos::Expr(tip),
                arrow: true,
            };
            let mut o = plain(format!("load_{name}"), arrow);
            o.color = Some("load".into());
            o.width = Some(if small { 3.6 } else { 1.8 });
            o.label = (loaded.len() == 1).then(|| "p".to_string());
            out.push(o);
        }
    }
    Some(out)
}

/// An action with default timing.
fn act(kind: Kind) -> Action {
    Action {
        kind,
        dur: None,
        with: false,
        delay: 0.0,
    }
}

/// An equation written by `eq` or `then`: named `eq<n>` and aligned on its equals sign.
fn equation(v: &Value, at: &str, cx: &mut Cx) -> Result<ObjSpec, String> {
    let mut o = match v {
        Value::Mapping(_) => object(v, at, cx)?,
        _ => plain(cx.auto(), Content::Tex(text(v).unwrap_or_default())),
    };
    if o.id.starts_with("_auto") {
        o.id = o.id.replace("_auto", "eq");
    }
    o.align = o.align.or(Some("=".into()));
    Ok(o)
}

/// The moves that bring every free node to rest, after changes to q, rim, lift or load.
fn solve(v: &Value, at: &str, cx: &Cx) -> Result<Kind, String> {
    let net = cx.net.clone();
    let mut net = net.ok_or_else(|| format!("{at}: the chapter has no net"))?;
    if let Value::Mapping(changes) = v {
        net_values(&mut net, changes, at)?;
        solvable(&net, at)?;
    }
    let rest = net.solve();
    let mut moves = Vec::new();
    for i in net.free() {
        let to = Pos::Abs(rest[i].map(Scalar::N));
        let mut a = act(Kind::Move {
            point: net.names[i].clone(),
            to,
        });
        a.dur = Some(2.0);
        moves.push(a);
    }
    Ok(Kind::Play(moves))
}

/// A position as the expression that names it.
fn written(p: &Pos) -> String {
    let scalar = |c: &Scalar| match c {
        Scalar::N(n) => n.to_string(),
        Scalar::E(e) => e.clone(),
    };
    match p {
        Pos::Name(name) => name.clone(),
        Pos::Expr(e) => format!("({e})"),
        Pos::Abs(c) => format!("[{}, {}, {}]", scalar(&c[0]), scalar(&c[1]), scalar(&c[2])),
    }
}

/// `project: {of: F1, parts: 'F_{1,%}', axes: [x, z]}`: the box an arrow spans, the walk along its
/// edges, and a label on each edge; `%` in `parts` becomes the axis.
fn project(v: &Value, at: &str, cx: &mut Cx) -> Result<Vec<Action>, String> {
    let Value::Mapping(m) = v else {
        return err(at, "project: {of: arrow}");
    };
    let of = get(m, "of").and_then(text).unwrap_or_default();
    let (tail, tip) = cx
        .ends
        .get(&of)
        .cloned()
        .ok_or_else(|| format!("{at}: no line or arrow {of} before it"))?;
    let axes = get(m, "axes").map(strings).unwrap_or_else(|| vec!["x".into(), "y".into(), "z".into()]);
    let parts = get(m, "parts").and_then(text);
    let expr = |e: String| Pos::Expr(e);
    let mut corner = [tail.clone(), tail.clone(), tail.clone()];
    let mut out = Vec::new();
    let mut cuboid = plain(format!("{of}.box"), Content::Cuboid { a: expr(tail), b: expr(tip.clone()) });
    cuboid.color = Some("dim".into());
    out.push(act(Kind::Add(vec![cuboid])));
    let mut labels = Vec::new();
    for axis in &axes {
        let k = ["x", "y", "z"].iter().position(|a| a == axis).ok_or_else(|| format!("{at}: no axis {axis}"))?;
        let from = format!("[x({}), y({}), z({})]", corner[0], corner[1], corner[2]);
        corner[k] = tip.clone();
        let to = format!("[x({}), y({}), z({})]", corner[0], corner[1], corner[2]);
        let edge = Content::Line {
            a: expr(from.clone()),
            b: expr(to.clone()),
            arrow: true,
        };
        let mut arrow = plain(format!("{of}.{axis}"), edge);
        arrow.color = Some(axis.clone());
        arrow.width = Some(3.6);
        out.push(act(Kind::Derive {
            from: of.clone(),
            obj: arrow,
            map: Vec::new(),
        }));
        if let Some(parts) = &parts {
            let label = Content::Label {
                at: expr(format!("mid({from}, {to})")),
                tex: parts.replace('%', axis),
            };
            let mut text = plain(format!("{of}.{axis}.label"), label);
            text.color = Some(axis.clone());
            text.size = Some(22.0);
            labels.push(text);
        }
    }
    if !labels.is_empty() {
        out.push(act(Kind::Add(labels)));
    }
    Ok(out)
}

/// `walk: [F1, F2, F3]` puts each arrow at the tip of the one before; `walk: back` returns them.
/// The arrows must run between named points.
fn walk(v: &Value, at: &str, cx: &mut Cx) -> Result<Vec<Action>, String> {
    let to = |point: &str, p: [f64; 3], with: bool| {
        let mut a = act(Kind::Move {
            point: point.into(),
            to: Pos::Abs(p.map(Scalar::N)),
        });
        a.with = with;
        a
    };
    let mut out = Vec::new();
    if text(v).as_deref() == Some("back") {
        for name in std::mem::take(&mut cx.walked) {
            if let Some(p) = cx.home.get(&name) {
                out.push(to(&name, *p, !out.is_empty()));
            }
        }
        return Ok(out);
    }
    let mut reached: Option<[f64; 3]> = None;
    for arrow in strings(v) {
        let (tail, tip) = cx.ends.get(&arrow).cloned().ok_or_else(|| format!("{at}: no arrow {arrow}"))?;
        let (Some(a), Some(b)) = (cx.home.get(&tail), cx.home.get(&tip)) else {
            return err(at, format!("{arrow} must run between named points"));
        };
        let start = reached.unwrap_or(*a);
        let end: [f64; 3] = std::array::from_fn(|i| start[i] + b[i] - a[i]);
        if reached.is_some() {
            out.push(to(&tail, start, false));
            out.push(to(&tip, end, true));
            cx.walked.extend([tail, tip]);
        }
        reached = Some(end);
    }
    Ok(out)
}

/// `node: {of: n4_4, row: DN}` lights a node of the net, its cables and its row in a matrix;
/// `clear: true` puts the colours back.
fn node(v: &Value, at: &str, cx: &mut Cx) -> Result<Vec<Action>, String> {
    let (name, row, clear) = match v {
        Value::Mapping(m) => (
            get(m, "of").and_then(text).unwrap_or_default(),
            get(m, "row").and_then(text),
            get(m, "clear").and_then(Value::as_bool).unwrap_or(false),
        ),
        other => (text(other).unwrap_or_default(), None, false),
    };
    let net = cx.net.as_ref().ok_or_else(|| format!("{at}: the chapter has no net"))?;
    let i = net.find(&name).ok_or_else(|| format!("{at}: the net has no node {name}"))?;
    let color = if clear { "reset" } else { "accent" };
    let paint = |target: String| {
        let mut a = act(Kind::Color {
            target,
            color: color.into(),
        });
        a.with = true;
        a
    };
    let mut out = Vec::new();
    let mark = format!("{name}.mark");
    if clear {
        out.push(act(Kind::Remove(vec![mark])));
    } else {
        let mut dot = plain(mark, Content::Point(Pos::Name(name.clone())));
        dot.color = Some("accent".into());
        out.push(act(Kind::Add(vec![dot])));
    }
    for (cable, a, b) in &net.cables {
        if *a == i || *b == i {
            out.push(paint(cable.clone()));
        }
    }
    if let Some(matrix) = row
        && let Some(k) = net.free().iter().position(|&f| f == i)
    {
        out.push(paint(format!("{matrix}[{k},:]")));
    }
    Ok(out)
}

/// `eliminate: {matrix: M, row: 2, with: 1}` adds the multiple of row `with` to `row` that clears
/// the entry under the pivot; rows count from 1 and the changed entries are computed.
fn eliminate(v: &Value, at: &str, cx: &mut Cx) -> Result<Vec<Action>, String> {
    let Value::Mapping(m) = v else {
        return err(at, "eliminate: {matrix: M, row: 2, with: 1}");
    };
    let name = get(m, "matrix").and_then(text).unwrap_or_default();
    let index = |key: &str| get(m, key).and_then(num).map(|n| n as usize).filter(|n| *n > 0);
    let (Some(row), Some(pivot)) = (index("row"), index("with")) else {
        return err(at, "needs `row` and `with`, counted from 1");
    };
    let (row, pivot) = (row - 1, pivot - 1);
    let table = cx.tables.get_mut(&name).ok_or_else(|| format!("{at}: no numeric matrix {name}"))?;
    if row >= table.len() || pivot >= table.len() || table[pivot][pivot] == 0.0 {
        return err(at, "has no such rows, or a zero pivot");
    }
    let factor = table[row][pivot] / table[pivot][pivot];
    let mut out = vec![act(Kind::Color {
        target: format!("{name}[{pivot},:]"),
        color: "accent".into(),
    })];
    let mut first = true;
    for col in 0..table[row].len() {
        let value = table[row][col] - factor * table[pivot][col];
        if (value - table[row][col]).abs() < 1e-12 {
            continue;
        }
        table[row][col] = value;
        let mut set = act(Kind::Set {
            target: format!("{name}[{row},{col}]"),
            tex: num_text(value),
        });
        set.with = !first;
        first = false;
        out.push(set);
    }
    out.push(act(Kind::Color {
        target: format!("{name}[{pivot},:]"),
        color: "reset".into(),
    }));
    Ok(out)
}

/// `multiply: {left: A, right: B, into: C, entry: [0, 1]}` tints a row of `A` and a column of `B`
/// and reveals their product in `C`, which was added with `empty: true`.
fn multiply(v: &Value, at: &str) -> Result<Vec<Action>, String> {
    let Value::Mapping(m) = v else {
        return err(at, "multiply: {left: A, right: B, into: C, entry: [0, 0]}");
    };
    let name = |key: &str| get(m, key).and_then(text).ok_or_else(|| format!("{at}: needs `{key}`"));
    let (left, right, into) = (name("left")?, name("right")?, name("into")?);
    let [r, c] = get(m, "entry").and_then(pair).ok_or_else(|| format!("{at}: needs `entry: [row, column]`"))?;
    let (row, column) = (format!("{left}[{r},:]"), format!("{right}[:,{c}]"));
    let paint = |target: &String, color: &str, with: bool| {
        let mut a = act(Kind::Color {
            target: target.clone(),
            color: color.into(),
        });
        a.with = with;
        a
    };
    Ok(vec![
        paint(&row, "accent", false),
        paint(&column, "accent", true),
        act(Kind::Show(vec![format!("{into}[{r},{c}]")])),
        paint(&row, "reset", false),
        paint(&column, "reset", true),
    ])
}

/// A block template with every `$name` replaced by what the step passed under `name`.
fn filled(template: &Value, given: &Value) -> Value {
    let lookup = |name: &str| match given {
        Value::Mapping(m) => get(m, name).cloned(),
        _ => None,
    };
    match template {
        Value::String(s) => {
            if let Some(whole) = s.strip_prefix('$').and_then(lookup) {
                return whole;
            }
            let mut out = s.clone();
            if let Value::Mapping(m) = given {
                for (k, v) in m {
                    if let (Some(k), Some(v)) = (text(k), text(v)) {
                        out = out.replace(&format!("${k}"), &v);
                    }
                }
            }
            Value::String(out)
        }
        Value::Sequence(items) => Value::Sequence(items.iter().map(|item| filled(item, given)).collect()),
        Value::Mapping(m) => {
            let mut out = Mapping::new();
            for (k, v) in m {
                out.insert(k.clone(), filled(v, given));
            }
            Value::Mapping(out)
        }
        other => other.clone(),
    }
}

/// Step keys that stand for a common action.
const SHORT: [&str; 17] = [
    "eliminate",
    "multiply",
    "project",
    "walk",
    "node",
    "eq",
    "then",
    "becomes",
    "text",
    "code",
    "image",
    "matrix",
    "row",
    "tex",
    "highlight",
    "show",
    "solve",
];

/// True for a key that styles the object written beside it: `id`, `align`, `size`, `gap`, `color: name`.
fn looks(key: &Value, value: &Value) -> bool {
    match key.as_str() {
        Some("id" | "align" | "size" | "gap") => true,
        Some("color") => !value.is_mapping(),
        _ => false,
    }
}

/// The actions of one step, in the order its keys are written.
fn step(m: &Mapping, at: &str, cx: &mut Cx) -> Result<Vec<Action>, String> {
    let mut out = Vec::new();
    for (k, v) in m {
        let key = text(k).unwrap_or_default();
        let at = &format!("{at} {key}");
        let mut one = Mapping::new();
        one.insert(k.clone(), v.clone());
        for (name, value) in m {
            if looks(name, value) {
                one.insert(name.clone(), value.clone());
            }
        }
        let one = Value::Mapping(one);
        match key.as_str() {
            "say" | "with" | "dur" | "delay" => {}
            _ if looks(k, v) => {}
            "do" => {
                for (i, a) in list(v).iter().enumerate() {
                    let at = &format!("{at} {}", i + 1);
                    match a {
                        Value::Mapping(am) if SHORT.iter().any(|k| get(am, k).is_some()) => {
                            out.extend(step(am, at, cx)?)
                        }
                        _ => out.push(action(a, at, cx)?),
                    }
                }
            }
            "eq" => {
                let o = equation(v, at, cx)?;
                cx.eq = Some(o.id.clone());
                out.push(act(Kind::Add(vec![o])));
            }
            "then" => {
                let obj = equation(v, at, cx)?;
                let from = cx.eq.replace(obj.id.clone());
                let from = from.ok_or_else(|| format!("{at}: no equation before it"))?;
                out.push(act(Kind::Derive {
                    from,
                    obj,
                    map: Vec::new(),
                }));
            }
            "becomes" => {
                let last = cx.eq.clone();
                let target = last.ok_or_else(|| format!("{at}: no equation before it"))?;
                out.push(act(Kind::Morph {
                    target,
                    to: Content::Tex(text(v).unwrap_or_default()),
                    map: Vec::new(),
                }));
            }
            "text" | "code" | "image" | "matrix" | "row" | "tex" => {
                out.push(act(Kind::Add(vec![object(&one, at, cx)?])))
            }
            "highlight" => out.push(act(Kind::Indicate(strings(v), None))),
            "show" => {
                let (mut objs, mut refs) = (Vec::new(), Vec::new());
                for name in strings(v) {
                    match cx.net.as_ref().and_then(|net| net_objects(net, &name)) {
                        Some(o) => objs.extend(o),
                        None => refs.push(name),
                    }
                }
                if !objs.is_empty() {
                    out.push(act(Kind::Add(objs)));
                }
                if !refs.is_empty() {
                    out.push(act(Kind::Show(refs)));
                }
            }
            "solve" => out.push(act(solve(v, at, cx)?)),
            "project" => out.extend(project(v, at, cx)?),
            "walk" => out.extend(walk(v, at, cx)?),
            "eliminate" => out.extend(eliminate(v, at, cx)?),
            "multiply" => out.extend(multiply(v, at)?),
            "node" => out.extend(node(v, at, cx)?),
            _ if KINDS.contains(&key.as_str()) => out.push(action(&one, at, cx)?),
            _ if cx.blocks.contains_key(&key) => {
                let template = filled(&cx.blocks[&key], v);
                for (i, item) in list(&template).iter().enumerate() {
                    let at = &format!("{at} {}", i + 1);
                    match item {
                        Value::Mapping(im) => out.extend(step(im, at, cx)?),
                        _ => return err(at, "a block is a list of step keys"),
                    }
                }
            }
            _ => return err(at, "is not a step key"),
        }
    }
    if let Some(first) = out.first_mut() {
        first.with = get(m, "with")
            .and_then(Value::as_bool)
            .unwrap_or(first.with);
        first.delay = get(m, "delay").and_then(num).unwrap_or(first.delay);
    }
    if let Some(dur) = get(m, "dur").and_then(num) {
        for a in &mut out {
            a.dur = Some(dur);
        }
    }
    Ok(out)
}

/// The picture files a chapter names with `image:`, in the order met.
pub fn pictures(src: &str) -> Vec<String> {
    fn walk(v: &Value, out: &mut Vec<String>) {
        match v {
            Value::Mapping(m) => {
                for (k, v) in m {
                    let file = match v {
                        Value::Mapping(im) => get(im, "src").and_then(text),
                        other => text(other),
                    };
                    match (k.as_str(), file) {
                        (Some("image"), Some(file)) if !out.contains(&file) => out.push(file),
                        _ => walk(v, out),
                    }
                }
            }
            Value::Sequence(items) => items.iter().for_each(|item| walk(item, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    if let Ok(v) = serde_yaml_ng::from_str::<Value>(src) {
        walk(&v, &mut out);
    }
    out
}
