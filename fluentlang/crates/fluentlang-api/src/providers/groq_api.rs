use crate::providers::api_build_request::ApiBuildRequest;
use crate::providers::api_metadata::{ApiMetaData, PROMPT};
use crate::providers::api_parse::ApiParse;
use reqwest::RequestBuilder;
use serde_json::{Value, json};

#[derive(Default)]
pub struct GroqAPI {}

impl ApiMetaData for GroqAPI {
    fn url(&self) -> &str {
        "https://api.groq.com/openai/v1/chat/completions"
    }

    fn body(&self, sentence: &str) -> Value {
        json!({
            "model": "openai/gpt-oss-120b",
            "messages": [
                {
                    "role": "system",
                    "content": PROMPT
                },
                {
                    "role": "user",
                    "content": sentence
                }
            ]
        })
    }
}

impl ApiBuildRequest for GroqAPI {
    fn build_request(&self, key: &str) -> RequestBuilder {
        reqwest::Client::new()
            .post(self.url())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", key))
    }
}

impl ApiParse for GroqAPI {
    fn json_content_route(&self) -> String {
        "/choices/0/message/content".to_string()
    }
}