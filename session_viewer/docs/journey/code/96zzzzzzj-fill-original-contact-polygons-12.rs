    output.normal = (object.normal_matrix * vec4<f32>(normal, 0.0)).xyz;
    if(contact) {output.position=contact_layer(output.position,world.xyz,output.normal,view.transform,view.viewport.xy,view.eye);}
