mod _utils;
use crate::_utils::{audio::play_audio, azure::synthesize_speech};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();
    let args: Vec<String> = env::args().collect();
    let action: &str = &args[1];
    let text: Option<&String> = args.get(2);

    match action {
        "play" => {
            if let Some(text) = text {
                play(text).await;
            } else {
                println!("Please provide text to play.");
            }
        }
        _ => println!("Invalid action: {}", action),
    }
}

async fn play(input_text: &str) {
    println!("play_text - input_text: {}", input_text);

    let subscription_key = env::var("AZURE_SUBSCRIPTION_KEY").unwrap_or("".to_string());
    let region = env::var("AZURE_REGION").unwrap_or("eastus".to_string());
    let voice_gender = env::var("VOICE_GENDER").unwrap_or("Female".to_string());
    let voice_name = env::var("VOICE_NAME").unwrap_or("en-US-JennyNeural".to_string());

    match synthesize_speech(
        subscription_key.as_str(),
        region.as_str(),
        input_text,
        voice_gender.as_str(),
        voice_name.as_str(),
    )
    .await
    {
        Ok(response) => {
            println!("synthesize_speech response: {:?}", response);
            if let Err(err) = play_audio(response) {
                println!("Error playing audio: {}", err);
            }
        }
        Err(err) => {
            println!("Error synthesizing speech: {}", err);
        }
    }
}
