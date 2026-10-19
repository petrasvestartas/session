th: window.copy[2], height: window.copy[3], depth_or_array_layers: 1 });
        encoder.finish()
}

fn overlay<'a>(encoder:&'a mut wgpu::CommandEncoder,image:&'a Image,label:&'a str,physical:bool)->wgpu::RenderPass<'a>{
 let colours=[
  // original owner attachment,
  // original primitive attachment,
 ];
 // original owner pass
}
