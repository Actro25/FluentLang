use crate::providers::api_build_request::ApiBuildRequest;
use crate::providers::api_metadata::{ApiMetaData, PROMPT};
use crate::providers::api_parse::ApiParse;
use reqwest::RequestBuilder;
use serde_json::{Value, json};

#[derive(Default)]
pub struct GoogleAiStudioAPI {}

impl ApiMetaData for GoogleAiStudioAPI {
    fn url(&self) -> &str {
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.8-flash:generateContent"
    }

    fn body(&self, sentence: &str) -> Value {
        json!({
            "system_instruction": {
                "parts": [
                    {
                        "text": PROMPT
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
        })
    }
}

impl ApiBuildRequest for GoogleAiStudioAPI {
    fn build_request(&self, key: &str) -> RequestBuilder {
        reqwest::Client::new()
            .post(format!("{}?key={}", self.url(), key))
            .header("Content-Type", "application/json")
    }
}

impl ApiParse for GoogleAiStudioAPI {
    fn json_content_route(&self) -> String {
        "/choices/0/message/content".to_string()
    }
}