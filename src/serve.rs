// serve.rs
use axum::{routing::post, Router};
use std::sync::{Arc, Mutex};

struct AppState {
    // Add any necessary shared state here
}

pub async fn serve() {
    let app_state = Arc::new(Mutex::new(AppState {
      // Initialize any shared state here
  }));

    // Build the application with routes
    let app = Router::new()
        .route("/play", post(handle_play))
        .route("/pause", post(handle_pause))
        .route("/resume", post(handle_resume))
        .route("/stop", post(handle_stop))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!(
        "Starting the speech service on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}

async fn handle_play(
    axum::extract::State(_state): axum::extract::State<Arc<Mutex<AppState>>>,
    axum::extract::Json(payload): axum::extract::Json<serde_json::Value>,
) -> impl axum::response::IntoResponse {
    let text = payload["text"].as_str().unwrap_or("");
    println!("Playing text: {}", text);
    // Implement the logic to play the provided text
    axum::response::Json(serde_json::json!({ "status": "success" }))
}

async fn handle_pause(
    axum::extract::State(_state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl axum::response::IntoResponse {
    println!("Pausing the current playback...");
    // Implement the logic to pause the current playback
    axum::response::Json(serde_json::json!({ "status": "success" }))
}

async fn handle_resume(
    axum::extract::State(_state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl axum::response::IntoResponse {
    println!("Resuming the paused playback...");
    // Implement the logic to resume the paused playback
    axum::response::Json(serde_json::json!({ "status": "success" }))
}

async fn handle_stop(
    axum::extract::State(_state): axum::extract::State<Arc<Mutex<AppState>>>,
) -> impl axum::response::IntoResponse {
    println!("Stopping the current playback...");
    // Implement the logic to stop the current playback
    axum::response::Json(serde_json::json!({ "status": "success" }))
}
