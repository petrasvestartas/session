use session_rust::Point;
pub struct Sample {pub vertex:u32,pub surface:usize,pub uv:[f64;2],pub sign:f64}
pub struct Data {pub points:Vec<Point>,pub faces:Vec<Vec<usize>>,pub vertices:Vec<[f32;6]>,pub normals:Vec<[f32;3]>,pub indices:Vec<u32>,pub face_ids:Vec<u32>,pub keys:Vec<std::collections::HashMap<usize,u32>>,pub samples:Vec<Sample>}

pub fn prepare(boundary:&crate::cad_boundary::Prepared)->Result<Data,&'static str> {
        let source=&boundary.source;let signs=crate::cad_orient::face_signs(source,&boundary.faces,&boundary.chains);
        let colour=source.surfacecolor.to_f32();let (mut points,mut faces,mut vertices,mut normals,mut indices,mut face_ids)=(Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new(),Vec::new());
        let mut mesh_keys=Vec::new();let mut samples=Vec::new();
        for (fi,mesh) in boundary.faces.iter().enumerate() {
            let render=mesh.to_render();let mut keys=mesh.vertex.keys().copied().collect::<Vec<_>>();keys.sort_unstable();if keys.len()!=render.vertices.len() {return Err("Split face vertices require an explicit source map");}
            let base=u32::try_from(vertices.len()).map_err(|_|"Too many CAD display vertices")?;let map=keys.iter().enumerate().map(|(i,key)|(*key,base+i as u32)).collect::<std::collections::HashMap<_,_>>();mesh_keys.push(map);
            for (&key,vertex) in keys.iter().zip(&render.vertices) {
                let original=&mesh.vertex[&key];points.push(Point::new(original.x,original.y,original.z));let [x,y,z]=vertex.position;vertices.push([x,y,z,colour[0],colour[1],colour[2]]);normals.push(vertex.normal.map(|v|(v as f64*signs[fi]) as f32));
            }
            for sample in crate::cad_samples::face(boundary,fi)? {samples.push(Sample {vertex:mesh_keys[fi][&sample.key],surface:sample.surface,uv:sample.uv,sign:signs[fi]});}
            for tri in render.indices.chunks_exact(3) {
                let mut tri=[base+tri[0],base+tri[1],base+tri[2]];if signs[fi]<0. {tri.swap(1,2);}indices.extend_from_slice(&tri);faces.push(tri.map(|i|i as usize).to_vec());face_ids.push(fi as u32);
            }
        }
        Ok(Data {points,faces,vertices,normals,indices,face_ids,keys:mesh_keys,samples})
}
