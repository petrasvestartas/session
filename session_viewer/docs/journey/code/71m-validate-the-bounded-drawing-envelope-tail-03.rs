    impl MetadataWindow {
        /// The bytes at `at`, reading more when not cached.
        pub(crate) async fn read(
            &mut self,
            url: &str,
