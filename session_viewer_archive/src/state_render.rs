use std::iter;
use crate::State;
use crate::{gpu_adapters, gpu_session, gumball, text};
use wgpu::util::DeviceExt;

impl State {
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        if !self.gpu.is_surface_configured {
            return Ok(());
        }

        let output = match self.gpu.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                self.gpu.surface.configure(&self.gpu.device, &self.gpu.config);
                texture
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.gpu.surface.configure(&self.gpu.device, &self.gpu.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
            wgpu::CurrentSurfaceTexture::Lost => {
                self.gpu.surface.configure(&self.gpu.device, &self.gpu.config);
                return Ok(());
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.gpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.scene.gpu_session.flush_geometry(&self.gpu.device, &self.gpu.queue, &self.gpu.pipelines.geom_bgl);
        let new_bg = self.scene.gpu_session.take_rebuilt_bind_group(
            &self.gpu.device, &self.gpu.pipelines.bind_group_layout, &self.gpu.camera_buf,
        );
        if let Some(bg) = new_bg { self.gpu.bind_group = bg; }

        // Build text/glyph vertex buffers before opening the geometry pass
        // (buffers must outlive the render pass that borrows them).
        let visible_labels: Vec<&text::TextLabel> = self.scene.text_labels.iter()
            .filter(|l| !self.scene.hidden_guids.contains(&l.guid))
            .collect();
        let quad_verts = text::build_label_quads(&visible_labels, &self.scene.selected_guids);
        let quad_buf = if !quad_verts.is_empty() {
            Some(self.gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text.quads"),
                contents: bytemuck::cast_slice(&quad_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };
        let glyph_verts = text::build_glyph_quads(&visible_labels);
        let glyph_buf = if !glyph_verts.is_empty() {
            Some(self.gpu.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text.glyphs"),
                contents: bytemuck::cast_slice(&glyph_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        // Frustum cull (batched mesh path): flag off-screen objects before the pass so the
        // whole mesh arena still draws in one call. Built from the same view-projection the
        // GPU uses this frame.
        let frustum = self.scene.frustum_cull
            .then(|| crate::camera::Frustum::from_view_proj(&self.scene.camera.view_proj()));
        let mut stats = gpu_session::DrawStats::default();
        let (drawn, culled) = self.scene.gpu_session.apply_frustum_cull(frustum.as_ref(), &self.gpu.queue);
        stats.mesh_drawn = drawn;
        stats.mesh_culled = culled;
        stats.mesh_objects = drawn + culled;

        // Arctic mode: white shading + ground plane always; SSAO/composite only when
        // the backend can bind MSAA depth (arctic_full). The outline pass shares the
        // post-processing chain, so either feature routes the resolve through
        // scene_color + composite.
        let arctic = self.scene.arctic;
        let post_ok = self.gpu.pipelines.arctic.is_some() && self.gpu.arctic_targets.is_some();
        let arctic_full = arctic && post_ok;
        let outline_on = self.scene.outline && post_ok;
        let post_on = arctic_full || outline_on;

        // Geometry pass → MSAA texture
        {
            let clear = if arctic {
                wgpu::Color { r: 0.96, g: 0.96, b: 0.97, a: 1.0 }
            } else {
                self.gpu.clear_color
            };
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Geometry Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.gpu.msaa_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.gpu.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });

            render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
            if arctic {
                // Horizon gradient first (no depth), then the white depth-writing
                // ground plane — SSAO turns it into the soft contact shadow. The
                // grid draws on top of the plane (Always compare, no depth write)
                // so it stays visible in arctic too.
                if self.scene.arctic_gradient {
                    render_pass.set_pipeline(&self.gpu.pipelines.background);
                    render_pass.draw(0..3, 0..1);
                    stats.draw_calls += 1;
                }
                render_pass.set_pipeline(&self.gpu.pipelines.ground);
                render_pass.set_vertex_buffer(0, self.gpu.ground_vbo.slice(..));
                render_pass.draw(0..6, 0..1);
                stats.draw_calls += 1;
            }
            render_pass.set_pipeline(&self.gpu.pipelines.grid);
            render_pass.draw(0..298, 0..1);
            stats.draw_calls += 1;

            // Meshes: one batched draw for the whole arena, then template instance groups.
            render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
            render_pass.set_pipeline(&self.gpu.pipelines.mesh_batched);
            self.scene.gpu_session.draw_meshes(&mut render_pass);
            if self.scene.gpu_session.mesh_draw_count > 0 { stats.draw_calls += 1; }
            render_pass.set_pipeline(&self.gpu.pipelines.mesh);
            stats.draw_calls += self.scene.gpu_session.draw_instance_groups(&mut render_pass);

            render_pass.set_pipeline(&self.gpu.pipelines.line);
            self.scene.gpu_session.draw_lines(&mut render_pass);
            stats.draw_calls += self.scene.gpu_session.line.iter_slots().count() as u32;

            render_pass.set_pipeline(&self.gpu.pipelines.point);
            self.scene.gpu_session.draw_points(&mut render_pass);
            stats.draw_calls += self.scene.gpu_session.point.iter_slots().count() as u32;

            render_pass.set_pipeline(&self.gpu.pipelines.cylinder);
            render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
            render_pass.set_bind_group(1, &self.scene.gpu_session.segment_bg, &[]);
            self.scene.gpu_session.draw_cylinders(&mut render_pass);
            if !self.scene.gpu_session.segments_cpu.is_empty() { stats.draw_calls += 1; }

            render_pass.set_pipeline(&self.gpu.pipelines.sphere);
            render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
            render_pass.set_bind_group(1, &self.scene.gpu_session.glyph_sphere_bg, &[]);
            self.scene.gpu_session.draw_spheres(&mut render_pass);
            if !self.scene.gpu_session.glyphs_cpu.is_empty() { stats.draw_calls += 1; }

            render_pass.set_pipeline(&self.gpu.pipelines.point_cloud);
            render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
            render_pass.set_bind_group(1, &self.scene.gpu_session.cloud_bg, &[]);
            self.scene.gpu_session.draw_clouds(&mut render_pass);
            if !self.scene.gpu_session.clouds_cpu.is_empty() { stats.draw_calls += 1; }

            let nc = self.scene.gpu_session.cones_cpu.len() as u32;
            if nc > 0 {
                stats.draw_calls += 1;
                render_pass.set_pipeline(&self.gpu.pipelines.cone);
                render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
                render_pass.set_bind_group(1, &self.scene.gpu_session.cone_bg, &[]);
                render_pass.set_vertex_buffer(0, self.scene.gpu_session.cylinder_vbo.slice(..));
                render_pass.set_index_buffer(self.scene.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..nc);
            }

            // Text background quads — depth-tested, drawn after opaque geometry.
            if let Some(buf) = &quad_buf {
                render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
                render_pass.set_pipeline(&self.gpu.pipelines.text);
                render_pass.set_vertex_buffer(0, buf.slice(..));
                render_pass.draw(0..quad_verts.len() as u32, 0..1);
            }

            // Glyph characters — depth-tested, font atlas in group 1.
            if let Some(buf) = &glyph_buf {
                render_pass.set_bind_group(0, &self.gpu.bind_group, &[]);
                render_pass.set_bind_group(1, &self.gpu.glyph_bind_group, &[]);
                render_pass.set_pipeline(&self.gpu.pipelines.glyph);
                render_pass.set_vertex_buffer(0, buf.slice(..));
                render_pass.draw(0..glyph_verts.len() as u32, 0..1);
            }
        }

        self.scene.draw_stats = stats;

        // SSAO + blur — MUST run right after the geometry pass: the edit overlay and
        // gumball passes below clear the depth buffer the AO is computed from.
        if arctic_full {
            let ap = self.gpu.pipelines.arctic.as_ref().unwrap();
            let targets = self.gpu.arctic_targets.as_ref().unwrap();
            {
                let mut sp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("SSAO Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &targets.ao_raw_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
                sp.set_pipeline(&ap.ssao);
                sp.set_bind_group(0, &targets.ssao_bg, &[]);
                sp.draw(0..3, 0..1);
            }
            // Separable depth-aware Gaussian (H then V, ping-pong raw -> blur -> raw):
            // smooth soft shadows without bleeding across silhouettes. Composite
            // reads ao_raw.
            {
                let mut bp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("SSAO Blur Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &targets.ao_blur_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
                bp.set_pipeline(&ap.blur_h);
                bp.set_bind_group(0, &targets.blur_bg, &[]);
                bp.draw(0..3, 0..1);
            }
            {
                let mut bp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("SSAO Blur Pass 2"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &targets.ao_raw_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
                bp.set_pipeline(&ap.blur_v);
                bp.set_bind_group(0, &targets.blur2_bg, &[]);
                bp.draw(0..3, 0..1);
            }
        }

        // Outline mask: surface geometry only (mesh arena + instanced templates) →
        // R32Float id buffer with its own depth. Composite turns id discontinuities
        // into a boundary; lines/points never render here so they get no outline.
        if outline_on {
            let ap = self.gpu.pipelines.arctic.as_ref().unwrap();
            let targets = self.gpu.arctic_targets.as_ref().unwrap();
            if !arctic_full {
                // Composite multiplies by the AO texture — make sure it is white
                // when the SSAO passes did not run this frame.
                let _clear = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("AO Clear Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &targets.ao_raw_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
            }
            let mut mp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Outline Mask Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &targets.mask_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &targets.mask_depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            mp.set_bind_group(0, &self.gpu.bind_group, &[]);
            mp.set_pipeline(&ap.mask_batched);
            self.scene.gpu_session.draw_meshes(&mut mp);
            mp.set_pipeline(&ap.mask);
            self.scene.gpu_session.draw_instance_groups(&mut mp);
        }

        // Edit overlay — control-point spheres + control-polygon/edge cylinders.
        // Drawn after geometry (clears depth so handles show through) and before the
        // gumball pass (so the gizmo stays on top). Group 0 reuses the gumball's
        // identity-instance bind group.
        if self.edit.active && !self.edit.nodes.is_empty() {
            let scale = self.edit.handle_scale;
            // Cage lines ~ default object edge thickness; control points twice as big.
            let edge_r = 1.0 * scale;
            let node_r = 2.0 * edge_r;
            let move_set: std::collections::HashSet<usize> =
                self.edit.move_node_set().into_iter().collect();
            // Edge-move mode highlights the EDGE (control polygon, below), not the points —
            // so suppress the control-point spheres there.
            let node_glyphs: Vec<gpu_session::GlyphPoint> = if self.edit.edge_mode {
                Vec::new()
            } else {
                self.edit.nodes.iter().enumerate().map(|(i, n)| {
                    let sel = move_set.contains(&i);
                    // Black like default geometry; picked points highlight orange.
                    let color = if sel { [1.0, 1.0, 0.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] };
                    gpu_session::GlyphPoint { center: crate::edit_state::f32p(n.world), radius: node_r, color, instance_id: 0, _pad: [0; 3] }
                }).collect()
            };
            // A highlighted edge (edge-move mode) draws thicker so it reads as "the edge".
            let sel_edge_r = if self.edit.edge_mode { 2.5 * edge_r } else { edge_r };
            // When the selected edge has a true-curve polyline (surface boundary / BRep edge),
            // draw THAT — so a circular edge reads as a circle, not the chorded control polygon.
            let edge_segs: Vec<gpu_session::CylinderSegment> = if !self.edit.edge_polyline.is_empty() {
                let p = &self.edit.edge_polyline;
                (0..p.len().saturating_sub(1)).map(|i| gpu_session::CylinderSegment {
                    p0: p[i], radius: sel_edge_r,
                    p1: p[i + 1], instance_id: 0, color: [1.0, 1.0, 0.0, 1.0],
                }).collect()
            } else {
                self.edit.edges.iter().enumerate().map(|(ei, e)| {
                    let sel = self.edit.selected_edges.contains(&ei);
                    let color = if sel { [1.0, 1.0, 0.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] };
                    let r = if sel { sel_edge_r } else { edge_r };
                    gpu_session::CylinderSegment {
                        p0: crate::edit_state::f32p(self.edit.nodes[e.a].world), radius: r,
                        p1: crate::edit_state::f32p(self.edit.nodes[e.b].world), instance_id: 0, color,
                    }
                }).collect()
            };
            let (dev, q, bgl) = (&self.gpu.device, &self.gpu.queue, &self.gpu.pipelines.geom_bgl);
            self.edit.upload(dev, q, bgl, &node_glyphs, &edge_segs);
            {
                let mut epass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Edit Overlay Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.gpu.msaa_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: wgpu::StoreOp::Store },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.gpu.depth_view,
                        depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: wgpu::StoreOp::Store }),
                        stencil_ops: None,
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
                epass.set_bind_group(0, &self.gb.gumball_bind_group, &[]);
                if !edge_segs.is_empty() {
                    epass.set_pipeline(&self.gpu.pipelines.cylinder);
                    epass.set_bind_group(1, &self.edit.edge_bg, &[]);
                    epass.set_vertex_buffer(0, self.scene.gpu_session.cylinder_vbo.slice(..));
                    epass.set_index_buffer(self.scene.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    epass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..edge_segs.len() as u32);
                }
                if !node_glyphs.is_empty() {
                    epass.set_pipeline(&self.gpu.pipelines.sphere);
                    epass.set_bind_group(1, &self.edit.node_bg, &[]);
                    epass.set_vertex_buffer(0, self.scene.gpu_session.sphere_vbo.slice(..));
                    epass.set_index_buffer(self.scene.gpu_session.sphere_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    epass.draw_indexed(0..gpu_adapters::N_SPHERE_INDICES, 0, 0..node_glyphs.len() as u32);
                }
            }
        }

        // Gumball overlay — cylinders (shafts+arcs), cones (arrowheads), spheres (handles).
        if let Some(gb) = &self.gb.gumball {
            let lines   = gumball::build_lines(gb.origin, self.gb.gumball_scale, gb.hovered);
            let cones   = gumball::build_cones(gb.origin, self.gb.gumball_scale, gb.hovered);
            let spheres = gumball::build_spheres(gb.origin, self.gb.gumball_scale, gb.hovered);
            let segs: Vec<gpu_session::CylinderSegment> = lines.iter().map(|l| {
                let c = l.color;
                gpu_session::CylinderSegment {
                    p0: l.a, radius: l.radius, p1: l.b, instance_id: 0,
                    color: [c[0] as f32/255.0, c[1] as f32/255.0, c[2] as f32/255.0, c[3] as f32/255.0],
                }
            }).collect();
            let cone_segs: Vec<gpu_session::CylinderSegment> = cones.iter().map(|cn| {
                let c = cn.color;
                gpu_session::CylinderSegment {
                    p0: cn.base, radius: cn.radius, p1: cn.tip, instance_id: 0,
                    color: [c[0] as f32/255.0, c[1] as f32/255.0, c[2] as f32/255.0, c[3] as f32/255.0],
                }
            }).collect();
            let glyph_pts: Vec<gpu_session::GlyphPoint> = spheres.iter().map(|s| {
                let c = s.color;
                gpu_session::GlyphPoint {
                    center: s.center, radius: s.radius,
                    color: [c[0] as f32/255.0, c[1] as f32/255.0, c[2] as f32/255.0, c[3] as f32/255.0],
                    instance_id: 0, _pad: [0; 3],
                }
            }).collect();
            let (dev, q, bgl) = (&self.gpu.device, &self.gpu.queue, &self.gpu.pipelines.geom_bgl);
            self.gb.upload_geometry(dev, q, bgl, &segs, &cone_segs, &glyph_pts);
            {
                let mut gpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Gumball Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.gpu.msaa_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.gpu.depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                    multiview_mask: None,
                });
                gpass.set_bind_group(0, &self.gb.gumball_bind_group, &[]);
                // Draw cylinder shafts + arcs
                if !segs.is_empty() {
                    gpass.set_pipeline(&self.gpu.pipelines.cylinder);
                    gpass.set_bind_group(1, &self.gb.gumball_seg_bg, &[]);
                    gpass.set_vertex_buffer(0, self.scene.gpu_session.cylinder_vbo.slice(..));
                    gpass.set_index_buffer(self.scene.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..segs.len() as u32);
                }
                // Draw cone arrowheads
                if !cone_segs.is_empty() {
                    gpass.set_pipeline(&self.gpu.pipelines.cone);
                    gpass.set_bind_group(1, &self.gb.gumball_cone_bg, &[]);
                    gpass.set_vertex_buffer(0, self.scene.gpu_session.cylinder_vbo.slice(..));
                    gpass.set_index_buffer(self.scene.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..cone_segs.len() as u32);
                }
                // Draw sphere handles
                if !glyph_pts.is_empty() {
                    gpass.set_pipeline(&self.gpu.pipelines.sphere);
                    gpass.set_bind_group(1, &self.gb.gumball_glyph_bg, &[]);
                    gpass.set_vertex_buffer(0, self.scene.gpu_session.sphere_vbo.slice(..));
                    gpass.set_index_buffer(self.scene.gpu_session.sphere_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_SPHERE_INDICES, 0, 0..glyph_pts.len() as u32);
                }
            }
        }

        // Resolve pass: MSAA → swapchain (empty pass, resolve triggers on end).
        // Arctic: resolve to the offscreen scene_color instead; composite below
        // multiplies it by the blurred AO and writes the swapchain.
        {
            let resolve_view = if post_on {
                &self.gpu.arctic_targets.as_ref().unwrap().scene_color_view
            } else {
                &view
            };
            let _resolve = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Resolve Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.gpu.msaa_view,
                    resolve_target: Some(resolve_view),
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        }

        // Composite pass: scene_color × AO (+ outline boundary) → swapchain.
        if post_on {
            let ap = self.gpu.pipelines.arctic.as_ref().unwrap();
            let targets = self.gpu.arctic_targets.as_ref().unwrap();
            let mut cp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Composite Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            cp.set_pipeline(&ap.composite);
            cp.set_bind_group(0, &targets.composite_bg, &[]);
            cp.draw(0..3, 0..1);
        }

        // egui pass
        let full_out = self.build_ui();
        self.shell.egui_state.handle_platform_output(&self.window, full_out.platform_output);
        let tris = self.shell.egui_ctx.tessellate(full_out.shapes, full_out.pixels_per_point);
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.gpu.config.width, self.gpu.config.height],
            pixels_per_point: full_out.pixels_per_point,
        };
        for (id, delta) in &full_out.textures_delta.set {
            self.shell.egui_renderer.update_texture(&self.gpu.device, &self.gpu.queue, *id, delta);
        }
        let extra_cmds = self.shell.egui_renderer.update_buffers(&self.gpu.device, &self.gpu.queue, &mut encoder, &tris, &screen);
        {
            let epass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
            self.shell.egui_renderer.render(&mut epass.forget_lifetime(), &tris, &screen);
        }
        for id in &full_out.textures_delta.free { self.shell.egui_renderer.free_texture(id); }

        self.gpu.queue.submit(extra_cmds.into_iter().chain(iter::once(encoder.finish())));
        output.present();

        Ok(())
    }
}
