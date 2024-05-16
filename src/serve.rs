use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, Mutex},
};

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
    voice_gender: String,
    voice_name: String,
}

async fn play_endpoint(
    state: web::Data<Arc<Mutex<AppState>>>,
    payload: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let text = payload["text"].as_str().unwrap_or("Hello, world!");

    let api_response = synthesize_speech(
        state.lock().unwrap().subscription_key.as_str(),
        state.lock().unwrap().region.as_str(),
        state.lock().unwrap().voice_gender.as_str(),
        state.lock().unwrap().voice_name.as_str(),
        text,
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
    dotenv::dotenv().ok();
    let subscription_key = env::var("AZURE_SUBSCRIPTION_KEY").unwrap_or("".to_string());
    let region = env::var("AZURE_REGION").unwrap_or("eastus".to_string());
    let voice_gender = env::var("VOICE_GENDER").unwrap_or("Female".to_string());
    let voice_name = env::var("VOICE_NAME").unwrap_or("en-US-JennyNeural".to_string());

    let state = Arc::new(Mutex::new(AppState {
        subscription_key,
        region,
        voice_gender,
        voice_name,
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
