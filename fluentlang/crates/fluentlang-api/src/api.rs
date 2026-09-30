use fluentlang_core::error::AppErrors;
use serde_json::json;

pub struct GroqAPI {}

impl GroqAPI {
    pub async fn send_request(sentence: String, private_key: String) -> Result<String, AppErrors> {
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
            .header("Authorization", format!("Bearer {}", private_key))
            .json(&body)
            .send()
            .await;
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