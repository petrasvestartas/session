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

#[derive(Clone, Debug)]
pub struct Grid {
    pub center: [f64; 2],
    pub size: f64,
    pub step: f64,
    pub z: f64,
}

#[derive(Clone, Debug, Default)]
pub struct CameraSpec {
    pub yaw: Option<f64>,
    pub pitch: Option<f64>,
    pub zoom: Option<f64>,
    pub center: Option<Pos>,
    pub fit: Vec<Pos>,
    pub view: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Slide {
    pub title: String,
    pub keep: bool,
    pub steps: Vec<Step>,
}

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
    Matrix(MatrixSpec),
    Row(Vec<RowItem>),
}

#[derive(Clone, Debug)]
pub struct RowItem {
    pub id: Option<String>,
    pub content: Content,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct MatrixSpec {
    pub rows: Vec<Vec<String>>,
    pub name: Option<String>,
    pub rowlabels: Vec<String>,
    pub collabels: Vec<String>,
    pub empty: bool,
    pub signs: bool,
    pub cell: Option<f64>,
    pub colors: Vec<Vec<Option<String>>>,
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

#[derive(Clone, Debug)]
pub enum Scalar {
    N(f64),
    E(String),
}

fn err<T>(at: &str, msg: impl std::fmt::Display) -> Result<T, String> {
    Err(format!("{at}: {msg}"))
}

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

fn num(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => n.as_f64(),
        Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

fn get<'a>(m: &'a Mapping, key: &str) -> Option<&'a Value> {
    m.get(Value::String(key.into()))
}

fn list(v: &Value) -> Vec<Value> {
    match v {
        Value::Sequence(s) => s.clone(),
        Value::Null => Vec::new(),
        other => vec![other.clone()],
    }
}

fn strings(v: &Value) -> Vec<String> {
    list(v).iter().filter_map(text).collect()
}

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

fn pair(v: &Value) -> Option<[f64; 2]> {
    match v {
        Value::Sequence(s) if s.len() == 2 => Some([num(&s[0])?, num(&s[1])?]),
        _ => None,
    }
}

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

fn matrix(v: &Value, at: &str) -> Result<MatrixSpec, String> {
    let mut spec = MatrixSpec {
        signs: true,
        ..MatrixSpec::default()
    };
    let rows_value = match v {
        Value::Mapping(m) => {
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

const TYPES: [&str; 11] = [
    "point", "line", "arrow", "polyline", "polygon", "cuboid", "label", "tex", "text", "matrix",
    "row",
];

fn content(m: &Mapping, at: &str) -> Result<Content, String> {
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
            "tex" => {
                if get(m, "label").is_some() && get(m, "tex").is_some() && false {
                    unreachable!()
                }
                Content::Tex(text(v).unwrap_or_default())
            }
            "text" => Content::Text(text(v).unwrap_or_default()),
            "matrix" => Content::Matrix(matrix(v, at)?),
            "row" => {
                let mut items = Vec::new();
                for (i, item) in list(v).iter().enumerate() {
                    let at = &format!("{at}[{i}]");
                    items.push(match item {
                        Value::Mapping(im) => RowItem {
                            id: get(im, "id").and_then(text),
                            content: content(im, at)?,
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

fn object(v: &Value, at: &str, n: &mut usize) -> Result<ObjSpec, String> {
    let Value::Mapping(m) = v else {
        if let Some(t) = text(v) {
            *n += 1;
            return Ok(plain(format!("_auto{n}"), Content::Tex(t)));
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
        content(m, at)?
    };
    let id = match get(m, "id").and_then(text) {
        Some(id) => id,
        None => {
            *n += 1;
            format!("_auto{n}")
        }
    };
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

fn plain(id: String, content: Content) -> ObjSpec {
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

fn mapping(v: Option<&Value>) -> Vec<(String, String)> {
    match v {
        Some(Value::Mapping(m)) => m
            .iter()
            .filter_map(|(k, v)| Some((text(k)?, text(v)?)))
            .collect(),
        _ => Vec::new(),
    }
}

const KINDS: [&str; 19] = [
    "add", "remove", "morph", "derive", "fly", "indicate", "box", "color", "move", "camera",
    "wait", "set", "dim", "undim", "clear", "show", "hide", "play", "write",
];

fn action(v: &Value, at: &str, n: &mut usize) -> Result<Action, String> {
    let Value::Mapping(m) = v else {
        return match text(v).as_deref() {
            Some("clear") => Ok(Action {
                kind: Kind::Clear,
                dur: None,
                with: false,
                delay: 0.0,
            }),
            Some("wait") => Ok(Action {
                kind: Kind::Wait,
                dur: Some(1.0),
                with: false,
                delay: 0.0,
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
                objs.push(object(&o, at, n)?);
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
                to: content(mm, at)?,
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
                obj: object(v, at, n)?,
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
                let mut a = action(a, &format!("{at}[{i}]"), n)?;
                a.with = i > 0;
                acts.push(a);
            }
            Kind::Play(acts)
        }
        _ => unreachable!(),
    };
    // timing may sit beside the action key or inside its map
    let inner = |k: &str| get(m, k).or(match v {
        Value::Mapping(vm) => get(vm, k),
        _ => None,
    });
    let dur = inner("dur").and_then(num).or(if *key == "wait" { num(v).or(Some(1.0)) } else { None });
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
        ("neg", "#5b9cf6"),
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
        title: get(m, "title").and_then(text).unwrap_or_default(),
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
    let mut n = 0;
    let mut slides = Vec::new();
    for (si, s) in get(m, "slides")
        .map(list)
        .unwrap_or_default()
        .iter()
        .enumerate()
    {
        let Value::Mapping(sm) = s else {
            return err(name, format!("slide {} is not a map", si + 1));
        };
        let title = get(sm, "title").and_then(text).unwrap_or_default();
        let mut steps = Vec::new();
        for (ti, t) in get(sm, "steps")
            .map(list)
            .unwrap_or_default()
            .iter()
            .enumerate()
        {
            let at = format!("{name} slide {} step {}", si + 1, ti + 1);
            let Value::Mapping(tm) = t else {
                return err(&at, "a step is a map with `say` and `do`");
            };
            let mut actions = Vec::new();
            for (ai, a) in get(tm, "do")
                .map(list)
                .unwrap_or_default()
                .iter()
                .enumerate()
            {
                actions.push(action(a, &format!("{at} action {}", ai + 1), &mut n)?);
            }
            steps.push(Step {
                say: get(tm, "say").and_then(text),
                actions,
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
        title: get(m, "title").and_then(text).unwrap_or_default(),
        palette,
        points,
        camera: match get(m, "camera") {
            Some(c) => camera(c, name)?,
            None => CameraSpec::default(),
        },
        split: get(m, "split").and_then(num).unwrap_or(0.46) as f32,
        grid,
        axes: get(m, "axes").and_then(Value::as_bool).unwrap_or(false),
        slides,
    })
}
