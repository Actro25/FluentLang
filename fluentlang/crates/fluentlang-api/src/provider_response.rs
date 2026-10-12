use crate::api_response::ApiResponse;
use crate::providers::api_build_request::ApiBuildRequest;
use crate::providers::api_metadata::ApiMetaData;
use crate::providers::api_parse::ApiParse;
use fluentlang_core::error::AppErrors;
use reqwest::{Response, StatusCode};

pub async fn get_response<P: ApiMetaData + ApiBuildRequest + ApiParse>(
    provider: P,
    sentence: String,
    key: String,
) -> Result<ApiResponse, AppErrors> {
    let response = provider
        .build_request(&key)
        .json(&provider.body(&sentence))
        .send()
        .await?;

    let status_code = response.status();
    match status_code {
        StatusCode::OK => {
            Ok(parse_to_response(response, provider.json_content_rout()).await?)
        }
        code if code.is_client_error() => {
            Err(AppErrors::ClientErrorApi(status_code.to_string()))
        }
        code if code.is_server_error() => {
            Err(AppErrors::ServerErrorApi(status_code.to_string()))
        }
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

async fn parse_to_response(
    response: Response,
    route: String,
) -> Result<ApiResponse, AppErrors> {
    let json_value: serde_json::Value = response.json().await?;

    let content = json_value
        .pointer(&route)
        .and_then(|v| v.as_str())
        .ok_or(AppErrors::InappropriateJsonResponse)?
        .to_string();

    Ok(ApiResponse::new(content))
}
