use crate::providers::api_build_request::ApiBuildRequest;
use crate::providers::api_metadata::ApiMetaData;
use crate::providers::api_parse::ApiParse;
use reqwest::RequestBuilder;
use serde_json::{Value, json};

#[derive(Default)]
pub struct GoogleAiStudioAPI {}

impl ApiMetaData for GoogleAiStudioAPI {
    fn url(&self) -> String {
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-3.8-flash:generateContent?key={}".to_string()
    }

    fn body(&self, sentence: &str) -> Value {
        json!({
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
    fn json_content_rout(&self) -> String {
        "/choices/0/message/content".to_string()
    }
}