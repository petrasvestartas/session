    let points=if matches!(target.part,Part::SourceControl(_)) {
        crate::source_part::points(scene,target).map_err(|_|"Cannot resolve original control")?
    } else if let Some(row)=