    usage[1]=sources.len();usage[2]=prepared.len();usage
}

pub fn cads<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;6] {
    let (mut sources,mut prepared)=(HashSet::new(),HashSet::new());let mut counts=[0;6];
    for scene in scenes {for row in scene.objects() {if let Some(cad)=&row.cad {
        counts[0]+=1;sources.insert(Rc::as_ptr(&cad.source));
        if prepared.insert(Rc::as_ptr(cad)) {counts[3]+=cad.samples.len();counts[4]+=cad.face_count;counts[5]+=cad.edge_ids.len();}
    }}}
    counts[1]=sources.len();counts[2]=prepared.len();counts
}
