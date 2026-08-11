//! Stanford PLY -> Mesh / PointCloud.
//!
//! Handles `ascii`, `binary_little_endian` and `binary_big_endian`. A file with `element face`
//! rows becomes a Mesh; one with vertices only becomes a PointCloud. Per-vertex `red/green/blue`
//! (uchar 0-255 or float 0-1) is carried across; every other property is parsed for its width and
//! skipped, so unknown columns (normals, intensity, confidence) shift the layout correctly instead
//! of corrupting it.
use session_rust::{Color, Mesh, Point, PointCloud};
use std::io;

#[derive(Clone, Copy, PartialEq)]
enum Scalar { I8, U8, I16, U16, I32, U32, F32, F64 }

impl Scalar {
    fn parse(s: &str) -> Option<Scalar> {
        Some(match s {
            "char" | "int8" => Scalar::I8,
            "uchar" | "uint8" => Scalar::U8,
            "short" | "int16" => Scalar::I16,
            "ushort" | "uint16" => Scalar::U16,
            "int" | "int32" => Scalar::I32,
            "uint" | "uint32" => Scalar::U32,
            "float" | "float32" => Scalar::F32,
            "double" | "float64" => Scalar::F64,
            _ => return None,
        })
    }
    fn size(self) -> usize {
        match self {
            Scalar::I8 | Scalar::U8 => 1,
            Scalar::I16 | Scalar::U16 => 2,
            Scalar::I32 | Scalar::U32 | Scalar::F32 => 4,
            Scalar::F64 => 8,
        }
    }
    /// `is_int` distinguishes a 0-255 colour channel from a 0-1 float one.
    fn read(self, b: &[u8], le: bool) -> (f64, bool) {
        macro_rules! n {
            ($t:ty, $sz:expr) => {{
                let mut a = [0u8; $sz];
                a.copy_from_slice(&b[..$sz]);
                if le { <$t>::from_le_bytes(a) as f64 } else { <$t>::from_be_bytes(a) as f64 }
            }};
        }
        match self {
            Scalar::I8 => (b[0] as i8 as f64, true),
            Scalar::U8 => (b[0] as f64, true),
            Scalar::I16 => (n!(i16, 2), true),
            Scalar::U16 => (n!(u16, 2), true),
            Scalar::I32 => (n!(i32, 4), true),
            Scalar::U32 => (n!(u32, 4), true),
            Scalar::F32 => (n!(f32, 4), false),
            Scalar::F64 => (n!(f64, 8), false),
        }
    }
}

/// A `property`: either a fixed scalar, or a `list <count> <item>` (faces).
struct Prop {
    name: String,
    scalar: Scalar,
    list_count: Option<Scalar>,
}

struct Element {
    name: String,
    count: usize,
    props: Vec<Prop>,
}

enum Format { Ascii, BinaryLe, BinaryBe }

/// PLY holds vertices and faces in one file; which one the caller wants decides the return type.
pub struct Ply {
    pub points: Vec<Point>,
    pub colors: Vec<Color>,
    pub faces: Vec<Vec<usize>>,
}

impl Ply {
    pub fn into_mesh(self) -> Mesh {
        let mut mesh = Mesh::new();
        let vkeys: Vec<usize> = self.points.into_iter().map(|p| mesh.add_vertex(p, None)).collect();
        for f in self.faces {
            if f.len() >= 3 && f.iter().all(|i| *i < vkeys.len()) {
                let _ = mesh.add_face(f.into_iter().map(|i| vkeys[i]).collect(), None);
            }
        }
        mesh
    }

    pub fn into_pointcloud(self) -> PointCloud {
        let mut cloud = PointCloud::default();
        let colored = self.colors.len() == self.points.len();
        for (i, p) in self.points.iter().enumerate() {
            cloud.add_point(p);
            if colored {
                cloud.add_color(&self.colors[i]);
            }
        }
        cloud
    }
}

pub fn read_ply(filepath: &str) -> io::Result<Ply> {
    read_ply_from_bytes(&std::fs::read(filepath)?)
}

