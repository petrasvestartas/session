use crate::app::command::{Action, CommandHost, Spec};
use crate::app::coords;
use crate::app::gizmo::GizmoAlign;
use session_rust::Point;

pub const SPEC: Spec = Spec {
    options: &[
        "Gumball World",
        "Gumball Object",
        "Gumball CPlane",
        "Gumball Reset",
    ],
    wait_for_option: true,
    ..Spec::new(
        &["Gumball"],
        "Gumball <World | Object | CPlane | Pivot x,y,z | Reset>: align or relocate the gumball",
        parse,
    )
};

#[derive(Debug)]
enum GumballOp {
    Align(GizmoAlign),
    Pivot(Option<Point>),
}

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    if rest.is_empty() {
        return Err("Choose Gumball World, Object, CPlane, Pivot <x,y,z>, or Reset".into());
    }
    match rest[0].to_ascii_lowercase().as_str() {
        "world" => Ok(Box::new(GumballOp::Align(GizmoAlign::World))),
        "object" | "local" => Ok(Box::new(GumballOp::Align(GizmoAlign::Object))),
        "cplane" => Ok(Box::new(GumballOp::Align(GizmoAlign::CPlane))),
        "reset" => Ok(Box::new(GumballOp::Pivot(None))),
        "pivot" if rest.len() >= 2 => {
            let joined = rest[1..].join("");
            let [x, y, z] =
                coords::absolute(&joined).ok_or("Gumball Pivot needs x,y,z coordinates")?;
            Ok(Box::new(GumballOp::Pivot(Some(Point::new(x, y, z)))))
        }
        _ => Err("Usage: Gumball <World | Object | CPlane | Pivot x,y,z | Reset>".into()),
    }
}

impl Action for GumballOp {
    fn keeps_draft(&self) -> bool {
        true
    }

    fn run(&self, host: &mut dyn CommandHost) -> Result<String, String> {
        match self {
            GumballOp::Align(align) => Ok(host.set_gizmo_align(*align)),
            GumballOp::Pivot(pivot) => {
                if pivot.is_none() {
                    host.set_gizmo_align(GizmoAlign::World);
                }
                Ok(host.set_gizmo_pivot(pivot.clone()))
            }
        }
    }
}
