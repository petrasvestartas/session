        let Some(saved) = from.pop() else { return false; };
        to.push(scene.clone());
        scene.restore_view(saved);
        true
    }
