pub(crate) fn attributes(stride:u64)->&'static [wgpu::VertexAttribute] {
 const STROKE:[wgpu::VertexAttribute;4]=ATTRIBUTE_0;
 const CHAIN:[wgpu::VertexAttribute;7]=ATTRIBUTE_1;
 const MARKER:[wgpu::VertexAttribute;3]=ATTRIBUTE_2;
 const GUIDE:[wgpu::VertexAttribute;5]=wgpu::vertex_attr_array![0=>Float32x3,1=>Float32x3,2=>Float32x4,3=>Float32,4=>Uint32];
 match stride {44=>&STROKE,72=>&CHAIN,32=>&MARKER,48=>&GUIDE,_=>unreachable!()}
}

pub struct Lane {