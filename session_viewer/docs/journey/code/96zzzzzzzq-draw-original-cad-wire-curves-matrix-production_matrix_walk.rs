use crate::production_matrix::Walk as Row;
use crate::production_ribbon::pack_rgba;
use crate::production_matrix::{
    CELL_AT, COLS_AT, HEADER_WORDS, HIGH_AT, INSTANCE_AT, LINE_AT, LOW_AT, MAX_AT, MIN_AT, MatrixRows, ROWS_AT, SIGN_BIT,
    TEXT_AT, ZERO_AT,
};
use session_rust::AABB;
use session_rust::objects::Component;
use serde_json::Value;

/// The component type a session stores a kernel Matrix as.
pub const TYPE_NAME: &str = "Matrix";

/// Title height over cell size.
pub const TITLE_EM: f64 = 0.35;

/// Defaults of a field the component leaves out: the kernel Matrix's BRG colours, a unit cell.
const MINCOLOR: [f32; 4] = [11.0 / 255.0, 25.0 / 255.0, 87.0 / 255.0, 1.0]; // BRG navy
const MAXCOLOR: [f32; 4] = [206.0 / 255.0, 64.0 / 255.0, 149.0 / 255.0, 1.0]; // BRG pink
const ZEROCOLOR: [f32; 4] = [185.0 / 255.0, 185.0 / 255.0, 189.0 / 255.0, 1.0]; // BRG grey
const TEXTCOLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const LINECOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// True when a session stores at least one kernel Matrix: drawable though it has no geometry.
pub fn holds_matrix(session: &session_rust::Session) -> bool {
    session.objects.components.iter().any(is_matrix)
}

/// True for a component that holds a kernel Matrix.
pub fn is_matrix(component: &Component) -> bool {
    component.type_name == TYPE_NAME
}

/// A matrix component as one block of words for object row `row`: the shader draws its cells, edges and values.
pub fn walk(rows_out: &mut MatrixRows, component: &Component, row: u32) -> Row {
    let rows = count(component, "rows");
    let cols = count(component, "cols");
    let cell = component.extra.get("cell").and_then(Value::as_f64).unwrap_or(1.0) as f32;
    let data: Vec<f64> = component
        .extra
        .get("data")
        .and_then(Value::as_array)
        .map(|values| values.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect())
        .unwrap_or_default();

    if rows == 0 || cols == 0 || data.len() != rows as usize * cols as usize {
        return Row::thin(AABB::empty());
    }

    // the range the colours span: the non-zero entries
    let mut low = f64::INFINITY;
    let mut high = f64::NEG_INFINITY;

    for &value in &data {
        if value.abs() >= 1e-12 {
            low = low.min(value);
            high = high.max(value);
        }
    }

    let colors = [
        color(component, "mincolor", MINCOLOR),
        color(component, "maxcolor", MAXCOLOR),
        color(component, "zerocolor", ZEROCOLOR),
        color(component, "textcolor", TEXTCOLOR),
    ];
    let linecolor = color(component, "linecolor", LINECOLOR);

    // one block: the header the shader reads, then the values row by row
    let mut header = [0u32; HEADER_WORDS];
    header[ROWS_AT] = rows;
    header[COLS_AT] = cols;
    header[INSTANCE_AT] = row;
    header[CELL_AT] = cell.to_bits();
    header[LOW_AT] = (low as f32).to_bits();
    header[HIGH_AT] = (high as f32).to_bits();
    header[LINE_AT] = linecolor;
    header[MIN_AT] = colors[0];
    header[MAX_AT] = colors[1];
    header[ZERO_AT] = colors[2];
    header[TEXT_AT] = colors[3];
    rows_out.words.extend(header);

    // every value: its f32 for the colour, then exactly for the shader to write: whole part, thousandths and sign
    for &value in &data {
        let thousandths = (value.abs() * 1000.0).round().min(4.0e12) as u64;
        let sign = if value < 0.0 { SIGN_BIT } else { 0 };
        rows_out.words.push((value as f32).to_bits());
        rows_out.words.push((thousandths / 1000).min(u64::from(u32::MAX)) as u32);
        rows_out.words.push((thousandths % 1000) as u32 | sign);
    }

    // the cells, and the title above them
    let half = f64::from(cell) * 0.5;
    let mut bounds = AABB::empty();
    bounds.union_with_point(-half, half + f64::from(cell) * (TITLE_EM + 0.5), 0.0);
    bounds.union_with_point(f64::from(cols) * f64::from(cell) - half, half - f64::from(rows) * f64::from(cell), 0.0);
    Row::thin(bounds)
}

/// The title of a matrix component: its name, its object-space top-left corner, its height and colour.
pub fn title(component: &Component) -> Option<(String, [f64; 3], f64, [u8; 4])> {
    if component.name.trim().is_empty() {
        return None;
    }

    let cell = component.extra.get("cell").and_then(Value::as_f64).unwrap_or(1.0);
    let height = TITLE_EM * cell;
    let corner = [-0.5 * cell, 0.5 * cell + height * 1.6, 0.0];
    let [r, g, b, a] = color_f32(component, "linecolor", LINECOLOR);
    let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Some((component.name.clone(), corner, height, [byte(r), byte(g), byte(b), byte(a)]))
}

/// A non-negative integer field.
fn count(component: &Component, key: &str) -> u32 {
    component.extra.get(key).and_then(Value::as_u64).unwrap_or(0) as u32
}

/// A colour field, packed rgba.
fn color(component: &Component, key: &str, default: [f32; 4]) -> u32 {
    pack_rgba(color_f32(component, key, default))
}

/// A colour field as the kernel Color writes it to JSON: r, g, b and a from 0 to 1.
fn color_f32(component: &Component, key: &str, default: [f32; 4]) -> [f32; 4] {
    let Some(object) = component.extra.get(key).and_then(Value::as_object) else {
        return default;
    };
    let channel = |name: &str, fallback: f32| object.get(name).and_then(Value::as_f64).map_or(fallback, |v| v as f32);
    [
        channel("r", default[0]),
        channel("g", default[1]),
        channel("b", default[2]),
        channel("a", default[3]),
    ]
}
