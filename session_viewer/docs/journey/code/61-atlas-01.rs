use glyphon::{Cache,Color,Resolution,SwashCache,TextArea,TextAtlas,TextBounds,TextRenderer,Viewport};
use crate::text::{TextDocument,TextPlacement};

pub struct Lane {
    pub document:TextDocument,
    cache:Cache,
    atlas:TextAtlas,
    viewport:Viewport,
    raster:SwashCache,
    renderer:TextRenderer,
    prepared:Option<(u64,u64,[u32;2],u32)>,
    pub preparations:u64,
    pub skips:u64,
    pub resets:u64,
    pub format:wgpu::TextureFormat,
}

impl Lane {
    pub fn new(device:&wgpu::Device,queue:&wgpu::Queue,format:wgpu::TextureFormat)->Self {
        let cache=Cache::new(device);let atlas=TextAtlas::with_color_mode(device,queue,&cache,format,glyphon::ColorMode::Accurate);let viewport=Viewport::new(device,&cache);let mut atlas=atlas;
        let renderer=TextRenderer::new(&mut atlas,device,Default::default(),None);
        Self {document:TextDocument::new(),cache,atlas,viewport,raster:SwashCache::new(),renderer,prepared:None,preparations:0,skips:0,resets:0,format}
    }
}
