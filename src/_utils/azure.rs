use reqwest::{header, Client, Url};
use std::error::Error;

const API_ENDPOINT: &str = "https://{}.tts.speech.microsoft.com/cognitiveservices/v1";

pub async fn synthesize_speech(
    subscription_key: &str,
    region: &str,
    text: &str,
    voice_gender: &str,
    voice_name: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let ssml = generate_ssml(text, voice_gender, voice_name);
    let url = Url::parse(&format!("{}", API_ENDPOINT.replace("{}", region)))?;

    let client = Client::new();
    let response = client
        .post(url)
        .header("Ocp-Apim-Subscription-Key", subscription_key)
        .header(header::CONTENT_TYPE, "application/ssml+xml")
        .header("X-Microsoft-OutputFormat", "riff-48khz-16bit-mono-pcm")
        .body(ssml)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.bytes().await?;
        Ok(body.to_vec())
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

fn generate_ssml(text: &str, voice_gender: &str, voice_name: &str) -> String {
    format!(
        r#"<speak version='1.0' xml:lang='en-US'>
<voice xml:lang='en-US' xml:gender='{}' name='{}'>{}</voice>
</speak>"#,
        voice_gender, voice_name, text
    )
}
