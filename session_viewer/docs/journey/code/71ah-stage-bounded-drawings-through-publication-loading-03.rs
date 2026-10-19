    }
    let extra=crate::stream_budget::MAX_POINTS.saturating_sub(points)/(cursors.len().max(1) as u32);for cursor in &mut cursors {cursor.ceiling=cursor.loaded+extra;}
    let extra=crate::sheet_stream::CEILING.saturating_sub(segments)/(sheets.len().max(1) as u32);for cursor in &mut sheets {cursor.ceiling=cursor.from.saturating_add(extra);}
    Ok((cursors,sheets))
}
