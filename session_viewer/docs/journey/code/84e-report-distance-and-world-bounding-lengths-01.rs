        out
    }
}
pub fn selected_bounds(scene: &Scene, ids: &[ObjectId]) -> Result<crate::bounds::Bounds, String> {
    let mut out: Option<crate::bounds::Bounds> = None;
    for id in ids {
        let Some(bounds) = scene.selected_bounds(*id) else {
            continue;
        };
        if let Some(out) = &mut out {
            out.include(bounds.min);
            out.include(bounds.max);
        } else {
            out = Some(bounds);
        }
    }
    out.ok_or_else(|| "Select an object with a bounding box".into())
}
pub fn distance(a: [f64; 3], b: [f64; 3], unit: Unit) -> (Mark, String) {
    let (a, b) = (Point::new(a[0], a[1], a[2]), Point::new(b[0], b[1], b[2]));
    let delta = &b - &a;
    let text = crate::measure_math::to_text;
    let value = format!("{} {}", text(delta.magnitude()), unit.suffix(1));
    let message = format!(
        "Distance {value} · dx {} · dy {} · dz {}",
        text(delta[0]),
        text(delta[1]),
        text(delta[2])
    );
    (
        Mark {
            points: vec![[a[0], a[1], a[2]], [b[0], b[1], b[2]]],
            label: value,
        },
        message,
    )
}
#[derive(Clone, Copy)]
pub enum Kind {
    Length,
    Area,
    Volume,
    BoundingLengths,
}
