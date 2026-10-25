use crate::providers::api::metadata::ApiMetaData;
use reqwest::RequestBuilder;

/// This trait was created to build requests to servers.
/// ApiMetaData needs for input data as body, url etc.
pub trait ApiBuildRequest: ApiMetaData {
    /// The function is supposed to create RequestBuilder.
    /// 
    /// * `key` - a private key that is needed for building url or body.
    fn build_request(&self, key: &str) -> RequestBuilder;
}