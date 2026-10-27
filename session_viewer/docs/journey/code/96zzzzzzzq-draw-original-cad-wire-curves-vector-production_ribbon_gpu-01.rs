use crate::production_joined::{self as ribbon, Rows};
use std::cell::RefCell;
use wgpu::util::DeviceExt;
use wgpu::{BufferBindingType, ShaderStages};

pub(crate) fn storage(binding: u32, stages: ShaderStages) -> wgpu::BindGroupLayoutEntry {
    crate::tile_pipelines::entry(
        binding,
        stages,
