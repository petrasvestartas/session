use session_rust::proto;
pub fn source() -> proto::Sheet {
    proto::Sheet {
        guid: "00000000-0000-0000-0000-000000000711".into(),
        name: "Original plan segments".into(),
        coords: vec![
            -0.8, -0.5, 0.3, 0.8, -0.5, 0.3, 0.8, -0.5, 0.3, 0.8, 0.5, 0.3, 0.8, 0.5, 0.3, -0.8,
            0.5, 0.3, -0.8, 0.5, 0.3, -0.8, -0.5, 0.3, -0.4, 0., 0.3, 0.4, 0., 0.3,
        ],
        colors: vec![0xff00_0000; 4]
            .into_iter()
            .chain([0xffd0_5020])
            .collect(),
        widths: vec![0.025, 0.025, 0.025, 0.025, 0.],
        segment_count: 5,
        entity_count: 3,
        meta: "plan.shm".into(),
        source_ids: vec![1, 1, 1, 1, 2],
    }
}
pub fn bytes() -> Vec<u8> {
    use prost::Message;
    proto::Session {
        name: "Drawing sheet".into(),
        objects: Some(proto::Objects {
            sheets: vec![source()],
            ..Default::default()
        }),
        ..Default::default()
    }
    .encode_to_vec()
}
pub fn metadata() -> Vec<u8> {
    let records = [
        r#"{}"#,
        r#"{"guid":"source-wall","name":"Plan boundary","kind":"wall","width":0.025,"color":[0,0,0,255]}"#,
        r#"{"guid":"source-axis","name":"Centre axis","kind":"axis","width":0,"color":[32,80,208,255]}"#,
    ];
    let mut out = b"SHM1".to_vec();
    out.extend(3u32.to_le_bytes());
    let mut offset = 0u64;
    for record in records {
        out.extend(offset.to_le_bytes());
        out.extend((record.len() as u64).to_le_bytes());
        offset += record.len() as u64;
    }
    for record in records {
        out.extend(record.as_bytes());
    }
    out
}
