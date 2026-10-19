    usage[1]=documents.len();usage[2]=definitions.len();usage
}

pub fn sheets<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;5] {
    let mut sources=HashSet::new();let mut pages=HashSet::new();let mut count=[0;5];for scene in scenes {count[0]+=scene.sheets().len();for row in scene.sheets() {let source=&row.prepared.original;if sources.insert(Rc::as_ptr(source)) {count[2]+=source.coords.capacity()*8+(source.colors.capacity()+source.widths.capacity()+source.source_ids.capacity())*4+source.guid.capacity()+source.name.capacity()+source.meta.capacity();}for page in row.prepared.pages() {if pages.insert(page.key()) {count[3]+=page.bytes();count[4]+=(page.rows.end-page.rows.start) as usize;}}}}count[1]=sources.len();count
}
