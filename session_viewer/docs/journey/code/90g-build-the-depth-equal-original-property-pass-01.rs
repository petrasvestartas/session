        stencil: Default::default(), bias: Default::default() }
}

pub fn state_for_ambient()->wgpu::DepthStencilState{state(false,wgpu::CompareFunction::Equal)}
