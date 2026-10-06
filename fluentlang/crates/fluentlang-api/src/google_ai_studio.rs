use fluentlang_core::error::AppErrors;
use serde_json::json;

pub struct GoogleAiStudio {}

impl GoogleAiStudio {
    pub async fn send_request(sentence: String, private_key: String) -> Result<String, AppErrors> {
        let body = json!({
            "contents": [{
                "parts":[{"text": sentence}]
            }]
        });

        let response = reqwest::Client::new()
            .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-3.8-flash:generateContent?key={}", private_key))
            .header("Content-Type", "application/json")
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