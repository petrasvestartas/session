struct ViewSettings { transform: mat4x4<f32>, display: vec4<f32>, }
@group(0) @binding(0) var<uniform> view: ViewSettings;
@group(1) @binding(0) var<uniform> object_settings: ObjectSettings;
fn object_at(index:u32)->ObjectSettings {return object_settings;}

struct CountOut {
    @builtin(position) pos: vec4<f32>, // clip position
    @location(0) world_pos: vec3<f32>, // scene-space position
    @location(1) @interpolate(flat) inst_id: u32, // object row
    @location(2) @interpolate(flat) plane: u32, // clipping plane
    @location(3) @interpolate(flat) flip: u32, // 1 when the solid is mirrored or wound inward
    @location(4) @interpolate(flat) selected: u32, // nonzero when the solid is selected
}

fn count_corner(position:vec3<f32>,plane:u32,object:ObjectSettings)->CountOut{
 var out:CountOut;out.pos=vec4<f32>(3.0,3.0,0.5,1.0);
 if((object.selection.z&(FLAG_CLOSED|FLAG_CLIPPING_PLANE))!=FLAG_CLOSED){return out;}
 let world=object.model*vec4<f32>(position,1.0);let mirrored=dot(object.model[0].xyz,cross(object.model[1].xyz,object.model[2].xyz))<0.0;
 out.pos=view.transform*world;out.world_pos=world.xyz;out.inst_id=object.selection.w;out.plane=plane;out.flip=select(0u,1u,mirrored!=((object.selection.z&FLAG_INWARD)!=0u));out.selected=object.selection.x&1u;return out;
}
@vertex fn vs_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{return count_corner(position,plane,object_at(0u));}
@group(3) @binding(0) var<storage,read> cap_vertices:array<PackedVertex>;
@group(3) @binding(1) var<uniform> cap_plane:vec4<u32>;
@group(3) @binding(2) var<storage,read> cap_indices:array<u32>;
@vertex fn vs_record_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=cap_vertices[cap_indices[occurrence+object.range.x]];return count_corner(packed_position(vertex),cap_plane.x,object);}
fn behind(plane: u32, p: vec3<f32>) -> f32 {
    return -clip_eye_side(plane) * clip_distance(plane, p);
}
fn leaving(in: CountOut, front: bool) -> bool {
    return front == (in.flip != 0u);
}
@fragment fn fs_count(in:CountOut,@builtin(front_facing) front:bool)->@location(0) vec4<f32>{if(clip_eye_side(in.plane)==0.0||behind(in.plane,in.world_pos)<0.0){discard;}let sign=select(-1.0,1.0,leaving(in,front));let weight=select(1.0,SELECTED_CROSSING,in.selected!=0u);return vec4<f32>(sign*weight,0.0,0.0,0.0);}
@group(3) @binding(0) var<storage,read_write> pick_caps:array<atomic<u32>>;
struct PickFrame{full:vec2<u32>,size:vec2<u32>,origin:vec2<u32>,mode:u32,pad:u32}
@group(3) @binding(2) var<uniform> pick_frame:PickFrame;
fn pick_cap_word(plane: u32, px: vec2<f32>) -> u32 {
    return clip_pick_word(plane, px, pick_frame.size);
}
@fragment fn fs_pick_count(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0) {
        discard;
    }

    let leaves = leaving(in, front);
    let sign = select(-1, 1, leaves);
    let word = pick_cap_word(in.plane, in.pos.xy);
    atomicAdd(&pick_caps[word], bitcast<u32>(sign));
    atomicAdd(&pick_caps[word + 1u], bitcast<u32>(sign * i32(in.inst_id)));

    if (leaves) {
        atomicMax(&pick_caps[word + 2u], bitcast<u32>(in.pos.z));
    }

    return vec4<f32>(0.0);
}
@fragment
fn fs_pick_owner(in: CountOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    if (clip_eye_side(in.plane) == 0.0 || behind(in.plane, in.world_pos) < 0.0 || !leaving(in, front)) {
        discard;
    }

    let word = pick_cap_word(in.plane, in.pos.xy);

    if (bitcast<u32>(in.pos.z) == atomicLoad(&pick_caps[word + 2u])) {
        atomicStore(&pick_caps[word + 3u], in.inst_id);
    }

    return vec4<f32>(0.0);
}

struct Crop { transform: mat4x4<f32>, mode: u32, _a: u32, _b: u32, _c: u32 }
@group(3) @binding(3) var<storage,read> pick_cap_vertices:array<PackedVertex>;
@group(3) @binding(4) var<uniform> pick_cap_plane:vec4<u32>;
@group(3) @binding(5) var<storage,read> pick_cap_indices:array<u32>;
@vertex fn vs_record_pick_count(@builtin(vertex_index) occurrence:u32,@location(0) row:u32)->CountOut {let object=object_at(row);let vertex=pick_cap_vertices[pick_cap_indices[occurrence+object.range.x]];var output=count_corner(packed_position(vertex),pick_cap_plane.x,object);output.pos=cap_crop.transform*output.pos;return output;}
fn wanted(pixel: u32, mode: u32) -> bool {
    if (mode == 0u) { return true; }
    if (pixel == 0u) { return false; }
    let kind = (pixel - 1u) & 0xe0000000u;
    return (mode == 2u && kind == 0x80000000u) || (mode == 3u && kind == 0x40000000u);
}

@group(3) @binding(1) var<uniform> cap_crop:Crop;
@vertex fn vs_pick_count(@location(0) position:vec3<f32>,@builtin(instance_index) plane:u32)->CountOut{var output=count_corner(position,plane,object_at(0u));output.pos=cap_crop.transform*output.pos;return output;}
