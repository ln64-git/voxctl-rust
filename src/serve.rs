use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::_utils::azure::synthesize_speech;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    status: &'static str,
    message: String,
}

#[derive(Debug)]
pub struct AppState {
    subscription_key: String,
    region: String,
}

async fn play_endpoint(
    state: web::Data<Arc<Mutex<AppState>>>,
    payload: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let text = payload["text"].as_str().unwrap_or("Hello, world!");
    let voice_gender = payload["voice_gender"].as_str().unwrap_or("Female");
    let voice_name = payload["voice_name"].as_str().unwrap_or("en-US-AriaNeural");

    println!("play_endpoint - making api call - {:#?}", text);

    let api_response = synthesize_speech(
        state.lock().unwrap().subscription_key.as_str(),
        state.lock().unwrap().region.as_str(),
        text,
        voice_gender,
        voice_name,
    )
    .await;

    match api_response {
        Ok(response) => Ok(HttpResponse::Ok().json(ApiResponse {
            status: "success",
            message: response.body,
        })),
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse {
            status: "error",
            message: format!("Error synthesizing speech: {}", err),
        })),
    }
}

async fn pause_endpoint(_state: web::Data<Arc<Mutex<AppState>>>) -> Result<HttpResponse> {
    println!("Pausing the current playback...");
    // Implement the logic to pause the current playback
    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success",
        message: "Playback paused".to_string(),
    }))
}

async fn resume_endpoint(_state: web::Data<Arc<Mutex<AppState>>>) -> Result<HttpResponse> {
    println!("Resuming the paused playback...");
    // Implement the logic to resume the paused playback
    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success",
        message: "Playback resumed".to_string(),
    }))
}

async fn stop_endpoint(_state: web::Data<Arc<Mutex<AppState>>>) -> Result<HttpResponse> {
    println!("Stopping the current playback...");
    // Implement the logic to stop the current playback
    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success",
        message: "Playback stopped".to_string(),
    }))
}

pub async fn serve() {
    let state = Arc::new(Mutex::new(AppState {
        subscription_key: "".to_string(),
        region: "eastus".to_string(),
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&state)))
            .route("/play", web::post().to(play_endpoint))
            .route("/pause", web::post().to(pause_endpoint))
            .route("/resume", web::post().to(resume_endpoint))
            .route("/stop", web::post().to(stop_endpoint))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
