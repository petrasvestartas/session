use crate::app::command::{Action, CommandHost, Spec, on_off};
use crate::camera::View;

pub const SPEC: Spec = Spec {
    options: &[
        "View Top",
        "View Side",
        "View Front",
        "View Back",
        "View Left",
        "View Right",
        "View Bottom",
        "View Isometric",
        "View Isometric Back",
        "View Reset",
        "View Perspective",
        "View Orthographic",
        "View Show Edges",
        "View Hide Edges",
        "View Show Lines",
        "View Hide Lines",
        "View Show Points",
        "View Hide Points",
        "View Outline",
        "View Outline On",
        "View Outline Off",
        "View Lighting",
        "View Backfaces",
        "View Cull Backfaces",
        "View Wireframe",
        "View Wireframe On",
        "View Wireframe Off",
        "View Shaded",
        "View Xray",
        "View Point Size",
        "View Plane Size",
        "View Names",
        "View Hide Selected",
        "View Show All",
    ],
    wait_for_option: true,
    ..Spec::new(
        &["View"],
        "View: direction, projection, edges, outlines and display settings",
        parse,
    )
};

#[derive(Debug)]
enum Setting {
    Points,
    Lines,
    Edges,
    Outline,
    Lighting,
    Backfaces,
    Wireframe,
    CullBackfaces,
}

#[derive(Debug)]
enum Change {
    Camera(View),
    Reset,
    Projection(bool),
    Show(Setting, Option<bool>),
    Xray(Option<bool>),
    PointSize(f32),
    PlaneSize(f64),
    Names,
    HideSelected,
    ShowAll,
}

fn parse(_: &str, rest: &[&str]) -> Result<Box<dyn Action>, String> {
    let words = rest.join(" ").to_ascii_lowercase();
    let change = match words.as_str() {
        "top" => Change::Camera(View::Top),
        "side" | "right" => Change::Camera(View::Right),
        "front" => Change::Camera(View::Front),
        "back" => Change::Camera(View::Back),
        "left" => Change::Camera(View::Left),
        "bottom" => Change::Camera(View::Bottom),
        "isometric" => Change::Camera(View::Iso),
        "isometric back" => Change::Camera(View::IsoBack),
        "reset" => Change::Reset,
        "perspective" => Change::Projection(true),
        "orthographic" => Change::Projection(false),
        "show edges" => Change::Show(Setting::Edges, Some(true)),
        "hide edges" => Change::Show(Setting::Edges, Some(false)),
        "show lines" => Change::Show(Setting::Lines, Some(true)),
        "hide lines" => Change::Show(Setting::Lines, Some(false)),
        "show points" => Change::Show(Setting::Points, Some(true)),
        "hide points" => Change::Show(Setting::Points, Some(false)),
        "shaded" => Change::Show(Setting::Wireframe, Some(false)),
        "names" => Change::Names,
        "hide selected" => Change::HideSelected,
        "show all" => Change::ShowAll,
        _ => match rest
            .first()
            .map(|word| word.to_ascii_lowercase())
            .as_deref()
        {
            Some("outline") => Change::Show(
                Setting::Outline,
                on_off(&rest[1..], "View Outline (On Off)")?,
            ),
            Some("lighting") => Change::Show(
                Setting::Lighting,
                on_off(&rest[1..], "View Lighting (On Off)")?,
            ),
            Some("backfaces") => Change::Show(
                Setting::Backfaces,
                on_off(&rest[1..], "View Backfaces (On Off)")?,
            ),
            Some("cull")
                if rest
                    .get(1)
                    .is_some_and(|w| w.eq_ignore_ascii_case("backfaces")) =>
            {
                Change::Show(
                    Setting::CullBackfaces,
                    on_off(&rest[2..], "View Cull Backfaces (On Off)")?,
                )
            }
            Some("wireframe") => Change::Show(
                Setting::Wireframe,
                on_off(&rest[1..], "View Wireframe (On Off)")?,
            ),
            Some("xray") => Change::Xray(on_off(&rest[1..], "View Xray (On Off)")?),
            Some("point") if rest.len() == 3 && rest[1].eq_ignore_ascii_case("size") => {
                let size: f32 = rest[2]
                    .parse()
                    .map_err(|_| "View Point Size needs a positive number")?;
                if !size.is_finite() || size <= 0.0 {
                    return Err("View Point Size needs a positive number".into());
                }
                Change::PointSize(size)
            }
            Some("plane") if rest.len() == 3 && rest[1].eq_ignore_ascii_case("size") => {
                let size: f64 = rest[2]
                    .parse()
                    .map_err(|_| "View Plane Size needs a positive number of mm")?;
                if !size.is_finite() || size <= 0.0 {
                    return Err("View Plane Size needs a positive number of mm".into());
                }
                Change::PlaneSize(size)
            }
            _ => return Err("Choose a View direction or display setting".into()),
        },
    };
    Ok(Box::new(change))
}

impl Action for Change {
    fn keeps_draft(&self) -> bool {
        true
    }
    fn keeps_split(&self) -> bool {
        true
    }

    fn run(&self, state: &mut dyn CommandHost) -> Result<String, String> {
        match *self {
            Self::Camera(view) => {
                state.camera_mut().set_view(view);
                state.touch();
            }
            Self::Reset => {
                state.camera_mut().reset();
                state.touch();
            }
            Self::Projection(perspective) => {
                if state.camera().perspective != perspective {
                    state.refresh_bounds();
                    let bounds = state.scene_bounds();
                    let aspect = state.aspect();
                    state.camera_mut().toggle_projection_framed(&bounds, aspect);
                    state.touch();
                }
            }
            Self::Show(ref setting, value) => {
                let flag = match setting {
                    Setting::Points => crate::app::command::ViewFlag::Points,
                    Setting::Lines => crate::app::command::ViewFlag::Lines,
                    Setting::Edges => crate::app::command::ViewFlag::Edges,
                    Setting::Outline => crate::app::command::ViewFlag::Outline,
                    Setting::Lighting => crate::app::command::ViewFlag::Lighting,
                    Setting::Backfaces => crate::app::command::ViewFlag::Backfaces,
                    Setting::Wireframe => crate::app::command::ViewFlag::Wireframe,
                    Setting::CullBackfaces => crate::app::command::ViewFlag::CullBackfaces,
                };
                let next = value.unwrap_or(!state.view_flag(flag));
                state.set_view_flag(flag, next);
            }
            Self::Xray(value) => {
                if value.is_none_or(|on| on != (state.opacity() == 0.0)) {
                    state.toggle_xray();
                }
            }
            Self::PointSize(size) => state.set_cloud_size(size),
            Self::PlaneSize(size) => state.set_plane_size(size),
            Self::Names => state.toggle_selected_names(),
            Self::HideSelected => state.hide_selected(),
            Self::ShowAll => state.show_all(),
        }
        Ok("View updated".into())
    }
}
