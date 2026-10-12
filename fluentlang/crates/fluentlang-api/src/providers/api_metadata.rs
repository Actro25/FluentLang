use serde_json::Value;

pub trait ApiMetaData {
    fn url(&self) -> String;
    fn body(&self, sentence: &str) -> Value;
}