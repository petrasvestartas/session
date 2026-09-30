use super::*;
use crate::app::clipping::{Mode, clip_plane, plane_from};
use crate::engine::gpu::Instance;
use session_rust::{Point, Vector};

/// A camera at `eye` looking at `target`, relative to `anchor`, in scene units; reverse depth.
pub(super) fn view(
    eye: [f64; 3],
    target: [f64; 3],
    anchor: [f64; 3],
    perspective: bool,
) -> Xform {
    let at = |p: [f64; 3]| Point::new(p[0] - anchor[0], p[1] - anchor[1], p[2] - anchor[2]);
    let d = [target[0] - eye[0], target[1] - eye[1], target[2] - eye[2]];
    let distance = dot(d, d).sqrt();
    // straight up or down: y is up on screen
    let up = if d[0].abs() + d[1].abs() < 1e-9 * distance {
        Vector::new(0.0, 1.0, 0.0)
    } else {
        Vector::new(0.0, 0.0, 1.0)
    };
    let look = Xform::look_at_right_handed(&at(eye), &at(target), &up);
    let projection = if perspective {
        Xform::perspective(60f64.to_radians(), 1.0, distance * 10.0, distance * 1e-3)
    } else {
        let h = distance * 30f64.to_radians().tan();
        Xform::orthographic(-h, h, -h, h, distance * 3.0, -distance * 3.0)
    };
    &projection * &look
}

/// The uniform for one plane under a camera.
fn uniform_for(plane: &ClipPlane, view_proj: &Xform, anchor: [f64; 3]) -> ClipUniform {
    clip_uniform(
        std::slice::from_ref(plane),
        0,
        &ClipView {
            view_proj,
            anchor,
            height: 512,
            pixel_scale: 1.0,
            samples: 4,
        },
    )
}

/// The shader declares ClipUniform with the Rust fields at the Rust offsets, and the same flags.
#[test]
fn clip_uniform_mirror() {
    use std::mem::{offset_of, size_of};
    let source = format!(
        "@group(1) @binding(1) var<uniform> clipping: ClipUniform;\n{}",
        crate::engine::pipelines::CLIP
    );
    let module = naga::front::wgsl::parse_str(&source)
        .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::default(),
    )
    .validate(&module)
    .unwrap_or_else(|error| panic!("{}", error.emit_to_string(&source)));
    let (_, ty) = module
        .types
        .iter()
        .find(|(_, ty)| ty.name.as_deref() == Some("ClipUniform"))
        .expect("ClipUniform in the shader");
    let naga::TypeInner::Struct { members, span } = &ty.inner else {
        panic!("ClipUniform is not a struct")
    };
    let offsets: Vec<u32> = members.iter().map(|member| member.offset).collect();
    let pad = offset_of!(ClipUniform, pad) as u32;
    let rust = [
        offset_of!(ClipUniform, planes),
        offset_of!(ClipUniform, screen),
        offset_of!(ClipUniform, hatch),
        offset_of!(ClipUniform, hatch_w),
        offset_of!(ClipUniform, sides),
        offset_of!(ClipUniform, count),
        offset_of!(ClipUniform, samples),
        offset_of!(ClipUniform, fill),
        offset_of!(ClipUniform, spacing),
        offset_of!(ClipUniform, width),
        offset_of!(ClipUniform, outline),
    ]
    .map(|offset| offset as u32);
    assert_eq!(offsets[..11], rust);
    assert_eq!(offsets[11..], [pad, pad + 4]);
    assert_eq!(*span as usize, size_of::<ClipUniform>());

    for (name, bit) in [
        ("FLAG_CLIPPING_PLANE", Instance::FLAG_CLIPPING_PLANE),
        ("FLAG_CLOSED", Instance::FLAG_CLOSED),
        ("FLAG_INWARD", Instance::FLAG_INWARD),
    ] {
        assert!(
            crate::engine::pipelines::CLIP.contains(&format!("const {name}: u32 = {bit}u;")),
            "{name} matches Instance::{name}"
        );
    }
}

