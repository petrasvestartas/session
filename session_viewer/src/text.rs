use bytemuck::{Pod, Zeroable};
use wasm_bindgen::JsCast;

// ── TextLabel ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct TextLabel {
    pub guid:     String,
    pub position: [f32; 3],
    pub text:     String,
    pub color:    [u8; 4],
}

// ── TextVertex (background quad) ──────────────────────────────────────────────

/// Vertex for the background-quad pipeline (text.wgsl).
/// 52 bytes: world anchor (12) + pixel offset (8) + local pos (8) + quad size (8) + color (16).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TextVertex {
    pub world_pos:   [f32; 3],
    pub quad_offset: [f32; 2],  // pixel offset from projected anchor (+y = screen-up)
    pub local:       [f32; 2],  // pixel position within the quad (0,0 = bottom-left)
    pub size:        [f32; 2],  // quad dimensions in pixels (width, height)
    pub color:       [f32; 4],
}

impl TextVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 5] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x2, 3 => Float32x2, 4 => Float32x4];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// ── GlyphVertex (font atlas sampling) ─────────────────────────────────────────

/// Vertex for the glyph pipeline (glyph.wgsl).
/// 44 bytes: world anchor (12) + px offset (8) + UV (8) + RGBA f32 color (16).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct GlyphVertex {
    pub world_pos: [f32; 3],
    pub glyph_px:  [f32; 2],   // pixel offset from projected anchor (+y = screen-up)
    pub glyph_uv:  [f32; 2],   // UV into font atlas
    pub color:     [f32; 4],
}

impl GlyphVertex {
    pub const ATTRIBS: [wgpu::VertexAttribute; 4] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x2, 3 => Float32x4];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

// Font atlas constants: 16 cols × 6 rows, 8×16 px per cell → 128×96 px total.
// "bold 14px monospace" advance ≈ 8px, matching the 8px cell width.
const CHAR_W: f32 = 8.0;
const CHAR_H: f32 = 16.0;
const ATLAS_COLS: f32 = 16.0;
const ATLAS_ROWS: f32 = 6.0;
pub const ATLAS_WIDTH: u32 = 128;
pub const ATLAS_HEIGHT: u32 = 96;  // 6 rows × 16px

// Label layout constants (pixels, screen +y = up).
// Label is centered on the 3D point — offsets are symmetric around 0.
const LABEL_PAD_X: f32 = 6.0;     // left/right padding inside bg rect
const LABEL_PAD_Y: f32 = 2.0;     // top/bottom padding inside bg rect
const GLYPH_Y_OFFSET: f32 = -2.0; // shift glyphs down so cap-height is visually centred in bg rect

// ── background quad builder ────────────────────────────────────────────────────

/// Build black semi-transparent background quads for `labels`.
/// Returns 6 `TextVertex` per label (two triangles).
pub fn build_label_quads(labels: &[&TextLabel], selected: &std::collections::HashSet<String>) -> Vec<TextVertex> {
    let mut verts = Vec::with_capacity(labels.len() * 6);
    for label in labels {
        let color = if selected.contains(&label.guid) {
            [1.0, 1.0, 0.0, 1.0]
        } else {
            [
                label.color[0] as f32 / 255.0,
                label.color[1] as f32 / 255.0,
                label.color[2] as f32 / 255.0,
                1.0_f32,
            ]
        };
        let w = LABEL_PAD_X + label.text.len() as f32 * CHAR_W + LABEL_PAD_X;
        let h = LABEL_PAD_Y + CHAR_H + LABEL_PAD_Y;
        let x0 = -w * 0.5;
        let x1 =  w * 0.5;
        let y0 = -h * 0.5;
        let y1 =  h * 0.5;
        let pos = label.position;
        // local coords: (0,0) = bottom-left corner of the quad
        for &[ox, oy, lx, ly] in &[
            [x0, y0, 0.0, 0.0], [x1, y0, w, 0.0], [x1, y1, w, h],
            [x0, y0, 0.0, 0.0], [x1, y1, w,  h ], [x0, y1, 0.0, h],
        ] {
            verts.push(TextVertex {
                world_pos: pos,
                quad_offset: [ox, oy],
                local: [lx, ly],
                size: [w, h],
                color,
            });
        }
    }
    verts
}

