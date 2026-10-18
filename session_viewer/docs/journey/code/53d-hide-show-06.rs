            _ => None,
        }
    } else if !focused {match text.to_ascii_lowercase().as_str() {"f"=>Some(Shortcut::Fit),"h"=>Some(Shortcut::Hide),"s"=>Some(Shortcut::Show),"delete"=>Some(Shortcut::Delete),_=>None}}
    else {None}
}
