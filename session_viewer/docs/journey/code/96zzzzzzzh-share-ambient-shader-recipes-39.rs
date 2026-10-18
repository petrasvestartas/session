at],true,samples);
  let edges=(samples==4).then(||pipeline(device,&composite_shader,"vs_edges","fs_edges",&[Some(&composite_layout),Some(&sample)],&[format],true,samples));
  let