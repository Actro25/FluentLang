use crate::providers::api_build_request::ApiBuildRequest;
use crate::providers::api_metadata::ApiMetaData;
use crate::providers::api_parse::ApiParse;
use reqwest::RequestBuilder;
use serde_json::{Value, json};

#[derive(Default)]
pub struct GroqAPI {}

impl ApiMetaData for GroqAPI {
    fn url(&self) -> String {
        "https://api.groq.com/openai/v1/chat/completions".to_string()
    }

    fn body(&self, sentence: &str) -> Value {
        json!({
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
    fn json_content_rout(&self) -> String {
        "/choices/0/message/content".to_string()
    }
}