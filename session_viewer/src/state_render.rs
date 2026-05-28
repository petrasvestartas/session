impl State {
    pub fn render(&mut self) -> anyhow::Result<()> {
        self.window.request_redraw();

        if !self.is_surface_configured {
            return Ok(());
        }

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                self.surface.configure(&self.device, &self.config);
                texture
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return Ok(()),
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return Ok(());
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.gpu_session.flush_geometry(&self.device, &self.queue, &self.pipelines.geom_bgl);
        if let Some(bg) = self.gpu_session.take_rebuilt_bind_group(
            &self.device, &self.pipelines.bind_group_layout, &self.camera_buf,
        ) {
            self.bind_group = bg;
        }

        // Build text/glyph vertex buffers before opening the geometry pass
        // (buffers must outlive the render pass that borrows them).
        let visible_labels: Vec<&text::TextLabel> = self.text_labels.iter()
            .filter(|l| !self.hidden_guids.contains(&l.guid))
            .collect();
        let quad_verts = text::build_label_quads(&visible_labels, &self.selected_guids);
        let quad_buf = if !quad_verts.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text.quads"),
                contents: bytemuck::cast_slice(&quad_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };
        let glyph_verts = text::build_glyph_quads(&visible_labels);
        let glyph_buf = if !glyph_verts.is_empty() {
            Some(self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("text.glyphs"),
                contents: bytemuck::cast_slice(&glyph_verts),
                usage: wgpu::BufferUsages::VERTEX,
            }))
        } else {
            None
        };

        // Geometry pass → MSAA texture
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Geometry Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
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

            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_pipeline(&self.pipelines.grid);
            render_pass.draw(0..298, 0..1);

            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_pipeline(&self.pipelines.mesh);
            self.gpu_session.draw_all_mesh(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.line);
            self.gpu_session.draw_lines(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.point);
            self.gpu_session.draw_points(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.cylinder);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_bind_group(1, &self.gpu_session.segment_bg, &[]);
            self.gpu_session.draw_cylinders(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.sphere);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_bind_group(1, &self.gpu_session.glyph_sphere_bg, &[]);
            self.gpu_session.draw_spheres(&mut render_pass);

            render_pass.set_pipeline(&self.pipelines.point_cloud);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.set_bind_group(1, &self.gpu_session.cloud_bg, &[]);
            self.gpu_session.draw_clouds(&mut render_pass);

            let nc = self.gpu_session.cones_cpu.len() as u32;
            if nc > 0 {
                render_pass.set_pipeline(&self.pipelines.cone);
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_bind_group(1, &self.gpu_session.cone_bg, &[]);
                render_pass.set_vertex_buffer(0, self.gpu_session.cylinder_vbo.slice(..));
                render_pass.set_index_buffer(self.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..nc);
            }

            // Text background quads — depth-tested, drawn after opaque geometry.
            if let Some(buf) = &quad_buf {
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_pipeline(&self.pipelines.text);
                render_pass.set_vertex_buffer(0, buf.slice(..));
                render_pass.draw(0..quad_verts.len() as u32, 0..1);
            }

            // Glyph characters — depth-tested, font atlas in group 1.
            if let Some(buf) = &glyph_buf {
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.set_bind_group(1, &self.glyph_bind_group, &[]);
                render_pass.set_pipeline(&self.pipelines.glyph);
                render_pass.set_vertex_buffer(0, buf.slice(..));
                render_pass.draw(0..glyph_verts.len() as u32, 0..1);
            }
        }

        // Gumball overlay — cylinders (shafts+arcs), cones (arrowheads), spheres (handles).
        if let Some(gb) = &self.gumball {
            let lines   = gumball::build_lines(gb.origin, self.gumball_scale, gb.hovered);
            let cones   = gumball::build_cones(gb.origin, self.gumball_scale, gb.hovered);
            let spheres = gumball::build_spheres(gb.origin, self.gumball_scale, gb.hovered);
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
            // Upload segment buffer
            let seg_bytes = bytemuck::cast_slice::<_, u8>(&segs);
            if seg_bytes.len() as u64 > self.gumball_seg_buf.size() {
                self.gumball_seg_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gumball.segments"),
                    size: seg_bytes.len() as u64 * 2,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.gumball_seg_bg = gpu_session::make_geom_bind_group(
                    &self.device, &self.pipelines.geom_bgl, &self.gumball_seg_buf,
                );
            }
            self.queue.write_buffer(&self.gumball_seg_buf, 0, seg_bytes);
            // Upload cone buffer
            let cone_bytes = bytemuck::cast_slice::<_, u8>(&cone_segs);
            if cone_bytes.len() as u64 > self.gumball_cone_buf.size() {
                self.gumball_cone_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gumball.cones"),
                    size: cone_bytes.len() as u64 * 2,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.gumball_cone_bg = gpu_session::make_geom_bind_group(
                    &self.device, &self.pipelines.geom_bgl, &self.gumball_cone_buf,
                );
            }
            self.queue.write_buffer(&self.gumball_cone_buf, 0, cone_bytes);
            // Upload glyph buffer
            let glyph_bytes = bytemuck::cast_slice::<_, u8>(&glyph_pts);
            if glyph_bytes.len() as u64 > self.gumball_glyph_buf.size() {
                self.gumball_glyph_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("gumball.glyphs"),
                    size: glyph_bytes.len() as u64 * 2,
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });
                self.gumball_glyph_bg = gpu_session::make_geom_bind_group(
                    &self.device, &self.pipelines.geom_bgl, &self.gumball_glyph_buf,
                );
            }
            self.queue.write_buffer(&self.gumball_glyph_buf, 0, glyph_bytes);
            {
                let mut gpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Gumball Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &self.msaa_view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_view,
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
                gpass.set_bind_group(0, &self.gumball_bind_group, &[]);
                // Draw cylinder shafts + arcs
                if !segs.is_empty() {
                    gpass.set_pipeline(&self.pipelines.cylinder);
                    gpass.set_bind_group(1, &self.gumball_seg_bg, &[]);
                    gpass.set_vertex_buffer(0, self.gpu_session.cylinder_vbo.slice(..));
                    gpass.set_index_buffer(self.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..segs.len() as u32);
                }
                // Draw cone arrowheads
                if !cone_segs.is_empty() {
                    gpass.set_pipeline(&self.pipelines.cone);
                    gpass.set_bind_group(1, &self.gumball_cone_bg, &[]);
                    gpass.set_vertex_buffer(0, self.gpu_session.cylinder_vbo.slice(..));
                    gpass.set_index_buffer(self.gpu_session.cylinder_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_CYL_INDICES, 0, 0..cone_segs.len() as u32);
                }
                // Draw sphere handles
                if !glyph_pts.is_empty() {
                    gpass.set_pipeline(&self.pipelines.sphere);
                    gpass.set_bind_group(1, &self.gumball_glyph_bg, &[]);
                    gpass.set_vertex_buffer(0, self.gpu_session.sphere_vbo.slice(..));
                    gpass.set_index_buffer(self.gpu_session.sphere_ibo.slice(..), wgpu::IndexFormat::Uint32);
                    gpass.draw_indexed(0..gpu_adapters::N_SPHERE_INDICES, 0, 0..glyph_pts.len() as u32);
                }
            }
        }

        // Resolve pass: MSAA → swapchain (empty pass, resolve triggers on end)
        {
            let _resolve = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Resolve Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.msaa_view,
                    resolve_target: Some(&view),
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

        // egui pass
        let full_out = self.build_ui();
        self.egui_state.handle_platform_output(&self.window, full_out.platform_output);
        let tris = self.egui_ctx.tessellate(full_out.shapes, full_out.pixels_per_point);
        let screen = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [self.config.width, self.config.height],
            pixels_per_point: full_out.pixels_per_point,
        };
        for (id, delta) in &full_out.textures_delta.set {
            self.egui_renderer.update_texture(&self.device, &self.queue, *id, delta);
        }
        let extra_cmds = self.egui_renderer.update_buffers(&self.device, &self.queue, &mut encoder, &tris, &screen);
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
            self.egui_renderer.render(&mut epass.forget_lifetime(), &tris, &screen);
        }
        for id in &full_out.textures_delta.free { self.egui_renderer.free_texture(id); }

        self.queue.submit(extra_cmds.into_iter().chain(iter::once(encoder.finish())));
        output.present();

        Ok(())
    }
}
