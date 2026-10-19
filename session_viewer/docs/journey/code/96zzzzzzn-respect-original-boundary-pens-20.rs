  if (occurrence >= arrayLength(&indices)) { return Out(vec4<f32>(2.,2.,2.,1.),vec4<f32>(0.)); }
  let index=indices[occurrence];
  if (index >= arrayLength(&vertices)) { return Out(vec4<f32>(2.,2.,2.,1.),vec4<f32>(0.)); }
  let v=vertices[index];return Out(vec4<f32>(packed_position(v),1.),unpack4x8unorm(v.color));