e,attributes,depth,pickable,false)
    }
}

#[cfg(test)]
#[path="grid_pick_pipeline_tests.rs"]
mod pick_pipeline_tests;

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="stroke_sample_tests.rs"]
mod sample_tests;
