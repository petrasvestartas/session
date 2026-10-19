                let marked = crate::cap_gpu_tests::pixels_at(&r, m);
                if projection == crate::camera::Projection::Orthographic && samples == 4 {
                    visibility_picture(if owner == left { "selected-left-edge" } else { "selected-right-edge" }, &marked);
                }
                assert!(