            "error" | "unhandledrejection" => {
                if let Some(message) = crate::browser_errors::message(&event) { report::fatal(&message); }
                return;
            }
            _ => return,