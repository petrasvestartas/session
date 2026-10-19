pub fn session(source: &proto::Session) -> Result<(), String> {
    if let Some(objects) = &source.objects {
        let count = objects.planes.len()+objects.bboxes.len()+objects.points.len()
            + objects.lines.len()
            + objects.polylines.len()
