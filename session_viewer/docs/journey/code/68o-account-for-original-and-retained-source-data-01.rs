    usage[1]=seen.len();usage
}

pub fn streams<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;5] {
    let mut sources=HashSet::new();let mut pages=HashSet::new();let mut usage=[0;5];
    for scene in scenes {usage[0]+=scene.streams().len();for row in scene.streams() {
        if sources.insert(Rc::as_ptr(&row.prepared.source)) {usage[2]+=row.prepared.source.fields.count as usize;}
        for page in row.prepared.pages() {if pages.insert(Rc::as_ptr(&page)) {usage[3]+=page.bytes();usage[4]+=page.value.ids.len();}}
    }}usage[1]=sources.len();usage
}
