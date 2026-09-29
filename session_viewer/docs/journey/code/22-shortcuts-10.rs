                event.prevent_default();
                if canvas.focus().is_err() || canvas.set_pointer_capture(id).is_err() {