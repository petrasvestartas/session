  Self{projected,buffer,pipes,line,line_group,raster,scan,rows:Vec::new(),witness:Vec::new(),triangles:0,viewport:[0;2],layout:None,target:None,pool:0,report:PoolReport::new(device),key:None,projections:0,bins:0,reported:None,fallback:false}
 }
 pub fn set(&mut self,device:&wgpu::Device,meshes:&[GpuMesh]){
  let limit=u64::from(device.limits().max_storage_buffer_binding_size).min(device.limits().max_buffer_size);
  let count=meshes.iter().try_fold(0u32,|count,row|count.checked_add(row.geometry.index_count/3));
  self.fallback=count.is_none_or(|count|u64::from(count)*PROJECTED_BYTES>limit);
  if self.fallback{self.rows.clear();self.witness.clear();self.triangles=0;self.key=None;return}
  let witness=meshes.iter().filter(|row|row.geometry.index_count>0).map(|row|(row.geometry.vertices.clone(),row.geometry.indices.clone(),row.model_buffer().clone(),row.packed()[..64].try_into().unwrap())).collect::<Vec<_>>();
  if witness==self.witness{return}
  self.rows.clear();let mut offset=0;
  for row in meshes.iter().filter(|row|row.geometry.index_count>0){let triangles=row.geometry.index_count/3;
   let bytes=[triangles,offset,0,0].into_iter().flat_map(u32::to_ne_bytes).collect::<Vec<_>>();
   let count=device.create_buffer_init(&wgpu::util::BufferInitDescriptor{label:Some("finite original triangle interval"),contents:&bytes,usage:wgpu::BufferUsages::UNIFORM});
   self.rows.push(Row{vertices:row.geometry.vertices.clone(),indices:row.geometry.indices.clone(),model:row.model_buffer().clone(),count,triangles,group:None});offset+=triangles;
  }
  self.witness=witness;self.triangles=count.unwrap();self.key=None;
 }


