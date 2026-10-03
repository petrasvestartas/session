struct ObjectSettings {
    model: mat4x4<f32>,
    selection: vec4<f32>,
}
@group(1) @binding(0) var<uniform> object: ObjectSettings;
