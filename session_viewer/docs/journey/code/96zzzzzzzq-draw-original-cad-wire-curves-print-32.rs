fn print_ids(encoder:&mut wgpu::CommandEncoder,image:&Image,pipeline:Option<&wgpu::RenderPipeline>,view:&wgpu::BindGroup,crop:&wgpu::BindGroup,meshes:&[GpuMesh],text:bool){
 let Some(pipeline)=pipeline else{return;};
 let mut pass=overlay(encoder,image,"printed owner IDs",true);pass.set_pipeline(pipeline);pass.set_bind_group(0,view,&[]);pass.set_bind_group(2,crop,&[]);
 for mesh in meshes.iter().filter(|mesh|mesh.printed()&&mesh.geometry.source.print_text()==text){mesh.draw_pulled(&mut pass);}
}

fn overlay<'a>(