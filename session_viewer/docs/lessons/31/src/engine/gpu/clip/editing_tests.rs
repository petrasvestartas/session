use super::tests::view;

#[cfg(not(target_arch = "wasm32"))]
mod gpu {
    use super::view;
    use crate::app::clipping::{Mode, clip_plane, plane_from};
    use crate::app::scene::{FileDoc, Scene};
    use crate::engine::gpu::clip::ClipPlane;
    use crate::engine::gpu::{FrameInput, Gpu, Instance};
    use session_rust::{Color, Mesh, Point, Session, Xform};
    use std::rc::Rc;

    /// Canvas side, px.
    const SIZE: usize = 256;

    /// Face colors as the canvas stores them.
    const GREY: [u8; 3] = [188, 188, 188];
    const RED: [u8; 3] = [255, 0, 0];
    const BLUE: [u8; 3] = [0, 0, 255];
    const YELLOW: [u8; 3] = [255, 255, 0];

    /// A box from `lo` to `hi` in one color, faces outward; `top` false leaves its top open.
    fn block(lo: [f64; 3], hi: [f64; 3], color: Color, top: bool) -> Mesh {
        // corners as Mesh::create_box orders them: bottom then top, counterclockwise
        let corners = (0..8)
            .map(|i: usize| {
                let x = if i % 4 == 1 || i % 4 == 2 {
                    hi[0]
                } else {
                    lo[0]
                };
                let y = if i % 4 >= 2 { hi[1] } else { lo[1] };
                let z = if i >= 4 { hi[2] } else { lo[2] };
                Point::new(x, y, z)
            })
            .collect();
        let mut faces = vec![
            vec![0, 3, 2, 1],
            vec![0, 1, 5, 4],
            vec![2, 3, 7, 6],
            vec![0, 4, 7, 3],
            vec![1, 2, 6, 5],
        ];

        if top {
            faces.push(vec![4, 5, 6, 7]);
        }

        let mut mesh = Mesh::from_vertices_and_faces(corners, faces);
        mesh.set_objectcolor(color);
        mesh
    }

