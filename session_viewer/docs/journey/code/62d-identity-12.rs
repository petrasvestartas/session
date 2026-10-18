    counts[1]=sources.len();counts[2]=prepared.len();counts
}

pub fn texts<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;4] {
 let mut sources=HashSet::new();let mut usage=[0;4];for scene in scenes {usage[0]+=scene.texts().len();usage[2]+=scene.texts().len()*std::mem::size_of::<crate::scene_label::Row>();for row in scene.texts() {if sources.insert(Rc::as_ptr(&row.prepared.source)) {let source=&row.prepared.source;usage[3]+=std::mem::size_of::<crate::scene_label::Source>()+source.guid.capacity()+source.name.capacity()+source.label.text.capacity();}}}usage[1]=sources.len();usage
}
