mod _utils;
use crate::_utils::playback::PlaybackCommand;
use _utils::azure::synthesize_speech;
use _utils::{log::print_log, playback::init_playback_channel};
use std::env;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct AppState {
    pub playback_send: mpsc::Sender<PlaybackCommand>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        AppState {
            playback_send: self.playback_send.clone(),
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    print_log("Server starting...");

    let args: Vec<String> = env::args().collect();
    let action: &str = &args[1];
    let text: Option<&String> = args.get(2);

    let playback_send = init_playback_channel().await;

    let app_state = AppState { playback_send };

    match action {
        "play" => {
            if let Some(text) = text {
                play(text, &app_state).await;
            } else {
                println!("Please provide text to play.");
            }
        }
        _ => println!("Invalid action: {}", action),
    }
}

async fn play(input_text: &str, app_state: &crate::AppState) {
    print_log(&("play_text - input_text: ".to_string() + input_text));

    let subscription_key = env::var("AZURE_SUBSCRIPTION_KEY").unwrap_or("".to_string());
    let region = env::var("AZURE_REGION").unwrap_or("eastus".to_string());
    let voice_gender = env::var("VOICE_GENDER").unwrap_or("Female".to_string());
    let voice_name = env::var("VOICE_NAME").unwrap_or("en-US-JennyNeural".to_string());

    if let Ok(response) = synthesize_speech(
        subscription_key.as_str(),
        region.as_str(),
        input_text,
        voice_gender.as_str(),
        voice_name.as_str(),
    )
    .await
    {
        print_log(&format!("play - about to play audio response..."));
        app_state
            .playback_send
            .send(PlaybackCommand::Play(response))
            .await
            .unwrap();
    } else {
        print_log(&format!("Error synthesizing speech: "));
    }
}
