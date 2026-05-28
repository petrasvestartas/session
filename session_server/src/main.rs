use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use axum::{Json, Router, extract::State, routing::{get, post}};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

type Queue = Arc<Mutex<VecDeque<String>>>;

#[derive(Deserialize)]
struct CmdRequest {
    cmd: String,
}

#[derive(Serialize)]
struct PollResponse {
    commands: Vec<String>,
}

async fn push_cmd(State(q): State<Queue>, Json(req): Json<CmdRequest>) -> &'static str {
    q.lock().unwrap().push_back(req.cmd);
    "ok"
}

async fn poll_cmds(State(q): State<Queue>) -> Json<PollResponse> {
    let commands: Vec<String> = q.lock().unwrap().drain(..).collect();
    Json(PollResponse { commands })
}

#[tokio::main]
async fn main() {
    let queue: Queue = Arc::new(Mutex::new(VecDeque::new()));
    let app = Router::new()
        .route("/cmd", post(push_cmd))
        .route("/poll", get(poll_cmds))
        .layer(CorsLayer::permissive())
        .with_state(queue);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9999").await.unwrap();
    println!("session_server on http://127.0.0.1:9999");
    println!("  curl -X POST http://127.0.0.1:9999/cmd -H 'Content-Type: application/json' -d '{{\"cmd\":\"point 10 20 30\"}}'");
    axum::serve(listener, app).await.unwrap();
}
