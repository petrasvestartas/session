//! Native CLI and stdio JSONL server for Session Viewer (`session-viewer-cli`).
//!
//! Subcommands:
//! - `session-viewer-cli exec [scene.yaml|file.pb|file.session]... [--open <path>] [--op <json>]... [--cmd <line>]... [--save <out.session>] [--export-obj <out.obj>] [--render <out.ppm>] [--width <W>] [--height <H>]`
//! - `session-viewer-cli serve [--stdio] [--open <path>]`
//! - `session-viewer-cli schema [--markdown]`

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(err) = cli::run() {
        eprintln!("session-viewer-cli: {err}");
        std::process::exit(1);
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod cli {
    use serde_json::json;
    use session_viewer::core::command::HeadlessHost;
    use session_viewer::op::{self, ErrorCode, Op, OpError, Reply, Request, Response};
    use std::io::{self, BufRead, Write};

    pub fn run() -> Result<(), String> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        match args.first().map(String::as_str) {
            Some("exec") => run_exec(&args[1..]),
            Some("serve") => run_serve(&args[1..]),
            Some("schema") => run_schema(&args[1..]),
            Some("render-stage") => run_render_stage(&args[1..]),
            Some("render-tutorials") => run_render_tutorials(&args[1..]),
            Some("-h" | "--help" | "help") | None => {
                print_usage();
                Ok(())
            }
            Some(other) => Err(format!(
                "unknown subcommand `{other}` (expected `exec`, `serve`, `schema`, `render-stage`, or `render-tutorials`)"
            )),
        }
    }

    fn print_usage() {
        println!(
            "Usage:\n  \
             session-viewer-cli exec [<scene>...] [--open <path>] [--op <json>]... [--cmd <line>]... [--save <out.session>] [--export-obj <out.obj>] [--render <out.ppm>] [--width <W>] [--height <H>]\n  \
             session-viewer-cli serve [--stdio] [--open <path>]\n  \
             session-viewer-cli schema [--markdown]\n  \
             session-viewer-cli render-stage <deck_dir>... [--width <W>] [--height <H>]\n  \
             session-viewer-cli render-tutorials <out_dir>"
        );
    }

    fn run_render_tutorials(args: &[String]) -> Result<(), String> {
        let dir = args.first().ok_or("render-tutorials requires <out_dir>")?;
        let written = session_viewer::selftest::render_tutorial_gallery(dir)?;
        for path in written {
            println!("{path}");
        }
        Ok(())
    }

    fn run_render_stage(args: &[String]) -> Result<(), String> {
        let mut dirs = Vec::new();
        let mut width = 1280u32;
        let mut height = 720u32;
        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--width" => {
                    i += 1;
                    width = args
                        .get(i)
                        .ok_or("--width needs a positive integer")?
                        .parse()
                        .map_err(|_| "invalid --width")?;
                }
                "--height" => {
                    i += 1;
                    height = args
                        .get(i)
                        .ok_or("--height needs a positive integer")?
                        .parse()
                        .map_err(|_| "invalid --height")?;
                }
                arg if arg.starts_with('-') => {
                    return Err(format!("unknown render-stage flag `{arg}`"));
                }
                dir => dirs.push(dir.to_string()),
            }
            i += 1;
        }
        if dirs.is_empty() {
            return Err("render-stage requires at least one <deck_dir>".into());
        }
        for dir in dirs {
            let written = session_viewer::selftest::render_stage_deck_ppms(&dir, width, height)?;
            for path in written {
                println!("{path}");
            }
        }
        Ok(())
    }

    fn execute_with_render(host: &mut HeadlessHost, req: Request) -> Response {
        if let Op::RenderFrame {
            width,
            height,
            path,
        } = &req.op
        {
            return match session_viewer::selftest::render_headless_host(host, *width, *height, path)
            {
                Ok(ink) => Response {
                    id: req.id,
                    ok: true,
                    reply: Some(Reply {
                        message: format!(
                            "Rendered {path} ({width}x{height}, {ink} non-background pixels)"
                        ),
                        created: Vec::new(),
                        changed: Vec::new(),
                        value: json!({
                            "path": path,
                            "width": width,
                            "height": height,
                            "ink_pixels": ink,
                        }),
                    }),
                    error: None,
                },
                Err(msg) => Response {
                    id: req.id,
                    ok: false,
                    reply: None,
                    error: Some(OpError::new(ErrorCode::Unsupported, msg)),
                },
            };
        }
        op::execute_request(host, req)
    }

    fn load_input_path(host: &mut HeadlessHost, path: &str) -> Result<(), String> {
        let lower = path.to_ascii_lowercase();
        let load_op = if lower.ends_with(".session") {
            Op::SceneOpen { path: path.into() }
        } else if lower.ends_with(".yaml")
            || lower.ends_with(".yml")
            || lower.ends_with(".json")
            || lower.ends_with(".toml")
        {
            Op::SceneLoadManifest { path: path.into() }
        } else {
            Op::SceneImport { path: path.into() }
        };
        op::execute(host, load_op)
            .map(|_| ())
            .map_err(|e| e.message)
    }

    fn run_exec(args: &[String]) -> Result<(), String> {
        let mut host = HeadlessHost::new();
        let mut steps: Vec<Request> = Vec::new();
        let mut save_path: Option<String> = None;
        let mut export_obj: Option<String> = None;
        let mut render_ppm: Option<String> = None;
        let mut width = 900u32;
        let mut height = 700u32;
        let mut inputs: Vec<String> = Vec::new();

        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--open" | "--scene" => {
                    i += 1;
                    let p = args.get(i).ok_or("missing path after --open")?;
                    inputs.push(p.clone());
                }
                "--op" => {
                    i += 1;
                    let raw = args.get(i).ok_or("missing JSON after --op")?;
                    let req: Request = serde_json::from_str(raw)
                        .map_err(|e| format!("invalid --op JSON `{raw}`: {e}"))?;
                    steps.push(req);
                }
                "--cmd" => {
                    i += 1;
                    let line = args.get(i).ok_or("missing command line after --cmd")?;
                    steps.push(Request {
                        id: None,
                        op: Op::CommandRun { line: line.clone() },
                    });
                }
                "--save" => {
                    i += 1;
                    save_path = Some(args.get(i).ok_or("missing path after --save")?.clone());
                }
                "--export-obj" => {
                    i += 1;
                    export_obj = Some(
                        args.get(i)
                            .ok_or("missing path after --export-obj")?
                            .clone(),
                    );
                }
                "--render" => {
                    i += 1;
                    render_ppm = Some(args.get(i).ok_or("missing path after --render")?.clone());
                }
                "--width" => {
                    i += 1;
                    width = args
                        .get(i)
                        .ok_or("missing number after --width")?
                        .parse()
                        .map_err(|_| "invalid --width")?;
                }
                "--height" => {
                    i += 1;
                    height = args
                        .get(i)
                        .ok_or("missing number after --height")?
                        .parse()
                        .map_err(|_| "invalid --height")?;
                }
                "-h" | "--help" => {
                    print_usage();
                    return Ok(());
                }
                flag if flag.starts_with("--") => {
                    return Err(format!("unknown flag `{flag}`"));
                }
                positional => inputs.push(positional.to_string()),
            }
            i += 1;
        }

        host.viewport_size = (f64::from(width), f64::from(height));

        for path in &inputs {
            load_input_path(&mut host, path)?;
        }

        let mut stdout = io::stdout().lock();
        for req in steps {
            let resp = execute_with_render(&mut host, req);
            let line = serde_json::to_string(&resp).map_err(|e| e.to_string())?;
            writeln!(stdout, "{line}").map_err(|e| e.to_string())?;
            if !resp.ok {
                let err = resp
                    .error
                    .map(|e| format!("{:?}: {}", e.code, e.message))
                    .unwrap_or_else(|| "operation failed".into());
                return Err(err);
            }
        }

        if let Some(path) = export_obj {
            let resp = execute_with_render(
                &mut host,
                Request {
                    id: None,
                    op: Op::SceneExportObj { path: Some(path) },
                },
            );
            writeln!(
                stdout,
                "{}",
                serde_json::to_string(&resp).map_err(|e| e.to_string())?
            )
            .map_err(|e| e.to_string())?;
            if !resp.ok {
                return Err(resp.error.map(|e| e.message).unwrap_or_default());
            }
        }

        if let Some(path) = save_path {
            let resp = execute_with_render(
                &mut host,
                Request {
                    id: None,
                    op: Op::SceneSave { path: Some(path) },
                },
            );
            writeln!(
                stdout,
                "{}",
                serde_json::to_string(&resp).map_err(|e| e.to_string())?
            )
            .map_err(|e| e.to_string())?;
            if !resp.ok {
                return Err(resp.error.map(|e| e.message).unwrap_or_default());
            }
        }

        if let Some(path) = render_ppm {
            let resp = execute_with_render(
                &mut host,
                Request {
                    id: None,
                    op: Op::RenderFrame {
                        width,
                        height,
                        path,
                    },
                },
            );
            writeln!(
                stdout,
                "{}",
                serde_json::to_string(&resp).map_err(|e| e.to_string())?
            )
            .map_err(|e| e.to_string())?;
            if !resp.ok {
                return Err(resp.error.map(|e| e.message).unwrap_or_default());
            }
        }

        stdout.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    fn run_serve(args: &[String]) -> Result<(), String> {
        let mut host = HeadlessHost::new();
        let mut i = 0usize;
        while i < args.len() {
            match args[i].as_str() {
                "--stdio" => {}
                "--open" | "--scene" => {
                    i += 1;
                    let p = args.get(i).ok_or("missing path after --open")?;
                    load_input_path(&mut host, p)?;
                }
                flag => return Err(format!("unknown serve flag `{flag}`")),
            }
            i += 1;
        }

        let stdin = io::stdin();
        let mut stdout = io::stdout().lock();
        for line in stdin.lock().lines() {
            let line = line.map_err(|e| e.to_string())?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let resp = match serde_json::from_str::<Request>(trimmed) {
                Ok(req) => execute_with_render(&mut host, req),
                Err(err) => Response {
                    id: None,
                    ok: false,
                    reply: None,
                    error: Some(OpError::new(
                        ErrorCode::InvalidArgument,
                        format!("invalid JSON request: {err}"),
                    )),
                },
            };
            let out = serde_json::to_string(&resp).map_err(|e| e.to_string())?;
            writeln!(stdout, "{out}").map_err(|e| e.to_string())?;
            stdout.flush().map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    fn run_schema(args: &[String]) -> Result<(), String> {
        if args.iter().any(|a| a == "--markdown" || a == "-m") {
            print!("{}", op::reference_markdown());
        } else {
            let text =
                serde_json::to_string_pretty(&op::op_catalog()).map_err(|e| e.to_string())?;
            println!("{text}");
        }
        Ok(())
    }
}
