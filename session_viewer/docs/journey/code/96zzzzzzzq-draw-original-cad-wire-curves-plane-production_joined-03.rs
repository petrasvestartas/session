    if out.ids.len() != out.owners.len() {
        return Err("Joined source IDs do not match production rows");
    }
    if [&out.segments, &out.objects, &out.translations, &out.heads, &out.planes]
        .iter()
        .any(|b| b.len() as u64 > limit)
        || out.ids.len() as u64 * 8 > limit
