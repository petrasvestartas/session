    StreamedCloud(Box<StreamedInit>), // a point cloud starts streaming; register:stream
    CloudChunk(CloudChunk),        // more points arrived; register:stream
    CloudQueryBatch(app::cloud_query::Batch), // points asked for on click; register:cloud_query
    CloudQueryResolved(app::cloud_query::Resolved), // those points answered; register:cloud_query
