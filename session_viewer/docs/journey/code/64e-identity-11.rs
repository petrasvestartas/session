    for row in scene.points().iter().filter(|row|eligible(row.id)) {let point=row.point();if matches(matrix,size,rectangle,vec![vec![[point[0],point[1],point[2]]]]) {found.push(row.id);}}
    for row in scene.texts().iter().filter(|row|eligible(row.id)) {if crate::scene_label_screen::matches(row,matrix,size,rectangle) {found.push(row.id);}}
    for row in scene.clouds().iter().filter(|row|eligible(row.id)) {let mut points=row.prepared.source.coords().chunks_exact(3).map(|p|screen(matrix,world(&row.model,[p[0],p[1],p[2]]),size));let hit=if rectangle.crossing {points.any(|p|p.is_some_and(|p|rectangle.contains(p)))} else {points.all(|p|p.is_some_and(|p|rectangle.contains(p)))};if hit {found.push(row.id);}}
    found.sort_unstable_by_key(|id|id.value());found.dedup();found
}
