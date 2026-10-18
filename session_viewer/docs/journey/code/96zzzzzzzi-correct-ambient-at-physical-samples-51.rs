uffers.iter().map(wgpu::Buffer::size).sum::<u64>()),1,self.preparations,self.writes,self.passes]}
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ambient_sample_inspect.rs"]
mod sample_inspect;
