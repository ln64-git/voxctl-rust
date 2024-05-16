use std::error::Error;

pub async fn synthesize_speech(
    subscription_key: &str,
    region: &str,
    text: &str,
    voice_gender: &str,
    voice_name: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let token_url = format!(
        "https://{}.api.cognitive.microsoft.com/sts/v1.0/issueToken",
        region
    );

    let tts_url = format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
        region
    );

    let token_response = reqwest::Client::new()
        .post(&token_url)
        .header("Ocp-Apim-Subscription-Key", subscription_key)
        .header("Content-Length", "0")
        .send()
        .await?;

    let access_token = token_response.text().await?;

    let tts_response = reqwest::Client::new()
        .post(&tts_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Content-Type", "application/ssml+xml")
        .header("X-Microsoft-OutputFormat", "riff-48khz-16bit-mono-pcm")
        .header("User-Agent", "text-to-speech-exp")
        .body(format!(
            r#"<speak version='1.0' xml:lang='en-US'><voice xml:lang='en-US' xml:gender='{}' name='{}'>{}</voice></speak>"#,
            voice_gender, voice_name, text
        ))
        .send()
        .await?;

    if tts_response.status().is_success() {
        let bytes = tts_response.bytes().await?;
        Ok(bytes.to_vec())
    } else {
        Err(format!("Request failed with status: {}", tts_response.status()).into())
    }
}
