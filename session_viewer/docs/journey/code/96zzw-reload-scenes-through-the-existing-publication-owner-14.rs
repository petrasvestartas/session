t_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}

#[cfg(test)]
#[test]
fn explicit_named_reload_uses_current_base_and_path_rules(){
 for (path,manifest) in [("floor","scenes/floor.yaml"),("fixtures/empty.yaml","fixtures/empty.yaml"),("https://example.com/empty.yaml","https://example.com/empty.yaml")]{let route=named(path,Some("off"));assert_eq!(route.manifest,manifest);assert_eq!(route.base,"");assert_eq!(route.poll_ms,None);assert_eq!(route.notify,None);}
 assert_eq!(named("floor",Some("http://127.0.0.1:8780/data")).manifest,"http://127.0.0.1:8780/data/scenes/floor.yaml");
}
