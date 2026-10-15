use crate::providers::api::metadata::ApiMetaData;
use reqwest::RequestBuilder;

/// This trait was created to build requests to servers.
/// ApiMetaData needs for input data as body, url etc.
pub trait ApiBuildRequest: ApiMetaData {
    /// The function creates RequestBuilder and expects that a struct implements ApiMetaData.
    fn build_request(&self, key: &str) -> RequestBuilder;
}