// ── glyph quad builder ─────────────────────────────────────────────────────────

/// Build glyph quads for all characters in `labels`.
/// Returns 6 `GlyphVertex` per character (two triangles).
pub fn build_glyph_quads(labels: &[&TextLabel]) -> Vec<GlyphVertex> {
    let mut verts = Vec::new();
    for label in labels {
        let pos = label.position;
        let r = label.color[0] as f32 / 255.0;
        let g = label.color[1] as f32 / 255.0;
        let b = label.color[2] as f32 / 255.0;
        let lum = 0.299 * r + 0.587 * g + 0.114 * b;
        let v = if lum > 0.5 { 0.0_f32 } else { 1.0_f32 };
        let color = [v, v, v, 1.0_f32];
        for (ci, ch) in label.text.chars().enumerate() {
            let code = ch as u32;
            let idx = if code >= 32 && code < 128 { code - 32 } else { 0 } as usize;
            let col = (idx % 16) as f32;
            let row = (idx / 16) as f32;
            let u0 = col / ATLAS_COLS;
            let u1 = (col + 1.0) / ATLAS_COLS;
            let v0 = row / ATLAS_ROWS;         // top of cell in atlas (lower v)
            let v1 = (row + 1.0) / ATLAS_ROWS; // bottom of cell in atlas (higher v)
            let text_w = label.text.len() as f32 * CHAR_W;
            let x0 = -text_w * 0.5 + ci as f32 * CHAR_W;
            let x1 = x0 + CHAR_W;
            let y0 = -CHAR_H * 0.5 + GLYPH_Y_OFFSET;
            let y1 =  CHAR_H * 0.5 + GLYPH_Y_OFFSET;
            // y1 (screen-top) maps to v0 (atlas-top); y0 (screen-bot) maps to v1 (atlas-bot)
            for &[px, py, pu, pv] in &[
                [x0, y0, u0, v1],
                [x1, y0, u1, v1],
                [x1, y1, u1, v0],
                [x0, y0, u0, v1],
                [x1, y1, u1, v0],
                [x0, y1, u0, v0],
            ] {
                verts.push(GlyphVertex {
                    world_pos: pos,
                    glyph_px:  [px, py],
                    glyph_uv:  [pu, pv],
                    color,
                });
            }
        }
    }
    verts
}

// ── font atlas (Canvas 2D → wgpu texture) ─────────────────────────────────────

/// Render printable ASCII chars to a Canvas 2D element and upload as a
/// Rgba8Unorm wgpu texture.  Returns the view and sampler.
pub fn create_font_atlas(
    device: &wgpu::Device,
    queue:  &wgpu::Queue,
) -> (wgpu::TextureView, wgpu::Sampler) {
    let window   = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    let canvas = document
        .create_element("canvas").unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    canvas.set_width(ATLAS_WIDTH);
    canvas.set_height(ATLAS_HEIGHT);

    let ctx = canvas
        .get_context("2d").unwrap().unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    ctx.set_fill_style_str("white");
    ctx.set_font("bold 14px monospace");
    ctx.set_text_baseline("top");

    for i in 0_u32..96 {
        let ch = char::from_u32(32 + i).unwrap_or(' ');
        let col = (i % 16) as f64 * CHAR_W as f64;
        let row = (i / 16) as f64 * CHAR_H as f64;
        let _ = ctx.fill_text(&ch.to_string(), col, row);
    }

    let image_data = ctx.get_image_data(
        0.0, 0.0,
        ATLAS_WIDTH as f64, ATLAS_HEIGHT as f64,
    ).unwrap();
    let pixels: Vec<u8> = image_data.data().0;

    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("font_atlas"),
        size: wgpu::Extent3d {
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });

    queue.write_texture(
        tex.as_image_copy(),
        &pixels,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(ATLAS_WIDTH * 4),
            rows_per_image: None,
        },
        wgpu::Extent3d {
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
            depth_or_array_layers: 1,
        },
    );

    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("font_sampler"),
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    (view, sampler)
}

