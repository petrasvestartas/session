    let point=[(clip(0)/w+1.)*size[0]/2.,(1.-clip(1)/w)*size[1]/2.];point.iter().all(|value|value.is_finite()).then_some(point)
}

fn world(matrix:&session_rust::Xform,point:[f64;3])->[f64;3] {let point=matrix.transform_point(&session_rust::Point::new(point[0],point[1],point[2]));[point[0],point[1],point[2]]}

fn matches(matrix:&[f64;16],size:[f64;2],rectangle:Rectangle,strokes:Vec<Vec<[f64;3]>>)->bool {
    if strokes.is_empty() {return false;}
    if !rectangle.crossing {return strokes.iter().flatten().all(|point|screen(matrix,*point,size).is_some_and(|point|rectangle.contains(point)));}
    strokes.iter().any(|stroke| {
        let points=stroke.iter().map(|point|screen(matrix,*point,size)).collect::<Vec<_>>();
        points.iter().flatten().any(|point|rectangle.contains(*point))||points.windows(2).any(|pair|matches!(pair,[Some(a),Some(b)] if rectangle.crosses(*a,*b)))
    })
}

pub fn found(scene:&Scene,matrix:&[f64;16],size:[f64;2],rectangle:Rectangle)->Vec<ObjectId> {
    let eligible=|id:ObjectId|scene.pick_id(id.value()).is_some();let mut found=Vec::new();
    for row in scene.objects().iter().filter(|row|eligible(row.id)) {
        let mut strokes=row.mesh.parts.endpoints.iter().map(|ends|ends.iter().map(|point|world(&row.model,*point)).collect()).collect::<Vec<_>>();
        if let Some(source)=row.geometry() {strokes.extend(source.vertices().into_iter().filter_map(|key|source.vertex_point(key)).map(|point|vec![world(&row.model,[point[0],point[1],point[2]])]));}
        if matches(matrix,size,rectangle,strokes) {found.push(row.id);}
    }
    for row in scene.lines().iter().filter(|row|eligible(row.id)) {if matches(matrix,size,rectangle,vec![row.endpoints().map(|point|[point[0],point[1],point[2]]).to_vec()]) {found.push(row.id);}}
    for row in scene.paths().iter().filter(|row|eligible(row.id)) {if matches(matrix,size,rectangle,vec![row.points().into_iter().map(|point|[point[0],point[1],point[2]]).collect()]) {found.push(row.id);}}
    for row in scene.points().iter().filter(|row|eligible(row.id)) {let point=row.point();if matches(matrix,size,rectangle,vec![vec![[point[0],point[1],point[2]]]]) {found.push(row.id);}}
    found.sort_unstable_by_key(|id|id.value());found.dedup();found
}
