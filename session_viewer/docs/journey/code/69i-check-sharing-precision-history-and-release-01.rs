    }}usage[1]=sources.len();usage
}

pub fn instances<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;3] {
    let mut documents=HashSet::new();let mut definitions=HashSet::new();let mut usage=[0;3];
    for scene in scenes {usage[0]+=scene.instances.len();for owner in scene.instances.values() {let document=Rc::as_ptr(&owner.document) as usize;documents.insert(document);definitions.insert((document,owner.definition.guid().to_owned()));}}
    usage[1]=documents.len();usage[2]=definitions.len();usage
}
