    }
  }
  pub fn set_scene(
    &mut self,
    ctx: &GpuCtx,
    scene: &crate::scene::Scene,
    selected: &[crate::scene::ObjectId],
  ) -> Result<(), &'static str> {
    let mut cache: HashMap<usize, Rc<Columns>> =
      self.draws.iter().map(|d| (Rc::as_ptr(&d.row.prepared.source) as usize, d.columns.clone())).collect();
    let mut old = std::mem::take(&mut self.draws);
    let mut draws = Vec::with_capacity(scene.clouds().len());
    for row in scene.clouds() {
      let address = Rc::as_ptr(&row.prepared.source) as usize;
      let columns = if let Some(columns) = cache.get(&address) {
        columns.clone()
      } else {
        let columns = Rc::new(Columns::new(ctx, row)?);
        self.uploads += 1;
        cache.insert(address, columns.clone());
        columns
      };
      let (records, group) =
        if let Some(i) = old.iter().position(|d| d.row.id == row.id && Rc::ptr_eq(&d.columns, &columns)) {
          let d = old.swap_remove(i);
          (d.records, d.group)
        } else {
          let records = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bounded cloud draw records"),
            size: 16,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
          });
          let group = resource_group(
            ctx,
            &self.columns_layout,
            "cloud source and records",
            [
              (0, records.as_entire_binding()),
              (1, columns.positions.as_entire_binding()),
              (2, columns.colors.as_entire_binding()),
              (3, columns.normals.as_entire_binding()),
            ],
          );
          (records, group)
        };
      draws.push(Draw {
        row: row.clone(),
        columns,
        records,
        group,
        visible: scene.visible(row.id),
        pickable: scene.pick_id(row.id.value()).is_some(),
        selected: selected.contains(&row.id),
        color: scene.view.faces.get(&row.id).copied(),
        total: 0,
      });
    }
    self.draws = draws;
    self.revision = self.revision.wrapping_add(1);
    self.key = None;
    Ok(())
  }
  pub fn usage(&self) -> [u64; 5] {
    let mut seen = std::collections::HashSet::new();
    let columns =
      self.draws.iter().filter(|d| seen.insert(Rc::as_ptr(&d.columns))).map(|d| d.columns.bytes()).sum::<u64>();
    let targets = self.targets.as_ref().map_or(0, |t| t.color.bytes + t.depth.bytes + t.ids.bytes);
    [
      self.total,
      columns
        + self.draws.iter().map(|d| d.records.size()).sum::<u64>()
        + targets
        + self.settings.size()
        + self.window.size(),
      self.uploads,
      self.preparations,
      self.skips,
    ]
  }
}
fn pipeline(
