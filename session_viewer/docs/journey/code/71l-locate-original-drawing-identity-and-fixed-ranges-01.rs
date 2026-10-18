use crate::stream::{SheetFields, field_at};
pub struct Source {
    pub guid: String,
    pub name: String,
    pub url: String,
    pub fields: SheetFields,
}
pub fn identity(head: &[u8]) -> Option<(String, String)> {
    let mut at = 0u64;
    let objects = loop {
        let f = field_at(head.get(at as usize..)?, at, u64::MAX)?;
        if f.field == 3 && f.wire == 2 {
            break f;
        }
        if f.field > 2 || f.wire != 2 {
            return None;
        }
        at = f.next;
    };
    at = objects.body;
    let sheet = loop {
        let f = field_at(head.get(at as usize..)?, at, objects.next)?;
        if f.field == 17 && f.wire == 2 && f.next == objects.next {
            break f;
        }
        if f.field > 2 || f.wire != 2 {
            return None;
        }
        at = f.next;
    };
    at = sheet.body;
    let mut guid = None;
    let mut name = String::new();
    loop {
        let f = field_at(head.get(at as usize..)?, at, sheet.next)?;
        if f.field == 3 && f.wire == 2 {
            return Some((guid?, name));
        }
        if f.wire != 2 || f.field > 2 || f.value > 256 {
            return None;
        }
        let text = std::str::from_utf8(head.get(f.body as usize..f.next as usize)?)
            .ok()?
            .to_owned();
        match f.field {
            1 if guid.is_none() && !text.is_empty() => guid = Some(text),
            2 => name = text,
            _ => return None,
        }
        at = f.next;
    }
}
pub fn slice(
    fields: &SheetFields,
    at: u64,
    length: u64,
    rows: std::ops::Range<u32>,
    stride: u64,
) -> Option<(u64, u64)> {
    if rows.start > rows.end || rows.end > fields.count {
        return None;
    }
    if length == 0 {
        return Some((0, 0));
    }
    let start = at.checked_add(u64::from(rows.start).checked_mul(stride)?)?;
    let bytes = u64::from(rows.end - rows.start).checked_mul(stride)?;
    let end = at.checked_add(length)?;
    if start.checked_add(bytes)? > end || end > fields.end {
        return None;
    }
    Some((start, bytes))
}
