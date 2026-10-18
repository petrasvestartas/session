        if encoding.name.is_empty()&&url.split('?').next().is_some_and(|path|path.ends_with(".pb")) {
            if let Some(probe)=crate::stream::probe(&url).await {
                if crate::sheet_range::identity(&probe.bytes).is_some() {
                    let source=crate::sheet_range::Source::from_probe(url.clone(),&probe).await?;
                    if source.fields.count>crate::sheet_stream::PREFIX {
                        if index.cached(&url)&&source.fields.revision.is_some()&&index.etag(&url)==source.fields.revision {continue;}if signal.aborted() {return Err("Drawing source read cancelled".into());}segments=segments.saturating_sub(index.source_segments(&url));let remaining=crate::sheet_stream::CEILING.saturating_sub(segments);let cursor=crate::sheet_stream::start(Rc::new(source),budget.clone(),remaining).await?;if signal.aborted() {cursor.prepared.streamed.as_ref().unwrap().generation.borrow_mut().close();return Err("Drawing source read cancelled".into());}segments=segments.saturating_add(cursor.from);index.stream_sheet(&url,cursor.prepared.clone())?;sheets.push(cursor);continue;
                    }
                }
                if crate::range_shape::single_cloud(&probe.bytes)&&crate::stream::cloud_identity(&probe.bytes).is_some() {
                    let source=crate::range_source::Source::from_probe(url.clone(),&probe).await?;