pub fn read_ply_from_bytes(data: &[u8]) -> io::Result<Ply> {
    let err = |m: &str| io::Error::new(io::ErrorKind::InvalidData, m.to_string());

    // The header is ascii even in a binary file, and ends at the line "end_header".
    let head_end = find(data, b"end_header")
        .ok_or_else(|| err("ply: no end_header"))?;
    let header = std::str::from_utf8(&data[..head_end])
        .map_err(|_| err("ply: non-utf8 header"))?;
    // Skip past "end_header" and its line ending (\n or \r\n).
    let mut pos = head_end + b"end_header".len();
    while pos < data.len() && (data[pos] == b'\r' || data[pos] == b'\n') {
        pos += 1;
        if data[pos - 1] == b'\n' { break; }
    }

    let mut format = None;
    let mut elements: Vec<Element> = Vec::new();
    for line in header.lines() {
        let t: Vec<&str> = line.split_whitespace().collect();
        match t.as_slice() {
            ["format", f, ..] => {
                format = Some(match *f {
                    "ascii" => Format::Ascii,
                    "binary_little_endian" => Format::BinaryLe,
                    "binary_big_endian" => Format::BinaryBe,
                    _ => return Err(err("ply: unknown format")),
                })
            }
            ["element", name, count] => elements.push(Element {
                name: name.to_string(),
                count: count.parse().unwrap_or(0),
                props: Vec::new(),
            }),
            ["property", "list", cnt, item, name] => {
                if let Some(e) = elements.last_mut() {
                    e.props.push(Prop {
                        name: name.to_string(),
                        scalar: Scalar::parse(item).ok_or_else(|| err("ply: bad list type"))?,
                        list_count: Some(Scalar::parse(cnt).ok_or_else(|| err("ply: bad count type"))?),
                    });
                }
            }
            ["property", ty, name] => {
                if let Some(e) = elements.last_mut() {
                    e.props.push(Prop {
                        name: name.to_string(),
                        scalar: Scalar::parse(ty).ok_or_else(|| err("ply: bad property type"))?,
                        list_count: None,
                    });
                }
            }
            _ => {}
        }
    }
    let format = format.ok_or_else(|| err("ply: no format line"))?;

    let mut out = Ply { points: Vec::new(), colors: Vec::new(), faces: Vec::new() };
    match format {
        Format::Ascii => {
            let body = std::str::from_utf8(&data[pos..]).map_err(|_| err("ply: non-utf8 body"))?;
            let mut rows = body.lines().filter(|l| !l.trim().is_empty());
            for e in &elements {
                for _ in 0..e.count {
                    let row = match rows.next() { Some(r) => r, None => break };
                    let f: Vec<&str> = row.split_whitespace().collect();
                    read_ascii_row(e, &f, &mut out);
                }
            }
        }
        Format::BinaryLe | Format::BinaryBe => {
            let le = matches!(format, Format::BinaryLe);
            for e in &elements {
                for _ in 0..e.count {
                    if pos >= data.len() { break; }
                    read_binary_row(e, data, &mut pos, le, &mut out);
                }
            }
        }
    }
    Ok(out)
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

/// `red`/`green`/`blue` may be 0-255 ints or 0-1 floats; scale only when a channel exceeds 1.
fn push_color(out: &mut Ply, rgba: [f64; 4], seen: bool) {
    if !seen { return; }
    let s = if rgba[0] > 1.0 || rgba[1] > 1.0 || rgba[2] > 1.0 { 1.0 / 255.0 } else { 1.0 };
    let a = if rgba[3] > 1.0 { rgba[3] / 255.0 } else if rgba[3] > 0.0 { rgba[3] } else { 1.0 };
    out.colors.push(Color::new(
        (rgba[0] * s) as f32,
        (rgba[1] * s) as f32,
        (rgba[2] * s) as f32,
        a as f32,
    ));
}

fn read_ascii_row(e: &Element, f: &[&str], out: &mut Ply) {
    if e.name == "vertex" {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        let mut rgba = [0.0f64, 0.0, 0.0, 0.0];
        let mut seen_color = false;
        for (i, p) in e.props.iter().enumerate() {
            let v: f64 = f.get(i).and_then(|s| s.parse().ok()).unwrap_or(0.0);
            match p.name.as_str() {
                "x" => x = v,
                "y" => y = v,
                "z" => z = v,
                "red" | "r" => { rgba[0] = v; seen_color = true }
                "green" | "g" => { rgba[1] = v; seen_color = true }
                "blue" | "b" => { rgba[2] = v; seen_color = true }
                "alpha" | "a" => rgba[3] = v,
                _ => {}
            }
        }
        out.points.push(Point::new(x, y, z));
        push_color(out, rgba, seen_color);
    } else if e.name == "face" {
        // The vertex-index list is the first list property on the element.
        if let Some(pi) = e.props.iter().position(|p| p.list_count.is_some()) {
            // Fixed scalars before the list each take one column.
            let start = pi;
            let n: usize = f.get(start).and_then(|s| s.parse().ok()).unwrap_or(0);
            let mut face = Vec::with_capacity(n);
            for k in 0..n {
                if let Some(v) = f.get(start + 1 + k).and_then(|s| s.parse::<i64>().ok()) {
                    if v >= 0 { face.push(v as usize) }
                }
            }
            if face.len() >= 3 { out.faces.push(face) }
        }
    }
}

fn read_binary_row(e: &Element, data: &[u8], pos: &mut usize, le: bool, out: &mut Ply) {
    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
    let mut rgba = [0.0f64, 0.0, 0.0, 0.0];
    let mut seen_color = false;
    let mut face: Vec<usize> = Vec::new();

    for p in &e.props {
        if let Some(cnt) = p.list_count {
            if *pos + cnt.size() > data.len() { return }
            let (n, _) = cnt.read(&data[*pos..], le);
            *pos += cnt.size();
            let n = n.max(0.0) as usize;
            for _ in 0..n {
                if *pos + p.scalar.size() > data.len() { return }
                let (v, _) = p.scalar.read(&data[*pos..], le);
                *pos += p.scalar.size();
                if v >= 0.0 { face.push(v as usize) }
            }
        } else {
            if *pos + p.scalar.size() > data.len() { return }
            let (v, _) = p.scalar.read(&data[*pos..], le);
            *pos += p.scalar.size();
            match p.name.as_str() {
                "x" => x = v,
                "y" => y = v,
                "z" => z = v,
                "red" | "r" => { rgba[0] = v; seen_color = true }
                "green" | "g" => { rgba[1] = v; seen_color = true }
                "blue" | "b" => { rgba[2] = v; seen_color = true }
                "alpha" | "a" => rgba[3] = v,
                _ => {}
            }
        }
    }

    if e.name == "vertex" {
        out.points.push(Point::new(x, y, z));
        push_color(out, rgba, seen_color);
    } else if e.name == "face" && face.len() >= 3 {
        out.faces.push(face);
    }
}
