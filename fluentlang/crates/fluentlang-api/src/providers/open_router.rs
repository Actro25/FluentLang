use crate::providers::api::build_request::ApiBuildRequest;
use crate::providers::api::metadata::{ApiMetaData, PROMPT};
use crate::providers::api::parse::ApiParse;
use reqwest::RequestBuilder;
use serde_json::{Value, json};

#[derive(Default)]
pub struct OpenRouterAPI {}

impl ApiMetaData for OpenRouterAPI {
    fn url(&self) -> &str {
        "https://openrouter.ai/api/v1/chat/completions"
    }

    fn body(&self, sentence: &str) -> Value {
        json!({
            "model": "apodex/apodex-1.1-mini:free",
            "messages": [
              {
                "role": "system",
                "content": PROMPT
              },
              {
                "role": "user",
                "content": sentence
              }
            ],
            "reasoning": {
              "enabled": true
            }
        })
    }
}

impl ApiBuildRequest for OpenRouterAPI {
    fn build_request(&self, key: &str) -> RequestBuilder {
        reqwest::Client::new()
            .post(self.url())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", key))
    }
}

impl ApiParse for OpenRouterAPI {
    fn json_content_route(&self) -> String {
        "/choices/0/message/content".to_string()
    }
}