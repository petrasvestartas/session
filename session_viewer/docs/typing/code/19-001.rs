    Sheet(Box<SheetInit>),         // a drawing sheet starts streaming; register:sheets
    SheetChunk(SheetChunk),        // more segments arrived; register:sheets
    SheetEntity(app::sheet_query::Resolved), // a picked sheet entity answered; register:sheets
