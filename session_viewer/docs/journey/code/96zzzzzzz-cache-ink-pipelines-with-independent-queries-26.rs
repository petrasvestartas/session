pass.set_vertex_buffer(0,vertices.slice(..));pass.set_vertex_buffer(1,ids.slice(..));pass.set_vertex_buffer(2,flags.slice(..));pass.draw(0..verts,0..count);}
}

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="finite_sample_tests.rs"]
mod sample_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="ink_pixels_tests.rs"]
mod pixels_tests;
