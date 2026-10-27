            if key.key() == "Escape" {
                gesture.cancel();
                event.prevent_default();
                return Some(Action::Escape);
            }
