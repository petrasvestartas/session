    }
}

pub fn surfaces<'a>(scenes:impl IntoIterator<Item=&'a Scene>)->[usize;5] {
    let mut sources=HashSet::new();let mut prepared=HashSet::new();let mut usage=[0;5];
    for scene in scenes {for row in scene.objects() {if let Some(surface)=row.surface.as_ref() {
        usage[0]+=1;let source=&surface.source;
        if sources.insert(Rc::as_ptr(source)) {usage[3]+=source.m_cv.capacity()*8+source.m_nurbsknot.iter().map(|knots|knots.capacity()*8).sum::<usize>()+source.facecolors.capacity()*std::mem::size_of::<session_rust::Color>()+source.linecolors.capacity()*std::mem::size_of::<session_rust::Color>()+source.name.capacity();}
        if prepared.insert(Rc::as_ptr(surface)) {usage[4]+=surface.grid.samples.capacity()*std::mem::size_of::<crate::surface::Sample>()+surface.grid.indices.capacity()*4+surface.grid.boundaries.capacity()*std::mem::size_of::<crate::surface::Boundary>()+surface.grid.boundaries.iter().map(|b|b.segments.capacity()*8).sum::<usize>();}
    }}}
    usage[1]=sources.len();usage[2]=prepared.len();usage
}
