 let mut sources=HashSet::new();let mut usage=[0;4];for scene in scenes {usage[0]+=scene.texts().len();usage[2]+=scene.texts().len()*std::mem::size_of::<crate::scene_label::Row>();for row in scene.texts() {if sources.insert(Rc::as_ptr(&row.prepared.source)) {let source=&row.prepared.source;usage[3]+=std::mem::size_of::<crate::scene_label::Source>()+source.guid.capacity()+source.name.capacity()+source.label.text.capacity();}}}usage[1]=sources.len();usage
}

pub fn clouds<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;5] {
    let mut seen=HashSet::new();let mut nodes=HashSet::new();let mut usage=[0;5];
    for scene in scenes {usage[0]+=scene.clouds().len();for row in scene.clouds() {let source=&row.prepared.source;
        if seen.insert(Rc::as_ptr(source)) {usage[2]+=source.len();usage[3]+=source.coords().len()*8+source.colors().len()*4+source.normals().len()*8+source.point_ids().len()*4+source.lod_node_count()*84;}
        if nodes.insert(Rc::as_ptr(&row.prepared.nodes)) {usage[4]+=row.prepared.nodes.capacity()*std::mem::size_of::<crate::cloud::LodNode>();}
    }}
    usage[1]=seen.len();usage
}
