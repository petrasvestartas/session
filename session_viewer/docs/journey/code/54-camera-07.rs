        _ => return None,
    };
    let pixels = (delta * unit).clamp(-1_000.0, 1_000.0);
    Some(Action::Zoom(0.9_f64.powf(pixels / 100.0) as f32))
}
