use reqwest::RequestBuilder;
use crate::providers::api_metadata::ApiMetaData;

pub trait ApiBuildRequest: ApiMetaData {
    fn build_request(&self, key: &str) -> RequestBuilder;
}