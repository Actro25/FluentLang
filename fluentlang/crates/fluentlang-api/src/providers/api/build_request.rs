use crate::providers::api::metadata::ApiMetaData;
use reqwest::RequestBuilder;

/// This trait was created to build requests to servers.
/// ApiMetaData needs for input data as body, url etc.
pub trait ApiBuildRequest: ApiMetaData {
    /// Builds the request to the LLM API.
    ///
    /// # Arguments
    ///
    /// * `key` is a private key to access the desired API
    fn build_request(&self, key: &str) -> RequestBuilder;
}