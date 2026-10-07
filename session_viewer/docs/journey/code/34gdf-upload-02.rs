        self.uploads += 1;
        self.uploaded_bytes += (source.vertices().len() * 24 + source.indices().len() * 2) as u64;