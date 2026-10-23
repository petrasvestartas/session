pub(crate) fn wait(r: &Renderer) {
    let seconds = if cfg!(windows) { 30 } else { 5 };
    r.device.poll(wgpu::PollType::Wait {
        submission_index: None,
        timeout: Some(std::time::Duration::from_secs(seconds)),
    }).unwrap();
}
