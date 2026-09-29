use serde_json::json;
use fluentlang_core::config_parsing;
use fluentlang_core::error::AppErrors;

pub struct GroqAPI {}

impl GroqAPI {
    pub fn new() -> Self {
        Self{}
    }

    pub async fn send_request(sentence: String) -> Result<String, AppErrors> {
        let config_data = config_parsing::get_config_data(config_parsing::get_path("config.json")?)?;
        let body = json!({
            "model": "openai/gpt-oss-120b",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional language assistant for the language in the \
                    sentence below. You must analyze the grammar used in the sentence and point \
                    out any errors, if there are any. Respond concisely, in no more than 5 sentences, \
                    and only in the language of the sentence below."
                },
                {
                    "role": "user",
                    "content": sentence
                }
            ]
        });

        let response = reqwest::Client::new()
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", config_data.private))
            .json(&body)
            .send().await;
        if let Ok(res) = response {
            println!("Status: {}", res.status());
            let response_text = res.text().await;
            if let Ok(r) = response_text {
                println!("Response: {}", r);
            }
        }
        Ok("".into())
    }
}