use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
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
        .route("/healt", get(health_check))
        .route("/tasks", get(list_tasks))
        .route("/tasks/{id}", get(get_task))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Servidor escuchando en http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
}

async fn health_check(State(state): State<AppState>) -> &'static str {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM tasks")
        .fetch_one(&state.pool)
        .await
        .expect("la consulta de salud fallo");
    "OK"
}

async fn list_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, StatusCode> {
    let tasks = sqlx::query_as!(Task, "SELECT id, title, description, completed FROM tasks")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tasks))
}

async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Task>, StatusCode> {
    let task = sqlx::query_as!(
        Task,
        "SELECT id, title, description, completed FROM tasks WHERE id = ?",
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(task))
}
