
#[cfg(test)]
mod walk_tests {
    use super::tests::{reach, thinnest, tip_offset};
    use super::*;

    /// Curve heads: each tip lands on its curve's end point and no ink, curve or head, reaches past it.
    #[test]
    fn curve_heads_land_on_their_ends() {
        use crate::app::walk::{Walk, WalkCx, walk_geometry};
        use crate::camera::{Camera, View};
        use crate::engine::gpu::{FrameInput, Gpu, ObjectRow};
        use session_rust::{AABB, Arrowhead, Geometry, NurbsCurve, Point, Polyline, Xform};
        use std::rc::Rc;

        const SIZE: u32 = 256;
        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(SIZE, SIZE)) else {
            eprintln!("no GPU adapter; skipped");
            return;
        };
        gpu.view.show_grid = false;
        let mut bounds = AABB::empty();
        bounds.union_with_point(-100.0, -100.0, 0.0);
        bounds.union_with_point(100.0, 100.0, 0.0);
        let p = |x: f64, y: f64| Point::new(x, y, 0.0);
        let polyline = |head: Arrowhead, width: f64| {
            let mut pl = Polyline::new(vec![
                p(-70.0, -30.0),
                p(-20.0, 30.0),
                p(20.0, -30.0),
                p(70.0, 10.0),
            ]);
            pl.arrowhead = head;
            pl.width = width;
            Geometry::Polyline(Rc::new(pl))
        };
        let curve = |head: Arrowhead, width: f64| {
            let cvs = [p(-70.0, 0.0), p(-30.0, 60.0), p(30.0, -60.0), p(70.0, 0.0)];
            let mut c = NurbsCurve::create(false, 3, &cvs);
            c.arrowhead = head;
            c.width = width;
            Geometry::NurbsCurve(Rc::new(c))
        };
        // a last segment shorter than its head, and a curve turning sharply into its end
        let mut short = Polyline::new(vec![
            p(-70.0, -30.0),
            p(20.0, -30.0),
            p(70.0, 10.0),
            p(73.0, 10.0),
        ]);
        short.arrowhead = Arrowhead::END;
        let short = Geometry::Polyline(Rc::new(short));
        let cvs = [p(-70.0, 0.0), p(-30.0, 60.0), p(60.0, 0.0), p(70.0, -40.0)];
        let mut sharp = NurbsCurve::create(false, 3, &cvs);
        sharp.arrowhead = Arrowhead::END;
        let sharp = Geometry::NurbsCurve(Rc::new(sharp));
        // name, curve, selected
        let cases = [
            ("polyline both", polyline(Arrowhead::BOTH, 1.0), false),
            ("polyline thick", polyline(Arrowhead::END, 6.0), false),
            ("polyline selected", polyline(Arrowhead::START, 1.0), true),
            ("curve end", curve(Arrowhead::END, 1.0), false),
            ("curve both thick", curve(Arrowhead::BOTH, 4.0), true),
            ("polyline short end", short, false),
            ("curve sharp end", sharp, false),
        ];
        let (mut worst_tip, mut worst_far, mut worst_flat): (f64, f64, f64) = (0.0, 0.0, 0.0);
        // the default view, top in both projections, iso
        let views = [
            ("default", None, true),
            ("top", Some(View::Top), false),
            ("top", Some(View::Top), true),
            ("iso", Some(View::Iso), false),
        ];

