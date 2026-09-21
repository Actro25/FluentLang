use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("This feature is current unavailable. Please wait till we complete this feature.")]
    CurrentlyUnavailable,
    #[error("The config file doesn't exist. Please set you config data first.")]
    ConfigFileDoesntExist,
    #[error("There is json problem with the config file: {0}")]
    ConfigJsonProblem(#[from] serde_json::Error),
    #[error("There is io problems with the config file: {0}")]
    ConfigIOProblem(#[from] std::io::Error),
    #[error("Can't find path to the config file.")]
    CantFindPathToConfigFile,
}