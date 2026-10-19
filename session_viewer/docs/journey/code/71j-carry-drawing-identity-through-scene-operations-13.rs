    for row in scene.texts().iter().filter(|row|eligible(row.id)) {if crate::scene_label_screen::matches(row,matrix,size,rectangle) {found.push(row.id);}}
    for row in scene.clouds().iter().filter(|row|eligible(row.id)) {let mut points=row.prepared.source.coords().chunks_exact(3).map(|p|screen(matrix,world(&row.model,[p[0],p[1],p[2]]),size));let hit=if rectangle.crossing {points.any(|p|p.is_some_and(|p|rectangle.contains(p)))} else {points.all(|p|p.is_some_and(|p|rectangle.contains(p)))};if hit {found.push(row.id);}}
    for row in scene.sheets().iter().filter(|row|eligible(row.id)) {let strokes=row.prepared.pages().iter().flat_map(|page|page.coordinates.chunks_exact(6).map(|p|vec![world(&row.model,[p[0],p[1],p[2]]),world(&row.model,[p[3],p[4],p[5]])]).collect::<Vec<_>>()).collect();if matches(matrix,size,rectangle,strokes) {found.push(row.id);}}
    found.sort_unstable_by_key(|id|id.value());found.dedup();found
}
