use crate::State;
use crate::app::command::{Action, Spec, on_off};
use crate::camera::View;

pub const SPEC: Spec = Spec {
    options: &[
        "View Top", "View Side", "View Front", "View Back", "View Left", "View Right",
        "View Bottom", "View Isometric", "View Isometric Back", "View Reset", "View Perspective", "View Orthographic",
        "View Show Edges", "View Hide Edges", "View Show Lines", "View Hide Lines",
        "View Show Points", "View Hide Points", "View Outline", "View Outline On", "View Outline Off",
        "View Lighting", "View Backfaces", "View Xray", "View Point Size", "View Plane Size", "View Names",
        "View Hide Selected", "View Show All",
    ],
    wait_for_option: true,
    ..Spec::new(&["View"], "View: direction, projection, edges, outlines and display settings", parse)
};

#[derive(Debug)]
enum Setting { Points, Lines, Edges, Outline, Lighting, Backfaces }

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
        "names" => Change::Names,
        "hide selected" => Change::HideSelected,
        "show all" => Change::ShowAll,
        _ => match rest.first().map(|word| word.to_ascii_lowercase()).as_deref() {
            Some("outline") => Change::Show(Setting::Outline, on_off(&rest[1..], "View Outline (On Off)")?),
            Some("lighting") => Change::Show(Setting::Lighting, on_off(&rest[1..], "View Lighting (On Off)")?),
            Some("backfaces") => Change::Show(Setting::Backfaces, on_off(&rest[1..], "View Backfaces (On Off)")?),
            Some("xray") => Change::Xray(on_off(&rest[1..], "View Xray (On Off)")?),
            Some("point") if rest.len() == 3 && rest[1].eq_ignore_ascii_case("size") => {
                let size: f32 = rest[2].parse().map_err(|_| "View Point Size needs a positive number")?;
                if !size.is_finite() || size <= 0.0 { return Err("View Point Size needs a positive number".into()); }
                Change::PointSize(size)
            }
            Some("plane") if rest.len() == 3 && rest[1].eq_ignore_ascii_case("size") => {
                let size: f64 = rest[2].parse().map_err(|_| "View Plane Size needs a positive number of mm")?;
                if !size.is_finite() || size <= 0.0 { return Err("View Plane Size needs a positive number of mm".into()); }
                Change::PlaneSize(size)
            }
            _ => return Err("Choose a View direction or display setting".into()),
        },
    };
    Ok(Box::new(change))
}

impl Action for Change {
    fn keeps_draft(&self) -> bool { true }
    fn keeps_split(&self) -> bool { true }

    fn run(&self, state: &mut State) -> Result<String, String> {
        match *self {
            Self::Camera(view) => state.camera.set_view(view),
            Self::Reset => state.camera.reset(),
            Self::Projection(perspective) => {
                if state.camera.perspective != perspective {
                    state.refresh_bounds();
                    state.camera.toggle_projection_framed(&state.gpu.bounds, state.aspect());
                }
            }
            Self::Show(ref setting, value) => {
                let flag = match setting {
                    Setting::Points => &mut state.gpu.view.show_points,
                    Setting::Lines => &mut state.gpu.view.show_lines,
                    Setting::Edges => &mut state.gpu.view.show_mesh_edges,
                    Setting::Outline => &mut state.gpu.view.show_outlines,
                    Setting::Lighting => &mut state.gpu.view.lit,
                    Setting::Backfaces => &mut state.gpu.view.backface,
                };
                *flag = value.unwrap_or(!*flag);
            }
            Self::Xray(value) => {
                if value.is_none_or(|on| on != (state.gpu.view.opacity == 0.0)) { state.toggle_xray(); }
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
