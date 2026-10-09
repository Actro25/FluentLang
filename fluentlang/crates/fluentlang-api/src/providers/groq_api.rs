use reqwest::StatusCode;
use fluentlang_core::error::AppErrors;
use serde_json::json;
use crate::api_response::ApiResponse;

pub struct GroqAPI {}

impl GroqAPI {
    pub async fn send_request(sentence: String, private_key: String) -> Result<ApiResponse, AppErrors> {
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
            .await?;

        let status_code = response.status();
        match status_code {
            StatusCode::OK => {
                let json_value: serde_json::Value = response.json().await?;

                let content = json_value
                    .pointer("/choices/0/message/content")
                    .and_then(|v| v.as_str())
                    .ok_or(AppErrors::InappropriateJsonResponse)?
                    .to_string();

                Ok(ApiResponse::new(content))
            }
            code if code.is_client_error() => {
                Err(AppErrors::ClientErrorApi(status_code.to_string()))
            }
            code if code.is_server_error() => {
                Err(AppErrors::ServerErrorApi(status_code.to_string()))
            }
            code if code.is_redirection() => {
                Err(AppErrors::UnexpectedStatus(format!("Redirection status: {}", code)))
            }
            _ => {
                Err(AppErrors::UnexpectedStatus(format!("Unexpected HTTP status: {}", status_code)))
            }
        }
    }
}