/// Fragment stages never touch the object rows and vertex stages never touch fragment-only
/// tables: the layouts show each binding to those stages only, and WebGPU rejects the pipeline.
#[test]
fn every_stage_stays_within_its_bindings() {
    use crate::engine::pipelines::{scene_source, shared};
    let scene = |source: &str| shared(&scene_source(source));
    // (group, binding) of the storage each stage may not use, per shader
    let triangle: &[((u32, u32), naga::ShaderStage)] = &[
        ((2, 0), naga::ShaderStage::Fragment),
        ((2, 1), naga::ShaderStage::Fragment),
        ((3, 0), naga::ShaderStage::Fragment),
        ((3, 1), naga::ShaderStage::Fragment),
        ((3, 2), naga::ShaderStage::Fragment),
        ((3, 3), naga::ShaderStage::Fragment),
        ((3, 4), naga::ShaderStage::Fragment),
        ((3, 5), naga::ShaderStage::Vertex),
    ];
    let rows: &[((u32, u32), naga::ShaderStage)] = &[
        ((2, 0), naga::ShaderStage::Fragment),
        ((2, 1), naga::ShaderStage::Fragment),
        ((3, 0), naga::ShaderStage::Vertex),
        ((3, 1), naga::ShaderStage::Vertex),
        ((3, 5), naga::ShaderStage::Vertex),
    ];
    let text_outline = shader!("text_outline.wgsl");

    for (name, source, forbidden) in [
        ("triangle.wgsl", scene(TRIANGLE), triangle),
        ("text_outline.wgsl", scene(text_outline), rows),
        ("cap.wgsl 1x", scene(&cap_source(1)), rows),
        ("cap.wgsl 4x", scene(&cap_source(4)), rows),
    ] {
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
        let info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::default()
                | naga::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,
        )
        .validate(&module)
        .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));

        for (index, entry) in module.entry_points.iter().enumerate() {
            let uses = info.get_entry_point(index);

            for (handle, global) in module.global_variables.iter() {
                let Some(binding) = &global.binding else {
                    continue;
                };
                let used = !uses[handle].is_empty();
                let banned = forbidden.iter().any(|(at, stage)| {
                    *at == (binding.group, binding.binding) && *stage == entry.stage
                });
                assert!(
                    !(used && banned),
                    "{name}: {} uses {:?} at group {} binding {}",
                    entry.name,
                    global.name,
                    binding.group,
                    binding.binding
                );
            }
        }
    }
}

