use std::env;
mod _utils;
mod serve;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
        || args.contains(&String::from("-h"))
        || args.contains(&String::from("--help"))
    {
        display_help();
        return;
    }

    let action: &str = &args[1];
    let text: Option<&String> = args.get(2);

    match action {
        "serve" => serve().await,
        "play" => {
            if let Some(text) = text {
                play(text).await;
            } else {
                println!("Please provide text to play.");
            }
        }
        "pause" => pause().await,
        "resume" => resume().await,
        "stop" => stop().await,
        "service_provider" => set_service_provider(),
        "help" => display_help(),
        _ => println!("Invalid action: {}", action),
    }
}

async fn serve() {
    println!("Starting the speech service...");
    serve::serve().await;
}

async fn play(text: &str) {
    println!("Playing text: {}", text);
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:3000/play")
        .json(&serde_json::json!({ "text": text }))
        .send()
        .await;

    match response {
        Ok(res) => println!("main - {:?}", res.text().await),
        Err(err) => println!("Error sending request: {}", err),
    }
}

async fn pause() {
    println!("Pausing the current playback...");
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3000/pause").send().await;

    match response {
        Ok(res) => println!("main - {:?}", res.text().await),
        Err(err) => println!("Error sending request: {}", err),
    }
}

async fn resume() {
    println!("Resuming the paused playback...");
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3000/resume").send().await;

    match response {
        Ok(res) => println!("main - {:?}", res.text().await),
        Err(err) => println!("Error sending request: {}", err),
    }
}

async fn stop() {
    println!("Stopping the current playback...");
    let client = reqwest::Client::new();
    let response = client.post("http://localhost:3000/stop").send().await;

    match response {
        Ok(res) => println!("main - {:?}", res.text().await),
        Err(err) => println!("Error sending request: {}", err),
    }
}

fn set_service_provider() {
    println!("Setting the speech service provider...");
    // Implement the logic to set the speech service provider
}

fn display_help() {
    println!("Usage: <action> [text]");
    println!("Actions:");
    println!("  serve | s - Start the speech service");
    println!("  play <text> | p - Play the provided text");
    println!("  pause | pa - Pause the current playback");
    println!("  resume | r - Resume the paused playback");
    println!("  stop | st - Stop the current playback");
    println!("  service_provider | sp - Set the speech service provider");
    println!("  help | h - Display this help message");
}
