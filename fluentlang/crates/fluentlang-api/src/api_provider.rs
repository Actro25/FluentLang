use fluentlang_core::error::AppErrors;
use crate::api_response::ApiResponse;

pub trait ApiProvider {
    #[allow(async_fn_in_trait)]
    async fn send_request(sentence: String, private_key: String) -> Result<ApiResponse, AppErrors>;
}