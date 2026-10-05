use axum::{Json, Router, extract::State, routing::get};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL debe estar definida en .env");

    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("No se pudieron ejecutar las migraciones");

    println!("Conectado a la base de datos y migraciones al día");

    let state = AppState { pool };

    let app = Router::new()
        .route("/", get(root))
        .route("/tasks/demo", get(demo_task))
        .route("/healt", get(health_check))
        .with_state(state);

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

async fn health_check(State(state): State<AppState>) -> &'static str {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
    .fetch_one(&state.pool)
    .await
    .expect("la consulta de salud fallo");
    "OK"
}
