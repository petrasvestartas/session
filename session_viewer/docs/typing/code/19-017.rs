
use crate::app::sheet_query::{EntityMeta, SheetTable};

use crate::app::walk::sheet::{SheetRows, SheetSlice, walk_sheet_slice};

/// A streamed sheet's first slice.
pub struct SheetInit {
    pub name: String,             // display name
    pub url: String,              // the sheet file
    pub meta_url: Option<String>, // its entity side table
    pub place: Xform,             // world placement
    pub rows: SheetRows,          // the first segments
    pub fields: SheetFields,      // array positions in the file
    pub resident: u32,            // segments in this slice
}

/// A sheet's slot in the scene.
pub struct SheetBatch {
    pub name: String,                        // display name
    pub url: String,                         // the sheet file
    pub meta_url: Option<String>,            // its entity side table
    pub row: u32,                            // its object row
    pub fields: SheetFields,                 // array positions in the file
    pub place: Xform,                        // world placement
    pub done_to: u32,                        // segments loaded so far
    pub total: u32,                          // segments in the file
    pub resolved: Option<(u32, EntityMeta)>, // entity the last pick found
    pub table: Option<SheetTable>,           // side table head, read once
}

impl Scene {
    /// Add a streamed sheet from its first slice; returns its slot.
    pub fn add_sheet(&mut self, init: SheetInit, gpu: &mut Gpu) -> usize {
        let slot = self.stream_sheet(init);
        self.upload_to(gpu);
        slot
    }

    /// The rows of a streamed sheet's first slice and its read-only shell document.
    pub(crate) fn stream_sheet(&mut self, init: SheetInit) -> usize {
        let SheetInit {
            name,
            url,
            meta_url,
            place,
            rows,
            fields,
            resident,
        } = init;
        let total = fields.count;
        let row = self.push_row(
            self.docs.len(),
            &format!("sheet:{url}"),
            place.clone(),
            Instance::FLAG_SHEET,
        );
        let slice = SheetSlice { rows, from: 0, row };
        let bounds = walk_sheet_slice(&mut self.tables.seg, &slice);
        let o = self.tables.obj.rows.last_mut().unwrap();
        o.bounds = bounds;
        self.tables.bounds.union_with(&bounds.transformed(&place));
        let model = place.clone();
        self.push_doc(
            FileDoc {
                name: name.clone(),
                place,
                session: Rc::new(Session::new(&name)),
                point_px: 0.0,
                display_only: true,
            },
            DocState::default(),
        );
        self.sheets.push(SheetBatch {
            name,
            url,
            meta_url,
            row,
            fields,
            place: model,
            done_to: resident,
            total,
            resolved: None,
            table: None,
        });
        self.sheets.len() - 1 // register:sheets
    }

    /// Add the next slice of sheet `idx`.
    pub fn extend_sheet(&mut self, idx: usize, rows: SheetRows, to: u32, gpu: &mut Gpu) {
        let Some(sheet) = self.sheets.get(idx) else {
            return;
        };

        if to <= sheet.done_to {
            return;
        }

        let place = sheet.place.clone();
        let row = sheet.row;
        let slice = SheetSlice {
            rows,
            from: sheet.done_to,
            row,
        };
        let bounds = walk_sheet_slice(&mut self.tables.seg, &slice);
        self.tables.bounds.union_with(&bounds.transformed(&place));
        self.sheets[idx].done_to = to; // register:sheets
        self.upload_to(gpu);
        gpu.objects
            .grow_local_bounds(&gpu.ctx, row, &bounds, &place);
    }

    /// The sheet slot on object row `row`, if that row is a sheet.
    pub fn sheet_slot(&self, row: u32) -> Option<usize> {
        for (slot, sheet) in self.sheets.iter().enumerate() {
            if sheet.row == row {
                return Some(slot);
            }
        }

        None
    }

    /// The sheet on object row `row`.
    pub fn sheet_at(&self, row: u32) -> Option<&SheetBatch> {
        self.sheets.get(self.sheet_slot(row)?) // register:sheets
    }
}

impl Scene {
    /// A sheet row's name: its picked entity's name or kind, else the sheet's.
    fn sheet_name(&self, row: u32) -> Option<&str> {
        let sheet = self.sheet_at(row)?;
        Some(match &sheet.resolved {
            Some((_, meta)) if !meta.name.trim().is_empty() => &meta.name,
            Some((_, meta)) if !meta.kind.trim().is_empty() => &meta.kind,
            _ => &sheet.name,
        })
    }
}

impl Scene {
    /// The sheet entity a ribbon pick on a sheet row landed on.
    fn sheet_entity_at(&self, pick: Pick, gpu: &Gpu) -> Option<u32> {
        // bit 31 set: the sub id is a ribbon row
        let ribbon = pick.sub & 0x7fff_ffff;

        if pick.sub & 0x8000_0000 != 0
            && self.sheet_at(pick.row).is_some()
            && let Some((parent, _)) = gpu.segments.row_of(ribbon)
            && parent == pick.row
        {
            return gpu.segments.source_id(ribbon).filter(|id| *id != u32::MAX);
        }

        None
    }
}
