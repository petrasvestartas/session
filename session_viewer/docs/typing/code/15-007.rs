                PendingDocument::Streamed(stream) => _ = post(Msg::StreamedCloud(stream)), // register:stream