        for (shown, view, perspective) in views {
            let mut camera = Camera::new();

            if let Some(view) = view {
                camera.set_view(view);
            }

            camera.perspective = perspective;
            camera.fit(&bounds, 1.0);
            let px = |q: [f32; 3]| {
                let at = Point::new(f64::from(q[0]), f64::from(q[1]), f64::from(q[2]));
                let m = camera.view_proj_anchored(1.0, &at).m;
                let s = f64::from(SIZE);
                [
                    (m[12] / m[15] * 0.5 + 0.5) * s,
                    (0.5 - m[13] / m[15] * 0.5) * s,
                ]
            };

            for (dpr, samples) in [(1.0, 1), (1.0, 4), (2.0, 1), (2.0, 4)] {
                gpu.logical_size = [f64::from(SIZE) / dpr; 2];
                gpu.view.msaa_forced = Some(samples);
                gpu.resize(SIZE, SIZE);

                for (name, geometry, selected) in &cases {
                    gpu.reset();
                    let mut up = Upload::default();
                    let cx = WalkCx {
                        vert_base: 0,
                        cloud_px: 0.0,
                        row: 0,
                        attributes: false,
                    };
                    let walked = walk_geometry(&mut Walk::of(&mut up), &cx, geometry);
                    let mut object = ObjectRow::new(Xform::identity(), walked.flags);
                    object.bounds = bounds;
                    up.obj.rows.push(object);
                    up.bounds = bounds;
                    let heads = up
                        .lanes
                        .get::<VectorRows>()
                        .expect("head rows")
                        .rows
                        .clone();
                    gpu.set_scene(&up);
                    let rebase = gpu.rebase_anchor(&camera.origin(), camera.distance_world(), 0.0);
                    let input = FrameInput {
                        view_proj: camera.view_proj_anchored(1.0, &rebase.anchor),
                        clear: wgpu::Color::WHITE,
                        now_ms: 0.0,
                    };
                    gpu.set_hidden(0, true);
                    let paper = gpu.render_offscreen(&input);
                    gpu.set_hidden(0, false);
                    gpu.set_selected(0, *selected);
                    let rgba = gpu.render_offscreen(&input);
                    let label = format!(
                        "{shown}, perspective {perspective}, dpr {dpr}, msaa {samples}, {name}"
                    );

                    // only the ink near `tip`: the rest of the curve may lie past it on screen
                    let near = |tip: [f64; 2]| -> Vec<u8> {
                        rgba.chunks_exact(4)
                            .zip(paper.chunks_exact(4))
                            .enumerate()
                            .flat_map(|(i, (ink, blank))| {
                                let x = (i % SIZE as usize) as f64 + 0.5 - tip[0];
                                let y = (i / SIZE as usize) as f64 + 0.5 - tip[1];
                                if x * x + y * y < 40.0 * 40.0 {
                                    ink
                                } else {
                                    blank
                                }
                                .to_vec()
                            })
                            .collect()
                    };

                    for head in &heads {
                        let (aim, tip) = (px(head.start), px(head.end));
                        let len = ((tip[0] - aim[0]).powi(2) + (tip[1] - aim[1]).powi(2)).sqrt();
                        let dir = [(tip[0] - aim[0]) / len, (tip[1] - aim[1]) / len];
                        let [past, _] = reach(&near(tip), &paper, SIZE as usize, aim, tip);
                        let offset = tip_offset(&rgba, &paper, SIZE as usize, tip, dir, 2.5);
                        assert!(offset.abs() <= 0.5, "{label}: tip off by {offset:+.3}");
                        assert!(
                            past.far <= 0.5,
                            "{label}: ink {:+.3} past the tip",
                            past.far
                        );
                        worst_tip = worst_tip.max(offset.abs());
                        worst_far = worst_far.max(past.far);

                        // a thick curve runs on under its head's base: no seam
                        if *name == "polyline thick" {
                            let least =
                                thinnest(&rgba, &paper, SIZE as usize, aim, tip, 1.5, len - 3.0);
                            assert!(least >= 0.9, "{label}: center line only {least:.3} inked");
                        }
                    }

                    // a bare end of a headed curve is flat on its point, as a vector's
                    let ribbons = &up.seg.ribbons;
                    let (first, last) = (ribbons[0], ribbons[ribbons.len() - 1]);

                    for (aim, end) in [(first.p1, first.p0), (last.p0, last.p1)] {
                        if heads.iter().any(|head| head.end == end) {
                            continue;
                        }

                        let (aim, tip) = (px(aim), px(end));
                        let [past, _] = reach(&near(tip), &paper, SIZE as usize, aim, tip);
                        assert!(
                            past.edge.abs() <= 0.5,
                            "{label}: flat edge {:+.3}",
                            past.edge
                        );
                        assert!(
                            past.far <= 1.0,
                            "{label}: ink {:+.3} past a flat end",
                            past.far
                        );
                        worst_flat = worst_flat.max(past.far);
                    }
                }
            }
        }

        eprintln!(
            "curve heads: worst tip error {worst_tip:.3}, ink past a tip {worst_far:.3}, past a flat end {worst_flat:.3}"
        );
    }
}
