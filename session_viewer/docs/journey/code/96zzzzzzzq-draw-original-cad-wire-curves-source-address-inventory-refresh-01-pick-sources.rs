let path = &editor.scene.paths()[0];
    assert!(matches!(&path.prepared.source, crate::chain::Source::Curve(..)));
    let points = crate::production_joined::source_points(&path.prepared.source);
    let expected_addresses = points.windows(2)
        .filter(|pair| pair[0] != pair[1] || matches!(&path.prepared.source, crate::chain::Source::Line(_)))
        .count();
    assert!(expected_addresses > 0);
    assert_eq!(sources.path_ids.len(), expected_addresses);
    assert_eq!(sources.path_ids.iter().collect::<std::collections::HashSet<_>>().len(), expected_addresses);
    for &address in &sources.path_ids {
        let target = sources.resolve(&editor.scene, address, Mode::Edge).expect("Production curve address resolves");
        assert_eq!(target.owner, path.id);
        assert_eq!(target.part, Part::Curve);
    }
