use std::error::Error;
use std::fmt;

use reqwest::{Client, Url};

const API_ENDPOINT: &str = "https://{}.tts.speech.microsoft.com/cognitiveservices/v1";

#[derive(Debug)]
pub struct ApiResponse {
    pub status_code: u16,
    pub body: String,
}

impl fmt::Display for ApiResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Status: {}\nBody: {}", self.status_code, self.body)
    }
}
pub async fn synthesize_speech(
    subscription_key: &str,
    region: &str,
    text: &str,
    voice_gender: &str,
    voice_name: &str,
) -> Result<ApiResponse, Box<dyn Error>> {
    println!("synthesize_speech - called - {:#?}", text);

    let ssml = generate_ssml(text, voice_gender, voice_name);

    let url = Url::parse(&format!("{} {}", API_ENDPOINT, region))
        .map_err(|e| format!("Error parsing URL: {}", e))?;

    println!("synthesize_speech - {:#?}", text);

    let client = Client::new();
    let response = client
        .post(url)
        .header("Ocp-Apim-Subscription-Key", subscription_key)
        .header("Content-Type", "application/ssml+xml")
        .header("X-Microsoft-OutputFormat", "riff-48khz-16bit-mono-pcm")
        .body(ssml)
        .send()
        .await
        .map_err(|e| format!("Error sending request: {}", e))?;

    let status_code = response.status().as_u16();
    println!("synthesize_speech - status code - {:#?}", status_code);

    let body = response
        .text()
        .await
        .map_err(|e| format!("Error reading response body: {}", e))?;

    Ok(ApiResponse { status_code, body })
}

fn generate_ssml(text: &str, voice_gender: &str, voice_name: &str) -> String {
    format!(
        r#"<speak version='1.0' xml:lang='en-US'>
        <voice xml:lang='en-US' xml:gender='{}' name='{}'>{}</voice>
        </speak>"#,
        voice_gender, voice_name, text
    )
}