/// Only a box the plane passes through can hold a section; touching counts, within rounding.
#[test]
fn a_plane_crosses_the_boxes_it_passes_through() {
    let plane = clip_plane(
        &plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let tilted = clip_plane(
        &plane_from(Mode::Normal, &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let block = |z: f64| AABB::new(0.0, 0.0, z, 10.0, 10.0, 10.0);
    let origin = [0.0; 3];
    assert!(plane.crosses(&block(45.0), origin));
    assert!(plane.crosses(&block(60.0), origin), "touching from above");
    assert!(!plane.crosses(&block(61.0), origin));
    assert!(!plane.crosses(&block(-100.0), origin));
    assert!(tilted.crosses(&block(0.0), origin));
    assert!(!tilted.crosses(&AABB::new(40.0, 40.0, 40.0, 5.0, 5.0, 5.0), origin));

    // a face on the plane whose box rounds a hair short of it still counts
    let bottom = clip_plane(
        &plane_from(Mode::Normal, &[[0.0, 0.0, 5.9], [0.0, 0.0, -4.1]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let mut b = AABB::empty();
    b.union_with_point(13.7, -21.3, 5.9 + 1e-9);
    b.union_with_point(113.7, 58.7, 65.9);
    assert!(bottom.crosses(&b, [63.7, 18.7, 35.9]));
}

/// No plane: an all-zero uniform, written once and never again.
#[test]
fn no_plane_is_all_zero() {
    let eye = view([0.0, -500.0, 300.0], [0.0; 3], [0.0; 3], true);
    let u = clip_uniform(
        &[],
        1,
        &ClipView {
            view_proj: &eye,
            anchor: [0.0; 3],
            height: 512,
            pixel_scale: 2.0,
            samples: 4,
        },
    );
    assert_eq!(u, ClipUniform::default());
}

/// Far from the origin the planes relative to the anchor still cut where the world planes do.
#[test]
fn anchored_planes_match_the_world() {
    let anchor = [1.0e6, -3.0e5, 2.0e3];
    let origin = [anchor[0] + 3.0, anchor[1] - 2.0, anchor[2] + 1.0];
    let normal = [origin[0] + 1.0, origin[1] + 2.0, origin[2] + 2.0];
    let plane = plane_from(Mode::Normal, &[origin, normal], 100.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let camera = view(
        [anchor[0] + 400.0, anchor[1] - 900.0, anchor[2] + 500.0],
        anchor,
        anchor,
        true,
    );
    let u = uniform_for(&clip, &camera, anchor);

    for i in 0..50 {
        let t = f64::from(i);
        let p = [
            anchor[0] + (t * 7.3).sin() * 50.0,
            anchor[1] + (t * 3.1).cos() * 50.0,
            anchor[2] + (t * 1.7).sin() * 50.0,
        ];
        let local = [
            (p[0] - anchor[0]) as f32,
            (p[1] - anchor[1]) as f32,
            (p[2] - anchor[2]) as f32,
        ];
        let k = u.planes[0];
        let gpu = k[0] * local[0] + k[1] * local[1] + k[2] * local[2] + k[3];
        assert!(
            (f64::from(gpu) - clip.distance(p)).abs() < 1e-3,
            "point {i}"
        );
    }
}

/// The clip-space plane is zero on the plane, positive on the kept side, in both projections.
#[test]
fn screen_planes_vanish_on_the_plane() {
    let plane = plane_from(
        Mode::Normal,
        &[[10.0, 20.0, 30.0], [11.0, 21.0, 32.0]],
        100.0,
    )
    .unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let anchor = [5.0, 5.0, 5.0];

    for perspective in [true, false] {
        let camera = view(
            [300.0, -400.0, 250.0],
            [10.0, 20.0, 30.0],
            anchor,
            perspective,
        );
        let u = uniform_for(&clip, &camera, anchor);
        let s = u.screen[0].map(f64::from);
        let ndc = |p: [f64; 3]| {
            camera.transform_point(&Point::new(
                p[0] - anchor[0],
                p[1] - anchor[1],
                p[2] - anchor[2],
            ))
        };
        let value = |p: [f64; 3]| {
            let q = ndc(p);
            s[0] * q[0] + s[1] * q[1] + s[2] * q[2] + s[3]
        };
        let x = array(&plane.x_axis());
        let y = array(&plane.y_axis());

        for (a, b) in [(0.0, 0.0), (0.3, -0.7), (-0.9, 0.2)] {
            let on = [
                10.0 + a * x[0] + b * y[0],
                20.0 + a * x[1] + b * y[1],
                30.0 + a * x[2] + b * y[2],
            ];
            assert!(
                value(on).abs() < 1e-4,
                "on the plane, perspective {perspective}"
            );
        }

        assert!(
            value([10.0, 20.0, 29.0]) > 0.0,
            "kept below the normal point"
        );
        assert!(
            value([10.0, 20.0, 31.0]) < 0.0,
            "cut toward the normal point"
        );
    }
}

/// The hatch coordinate over the screen is the scene distance across the lines, up to its period.
#[test]
fn hatch_coordinate_follows_the_plane() {
    let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 50.0]], 200.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let anchor = [30.0, -10.0, 0.0];
    let camera = view([250.0, -300.0, 400.0], [0.0, 0.0, 50.0], anchor, true);
    let inverse = camera.inverse().unwrap();
    let u = uniform_for(&clip, &camera, anchor);
    let s = u.screen[0].map(f64::from);
    // the scene point under canvas point (x, y), and the hatch coordinate there
    let point = |x: f64, y: f64| {
        let z = -(s[0] * x + s[1] * y + s[3]) / s[2];
        let p = inverse.transform_point(&Point::new(x, y, z));
        [p[0] + anchor[0], p[1] + anchor[1], p[2] + anchor[2]]
    };
    let coordinate = |x: f64, y: f64| {
        let h = [x, y, 1.0];
        let n: f64 = (0..3).map(|c| f64::from(u.hatch[0][c]) * h[c]).sum();
        let w: f64 = (0..3).map(|c| f64::from(u.hatch_w[0][c]) * h[c]).sum();
        n / w
    };
    let a = point(-0.2, 0.1);
    let b = point(0.3, -0.25);
    assert!((a[2] - 50.0).abs() < 1e-3 && (b[2] - 50.0).abs() < 1e-3);
    let across = |p: [f64; 3]| dot(clip.hatch, p);
    let expected = across(b) - across(a);
    let found = coordinate(0.3, -0.25) - coordinate(-0.2, 0.1);
    assert!((expected - found).abs() < 1e-2, "{expected} vs {found}");
}

/// A real millimetre camera, perspective and ortho, far from the origin: the planes still reach
/// the shaders, and a point on the plane lands on the clip-space plane.
#[test]
fn millimetre_cameras_keep_their_planes() {
    use crate::camera::Camera;
    use session_rust::AABB;

    let plane = plane_from(Mode::Xy, &[[2.0e5, -1.0e5, 839.0]], 8000.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();
    let mut bounds = AABB::empty();
    bounds.union_with_point(2.0e5 - 7000.0, -1.0e5 - 4000.0, -1500.0);
    bounds.union_with_point(2.0e5 + 7000.0, -1.0e5 + 4000.0, 3000.0);

    for ortho in [false, true] {
        let mut camera = Camera::new();
        camera.fit(&bounds, 1.5);

        if ortho {
            camera.toggle_projection();
        }

        let origin = camera.origin();
        let anchor = [origin[0], origin[1], origin[2]];
        let view_proj = camera.view_proj_anchored(1.5, &origin);
        let u = uniform_for(&clip, &view_proj, anchor);
        assert_eq!(u.count, 1, "ortho {ortho}: the plane reaches the shaders");
        let s = u.screen[0].map(f64::from);
        let q = view_proj.transform_point(&Point::new(
            2.0e5 + 1000.0 - anchor[0],
            -1.0e5 - 500.0 - anchor[1],
            839.0 - anchor[2],
        ));
        // clip-space distance to the plane: the f32 coefficients round at their own scale
        let value = (s[0] * q[0] + s[1] * q[1] + s[2] * q[2] + s[3])
            / dot([s[0], s[1], s[2]], [s[0], s[1], s[2]]).sqrt();
        assert!(value.abs() < 1e-6, "ortho {ortho}: {value}, plane {s:?}");
    }
}

/// The eye side flips as the eye crosses the plane, in perspective and in ortho.
#[test]
fn eye_sides_follow_the_eye() {
    let plane = plane_from(Mode::Xy, &[[0.0, 0.0, 0.0]], 100.0).unwrap();
    let clip = clip_plane(&plane, &Xform::identity()).unwrap();

    for (eye, side) in [([0.0, -300.0, 200.0], -1.0), ([0.0, -300.0, -200.0], 1.0)] {
        for perspective in [true, false] {
            let camera = view(eye, [0.0; 3], [0.0; 3], perspective);
            let u = uniform_for(&clip, &camera, [0.0; 3]);
            assert_eq!(u.sides[0][0], side, "{eye:?}, perspective {perspective}");
        }
    }
}

/// Hatch spacing and line width follow the device scale.
#[test]
fn hatch_spacing_follows_the_device_scale() {
    let plane = clip_plane(
        &plane_from(Mode::Xy, &[[0.0; 3]], 10.0).unwrap(),
        &Xform::identity(),
    )
    .unwrap();
    let camera = view([0.0, -300.0, 200.0], [0.0; 3], [0.0; 3], true);
    let u = clip_uniform(
        &[plane],
        0,
        &ClipView {
            view_proj: &camera,
            anchor: [0.0; 3],
            height: 512,
            pixel_scale: 2.0,
            samples: 1,
        },
    );
    assert_eq!(
        (u.count, u.samples, u.spacing, u.width, u.outline),
        (1, 1, 16.0, 2.0, 4.0)
    );
}

/// A point or vector as three numbers.
fn array<T: std::ops::Index<usize, Output = f64>>(v: &T) -> [f64; 3] {
    [v[0], v[1], v[2]]
}
