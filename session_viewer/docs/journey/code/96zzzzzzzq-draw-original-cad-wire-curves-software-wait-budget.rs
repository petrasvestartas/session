    let seconds = std::env::var("VIEWER_GPU_WAIT_SECONDS").map_or(
        if cfg!(windows) { 30 } else { 5 },
        |value| value.parse::<u64>().ok().filter(|seconds| (1..=120).contains(seconds))
            .expect("VIEWER_GPU_WAIT_SECONDS must be an integer from 1 to 120"),
    );
