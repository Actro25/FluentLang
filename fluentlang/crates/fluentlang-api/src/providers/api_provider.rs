use crate::api_response::ApiResponse;
use fluentlang_core::error::AppErrors;
use reqwest::Response;

pub trait ApiProvider {
    #[allow(async_fn_in_trait)]
    async fn send_request(sentence: String, private_key: String) -> Result<ApiResponse, AppErrors>;
    #[allow(async_fn_in_trait)]
    async fn parse_to_response(response: Response) -> Result<ApiResponse, AppErrors>;
}