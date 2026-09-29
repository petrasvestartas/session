
impl State {
    /// Half the rectangle of a new clipping plane: a little more than the scene's widest half.
    pub(crate) fn clipping_half_size(&mut self) -> f64 {
        self.refresh_bounds();
        let b = &self.gpu.bounds;
        let half = b.hx.max(b.hy).max(b.hz);

        if b.is_valid() && half.is_finite() && half > 0.0 {
            half * 1.1
        } else {
            500.0
        }
    }

    /// Start picking the points of a clipping plane in `mode`.
    pub(crate) fn pick_clipping_plane(&mut self, mode: Mode) -> String {
        let prefix = format!("{} {}", clipping::NAME, mode.word());
        self.ask_points(clipping::NAME, &prefix, mode.prompts())
    }

    /// Add a clipping plane as one undo step, then select it.
    pub(crate) fn create_clipping_plane(&mut self, plane: Plane) -> Result<String, String> {
        if self.gpu.objects.clipping_rows().len() >= MAX_PLANES {
            return Err(format!("at most {MAX_PLANES} clipping planes"));
        }

        let (doc, guid) = self
            .scene
            .create_geometry(Geometry::Plane(Rc::new(plane)))?;
        self.after_history();
        let row = self.scene.row_of(doc, &guid);
        self.select(row);
        Ok(
            "Created Clipping Plane · the gumball moves the cut · Clipping Plane Off shows everything · Undo removes it"
                .into(),
        )
    }

    /// Swap the kept and cut sides of every selected clipping plane.
    pub(crate) fn flip_clipping_planes(&mut self) -> Result<String, String> {
        let rows: Vec<u32> = self
            .selected_rows()
            .into_iter()
            .filter(|row| self.gpu.objects.clipping_rows().binary_search(row).is_ok())
            .collect();

        if rows.is_empty() {
            return Err("select a clipping plane to flip".into());
        }

        for &row in &rows {
            let flipped = match self.scene.geometry(row) {
                Some(Geometry::Plane(plane)) => clipping::flipped(plane),
                _ => continue,
            };
            self.scene.commit_geometry(
                row,
                Geometry::Plane(Rc::new(flipped)),
                "flip clipping plane",
            )?;
        }

        self.commit_rows();
        self.select_rows(rows, false);
        Ok("Flipped: the other side is cut away · Undo flips it back".into())
    }

    /// Turn every cut on or off; the planes stay either way.
    pub(crate) fn set_clipping(&mut self, on: bool) -> String {
        self.gpu.pass_mut::<Clip>().enabled = on;
        self.touch();

        if on {
            "Clipping Plane On: the planes cut again".into()
        } else {
            "Clipping Plane Off: everything shows, the planes stay".into()
        }
    }

    /// Fill the section caps with black hatch or solid light grey; `None` switches.
    pub(crate) fn set_clipping_fill(&mut self, solid: Option<bool>) -> String {
        let clip = self.gpu.pass_mut::<Clip>();
        let solid = solid.unwrap_or(clip.fill == 0);
        clip.fill = u32::from(solid);
        self.touch();
        format!(
            "Clipping Plane Fill {}",
            if solid { "Solid" } else { "Hatch" }
        )
    }
}
