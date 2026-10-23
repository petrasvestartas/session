use crate::scene::{ObjectId, Scene};

pub const SOURCE: &str = include_str!("production_ribbon.wgsl");
pub const PROVENANCE: &str = include_str!("production_ribbon_provenance.json");

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Segment {
    pub p0: [f32; 3],
    pub radius: f32,
    pub p1: [f32; 3],
    pub instance_id: u32,
    pub color: u32,
    pub facing: u32,
    pub previous: u32,
    pub next: u32,
    pub sag: f32,
}

const _: () = {
    assert!(std::mem::size_of::<Segment>() == 52);
    assert!(std::mem::offset_of!(Segment, radius) == 12);
    assert!(std::mem::offset_of!(Segment, color) == 32);
};

pub fn encode_width(width: f64) -> f32 {
    if width.is_finite() && width > 0.0 && (width - 1.0).abs() > 1e-9 {
        -(width as f32) * 0.5
    } else {
        0.0
    }
}

pub fn pack_rgba(color: [f32; 4]) -> u32 {
    let quant8 = |v: f32| ((v.clamp(0.0, 1.0) * 255.0 + 0.5) as u32) & 0xff;
    quant8(color[0]) | quant8(color[1]) << 8 | quant8(color[2]) << 16 | quant8(color[3]) << 24
}

pub fn floats(bytes: &mut Vec<u8>, values: impl IntoIterator<Item = f32>) {
    bytes.extend(values.into_iter().flat_map(f32::to_ne_bytes));
}

pub struct Rows {
    pub segments: Vec<u8>,
    pub objects: Vec<u8>,
    pub translations: Vec<u8>,
    pub owners: Vec<u32>,
}

pub fn rows(
    scene: &Scene,
    anchor: [f64; 3],
    selected: &[ObjectId],
    storage_limit: u64,
) -> Result<Rows, &'static str> {
    let lines: Vec<_> = scene
        .lines()
        .iter()
        .filter(|row| scene.visible(row.id))
        .collect();
    let count = lines.iter().map(|row| row.id.value()).max().unwrap_or(1) as usize;
    if count as u64 > storage_limit / 96 {
        return Err("Production line object rows exceed the device limit");
    }
    let mut objects = Vec::with_capacity(count * 96);
    let identity = session_rust::Xform::identity().to_f32();
    for _ in 0..count {
        floats(&mut objects, identity);
        floats(&mut objects, [1.0; 4]);
        objects.extend(2u32.to_ne_bytes());
        objects.extend([0; 12]);
    }
    let mut translations = vec![0; count * 16];
    let mut segments = Vec::with_capacity(lines.len() * 52);
    let mut owners = Vec::with_capacity(lines.len());
    for row in lines {
        let source = &row.prepared.source;
        let owner = row.id.value();
        let index = owner.checked_sub(1).ok_or("Invalid original line owner")?;
        let model = row.model.to_f32();
        let mut local = model;
        local[12..15].fill(0.0);
        let mut object = Vec::with_capacity(96);
        floats(&mut object, local);
        let override_color = scene.face_rgba(row.id);
        floats(&mut object, override_color.unwrap_or([1.0; 4]));
        let flags =
            u32::from(selected.contains(&row.id)) | if override_color.is_some() { 256 } else { 0 };
        object.extend(flags.to_ne_bytes());
        object.extend([0; 12]);
        objects[index as usize * 96..(index as usize + 1) * 96].copy_from_slice(&object);
        let translation = std::array::from_fn::<_, 4, _>(|i| {
            if i < 3 {
                (row.model.m[12 + i] - anchor[i]) as f32
            } else {
                0.0
            }
        });
        let offset = index as usize * 16;
        for (out, value) in translations[offset..offset + 16]
            .chunks_exact_mut(4)
            .zip(translation)
        {
            out.copy_from_slice(&value.to_ne_bytes());
        }
        let segment = Segment {
            p0: [source[0] as f32, source[1] as f32, source[2] as f32],
            radius: encode_width(source.width),
            p1: [source[3] as f32, source[4] as f32, source[5] as f32],
            instance_id: index,
            color: pack_rgba(source.linecolor.to_f32()),
            facing: u32::MAX,
            previous: u32::MAX,
            next: u32::MAX,
            sag: 0.0,
        };
        floats(&mut segments, segment.p0);
        floats(&mut segments, [segment.radius]);
        floats(&mut segments, segment.p1);
        segments.extend(
            [
                segment.instance_id,
                segment.color,
                segment.facing,
                segment.previous,
                segment.next,
            ]
            .into_iter()
            .flat_map(u32::to_ne_bytes),
        );
        floats(&mut segments, [segment.sag]);
        owners.push(owner);
    }
    Ok(Rows {
        segments,
        objects,
        translations,
        owners,
    })
}

pub fn line_uniform(
    matrix: [f32; 16],
    size: [u32; 2],
    density: f32,
    opacity: f32,
    anchor: [f64; 3],
) -> Vec<u8> {
    let frame = session_rust::Xform::from_matrix(matrix.map(f64::from));
    let eye = frame.eye();
    let mut bytes = Vec::with_capacity(80);
    floats(
        &mut bytes,
        [
            density,
            1.0 / (60.0_f32 * 0.5).to_radians().tan() * 0.001,
            frame.ortho_half_height() as f32,
            size[1] as f32,
            size[0] as f32,
            eye[0] as f32,
            eye[1] as f32,
            eye[2] as f32,
        ],
    );
    floats(&mut bytes, anchor.map(|v| v as f32));
    floats(
        &mut bytes,
        [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            size[0] as f32,
            size[1] as f32,
            opacity,
            100.0,
        ],
    );
    assert_eq!(bytes.len(), 80);
    bytes
}

pub fn pick_frame(
    matrix: [f32; 16],
    mut line: Vec<u8>,
    window: crate::pick_window::Window,
) -> ([f32; 16], Vec<u8>) {
    let [x, y, w, h] = window.draw;
    let (fw, fh) = (window.full[0].max(1) as f32, window.full[1].max(1) as f32);
    let (w, h) = (w.max(1) as f32, h.max(1) as f32);
    let (sx, sy) = (fw / w, fh / h);
    let crop = [
        sx,
        0.0,
        0.0,
        0.0,
        0.0,
        sy,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        sx - 2.0 * x as f32 / w - 1.0,
        1.0 - (fh - 2.0 * y as f32) / h,
        0.0,
        1.0,
    ];
    let mut mvp = [0.0; 16];
    for col in 0..4 {
        for row in 0..4 {
            mvp[col * 4 + row] = (0..4)
                .map(|k| crop[k * 4 + row] * matrix[col * 4 + k])
                .sum();
        }
    }
    let read = |offset| f32::from_ne_bytes(line[offset..offset + 4].try_into().unwrap());
    let ratio = fh / h;
    let values = [
        (4, read(4) * ratio),
        (8, read(8) / ratio),
        (12, h),
        (16, w),
        (56, x as f32),
        (60, y as f32),
    ];
    for (offset, value) in values {
        line[offset..offset + 4].copy_from_slice(&value.to_ne_bytes());
    }
    (mvp, line)
}
