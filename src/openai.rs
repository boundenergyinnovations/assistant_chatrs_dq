use reqwest::Client;
use serde_json::Value;
use std::env;

pub struct OpenAIClient {
    client: Client,
    api_key: String,
    assistant_id: String,
}

impl OpenAIClient {
    pub fn new() -> Self {
        let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
        let assistant_id = env::var("OPENAI_ASSISTANT_ID").expect("OPENAI_ASSISTANT_ID must be set");
        Self {
            client: Client::new(),
            api_key,
            assistant_id,
        }
    }

    pub async fn create_thread(&self) -> Result<String, reqwest::Error> {
        let url = "https://api.openai.com/v1/threads";

        let response: Value = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("OpenAI-Beta", "assistants=v2")
            .send()
            .await?
            .json()
            .await?;

        Ok(response["id"].as_str().unwrap().to_string())
    }

    pub async fn add_message(&self, thread_id: &str, content: &str) -> Result<(), reqwest::Error> {
        let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);
        let body = serde_json::json!({
            "role": "user",
            "content": content
        });

        self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("OpenAI-Beta", "assistants=v2")
            .json(&body)
            .send()
            .await?;

        Ok(())
    }

    pub async fn run_assistant(&self, thread_id: &str) -> Result<String, reqwest::Error> {
        let url = format!("https://api.openai.com/v1/threads/{}/runs", thread_id);
        let body = serde_json::json!({
            "assistant_id": self.assistant_id
        });

        let response: Value = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("OpenAI-Beta", "assistants=v2")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response["id"].as_str().unwrap().to_string())
    }

    pub async fn get_run_status(&self, thread_id: &str, run_id: &str) -> Result<String, reqwest::Error> {
        let url = format!("https://api.openai.com/v1/threads/{}/runs/{}", thread_id, run_id);

        let response: Value = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v2")
            .send()
            .await?
            .json()
            .await?;

        Ok(response["status"].as_str().unwrap().to_string())
    }

    pub async fn get_messages(&self, thread_id: &str) -> Result<String, reqwest::Error> {
        let url = format!("https://api.openai.com/v1/threads/{}/messages", thread_id);

        let response: Value = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .header("OpenAI-Beta", "assistants=v2")
            .send()
            .await?
            .json()
            .await?;

        let last_message = response["data"].as_array().unwrap().first().unwrap();
        let content = last_message["content"][0]["text"]["value"].as_str().unwrap();

        Ok(content.to_string())
    }
}
