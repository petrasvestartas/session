            Msg::Sheet(init) => start_sheet(state, init), // register:sheets
            Msg::SheetChunk(c) => state.extend_sheet(c.idx, c.rows, c.to), // register:sheets
            Msg::SheetEntity(resolved) => state.sheet_entity(resolved), // register:sheets
