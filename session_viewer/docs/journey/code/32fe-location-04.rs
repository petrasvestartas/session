        Self { id: uuid::Uuid::new_v4(), header, version: FileVersion::of(bytes), location: None }
