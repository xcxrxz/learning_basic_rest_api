use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/tasks/demo", get(demo_task));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor escuchando en http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn demo_task() -> Json<Task> {
    Json(Task {
        id: 1,
        title: "Aprender Rust".to_string(),
        description: Some("Terminar el tutorial de API REST".to_string()),
        completed: false,
    })
}
