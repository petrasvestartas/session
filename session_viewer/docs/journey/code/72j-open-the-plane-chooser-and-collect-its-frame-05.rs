}

pub fn choice(text:&str) -> Option<crate::editor::Action> {
    use crate::editor::Action;
    Some(match text.split_whitespace().collect::<String>().to_ascii_lowercase().as_str() {
        "xy"=>Action::ConstructionPlane(Some(CPlane::Xy)),"xz"=>Action::ConstructionPlane(Some(CPlane::Xz)),
        "yz"=>Action::ConstructionPlane(Some(CPlane::Yz)),"view"=>Action::ConstructionPlane(None),
        "3point"=>Action::Plane3,"cancel"|"escape"=>Action::CancelTool,_=>return None,
    })
}

pub fn buttons() -> Vec<(String,String)> {
    ["XY","XZ","YZ","3 Point","View","Cancel"].into_iter().map(|s|(s.into(),format!("CPlane {s}"))).collect()
