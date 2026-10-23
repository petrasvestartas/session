pub const POINT_SOURCE: &str = include_str!("production_splat.wgsl");
pub const RESOLVE_SOURCE: &str = include_str!("production_splat_resolve.wgsl");
pub const PROVENANCE: &str = include_str!("production_cloud_provenance.json");

pub fn point_source() -> String {
    format!("{POINT_SOURCE}\n{}", include_str!("cloud_point_ids.wgsl"))
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SplatRecord {
    pub mvp_model: [f32; 16],
    pub tint: [f32; 4],
    pub first: u32,
    pub count: u32,
    pub cum: u32,
    pub k: f32,
    pub rot: [f32; 12],
    pub nrm_first: u32,
    pub instance: u32,
    pub flags: u32,
    pub selected_point: u32,
}

const _: () = {
    assert!(std::mem::size_of::<SplatRecord>() == 160);
    assert!(std::mem::offset_of!(SplatRecord, tint) == 64);
    assert!(std::mem::offset_of!(SplatRecord, first) == 80);
    assert!(std::mem::offset_of!(SplatRecord, k) == 92);
    assert!(std::mem::offset_of!(SplatRecord, rot) == 96);
    assert!(std::mem::offset_of!(SplatRecord, selected_point) == 156);
};

impl SplatRecord {
    pub fn append(&self, bytes: &mut Vec<u8>) {
        let words = self.mvp_model.map(f32::to_bits).into_iter()
            .chain(self.tint.map(f32::to_bits))
            .chain([self.first, self.count, self.cum, self.k.to_bits()])
            .chain(self.rot.map(f32::to_bits))
            .chain([self.nrm_first, self.instance, self.flags, self.selected_point]);
        bytes.extend(words.flat_map(u32::to_ne_bytes));
    }
}
