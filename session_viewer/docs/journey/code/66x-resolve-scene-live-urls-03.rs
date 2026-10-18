    Some(Route {manifest:join(&base,&name),base,poll_ms:None,notify:None})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn named_routes_keep_bucket_relative_files_and_refuse_parent_traversal() {
        assert!(!safe_scene("../outside"));assert!(!safe_scene("folder/../outside"));assert!(!safe_scene("https://other/scene"));assert!(!safe_scene("/scene"));
        let route=select(Some("view_floor"),"/",None,None,None,None).unwrap();assert_eq!(route.manifest,format!("{DATA_BASE}scenes/view_floor.yaml"));assert_eq!(join(&route.base,"pb/floor.pb"),format!("{DATA_BASE}pb/floor.pb"));
        let route=select(Some("fixtures/scene.yaml"),"/",Some("off"),Some("off"),None,None).unwrap();assert_eq!(route.manifest,"fixtures/scene.yaml");assert_eq!(join(&route.base,"./fixtures/a.pb"),"fixtures/a.pb");assert_eq!(path_scene("/view_floor"),Some("view_floor"));assert!(path_scene("/index.html").is_none());assert!(select(None,"/",Some("off"),None,None,None).is_none());
    }
    #[test]
    fn live_routes_keep_local_directories_poll_bounds_and_explicit_notification_urls() {
        let route=select(None,"/",Some("off"),Some("http://localhost:8770/live/scene.yaml"),Some("0.1"),Some("off")).unwrap();assert_eq!(route.base,"http://localhost:8770/live/");assert_eq!(route.poll_ms,Some(5000));assert!(route.notify.is_none());
        let route=select(Some("view_floor"),"/",None,Some("https://example.test/live.yaml"),Some("1"),Some("https://example.test/events")).unwrap();assert_eq!(route.poll_ms,Some(1000));assert_eq!(route.notify.as_deref(),Some("https://example.test/events"));assert_eq!(route.base,"https://example.test/");
    }
}
