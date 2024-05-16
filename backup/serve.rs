use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use std::{
    env,
    sync::{Arc, Mutex},
};

use crate::_utils::{audio::play_audio, azure::synthesize_speech};

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    status: &'static str,
    message: String,
}

#[derive(Debug)]
pub struct AppState {
    pub subscription_key: String,
    pub region: String,
    pub voice_gender: String,
    pub voice_name: String,
}

pub async fn play_endpoint(
    state: web::Data<Arc<Mutex<AppState>>>,
    payload: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    println!("play_endpoint - function called");
    let text = payload["text"].as_str().unwrap_or("Hello, world!");

    println!("play_endpoint - about to call synthesize_speech");

    let audio_response = match synthesize_speech(
        state.lock().unwrap().subscription_key.as_str(),
        state.lock().unwrap().region.as_str(),
        state.lock().unwrap().voice_gender.as_str(),
        state.lock().unwrap().voice_name.as_str(),
        text,
    )
    .await
    {
        Ok(response) => {
            println!("synthesize_speech response: {:?}", response);
            response
        }
        Err(err) => {
            println!("Error synthesizing speech: {}", err);
            return Ok(HttpResponse::InternalServerError().json(ApiResponse {
                status: "error",
                message: format!("Error synthesizing speech: {}", err),
            }));
        }
    };

    match play_audio(audio_response) {
        Ok(_) => {
            println!("Audio played successfully");
            Ok(HttpResponse::Ok().json(ApiResponse {
                status: "success",
                message: "Audio played successfully".to_string(),
            }))
        }
        Err(err) => Ok(HttpResponse::InternalServerError().json(ApiResponse {
            status: "error",
            message: format!("Error playing audio: {}", err),
        })),
    }
}

pub async fn serve() {
    let subscription_key = "".to_string();
    let region = "eastus".to_string();
    let voice_gender = "Female".to_string();
    let voice_name = "en-US-JennyNeural".to_string();

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
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .await
    .unwrap();
}