    /// The meshes as one document on a headless GPU, flat faces, no edges; None without an adapter.
    fn solids(meshes: Vec<Mesh>) -> Option<(Gpu, Scene)> {
        crate::app::clipping::verify_solids();
        let mut gpu = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32)).ok()?;
        gpu.pass_mut::<super::super::Clip>().fill = 0; // existing hatch regressions explicitly choose Hatch
        gpu.view.show_grid = false;
        gpu.view.show_mesh_edges = false;
        gpu.view.show_points = false;
        gpu.view.lit = false;
        let mut session = Session::new("solids");

        for mesh in meshes {
            session.add_mesh(mesh, None);
        }

        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "solids".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        Some((gpu, scene))
    }

    /// Cut with `planes` and find the solids they cross, as a frame of the viewer does.
    fn cut(gpu: &mut Gpu, scene: &Scene, planes: &[ClipPlane]) {
        gpu.set_clip_planes(planes);
        gpu.find_solids(|row| scene.solid_faces(row));
    }

    /// The world XY plane through height `z`, everything above cut away.
    fn cut_above(z: f64) -> ClipPlane {
        let plane = plane_from(Mode::Xy, &[[0.0, 0.0, z]], 200.0).unwrap();
        clip_plane(&plane, &Xform::identity()).unwrap()
    }

    /// The frame seen from `eye` toward `target`, and a scene point's pixel in it.
    fn frame(
        gpu: &mut Gpu,
        eye: [f64; 3],
        target: [f64; 3],
        perspective: bool,
    ) -> (FrameInput, impl Fn([f64; 3]) -> (usize, usize) + use<>) {
        let d: f64 = (0..3)
            .map(|i| (target[i] - eye[i]).powi(2))
            .sum::<f64>()
            .sqrt();
        let rebase = gpu.rebase_anchor(&Point::new(target[0], target[1], target[2]), d, 0.0);
        let anchor = [rebase.anchor[0], rebase.anchor[1], rebase.anchor[2]];
        let view_proj = view(eye, target, anchor, perspective);
        let map = view_proj.clone();
        let pixel = move |p: [f64; 3]| {
            let q = map.transform_point(&Point::new(
                p[0] - anchor[0],
                p[1] - anchor[1],
                p[2] - anchor[2],
            ));
            (
                ((q[0] * 0.5 + 0.5) * SIZE as f64) as usize,
                ((0.5 - q[1] * 0.5) * SIZE as f64) as usize,
            )
        };
        let input = FrameInput {
            view_proj,
            clear: wgpu::Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            now_ms: 0.0,
        };
        (input, pixel)
    }

    /// Shares of hatch ink, cap paper and `color` among the pixels within `r` px of `at`.
    fn patch(rgba: &[u8], at: (usize, usize), r: usize, color: [u8; 3]) -> [f64; 3] {
        let mut counts = [0.0; 3];
        let mut total = 0.0_f64;

        for y in at.1.saturating_sub(r)..(at.1 + r + 1).min(SIZE) {
            for x in at.0.saturating_sub(r)..(at.0 + r + 1).min(SIZE) {
                let p = &rgba[(y * SIZE + x) * 4..][..3];
                let luma = p.iter().map(|c| u32::from(*c)).sum::<u32>() / 3;
                let grey = p.iter().max().unwrap() - p.iter().min().unwrap() < 30;
                total += 1.0;
                // sRGB: a line pixel of 0.75 coverage reads 137
                counts[0] += f64::from(u8::from(grey && luma < 170));
                counts[1] += f64::from(u8::from(p.iter().all(|c| *c > 225)));
                counts[2] += f64::from(u8::from(
                    (0..3).all(|k| (i32::from(p[k]) - i32::from(color[k])).abs() < 30),
                ));
            }
        }

        counts.map(|count| count / total.max(1.0))
    }

    /// True for black lines on white paper.
    fn hatched(shares: [f64; 3]) -> bool {
        shares[0] > 0.04 && shares[0] < 0.6 && shares[1] > 0.3
    }

    /// Every sample count and projection, with the frame drawn from `eye` toward `target`.
    fn each_view(
        gpu: &mut Gpu,
        eye: [f64; 3],
        target: [f64; 3],
        projections: &[bool],
        check: impl Fn(&[u8], &dyn Fn([f64; 3]) -> (usize, usize), &str),
    ) {
        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(SIZE as u32, SIZE as u32);

            for &perspective in projections {
                let (input, pixel) = frame(gpu, eye, target, perspective);
                let rgba = gpu.render_offscreen(&input);
                let label = format!("{samples}x, perspective {perspective}");
                check(&rgba, &pixel, &label);
            }
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn sections_default_to_light_grey_with_black_boundaries() {
        let (mut gpu, scene) = solids(vec![block([-50.0; 3], [50.0; 3], Color::blue(), true)])
            .expect("native GPU");
        gpu.pass_mut::<super::super::Clip>().fill = super::super::Clip::new(gpu.target()).fill;
        assert_eq!(gpu.pass::<super::super::Clip>().fill, 1);
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(SIZE as u32, SIZE as u32);
            for dpr in [1.0, 2.0] {
                gpu.logical_size = [SIZE as f64 / dpr; 2];
                for perspective in [false, true] {
                    let (input, pixel) =
                        frame(&mut gpu, [0.0, 0.0, 250.0], [0.0; 3], perspective);
                    let rgba = gpu.render_offscreen(&input);
                    let center = pixel([0.0; 3]);
                    for y in center.1 - 8..=center.1 + 8 {
                        for x in center.0 - 8..=center.0 + 8 {
                            let rgb = &rgba[(y * SIZE + x) * 4..][..3];
                            assert!(
                                rgb.iter().all(|v| (202..=204).contains(v)),
                                "solid light grey, without hatch: {rgb:?}"
                            );
                        }
                    }
                    for edge in [
                        [-50.0, 0.0, 0.0],
                        [50.0, 0.0, 0.0],
                        [0.0, -50.0, 0.0],
                        [0.0, 50.0, 0.0],
                    ] {
                        let at = pixel(edge);
                        let black = (at.1 - 4..=at.1 + 4)
                            .flat_map(|y| {
                                (at.0 - 4..=at.0 + 4).map(move |x| (y * SIZE + x) * 4)
                            })
                            .filter(|at| rgba[*at..*at + 3].iter().all(|v| *v < 16))
                            .count();
                        assert!(
                            black >= 5,
                            "black cut edge: {edge:?}, {samples}x, DPR {dpr}, perspective {perspective}"
                        );
                    }
                    let ids = gpu.render_ids_offscreen(&input);
                    assert_eq!(
                        ids[center.1 * SIZE + center.0][0],
                        1,
                        "the cap still picks its solid"
                    );
                }
            }
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A cut cube seen from the cut side: hatch on the section, the kept top face stays plain,
    /// and the removed half shows the section behind it.
    fn a_cut_cube_caps_its_section_and_nothing_else() {
        let Some((mut gpu, scene)) =
            solids(vec![block([0.0; 3], [100.0; 3], Color::grey(), true)])
        else {
            return;
        };
        assert_ne!(gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED, 0);
        let plane = plane_from(Mode::Normal, &[[50.0, 0.0, 0.0], [40.0, 0.0, 0.0]], 100.0);
        cut(
            &mut gpu,
            &scene,
            &[clip_plane(&plane.unwrap(), &Xform::identity()).unwrap()],
        );
        each_view(
            &mut gpu,
            [-200.0, 50.0, 300.0],
            [50.0, 50.0, 50.0],
            &[true, false],
            |rgba, pixel, label| {
                let cap = patch(rgba, pixel([50.0, 50.0, 50.0]), 8, GREY);
                assert!(hatched(cap), "{label}: the section is hatched: {cap:?}");
                let top = patch(rgba, pixel([75.0, 50.0, 100.0]), 4, GREY);
                assert!(
                    top[2] > 0.95,
                    "{label}: the kept top face is plain: {top:?}"
                );
                let removed = patch(rgba, pixel([10.0, 50.0, 100.0]), 8, GREY);
                assert!(
                    hatched(removed),
                    "{label}: the removed half shows the section: {removed:?}"
                );
            },
        );

        cut(&mut gpu, &scene, &[]);
        let (input, pixel) = frame(&mut gpu, [-200.0, 50.0, 300.0], [50.0, 50.0, 50.0], true);
        let whole = gpu.render_offscreen(&input);
        let top = patch(&whole, pixel([10.0, 50.0, 100.0]), 4, GREY);
        assert!(
            top[2] > 0.95,
            "no plane: the whole top face is back: {top:?}"
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A dowel through a beam, no boolean hole: both sections hatched, the dowel never shows
    /// through the beam's section, and a pick names the solid under the cap.
    fn a_dowel_inside_a_beam_never_shows_through() {
        let Some((mut gpu, scene)) = solids(vec![
            block(
                [-100.0, -50.0, -50.0],
                [100.0, 50.0, 50.0],
                Color::grey(),
                true,
            ),
            block(
                [-10.0, -150.0, -10.0],
                [10.0, 150.0, 10.0],
                Color::red(),
                true,
            ),
        ]) else {
            return;
        };
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        let eye = [150.0, -250.0, 250.0];
        each_view(
            &mut gpu,
            eye,
            [0.0; 3],
            &[true, false],
            |rgba, pixel, label| {
                // the dowel runs through the beam's middle; no red may show anywhere on the beam's cap
                for x in [-80.0, -40.0, -5.0, 0.0, 5.0, 40.0, 80.0] {
                    for y in [-25.0, 0.0, 25.0] {
                        let shares = patch(rgba, pixel([x, y, 0.0]), 6, RED);
                        assert!(
                            shares[2] == 0.0,
                            "{label}: no dowel through the beam at {x},{y}: {shares:?}"
                        );
                        assert!(
                            hatched(shares),
                            "{label}: beam section at {x},{y}: {shares:?}"
                        );
                    }
                }

                let dowel = patch(rgba, pixel([0.0, 120.0, 0.0]), 6, RED);
                assert!(
                    hatched(dowel),
                    "{label}: the dowel's own section: {dowel:?}"
                );
            },
        );

        // picks: the beam where only it holds the cap point, the dowel outside the beam
        gpu.view.msaa_forced = Some(1);
        gpu.resize(SIZE as u32, SIZE as u32);
        let (input, pixel) = frame(&mut gpu, eye, [0.0; 3], true);
        let ids = gpu.render_ids_offscreen(&input);
        let id = |p: [f64; 3]| {
            let (x, y) = pixel(p);
            ids[y * SIZE + x]
        };
        assert_eq!(
            id([15.0, 0.0, 0.0]),
            [1, 0],
            "the beam's cap picks the beam"
        );
        assert_eq!(
            id([60.0, -30.0, 0.0]),
            [1, 0],
            "the beam's cap picks the beam"
        );
        assert_eq!(
            id([0.0, 120.0, 0.0]),
            [2, 0],
            "the dowel's cap picks the dowel"
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Two overlapping boxes: the union is hatched, the overlap too; a selected one's cap turns yellow.
    fn overlapping_boxes_are_capped_as_their_union() {
        let Some((mut gpu, scene)) = solids(vec![
            block(
                [-100.0, -50.0, -50.0],
                [30.0, 50.0, 50.0],
                Color::red(),
                true,
            ),
            block(
                [-30.0, -50.0, -50.0],
                [100.0, 50.0, 50.0],
                Color::blue(),
                true,
            ),
        ]) else {
            return;
        };
        cut(&mut gpu, &scene, &[cut_above(500.0)]);
        assert_eq!(
            gpu.pass::<super::super::Clip>().cap_planes(&gpu.view).1,
            0,
            "a plane above both boxes draws no section pass"
        );
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        assert_eq!(
            gpu.pass::<super::super::Clip>().cap_planes(&gpu.view).1,
            1,
            "the plane through both boxes does"
        );
        each_view(
            &mut gpu,
            [100.0, -250.0, 250.0],
            [0.0; 3],
            &[true, false],
            |rgba, pixel, label| {
                for x in [-70.0, 0.0, 70.0] {
                    let red = patch(rgba, pixel([x, 0.0, 0.0]), 8, RED);
                    let blue = patch(rgba, pixel([x, 0.0, 0.0]), 8, BLUE);
                    assert!(hatched(red), "{label}: section at {x}: {red:?}");
                    assert!(
                        red[2] + blue[2] < 0.01,
                        "{label}: no face shows through at {x}"
                    );
                }
            },
        );

        // the selected box's cap turns yellow where it holds the cap point, the overlap too
        gpu.set_selected(0, true);
        let (input, pixel) = frame(&mut gpu, [100.0, -250.0, 250.0], [0.0; 3], true);
        let rgba = gpu.render_offscreen(&input);
        let yellow = |x: f64| patch(&rgba, pixel([x, 0.0, 0.0]), 8, YELLOW)[2];
        assert!(yellow(-70.0) > 0.3, "selected alone: {}", yellow(-70.0));
        assert!(yellow(0.0) > 0.3, "selected and not: {}", yellow(0.0));
        assert_eq!(yellow(70.0), 0.0, "the other box's cap stays white");
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// An open box is cut but never capped: its inside shows.
    fn an_open_mesh_gets_no_cap() {
        let Some((mut gpu, scene)) =
            solids(vec![block([-50.0; 3], [50.0; 3], Color::grey(), false)])
        else {
            return;
        };
        assert_eq!(gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED, 0);
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        each_view(
            &mut gpu,
            [100.0, -150.0, 250.0],
            [0.0; 3],
            &[true, false],
            |rgba, pixel, label| {
                let inside = patch(rgba, pixel([0.0, 0.0, 0.0]), 8, GREY);
                assert!(
                    !hatched(inside) && inside[0] < 0.01,
                    "{label}: no cap: {inside:?}"
                );
            },
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Seen from the kept side the kept faces hide the cap; from inside the solid it shows.
    fn from_the_kept_side_the_cap_hides_behind_kept_faces() {
        let Some((mut gpu, scene)) =
            solids(vec![block([-50.0; 3], [50.0; 3], Color::grey(), true)])
        else {
            return;
        };
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        each_view(
            &mut gpu,
            [100.0, -150.0, -250.0],
            [0.0; 3],
            &[true, false],
            |rgba, pixel, label| {
                let bottom = patch(rgba, pixel([0.0, 0.0, -50.0]), 8, GREY);
                assert!(
                    bottom[2] > 0.95,
                    "{label}: the kept bottom face: {bottom:?}"
                );
            },
        );
        each_view(
            &mut gpu,
            [10.0, 10.0, -30.0],
            [0.0, 0.0, 10.0],
            &[true],
            |rgba, _pixel, label| {
                let center = patch(rgba, (SIZE / 2, SIZE / 2), 8, GREY);
                assert!(
                    hatched(center),
                    "{label}: inside the solid the cap shows: {center:?}"
                );
            },
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Lines, dots and arrows lose the part a plane cuts away, in color and in the pick.
    fn ink_lanes_lose_the_cut_part() {
        use crate::engine::gpu::vectors::{VectorRow, VectorRows};
        use crate::engine::gpu::{CylinderSegment, GlyphPoint, ObjectRow, Upload};
        use session_rust::AABB;

        let Ok(mut gpu) = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32))
        else {
            return;
        };
        gpu.view.show_grid = false;
        let mut up = Upload::default();
        let mut bounds = AABB::empty();
        bounds.union_with_point(-100.0, -40.0, 0.0);
        bounds.union_with_point(100.0, 40.0, 0.0);

        for _ in 0..3 {
            let mut row = ObjectRow::new(Xform::identity(), 0);
            row.bounds = bounds;
            up.obj.rows.push(row);
        }

        up.bounds = bounds;
        up.seg.ribbons.push(CylinderSegment {
            p0: [-100.0, 30.0, 0.0],
            radius: 3.0,
            p1: [100.0, 30.0, 0.0],
            instance_id: 0,
            color: 0xff00_0000,
            facing: u32::MAX,
        });

        for x in [-60.0, 60.0] {
            up.glyph.dots.push(GlyphPoint {
                center: [x, 0.0, 0.0],
                radius: -6.0,
                color: [0.0, 0.0, 0.0, 1.0],
                instance_id: 1,
                facing: u32::MAX,
                facing_ext: [u32::MAX; 2],
            });
        }

        up.lanes.get_mut::<VectorRows>().rows.push(VectorRow {
            start: [-100.0, -30.0, 0.0],
            radius: 3.0,
            end: [100.0, -30.0, 0.0],
            instance_id: 2,
            color: 0xff00_0000,
            head: 0.0,
            heads: VectorRow::HEAD_END,
            pad: 0,
        });
        gpu.set_scene(&up);
        // ink left and right of x = 0, and pick ids right of it
        let halves = |rgba: &[u8]| {
            let mut ink = [0_i32; 2];

            for (i, p) in rgba.chunks_exact(4).enumerate() {
                if p[..3].iter().any(|c| *c < 230) {
                    ink[usize::from(i % SIZE >= SIZE / 2)] += 1;
                }
            }

            ink
        };
        let right = |ids: &[[u32; 2]]| {
            ids.iter()
                .enumerate()
                .filter(|(i, id)| i % SIZE > SIZE / 2 + 2 && id[0] != 0)
                .count()
        };
        let yz = plane_from(Mode::Yz, &[[0.0; 3]], 200.0).unwrap();

        for samples in [1, 4] {
            gpu.view.msaa_forced = Some(samples);
            gpu.resize(SIZE as u32, SIZE as u32);

            for perspective in [true, false] {
                let (input, _) = frame(&mut gpu, [0.0, 0.0, 250.0], [0.0; 3], perspective);
                gpu.set_clip_planes(&[]);
                let whole = halves(&gpu.render_offscreen(&input));
                let picked = right(&gpu.render_ids_offscreen(&input));
                gpu.set_clip_planes(&[clip_plane(&yz, &Xform::identity()).unwrap()]);
                let cut = halves(&gpu.render_offscreen(&input));
                let label = format!("{samples}x, perspective {perspective}");
                assert!(
                    whole[0] > 150 && whole[1] > 150,
                    "{label}: ink both sides: {whole:?}"
                );
                assert!(
                    (cut[0] - whole[0]).abs() <= 4,
                    "{label}: the kept half is untouched: {cut:?} {whole:?}"
                );
                assert!(cut[1] <= 8, "{label}: the cut half is empty: {cut:?}");
                assert!(picked > 60, "{label}: the right half picks uncut: {picked}");
                let picked = right(&gpu.render_ids_offscreen(&input));
                assert_eq!(picked, 0, "{label}: nothing cut away can be picked");
            }
        }

        // cutting again and stopping again compiles nothing: both ink variants are cached
        let (input, _) = frame(&mut gpu, [0.0, 0.0, 250.0], [0.0; 3], false);
        let compiled = crate::engine::pipelines::created().0;
        gpu.set_clip_planes(&[]);
        let whole = halves(&gpu.render_offscreen(&input));
        gpu.set_clip_planes(&[clip_plane(&yz, &Xform::identity()).unwrap()]);
        let cut = halves(&gpu.render_offscreen(&input));
        assert!(whole[1] > 150 && cut[1] <= 8, "{whole:?} {cut:?}");
        assert_eq!(crate::engine::pipelines::created().0, compiled);
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Seen along the cut, the removed half is empty: nothing to see, nothing to pick.
    fn cut_away_geometry_cannot_be_picked() {
        let Some((mut gpu, scene)) =
            solids(vec![block([0.0; 3], [100.0; 3], Color::grey(), true)])
        else {
            return;
        };
        let plane = plane_from(Mode::Normal, &[[50.0, 0.0, 0.0], [40.0, 0.0, 0.0]], 100.0);
        cut(
            &mut gpu,
            &scene,
            &[clip_plane(&plane.unwrap(), &Xform::identity()).unwrap()],
        );

        for perspective in [true, false] {
            let (input, pixel) = frame(
                &mut gpu,
                [50.0, -300.0, 50.0],
                [50.0, 50.0, 50.0],
                perspective,
            );
            let ids = gpu.render_ids_offscreen(&input);
            let id = |p: [f64; 3]| {
                let (x, y) = pixel(p);
                ids[y * SIZE + x]
            };
            assert_eq!(id([20.0, 0.0, 50.0])[0], 0, "cut away: nothing to pick");
            assert_eq!(id([80.0, 0.0, 50.0])[0], 1, "kept: the cube");
        }
    }

    /// Meshes and breps as one document placed at `place`, back faces red so an inside reads red.
    fn placed(
        meshes: Vec<Mesh>,
        breps: Vec<session_rust::BRep>,
        place: Xform,
    ) -> Option<(Gpu, Scene)> {
        crate::app::clipping::verify_solids();
        let mut gpu = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32)).ok()?;
        gpu.pass_mut::<super::super::Clip>().fill = 0;
        gpu.view.show_grid = false;
        gpu.view.show_mesh_edges = false;
        gpu.view.show_points = false;
        gpu.view.lit = false;
        gpu.view.backface = true;
        let mut session = Session::new("placed");

        for mesh in meshes {
            session.add_mesh(mesh, None);
        }

        for brep in breps {
            session.add_brep(brep, None);
        }

        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "placed".into(),
            session: Rc::new(session),
            place,
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        Some((gpu, scene))
    }

    /// Shares of neutral (section or background), blue, red and other pixels within `r` px of `at`.
    fn tints(rgba: &[u8], at: (usize, usize), r: usize) -> [f64; 4] {
        let mut counts = [0.0; 4];
        let mut total = 0.0_f64;

        for y in at.1.saturating_sub(r)..(at.1 + r + 1).min(SIZE) {
            for x in at.0.saturating_sub(r)..(at.0 + r + 1).min(SIZE) {
                let p = &rgba[(y * SIZE + x) * 4..][..3];
                let spread = p.iter().max().unwrap() - p.iter().min().unwrap();
                let class = if spread < 40 {
                    0
                } else if p[2] > 150 && p[0] < 110 && p[1] < 110 {
                    1
                } else if p[0] > 150 && p[1] < 120 && p[2] < 120 {
                    2
                } else {
                    3
                };
                counts[class] += 1.0;
                total += 1.0;
            }
        }

        counts.map(|count| count / total.max(1.0))
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A plane lying on a face shows that face or the section, one of them, never a speckle.
    fn a_plane_on_a_face_shows_one_thing() {
        let lo = [13.7, -21.3, 5.9];
        let hi = [113.7, 58.7, 65.9];
        let Some((mut gpu, scene)) = placed(
            vec![block(lo, hi, Color::blue(), true)],
            vec![],
            Xform::identity(),
        ) else {
            return;
        };
        let mid = [
            (lo[0] + hi[0]) * 0.5,
            (lo[1] + hi[1]) * 0.5,
            (lo[2] + hi[2]) * 0.5,
        ];
        // the top, +x and bottom faces, each seen from the side its plane cuts away
        let cases = [
            (
                plane_from(Mode::Xy, &[[0.0, 0.0, hi[2]]], 300.0),
                [mid[0], mid[1], hi[2]],
                [mid[0] - 150.0, mid[1] - 200.0, hi[2] + 250.0],
            ),
            (
                plane_from(Mode::Yz, &[[hi[0], 0.0, 0.0]], 300.0),
                [hi[0], mid[1], mid[2]],
                [hi[0] + 250.0, mid[1] - 200.0, mid[2] + 150.0],
            ),
            (
                plane_from(
                    Mode::Normal,
                    &[[0.0, 0.0, lo[2]], [0.0, 0.0, lo[2] - 10.0]],
                    300.0,
                ),
                [mid[0], mid[1], lo[2]],
                [mid[0] - 150.0, mid[1] - 200.0, lo[2] - 250.0],
            ),
        ];

        for (plane, face, eye) in cases {
            let plane = clip_plane(&plane.unwrap(), &Xform::identity()).unwrap();
            cut(&mut gpu, &scene, &[plane]);
            each_view(&mut gpu, eye, mid, &[true, false], |rgba, pixel, label| {
                let shares = tints(rgba, pixel(face), 12);
                let section = shares[0] > 0.99 && hatched(patch(rgba, pixel(face), 12, BLUE));
                assert!(
                    section || shares[1] > 0.99,
                    "{label}: the face or its section at {face:?}, not both: {shares:?}"
                );
            });
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Two boxes stacked and cut where they meet: one clean section, no speckle of either face.
    fn stacked_boxes_cut_where_they_meet() {
        let Some((mut gpu, scene)) = placed(
            vec![
                block([0.0; 3], [100.0, 100.0, 50.0], Color::blue(), true),
                block([0.0, 0.0, 50.0], [100.0; 3], Color::red(), true),
            ],
            vec![],
            Xform::identity(),
        ) else {
            return;
        };
        cut(&mut gpu, &scene, &[cut_above(50.0)]);
        each_view(
            &mut gpu,
            [-80.0, -150.0, 300.0],
            [40.0, 40.0, 40.0],
            &[true, false],
            |rgba, pixel, label| {
                let at = pixel([50.0, 50.0, 50.0]);
                let shares = tints(rgba, at, 12);
                let section = shares[0] > 0.99 && hatched(patch(rgba, at, 12, BLUE));
                assert!(
                    section || shares[1] > 0.99 || shares[2] > 0.99,
                    "{label}: one clean surface where the boxes meet: {shares:?}"
                );
            },
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A mirrored placement and inward faces flip the crossings, not the section.
    fn mirrored_and_inward_solids_cap_like_any_other() {
        let outward = block([-50.0; 3], [50.0; 3], Color::blue(), true);
        let mut inward = block([-50.0; 3], [50.0; 3], Color::blue(), true);

        for corners in inward.face.values_mut() {
            corners.reverse();
        }

        for (mesh, place, name) in [
            (outward, Xform::scale_xyz(-1.0, 1.0, 1.0), "mirrored"),
            (inward, Xform::identity(), "inward"),
        ] {
            let Some((mut gpu, scene)) = placed(vec![mesh], vec![], place) else {
                return;
            };
            assert_ne!(gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED, 0);
            cut(&mut gpu, &scene, &[cut_above(0.0)]);
            each_view(
                &mut gpu,
                [100.0, -150.0, 250.0],
                [0.0; 3],
                &[true, false],
                |rgba, pixel, label| {
                    let shares = tints(rgba, pixel([0.0; 3]), 12);
                    let cap = patch(rgba, pixel([0.0; 3]), 12, BLUE);
                    assert!(
                        hatched(cap) && shares[0] > 0.99,
                        "{name}, {label}: a clean section: {shares:?} {cap:?}"
                    );
                },
            );
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Curved breps with seams and poles, and a block with a hole, cap without leaks.
    fn brep_sections_are_watertight() {
        use session_rust::BRep;

        let solids: [(BRep, [f64; 3], [f64; 3], [f64; 3]); 4] = [
            (
                BRep::create_sphere(50.0),
                [0.0; 3],
                [0.0; 3],
                [120.0, -160.0, 250.0],
            ),
            (
                BRep::create_torus(50.0, 20.0),
                [50.0, 0.0, 0.0],
                [0.0; 3],
                [120.0, -160.0, 250.0],
            ),
            (
                BRep::create_block_with_hole(120.0, 100.0, 80.0, 25.0),
                [-42.0, 0.0, 0.0],
                [0.0; 3],
                [60.0, -90.0, 320.0],
            ),
            (
                BRep::create_cylinder(40.0, 100.0),
                [0.0, 0.0, 50.0],
                [0.0; 3],
                [-220.0, 0.5, 120.0],
            ),
        ];

        for (mut brep, inside, hole, eye) in solids {
            brep.surfacecolor = Color::blue();
            let name = brep.name.clone();
            let Some((mut gpu, scene)) = placed(vec![], vec![brep], Xform::identity()) else {
                return;
            };
            assert_ne!(
                gpu.objects.row(0).unwrap().flags & Instance::FLAG_CLOSED,
                0,
                "{name} is closed"
            );
            // the cylinder stands on z = 0: cut through its axis, the rays leaving along its seam
            let plane = if name == "cylinder" {
                let p = plane_from(Mode::Normal, &[[0.0; 3], [-10.0, 0.0, 0.0]], 300.0);
                clip_plane(&p.unwrap(), &Xform::identity()).unwrap()
            } else {
                cut_above(0.0)
            };
            cut(&mut gpu, &scene, &[plane]);
            each_view(
                &mut gpu,
                eye,
                inside,
                &[true, false],
                |rgba, pixel, label| {
                    let shares = tints(rgba, pixel(inside), 6);
                    let cap = patch(rgba, pixel(inside), 6, BLUE);
                    assert!(
                        hatched(cap) && shares[0] > 0.99,
                        "{name}, {label}: a clean section at {inside:?}: {shares:?} {cap:?}"
                    );

                    // the torus and the block keep their hole open
                    if name != "sphere" && name != "cylinder" {
                        let open = patch(rgba, pixel(hole), 5, BLUE);
                        assert!(
                            open[0] < 0.02,
                            "{name}, {label}: no hatch in the hole: {open:?}"
                        );
                    }
                },
            );
        }
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Two planes cut a corner off: both sections hatched up to the edge they share, nothing leaks.
    fn two_planes_cap_the_corner() {
        let Some((mut gpu, scene)) = placed(
            vec![block([-50.0; 3], [50.0; 3], Color::blue(), true)],
            vec![],
            Xform::identity(),
        ) else {
            return;
        };
        let yz = plane_from(Mode::Yz, &[[0.0; 3]], 300.0).unwrap();
        cut(
            &mut gpu,
            &scene,
            &[cut_above(0.0), clip_plane(&yz, &Xform::identity()).unwrap()],
        );
        each_view(
            &mut gpu,
            [200.0, -150.0, 250.0],
            [0.0; 3],
            &[true, false],
            |rgba, pixel, label| {
                for at in [[-25.0, -10.0, 0.0], [0.0, -10.0, -25.0]] {
                    let cap = patch(rgba, pixel(at), 8, BLUE);
                    assert!(hatched(cap), "{label}: section at {at:?}: {cap:?}");
                }

                for at in [[-4.0, -10.0, 0.0], [0.0, -10.0, -4.0], [-4.0, 30.0, 0.0]] {
                    let shares = tints(rgba, pixel(at), 2);
                    assert!(
                        shares[0] > 0.99,
                        "{label}: nothing leaks at {at:?}: {shares:?}"
                    );
                }
            },
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// A millimetre box far from the origin: the cut and its section hold at f32.
    fn far_from_the_origin_the_section_holds() {
        let lo = [2.0e5, -1.0e5, 800.0];
        let hi = [lo[0] + 3000.0, lo[1] + 2000.0, lo[2] + 1500.0];
        let Some((mut gpu, scene)) = placed(
            vec![block(lo, hi, Color::blue(), true)],
            vec![],
            Xform::identity(),
        ) else {
            return;
        };
        let mid = [
            (lo[0] + hi[0]) * 0.5,
            (lo[1] + hi[1]) * 0.5,
            (lo[2] + hi[2]) * 0.5,
        ];
        cut(&mut gpu, &scene, &[cut_above(mid[2] + 0.3)]);
        each_view(
            &mut gpu,
            [mid[0] + 3000.0, mid[1] - 4500.0, mid[2] + 6000.0],
            mid,
            &[true, false],
            |rgba, pixel, label| {
                let at = [mid[0], mid[1], mid[2] + 0.3];
                let shares = tints(rgba, pixel(at), 12);
                let cap = patch(rgba, pixel(at), 12, BLUE);
                assert!(
                    hatched(cap) && shares[0] > 0.99,
                    "{label}: a clean section far out: {shares:?} {cap:?}"
                );
            },
        );
    }

    #[test]
    #[ignore = "requires a native GPU adapter"]
    /// Where a dowel sits inside a beam, the section picks the dowel, the innermost solid.
    fn a_nested_section_picks_the_innermost_solid() {
        let Some((mut gpu, scene)) = placed(
            vec![
                block(
                    [-100.0, -50.0, -50.0],
                    [100.0, 50.0, 50.0],
                    Color::blue(),
                    true,
                ),
                block(
                    [-10.0, -150.0, -10.0],
                    [10.0, 150.0, 10.0],
                    Color::red(),
                    true,
                ),
            ],
            vec![],
            Xform::identity(),
        ) else {
            return;
        };
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        gpu.view.msaa_forced = Some(1);
        gpu.resize(SIZE as u32, SIZE as u32);

        for perspective in [true, false] {
            let (input, pixel) = frame(&mut gpu, [150.0, -250.0, 250.0], [0.0; 3], perspective);
            let ids = gpu.render_ids_offscreen(&input);
            let id = |p: [f64; 3]| {
                let (x, y) = pixel(p);
                ids[y * SIZE + x]
            };

            for at in [[0.0, 0.0, 0.0], [3.0, -30.0, 0.0], [-3.0, 30.0, 0.0]] {
                assert_eq!(
                    id(at),
                    [2, 0],
                    "perspective {perspective}: the dowel at {at:?}"
                );
            }

            assert_eq!(
                id([40.0, 0.0, 0.0]),
                [1, 0],
                "perspective {perspective}: the beam"
            );
        }
    }

    /// A block defined once and placed three times, straight, turned and mirrored: each
    /// placed solid gets its section like the same blocks baked, and a pick through a cap
    /// names the instance.
    #[test]
    #[ignore = "requires a native GPU adapter"]
    fn instanced_solids_cap_like_baked_ones() {
        use session_rust::{Geometry, InstanceRef};

        let places = [
            Xform::translation(-60.0, 0.0, 0.0),
            &Xform::translation(60.0, 0.0, 0.0) * &Xform::rotation_z(30.0, true),
            &Xform::translation(0.0, 70.0, 0.0) * &Xform::scale_xyz(-1.0, 1.0, 1.0),
        ];
        let mut source = Session::new("instanced");
        let cube = block([-25.0; 3], [25.0; 3], Color::blue(), true);
        let definition = source.add_definition(Geometry::Mesh(Rc::new(cube)));

        for place in &places {
            let instance = InstanceRef::new(&definition, Xform::identity());
            source.add_instance(instance, place.clone(), None);
        }

        let baked: Vec<Mesh> = source
            .get_geometry()
            .meshes
            .iter()
            .map(|mesh| (**mesh).clone())
            .collect();
        let Some((mut gpu, scene)) = shown(source) else {
            return;
        };
        let Some((mut want, want_scene)) = placed(baked, vec![], Xform::identity()) else {
            return;
        };
        // baking a mirror keeps the winding, so the baked mirrored block would show red backs
        gpu.view.backface = false;
        want.view.backface = false;
        cut(&mut gpu, &scene, &[cut_above(0.0)]);
        cut(&mut want, &want_scene, &[cut_above(0.0)]);
        assert_eq!(
            gpu.pass::<super::super::Clip>().placed.records.len(),
            3,
            "every instance is cut"
        );
        let centers = [[-60.0, 0.0, 0.0], [60.0, 0.0, 0.0], [0.0, 70.0, 0.0]];

        for samples in [1, 4] {
            for target in [&mut gpu, &mut want] {
                target.view.msaa_forced = Some(samples);
                target.resize(SIZE as u32, SIZE as u32);
            }

            for perspective in [true, false] {
                let eye = [40.0, -200.0, 260.0];
                let (input, pixel) = frame(&mut gpu, eye, [0.0, 20.0, 0.0], perspective);
                let rgba = gpu.render_offscreen(&input);
                let (input, _) = frame(&mut want, eye, [0.0, 20.0, 0.0], perspective);
                let expected = want.render_offscreen(&input);
                let label = format!("{samples}x, perspective {perspective}");

                for at in centers {
                    let shares = tints(&rgba, pixel(at), 8);
                    let cap = patch(&rgba, pixel(at), 8, BLUE);
                    assert!(
                        hatched(cap) && shares[0] > 0.99,
                        "{label}: the section at {at:?}: {shares:?} {cap:?}"
                    );
                }

                let far = rgba
                    .chunks_exact(4)
                    .zip(expected.chunks_exact(4))
                    .filter(|(a, b)| (0..3).any(|k| a[k].abs_diff(b[k]) > 24))
                    .count();
                eprintln!("{label}: {far} pixels unlike baked");

                if let Some(dir) = std::env::var_os("INSTANCING_SHOTS") {
                    for (name, pixels) in [("instanced", &rgba), ("baked", &expected)] {
                        let mut ppm = format!("P6 {SIZE} {SIZE} 255\n").into_bytes();
                        ppm.extend(pixels.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]));
                        let file = format!("caps_{samples}_{perspective}_{name}.ppm");
                        std::fs::write(std::path::Path::new(&dir).join(file), ppm).unwrap();
                    }
                }

                assert!(far * 400 <= SIZE * SIZE, "{label}: {far} unlike baked");
            }
        }

        gpu.view.msaa_forced = Some(1);
        gpu.resize(SIZE as u32, SIZE as u32);
        let (input, pixel) = frame(&mut gpu, [40.0, -200.0, 260.0], [0.0, 20.0, 0.0], true);
        let ids = gpu.render_ids_offscreen(&input);

        for (at, place) in centers.iter().zip(&places) {
            let (x, y) = pixel(*at);
            let row = ids[y * SIZE + x][0].checked_sub(1).expect("a cap pick");
            let placement = scene.placement_of(row).expect("an instance row");
            let same = (0..16).all(|i| (placement.m[i] - place.m[i]).abs() < 1e-9);
            assert!(same, "the cap at {at:?} picks its instance");
        }
    }

    /// A session's instances on a headless GPU, as `placed` sets it up.
    fn shown(session: Session) -> Option<(Gpu, Scene)> {
        crate::app::clipping::verify_solids();
        let mut gpu = pollster::block_on(Gpu::new_headless(SIZE as u32, SIZE as u32)).ok()?;
        gpu.pass_mut::<super::super::Clip>().fill = 0;
        gpu.view.show_grid = false;
        gpu.view.show_mesh_edges = false;
        gpu.view.show_points = false;
        gpu.view.lit = false;
        gpu.view.backface = true;
        let mut scene = Scene::new();
        scene.add_file(FileDoc {
            name: "instanced".into(),
            session: Rc::new(session),
            place: Xform::identity(),
            point_px: 0.0,
            display_only: false,
        });
        scene.upload_to(&mut gpu);
        Some((gpu, scene))
    }
}
