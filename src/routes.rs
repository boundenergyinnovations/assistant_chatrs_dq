use axum::{
    extract::Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use crate::openai::OpenAIClient;
use std::time::Duration;
use tokio::time::sleep;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    response: String,
}

pub async fn chat(Json(payload): Json<ChatRequest>) -> impl IntoResponse {
    if payload.message.starts_with("!msg ") {
        // Handle the !msg command
        let content = &payload.message[5..]; // Skip "!msg "
        match write_to_file(content) {
            Ok(_) => return Json(ChatResponse { response: "Message received, thank you.".to_string() }),
            Err(e) => return Json(ChatResponse { response: format!("Error writing to file: {}", e) }),
        }
    }

    let client = OpenAIClient::new();

    // Create a thread
    let thread_id = client.create_thread().await.unwrap();

    // Add user message to the thread
    client.add_message(&thread_id, &payload.message).await.unwrap();

    // Run the assistant
    let run_id = client.run_assistant(&thread_id).await.unwrap();

    // Poll for completion
    loop {
        let status = client.get_run_status(&thread_id, &run_id).await.unwrap();
        if status == "completed" {
            break;
        }
        sleep(Duration::from_secs(1)).await;
    }

    // Get the assistant's response
    let response = client.get_messages(&thread_id).await.unwrap();

    Json(ChatResponse { response })
}

fn write_to_file(content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("messages.txt")?;
    
    writeln!(file, "{}", content)?;
    Ok(())
}
