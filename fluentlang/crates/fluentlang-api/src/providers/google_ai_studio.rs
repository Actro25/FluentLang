use crate::api_response::ApiResponse;
use crate::providers::api_provider::ApiProvider;
use fluentlang_core::error::AppErrors;
use reqwest::{Response, StatusCode};
use serde_json::json;

pub struct GoogleAiStudioAPI {}

impl GoogleAiStudioAPI {
    pub fn new() -> Self {
        Self {}
    }
}

impl ApiProvider for GoogleAiStudioAPI {
    async fn send_request(sentence: String, private_key: String) -> Result<ApiResponse, AppErrors> {
        let body = json!({
            "system_instruction": {
                "parts": [
                    {
                        "text": "You are a professional language assistant for the language in the \
                        sentence below. You must analyze the grammar used in the sentence and point \
                        out any errors, if there are any. Respond concisely, in no more than 5 sentences, \
                        and only in the language of the sentence below."
                    }
                ]
            },
            "contents": [
                {
                    "parts": [
                        {
                            "text": sentence
                        }
                    ]
                }
            ]
        });

        let response = reqwest::Client::new()
            .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-3.8-flash:generateContent?key={}", private_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status_code = response.status();
        match status_code {
            StatusCode::OK => {
                Ok(Self::parse_to_response(response).await?)
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

    async fn parse_to_response(response: Response) -> Result<ApiResponse, AppErrors> {
        let json_value: serde_json::Value = response.json().await?;

        let content = json_value
            .pointer("/candidates/0/content/parts/0/text")
            .and_then(|v| v.as_str())
            .ok_or(AppErrors::InappropriateJsonResponse)?
            .to_string();

        Ok(ApiResponse::new(content))
    }
}