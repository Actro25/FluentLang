use crate::providers::api::build_request::ApiBuildRequest;
use crate::providers::api::parse::ApiParse;
use fluentlang_core::error::AppErrors;
use reqwest::{Response, StatusCode};

pub trait ApiSender: ApiBuildRequest + ApiParse {}

/// Here we are saying that if ApiBuildRequest and ApiParse implemented for T then ApiSender also implemented.
impl<T: ApiBuildRequest + ApiParse> ApiSender for T {}

/// This function returns ApiResponse struct that builds from api answer.
/// The function gets provider that has to implement all expected traits.
/// The function gets sentence to send to server.
/// The function gets private key for the provider.
pub async fn get_response<P: ApiSender>(
    provider: P,
    sentence: String,
    key: String,
) -> Result<String, AppErrors> {
    let response = provider
        .build_request(&key)
        .json(&provider.body(&sentence))
        .send()
        .await?;

    let status_code = response.status();
    match status_code {
        StatusCode::OK => Ok(parse_to_response(response, provider.json_content_route()).await?),
        code if code.is_client_error() => Err(AppErrors::ClientErrorApi(status_code.to_string())),
        code if code.is_server_error() => Err(AppErrors::ServerErrorApi(status_code.to_string())),
        code if code.is_redirection() => Err(AppErrors::UnexpectedStatus(format!(
            "Redirection status: {}",
            code
        ))),
        _ => Err(AppErrors::UnexpectedStatus(format!(
            "Unexpected HTTP status: {}",
            status_code
        ))),
    }
}

async fn parse_to_response(response: Response, route: String) -> Result<String, AppErrors> {
    let json_value: serde_json::Value = response.json().await?;

    let content = json_value
        .pointer(&route)
        .and_then(|v| v.as_str())
        .ok_or(AppErrors::InappropriateJsonResponse)?
        .to_string();

    Ok(content)
}