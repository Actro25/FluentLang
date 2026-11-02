use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    // Config Errors
    #[error("This feature is current unavailable. Please wait till we complete this feature.")]
    CurrentlyUnavailable,
    #[error("The config file doesn't exist. Please set you config data first.")]
    ConfigFileDoesntExist,
    #[error("There is json problem with the config file: {0}")]
    ConfigJsonProblem(#[from] serde_json::Error),
    #[error("Can't find path to the config file.")]
    CantFindPathToConfigFile,
    #[error("Current provider is not chosen. Please chose a provider.")]
    ProviderIsNotChosen,
    #[error("Current provider is unknown. Please enter current provider again.")]
    UnknowProvider,

    #[error("There is io problem: {0}")]
    IOProblem(#[from] std::io::Error),
    #[error("There is inappropriate provider's json file.")]
    InappropriateJsonResponse,
    #[error("There is a terminal problem: {0}")]
    ColorTerminalProblem(#[from] terminal_colorsaurus::Error),

    #[error("There is a terminal problem: The terminal is too small to show the output. Please make the terminal bigger.")]
    SizeTerminalProblem(),
    
    // Network Errors
    #[error("There is a NetWork error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Client error: {0}")]
    ClientErrorApi(String),
    #[error("Server error: {0}")]
    ServerErrorApi(String),
    #[error("{0}")]
    UnexpectedStatus(String),
}